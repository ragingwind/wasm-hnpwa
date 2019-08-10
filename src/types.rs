use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct News {
  pub id: u64,
  pub title: String,
  pub points: Option<u64>,
  pub user: Option<String>,
  pub time: u64,
  pub time_ago: String,
  pub comments_count: u64,
  pub r#type: String,
  pub url: String,
  pub domain: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
  pub id: u64,
  pub title: Option<String>,
  pub points: Option<u64>,
  pub user: Option<String>,
  pub time: u64,
  pub time_ago: String,
  pub content: String,
  pub deleted: Option<bool>,
  pub dead: Option<bool>,
  pub r#type: String,
  pub url: Option<String>,
  pub domain: Option<String>,
  pub comments: Vec<Item>,
  pub level: Option<u64>,
  pub comments_count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
  pub id: String,
  pub created: String,
  pub created_time: u64,
  pub karma: u64,
}

pub fn get_url(item_type: &str, page: u32) -> String {
  format!("https://api.hnpwa.com/v0/{}/{}.json", item_type, page)
}

pub fn to_static_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}
