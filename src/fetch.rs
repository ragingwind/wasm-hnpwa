use futures::{future, Future};
use js_sys::{Array, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = window)]
  pub fn fetch(url: &str) -> Promise;
}

pub struct Fetch {}

impl Fetch {
  pub fn get_json(ep: &String) -> Promise {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(ep, &opts).unwrap();

    request.headers().set("Accept", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);
    let future = JsFuture::from(request_promise)
      .and_then(|resp_value| {
        let resp: Response = resp_value.dyn_into().unwrap();
        resp.json()
      })
      .and_then(|json_value: Promise| JsFuture::from(json_value))
      .and_then(|json| future::ok(json));

    future_to_promise(future)
  }

  pub fn get_jsons(urls: &Vec<String>) -> JsFuture {
    let fetches = Array::new();

    for u in urls {
      fetches.push(&JsValue::from(fetch(&u)));
    }

    let future = JsFuture::from(Promise::all(&fetches))
      .and_then(|responses| {
        let responses: Array = responses.dyn_into().unwrap();
        let jsons = Array::new();

        for r in responses.values() {
          let response: Response = r.unwrap().dyn_into().unwrap();
          jsons.push(&response.json().unwrap());
        }

        future::ok(Promise::all(&jsons))
      })
      .and_then(|jsons: Promise| JsFuture::from(jsons));

    JsFuture::from(future_to_promise(future))
  }
}
