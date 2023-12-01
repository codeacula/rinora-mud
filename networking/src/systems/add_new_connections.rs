use shared::prelude::*;
use std::sync::mpsc::{Receiver, Sender};

use crate::{IncomingEvent, NetworkConnection, NetworkEventType};

use super::check_for_new_connections::*;

/// Takes all new connections, adds them to the provided vector, and sends a connect event to the game.
pub(crate) fn add_new_connections(
    all_connections: &mut Vec<NetworkConnection>,
    between_threads_rx: &Receiver<NetworkConnection>,
    connection_event_tx: &Sender<IncomingEvent>,
) {
    let new_connections = check_for_new_connections(&between_threads_rx);

    for new_conn in new_connections {
        if let Err(err) = connection_event_tx.send(IncomingEvent {
            id: new_conn.id,
            command: None,
            data: None,
            event_type: NetworkEventType::Connect,
        }) {
            error!("Failed to send incoming event to game: {}", err);
            break;
        };
        all_connections.push(new_conn);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        net::{TcpListener, TcpStream},
        sync::mpsc::channel,
    };

    use shared::prelude::*;

    use crate::{systems::add_new_connections::*, IncomingEvent, NetworkConnection};

    #[test]
    fn sends_the_events_to_the_channel() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let (net_send, net_recv) = channel::<NetworkConnection>();
        let (inc_send, inc_recv) = channel::<IncomingEvent>();

        let conn1 = NetworkConnection {
            id: Uuid::new_v4(),
            conn: TcpStream::connect(addr).unwrap(),
            gmcp: false,
            do_room: false,
        };
        conn1.conn.set_nonblocking(true).unwrap();
        net_send.send(conn1).unwrap();

        let conn2 = NetworkConnection {
            id: Uuid::new_v4(),
            conn: TcpStream::connect(addr).unwrap(),
            gmcp: false,
            do_room: false,
        };
        conn2.conn.set_nonblocking(true).unwrap();
        net_send.send(conn2).unwrap();

        println!("Listening on {}", addr);
        let mut conn_vec = Vec::<NetworkConnection>::new();

        add_new_connections(&mut conn_vec, &net_recv, &inc_send);

        drop(listener);
        drop(inc_send);

        assert!(inc_recv.recv().is_ok());
        assert!(inc_recv.recv().is_ok());
        assert!(inc_recv.recv().is_err());
        assert!(conn_vec.len() == 2);
    }
}
