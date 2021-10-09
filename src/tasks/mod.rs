mod http;

use serde_json::Value;

pub trait Task {
    fn name(&self) -> &str;
    fn execute(&self) -> Result<Value, TaskExecutionError>;
}

pub enum TaskType {
    Http,
}

pub struct TaskExecutionError(pub String);

pub use self::http::Http;
