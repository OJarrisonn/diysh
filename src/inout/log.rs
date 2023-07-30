use std::{fmt::Display, fs::OpenOptions, io::{Write, self}, path::Path};

pub enum LogLevel {
    INFO,
    WARN,
    ERROR
}

pub fn log(file: &str, log_level: LogLevel, message: &str) {
    let to_log = format!("{} {}\n", log_level, message);
    
    match log_level {
        LogLevel::ERROR => eprintln!("{}", &to_log[..to_log.len()-1]),
        LogLevel::WARN => println!("{}", &to_log[..to_log.len()-1]),
        _ => {} 
    };

    OpenOptions::new().append(true).open(Path::new(file)).unwrap()
        .write(&to_log.as_bytes())
        .expect("Couldn't write to the log file!");
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::INFO => write!(f, "[INFO]"),
            LogLevel::WARN => write!(f, "[WARN]"),
            LogLevel::ERROR => write!(f, "[ERROR]")
        }
    }
}