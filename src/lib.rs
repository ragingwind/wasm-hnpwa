use std::cell::RefCell;
use std::rc::{Rc, Weak};

use futures::{future, Future};
use js_sys::{Array, Promise};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[macro_use]
mod console;
mod event;
mod fetch;

use crate::event::send_event;
use crate::fetch::*;

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

#[derive(Clone)]
pub struct Store {
  news: RefCell<Vec<News>>,
  newest: RefCell<Vec<News>>,
  ask: RefCell<Vec<News>>,
  show: RefCell<Vec<News>>,
  jobs: RefCell<Vec<News>>,
}

enum Endpoint {
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
}

impl Store {
  pub fn new() -> Store {
    Store {
      news: RefCell::new(Vec::new()),
      newest: RefCell::new(Vec::new()),
      ask: RefCell::new(Vec::new()),
      show: RefCell::new(Vec::new()),
      jobs: RefCell::new(Vec::new()),
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

  pub fn update(&self, page: u32) -> Promise {
    let future = Fetch::get_jsons(&self.get_endpoints(page)).then(|jsons_p| {
      let jsons: Array = jsons_p.unwrap().dyn_into().unwrap();
      for json in &jsons.values() {
        let j: Vec<News> = json.unwrap().into_serde().unwrap();
        console_log!("json, {:?}", j[0].title);
      }

      future::ok(JsValue::from(jsons))
    });

    future_to_promise(future)
  }
}

pub struct View {}

impl View {
  pub fn new() -> View {
    View {}
  }
}

pub struct Controller {}

impl Controller {
  pub fn new() -> Controller {
    Controller {}
  }
}

pub struct App {
  controller: RefCell<Rc<Controller>>,
  store: RefCell<Rc<Store>>,
  view: RefCell<Rc<View>>,
  active_route: String,
}

impl App {
  pub fn new(controller: Rc<Controller>, store: Rc<Store>, view: Rc<View>) -> App {
    App {
      controller: RefCell::new(controller),
      store: RefCell::new(store),
      view: RefCell::new(view),
      active_route: "".into(),
    }
  }

  pub fn init(&self) -> Promise {
    if let Ok(store) = self.store.try_borrow_mut() {
      store.update(1)
    } else {
      Promise::reject(&JsValue::from_str("failed"))
    }
  }
}

#[wasm_bindgen]
pub struct ClosureHandle(Closure<FnMut(JsValue)>);

#[wasm_bindgen]
pub fn start() {
  let store = Rc::new(Store::new());
  let view = Rc::new(View::new());
  let controller = Rc::new(Controller::new());
  let app: App = App::new(controller.clone(), store.clone(), view.clone());

  let done = Closure::wrap(Box::new(|v: JsValue| {
    console_log!("done {:?}", v);
  }) as Box<FnMut(JsValue)>);

  app.init().then(&done);

  done.forget();
}

#[wasm_bindgen]
pub fn run() {
  console_error_panic_hook::set_once();
  console_log!("run");
}
