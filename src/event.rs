use wasm_bindgen::prelude::*;
use web_sys::{CustomEvent, CustomEventInit};

pub fn send_event(detail: &JsValue) {
  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
  let mut ei = CustomEventInit::new();

  ei.detail(&detail);

  let event = CustomEvent::new_with_event_init_dict("app", &ei).unwrap();

  document.dispatch_event(&event).unwrap();
}
