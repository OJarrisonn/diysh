use std::{fmt::Display, io};

use crate::commands::argument::ArgType;


pub trait DishError { }

#[derive(Debug)]
pub enum InputError{
    EmptyInput,
    NotACommand(String),
    InterfaceError(io::Error)
}

pub enum CommandError {
    MismatchArgument(String, ArgType),
    NoCallback(String)
}

impl DishError for InputError {}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Empty input"),
            Self::NotACommand(cmd) => write!(f, "{} isn't a command", cmd),
            Self::InterfaceError(error) => write!(f, "Command-line input error {}", error)
        }
    }
}

impl DishError for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MismatchArgument(arg, arg_type) => write!(f, "Mismatch argument type, can convert {} into {}", arg, arg_type),
            Self::NoCallback(name) => write!(f, "No callback defined for {}", name)
        }
    }
}