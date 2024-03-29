use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationDTO {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
}

