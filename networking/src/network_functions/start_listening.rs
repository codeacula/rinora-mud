use std::{env, io::Write, net::TcpListener, sync::mpsc::*};

use shared::prelude::*;

use crate::constants::*;
use crate::NetworkConnection;

pub(crate) fn start_listening(between_threads_tx: Sender<NetworkConnection>) {
    // This is put into a separate thread because it blocks on the listener, and we don't want that to block
    // listening to the currently connected clients. I don't want to make the listener non-blocking because I don't
    // want to write error handling for that.
    let server_host = env::var("SERVER_HOST").unwrap_or(String::from("127.0.0.1"));
    let server_port = env::var("SERVER_PORT").unwrap_or(String::from("23"));

    let tcp_listener = TcpListener::bind(format!("{server_host}:{server_port}"))
        .expect("Error starting TCP listener");

    info!("Listening on {server_host}:{server_port}");

    for connection_result in tcp_listener.incoming() {
        let mut connection = match connection_result {
            Ok(conn) => conn,
            Err(err) => {
                error!("Error accepting connection: {}", err);
                break;
            }
        };

        if let Err(err) = connection.set_nonblocking(true) {
            error!("Failed to set to non-blocking: {}", err);
            break;
        }

        let err = connection.write_all(GREETING.as_bytes());

        if err.is_err() {
            let message = err.unwrap_err().to_string();
            error!("Failed to write greeting, closing connection: {}", message);
            break;
        }

        if let Err(err) = connection.write_all(GREETING.as_bytes()) {
            error!("Failed to write greeting, closing connection: {}", err);
            break;
        };

        if let Err(err) = connection.write_all(&[IAC, WILL, GMCP]) {
            error!(
                "Failed to write GMCP negotiation, closing connection: {}",
                err
            );
            break;
        };

        if let Err(err) = between_threads_tx.send(NetworkConnection {
            id: Uuid::new_v4(),
            conn: connection,
            gmcp: false,
            do_room: false,
        }) {
            error!("Failed to send connection to managing thread: {}", err);
            break;
        };
    }
}

#[cfg(test)]
mod tests {
    use std::{env, io::Read, net::TcpStream, sync::mpsc::*, thread};

    use crate::{constants::*, NetworkConnection};

    use super::start_listening;

    fn get_channel() -> Sender<NetworkConnection> {
        let (send, _) = channel::<NetworkConnection>();
        send
    }

    #[test]
    fn it_opens_to_default_ports() {
        let handle = thread::spawn(move || start_listening(get_channel()));
        let stream = TcpStream::connect("127.0.0.1:23");
        assert!(stream.is_ok());
        drop(handle);
    }

    #[test]
    fn it_opens_to_env_ports() {
        env::set_var("SERVER_HOST", "localhost");
        env::set_var("SERVER_PORT", "2323");

        let handle = thread::spawn(move || start_listening(get_channel()));
        let stream = TcpStream::connect("localhost:2323");

        assert!(stream.is_ok());
        drop(handle);
    }

    #[test]
    fn it_sends_initial_data() {
        let handle = thread::spawn(move || start_listening(get_channel()));
        let stream = TcpStream::connect("127.0.0.1:23");
        assert!(stream.is_ok());

        let mut buf = Vec::<u8>::new();
        stream.unwrap().read_to_end(&mut buf).unwrap();

        let mut test_vec = super::GREETING.as_bytes().to_vec();
        test_vec.push(IAC);
        test_vec.push(WILL);
        test_vec.push(GMCP);

        assert_eq!(buf, test_vec);

        drop(handle);
    }
}
