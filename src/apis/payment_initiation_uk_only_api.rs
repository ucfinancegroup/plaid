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


/// struct for typed errors of method `create_payment`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePaymentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_payment_recipient`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePaymentRecipientError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `create_payment_token`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePaymentTokenError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_payment`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPaymentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_payment_recipient`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPaymentRecipientError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_payment_recipients`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPaymentRecipientsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_payments`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPaymentsError {
    UnknownValue(serde_json::Value),
}


/// Create and configure a payment by specifying a `recipient_id`, `reference`, `amount`, and `currency` to create a Plaid payment resource. The Plaid API returns a `payment_id` to identify the created payment.
pub async fn create_payment(configuration: &configuration::Configuration, create_payment_request: crate::models::CreatePaymentRequest) -> Result<(), Error<CreatePaymentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/payment_initiation/payment/create", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_payment_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<CreatePaymentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// In the sandbox environment, you can use the `/payment_initiation/recipient/create` endpoint to generate recipients. Programmatic recipient creation in the development and production environments can be done after approval by our compliance team.  The endpoint is idempotent: if a developer has already made a request with the same payment details, Plaid will return the same recipient_id. Recipients are scoped per environment.
pub async fn create_payment_recipient(configuration: &configuration::Configuration, create_payment_recipient_request: crate::models::CreatePaymentRecipientRequest) -> Result<(), Error<CreatePaymentRecipientError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/payment_initiation/recipient/create", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_payment_recipient_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<CreatePaymentRecipientError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// In order to actually initiate a Payment, you must use the `payment_id` returned from `payment_initiation/payment/create` and create a `payment_token`.  The `payment_token` will allow for at most one payment initiation. If this attempt fails, the end user aborts the flow, or the token expires, the developer will need to create a new payment token. Creating a new payment token does not require end user input.
pub async fn create_payment_token(configuration: &configuration::Configuration, create_payment_token_request: crate::models::CreatePaymentTokenRequest) -> Result<(), Error<CreatePaymentTokenError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/payment_initiation/payment/token/create", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_payment_token_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<CreatePaymentTokenError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The endpoint `payment_initiation/payment/get` takes a `payment_id` and returns all of the payment details for a previously created payment.
pub async fn get_payment(configuration: &configuration::Configuration, get_payment_request: crate::models::GetPaymentRequest) -> Result<(), Error<GetPaymentError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/payment_initiation/payment/get", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&get_payment_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<GetPaymentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The `/payment_initiation/recipient/get` endpoint allows you to retrieve all of the details for a recipient that you have created.
pub async fn get_payment_recipient(configuration: &configuration::Configuration, get_payment_recipient_request: crate::models::GetPaymentRecipientRequest) -> Result<(), Error<GetPaymentRecipientError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/payment_initiation/recipient/get", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&get_payment_recipient_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<GetPaymentRecipientError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The `/payment_initiation/recipient/list` endpoint allows you to retrieve details for all recipients that you have created.
pub async fn list_payment_recipients(configuration: &configuration::Configuration, list_payment_recipients_request: crate::models::ListPaymentRecipientsRequest) -> Result<(), Error<ListPaymentRecipientsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/payment_initiation/recipient/list", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&list_payment_recipients_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<ListPaymentRecipientsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The endpoint `payment_initiation/payment/get` list payment details for all of your previously created payments.  This endpoint accepts `count` and `cursor` as optional parameters in order to support pagination. `count` limits how many payments are returned and can be set between 0 and 200. `cursor` should be a string in RFC 3339 format (i.e. `\"2019-12-06T22:35:49Z\"`). Only payments created before the `cursor` will be returned.
pub async fn list_payments(configuration: &configuration::Configuration, list_payments_request: crate::models::ListPaymentsRequest) -> Result<(), Error<ListPaymentsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/payment_initiation/payment/list", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&list_payments_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        Ok(())
    } else {
        let local_var_entity: Option<ListPaymentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

