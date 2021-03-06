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
pub struct Item8 {
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::models::Account6>,
    #[serde(rename = "date_last_updated")]
    pub date_last_updated: String,
    #[serde(rename = "institution_id")]
    pub institution_id: String,
    #[serde(rename = "institution_name")]
    pub institution_name: String,
    #[serde(rename = "item_id")]
    pub item_id: String,
}

impl Item8 {
    pub fn new(accounts: Vec<crate::models::Account6>, date_last_updated: String, institution_id: String, institution_name: String, item_id: String) -> Item8 {
        Item8 {
            accounts,
            date_last_updated,
            institution_id,
            institution_name,
            item_id,
        }
    }
}


