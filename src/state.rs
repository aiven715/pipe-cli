// use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

// type Value = RefCell<HashMap<String, String>>;

pub struct State {}

impl State {
    pub fn new() -> Self {
        // let store = RefCell::new(HashMap::new());

        Self {}
    }

    pub fn set(&self) {
        // let mut store = self.store.borrow_mut();

        // store.insert(String::from("foo"), String::from("bar"));
    }

    pub fn evaluate(&self, path: &str) -> Result<String, EvaluationError> {
        // let store = self.store.borrow();
        let mut split = path.split('.');

        match split.next() {
            Some("env") => {
                let variable_name = split.next().unwrap();

                match read_env_file() {
                    Ok(mut env) => match env.remove(variable_name) {
                        Some(value) => Ok(value),
                        None => Err(EvaluationError::EnvError(format!(
                            "variable {} not exists",
                            variable_name,
                        ))),
                    },
                    Err(err) => Err(EvaluationError::EnvError(format!(
                        "error while reading a file - {}",
                        err,
                    ))),
                }
            }
            None => Err(EvaluationError::NamespaceError(
                "no namespace provided".to_string(),
            )),
            _ => Err(EvaluationError::NamespaceError(
                "unknown namespace".to_string(),
            )),
        }
    }
}

#[derive(Debug)]
pub enum EvaluationError {
    EnvError(String),
    NamespaceError(String),
}

fn read_env_file() -> io::Result<HashMap<String, String>> {
    let file = File::open(".env")?;
    let mut vars = HashMap::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let line: Vec<&str> = line.splitn(2, "=").collect();

        match (line.get(0), line.get(1)) {
            (Some(key), Some(value)) => {
                vars.insert(key.to_string(), value.to_string());
            }
            _err => {
                // TODO: throw error if incorrect variable was passed
            }
        }
    }

    Ok(vars)
}
