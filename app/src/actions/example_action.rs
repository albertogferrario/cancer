//! example_action action

use ferro_rs::injectable;

#[injectable]
pub struct ExampleAction {
    // Dependencies injected via container
}

impl ExampleAction {
    pub fn execute(&self) -> String {
        "Hello from ExampleAction!".to_string()
    }
}
