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
pub struct Student {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "account_number")]
    pub account_number: String,
    #[serde(rename = "disbursement_dates")]
    pub disbursement_dates: Vec<String>,
    #[serde(rename = "expected_payoff_date")]
    pub expected_payoff_date: String,
    #[serde(rename = "guarantor")]
    pub guarantor: String,
    #[serde(rename = "interest_rate_percentage")]
    pub interest_rate_percentage: f32,
    #[serde(rename = "is_overdue")]
    pub is_overdue: bool,
    #[serde(rename = "last_payment_amount")]
    pub last_payment_amount: f32,
    #[serde(rename = "last_payment_date")]
    pub last_payment_date: String,
    #[serde(rename = "last_statement_balance")]
    pub last_statement_balance: f32,
    #[serde(rename = "last_statement_issue_date")]
    pub last_statement_issue_date: String,
    #[serde(rename = "loan_name")]
    pub loan_name: String,
    #[serde(rename = "loan_status")]
    pub loan_status: crate::models::LoanStatus,
    #[serde(rename = "minimum_payment_amount")]
    pub minimum_payment_amount: i32,
    #[serde(rename = "next_payment_due_date")]
    pub next_payment_due_date: String,
    #[serde(rename = "origination_date")]
    pub origination_date: String,
    #[serde(rename = "origination_principal_amount")]
    pub origination_principal_amount: i32,
    #[serde(rename = "outstanding_interest_amount")]
    pub outstanding_interest_amount: f32,
    #[serde(rename = "payment_reference_number")]
    pub payment_reference_number: Option<String>,
    #[serde(rename = "pslf_status")]
    pub pslf_status: crate::models::PslfStatus,
    #[serde(rename = "repayment_plan")]
    pub repayment_plan: crate::models::RepaymentPlan,
    #[serde(rename = "sequence_number")]
    pub sequence_number: String,
    #[serde(rename = "servicer_address")]
    pub servicer_address: crate::models::ServicerAddress,
    #[serde(rename = "ytd_interest_paid")]
    pub ytd_interest_paid: f32,
    #[serde(rename = "ytd_principal_paid")]
    pub ytd_principal_paid: f32,
}

impl Student {
    pub fn new(account_id: String, account_number: String, disbursement_dates: Vec<String>, expected_payoff_date: String, guarantor: String, interest_rate_percentage: f32, is_overdue: bool, last_payment_amount: f32, last_payment_date: String, last_statement_balance: f32, last_statement_issue_date: String, loan_name: String, loan_status: crate::models::LoanStatus, minimum_payment_amount: i32, next_payment_due_date: String, origination_date: String, origination_principal_amount: i32, outstanding_interest_amount: f32, payment_reference_number: Option<String>, pslf_status: crate::models::PslfStatus, repayment_plan: crate::models::RepaymentPlan, sequence_number: String, servicer_address: crate::models::ServicerAddress, ytd_interest_paid: f32, ytd_principal_paid: f32) -> Student {
        Student {
            account_id,
            account_number,
            disbursement_dates,
            expected_payoff_date,
            guarantor,
            interest_rate_percentage,
            is_overdue,
            last_payment_amount,
            last_payment_date,
            last_statement_balance,
            last_statement_issue_date,
            loan_name,
            loan_status,
            minimum_payment_amount,
            next_payment_due_date,
            origination_date,
            origination_principal_amount,
            outstanding_interest_amount,
            payment_reference_number,
            pslf_status,
            repayment_plan,
            sequence_number,
            servicer_address,
            ytd_interest_paid,
            ytd_principal_paid,
        }
    }
}


