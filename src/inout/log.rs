pub enum Log {
    Raw(String),
    Info(String)
}

pub fn log(message: Log) {
    match message {
        Log::Raw(text) => println!("{}", text),
        Log::Info(text) => println!("[INFO] {}", text)
    }
}