use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PageType {
    Home,
    About,
    Contact,
    NotFound,
    BlogHome,
    BlogPost,
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
