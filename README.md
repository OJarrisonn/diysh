# diysh - the Do It Yourself SHell

diysh is a library which allows developers to create their own shell-like interface for their Rust programs.

# Creating a Shell

A shell is the text interface from where you can read the commands and log the info you need. In order to create a shell you must follow those steps:

```rust
let mut shell = Shell::new(); // Mandatory

shell.set_sparse(do_sparse: bool);

shell.set_prompt(prompt: &str); 

shell.register_command(command: CommandDefinition);

shell.register_default_commands();

shell.set_log_file(path: &str);
```

Once your shell is created, you have access to methods such as:

```rust
shell.read_and_run();

shell.log(level: LogLevel, text: &str);

shell.help();

shell.history(len: usize);

shell.exit();
```
## Set Sparse

If set to true, will print an empty line after a command output in order to give some visual separation between commands.

```
help
exit - Exists the program
history len - Shows the len-th last commands 
help - Shows this page
print text:str - Prints the specified text to the terminal

print "Hello World"
Hello World
```

## Set Prompt

Sets the text to be displayed before the user input. For example, setting it to ">> ", will give the result:

```
>> print "Hello World"
Hello World
>> help
exit - Exists the program
history len - Shows the len-th last commands 
help - Shows this page
print text:str - Prints the specified text to the terminal
```

## Register Command

Probably the most important method. It's used to register new commands to your shell using a CommandDefinition. A CommandDefinition has: a name, a description, arguments and a callback function. Here's an example of a CommandDefinition of a print command:

```rust
let print_command = CommandDefinition::new("print") // Creates a empty command with given name
    .set_description("text:str - Prints the specified text to the terminal")
    .add_arg(ArgType::Str) // You can add positional arguments of Str, Int, Float and Bool, as many as you wish
    // Here you can both pass the pointer to a function or use a closure that will be called when this command is called
    // Your function must be fn(&Shell, &Vec<EvaluatedArg>) -> Box<dyn CommandStatus>
    .set_callback(|shell, args| {
        let text = args[0].get_str().unwrap();

        println!("{}", text);

        Box::new(Passed())
    })
    .build() // Builds the command
```

A description isn't mandatory, but it's recommended to help users to use your shell. 

The ```add_arg``` method can be called as many times as you wish to add any of the avaliable ```ArgType```s.

Setting a callback is the most important thing about a command, you can create a command with no callback, but it's useless. The callback receives a reference to the running Shell and the EvaluatedArg vector with the values read from the input. 

Also your callback can return any type that implements the ```CommandStatus``` trait. There are two implemented types: ```Passed()``` and ```Failed(Box<dyn Error>)```.

### ArgType and EvaluatedArg
When specifying command arguments, you need to specify the type of the argument both on the command definition and when you use the argument inside the callback function

```ArgType``` is used to specify the type in the ```CommandDefinition```. Once defined, when the command is parsed, you will receive a vector of ```EvaluatedArg``` is the same order that you defined in the definition.

So if you create the following command:

```rust
CommandDefinition::new("print")
    .add_arg(ArgType::Str)
    .add_arg(ArgType::Int)
    .add_arg(ArgType::Bool)
    .set_callback(|shell, args| { ... })
    .build()
```

```shell``` is a reference to the running shell, so you can access it's methods and functions inside the callback

```args``` will be a vector where ```args[0]``` has a ```EvaluatedArg::Str```, ```args[1]``` has a ```EvaluatedArg::Int``` and ```args[2]``` has a ```EvaluatedArg::Bool```. And inside the function, to get the proper value stored, just call ```args[0].get_str().unwrap()``` or ```args[1].get_int().unwrap()``` or ```args[2].get_bool().unwrap()```.

The methods ```get_str()```,```get_int()```, ```get_float()``` and ```get_bool()``` returns a ```Option``` and don't try casting, if you call ```get_int()``` on a ```EvaluatedArg::Float``` you'll receive a None instead of Some.

### Description
A command description is just a ```&'static str``` that will be shown by the ```help()``` method of the Shell struct.

A recommendation is that descriptions have the following format:
```arg_name:arg_type arg_name:arg_type ... - A proper command description```. If you wish to break line, use \n\t to keep some organization

Argument names should be ```snake_case``` and argument types are: ```str```, ```int```, ```float``` and ```bool```.

## Full example

Here it's a full example of a shell