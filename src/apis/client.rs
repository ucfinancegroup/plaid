use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    assets_api: Box<dyn crate::apis::AssetsApi>,
    auth_api: Box<dyn crate::apis::AuthApi>,
    balance_api: Box<dyn crate::apis::BalanceApi>,
    categories_api: Box<dyn crate::apis::CategoriesApi>,
    identity_api: Box<dyn crate::apis::IdentityApi>,
    income_api: Box<dyn crate::apis::IncomeApi>,
    institutions_api: Box<dyn crate::apis::InstitutionsApi>,
    investments_api: Box<dyn crate::apis::InvestmentsApi>,
    item_creation_api: Box<dyn crate::apis::ItemCreationApi>,
    item_management_api: Box<dyn crate::apis::ItemManagementApi>,
    liabilities_api: Box<dyn crate::apis::LiabilitiesApi>,
    liabilities_report_api: Box<dyn crate::apis::LiabilitiesReportApi>,
    link_tokens_api: Box<dyn crate::apis::LinkTokensApi>,
    payment_initiation_uk_only_api: Box<dyn crate::apis::PaymentInitiationUKOnlyApi>,
    transactions_api: Box<dyn crate::apis::TransactionsApi>,
    webhooks_api: Box<dyn crate::apis::WebhooksApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            assets_api: Box::new(crate::apis::AssetsApiClient::new(rc.clone())),
            auth_api: Box::new(crate::apis::AuthApiClient::new(rc.clone())),
            balance_api: Box::new(crate::apis::BalanceApiClient::new(rc.clone())),
            categories_api: Box::new(crate::apis::CategoriesApiClient::new(rc.clone())),
            identity_api: Box::new(crate::apis::IdentityApiClient::new(rc.clone())),
            income_api: Box::new(crate::apis::IncomeApiClient::new(rc.clone())),
            institutions_api: Box::new(crate::apis::InstitutionsApiClient::new(rc.clone())),
            investments_api: Box::new(crate::apis::InvestmentsApiClient::new(rc.clone())),
            item_creation_api: Box::new(crate::apis::ItemCreationApiClient::new(rc.clone())),
            item_management_api: Box::new(crate::apis::ItemManagementApiClient::new(rc.clone())),
            liabilities_api: Box::new(crate::apis::LiabilitiesApiClient::new(rc.clone())),
            liabilities_report_api: Box::new(crate::apis::LiabilitiesReportApiClient::new(rc.clone())),
            link_tokens_api: Box::new(crate::apis::LinkTokensApiClient::new(rc.clone())),
            payment_initiation_uk_only_api: Box::new(crate::apis::PaymentInitiationUKOnlyApiClient::new(rc.clone())),
            transactions_api: Box::new(crate::apis::TransactionsApiClient::new(rc.clone())),
            webhooks_api: Box::new(crate::apis::WebhooksApiClient::new(rc.clone())),
        }
    }

    pub fn assets_api(&self) -> &dyn crate::apis::AssetsApi{
        self.assets_api.as_ref()
    }

    pub fn auth_api(&self) -> &dyn crate::apis::AuthApi{
        self.auth_api.as_ref()
    }

    pub fn balance_api(&self) -> &dyn crate::apis::BalanceApi{
        self.balance_api.as_ref()
    }

    pub fn categories_api(&self) -> &dyn crate::apis::CategoriesApi{
        self.categories_api.as_ref()
    }

    pub fn identity_api(&self) -> &dyn crate::apis::IdentityApi{
        self.identity_api.as_ref()
    }

    pub fn income_api(&self) -> &dyn crate::apis::IncomeApi{
        self.income_api.as_ref()
    }

    pub fn institutions_api(&self) -> &dyn crate::apis::InstitutionsApi{
        self.institutions_api.as_ref()
    }

    pub fn investments_api(&self) -> &dyn crate::apis::InvestmentsApi{
        self.investments_api.as_ref()
    }

    pub fn item_creation_api(&self) -> &dyn crate::apis::ItemCreationApi{
        self.item_creation_api.as_ref()
    }

    pub fn item_management_api(&self) -> &dyn crate::apis::ItemManagementApi{
        self.item_management_api.as_ref()
    }

    pub fn liabilities_api(&self) -> &dyn crate::apis::LiabilitiesApi{
        self.liabilities_api.as_ref()
    }

    pub fn liabilities_report_api(&self) -> &dyn crate::apis::LiabilitiesReportApi{
        self.liabilities_report_api.as_ref()
    }

    pub fn link_tokens_api(&self) -> &dyn crate::apis::LinkTokensApi{
        self.link_tokens_api.as_ref()
    }

    pub fn payment_initiation_uk_only_api(&self) -> &dyn crate::apis::PaymentInitiationUKOnlyApi{
        self.payment_initiation_uk_only_api.as_ref()
    }

    pub fn transactions_api(&self) -> &dyn crate::apis::TransactionsApi{
        self.transactions_api.as_ref()
    }

    pub fn webhooks_api(&self) -> &dyn crate::apis::WebhooksApi{
        self.webhooks_api.as_ref()
    }

}
