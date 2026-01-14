use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

mod migrate;

pub struct Database {
    pub conn: Mutex<Connection>,
}

pub fn init(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = database_path(app)?;
    std::fs::create_dir_all(db_path.parent().unwrap())?;

    let conn = Connection::open(db_path)?;
    migrate::run(&conn)?;

    app.manage(Database {
        conn: Mutex::new(conn),
    });

    Ok(())
}

fn database_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut dir = app.path().app_data_dir()?;

    dir.push("project.db");
    Ok(dir)
}

pub fn list_scenes(conn: &Connection) -> rusqlite::Result<Vec<crate::models::scene::SceneDTO>> {
    let mut stmt = conn.prepare("SELECT id, name, text, position FROM scenes ORDER BY position")?;
    let rows = stmt.query_map([], |row| {
        Ok(crate::models::scene::SceneDTO {
            id: row.get(0)?,
            name: row.get(1)?,
            text: row.get(2)?,
            position: row.get(3)?,
        })
    })?;

    let mut scenes = Vec::new();
    for row in rows {
        scenes.push(row?);
    }

    Ok(scenes)
}

pub fn list_entities(conn: &Connection) -> rusqlite::Result<Vec<crate::models::entity::EntityDTO>> {
    let mut stmt =
        conn.prepare("SELECT id, name, description, category, sub_category FROM entities")?;
    let rows = stmt.query_map([], |row| {
        Ok(crate::models::entity::EntityDTO {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            category: row.get(3)?,
            sub_category: row.get(4)?,
        })
    })?;

    let mut entities = Vec::new();
    for row in rows {
        entities.push(row?);
    }

    Ok(entities)
}
