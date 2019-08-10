use super::console::*;
use crate::app::{App, Message};
use crate::fetch::*;
use crate::types::*;
use crate::view::ViewMessage;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct Controller {
  app: RefCell<Rc<App>>,
}

pub enum ControllerMessage {
  ChangePage(&'static str),
}

impl Controller {
  pub fn new(app: Rc<App>) -> Controller {
    Controller {
      app: RefCell::new(app),
    }
  }

  pub fn call(&mut self, method_name: ControllerMessage) {
    use self::ControllerMessage::*;
    match method_name {
      ChangePage(hash) => self.change_page(hash),
    }
  }

  fn change_page(&self, hash: &'static str) {
    let hash = hash.trim_start_matches("#/");
    let v: Vec<&str> = to_static_str(hash.to_string()).split("&").collect();
    let pathname = v[0];

    match pathname {
      "news" | "newest" | "ask" | "show" | "jobs" => {
        self.get_news(pathname, v[1].parse::<u32>().unwrap());
      }
      "user" => self.get_user(pathname, v[1]),
      "comment" => {
        self.get_comment(pathname, v[1].parse::<u32>().unwrap());
      }
      _ => self.get_news("news", 1),
    }
  }

  pub fn get_comment(&self, pathname: &'static str, index: u32) {
    let app = self.app.clone();
    let fetch = move || {
      let done = Closure::wrap(Box::new(move |json: JsValue| {
        let item: Item = json.into_serde().unwrap();
        console_log!("data: {:?}", item);

        if let Ok(app) = &(app.try_borrow_mut()) {
          app.add_message(Message::View(ViewMessage::ShowComment(
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

  pub fn get_user(&self, pathname: &'static str, uid: &'static str) {
    let app = self.app.clone();
    let fetch = move || {
      let done = Closure::wrap(Box::new(move |json: JsValue| {
        let user: User = json.into_serde().unwrap();

        if let Ok(app) = &(app.try_borrow_mut()) {
          app.add_message(Message::View(ViewMessage::ShowUser(user, pathname, uid)));
        }
      }) as Box<FnMut(JsValue)>);

      let endpoint = format!("https://api.hnpwa.com/v0/user/{}.json", uid);
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

      let endpoint = get_url(pathname, index);
      Fetch::get_json(&endpoint).then(&done);
      done.forget();
    };

    fetch();
  }
}
