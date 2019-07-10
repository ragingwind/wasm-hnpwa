pub struct Store {

}

pub struct Controller {
  store: Store,
  active_route: String,
}

impl Controller {
  pub fn new (store: Store) -> Controller {
    Controller {
      store,
      active_route: "".into(),
    }
  }
}