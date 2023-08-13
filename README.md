# diysh - the Do It Yourself SHell

diysh is a library which allows developers to create their own shell-like interface for their Rust programs.

# Creating a Shell

A shell is the text interface from where you can read the commands and log the info you need. In order to create a shell you must follow those steps:

```rust
let mut shell = Shell::new(); // Mandatory

shell.set_sparse(do_sparse: bool);

shell.set_prompt(prompt: &str); 

shell.register_command(command: CommandDefinition);

shell.register_help();

shell.register_history();

shell.register_exit();

shell.set_log_directory(path: &str);

shell.register_env_var(name:&str, value: &str)
```

Once your shell is created, you have access to methods such as:

```rust
shell.read_and_run();

shell.log(level: LogLevel, text: &str);

shell.get_env_var<T: FromStr>(name: &str) -> Result<T, CommandError>;

shell.help();

shell.history(len: usize);

shell.exit();
```
## Set Sparse

If set to true, will print an empty line after a command output in order to give some visual separation between commands.

```console
help
exit - Exits the program
history len - Shows the len-th last commands 
help - Shows this page
print text:str - Prints the specified text to the terminal

print "Hello World"
Hello World
```

## Set Prompt

Sets the text to be displayed before the user input. For example, setting it to ">> ", will give the result:

```console
>> print "Hello World"
Hello World
>> help
exit - Exits the program
history len:int - Shows the list of the last len-th commands ran 
help - Shows this page
print text:str - Prints the specified text to the terminal
```

Your prompt supports environment variables, so you can do cool things such like setting your prompt to: ```"$USER$ ~>>"```, and then the variable will be evaluated before the prompt gets printed to the screen. Actually, your prompt is stored in a environment variable called ```SYSTEM_PROMPT_DEFINITION```. Do not modify this variable on the runtime if you're using environment variables in your prompt. Doing so, the prompt will evaluate any environment variables when you set.

```console
ojarrisonn_ ~>> $USER=rust
rust ~>> $SYSTEM_PROMPT_DEFINITION=<$USER$> ~ 
<rust> ~ $USER=ojarrisonn_
<rust> ~ help
...
```

## Register Command

Probably the most important method. It's used to register new commands to your shell using a CommandDefinition. A CommandDefinition has: a name, a description, arguments and a callback function. Here's an example of a CommandDefinition of a print command:

```rust
let print_command = CommandDefinition::new("print") // Creates a empty command with given name
    .set_description("text:str - Prints the specified text to the terminal")
    .add_arg(ArgType::Str) // You can add positional arguments of Str, Int, Float and Bool, as many as you wish
    // Here you can both pass the pointer to a function or use a closure that will be called when this command is called
    // Your function must be fn(&Shell, &Vec<EvaluatedArg>)
    .set_callback(|shell, args| {
        let text = args[0].get_str().unwrap();

        println!("{}", text);

        Box::new(Passed())
    })
    .build() // Builds the command
```
The command names **must** be ```camelCase``` and contain just letters and numbers (but the name can't start with a number). It's preffered to command names be just a single short word. But you're free to create a command called ```myAwesomeCommandToDoSomethingAmazing``` even though it's not good for the user to type such a long command.

A description isn't mandatory, but it's recommended to help users to use your shell. A good description should inform the argument types and the a explanatory name with a full command description to tell the user what it does.

If the command takes some arguments, a good description would look like ```"arg_name:arg_type arg_name2:arg_type2 ... arg_nameN:arg_type - The description of what the command does"```. If it takes no arguments, just ```"- The description of what the command does"``` should be ok. It's good to remember that the argument names are just for helping users to understand their meaning, it has no real impact on the program itself. 

The ```add_arg``` method can be called as many times as you wish to add any of the avaliable ```ArgType```s.

Setting a callback is the most important thing about a command, you can create a command with no callback, but it's useless. The callback receives a reference to the running Shell and the EvaluatedArg vector with the values read from the input. 

### ArgType and EvaluatedArg
When specifying command arguments, you need to specify the type of the argument both on the command definition and when you use the argument inside the callback function. 

```ArgType``` is used to specify the type in the ```CommandDefinition```. Once defined, when the command is read and evaluated, you will receive a vector of ```EvaluatedArg``` is the same order that you defined in the definition. They both can be ```Str```, ```Int```, ```Float``` or ```Bool```.

So if you create the following command:

```rust
CommandDefinition::new("print")
    .add_arg(ArgType::Str)
    .add_arg(ArgType::Int)
    .add_arg(ArgType::Bool)
    .set_callback(|shell, args| { ... })
    .build()
```

```args``` will be a vector where ```args[0]``` has a ```EvaluatedArg::Str```, ```args[1]``` has a ```EvaluatedArg::Int``` and ```args[2]``` has a ```EvaluatedArg::Bool```. And inside the function, to get the proper value stored, just call ```args[0].get_str().unwrap()``` or ```args[1].get_int().unwrap()``` or ```args[2].get_bool().unwrap()```.

The methods ```get_str()```,```get_int()```, ```get_float()``` and ```get_bool()``` returns a ```Option``` and don't try casting, if you call ```get_int()``` on a ```EvaluatedArg::Float``` you'll receive a None instead of Some.

When passing arguments on the command line the ```Str``` can be unquoted if it has no spaces, other wise, use double quotes. ```Int``` are just regular numbers made of digits from 0 to 9. ```Float``` are numbers with a single '```.```' separating the integer and the decimal part. And finally, a ```Bool``` is an unquoted case-sensitive ```true``` or ```false```.

### Reading Environment Variables

Inside the callback, you can get the values of your environment variables by calling ```shell.get_env_var::<T>(name: &str)```. Pass it the name of your variable (just like ```USER```), and a type (just like ```String```).

```rust
let user = shell.get_env_var::<String>("USER");
```
It will return a ```Result<T, EnvVarError>```. If it's a ```Ok``` so no worries, but if it's an ```Err``` we have two scenarios:

1) [```Unset```] The variable wasn't setted, and that's why you may want to use the ```register_env_var``` method on the shell creation.
2) [```Mismatch```] The user setted the variable to some value that can't be parsed to the value that you wish. Maybe you should log an error and inform the user to set the variable to a proper value. It's open to you.

