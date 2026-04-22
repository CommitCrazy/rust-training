use std::num::ParseIntError;
use thiserror::Error;
fn main() {
    let port_inputs = vec!["", "hello", "80", "8080"];

    for port_input in port_inputs {
        match parse_port(port_input) {
            Ok(port) => {
                println!("port -> {}", port);
            }
            Err(e) => {
                println!("error -> {}", e);
            }
        }
    }
}

fn parse_port(s: &str) -> Result<u16, CustomError> {
    if s.is_empty() {
        return Err(CustomError::EmptyInput)
    }

    let port = s.parse::<u16>()?;
    if port < 1024 {
        return Err(CustomError::PortBelow { port })
    }
    Ok(port)
}

#[derive(Debug, Error)]
enum CustomError {
    #[error("empty input")]
    EmptyInput,

    #[error("invalid number: {0}")]
    ParsingError(#[from] ParseIntError),

    #[error("port {port} is privileged")]
    PortBelow { port: u16 },
}
