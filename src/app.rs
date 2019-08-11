use crate::controller::{Controller, ControllerMessage};
use crate::view::{View, ViewMessage};
use std::cell::RefCell;
use std::rc::Rc;

pub enum Message {
  Controller(ControllerMessage),
  View(ViewMessage),
}

pub struct App {
  controller: Rc<RefCell<Option<Controller>>>,
  view: Rc<RefCell<Option<View>>>,
  events: RefCell<Vec<Message>>,
}

impl App {
  pub fn new() -> App {
    App {
      controller: Rc::new(RefCell::new(None)),
      view: Rc::new(RefCell::new(None)),
      events: RefCell::new(Vec::new()),
    }
  }

  pub fn set_controller(&self, controller: Controller) {
    if let Ok(mut controller_data) = self.controller.try_borrow_mut() {
      *controller_data = Some(controller);
    }
  }

  pub fn set_view(&self, view: View) {
    let mut view_data = self.view.try_borrow_mut().unwrap();
    *view_data = Some(view);
  }

  pub fn add_message(&self, message: Message) {
    {
      let mut events = self.events.try_borrow_mut().unwrap();
      events.push(message);
    }

    {
      self.run();
    }
  }

  fn run(&self) {
    self.next_message();
  }

  fn next_message(&self) {
    let event = {
      if let Ok(mut events) = self.events.try_borrow_mut() {
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
    }
  }
}
