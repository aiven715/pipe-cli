use std::collections::HashMap;

pub struct State {
    value: HashMap<String, String>,
}

impl State {
    pub fn new() -> Self {
        let value = HashMap::new();

        Self { value }
    }

    pub fn set(&mut self) {
        self.value.insert(String::from("foo"), String::from("bar"));
    }
}
