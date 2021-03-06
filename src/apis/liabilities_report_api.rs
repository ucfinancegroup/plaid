/*
 * Plaid API Endpoints Copy 2
 *
 * A collection of Plaid API endpoints for the `sandbox` environment. Each endpoint request comes with an example request & response. It also contains 'use cases' for each product.   <br /> Before you begin, please set your `client_id` and `secret_key` variables in the Sandbox environment. You can find them in your Plaid [dashboard](https://dashboard.plaid.com/account/keys). Set the variables by clicking on the 'eye' icon in the top-right corner of the screen.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `create_liabilities_report`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLiabilitiesReportError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `remove_liabilities_report`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveLiabilitiesReportError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `retrieve_liabilities_report`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveLiabilitiesReportError {
    UnknownValue(serde_json::Value),
}


/// With your desired `access_tokens` in hand, all you need to do to create a Liabilities Report is to call the `/liabilities_report/create` endpoint.  When creating a Liabilities Report, the only required fields are your `client_id`, `secret`, an `array of access_tokens` (one for each Item to be included in the Report), and the number of `days_requested` which determines the duration of transaction history to be included.
pub async fn create_liabilities_report(configuration: &configuration::Configuration, create_liabilities_report_request: crate::models::CreateLiabilitiesReportRequest) -> Result<crate::models::CreateLiabilitiesReportResponse, Error<CreateLiabilitiesReportError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/liabilities_report/create", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_liabilities_report_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateLiabilitiesReportError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The `/liabilities_report/remove` endpoint allows you to remove a Liabilities Report. Removing a Liabilities Report invalidates its `liabilities_report_token`, meaning you will no longer be able to use it to access report data. Removing a Liabilities Report does not affect the underlying Items.
pub async fn remove_liabilities_report(configuration: &configuration::Configuration, remove_liabilities_report_request: crate::models::RemoveLiabilitiesReportRequest) -> Result<crate::models::RemoveLiabilitiesReportResponse, Error<RemoveLiabilitiesReportError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/liabilities_report/remove", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&remove_liabilities_report_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RemoveLiabilitiesReportError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// You can retrieve your Liabilities Report in JSON by calling `/liabilities_report/get`.  <br /> Just like retrieving `transaction` data, you need to wait several seconds to retrieve a liabilities report after you create one.
pub async fn retrieve_liabilities_report(configuration: &configuration::Configuration, retrieve_liabilities_report_request: crate::models::RetrieveLiabilitiesReportRequest) -> Result<crate::models::RetrieveLiabilitiesReportResponse, Error<RetrieveLiabilitiesReportError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/liabilities_report/get", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&retrieve_liabilities_report_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveLiabilitiesReportError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

