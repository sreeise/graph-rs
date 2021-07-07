use crate::parser::HttpMethod;
use std::collections::{VecDeque, HashSet};
use crate::api_types::{RequestTask, RequestMetadata};
use bytes::{BytesMut, BufMut};
use graph_http::RequestAttribute::RequestType;
use core::slice::SlicePattern;

/*
TODO:

 The goal is to make parameter names for request methods the same as they are in the path:

 Given the list of parameter names that were originally available in the path of the request
 we can set the parameter names on each method.

 The two macros we need are the register_method macro:

     ( { doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [ $p:ident $p1:ident ], has_body: false } ) => {
        #[doc = $doc]
        pub fn $name<S: AsRef<str>>(&'a self, $p: S, $p1: S)-> IntoResponse<'a, $T, Client>
        {
            self.client.request()
                .set_method($m);


            render_path!(
                    self.client,
                    $template,
                    &serde_json::json!({ "id": $p.as_ref(), "id1": $p1.as_ref() }));
            IntoResponse::new(&self.client.request)
        }
    };


 and the http method macros such as:

     ({ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, params: [ $p:ident $p1:ident ], has_body: false }) => {
        register_method!(
            { doc: $doc, name: $name, response: $T, path: $template, method: Method::GET, params: [ $p $p1 ], has_body: false }
        );
    };

 This will allow us to write the request macro with the parameter names:

    get!({
        doc: "# Get historyItems from me",
        name: get_activity_history,
        response: serde_json::Value,
        path: "/activities/{{id}}/historyItems/{{id1}}}",
        params: [ user_activity_id history_items_id ],
        has_body: false
    });

 When syntax highlighting takes place the user will see the parameter names
 instead of id and id1.
 */

pub trait MacroWriter {
    fn write(&self, path: std::path::Path);

    fn parameter_bounds(param_size: usize, has_body: bool) -> &'static str {
        if param_size > 0 {
            match has_body {
                false => "<S: AsRef<str>>",
                true => "<S: AsRef<str>, B: serde::Serialize>",
            }
        } else if has_body {
            "<B: serde::Serialize>"
        } else {
            ""
        }
    }

    fn parameters(param_size: usize, has_body: bool) -> String {
        let mut s = String::from("(&'a self");

        for i in 0..param_size {
            if i == 0 {
                s.push_str(", $p: S");
            } else {
                s.push_str(&format!(", $p{}: S", i));
            }
        }

        if has_body {
            s.push_str(", body: &B");
        }
        s.push(')');
        s
    }
}

/// Writes the macro used for describing requests. This is the outer
/// most macro and is used to describe all requests.
///
/// # Example Macro
/// ```rust,ignore
/// get!({
///     doc: "# Get historyItems from me",
///     name: get_activity_history,
///     response: serde_json::Value,
///     path: "/activities/{{id}}/historyItems/{{id1}}}",
///     params: [ user_activity_id history_items_id ],
///     has_body: false
/// });
/// ```
pub trait ApiMacroWriter {
    /// A description of what the request is doing.
    fn doc(&self) -> Option<String>;

    /// The HTTP method for the request. Must be one of GET, PUT, POST, PATCH, DELETE
    /// Macro type: expr
    fn http_method(&self) -> HttpMethod;

    /// The method name that is used to call this request.
    /// Macro type: ident
    fn fn_name(&self) -> String;

    /// The request task describes the type of action this request will perform.
    fn request_task(&self) -> RequestTask;

    /// The URL path.
    /// Macro type: expr
    fn path(&self) -> String;

    /// A list of parameter names that will be used for the request method.
    fn params(&self) -> &VecDeque<String>;

    /// Does the request require a body.
    fn has_body(&self) -> bool;

    fn request_metadata(&self) -> &VecDeque<RequestMetadata>;

    /// The macro call name such as `vec!`
    fn macro_fn_name(&self) -> &str {
        let http_method = self.http_method();
        match self.request_task() {
            RequestTask::NoContent
            | RequestTask::Json
            | RequestTask::Bytes
            | RequestTask::Upload
            | RequestTask::UploadSession
            | RequestTask::Delta => http_method.as_ref(),
            RequestTask::Download => "download",
            RequestTask::AsyncDownload => "async_download"
        }
    }

    fn macro_params(&self) -> String {
        let mut parameter_str = String::new();
        for param in self.params().iter() {
            parameter_str.push_str(&format!(" {} ", param));
        }
        parameter_str
    }

