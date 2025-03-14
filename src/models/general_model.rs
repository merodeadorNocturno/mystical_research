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
