use std::rc::Rc;

use crate::app::{App, Message};
use crate::controller::Controller;
use crate::controller::ControllerMessage;
use crate::view::View;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn app() {
  let app = Rc::new(App::new());
  let mut view = View::new(app.clone());
  let controller = Controller::new(app.clone());

  {
    view.init();
    app.set_view(view);
    app.set_controller(controller);

    app.add_message(Message::Controller(ControllerMessage::GetNews(1)));
  }
}

#[wasm_bindgen]
pub fn run() {
  console_error_panic_hook::set_once();
  app()
}
