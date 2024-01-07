// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use database::{get_db_interface, prelude::*, ConnectionSettings};

#[derive(serde::Serialize, Debug, Clone)]
struct ConnectionResult {
    success: bool,
    error: Option<String>,
}

#[tauri::command]
async fn connect_to_database(
    conn_settings: ConnectionSettings,
    db_interface: tauri::State<'_, DatabaseInterface>,
) -> Result<ConnectionResult, ()> {
    if db_interface.0.lock().unwrap().is_some() {
        return Ok(ConnectionResult {
            success: true,
            error: Some("Already connected to the database!".to_string()),
        });
    }

    let interface = match get_db_interface(conn_settings) {
        Ok(interface) => interface,
        Err(err) => {
            eprintln!("Error connecting to database: {}", err);
            return Ok(ConnectionResult {
                success: false,
                error: Some(err.to_string()),
            });
        }
    };

    db_interface.0.lock().unwrap().replace(interface);

    Ok(ConnectionResult {
        success: true,
        error: None,
    })
}

pub struct DatabaseInterface(Mutex<Option<DbInterface>>);

fn main() {
    let db_interface = DatabaseInterface(Default::default());

    tauri::Builder::default()
        .manage(db_interface)
        .invoke_handler(tauri::generate_handler![connect_to_database])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
