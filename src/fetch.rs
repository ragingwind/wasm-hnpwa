use futures::{future, Future};
use js_sys::Promise;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

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
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();
        resp.json()
      })
      .and_then(|json_value: Promise| JsFuture::from(json_value))
      .and_then(|json| future::ok(json));

    future_to_promise(future)
  }
}
