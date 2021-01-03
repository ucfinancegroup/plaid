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
pub struct Balances6 {
    #[serde(rename = "available")]
    pub available: f32,
    #[serde(rename = "current")]
    pub current: f32,
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    #[serde(rename = "limit")]
    pub limit: Option<f32>,
    #[serde(rename = "unofficial_currency_code")]
    pub unofficial_currency_code: Option<String>,
}

impl Balances6 {
    pub fn new(available: f32, current: f32, iso_currency_code: String, limit: Option<f32>, unofficial_currency_code: Option<String>) -> Balances6 {
        Balances6 {
            available,
            current,
            iso_currency_code,
            limit,
            unofficial_currency_code,
        }
    }
}


