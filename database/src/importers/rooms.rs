use bevy::prelude::*;
use mongodb::bson::doc;

use crate::prelude::DbInterface;

pub fn add_rooms_to_world(db_repo: Res<DbInterface>, mut commands: Commands) {
    let all_rooms_res = db_repo.rooms.rooms.find(doc! {}, None);

    let all_rooms = match all_rooms_res {
        Ok(rooms) => rooms,
        Err(e) => {
            error!("Unable to load rooms from query: {:?}", e);
            return;
        }
    };

    for room_res in all_rooms {
        let room = match room_res {
            Ok(room) => room,
            Err(e) => {
                error!("Unable to fetch room: {:?}", e);
                return;
            }
        };

        commands.spawn(room.to_game_room());
    }
}
