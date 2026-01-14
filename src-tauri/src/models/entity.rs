use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EntityDTO {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub sub_category: String,
}

#[derive(Debug, Serialize)]
pub struct EntityListDTO {
    pub entities: Vec<EntityDTO>,
}
