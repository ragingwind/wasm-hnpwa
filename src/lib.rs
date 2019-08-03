use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};

use crate::element::Element;
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

pub mod element;

#[macro_use]
extern crate lazy_static;

use crate::event::send_event;
use crate::fetch::*;
// use crate::exit;
// use crate::Message;

pub fn exit(message: &str) {
  let v = wasm_bindgen::JsValue::from_str(&message.to_string());
  web_sys::console::exception_1(&v);
  std::process::abort();
}

pub enum ControllerMessage {
  AddItem(String),
}

pub enum ViewMessage {
  UpdateFilterButtons(String),
}

pub enum Message {
  Controller(ControllerMessage),
  View(ViewMessage),
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
}

#[wasm_bindgen]
// pub struct ClosureHandle(Closure<FnMut(JsValue)>);
pub struct ClosureHandle(Closure<FnMut()>);

#[derive(Clone)]
pub struct Store {
  news: Vec<News>,
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

pub struct View {
  body: Element,
  app: RefCell<Rc<App>>,
}

impl View {
  pub fn new(app: Rc<App>) -> View {
    let body = Element::qs("header").unwrap();
    View {
      body,
      app: RefCell::new(app),
    }
  }

  pub fn init(&mut self) {
    self.bind_body();
  }

  pub fn bind_body(&mut self) {
    console_log!("event binding");

    let app = self.app.clone();
    let handler = move |_| {
      console_log!("event");

      if let Ok(app) = &(app.try_borrow_mut()) {
        app.add_message(Message::Controller(ControllerMessage::AddItem(
          "1".to_string(),
        )));
      }
    };
    self.body.delegate("h1", "click", handler, false)
  }

  pub fn call(&mut self, method_name: ViewMessage) {
    use self::ViewMessage::*;
    match method_name {
      UpdateFilterButtons(route) => self.update_filter_buttons(&route),
    }
  }

  pub fn update_filter_buttons(&self, route: &str) {
    console_log!("update_filter_buttons");
  }
}

pub struct Controller {
  // news: Vec<News>,
// store: Store,
}

impl Controller {
  pub fn new() -> Controller {
    Controller {
      // news: vec![],
      // store,
    }
  }

  pub fn call(&mut self, method_name: ControllerMessage) {
    use self::ControllerMessage::*;
    match method_name {
      AddItem(title) => self.add_item(title),
    }
  }

  pub fn add_item(&self, title: String) {
    console_log!("title: {:?}", title);
    // let st: &Rc<Store> = &self.store;
    //  let sch: &Rc<Scheduler> = &sched;
    // self.store.get_news(Endpoint::News, 1);
  }
}

pub struct App {
  controller: Rc<RefCell<Option<Controller>>>,
  view: Rc<RefCell<Option<View>>>,
  events: RefCell<Vec<Message>>,
  running: RefCell<bool>,
}

impl App {
  pub fn new() -> App {
    App {
      controller: Rc::new(RefCell::new(None)),
      view: Rc::new(RefCell::new(None)),
      events: RefCell::new(Vec::new()),
      running: RefCell::new(false),
    }
  }

  pub fn set_controller(&self, controller: Controller) {
    if let Ok(mut controller_data) = self.controller.try_borrow_mut() {
      *controller_data = Some(controller);
    } else {
      exit("This might be a deadlock");
    }
  }

  pub fn set_view(&self, view: View) {
    let mut view_data = self.view.try_borrow_mut().unwrap();
    *view_data = Some(view);
  }

  pub fn add_message(&self, message: Message) {
    console_log!("add message");
    let running = self.running.try_borrow_mut().unwrap().clone();

    {
      let mut events = self.events.try_borrow_mut().unwrap();
      events.push(message);
    }

    {
      if !running {
        self.run();
      }
    }
  }

  /// Start the event loop, taking messages from the stack to run
  fn run(&self) {
    console_log!("run");
    {
      let events = self.events.try_borrow().unwrap();
      let events_len = events.len().clone();
      let mut running = self.running.try_borrow_mut().unwrap().clone();

      if events_len == 0 {
        running = false;
      } else {
        running = true;
      }
    }

    console_log!("next message");
    self.next_message();
  }

  fn next_message(&self) {
    // let events = self.events.try_borrow().unwrap();
    let mut running = self.running.try_borrow_mut().unwrap().clone();

    let event = {
      if let Ok(mut events) = self.events.try_borrow_mut() {
        Some(events.pop())
      } else {
        None
      }
    };

    if let Some(Some(event)) = event {
      match event {
        Message::Controller(e) => {
          let mut controller = self.controller.try_borrow_mut().unwrap();
          if let Some(ref mut controller) = *controller {
            controller.call(e)
          }
        }
        Message::View(e) => {}
      }
      self.run();
    } else if running == true {
      running = false;
    }
  }

  pub fn get_json(&self) {
    let fetch = || {
      let done = Closure::wrap(Box::new(move |json: JsValue| {
        let data: Vec<News> = json.into_serde().unwrap();

        let mut store_ref = store.lock().unwrap();
        store_ref.news.extend(data.iter().cloned());

        console_log!("get_json {:?}", store_ref.news[0]);
      }) as Box<FnMut(JsValue)>);

      Fetch::get_json(&Endpoint::News.as_str(1)).then(&done);
      done.forget();
    };

    fetch();
  }
}

lazy_static! {
  // static ref APP: Mutex<App> = Mutex::new(App::new());
  static ref store: Mutex<Store> = Mutex::new(Store::new());
}

#[wasm_bindgen]
pub fn app() {
  let app = Rc::new(App::new());
  let mut view = View::new(app.clone());
  let controller = Controller::new();

  // let rapp: &Rc<App> = &app;

  {
    view.init();
    app.set_view(view);
    app.set_controller(controller);
  }
  // rapp.add_message(Message::Controller(ControllerMessage::))
}

#[wasm_bindgen]
pub fn run() {
  console_error_panic_hook::set_once();
  console_log!("run1");
  app()
}
