pub mod smtp;

use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn run_client() {
    match TcpStream::connect("127.0.0.1:2525") {
        Ok(mut stream) => {
            println!("Connected to SMTP server!");

            let mut buffer = [0; 1024];

            // Read the server's initial greeting
            let bytes_read = stream.read(&mut buffer).unwrap();
            println!("Server: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

            // SMTP conversion
            let commands = [
                "HELO client.example.com\r\n",
                "MAIL FROM:<sender@example.com>\r\n",
                "RCPT TO:<recipient@example.com>\r\n",
                "DATA\r\n",
                "Subject: Test Email\r\nThis is a test email.\r\n.\r\n",
                "QUIT\r\n",
            ];

            commands.iter().for_each(|cmd| {
                println!("Client: {}", cmd.trim());
                stream.write_all(cmd.as_bytes()).unwrap();

                buffer = [0; 1024];
                let bytes_read = stream.read(&mut buffer).unwrap();
                println!("Server: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
            });
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}
