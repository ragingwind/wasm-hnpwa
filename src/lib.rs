use std::rc::Rc;

use crate::app::{App, Message};
use crate::console::*;
use crate::controller::Controller;
use crate::controller::ControllerMessage;
use crate::view::View;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[macro_use]
mod console;
mod app;
mod controller;
mod event;
mod fetch;
mod store;
mod view;

#[macro_use]
extern crate lazy_static;

fn string_to_static_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}

#[wasm_bindgen]
pub fn app() {
  let app = Rc::new(App::new());
  let mut view = View::new(app.clone());
  let controller = Controller::new(app.clone());

  {
    view.init();
    app.set_view(view);
    app.set_controller(controller);
    // app.add_message(Message::Controller(ControllerMessage::GetNews("news", 1)));
  }

  {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let set_page = Closure::wrap(Box::new(move || {
      if let Some(location) = document.location() {
        if let Ok(hash) = location.hash() {
          console_log!("hash change {}", hash);
          app.add_message(Message::Controller(ControllerMessage::ChangePage(
            string_to_static_str(hash),
          )));
        }
      }
    }) as Box<dyn FnMut()>);

    let et: web_sys::EventTarget = window.into();
    et.add_event_listener_with_callback("hashchange", set_page.as_ref().unchecked_ref())
      .unwrap();

    set_page.forget();
  }

  {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();

    match location.set_href("#/news&1") {
      Err(e) => console_log!("{:?}", e),
      _ => (),
    }
  }
}

#[wasm_bindgen]
pub fn run() {
  console_error_panic_hook::set_once();
  app()
}
