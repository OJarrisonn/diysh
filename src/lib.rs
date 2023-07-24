pub mod shell;
pub mod inout;
pub mod error;
pub mod commands;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_input() {
        println!("{:?}", inout::read::read_as_tokens());
    }
}
