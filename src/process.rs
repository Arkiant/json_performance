use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CustomField {
    HashMapString(std::collections::HashMap<String, serde_json::Value>),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct Line {
    quantity: i64,
}

impl Line {
    pub fn new(quantity: i64) -> Line {
        Line { quantity: quantity }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub id: i64,
    pub name: String,
    pub custom_field: CustomField,
    pub custom_field2: CustomField,
    pub created_at: String,
    pub lines: Vec<Line>,
}

impl Activity {
    pub fn new(
        id: i64,
        name: String,
        custom_field: CustomField,
        custom_field2: CustomField,
        created_at: String,
        lines: Vec<Line>,
    ) -> Activity {
        Activity {
            id: id,
            name: name,
            custom_field: custom_field,
            custom_field2: custom_field2,
            created_at: created_at,
            lines: lines,
        }
    }
}
