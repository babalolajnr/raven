use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:2525").unwrap();
    println!("SMTP Server listening on port 2525");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

/// Handles a client connection to the SMTP server.
///
/// This function reads commands from the client, processes them, and sends appropriate
/// responses back to the client. It supports basic SMTP commands such as HELO, MAIL FROM,
/// RCPT TO, DATA, and QUIT.
///
/// # Arguments
///
/// * `stream` - A `TcpStream` representing the client's connection.
#[allow(clippy::if_same_then_else)]
pub fn handle_client(mut stream: TcpStream) {
    // Send initial greeting
    write_response(&mut stream, "220 localhost SMTP server ready\r\n");

    let mut buffer = [0; 1024];
    let mut message = String::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n == 0 => break,
            Ok(n) => {
                let recieved = std::str::from_utf8(&buffer[..n]).unwrap_or("");
                println!("Recieved: {}", recieved.trim());

                if recieved.to_uppercase().starts_with("HELO") {
                    write_response(&mut stream, "250 Hello\r\n");
                } else if recieved.to_uppercase().starts_with("MAIL FROM:") {
                    write_response(&mut stream, "250 OK\r\n");
                } else if recieved.to_uppercase().starts_with("RCPT TO:") {
                    write_response(&mut stream, "250 OK\r\n");
                } else if recieved.to_uppercase().starts_with("DATA") {
                    write_response(
                        &mut stream,
                        "354 Start mail input; end data with <CRLF>.<CRLF>\r\n",
                    );
                } else if recieved.trim() == "." {
                    write_response(&mut stream, "250 OK\r\n");
                    println!("Message recieved:\n{}", message);
                    message.clear();
                } else if recieved.to_uppercase().starts_with("QUIT") {
                    write_response(&mut stream, "221 Bye!\r\n");
                    break;
                } else {
                    // Collect message body after DATA
                    if !message.is_empty() || recieved.trim() != "" {
                        message.push_str(recieved);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                break;
            }
        }
    }
}

fn write_response(stream: &mut TcpStream, response: &str) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
