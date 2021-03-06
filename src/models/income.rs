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
pub struct Income {
    #[serde(rename = "income_streams")]
    pub income_streams: Vec<crate::models::IncomeStream>,
    #[serde(rename = "last_year_income")]
    pub last_year_income: i32,
    #[serde(rename = "last_year_income_before_tax")]
    pub last_year_income_before_tax: i32,
    #[serde(rename = "max_number_of_overlapping_income_streams")]
    pub max_number_of_overlapping_income_streams: i32,
    #[serde(rename = "number_of_income_streams")]
    pub number_of_income_streams: i32,
    #[serde(rename = "projected_yearly_income")]
    pub projected_yearly_income: i32,
    #[serde(rename = "projected_yearly_income_before_tax")]
    pub projected_yearly_income_before_tax: i32,
}

impl Income {
    pub fn new(income_streams: Vec<crate::models::IncomeStream>, last_year_income: i32, last_year_income_before_tax: i32, max_number_of_overlapping_income_streams: i32, number_of_income_streams: i32, projected_yearly_income: i32, projected_yearly_income_before_tax: i32) -> Income {
        Income {
            income_streams,
            last_year_income,
            last_year_income_before_tax,
            max_number_of_overlapping_income_streams,
            number_of_income_streams,
            projected_yearly_income,
            projected_yearly_income_before_tax,
        }
    }
}


