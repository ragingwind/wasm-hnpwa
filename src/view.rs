pub mod element;

use super::console::*;
use crate::app::{App, Message};
use crate::controller::ControllerMessage;
use crate::store::News;
use crate::view::element::Element;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub enum ViewMessage {
  ShowNews(Vec<News>, &'static str, u32),
}

fn remove_first(s: &str) -> &str {
  let mut chars = s.chars();
  chars.next();
  chars.as_str()
}

pub struct View {
  app: RefCell<Rc<App>>,
}

impl View {
  pub fn new(app: Rc<App>) -> View {
    View {
      app: RefCell::new(app),
    }
  }

  pub fn init(&mut self) {
    // self.bind_nav_item("#news");
    // self.bind_nav_item("#new");
    // self.bind_nav_item("#ask");
    // self.bind_nav_item("#show");
    // self.bind_nav_item("#jobs");
    // self.bind_more("news", 2);
  }

  fn bind_more(&mut self, item_name: &'static str, page: u32) {
    if let Some(mut more) = Element::qs("#more") {
      more.set_href(&format!("#/{}&{}", item_name, page));
    }
  }

  fn bind_nav_item(&mut self, item_name: &'static str) {
    let app = self.app.clone();
    let mut nav = Element::qs("nav").unwrap();
    nav.delegate(
      item_name,
      "click",
      move |_| {
        if let Ok(app) = &(app.try_borrow_mut()) {
          app.add_message(Message::Controller(ControllerMessage::GetNews(
            remove_first(item_name),
            1,
          )));
        }
      },
      false,
    );
  }

  pub fn call(&mut self, method_name: ViewMessage) {
    use self::ViewMessage::*;
    match method_name {
      ShowNews(news, item_name, page) => self.show_news(&news, item_name, page),
    }
  }

  pub fn show_news(&mut self, news: &Vec<News>, item_name: &'static str, page: u32) {
    self.bind_more(item_name, if page < 10 { page + 1 } else { page });

    if let Some(mut section) = Element::qs("section") {
      if let Some(ul) = section.qs_from("ul") {
        section.remove_child(ul);
      }

      if let Some(mut ul) = Element::create_element("ul") {
        section.append_child(&mut ul);

        let mut items = String::new();
        for item in news.iter() {
          items.push_str(&format!(
            "<li><div><a href={:?} target='_blank'>{:?}<a></div></li>",
            item.url, item.title
          ));
        }
        ul.set_inner_html(items.to_string());
      }
    }
  }
}
