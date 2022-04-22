use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub id_list: Vec<String>,
    pub name: String,
}

impl Subject {
    pub fn new(name: String, id_list: Vec<String>) -> Self {
        Subject { id_list, name }
    }
}
