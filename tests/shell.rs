use diysh::{shell::Shell, commands::{definition::CommandDefinition, argument::ArgType}, inout::log::LogLevel};

fn main() {
    let mut shell = Shell::new();

    shell
        .set_sparse(true)
        .set_prompt("$USER$ ~>>")
        .set_log_directory("/tmp/diysh/")
        .register_env_var("USER", "ojarrisonn_")
        .register_help()
        .register_history()
        .register_exit()
        
        .register_command( CommandDefinition::new("print")
            .set_description("text:str - Prints the specified text to the terminal")
            .add_arg(ArgType::Str)
            .set_callback(|shell, args| {
                let text = args[0].get_str().unwrap();

                shell.log(LogLevel::INFO, &text);
            })
            .build()
        )
        
        .register_command( CommandDefinition::new("sum")
            .set_description("a:int b:int - Prints the result of the sum of a + b")
            .add_arg(ArgType::Int)
            .add_arg(ArgType::Int)
            .set_callback(|shell, args| {
                let a = args[0].get_int().unwrap();
                let b = args[1].get_int().unwrap();

                shell.log(LogLevel::INFO, &format!("The sum is {}", a + b));
            })
            .build()
        )
        
        .register_command(CommandDefinition::new("echoEnv")
            .set_description("- Prints the value stored in $TO_PRINT")
            .set_callback(|shell, _args| {
                match shell.get_env_var::<String>("TO_PRINT") {
                    Ok(text) => shell.log(LogLevel::INFO, &text),
                    Err(e) => shell.log(LogLevel::ERROR, &format!("{}", e)),
                }
            })
            .build()
    
        );

    loop {
        shell.read_and_run();
    }
}