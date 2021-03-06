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
pub struct Apr {
    #[serde(rename = "apr_percentage")]
    pub apr_percentage: Option<String>,
    #[serde(rename = "apr_type")]
    pub apr_type: Option<String>,
    #[serde(rename = "balance_subject_to_apr")]
    pub balance_subject_to_apr: Option<String>,
    #[serde(rename = "interest_charge_amount")]
    pub interest_charge_amount: Option<String>,
}

impl Apr {
    pub fn new(apr_percentage: Option<String>, apr_type: Option<String>, balance_subject_to_apr: Option<String>, interest_charge_amount: Option<String>) -> Apr {
        Apr {
            apr_percentage,
            apr_type,
            balance_subject_to_apr,
            interest_charge_amount,
        }
    }
}


