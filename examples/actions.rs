//! Construct two different actions.
//! - Create a closure directly
//! - Implementing Complex, the type can have some additional information.
use std::sync::Arc;
use async_trait::async_trait;
use dagrs::{Complex, DefaultTask, EnvVar, Input, Output};

struct Act(usize);

#[async_trait]
impl Complex for Act {
    async fn run(&self, _input: dagrs::Input, _env: std::sync::Arc<dagrs::EnvVar>) -> Output {
        Output::new(self.0 + 10)
    }
}
fn main() {
    let simple = |_input, _env| Output::new("simple");
    let _simple_task = DefaultTask::with_closure("simple task", simple);

    let complex = Act(20);
    let _complex_task = DefaultTask::with_action("complex action", complex);
}
