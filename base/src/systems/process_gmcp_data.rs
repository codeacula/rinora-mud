use shared::prelude::*;

use crate::{gmcp::*, resources::*};

pub fn process_gmcp_data(
    query: Query<&UserSessionData>,
    mut gmcp_data_rx: EventReader<SendGmcpData>,
    mut outgoing_queue: ResMut<OutgoingQueue>,
) {
    for gmcp_data in gmcp_data_rx.read() {
        let user = match query.get(gmcp_data.entity) {
            // Is an entity that isn't a user, like an NPC
            Err(_) => continue,
            Ok(user) => user,
        };

        // Make sure we send it in the right format
        let to_send = format!("{} {}", gmcp_data.command_name, gmcp_data.data);

        let mut to_send_bytes = to_send.as_bytes().to_vec();

        // Need to add telnet subnegotiation start and end
        to_send_bytes.insert(0, IAC);
        to_send_bytes.insert(1, SB);
        to_send_bytes.insert(2, GMCP);

        to_send_bytes.push(IAC);
        to_send_bytes.push(SE);

        outgoing_queue.send_gmcp(user.connection, to_send_bytes);
    }
}
