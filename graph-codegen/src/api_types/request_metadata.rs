use crate::api_types::RequestTask;
use crate::inflector::Inflector;
use crate::parser::HttpMethod;
use crate::traits::RequestParser;
use from_as::*;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io::{Read, Write};
use crate::macros::writer::MacroFormatter;

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestMetadata {
    pub has_body: bool,
    pub request_task: RequestTask,
    pub operation_id: String,
    pub operation_mapping: String,
    pub http_method: HttpMethod,
    pub doc: Option<String>,
}

impl RequestMetadata {
    pub fn fn_name(&self) -> String {
        self.operation_id.method_name()
    }

    pub fn type_name(&self) -> &'static str {
        self.request_task.type_name()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct PathMetadata {
    pub path: String,
    pub param_size: usize,
    pub parameters: VecDeque<String>,
    pub metadata: VecDeque<RequestMetadata>,
}

impl PathMetadata {
    pub fn replace_brackets(&mut self) {
        self.path = self.path.replace('{', "{{").replace('}', "}}");
    }

    fn snake_case_parameters(&self) -> VecDeque<String> {
        self.parameters
            .iter()
            .map(|param| param.to_snake_case())
            .collect()
    }

    pub fn format_path_parameters(&mut self) {
        let mut counter = 0;
        let mut path = self.path.clone();

        for param in self.parameters.iter() {
            let param_snake_case = format!("{{{}}}", param.to_snake_case());
            if counter == 0 {
                path = path.replacen(param.as_str(), param_snake_case.as_str(), 1);
            } else {
                path = path.replacen(param.as_str(), param_snake_case.as_str(), 1);
            }
            counter += 1;
        }

        self.path = path;
        self.parameters = self.snake_case_parameters();
    }

    pub fn generate_request_impl(&self) -> VecDeque<String> {
        let mut parameter_str_list = String::new();
        for param in self.parameters.iter() {
            parameter_str_list.push_str(&format!(" {} ", param));
        }

        self.metadata
            .iter()
            .map(|m| {
                format!(
                    "\n\t{}!({{\n\t\tdoc: \"{}\",\n\t\tname: {},\n\t\tresponse: \
                         {},\n\t\tpath: \"{}\",\n\t\tparams: [{}],\n\t\thas_body: {}\n\t}});",
                    m.http_method.as_ref(),
                    m.doc.clone().unwrap_or_default(),
                    m.fn_name(),
                    m.type_name(),
                    self.path.as_str(),
                    parameter_str_list,
                    m.has_body
                )
            })
            .collect()
    }
}

pub struct PathMetadataQueue(VecDeque<PathMetadata>);

impl PathMetadataQueue {
    pub fn new() -> PathMetadataQueue {
        PathMetadataQueue(VecDeque::new())
    }
}

impl From<VecDeque<PathMetadata>> for PathMetadataQueue {
    fn from(queue: VecDeque<PathMetadata>) -> Self {
        PathMetadataQueue(queue)
    }
}
