// Parser
#[derive(Debug)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
    Ping,
    Unknown, // fallback
}

pub fn parse(input: &str) -> Command {
    /*
    1. Clean the input with .trim()
    2. Split the text based on spaces with .split_whitespace()
    3. Collect into a Vector with .collect()
    */

    // Specify what collection it builds from this chain of methods
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    // Match the first word for the command
    if parts.len() > 0 {
        match parts[0] {
            // K and V lives in parts[1] and parts[2] respectively
            // GET[0] foo[1] = len(2)
            "GET" if parts.len() == 2 => Command::Get {
                key: parts[1].to_string(),
            },
            // SET[0] foo[1] bar[2] = len(3)
            "SET" if parts.len() == 3 => Command::Set {
                key: parts[1].to_string(),
                value: parts[2].to_string(),
            },
            // DELETE[0] foo[1] = len(2)
            "DELETE" if parts.len() == 2 => Command::Delete {
                key: parts[1].to_string(),
            },
            "PING" => Command::Ping,
            _ => Command::Unknown, // fallback
        }
    } else {
        Command::Unknown
    }
}
