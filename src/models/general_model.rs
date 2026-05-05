use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PageType {
    Home,
    About,
    Contact,
    NotFound,
    BlogHome,
    BlogArticle,
    Topics,
    Resources,
}

#[allow(unused)]
#[derive(Serialize)]
pub struct Deleted {
    pub deleted: bool,
    pub updated_at: DateTime<Local>,
}

#[allow(unused)]
#[derive(Serialize)]
pub struct UpdatedAt {
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>, // The search query will be in a parameter named 'q'
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TitleError {
    pub error: String,
}

impl TitleError {
    pub fn new(error: String) -> TitleError {
        TitleError { error }
    }
}

#[derive(Debug, Deserialize, Serialize, SurrealValue)]
pub struct CountResult {
    pub count: u64,
}
