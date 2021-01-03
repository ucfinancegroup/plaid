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
pub struct Account {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "balances")]
    pub balances: crate::models::Balances,
    #[serde(rename = "mask")]
    pub mask: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "official_name")]
    pub official_name: Option<String>,
    #[serde(rename = "subtype")]
    pub subtype: String,
    #[serde(rename = "type")]
    pub _type: String,
}

impl Account {
    pub fn new(account_id: String, balances: crate::models::Balances, mask: String, name: String, official_name: Option<String>, subtype: String, _type: String) -> Account {
        Account {
            account_id,
            balances,
            mask,
            name,
            official_name,
            subtype,
            _type,
        }
    }
}


