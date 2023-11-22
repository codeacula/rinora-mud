mod connection_processor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_turns_on_gmcp_when_client_responds_yes() {}
}

/*
fn z_handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let gmcp_enabled = false;

    loop {
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from stream");
        if bytes_read == 0 {
            break;
        }

        let input = String::from_utf8_lossy(&buffer[..bytes_read]);
        let input = input.trim();

        if gmcp_enabled && input.starts_with("GMCP") {
            // Handle GMCP command
            // ...
        } else {
            // Handle regular string input
            // ...
        }

        stream
            .write_all(b"Received input\n")
            .expect("Failed to write to stream");
    }
}

fn z_start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
 */
