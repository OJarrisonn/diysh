use std::{fmt::Display, error::Error};

pub fn exception<E>(error: E)
where E: Error + Display + 'static {
    eprintln!("[ERROR] {}", error);
}