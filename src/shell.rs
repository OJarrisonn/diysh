use std::{collections::HashMap, io::{self, Write}, cmp::min, process};

use crate::{commands::{definition::CommandDefinition, status::{Failed, CommandStatus}}, inout::{read, throw}, error::CommandError};

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

    pub fn help(&self) {
        for def in self.command_registry.values() {
            println!("{}: {}\n", def.name(), def.description())
        }
    }

    pub fn history(&self, len: i32) {
        let len = if len <= 0 { 
            self.history.len() 
        } else { 
            min(
                len.to_string().parse().unwrap(), 
                self.history.len()
            ) 
        };

        for i in ((self.history.len() - len)..0).rev() {
            println!("{}: {}", i, self.history[i]);
        }
    }
    
    pub fn exit(&self) {
        process::exit(0)
    }

    pub fn read_and_run(&mut self) -> Box<dyn CommandStatus>{
        if self.prompt_header.0 != "" {
            println!("{}", self.prompt_header);
        }

        if self.prompt.0 != "" {
            print!("{}", self.prompt);
            let _ = io::stdout().flush();
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
                    None => { let e = CommandError::UnknownCommand(token.0.0); throw::exception(e) }
                }
            }
            Err(e) => throw::exception(e),
        }
    }

}