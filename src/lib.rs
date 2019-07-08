use std::cell::RefCell;

use futures::{future, Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CustomEvent, CustomEventInit, Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = console, js_name=log)]
  fn log_u32(a: u32);

  #[wasm_bindgen(js_namespace = console, js_name=log)]
  fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct News {
  pub id: u64,
  pub title: String,
  pub points: Option<u64>,
  pub user: Option<String>,
  pub time: u64,
  pub time_ago: String,
  pub comments_count: u32,
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

fn fetch(ep: &String) -> Promise {
  let mut opts = RequestInit::new();
  opts.method("GET");
  opts.mode(RequestMode::Cors);

  let request = Request::new_with_str_and_init(ep, &opts).unwrap();

  request.headers().set("Accept", "application/json").unwrap();

  let window = web_sys::window().unwrap();
  let request_promise = window.fetch_with_request(&request);
  let future = JsFuture::from(request_promise)
    .and_then(|resp_value| {
      assert!(resp_value.is_instance_of::<Response>());
      let resp: Response = resp_value.dyn_into().unwrap();
      resp.json()
    })
    .and_then(|json_value: Promise| JsFuture::from(json_value))
    .and_then(|json| future::ok(json));

  future_to_promise(future)
}

#[wasm_bindgen]
pub struct ClosureHandle(Closure<FnMut(JsValue)>);

#[wasm_bindgen]
pub fn prepare() -> ClosureHandle {
  console_error_panic_hook::set_once();
  let mut contorller = Controller::new();
  let cb = Closure::wrap(Box::new(move |json: JsValue| {
    contorller.latest = json.into_serde().unwrap();
    send_event();
  }) as Box<FnMut(JsValue)>);

  let url = String::from("https://api.hnpwa.com/v0/news/1.json");
  fetch(&url).then(&cb);

  ClosureHandle(cb)
}

#[derive(Serialize)]
pub struct Detail<T> {
  data: T,
}

fn send_event() {
  let detail = Detail { data: "name" };
  let d = JsValue::from_serde(&detail).unwrap();
  let mut i = CustomEventInit::new();
  i.detail(&d);

  let e = CustomEvent::new_with_event_init_dict("app", &i).unwrap();

  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();

  document.dispatch_event(&e).unwrap();
}

#[wasm_bindgen]
pub fn run() {
  send_event();
}
