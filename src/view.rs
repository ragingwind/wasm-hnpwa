pub mod element;

use super::console::*;
use crate::app::{App, Message};
use crate::controller::ControllerMessage;
use crate::types::*;
use crate::view::element::Element;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub enum ViewMessage {
  ShowNews(Vec<News>, &'static str, u32),
  ShowUser(User, &'static str, &'static str),
  ShowComment(Item, &'static str, u32),
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

  fn bind_more(&mut self, pathname: &'static str, index: u32) {
    if let Some(mut more) = Element::qs("#more") {
      if let Some(a) = more.qs_from("a") {
        more.remove_child(a);
      }

      let html: String = format!("<a href='#/{}&{}'>More...</a>", pathname, index);
      more.set_inner_html(html.to_string());
    }
  }

  pub fn call(&mut self, method_name: ViewMessage) {
    use self::ViewMessage::*;
    match method_name {
      ShowNews(news, pathname, index) => self.show_news(&news, pathname, index),
      ShowUser(user, pathname, uid) => self.show_user(&user, pathname, uid),
      ShowComment(item, pathname, index) => self.show_comment(&item, pathname, index),
    }
  }

  pub fn show_comment(&mut self, item: &Item, pathname: &'static str, index: u32) {
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

        if let Some(mut ul) = Element::create_element("ul") {
          div.append_child(&mut ul);

          let mut comments = String::new();

          for comment in item.comments.iter() {
            let user = match &comment.user {
              Some(user) => user,
              None => "John Doe",
            };

            comments.push_str(&format!(
              "<li class='comment'>
                <div>
                  <sub class='user'>{}</sub>
                  <sub class='time_ago'>{}</sub>
                </div>
                <div>
                  <div class='content'>{}</div>
                </li>",
              user, comment.time_ago, comment.content
            ));
          }
          ul.set_inner_html(comments.to_string());
        }
      }
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
                    <div class='info'> by <a href='#/user&{}'>{}</a> | <a href='#/comment&{}'>{} comments</a></div>
                  </div>
                </li>",
              points, item.url, item.title, domain, user, user, item.id, item.comments_count
            ));
          }
          ul.set_inner_html(items.to_string());
        }
      }
    }
  }

  pub fn show_user(&mut self, user: &User, pathname: &'static str, uid: &'static str) {
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
