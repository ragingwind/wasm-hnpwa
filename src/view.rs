pub mod element;

use super::console::*;
use crate::app::App;
// use crate::controller::ControllerMessage;
use crate::store::News;
use crate::view::element::Element;
use std::cell::RefCell;
use std::rc::Rc;

pub enum ViewMessage {
  ShowNews(Vec<News>),
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
    // self.bind_body();
  }

  // pub fn bind_body(&mut self) {
  //   let app = self.app.clone();
  //   let handler = move |_| {
  //     if let Ok(app) = &(app.try_borrow_mut()) {
  //       app.add_message(Message::Controller(ControllerMessage::GetNews(1)));
  //     }
  //   };
  //   self.body.delegate("h1", "click", handler, false)
  // }

  pub fn call(&mut self, method_name: ViewMessage) {
    use self::ViewMessage::*;
    match method_name {
      ShowNews(news) => self.show_news(news),
    }
  }

  pub fn show_news(&mut self, news: Vec<News>) {
    if let Some(mut ul) = Element::qs("section ul") {
      for item in news.iter() {
        console_log!("{:?}", item.title);
        if let Some(mut li) = Element::create_element("li") {
          li.set_inner_html(
            format!(
              "<li><div><a href={:?} target='_blank'>{:?}<a></div></li>",
              item.url, item.title
            )
            .to_string(),
          );

          ul.append_child(&mut li);
        }
      }
    }
  }
}
