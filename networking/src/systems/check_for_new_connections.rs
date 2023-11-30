use std::sync::mpsc::*;

use crate::*;

/// Collects all new connections into a Vec and returns them.
pub(crate) fn check_for_new_connections(
    recv: &Receiver<NetworkConnection>,
) -> Vec<NetworkConnection> {
    let mut new_connections = Vec::<NetworkConnection>::new();

    loop {
        match recv.try_recv() {
            Ok(conn) => {
                debug!("New connection received from listener thread: {}", conn.id);
                new_connections.push(conn);
            }
            Err(err) => {
                if err == TryRecvError::Empty {
                    break;
                }

                warn!("Error communicating between threads: {}", err);
                break;
            }
        }
    }

    new_connections
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    use super::*;

    #[test]
    fn check_for_new_connections_returns_all_new_connections() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let (send, recv) = channel::<NetworkConnection>();

        send.send(NetworkConnection {
            id: Uuid::new_v4(),
            conn: TcpStream::connect(addr).unwrap(),
            gmcp: false,
            do_room: false,
        })
        .unwrap();

        send.send(NetworkConnection {
            id: Uuid::new_v4(),
            conn: TcpStream::connect(addr).unwrap(),
            gmcp: false,
            do_room: false,
        })
        .unwrap();

        let new_connections = check_for_new_connections(&recv);

        assert_eq!(new_connections.len(), 2);
    }
}
