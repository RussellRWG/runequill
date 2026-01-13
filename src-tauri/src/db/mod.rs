use rusqlite::{params, Connection, Result};
use tauri::{AppHandle, Manager};

mod migrate;

pub struct Database {
    pub conn: Connection,
}

struct Scene {
    id: u32,
    title: String,
    preview: String,
    text: String,
    tags: Vec<String>,
    position: u32,
}

struct Entity {
    id: u32,
    title: String,
    description: String,
}

pub fn init(app: &AppHandle) -> Result<()> {
    let db_path = database_path(app)?;
    std::fs::create_dir_all(db_path.parent().unwrap())?;

    let conn = Connection::open(db_path)?;
    migrate::run(&conn)?;

    app.manage(Database { conn });

    Ok(())
}

fn database_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut dir = app
        .path()
        .app_data_dir()
        .ok_or("could not get app data directory")?;

    dir.push("project.db");
    Ok(dir)
}
