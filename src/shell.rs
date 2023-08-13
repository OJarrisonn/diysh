use std::{collections::HashMap, io::{self, Write}, cmp::min, process, path::Path, fs::{self, OpenOptions}, str::FromStr};

use chrono::Local;

use crate::{commands::{definition::CommandDefinition, argument::ArgType}, inout::{read, log::{LogLevel, self}}, error::{CommandError, EnvVarError}};


pub struct Shell {
    command_registry: HashMap<String, CommandDefinition>,
    environment_registry: HashMap<String, String>,
    history: Vec<String>,
    do_sparse: bool,
    log_file: Option<String>
}



impl Shell {
    pub fn new() -> Self {
        Shell {  
            command_registry: HashMap::new(),
            environment_registry: HashMap::new(),
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
        self.set_env("SYSTEM_PROMPT_DEFINITION", p);

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
            .set_description("- Exits the program")
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

    pub fn get_env_var<T: FromStr>(&self, name: &str) -> Result<T, EnvVarError> {
        match self.environment_registry.get(name) {
            None => Err(EnvVarError::Unset(name.to_string())),

            Some(value) => match value.parse::<T>() {
                Ok(value) => Ok(value),
                Err(_) => Err(EnvVarError::Mismatch(name.to_string(), value.to_string())),
            }
        }
    }

    fn set_env(&mut self, name: &str, value: &str) {
        self.environment_registry.insert(name.to_string(), value.to_string());
    }


    pub fn register_env_var(&mut self, name: &str, value: &str) -> &mut Self {
        self.environment_registry.insert(name.to_string(), value.to_string());
    
        self
    }

    pub fn read_and_run(&mut self) {
        let prompt = self.get_env_var("SYSTEM_PROMPT_DEFINITION").unwrap();

        if prompt != "" {
            print!("{} ", read::replace_masks(prompt, &self.environment_registry));
            let _ = io::stdout().flush();
        }

        let line = read::read_line();//.trim().to_string();
        let line = read::replace_masks(line, &self.environment_registry);

        
        self.log(LogLevel::INFO, &format!(">> {}", &line[..line.len()-1]));


        if line.trim().starts_with("$") { // Verify if it's a environment variable operation
            let env_var = read::get_env_var(&line);

            match env_var {
                Ok((name, value)) => { let _ = self.set_env(&name, &value); },
                Err(e) => self.log(LogLevel::ERROR, &format!("{}", e)),
            }
        } else {
            let tokens = read::get_tokens(&line); // Tokenize the read line
        
            match tokens {
                Ok(token) => { // It's a command
                    match self.command_registry.get(&token.0.0).as_mut() { // Verify is the command is registered
                        Some(def) => {
                            match def.instantiate(self, token.1) { // Creates an instance of the command with the given arg list
                                Ok(inst) => inst.run(), // Runs the instance if it's alright
                                Err(e) => self.log(LogLevel::ERROR, &format!("{}", e)) // Throws an instantiation error
                            }
                        }
                        None => { let e = CommandError::UnknownCommand(token.0.0); self.log(LogLevel::ERROR, &format!("{}", e)) } // Throws an unknown command error
                    }
                }
                Err(e) => self.log(LogLevel::ERROR, &format!("{}", e)), // Throws an invalid input error
            }
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