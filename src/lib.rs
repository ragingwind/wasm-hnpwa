use std::rc::Rc;

use crate::app::{App, Message};
use crate::console::*;
use crate::controller::Controller;
use crate::controller::ControllerMessage;
use crate::types::*;
use crate::view::View;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[macro_use]
mod console;
mod app;
mod controller;
mod fetch;
mod types;
mod view;

#[wasm_bindgen]
pub fn app() {
  let app = Rc::new(App::new());
  let view = View::new();
  let controller = Controller::new(app.clone());

  {
    app.set_view(view);
    app.set_controller(controller);
  }

  {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();

    let href = location.href().unwrap();
    let mut domain: Vec<&str> = href.split("/").collect();

    if let Some(hash) = domain.pop() {
      let mut hashes: Vec<&str> = hash.split("&").collect();

      if hashes.len() < 2 {
        hashes.push("1");
      }

      let hash = match hashes[0] {
        "news" => "news",
        "newest" => "newest",
        "ask" => "ask",
        "show" => "show",
        "jobs" => "jobs",
        "detail" => "detail",
        "user" => "user",
        "comment" => "comment",
        _ => "news",
      };

      app.add_message(Message::Controller(ControllerMessage::ChangePage(
        to_static_str(format!("#/{}&{}", hash, hashes[1])),
      )));
    }
  }

  {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let set_page = Closure::wrap(Box::new(move || {
      if let Some(location) = document.location() {
        if let Ok(hash) = location.hash() {
          console_log!("hash change {}", hash);
          app.add_message(Message::Controller(ControllerMessage::ChangePage(
            to_static_str(hash),
          )));
        }
      }
    }) as Box<dyn FnMut()>);

    let et: web_sys::EventTarget = window.into();
    et.add_event_listener_with_callback("hashchange", set_page.as_ref().unchecked_ref())
      .unwrap();

    set_page.forget();
  }
}

#[wasm_bindgen]
pub fn run() {
  console_error_panic_hook::set_once();
  app()
}
