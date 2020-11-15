/*
 * Plaid API Endpoints Copy 2
 *
 * A collection of Plaid API endpoints for the `sandbox` environment. Each endpoint request comes with an example request & response. It also contains 'use cases' for each product.   <br /> Before you begin, please set your `client_id` and `secret_key` variables in the Sandbox environment. You can find them in your Plaid [dashboard](https://dashboard.plaid.com/account/keys). Set the variables by clicking on the 'eye' icon in the top-right corner of the screen.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct InstitutionsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> InstitutionsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> InstitutionsApiClient<C> {
        InstitutionsApiClient {
            configuration,
        }
    }
}

pub trait InstitutionsApi {
    fn retrieve_insitution_list(&self, retrieve_insitution_list_request: crate::models::RetrieveInsitutionListRequest) -> Box<dyn Future<Item = crate::models::RetrieveInsitutionListExample, Error = Error<serde_json::Value>>>;
    fn search_institutionby_id(&self, search_institutionby_id_request: crate::models::SearchInstitutionbyIdRequest) -> Box<dyn Future<Item = crate::models::SearchInstitutionbyIdExample, Error = Error<serde_json::Value>>>;
    fn search_institutionby_name(&self, search_institutionby_name_request: crate::models::SearchInstitutionbyNameRequest) -> Box<dyn Future<Item = crate::models::SearchInstitutionbyNameExample, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>InstitutionsApi for InstitutionsApiClient<C> {
    fn retrieve_insitution_list(&self, retrieve_insitution_list_request: crate::models::RetrieveInsitutionListRequest) -> Box<dyn Future<Item = crate::models::RetrieveInsitutionListExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/institutions/get".to_string())
        ;
        req = req.with_body_param(retrieve_insitution_list_request);

        req.execute(self.configuration.borrow())
    }

    fn search_institutionby_id(&self, search_institutionby_id_request: crate::models::SearchInstitutionbyIdRequest) -> Box<dyn Future<Item = crate::models::SearchInstitutionbyIdExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/institutions/get_by_id".to_string())
        ;
        req = req.with_body_param(search_institutionby_id_request);

        req.execute(self.configuration.borrow())
    }

    fn search_institutionby_name(&self, search_institutionby_name_request: crate::models::SearchInstitutionbyNameRequest) -> Box<dyn Future<Item = crate::models::SearchInstitutionbyNameExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/institutions/search".to_string())
        ;
        req = req.with_body_param(search_institutionby_name_request);

        req.execute(self.configuration.borrow())
    }

}