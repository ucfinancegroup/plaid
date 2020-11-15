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

pub struct AssetsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AssetsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AssetsApiClient<C> {
        AssetsApiClient {
            configuration,
        }
    }
}

pub trait AssetsApi {
    fn create_asset_report(&self, create_asset_report_request: crate::models::CreateAssetReportRequest) -> Box<dyn Future<Item = crate::models::CreateAssetReportExample, Error = Error<serde_json::Value>>>;
    fn create_audit_copy(&self, create_audit_copy_request: crate::models::CreateAuditCopyRequest) -> Box<dyn Future<Item = crate::models::CreateAuditCopyExample, Error = Error<serde_json::Value>>>;
    fn refresh_asset_report(&self, refresh_asset_report_request: crate::models::RefreshAssetReportRequest) -> Box<dyn Future<Item = crate::models::RefreshAssetReportExample, Error = Error<serde_json::Value>>>;
    fn remove_asset_report(&self, remove_asset_report_request: crate::models::RemoveAssetReportRequest) -> Box<dyn Future<Item = crate::models::RemoveAssetReportExample, Error = Error<serde_json::Value>>>;
    fn remove_audit_copy(&self, remove_audit_copy_request: crate::models::RemoveAuditCopyRequest) -> Box<dyn Future<Item = crate::models::RemoveAuditCopyExample, Error = Error<serde_json::Value>>>;
    fn retrievean_asset_report_json(&self, retrievean_asset_report_json_request: crate::models::RetrieveanAssetReportJsonRequest) -> Box<dyn Future<Item = crate::models::RetrieveanAssetReportJsoNwithInsights, Error = Error<serde_json::Value>>>;
    fn retrievean_asset_report_pdf(&self, retrievean_asset_report_pdf_request: crate::models::RetrieveanAssetReportPdfRequest) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>AssetsApi for AssetsApiClient<C> {
    fn create_asset_report(&self, create_asset_report_request: crate::models::CreateAssetReportRequest) -> Box<dyn Future<Item = crate::models::CreateAssetReportExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/asset_report/create".to_string())
        ;
        req = req.with_body_param(create_asset_report_request);

        req.execute(self.configuration.borrow())
    }

    fn create_audit_copy(&self, create_audit_copy_request: crate::models::CreateAuditCopyRequest) -> Box<dyn Future<Item = crate::models::CreateAuditCopyExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/asset_report/audit_copy/create".to_string())
        ;
        req = req.with_body_param(create_audit_copy_request);

        req.execute(self.configuration.borrow())
    }

    fn refresh_asset_report(&self, refresh_asset_report_request: crate::models::RefreshAssetReportRequest) -> Box<dyn Future<Item = crate::models::RefreshAssetReportExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/asset_report/refresh".to_string())
        ;
        req = req.with_body_param(refresh_asset_report_request);

        req.execute(self.configuration.borrow())
    }

    fn remove_asset_report(&self, remove_asset_report_request: crate::models::RemoveAssetReportRequest) -> Box<dyn Future<Item = crate::models::RemoveAssetReportExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/asset_report/remove".to_string())
        ;
        req = req.with_body_param(remove_asset_report_request);

        req.execute(self.configuration.borrow())
    }

    fn remove_audit_copy(&self, remove_audit_copy_request: crate::models::RemoveAuditCopyRequest) -> Box<dyn Future<Item = crate::models::RemoveAuditCopyExample, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/asset_report/audit_copy/remove".to_string())
        ;
        req = req.with_body_param(remove_audit_copy_request);

        req.execute(self.configuration.borrow())
    }

    fn retrievean_asset_report_json(&self, retrievean_asset_report_json_request: crate::models::RetrieveanAssetReportJsonRequest) -> Box<dyn Future<Item = crate::models::RetrieveanAssetReportJsoNwithInsights, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/asset_report/get".to_string())
        ;
        req = req.with_body_param(retrievean_asset_report_json_request);

        req.execute(self.configuration.borrow())
    }

    fn retrievean_asset_report_pdf(&self, retrievean_asset_report_pdf_request: crate::models::RetrieveanAssetReportPdfRequest) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/asset_report/pdf/get".to_string())
        ;
        req = req.with_body_param(retrievean_asset_report_pdf_request);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}
