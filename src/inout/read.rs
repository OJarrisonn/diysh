use std::io;
use regex::Regex;

use crate::error::InputError;


#[derive(Debug)]
pub enum ArgToken {
    Literal(String),
    Flag(String)
}

#[derive(Debug)]
pub struct IdentifierToken(pub String);

// Read a line and return a tuple for building the command from the tokens or and error if the input can't be parsed to a command
pub fn get_tokens(line: &str) -> Result<(IdentifierToken, Vec<ArgToken>), InputError> {
    let mut raw_tokens = get_raw_tokens(line); // Gets the raw_token vector

    // Verifies if it isn't an empty input
    if raw_tokens.len() == 0 {
        return Err(InputError::EmptyInput);
    }

    // Verifies if the first element is a valid identifier
    if !is_identifier(&raw_tokens[0]) {
        return Err(InputError::NotACommand(raw_tokens[0].clone()));
    }

    // Gets the identifier
    let identifier = IdentifierToken(raw_tokens[0].clone());
    
    // Let the raw token list with only the arguments
    raw_tokens.remove(0);

    let mut args: Vec<ArgToken> = vec![];

    // Files the arg vector with the proper tokens
    for arg in raw_tokens {
        if is_flag(&arg) { args.push(ArgToken::Flag(arg[2..].to_string())) }
        else { args.push(ArgToken::Literal(arg)) }
    }

    Ok((identifier, args))
}

pub fn read_line() -> String {
    let mut buf = String::new();
    
    loop {
        match io::stdin().read_line(&mut buf) {
            Ok(_) => if buf.trim() != "" { break; } 
            Err(e) => println!("Error: {e}")
        }
    }

    buf
}

fn get_raw_tokens(raw_string: &str) -> Vec<String> {
    let mut raw_tokens: Vec<String> = vec![];
    let mut raw_token = String::new();
    let mut quoting = false;
    

    for c in raw_string.chars() {
        match c {
            '"' => {
                if quoting { 
                    raw_tokens.push(raw_token); 
                    raw_token = String::new(); 
                }
                quoting = !quoting; 
            },
            ' ' | '\n' | '\t' => {
                if quoting { 
                    raw_token.push(c);
                
                } else if raw_token.len() > 0 {
                    raw_tokens.push(raw_token); 
                    raw_token = String::new(); 
                
                } else {
                    raw_token = String::new(); 
                }
            },
            _ => raw_token.push(c)
        }
    }

    raw_tokens
}

fn is_identifier(text: &str) -> bool {
    match Regex::new(r"[a-z][a-zA-Z0-9]*").unwrap().captures(text) {
        None => false,
        _ => true
    }
}


fn is_flag(text: &str) -> bool {
    match Regex::new(r"--[a-z][a-zA-Z0-9]*").unwrap().captures(text) {
        None => false,
        _ => true
    }
}

impl ToString for ArgToken {
    fn to_string(&self) -> String {
        match self {
            ArgToken::Literal(text) => text.clone(),
            ArgToken::Flag(text) => text.clone()
        }
    }
}