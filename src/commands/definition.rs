use crate::{error::CommandError, inout::read::ArgToken};

use super::{argument::{ArgType, EvaluatedArg}, instance::CommandInstance, status::{CommandStatus, Passed}};

#[derive(Debug)]
pub struct CommandDefinition {
    name: &'static str,
    arg_list: Vec<ArgType>,
    callback: fn(&Vec<EvaluatedArg>) -> Box<dyn CommandStatus>
}

impl Clone for CommandDefinition {
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), arg_list: self.arg_list.clone(), callback: self.callback.clone() }
    }
}

impl CommandDefinition {
    pub fn new(name: &'static str) -> Self {
        Self { name: name, arg_list: vec![], callback: |_args| { Box::new(Passed()) } }
    }

    pub fn build(&self) -> CommandDefinition {
        self.clone()
    }

    pub fn add_arg(&mut self, arg_type: ArgType) -> &mut Self {
        self.arg_list.push(arg_type);

        self
    }

    pub fn set_callback(&mut self, callback: fn(&Vec<EvaluatedArg>) -> Box<dyn CommandStatus>) -> &mut Self {
        self.callback = callback;

        self
    }
}

impl CommandDefinition {
    pub fn instantiate(&self, arg_list: Vec<ArgToken>) -> Result<CommandInstance, CommandError>{
        if arg_list.len() > self.arg_list.len() { 
            return Err(CommandError::TooManyArguments(self.name.to_string(), self.arg_list.len(), arg_list.len())) 
        }

        else if arg_list.len() < self.arg_list.len() { 
            return Err(CommandError::TooFewArguments(self.name.to_string(), self.arg_list.len(), arg_list.len())) 
        }

        let mut inst_arg_list: Vec<EvaluatedArg> = vec![];
        let mut arg_list = arg_list.iter();

        for arg in &self.arg_list {
            match arg.evaluate(arg_list.next().unwrap()) {
                Ok(eval) => inst_arg_list.push(eval),
                Err(e) => return Err(e)
            }
        }


        Ok(CommandInstance::new(inst_arg_list, self.callback))
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn arg_list(&self) -> &Vec<ArgType> {
        &self.arg_list
    }
}