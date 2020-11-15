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

pub struct LiabilitiesReportApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> LiabilitiesReportApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> LiabilitiesReportApiClient<C> {
        LiabilitiesReportApiClient {
            configuration,
        }
    }
}

pub trait LiabilitiesReportApi {
    fn create_liabilities_report(&self, create_liabilities_report_request: crate::models::CreateLiabilitiesReportRequest) -> Box<dyn Future<Item = crate::models::CreateLiabilitiesReportExample, Error = Error<serde_json::Value>>>;
    fn remove_liabilities_report(&self, remove_liabilities_report_request: crate::models::RemoveLiabilitiesReportRequest) -> Box<dyn Future<Item = crate::models::RemoveLiabilitiesReportExample, Error = Error<serde_json::Value>>>;
    fn retrieve_liabilities_report(&self, retrieve_liabilities_report_request: crate::models::RetrieveLiabilitiesReportRequest) -> Box<dyn Future<Item = crate::models::RetrieveLiabilitiesReportExample, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>LiabilitiesReportApi for LiabilitiesReportApiClient<C> {
    fn create_liabilities_report(&self, create_liabilities_report_request: crate::models::CreateLiabilitiesReportRequest) -> Box<dyn Future<Item = crate::models::CreateLiabilitiesReportExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/liabilities_report/create".to_string())
        ;
        req = req.with_body_param(create_liabilities_report_request);

        req.execute(self.configuration.borrow())
    }

    fn remove_liabilities_report(&self, remove_liabilities_report_request: crate::models::RemoveLiabilitiesReportRequest) -> Box<dyn Future<Item = crate::models::RemoveLiabilitiesReportExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/liabilities_report/remove".to_string())
        ;
        req = req.with_body_param(remove_liabilities_report_request);

        req.execute(self.configuration.borrow())
    }

    fn retrieve_liabilities_report(&self, retrieve_liabilities_report_request: crate::models::RetrieveLiabilitiesReportRequest) -> Box<dyn Future<Item = crate::models::RetrieveLiabilitiesReportExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/liabilities_report/get".to_string())
        ;
        req = req.with_body_param(retrieve_liabilities_report_request);

        req.execute(self.configuration.borrow())
    }

}
