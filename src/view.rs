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
  ShowUser(User, &'static str, &'static str),
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
      if let Some(a) = more.qs_from("a") {
        more.remove_child(a);
      }

      let html: String = format!("<a href='#/{}&{}'>More...</a>", pathname, index);
      more.set_inner_html(html.to_string());
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
      ShowUser(user, pathname, uid) => self.show_user(&user, pathname, uid),
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
            let points = match item.points {
              Some(points) => points,
              None => 0,
            };
            let domain = match &item.domain {
              Some(domain) => domain,
              None => "",
            };
            let user = match &item.user {
              Some(user) => user,
              None => "John Doe",
            };

            items.push_str(&format!(
              "<li class='item'>
                  <div class='points'>{}</div>
                  <div class='content'>
                    <div class='detail'>
                      <span><a href='{}' target='_blank'>{}</a></span>
                      <span class='domain'>{}</span>
                    </div>
                    <div class='info'> by <a href='#/user&{}'>{}</a> | {} comments</div>
                  </div>
                </li>",
              points, item.url, item.title, domain, user, user, item.comments_count
            ));
          }
          ul.set_inner_html(items.to_string());
        }
      }
    }
  }

  pub fn show_user(&mut self, user: &User, pathname: &'static str, uid: &'static str) {
    console_log!("{}, {}", pathname, uid);
    if let Some(mut more) = Element::qs("#more") {
      if let Some(a) = more.qs_from("a") {
        more.remove_child(a);
      }
    }

    if let Some(mut section) = Element::qs("#content") {
      if let Some(div) = section.qs_from("div") {
        section.remove_child(div);
      }

      if let Some(mut div) = Element::create_element("div") {
        section.append_child(&mut div);

        if let Some(mut content) = Element::create_element("div") {
          div.append_child(&mut content);
          let html: String = format!(
            "<div class='detail'>
                <div class='title'><h2>{}</h2> <span>joined {}, and has {} karma</div>
              </div>",
            user.id, user.created_time, user.karma
          );

          div.set_inner_html(html);
        }
      }
    }
  }
}
