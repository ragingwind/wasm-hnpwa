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
  pub content: Option<String>,
  pub deleted: Option<bool>,
  pub dead: Option<bool>,
  pub r#type: String,
  pub url: Option<String>,
  pub domain: Option<String>,
  pub comments: Vec<Item>,
  pub level: Option<u64>,
  pub comments_count: u64,
}
pub enum Endpoint {
  News,
  Newest,
  Ask,
  Show,
  Jobs,
}

impl Endpoint {
  pub fn as_str(&self, page: u32) -> String {
    match self {
      Endpoint::News => format!("https://api.hnpwa.com/v0/news/{}.json", page),
      Endpoint::Newest => format!("https://api.hnpwa.com/v0/newest/{}.json", page),
      Endpoint::Ask => format!("https://api.hnpwa.com/v0/ask/{}.json", page),
      Endpoint::Show => format!("https://api.hnpwa.com/v0/show/{}.json", page),
      Endpoint::Jobs => format!("https://api.hnpwa.com/v0/jobs/{}.json", page),
    }
  }

  pub fn get_url(item_type: &str, page: u32) -> String {
    format!("https://api.hnpwa.com/v0/{}/{}.json", item_type, page)
  }
}

#[derive(Clone)]
pub struct Store {
  pub news: Vec<News>,
  numbers: u32,
}

impl Store {
  pub fn new() -> Store {
    Store {
      news: Vec::new(),
      numbers: 0,
    }
  }

  pub fn get_endpoints(&self, page: u32) -> Vec<String> {
    vec![
      Endpoint::News.as_str(page).into(),
      Endpoint::Newest.as_str(page).into(),
      Endpoint::Ask.as_str(page).into(),
      Endpoint::Show.as_str(page).into(),
      Endpoint::Jobs.as_str(page).into(),
    ]
  }
}
