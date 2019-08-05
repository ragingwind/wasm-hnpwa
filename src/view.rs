pub mod element;

use super::console::*;
use crate::app::{App, Message};
use crate::controller::ControllerMessage;
use crate::store::News;
use crate::view::element::Element;
use std::cell::RefCell;
use std::rc::Rc;

pub enum ViewMessage {
  ShowNews(Vec<News>),
}

fn remove_first(s: &str) -> &str {
  let mut chars = s.chars();
  chars.next();
  chars.as_str()
}

pub struct View {
  body: Element,
  app: RefCell<Rc<App>>,
}

impl View {
  pub fn new(app: Rc<App>) -> View {
    let body = Element::qs("body").unwrap();

    View {
      body,
      app: RefCell::new(app),
    }
  }

  pub fn init(&mut self) {
    self.bind_nav_item("#news");
    self.bind_nav_item("#newest");
    self.bind_nav_item("#ask");
    self.bind_nav_item("#show");
    self.bind_nav_item("#jobs");
  }

  pub fn bind_nav_item(&mut self, item_name: &'static str) {
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
      ShowNews(news) => View::show_news(&news),
    }
  }

  pub fn show_news(news: &Vec<News>) {
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
