use std::{collections::HashMap, io::{self, Write}};

use crate::{commands::definition::CommandDefinition, inout::{read, self, throw}, error::CommandError};

use self::prompt::{ Prompt, PromptHeader };

pub mod prompt;

pub struct Shell {
    welcome_message: String,
    prompt: Prompt,
    prompt_header: PromptHeader,
    command_registry: HashMap<String, CommandDefinition>,
    history: Vec<String>
}



impl Shell {
    pub fn new() -> Self {
        Shell { 
            welcome_message: String::new(), 
            prompt: Prompt(String::new()), 
            prompt_header: PromptHeader(String::new()), 
            command_registry: HashMap::new(),
            history: vec![]
        }
    }

    pub fn set_welcome(&mut self, message: &str) -> &mut Self {
        self.welcome_message = message.to_string();

        self
    }

    pub fn set_prompt(&mut self, p: Prompt) -> &mut Self {
        self.prompt = p;

        self
    }

    pub fn set_prompt_header(&mut self, ph: PromptHeader) -> &mut Self {
        self.prompt_header = ph;

        self
    }

    pub fn register_command(&mut self, definition: CommandDefinition) -> &mut Self {
        if !self.command_registry.contains_key(definition.name()) {
            self.command_registry.insert(definition.name().to_string(), definition.clone());
        }

        self
    }

    pub fn start(&self) {
        if self.welcome_message != "" {
            println!("{}", self.welcome_message);
        }
    }



    pub fn read_and_run(&mut self) {
        if self.prompt_header.0 != "" {
            println!("{}", self.prompt_header);
        }

        if self.prompt.0 != "" {
            print!("{}", self.prompt);
            io::stdout().flush();
        }

        let line = read::read_line();
        self.history.push(line.clone());

        let tokens = read::get_tokens(line);

        match tokens {
            Ok(token) => {
                match self.command_registry.get(&token.0.0) {
                    Some(def) => {
                        match def.instantiate(token.1) {
                            Ok(inst) => inst.run(),
                            Err(e) => throw::exception(e)
                        }
                    }
                    None => throw::exception(CommandError::UnknownCommand(token.0.0))
                }
            }
            Err(e) => throw::exception(e),
        }
    }

}