use crate::controller::{Controller, ControllerMessage};
// use crate::fetch::*;
use crate::console::*;
use crate::view::{View, ViewMessage};
use std::cell::RefCell;
use std::rc::Rc;
// use wasm_bindgen::prelude::*;

pub enum Message {
  Controller(ControllerMessage),
  View(ViewMessage),
}

pub struct App {
  controller: Rc<RefCell<Option<Controller>>>,
  view: Rc<RefCell<Option<View>>>,
  events: RefCell<Vec<Message>>,
  running: RefCell<bool>,
}

pub fn exit(message: &str) {
  let v = wasm_bindgen::JsValue::from_str(&message.to_string());
  web_sys::console::exception_1(&v);
  std::process::abort();
}

impl App {
  pub fn new() -> App {
    App {
      controller: Rc::new(RefCell::new(None)),
      view: Rc::new(RefCell::new(None)),
      events: RefCell::new(Vec::new()),
      running: RefCell::new(false),
    }
  }

  pub fn set_controller(&self, controller: Controller) {
    if let Ok(mut controller_data) = self.controller.try_borrow_mut() {
      *controller_data = Some(controller);
    } else {
      exit("This might be a deadlock");
    }
  }

  pub fn set_view(&self, view: View) {
    let mut view_data = self.view.try_borrow_mut().unwrap();
    *view_data = Some(view);
  }

  pub fn add_message(&self, message: Message) {
    log("add message");
    let running = self.running.try_borrow_mut().unwrap().clone();

    {
      let mut events = self.events.try_borrow_mut().unwrap();
      events.push(message);
    }

    {
      if !running {
        self.run();
      }
    }
  }

  /// Start the event loop, taking messages from the stack to run
  fn run(&self) {
    log("run");
    {
      let events = self.events.try_borrow().unwrap();
      let events_len = events.len().clone();
      let mut running = self.running.try_borrow_mut().unwrap().clone();

      if events_len == 0 {
        running = false;
      } else {
        running = true;
      }
    }

    log("next message");
    self.next_message();
  }

  fn next_message(&self) {
    // let events = self.events.try_borrow().unwrap();
    let mut running = self.running.try_borrow_mut().unwrap().clone();

    let event = {
      if let Ok(mut events) = self.events.try_borrow_mut() {
        console_log!("event pop");
        Some(events.pop())
      } else {
        None
      }
    };

    if let Some(Some(event)) = event {
      match event {
        Message::Controller(e) => {
          let mut controller = self.controller.try_borrow_mut().unwrap();
          if let Some(ref mut controller) = *controller {
            controller.call(e)
          }
        }
        Message::View(e) => {
          let mut view = self.view.try_borrow_mut().unwrap();
          if let Some(ref mut view) = *view {
            view.call(e)
          }
        }
      }
      self.run();
    } else if running == true {
      running = false;
    }
  }
}
