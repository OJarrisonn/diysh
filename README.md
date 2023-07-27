# dish - the Do-It SHell

Dish is a library which allows developers to create their own shell-like interface for their Rust programs.

## Creating a shell

A shell is the text interface from where one can read the commands. In order to create a shell one must follow those steps:
```rust
let mut shell = Shell::new(); // Mandatory

shell.set_welcome("Welcome message"); // Optional, sets a message to be shown when one calls shell.start();

shell.set_prompt(Prompt::from(">> ")); // Recomended, sets a character sequence to indicate where the command is going to be written

shell.set_prompt_header(PromptHeader::from("Header::::::::::")); // Optional, sets a line to be displayed over the prompt, in order to separate the lines

shell.register_command(my_command); // Recommended: Registers a new CommandDefinition. One can call this method multiple times
```

Once one's shell is created, one has access to methods such as:

```rust
shell.start(); // Shows the welcome message

shell.read_and_run(); // Reads a command from the terminal and tries to parse it to some defined command

shell.help(); // Prints a help screen

shell.history(len: usize); // Prints the last "len"-th commands (0 will show all the saved history)

shell.exit(); // Exits the program
```

## Creating a command

A command can be created using the ```CommandDefinition``` type following those steps:

```rust
let my_command = CommandDefinition::new("my_command") // Creates a empty command with given name
    .add_arg(ArgType::Str) // You can add positional arguments of Str, Int, Float and Bool
    ...
    .set_description("arg0:str - My command description")
    // Here you can both pass the pointer to a function or use a closure that will be called when this command is called
    // Your function must be fn(&Shell, &Vec<EvaluatedArg>) -> Box<dyn CommandStatus>
    .set_callback(|shell, args| {
        // Here goes your callback function

        Box::new(Passed())
    })
    .build() // Builds the command
```

### ArgType and EvaluatedArg
When specifying command arguments, one needs to specify the type of the argument both on the command definition and when one uses the argument inside the callback function

```ArgType``` is used to specify the type in the ```CommandDefinition```. Once defined, when the command is parsed, one will receive a vector of ```EvaluatedArg``` is the same order that one defined in the definition.

So if one creates the following command:

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
```arg_name:arg_type arg_name:arg_type ... - A proper command description, if you wish to break line, use \n\t to keep some organization```

Argument names should be ```snake_case``` and argument types are: ```str```, ```int```, ```float``` and ```bool```.