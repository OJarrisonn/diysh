use std::fmt::Display;

use crate::error::CommandError;

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

pub enum EvaluatedArg {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool)
}

impl ArgType {
    fn evaluate(&self, value: &str) -> Result<EvaluatedArg, CommandError> {
        match self {
            Self::Str => Ok(EvaluatedArg::Str(value.to_string())),
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