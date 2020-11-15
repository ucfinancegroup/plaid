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
pub struct SearchInstitutionbyNameRequest {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "query")]
    pub query: String,
    #[serde(rename = "country_codes")]
    pub country_codes: Vec<String>,
    #[serde(rename = "products")]
    pub products: Vec<String>,
}

impl SearchInstitutionbyNameRequest {
    pub fn new(client_id: String, secret: String, query: String, country_codes: Vec<String>, products: Vec<String>) -> SearchInstitutionbyNameRequest {
        SearchInstitutionbyNameRequest {
            client_id,
            secret,
            query,
            country_codes,
            products,
        }
    }
}

