use chrono::Local;

use crate::{shell::Shell, commands::{definition::CommandDefinition, argument::ArgType}, inout::log::LogLevel};

#[test]
fn time() {
    println!("{}", Local::now().format("%Y-%m-%d-%H:%M:%S") );
}

#[test]
fn shell_test() {
    let mut shell = Shell::new();

    shell
        .set_sparse(true)
        .set_prompt(">> ")
        .set_log_directory("/tmp/diysh/")
        .register_help()
        .register_history()
        .register_exit()
        
        .register_command( CommandDefinition::new("print")
            .set_description("text:str - Prints the specified text to the terminal")
            .add_arg(ArgType::Str)
            .set_callback(|shell, args| {
                let text = args[0].get_str().unwrap();

                println!("{}", &text);
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

                println!("{}", a + b);
                shell.log(LogLevel::INFO, &format!("The sum is {}", a + b));
            })
            .build()
        );

    loop {
        shell.read_and_run();
    }
}