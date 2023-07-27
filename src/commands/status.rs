use std::error::Error;

pub trait CommandStatus { }

pub struct Passed();
pub struct Failed(pub Box<dyn Error>);

impl CommandStatus for Passed {}
impl CommandStatus for Failed {}