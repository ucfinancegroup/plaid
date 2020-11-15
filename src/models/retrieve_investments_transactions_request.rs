/*
 * Plaid API Endpoints Copy 2
 *
 * A collection of Plaid API endpoints for the `sandbox` environment. Each endpoint request comes with an example request & response. It also contains 'use cases' for each product.   <br /> Before you begin, please set your `client_id` and `secret_key` variables in the Sandbox environment. You can find them in your Plaid [dashboard](https://dashboard.plaid.com/account/keys). Set the variables by clicking on the 'eye' icon in the top-right corner of the screen.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetrieveInvestmentsTransactionsRequest {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date")]
    pub end_date: String,
}

impl RetrieveInvestmentsTransactionsRequest {
    pub fn new(client_id: String, secret: String, access_token: String, start_date: String, end_date: String) -> RetrieveInvestmentsTransactionsRequest {
        RetrieveInvestmentsTransactionsRequest {
            client_id,
            secret,
            access_token,
            start_date,
            end_date,
        }
    }
}

