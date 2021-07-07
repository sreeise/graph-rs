use crate::parser::{HttpMethod, Modifier, Request};
use inflector::Inflector;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

lazy_static! {
    /// Matches any number. Some of the graph request data has
    /// numbers in the name of the operation id such as
    /// groups.users.get.23a. This becomes an issue when parsing
    /// the resource id. For instance, the method name for an
    /// individual request is taken from the last part of the resource id
    /// and method names really should not be named 23a.
    static ref NUM_REG: Regex = Regex::new(r"[0-9]").unwrap();

    /// matches ids attached to the resource name such as groups({id}).
    static ref PATH_ID_REG: Regex = Regex::new(r"(\(\{)(\w+)(}\))").unwrap();

    /// Matches named ids such as {group-id}.
    pub static ref PATH_ID_NAMED_REG: Regex = Regex::new(r"(\{)(\w+-\w+)(})").unwrap();

    pub static ref INTERNAL_PATH_ID: Regex = Regex::new(r"(\{\{)(\w+)(}})").unwrap();

    pub static ref KEY_VALUE_PAIR: Regex = Regex::new(r"(\w+)(=\{)(\w+)(})").unwrap();

    pub static ref KEY_VALUE_PAIR_RAW_QUOTED: Regex = Regex::new(r#"(\w+)(='\{)(\w+)(}')"#).unwrap();
}

pub trait RequestParserBuilder<RHS: ?Sized = Self> {
    fn build(&self, path: String, modifier: &Modifier, http_method: HttpMethod) -> Request;
}

pub trait RequestParser<RHS = Self> {
    fn method_name(&self) -> String;
    fn operation_mapping(&self) -> String;
    fn transform_path(&self) -> String;
    fn links(&self) -> HashSet<String> {
        Default::default()
    }
}

pub trait Modify<T> {
    fn modify(&self, value: &mut T);
}

impl RequestParser for &str {
    /// Parse the method name of a request.
    fn method_name(&self) -> String {
        let mut method_name = String::new();
        if let Some(index) = self.rfind('.') {
            let last: &str = self[index + 1..].as_ref();
            if NUM_REG.is_match(last) {
                if let Some(idx) = self[..index].rfind('.') {
                    method_name.push_str(self[idx + 1..index].as_ref());
                }
            } else {
                method_name.push_str(self[index + 1..].as_ref());
            }
        } else {
            method_name.push_str(&self);
        }

        if method_name.is_empty() {
            self.to_snake_case()
        } else {
            method_name.to_snake_case()
        }
    }

    /// Create the operation mapping that will be used to create
    /// client structs, links between these structs, and the mapping
    /// for where to place request methods.
    fn operation_mapping(&self) -> String {
        let mut op_mapping = String::new();

        if self.contains('.') {
            let mut ops: Vec<&str> = self.split('.').collect();
            ops.retain(|s| !s.is_empty());

            if let Some(last) = ops.pop() {
                if NUM_REG.is_match(last) && ops.len() > 1 {
                    ops.pop();
                }

                if ops.len() > 1 {
                    op_mapping = ops.join(".");
                } else {
                    op_mapping = ops.join("");
                }
            }
        } else {
            op_mapping = self.to_string();
        }

        if op_mapping.ends_with('.') {
            op_mapping.truncate(op_mapping.len() - 1);
        }

        if op_mapping.matches('.').count() == 1 {
            let mut queue: VecDeque<&str> = op_mapping.split('.').collect();
            let first = queue.pop_front().unwrap();
            let last = queue.pop_front().unwrap();
            if first[..first.len() - 1].eq(last) {
                op_mapping = first.to_string();
            } else if last[..last.len() - 1].eq(first) {
                op_mapping = last.to_string();
            }
        }

        op_mapping
    }

    fn transform_path(&self) -> String {
        let mut path = self.to_string();
        let path_clone = path.clone();

        let replace_ids = |count: usize, s: &str, path: &mut String| {
            if count == 1 {
                path.replacen(s, "{{id}}", 1)
            } else {
                path.replacen(s, &format!("{{{{id{}}}}}", count), 1)
            }
        };

        // Replaces ids in paths attached to the resource name such as groups({id})
        let mut count = 1;
        for cap in PATH_ID_REG.captures_iter(path_clone.as_str()) {
            let s = cap[0].to_string();
            if count == 1 {
                path = path.replacen(s.as_str(), "/{{id}}", 1);
            } else {
                path = path.replacen(s.as_str(), &format!("/{{{{id{}}}}}", count), 1);
            }
            count += 1;
        }

        // Replaces named ids such as {group-id}.
        let mut count = 1;
        for cap in PATH_ID_NAMED_REG.captures_iter(path_clone.as_str()) {
            let s = cap[0].to_string();
            path = replace_ids(count, s.as_str(), &mut path);
            count += 1;
        }

        // Replaces key-value pairs such as getActivitiesByInterval(interval={interval})
        for cap in KEY_VALUE_PAIR.captures_iter(path_clone.as_str()) {
            let s = cap[0].to_string();
            if let Some(i) = s.find('=') {
                path = replace_ids(count, &s[i + 1..], &mut path);
            }
            count += 1;
        }

        // Replaces key-value pairs such as
        // getActivitiesByInterval(interval=\'{interval}\')
        for cap in KEY_VALUE_PAIR_RAW_QUOTED.captures_iter(path_clone.as_str()) {
            let s = cap[0].to_string();
            if let Some(i) = s.find('=') {
                path = replace_ids(count, &s[i + 1..], &mut path);
            }
            count += 1;
        }

        // Some of the paths end with a name that starts with microsoft.graph
        // such as microsoft.graph.delta. We remove that part of the path in
        // case of issues when performing the actual request.
        if path.contains("microsoft.graph.") {
            path.replace("microsoft.graph.", "").replace("()", "")
        } else {
            path
        }
    }

    fn links(&self) -> HashSet<String> {
        let mut links: HashSet<String> = HashSet::new();

        if self.contains('.') {
            let mut vec: Vec<&str> = self.split('.').collect();
            vec.retain(|s| !s.is_empty());

            let mut iter = vec.iter().peekable();

            while let Some(current) = iter.next() {
                if let Some(next) = iter.peek() {
                    links.insert(format!("{}.{}", current, next));
                }
            }
        } else {
            links.insert(self.to_string());
        }

        links
    }
}

impl RequestParser for String {
    fn method_name(&self) -> String {
        self.as_str().method_name()
    }

    fn operation_mapping(&self) -> String {
        self.as_str().operation_mapping()
    }

    fn transform_path(&self) -> String {
        self.as_str().transform_path()
    }

    fn links(&self) -> HashSet<String> {
        self.as_str().links()
    }
}
