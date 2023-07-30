use std::{collections::HashMap, io::{self, Write}, cmp::min, process, path::Path, fs::{self, OpenOptions}};

use chrono::Local;

use crate::{commands::{definition::CommandDefinition, argument::ArgType}, inout::{read, log::{LogLevel, self}}, error::CommandError};


pub struct Shell {
    prompt: String,
    command_registry: HashMap<String, CommandDefinition>,
    history: Vec<String>,
    do_sparse: bool,
    log_file: Option<String>
}



impl Shell {
    pub fn new() -> Self {
        Shell { 
            prompt: String::new(), 
            command_registry: HashMap::new(),
            history: vec![],
            do_sparse: false,
            log_file: None
        }
    }

    pub fn set_sparse(&mut self, do_sparse: bool) -> &mut Self {
        self.do_sparse = do_sparse;

        self
    }

    pub fn set_log_directory(&mut self, log_directory: &str) -> &mut Self {
        let path = Path::new(log_directory);

        if !path.is_dir() {
            if let Err(_) = fs::create_dir_all(path) {
                panic!("Couldn't create the log directory {}", log_directory);
            }
        }

        let mut filename = log_directory.to_string();

        filename.push_str(&format!("diysh-{}.log", Local::now().format("%Y-%m-%d-%H:%M:%S")));
    
        self.log_file = match OpenOptions::new()
            .write(true)
            .create(true)
            .open(&filename) {
                Ok(_) => Some(filename.clone()),
                Err(_) => None,
            };

        self
    }

    pub fn set_prompt(&mut self, p: &str) -> &mut Self {
        self.prompt = p.to_string();

        self
    }

    pub fn register_help(&mut self) -> &mut Self {
        self.register_command(
        CommandDefinition::new("help")
            .set_description("- Shows this page")
            .set_callback(|shell, _args| {
                shell.help();
            })
            .build()
        );

        self
    }
    pub fn register_history(&mut self) -> &mut Self {
        self.register_command(
            CommandDefinition::new("history")
            .add_arg(ArgType::Int)
            .set_description("len:int - Shows the list of the last len-th commands ran")
            .set_callback(|shell, args| {
                let len = args[0].get_int().unwrap();

                shell.history(len);
            })
            .build()
        );

        self
    }
    pub fn register_exit(&mut self) -> &mut Self {
        self.register_command(
            CommandDefinition::new("exit")
            .set_description("- Exists the program")
            .set_callback(|shell, _args| {
                shell.exit();
            })
            .build()
        );

        self
    }

    pub fn register_command(&mut self, definition: CommandDefinition) -> &mut Self {
        if !self.command_registry.contains_key(definition.name()) {
            self.command_registry.insert(definition.name().to_string(), definition.clone());
        }

        self
    }

    pub fn read_and_run(&mut self) {
        if self.prompt != "" {
            print!("{}", self.prompt);
            let _ = io::stdout().flush();
        }

        let line = read::read_line();

        self.log(LogLevel::INFO, &format!(">> {}", &line[..line.len()-1]));

        let tokens = read::get_tokens(&line);

        match tokens {
            Ok(token) => {
                match self.command_registry.get(&token.0.0).as_mut() {
                    Some(def) => {
                        match def.instantiate(self, token.1) {
                            Ok(inst) => inst.run(),
                            Err(e) => self.log(LogLevel::ERROR, &format!("{}", e))
                        }
                    }
                    None => { let e = CommandError::UnknownCommand(token.0.0); self.log(LogLevel::ERROR, &format!("{}", e)) }
                }
            }
            Err(e) => self.log(LogLevel::ERROR, &format!("{}", e)),
        }

        self.history.push(line.clone());

        if self.do_sparse {
            println!("");
        }
    }

    pub fn log(&self, log_level: LogLevel, message: &str) {
        if let Some(file) = &self.log_file {
            log::log(file, log_level, message);
        }
    }

    pub fn help(&self) {
        for def in self.command_registry.values() {
            println!("{} {}", def.name(), def.description())
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

        for i in (self.history.len() - len)..self.history.len() {
            print!("{}: {}", i, self.history[i]);
        }
    }
    
    pub fn exit(&self) {
        process::exit(0)
    }

}