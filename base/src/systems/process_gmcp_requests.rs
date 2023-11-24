use shared::prelude::*;

pub fn process_gmcp_requests(mut send_gmcp_data_rx: &EventReader<SendGmcpData>) {
    for ev in send_gmcp_data_rx.read() {
        
    }
}
