use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Serialize)]
pub struct Deleted {
    pub deleted: bool,
    pub updated_at: DateTime<Local>,
}

#[derive(Serialize)]
pub struct UpdatedAt {
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub r: Option<u32>, // The search query will be in a parameter named 'q'
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

#[derive(Debug, Deserialize)]
pub struct CountResult {
    pub count: u64,
}
