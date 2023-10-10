/// Macros for generating simple tasks.

/// # Example
///
/// ```rust
/// use dagrs::{Dag, Action, Input, EnvVar, Output, RunningError, DefaultTask, gen_task,Task};
/// use std::sync::Arc;
/// let task = gen_task!("task A", |input, env| {
/// Ok(Output::empty())
/// });
/// assert_eq!(task.id(),1);
/// assert_eq!(task.name(),"task A");
/// ```
#[macro_export]
macro_rules! gen_task {
    ($task_name:literal,$action:expr) => {{
        use crate::{EnvVar, Input, Output,RunningError};
        use std::sync::Arc;
        pub struct SimpleAction;
        impl Action for SimpleAction {
            fn run(&self, input: Input, env: Arc<EnvVar>) -> Result<Output, RunningError> {
                Ok($action(input, env))
            }
        }
        DefaultTask::new(SimpleAction, $task_name)
    }};
}

#[macro_export]
macro_rules! gen_action {
    ($action:expr) => {{
        use crate::{EnvVar, Input, Output,RunningError};
        use std::sync::Arc;
        pub struct SimpleAction;
        impl Action for SimpleAction {
            fn run(&self, input: Input, env: Arc<EnvVar>) -> Result<Output, RunningError> {
                Ok($action(input, env))
            }
        }
        SimpleAction
    }};
}