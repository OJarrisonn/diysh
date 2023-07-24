use std::collections::HashMap;

use crate::error::CommandError;

use super::argument::{ArgType, EvaluatedArg};

pub struct CommandDefinition {
    name: String,
    arg_list: Vec<ArgType>,
    opt_args: HashMap<String, ArgType>,
    callback: fn(Vec<EvaluatedArg>, HashMap<String, EvaluatedArg>) -> Result<(), CommandError>
}

pub struct CommandDefinitionBuilder {
    name: String,
    arg_list: Vec<ArgType>,
    opt_args: HashMap<String, ArgType>,
    callback: Option<fn(Vec<EvaluatedArg>, HashMap<String, EvaluatedArg>) -> Result<(), CommandError>>
}

impl CommandDefinitionBuilder {
    pub fn new(name: &str) -> Self {
        Self { name: name.clone().to_string(), arg_list: vec![], opt_args: HashMap::new(), callback: None }
    }

    pub fn build(&self) -> Result<CommandDefinition, CommandError> {
        match self.callback {
            Some(cb) => Ok(CommandDefinition { 
                name: self.name.clone(), 
                arg_list: self.arg_list.to_owned(), 
                opt_args: self.opt_args.to_owned(), 
                callback: cb
            }),
            None => Err(CommandError::NoCallback(self.name.clone()))
        }
    }

    pub fn add_arg(&mut self, arg_type: ArgType) -> &mut Self {
        self.arg_list.push(arg_type);

        self
    }

    pub fn add_opt_arg(&mut self, arg_name: &str, arg_type: ArgType) -> &mut Self {
        self.opt_args.insert(arg_name.to_string(), arg_type);

        self
    }

    pub fn set_callback(&mut self, callback: fn(Vec<EvaluatedArg>, HashMap<String, EvaluatedArg>) -> Result<(), CommandError>) -> &mut Self {
        self.callback = Some(callback);

        self
    }
}

impl CommandDefinition {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn arg_list(&self) -> &Vec<ArgType> {
        &self.arg_list
    }
    pub fn opt_arg(&self, arg_name: &str) -> Option<&ArgType> {
        self.opt_args.get(arg_name)
    }
    pub fn evaluate(&self, eval_arg_list: Vec<EvaluatedArg>, eval_opt_args: HashMap<String,EvaluatedArg>) -> Result<(), CommandError> {
        (self.callback)(eval_arg_list, eval_opt_args)
    }
}