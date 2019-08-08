use super::console::*;
use crate::app::{App, Message};
use crate::fetch::*;
use crate::store::*;
use crate::view::ViewMessage;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
  static ref STORE: Mutex<Store> = Mutex::new(Store::new());
}

pub struct Controller {
  app: RefCell<Rc<App>>,
  page: u32,
}

pub enum ControllerMessage {
  GetNews(&'static str, u32),
  ChangePage(&'static str),
}

fn string_to_static_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}

impl Controller {
  pub fn new(app: Rc<App>) -> Controller {
    Controller {
      app: RefCell::new(app),
      page: 1,
    }
  }

  pub fn call(&mut self, method_name: ControllerMessage) {
    use self::ControllerMessage::*;
    match method_name {
      GetNews(item_name, page) => self.get_news(item_name, page),
      ChangePage(hash) => self.change_page(hash),
    }
  }

  fn change_page(&self, hash: &'static str) {
    let hash = hash.trim_start_matches("#/");
    let v: Vec<&str> = string_to_static_str(hash.to_string()).split("&").collect();
    let pathname = v[0];
    let index = v[1].parse::<u32>().unwrap();

    console_log!("{} {}", pathname, index);
    match pathname {
      "news" | "newest" | "ask" | "show" | "jobs" => self.get_news(pathname, index),
      "detail" => self.get_detail(pathname, index),
      _ => self.get_news("news", 1),
    }
  }

  pub fn get_detail(&self, pathname: &'static str, index: u32) {
    let app = self.app.clone();
    let fetch = move || {
      let done = Closure::wrap(Box::new(move |json: JsValue| {
        let item: Item = json.into_serde().unwrap();
        console_log!("data: {:?}", item);

        if let Ok(app) = &(app.try_borrow_mut()) {
          app.add_message(Message::View(ViewMessage::ShowDetail(
            item, pathname, index,
          )));
        }
      }) as Box<FnMut(JsValue)>);

      let endpoint = format!("https://api.hnpwa.com/v0/item/{}.json", index);
      Fetch::get_json(&endpoint).then(&done);
      done.forget();
    };

    fetch();
  }

  pub fn get_news(&self, pathname: &'static str, index: u32) {
    let app = self.app.clone();
    let fetch = move || {
      let done = Closure::wrap(Box::new(move |json: JsValue| {
        let data: Vec<News> = json.into_serde().unwrap();

        if let Ok(app) = &(app.try_borrow_mut()) {
          app.add_message(Message::View(ViewMessage::ShowNews(
            data.clone(),
            pathname,
            index,
          )));
        }
      }) as Box<FnMut(JsValue)>);

      let endpoint = Endpoint::get_url(pathname, index);
      Fetch::get_json(&endpoint).then(&done);
      done.forget();
    };

    fetch();
  }
}
