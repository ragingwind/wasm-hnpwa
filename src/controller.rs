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
}

pub enum ControllerMessage {
  GetNews(&'static str, u32),
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
      GetNews(item_name, page) => self.get_news(item_name, page),
    }
  }

  pub fn get_news(&self, item_name: &'static str, page: u32) {
    let app = self.app.clone();
    let fetch = move || {
      let done = Closure::wrap(Box::new(move |json: JsValue| {
        let data: Vec<News> = json.into_serde().unwrap();

        if let Ok(app) = &(app.try_borrow_mut()) {
          app.add_message(Message::View(ViewMessage::ShowNews(data.clone())));
        }
      }) as Box<FnMut(JsValue)>);

      let endpoint = Endpoint::get_url(item_name, page);
      Fetch::get_json(&endpoint).then(&done);
      done.forget();
    };

    fetch();
  }
}
