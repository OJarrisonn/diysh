use std::fmt::Display;

use crate::{error::CommandError, inout::read::ArgToken};

#[derive(Debug)]
pub enum ArgType {
    Str,
    Int,
    Float,
    Bool
}

impl Display for ArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgType::Str => write!(f, "Str"),
            ArgType::Int => write!(f, "Int"),
            ArgType::Float => write!(f, "Float"),
            ArgType::Bool => write!(f, "Bool")
        }
    }
}

impl Clone for ArgType {
    fn clone(&self) -> Self {
        match self {
            Self::Str => Self::Str,
            Self::Int => Self::Int,
            Self::Float => Self::Float,
            Self::Bool => Self::Bool,
        }
    }
}

#[derive(Debug)]
pub enum EvaluatedArg {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool)
}

impl Clone for EvaluatedArg {
    fn clone(&self) -> Self {
        match self {
            Self::Str(arg0) => Self::Str(arg0.clone()),
            Self::Int(arg0) => Self::Int(arg0.clone()),
            Self::Float(arg0) => Self::Float(arg0.clone()),
            Self::Bool(arg0) => Self::Bool(arg0.clone()),
        }
    }
}

impl ArgType {
    pub fn evaluate(&self, value: &ArgToken) -> Result<EvaluatedArg, CommandError> {
        let value = value.to_string();
        match self {
            Self::Str => Ok(EvaluatedArg::Str(value)),
            Self::Int => match value.parse::<i32>() {
                Ok(int) => Ok(EvaluatedArg::Int(int)),
                Err(_) => Err(CommandError::MismatchArgument(value.to_string(), ArgType::Int))
            },
            Self::Float => match value.parse::<f32>() {
                Ok(float) => Ok(EvaluatedArg::Float(float)),
                Err(_) => Err(CommandError::MismatchArgument(value.to_string(), ArgType::Float))
            },
            Self::Bool => match value.parse::<bool>() { 
                Ok(b) => Ok(EvaluatedArg::Bool(b)),
                Err(_) => Err(CommandError::MismatchArgument(value.to_string(), ArgType::Bool))
            }        
        }
    }
}

impl EvaluatedArg {
    pub fn get_str(&self) -> Option<String> {
        match self {
            Self::Str(result) => Some(result.clone()),
            _ => None
        }
    }

    pub fn get_int(&self) -> Option<i32> {
        match self {
            Self::Int(result) => Some(*result),
            _ => None
        }
    }

    pub fn get_float(&self) -> Option<f32> {
        match self {
            Self::Float(result) => Some(*result),
            _ => None
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(result) => Some(*result),
            _ => None
        }
    }
}