    fn write(&self) {
        let mut doc = String::new();
        if let Some(doc_string) = self.doc() {
            doc = format!("\n\t\tdoc: \"{}\", ", doc_string);
        }

        let mut buf = BytesMut::new();
        let name = self.fn_name();
        let path = self.path();
        let params = self.macro_params();
        let has_body = self.has_body();
        let request_task = self.request_task();
        let is_upload = request_task == RequestTask::Upload;
        let is_upload_session = request_task == RequestTask::UploadSession;
        let type_name = request_task.type_name();

        buf.put(&format!("\n\t{}!({{", self.macro_fn_name()));
        buf.put(doc);
        buf.put(&format!("\n\t\tname: {}", name));
        buf.put(&format!(",\n\t\tresponse: {}", type_name));
        buf.put(&format!(",\n\t\tpath: {}", path));
        buf.put(&format!(",\n\t\tparams: [{}]", params));
        if has_body { buf.put(&format!(",\n\t\thas_body: {}\n\t", has_body)); }
        if is_upload { buf.put(",\n\t\tupload: true\n\t"); }
        if is_upload_session { buf.put(",\n\t\tupload_session: true\n\t"); }
        buf.put("}});");

        let s = std::str::from_utf8(buf.as_slice()).unwrap();
        println!("{}", s);
    }
}

/// Helper struct to write out the macros that create the request methods.
/// The macros are located at ./src/client/macros.rs
pub struct MacroFormatter;

impl MacroFormatter {
    pub fn method_macro(params: usize, has_body: bool, method: HttpMethod) -> String {
        let method_str = {
            match method {
                HttpMethod::GET => "Method::GET",
                HttpMethod::PUT => "Method::PUT",
                HttpMethod::PATCH => "Method::PATCH",
                HttpMethod::POST => "Method::POST",
                HttpMethod::DELETE => "Method::DELETE",
                HttpMethod::TRACE => panic!("Error, illegal trace method"),
            }
        };

        format!(
            "({{ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, \
             params: {}, has_body: {} }}) => {{
            register_method!(
                {{ doc: $doc, name: $name, response: $T, path: $template, method: {}, params: {}, \
             has_body: {} }}
            );
        }};",
            params, has_body, method_str, params, has_body
        )
    }

    fn param_bounds(param_size: usize, has_body: bool) -> &'static str {
        if param_size > 0 {
            match has_body {
                false => "<S: AsRef<str>>",
                true => "<S: AsRef<str>, B: serde::Serialize>",
            }
        } else if has_body {
            "<B: serde::Serialize>"
        } else {
            ""
        }
    }

    fn parameters(param_size: usize, has_body: bool) -> String {
        let mut s = String::from("(&'a self");

        for i in 0..param_size {
            if i == 0 {
                s.push_str(", $p: S");
            } else {
                s.push_str(&format!(", $p{}: S", i));
            }
        }

        if has_body {
            s.push_str(", body: &B");
        }
        s.push(')');
        s
    }

    fn render_path(param_size: usize) -> String {
        let mut s = String::new();
        for i in 0..param_size {
            if i == 0 {
                s.push_str(" \"id\": $p.as_ref()");
            } else {
                s.push_str(&format!(", \"id{}\": $p{}.as_ref()", i, i));
            }
        }

        format!(
            "render_path!(
                self.client,
                $template,
                &serde_json::json!({{{ } }})\
            );",
            s
        )
    }

    fn macro_parameters(param_size: usize, doc: bool, has_body: bool) -> String {
        let mut s = String::new();
        for i in 0..param_size {
            if i == 0 {
                s.push_str(" $p:ident ");
            } else {
                s.push_str(&format!("$p{}:ident ", i));
            }
        }

        if doc {
            format!("( {{ doc: $doc:expr, name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [{}], has_body: {} }} )", s, has_body)
        } else {
            format!("( {{ name: $name:ident, response: $T:ty, path: $template:expr, method: $m:expr, params: [{}], has_body: {} }} )", s, has_body)
        }
    }

    pub fn register_method_macros(param_size: usize, doc: bool, has_body: bool) -> String {
        let macro_params = MacroFormatter::macro_parameters(param_size, doc, has_body);
        let bounds_with_body = MacroFormatter::param_bounds(param_size, has_body);
        let params = MacroFormatter::parameters(param_size, has_body);
        let render_path = MacroFormatter::render_path(param_size);

        let doc_comment = {
            if doc {
                "#[doc = $doc]"
            } else {
                ""
            }
        };

        let serialize_body = {
            if has_body {
                "if let Err(err) = client.set_body_with_serialize(body) {{
                return IntoResponse::new_error(self.client.request(), err);
              }}"
            } else {
                ""
            }
        };

        format!("{} => {{
    {}
    pub fn $name{}{}-> IntoResponse<'a, $T, Client>
    {{
        self.client.request()
            .set_method($m);
        {}

        {}
        IntoResponse::new(&self.client.request)
    }}
}};",
            macro_params, doc_comment, bounds_with_body, params, serialize_body, render_path
        )
    }
}
