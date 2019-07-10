use std::cell::RefCell;

use futures::{future, Future};
use js_sys::{Function, Promise};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use serde_json::{Result, Value};

#[macro_use]
mod console;
mod event;
mod fetch;

use crate::event::send_event;
// use crate::fetch::*;

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

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Newss {

// }

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
pub struct ClosureHandle(Closure<FnMut(JsValue)>);

#[wasm_bindgen]
pub fn initialize() -> js_sys::Promise {
  let mut contorller = Controller::new();
  let cb = Closure::wrap(Box::new(move |json: JsValue| {
    contorller.latest = json.into_serde().unwrap();

    let detail = JsValue::from_serde(&Detail { data: "name" }).unwrap();
    send_event(&detail);
  }) as Box<FnMut(JsValue)>);

  let url = String::from("https://api.hnpwa.com/v0/news/1.json");
  let url2 = String::from("https://api.hnpwa.com/v0/news/1.json");
  // Fetch::get_json(&url).then(&cb);
  let pr: Promise = fetch(&url);
  let pr2: Promise = fetch(&url2);
  let pa = js_sys::Array::new();
  pa.push(&JsValue::from(pr));
  pa.push(&JsValue::from(pr2));
  // let future = JsFuture::from(pr)
  let res = JsFuture::from(Promise::all(&pa))
    .and_then(|resp_value| {
      console_log!("{:?}", resp_value);
      // assert!(resp_value.is_instance_of::<Response>());
      let arr: js_sys::Array = resp_value.dyn_into().unwrap();
      let resp = js_sys::Array::new();
      // let mut resp: [Promise; 2] = [
      //   Promise::resolve(&JsValue::NULL),
      //   Promise::resolve(&JsValue::NULL),
      // ];
      let mut i = 0;
      for v in &arr.values() {
        console_log!("v, {:?}", v);
        let rr: Response = v.unwrap().dyn_into().unwrap();
        // let v: Vec<News> = v.unwrap().into_serde().unwrap();
        resp.push(&rr.json().unwrap());

        // console_log!("resp, {:?}", resp[i]);
        i += 1;
        // console_log!("v, {:?}", v);
        // let n: Value = v.unwrap().into_serde().unwrap();
        // let r: Vec<News> = v.unwrap().into_serde().unwrap();
        // console_log!("arr1, {:?}", n);
      }

      future::ok(Promise::all(&resp))
      // let json: News = resp_value.into_serde().unwrap();
      // console_log!("arr, {:?}", arr.values());
      // future::ok(json)

      // let data = r#"
      //   {
      //       "name": "John Doe",
      //       "age": 43,
      //       "phones": [
      //           "+44 1234567",
      //           "+44 2345678"
      //       ]
      //   }"#;

      // let v: Value = serde_json::from_str(data).unwrap();
      // console_log!("arr1, {:?}", v);

      // future::ok("OK")
    })
    // .and_then(|json_value: Promise| JsFuture::from(json_value))
    .and_then(|json_value: Promise| JsFuture::from(json_value))
    .and_then(|json| {
      // console_log!("json, {:?}", json);

      let arr: js_sys::Array = json.dyn_into().unwrap();
      for v in &arr.values() {
        // console_log!("json, {:?}", v);
        let j: Vec<News> = v.unwrap().into_serde().unwrap();
        console_log!("json, {:?}", j[0].title);
      }

      future::ok(JsValue::from("OK"))
    });

  future_to_promise(res)

  // ClosureHandle(cb)
}

#[wasm_bindgen]
pub fn run() {
  console_error_panic_hook::set_once();
  console_log!("start");
}
