use std::{fmt::Display, io, error::Error};

use crate::commands::argument::ArgType;


#[derive(Debug)]
pub enum InputError{
    EmptyInput,
    NotACommand(String),
    NotAEnvVarAttrib(String),
    InterfaceError(io::Error)
}

#[derive(Debug)]
pub enum CommandError {
    MismatchArgument(String, ArgType),
    NoCallback(String),
    TooManyArguments(String, usize, usize),
    TooFewArguments(String, usize, usize),
    UnknownArgument(String),
    UnknownCommand(String)
}


#[derive(Debug)]
pub enum EnvVarError {
    Unset(String),
    Mismatch(String, String)
}


impl Error for InputError {}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Empty input"),
            Self::NotACommand(cmd) => write!(f, "{} isn't a command", cmd),
            Self::NotAEnvVarAttrib(env_var) => write!(f, "{} isn't a proper environment variable attribution", env_var),
            Self::InterfaceError(error) => write!(f, "Command-line input error {}", error)
        }
    }
}

impl Error for CommandError {}

impl Clone for CommandError {
    fn clone(&self) -> Self {
        match self {
            Self::MismatchArgument(arg0, arg1) => Self::MismatchArgument(arg0.clone(), arg1.clone()),
            Self::NoCallback(arg0) => Self::NoCallback(arg0.clone()),
            Self::TooManyArguments(arg0, arg1, arg2) => Self::TooManyArguments(arg0.clone(), arg1.clone(), arg2.clone()),
            Self::TooFewArguments(arg0, arg1, arg2) => Self::TooFewArguments(arg0.clone(), arg1.clone(), arg2.clone()),
            Self::UnknownArgument(arg0) => Self::UnknownArgument(arg0.clone()),
            Self::UnknownCommand(arg0) => Self::UnknownCommand(arg0.clone()),
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MismatchArgument(arg, arg_type) => write!(f, "Mismatch argument type, can convert {} into {}", arg, arg_type),
            Self::NoCallback(name) => write!(f, "No callback defined for {}", name),
            Self::TooManyArguments(name, expected, got)  => write!(f, "Too many arguments for {}, expected {}, but got {}", name, expected, got),
            Self::TooFewArguments(name, expected, got)  => write!(f, "Too few arguments for {}, expected {}, but got {}", name, expected, got),
            Self::UnknownArgument(arg) => write!(f, "Unknown argument {}", arg),
            Self::UnknownCommand(name) => write!(f, "Unknown command {}", name),
        }
    }
}

impl Clone for EnvVarError {
    fn clone(&self) -> Self {
        match self {
            Self::Unset(arg0) => Self::Unset(arg0.clone()),
            Self::Mismatch(arg0, arg1) => Self::Mismatch(arg0.clone(), arg1.clone()),
        }
    }
}

impl Display for EnvVarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unset(name) => write!(f, "Unset environment variable {}", name),
            Self::Mismatch(name, value) => write!(f, "{} environment variable with value {} can't be casted to desired type", name, value)
        }
    }
}