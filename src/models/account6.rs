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
pub struct Account6 {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "balances")]
    pub balances: crate::models::Balances6,
    #[serde(rename = "liabilities")]
    pub liabilities: Vec<crate::models::Liability>,
    #[serde(rename = "mask")]
    pub mask: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "official_name")]
    pub official_name: String,
    #[serde(rename = "owners")]
    pub owners: Vec<crate::models::Owner1>,
    #[serde(rename = "ownership_type")]
    pub ownership_type: Option<String>,
    #[serde(rename = "subtype")]
    pub subtype: String,
    #[serde(rename = "transactions")]
    pub transactions: Option<Vec<crate::models::Transaction2>>,
    #[serde(rename = "type")]
    pub _type: String,
}

impl Account6 {
    pub fn new(account_id: String, balances: crate::models::Balances6, liabilities: Vec<crate::models::Liability>, mask: String, name: String, official_name: String, owners: Vec<crate::models::Owner1>, ownership_type: Option<String>, subtype: String, transactions: Option<Vec<crate::models::Transaction2>>, _type: String) -> Account6 {
        Account6 {
            account_id,
            balances,
            liabilities,
            mask,
            name,
            official_name,
            owners,
            ownership_type,
            subtype,
            transactions,
            _type,
        }
    }
}


