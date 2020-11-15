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
pub struct Address3 {
    #[serde(rename = "street")]
    pub street: Vec<String>,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "postal_code")]
    pub postal_code: String,
    #[serde(rename = "country")]
    pub country: String,
}

impl Address3 {
    pub fn new(street: Vec<String>, city: String, postal_code: String, country: String) -> Address3 {
        Address3 {
            street,
            city,
            postal_code,
            country,
        }
    }
}

