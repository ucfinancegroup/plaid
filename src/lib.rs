#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

pub mod apis;
pub mod models;

pub fn new_api_client() -> apis::client::APIClient {
  apis::client::APIClient::new(apis::configuration::Configuration::new(hyper::Client::new(
    &tokio_core::reactor::Core::new().unwrap().handle(),
  )))
}
