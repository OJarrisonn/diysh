pub mod shell;
pub mod inout;
pub mod error;
pub mod commands;

#[cfg(test)]
mod tests {
    use std::io;

    use crate::{commands::{ definition::CommandDefinition, argument::ArgType, status::Passed }, shell::{Shell, prompt::Prompt}};

    #[test]
    fn shell_test() {
        let mut shell = Shell::new();

        shell.set_welcome("Welcome")
            .set_prompt(Prompt::from(">> "))

            .register_command(CommandDefinition::new("print")
                .add_arg(ArgType::Str)
                .set_callback(|args| {
                    let text = args[0].get_str().unwrap();

                    println!("{}", text);
                    Box::new(Passed())
                }).build()
            )
            
            .register_command(CommandDefinition::new("parrot")
                .set_callback(|_args| {
                    let mut input = String::new();

                    io::stdin().read_line(&mut input).expect("The input should work");

                    println!("{}", input);
                    Box::new(Passed())
                }).build()
        
            );

        shell.start();

        loop {
            shell.read_and_run();
        }
    }

    
}
