// Entry point w/ TCP listener
use std::io::{Read, Write}; // traits for read/write to streams
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
mod db;
mod protocol;
use db::Database; // removes db:: when using Database
use protocol::Command;

fn handle_connection(mut stream: TcpStream, db: &mut Database) {
    loop {
        // Create a buffer as raw TCP sends bytes, not strings
        let mut buffer: [u8; 512] = [0; 512];

        // Read the bytes by pulling data from the network into the buffer
        let bytes_read = match stream.read(&mut buffer) {
            Ok(n) => {
                // Return a usize representing how many bytes it read
                // If bytes_read is 0, the client disconnected.
                if n == 0 {
                    return;
                }
                n // Return the number of bytes out of the match block
            }
            Err(e) => {
                println!("Error: Could not read stream bytes. {}", e);
                return;
            }
        };

        // Convert raw bytes into a Rust UTF-8 String
        /*
        If all bytes are valid UTF-8 → returns a borrowed &str (no allocation).
        Else if invalid bytes are found → allocates a new String with replacements.

        A Cow is an Enum that can hold either BORROWED data or OWNED data.
        But in this case, we want an &str for our parser, so to_string() is inserted.

        We also use bytes_read to slice ONLY valid data, ignoring the null bytes
        In other words, from buffer(0) to bytes_read(n)
        */
        let user_input: &str = &String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        // Parse and execute
        let command: Command = protocol::parse(user_input);
        match command {
            Command::Get { key } => match db.get(&key) {
                Some(str) => {
                    // Format the string with a newline to make the terminal look cleaner
                    let response = format!("{}\n", str);

                    // Convert the formatted String into raw bytes and send
                    let _ = stream.write_all(response.as_bytes());
                    // let _ = evaluate, but throw the result away without assigning it to a var
                    // But why? write_all() returns a Result<(), std::io::Error>
                    // In prod, match the Result and log a warning to server console
                }
                None => {
                    let _ = stream.write_all(b"Error: KEY not found.\n");
                }
            },
            Command::Set { key, value } => {
                db.set(key, value);
                let _ = stream.write_all(b"SET successful.\n");
            }
            Command::Delete { key } => {
                db.delete(&key);
                let _ = stream.write_all(b"DELETE successful.\n");
            }
            Command::Ping => {
                let _ = stream.write_all(b"PONG\n");
            }
            Command::Unknown => {
                let _ = stream.write_all(b"Error: UNKNOWN command.\n");
            }
        }
    } // Loop for continuous commands
}

fn main() -> std::io::Result<()> {
    // Instantiate database
    let mut db: Database = Database::new("Classes");

    // Instantiate listener and bind to a socket
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8000")?; // propagates error

    // Assertion check to see if it is the same
    assert_eq!(
        listener.local_addr().unwrap(),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000))
    );

    // Accept connections and process serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream, &mut db);
            }

            Err(e) => {
                println!("Connection failed. Reason: {}", e.to_string());
            }
        }
    }

    Ok(()) // for TcpListener bind, meaning nothing/unit type/void in C++
    // In other words, "The program finished successfully, and I have nothing to give now."
}
