#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_from_to_file;
use transform_request::prelude::*;

// A struct that has the serde derive traits can also
// derive FromFile and ToFile making it easy to store
// Rust structs as files.
#[derive(Debug, Serialize, Deserialize, FromFile, ToFile, FromYamlFile, ToYamlFile)]
struct Field {
    name: String,
}

impl Field {
    pub fn new(n: &str) -> Field {
        Field {
            name: String::from(n),
        }
    }
}

fn main() {
    as_json_file();
    as_yaml_file();
}

fn as_json_file() {
    let field = Field::new("field_name");
    field
        .to_file("./examples/example_files/field.json")
        .unwrap();

    let field: Field = Field::from_file("./examples/example_files/field.json").unwrap();
    println!("{:#?}", &field);
}

fn as_yaml_file() {
    let field = Field::new("field_name");
    field
        .to_yaml_file("./examples/example_files/field.yaml")
        .unwrap();

    let field: Field = Field::from_yaml_file("./examples/example_files/field.yaml").unwrap();
    println!("{:#?}", &field);
}
