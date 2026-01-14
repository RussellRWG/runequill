mod db;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tauri::command]
fn hello_world() -> String {
    "Hello from Rust 👋".to_string()
}

#[tauri::command]
fn list_scenes(state: tauri::State<db::Database>) -> Result<models::scene::SceneListDTO, String> {
    let conn = state
        .conn
        .lock()
        .map_err(|_| "Database connection lock poisoned".to_string())?;
    let scenes = db::list_scenes(&conn).map_err(|err: rusqlite::Error| err.to_string())?;

    Ok(models::scene::SceneListDTO { scenes })
}

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
            db::init(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![hello_world, list_scenes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
