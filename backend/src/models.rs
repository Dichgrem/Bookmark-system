use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: Option<i64>,
    pub name: String,
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub id: Option<i64>,
    pub title: String,
    pub url: String,
    pub icon: Option<String>,
    pub category_id: Option<i64>,
    #[serde(default)]
    pub sort_order: i32,
}
