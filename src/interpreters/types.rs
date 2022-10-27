use std::str::FromStr;

use crate::{
    error::ChipError,
    models::interpreter::Interpreter
};

use crate::interpreters::interpreter::ChipInterpreter;

/// Intepreter type
#[derive(Debug, Clone, Copy)]
pub enum InterpreterType {
    Original
}

impl Default for InterpreterType {
    fn default() -> Self {
        Self::Original
    }
}

impl FromStr for InterpreterType {
    type Err = ChipError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let api = match s {
            "original" => Self::Original,
            _ => {
                return Err(ChipError::UseIntepreter);
            }
        };

        Ok(api)
    }
}

impl From<InterpreterType> for Box<dyn Interpreter> {
    fn from(it: InterpreterType) -> Self {
        match it {
            InterpreterType::Original => Box::new(ChipInterpreter::new())
        }
    }
}
