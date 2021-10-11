use crate::tasks::{Http, Task};
use crate::State;
// use regex::Regex;
use std::fs;
// TODO: migrate to serde yaml
use yaml_rust::{Yaml, YamlLoader};

// Spec is responsible for:
// - Yaml parsing
// - Variables interpolation
// - Modules embedding?

pub struct Spec {
    yaml: Yaml,
}

impl Spec {
    pub fn try_new(filename: &str) -> Result<Spec, SpecError> {
        let file = fs::read_to_string(filename).or(Err(SpecError::FileError))?;
        let mut docs = YamlLoader::load_from_str(&file).or(Err(SpecError::SyntaxError))?;
        let yaml = docs.remove(0);

        if let Some(error) = validate(&yaml) {
            return Err(SpecError::ValidationError(error.into()));
        }

        Ok(Spec { yaml })
    }

    pub fn tasks(&self) -> impl Iterator<Item = impl Task + '_> {
        match self.yaml["tasks"].as_vec() {
            Some(tasks) => tasks.into_iter().map(|task| {
                let name = task["name"].as_str().expect("task name is missing");
                let task_type = task["type"].as_str().expect("task type is missing");

                match task_type {
                    "http" => {
                        let method = task["method"].as_str().expect("http method is missing");
                        let url = task["url"].as_str().expect("http url is missing");

                        Http::new(name, method, url)
                    }
                    _ => panic!("unknown task type"),
                }
            }),
            // It is fine to panic here since we're checking for 'tasks' existence
            // during validation
            None => panic!("'tasks' field is missing"),
        }
    }
}

pub struct Slice<'a> {
    yaml: &'a Yaml,
    state: &'a State,
}

impl<'a> Slice<'a> {
    pub fn new(yaml: &'a Yaml, state: &'a State) -> Self {
        Self { yaml, state }
    }

    pub fn get(&self, key: &str) -> Self {
        Self {
            yaml: &self.yaml[key],
            state: self.state,
        }
    }

    pub fn as_str(&self, key: &str) -> Option<&str> {
        self.yaml[key].as_str().map(|value| {
            ();
            value
        })
    }

    //     fn substitute(&self, text: &str) -> &str {
    //         let re = Regex::new(r"(\{\{.+\}\})").unwrap();

    //         re.replace_all(text, |captured| "")
    //     }
}

pub enum SpecError {
    FileError,
    SyntaxError,
    ValidationError(String),
}

pub enum Field {
    Tasks,
    Output,
}

// TODO: perform schema based validation such as JSON schema
fn validate(yaml: &Yaml) -> Option<&str> {
    if let None = yaml["tasks"].as_vec() {
        return Some("'tasks' field is required");
    }

    None
}
