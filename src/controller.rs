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
  GetNews(u32),
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
      GetNews(page) => self.get_news(page),
    }
  }

  pub fn get_news(&self, page: u32) {
    let app = self.app.clone();
    let fetch = move || {
      let done = Closure::wrap(Box::new(move |json: JsValue| {
        let data: Vec<News> = json.into_serde().unwrap();

        let mut store_ref = STORE.lock().unwrap();
        store_ref.news.extend(data.iter().cloned());

        console_log!("get_json {:?}", store_ref.news[0]);
        if let Ok(app) = &(app.try_borrow_mut()) {
          app.add_message(Message::View(ViewMessage::ShowNews(store_ref.news.clone())));
        }
      }) as Box<FnMut(JsValue)>);

      Fetch::get_json(&Endpoint::News.as_str(page)).then(&done);
      done.forget();
    };

    fetch();
  }
}
