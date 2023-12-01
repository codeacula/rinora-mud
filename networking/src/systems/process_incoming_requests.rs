use std::{
    sync::mpsc::{Receiver, RecvTimeoutError},
    time::Duration,
};

use shared::prelude::*;

use crate::{events::*, IncomingEvent, NetworkEventType};

pub(crate) fn process_incoming_requests(
    incoming_event_rx: NonSend<Receiver<IncomingEvent>>,
    mut user_connected_tx: EventWriter<UserConnectedEvent>,
    mut user_disconnected_tx: EventWriter<UserDisconnectedEvent>,
    mut user_provided_command_tx: EventWriter<UserProvidedCommandEvent>,
    mut user_provided_gmcp_tx: EventWriter<UserProvidedGmcpEvent>,
) {
    loop {
        let incoming_event = match incoming_event_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => event,
            Err(err) => {
                if err == RecvTimeoutError::Timeout {
                    break;
                }

                error!("Error receiving from incoming event: {}", err);
                break;
            }
        };

        match incoming_event.event_type {
            NetworkEventType::Connect => {
                user_connected_tx.send(UserConnectedEvent(incoming_event.id));
            }
            NetworkEventType::Disconnect => {
                user_disconnected_tx.send(UserDisconnectedEvent(incoming_event.id));
            }
            NetworkEventType::Text => {
                let command = match incoming_event.data {
                    Some(data) => String::from_utf8(data).unwrap(),
                    None => String::new(),
                };

                user_provided_command_tx.send(UserProvidedCommandEvent {
                    id: incoming_event.id,
                    command,
                })
            }
            NetworkEventType::Gmcp => user_provided_gmcp_tx.send(UserProvidedGmcpEvent {
                id: incoming_event.id,
                command: incoming_event.command.unwrap(),
                data: String::from_utf8_lossy(&incoming_event.data.unwrap()).to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    use crate::IncomingEvent;

    fn test_event_was_emitted<T: Event>(event_type: NetworkEventType) {
        let (incoming_event_tx, incoming_event_channel_rx) = channel::<IncomingEvent>();

        let mut app = App::new();
        app.add_event::<UserConnectedEvent>()
            .add_event::<UserDisconnectedEvent>()
            .add_event::<UserProvidedCommandEvent>()
            .add_event::<UserProvidedGmcpEvent>();

        app.insert_non_send_resource(incoming_event_channel_rx);

        let mut system_state: SystemState<(
            NonSend<Receiver<IncomingEvent>>,
            EventWriter<UserConnectedEvent>,
            EventWriter<UserDisconnectedEvent>,
            EventWriter<UserProvidedCommandEvent>,
            EventWriter<UserProvidedGmcpEvent>,
        )> = SystemState::new(&mut app.world);

        let (
            incoming_event_rx,
            user_connected_tx,
            user_disconnected_tx,
            user_provided_command_tx,
            user_provided_gmcp_tx,
        ) = system_state.get_mut(&mut app.world);

        incoming_event_tx
            .send(IncomingEvent {
                id: Uuid::new_v4(),
                command: None,
                data: None,
                event_type,
            })
            .unwrap();

        process_incoming_requests(
            incoming_event_rx,
            user_connected_tx,
            user_disconnected_tx,
            user_provided_command_tx,
            user_provided_gmcp_tx,
        );

        let event_handler = app.world.get_resource::<Events<T>>().unwrap();
        assert_eq!(event_handler.len(), 1);
    }

    #[test]
    fn it_emits_user_connected_event() {
        test_event_was_emitted::<UserConnectedEvent>(NetworkEventType::Connect);
    }

    #[test]
    fn it_emits_user_disconnected_event() {
        test_event_was_emitted::<UserDisconnectedEvent>(NetworkEventType::Disconnect);
    }

    #[test]
    fn it_emits_text_event() {
        let (incoming_event_tx, incoming_event_channel_rx) = channel::<IncomingEvent>();

        let mut app = App::new();
        app.add_event::<UserConnectedEvent>()
            .add_event::<UserDisconnectedEvent>()
            .add_event::<UserProvidedCommandEvent>()
            .add_event::<UserProvidedGmcpEvent>();

        app.insert_non_send_resource(incoming_event_channel_rx);

        let mut system_state: SystemState<(
            NonSend<Receiver<IncomingEvent>>,
            EventWriter<UserConnectedEvent>,
            EventWriter<UserDisconnectedEvent>,
            EventWriter<UserProvidedCommandEvent>,
            EventWriter<UserProvidedGmcpEvent>,
        )> = SystemState::new(&mut app.world);

        let (
            incoming_event_rx,
            user_connected_tx,
            user_disconnected_tx,
            user_provided_command_tx,
            user_provided_gmcp_tx,
        ) = system_state.get_mut(&mut app.world);

        incoming_event_tx
            .send(IncomingEvent {
                id: Uuid::new_v4(),
                command: None,
                data: Some(String::from("This is a test\r\n").into_bytes()),
                event_type: NetworkEventType::Text,
            })
            .unwrap();

        process_incoming_requests(
            incoming_event_rx,
            user_connected_tx,
            user_disconnected_tx,
            user_provided_command_tx,
            user_provided_gmcp_tx,
        );

        let event_handler = app
            .world
            .get_resource::<Events<UserProvidedCommandEvent>>()
            .unwrap();
        assert_eq!(event_handler.len(), 1);

        let (event, _) = event_handler.get_event(event_handler.oldest_id()).unwrap();
        assert_eq!(event.command, String::from("This is a test\r\n"));
    }

    #[test]
    fn it_emits_gmcp_event() {
        let (incoming_event_tx, incoming_event_channel_rx) = channel::<IncomingEvent>();

        let mut app = App::new();
        app.add_event::<UserConnectedEvent>()
            .add_event::<UserDisconnectedEvent>()
            .add_event::<UserProvidedCommandEvent>()
            .add_event::<UserProvidedGmcpEvent>();

        app.insert_non_send_resource(incoming_event_channel_rx);

        let mut system_state: SystemState<(
            NonSend<Receiver<IncomingEvent>>,
            EventWriter<UserConnectedEvent>,
            EventWriter<UserDisconnectedEvent>,
            EventWriter<UserProvidedCommandEvent>,
            EventWriter<UserProvidedGmcpEvent>,
        )> = SystemState::new(&mut app.world);

        let (
            incoming_event_rx,
            user_connected_tx,
            user_disconnected_tx,
            user_provided_command_tx,
            user_provided_gmcp_tx,
        ) = system_state.get_mut(&mut app.world);

        let command_name = String::from("Char.Skills.Get");
        let command_data = String::from("{\"group\":\"perception\",\"name\":\"Butts\"}");

        incoming_event_tx
            .send(IncomingEvent {
                id: Uuid::new_v4(),
                command: Some(command_name.clone()),
                data: Some(command_data.clone().into_bytes()),
                event_type: NetworkEventType::Gmcp,
            })
            .unwrap();

        process_incoming_requests(
            incoming_event_rx,
            user_connected_tx,
            user_disconnected_tx,
            user_provided_command_tx,
            user_provided_gmcp_tx,
        );

        let event_handler = app
            .world
            .get_resource::<Events<UserProvidedGmcpEvent>>()
            .unwrap();
        assert_eq!(event_handler.len(), 1);

        let (event, _) = event_handler.get_event(event_handler.oldest_id()).unwrap();
        assert_eq!(event.command, command_name);
        assert_eq!(event.data, command_data);
    }
}
