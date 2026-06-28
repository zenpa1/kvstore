// Entry point w/ TCP listener
use std::io::{Read, Write}; // traits for read/write to streams
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
mod db;
mod protocol;
use db::Database; // removes db:: when using Database
// use protocol::Command;

fn handle_connection(stream: TcpStream) {
    // ...
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
    for mut stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }

            Err(e) => {
                println!("Connection failed. Reason: {}", e.to_string());
            }
        }
    }

    // Create a buffer as raw TCP sends bytes, not strings
    let mut buffer: [u8; 512] = [0; 512];

    // Read the bytes by pulling data from the network into the buffer
    stream.read(&mut buffer); // Mutable reference: we want to read it, and possibly write to it
    // but how do I get the variable from the loop?

    // Convert raw bytes into a Rust UTF-8 String
    let user_input: String = String::from_utf8_lossy(&buffer[..]).to_string();
    // wait if it's a String::from method, why do I still need to convert it to .to_string()?

    // Parse and execute
    protocol::parse()

    Ok(())
}
