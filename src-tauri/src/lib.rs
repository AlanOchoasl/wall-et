pub mod application;
pub mod commands;
pub mod domain;
pub mod infrastructure;

use std::path::PathBuf;

use tauri::Manager;

pub struct AppState {
    pub database_path: PathBuf,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let data_dir = app.path().app_data_dir()?;
            let database_path = infrastructure::database::resolve_path(&data_dir);
            infrastructure::database::open(&database_path)?;
            app.manage(AppState { database_path });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app_status::get_app_status
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
