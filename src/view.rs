pub mod element;

use super::console::*;
use crate::app::{App, Message};
use crate::controller::ControllerMessage;
use crate::store::*;
use crate::view::element::Element;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub enum ViewMessage {
  ShowNews(Vec<News>, &'static str, u32),
  ShowDetail(Item, &'static str, u32),
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

  fn bind_more(&mut self, pathname: &'static str, index: u32) {
    if let Some(mut more) = Element::qs("#more") {
      more.set_href(&format!("#/{}&{}", pathname, index));
    }
  }

  fn bind_nav_item(&mut self, pathname: &'static str) {
    let app = self.app.clone();
    let mut nav = Element::qs("nav").unwrap();
    nav.delegate(
      pathname,
      "click",
      move |_| {
        if let Ok(app) = &(app.try_borrow_mut()) {
          app.add_message(Message::Controller(ControllerMessage::GetNews(
            remove_first(pathname),
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
      ShowNews(news, pathname, index) => self.show_news(&news, pathname, index),
      ShowDetail(item, pathname, index) => self.show_detail(&item, pathname, index),
    }
  }

  pub fn show_news(&mut self, news: &Vec<News>, pathname: &'static str, index: u32) {
    self.bind_more(pathname, if index < 10 { index + 1 } else { index });

    if let Some(mut section) = Element::qs("#content") {
      if let Some(div) = section.qs_from("div") {
        section.remove_child(div);
      }

      if let Some(mut div) = Element::create_element("div") {
        section.append_child(&mut div);

        if let Some(mut ul) = Element::create_element("ul") {
          div.append_child(&mut ul);

          let mut items = String::new();
          for item in news.iter() {
            items.push_str(&format!(
              "<li class='item'>
                  <div class='points'>{:?}</div>
                  <div class='content'>
                    <div class='detail'><a href='#/detail&{}'>{:?}</a></div>
                    <div class='info'> by {:?} | {} comments</div>
                  </div>
                </li>",
              match item.points {
                Some(points) => points,
                None => 0,
              },
              item.id,
              item.title,
              match &item.user {
                Some(user) => user,
                None => "John Doe",
              },
              item.comments_count
            ));
          }
          ul.set_inner_html(items.to_string());
        }
      }
    }
  }

  pub fn show_detail(&mut self, item: &Item, pathname: &'static str, index: u32) {
    self.bind_more(pathname, if index < 10 { index + 1 } else { index });

    if let Some(mut section) = Element::qs("#content") {
      if let Some(div) = section.qs_from("div") {
        section.remove_child(div);
      }

      if let Some(mut div) = Element::create_element("div") {
        section.append_child(&mut div);

        if let Some(mut content) = Element::create_element("div") {
          div.append_child(&mut content);

          let html: String = format!(
            "<div class='item'>
                <div class='title'>{}</div>
                <div class='meta'>
                  <div class='detail'>by {} | {} comments</div>
                </div>
                <div class='content'>
                  {}
                </div>
              </div>",
            match &item.title {
              Some(title) => title.as_str(),
              None => "No title",
            },
            match &item.user {
              Some(user) => user,
              None => "John Doe",
            },
            item.comments_count,
            match &item.content {
              Some(content) => content,
              None => "",
            }
          );
          div.set_inner_html(html);
        }
      }
    }
  }
}
