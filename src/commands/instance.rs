use crate::shell::Shell;

use super::{argument::EvaluatedArg, status::CommandStatus};

#[derive(Debug)]
pub struct CommandInstance<'a> {
    shell: &'a Shell,
    arg_list: Vec<EvaluatedArg>,
    callback: fn(&Shell, &Vec<EvaluatedArg>) -> Box<dyn CommandStatus>
}

impl<'a> CommandInstance<'a> {
    pub fn new(shell: &'a Shell, arg_list: Vec<EvaluatedArg>, callback: fn(&Shell, &Vec<EvaluatedArg>) -> Box<dyn CommandStatus>) -> Self {
        CommandInstance { shell, arg_list, callback }
    }

    pub fn run(&self) -> Box<dyn CommandStatus>{
        (self.callback)(self.shell, &self.arg_list)
    }
}