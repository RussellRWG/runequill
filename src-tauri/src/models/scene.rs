use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SceneDTO {
    pub id: u32,
    pub name: String,
    pub text: Option<String>,
    pub position: u32,
}

#[derive(Debug, Serialize)]
pub struct SceneListDTO {
    pub scenes: Vec<SceneDTO>,
}
