use super::{argument::EvaluatedArg, status::CommandStatus};

#[derive(Debug)]
pub struct CommandInstance {
    arg_list: Vec<EvaluatedArg>,
    callback: fn(&Vec<EvaluatedArg>) -> Box<dyn CommandStatus>
}

impl CommandInstance {
    pub fn new(arg_list: Vec<EvaluatedArg>, callback: fn(&Vec<EvaluatedArg>) -> Box<dyn CommandStatus>) -> Self {
        CommandInstance { arg_list, callback }
    }

    pub fn run(&self) -> Box<dyn CommandStatus>{
        (self.callback)(&self.arg_list)
    }
}