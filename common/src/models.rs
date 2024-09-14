use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activity {
    pub id: String,
    pub name: String,
    pub status: String,
    pub url: String,
    pub display: bool
}
// add like versions maybe?