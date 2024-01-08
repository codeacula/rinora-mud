use crate::DatabaseInterface;

#[derive(serde::Serialize, Debug, Clone)]
pub struct CharacterData {
    name: String,
}

#[tauri::command]
pub async fn get_characters(
    db_interface: tauri::State<'_, DatabaseInterface>,
) -> Result<Vec<CharacterData>, ()> {
    let db_interface_guard = db_interface.0.lock().unwrap();
    let db_interface = db_interface_guard.as_ref().unwrap();

    let characters = db_interface.characters.get_all_characters();

    let result = match characters {
        Ok(characters) => characters
            .into_iter()
            .map(|c| CharacterData {
                name: c.display_name.0,
            })
            .collect::<Vec<CharacterData>>(),
        Err(err) => {
            eprintln!("Error getting characters: {}", err);
            return Err(());
        }
    };

    Ok(result)
}