## Register Help, History and Exit Commands

Registers a ```help```, a ```history len:int``` and an ```exit``` command.

Here are the respective ```CommandDefinition```s:

```rust
CommandDefinition::new("help")
    .set_description("- Shows this page")
    .set_callback(|shell, _args| {
        shell.help();
    })
    .build();

CommandDefinition::new("history")
    .add_arg(ArgType::Int)
    .set_description("len:int - Shows the list of the last len-th commands ran")
    .set_callback(|shell, args| {
        let len = args[0].get_int().unwrap();

        shell.history(len);
    })
    .build();

CommandDefinition::new("exit")
    .set_description("- Exists the program")
    .set_callback(|shell, _args| {
        shell.exit();
    })
    .build()
```

It's good to know that ```help```, ```history``` and ```exit``` are public methods, so you can create your own definitions of those commands and still use our provided methods.

## Set Log Directory

Sets a directory where to write the logs

## Register Environment Variables

diysh has support to environment variables, and this command let you predefine some. It doesn't mean that all the environment variables that you wish to use need to be registered, but you can register some if you know that some of your commands need a special environment variable.

### Environment Variables

In diysh you can set and use your own environment variables. Those variables are reseted in every session. You can define some of them in your shell definition, but the user can also define their environment variables on the runtime (and also redefine all environment variables).

To create a environment variable on the runtime, just type in: ```$VAR_NAME=THE VALUE```. A variable assignment must begin with a ```$```, then a name (only uppercase letters and underscores), then a `=` and finally the value which can be anything that you can write in a single line.

```console
>> $USER=ojarrisonn_
>> $YEAR=2023
>> $DOES_IT_WORK=true
>> $MESSAGE=Hello World this is a string
```

Later you can collect those values using ```get_env_var``` inside some command callback and parse it to a Rust type (if possible).

A good remainder is that anything after the ```=``` will be passed as the value of the variable. So you can write strings with spaces and don't need to use quoatation marks. Also remember that The ```$``` isn't part of the name of the variable.

To use your variables in the shell, just wrap it with ```$...$``` and it will be evaluated as soon as you hit ```Enter```.

```console
>> $MESSAGE=Hello World
>> print "$MESSAGE$"
Hello World
>> $CMD=print
>> $CMD$ "$MESSAGE$"
Hello World
```

## Read and Run

This method is the one who asks for the user to insert a command. It's default behaviour is to print the defined prompt, wait for the user to type the input, try to parse the user input to a command and then run the respective callback passing the arguments passed by the user. Also this function will log every errors, warnings and infos.

The usual is to use read_and_run inside a loop.

## Log

diysh log system is kinda simple. You just need to call the method ```log``` for the current shell and pass it the ```LogLevel``` which can be: ```INFO```, ```WARN``` or ```ERROR``` and then pass a ```&str``` containg the desired message. Warnings and Errors get logged to the log file and to the screen, but infos only get logged to the log file.

# Full example

Here it's a full example of a shell that implements the default commands and a print and a sum command

```rust
fn main() {
    let mut shell = Shell::new(); // Creates the shell

    // Now, let's set it up
    shell
        .set_sparse(true) // Enable sparsing
        .set_prompt("$USER$ ~>>") // Set the prompt to one which uses an environment variable
        .set_log_directory("/tmp/diysh/") // Set the log folder
        .register_env_var("USER", "ojarrisonn_") // Registers the variable to be used in the prompt
        .register_help()
        .register_history()
        .register_exit()
        
        // Our print command
        .register_command( CommandDefinition::new("print")
            .set_description("text:str - Prints the specified text to the terminal")
            .add_arg(ArgType::Str)
            .set_callback(|shell, args| {
                let text = args[0].get_str().unwrap();

                shell.log(LogLevel::INFO, &text);
            })
            .build()
        )
        
        // Our sum command
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
        
        // A command that prints the value stored in the TO_PRINT variable
        .register_command(CommandDefinition::new("echoEnv")
            .set_description("- Prints the value stored in $TO_PRINT")
            .set_callback(|shell, _args| {
                match shell.get_env_var::<String>("TO_PRINT") {
                    Ok(text) => { println!("{}", text); shell.log(LogLevel::INFO, &text)},
                    Err(e) => shell.log(LogLevel::ERROR, &format!("{}", e)),
                }
            })
            .build()
    
        );

    loop {
        shell.read_and_run();
    }
}
```