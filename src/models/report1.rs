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
pub struct Report1 {
    #[serde(rename = "client_report_id")]
    pub client_report_id: String,
    #[serde(rename = "date_generated")]
    pub date_generated: String,
    #[serde(rename = "days_requested")]
    pub days_requested: i32,
    #[serde(rename = "items")]
    pub items: Vec<crate::models::Item8>,
    #[serde(rename = "liabilities_report_id")]
    pub liabilities_report_id: String,
}

impl Report1 {
    pub fn new(client_report_id: String, date_generated: String, days_requested: i32, items: Vec<crate::models::Item8>, liabilities_report_id: String) -> Report1 {
        Report1 {
            client_report_id,
            date_generated,
            days_requested,
            items,
            liabilities_report_id,
        }
    }
}


