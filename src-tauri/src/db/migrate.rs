use rusqlite::Connection;

pub fn run(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS scenes (
        id INT PRIMARY KEY,
        name TEXT NOT NULL,
        text TEXT
        );

        CREATE TABLE IF NOT EXISTS entities (
        id INT PRIMARY KEY,
        category TEXT,
        sub_category TEXT,
        name TEXT NOT NULL,
        description TEXT
        );

        CREATE TABLE IF NOT EXISTS scene_entities (
        scene_id TEXT NOT NULL,
        entity_id TEXT NOT NULL,
        PRIMARY KEY (scene_id, entity_id),
        FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE,
        FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE
        );
        "#,
    )?;

    Ok(())
}
