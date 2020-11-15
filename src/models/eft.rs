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
pub struct Eft {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "branch")]
    pub branch: String,
    #[serde(rename = "institution")]
    pub institution: String,
}

impl Eft {
    pub fn new(account: String, account_id: String, branch: String, institution: String) -> Eft {
        Eft {
            account,
            account_id,
            branch,
            institution,
        }
    }
}


