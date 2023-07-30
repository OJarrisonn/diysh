use crate::shell::Shell;

use super::argument::EvaluatedArg;

pub struct CommandInstance<'a> {
    shell: &'a Shell,
    arg_list: Vec<EvaluatedArg>,
    callback: fn(&Shell, &Vec<EvaluatedArg>)
}

impl<'a> CommandInstance<'a> {
    pub fn new(shell: &'a Shell, arg_list: Vec<EvaluatedArg>, callback: fn(&Shell, &Vec<EvaluatedArg>)) -> Self {
        CommandInstance { shell, arg_list, callback }
    }

    pub fn run(&self) {
        (self.callback)(self.shell, &self.arg_list)
    }
}