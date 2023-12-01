use shared::prelude::*;
use std::sync::mpsc::*;

use crate::{IncomingEvent, NetworkConnection, NetworkEventType};

pub(crate) fn shutdown_connection(
    conn_id: Uuid,
    all_connections: &mut Vec<NetworkConnection>,
    incoming_event_tx: &Sender<IncomingEvent>,
) {
    // Send the connection dropped event to the game because we can't write to them anymore
    if let Err(err) = incoming_event_tx.send(IncomingEvent {
        id: conn_id,
        command: None,
        data: None,
        event_type: NetworkEventType::Disconnect,
    }) {
        error!("Failed to send connection dropped event: {:?}", err);
        return;
    };

    // Connection closed
    let item_pos = all_connections
        .into_iter()
        .position(|c| c.id == conn_id)
        .unwrap();
    let network_connection = all_connections.remove(item_pos);

    drop(network_connection.conn);
}

#[cfg(test)]
mod tests {
    use std::net::{TcpListener, TcpStream};

    use super::*;

    #[test]
    fn it_removes_the_connection_from_the_vec() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (inc_send, inc_recv) = get_channels::<IncomingEvent>();

        let conn1id = Uuid::new_v4();
        let conn1 = NetworkConnection {
            id: conn1id,
            conn: TcpStream::connect(addr).unwrap(),
            gmcp: false,
            do_room: false,
        };

        let conn2id = Uuid::new_v4();
        let conn2 = NetworkConnection {
            id: conn2id,
            conn: TcpStream::connect(addr).unwrap(),
            gmcp: false,
            do_room: false,
        };

        let mut all_connections = vec![conn1, conn2];

        shutdown_connection(conn2id, &mut all_connections, &inc_send);

        assert!(all_connections.len() == 1);
        assert!(all_connections[0].id == conn1id);

        let event = inc_recv.recv().unwrap();
        assert!(event.id == conn2id);
        assert!(event.event_type == NetworkEventType::Disconnect);
    }
}
