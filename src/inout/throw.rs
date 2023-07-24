use std::fmt::Display;

use crate::error::DishError;

pub fn exception<E>(error: E)
where E: DishError + Display {
    eprintln!("[ERROR] {}", error);   
}