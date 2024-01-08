use crate::DatabaseInterface;

#[derive(serde::Serialize, Debug, Clone)]
pub struct RoomData {
    name: String,
}

#[tauri::command]
pub async fn get_rooms(
    db_interface: tauri::State<'_, DatabaseInterface>,
) -> Result<Vec<RoomData>, ()> {
    let db_interface_guard = db_interface.0.lock().unwrap();
    let db_interface = db_interface_guard.as_ref().unwrap();

    let rooms = db_interface.locations.get_all_rooms();

    let result = match rooms {
        Ok(rooms) => rooms
            .into_iter()
            .map(|c: shared::prelude::RoomBundle| RoomData { name: c.name.0 })
            .collect::<Vec<RoomData>>(),
        Err(err) => {
            eprintln!("Error getting rooms: {}", err);
            return Err(());
        }
    };

    Ok(result)
}
