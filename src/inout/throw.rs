use std::{fmt::Display, error::Error};

use crate::commands::status::{CommandStatus, Failed};

pub fn exception<E>(error: E) -> Box<dyn CommandStatus>
where E: Error + Display + 'static {
    eprintln!("[ERROR] {}", error);   

    Box::new(Failed(Box::new(error)))
}