use std::cell::RefCell;

use futures::{future, Future};
use js_sys::{Array, Promise};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;

#[macro_use]
mod console;
mod event;
mod fetch;

use crate::event::send_event;
use crate::fetch::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = window)]
  pub fn fetch(url: &str) -> Promise;
}

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

pub struct Controller {
  latest: RefCell<Vec<News>>,
}

impl Controller {
  pub fn new() -> Controller {
    Controller {
      latest: RefCell::new(Vec::new()),
    }
  }

  pub fn starts() {
    console_log!("controller starts()");
  }
}

#[derive(Serialize)]
struct Detail<T> {
  data: T,
}

#[wasm_bindgen]
pub fn initialize() -> Promise {
  let urls: Vec<String> = vec![
    String::from("https://api.hnpwa.com/v0/news/1.json"),
    String::from("https://api.hnpwa.com/v0/newest/1.json"),
    String::from("https://api.hnpwa.com/v0/ask/1.json"),
    String::from("https://api.hnpwa.com/v0/show/1.json"),
    String::from("https://api.hnpwa.com/v0/jobs/1.json"),
  ];

  let future = Fetch::get_jsons(&urls).then(|jsons_p| {
    let jsons: Array = jsons_p.unwrap().dyn_into().unwrap();
    for json in &jsons.values() {
      let j: Vec<News> = json.unwrap().into_serde().unwrap();
      console_log!("json, {:?}", j[0].title);
    }

    future::ok(JsValue::from(jsons))
  });

  future_to_promise(future)
}

#[wasm_bindgen]
pub fn run() {
  console_error_panic_hook::set_once();
  console_log!("run");
}
