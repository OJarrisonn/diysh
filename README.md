# dish - the Do-It SHell

Dish is a library which allows developers to create their own shell-like interface for their Rust programs.

## The Shell

Shell is the struct where one stores one's commands definitions and handles the input, output and error output. The shell can handle both command-line input and form-like inputs. Also, one can customize the shell's welcome message, prompt style and prompt header using environment variables.

One creates a Shell using the ShellBuilder helper struct, where one defines the Shell name, register the commands, and customize the appearance of the shell.