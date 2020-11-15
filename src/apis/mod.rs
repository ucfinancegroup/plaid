use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod assets_api;
pub use self::assets_api::{ AssetsApi, AssetsApiClient };
mod auth_api;
pub use self::auth_api::{ AuthApi, AuthApiClient };
mod balance_api;
pub use self::balance_api::{ BalanceApi, BalanceApiClient };
mod categories_api;
pub use self::categories_api::{ CategoriesApi, CategoriesApiClient };
mod identity_api;
pub use self::identity_api::{ IdentityApi, IdentityApiClient };
mod income_api;
pub use self::income_api::{ IncomeApi, IncomeApiClient };
mod institutions_api;
pub use self::institutions_api::{ InstitutionsApi, InstitutionsApiClient };
mod investments_api;
pub use self::investments_api::{ InvestmentsApi, InvestmentsApiClient };
mod item_creation_api;
pub use self::item_creation_api::{ ItemCreationApi, ItemCreationApiClient };
mod item_management_api;
pub use self::item_management_api::{ ItemManagementApi, ItemManagementApiClient };
mod liabilities_api;
pub use self::liabilities_api::{ LiabilitiesApi, LiabilitiesApiClient };
mod liabilities_report_api;
pub use self::liabilities_report_api::{ LiabilitiesReportApi, LiabilitiesReportApiClient };
mod link_tokens_api;
pub use self::link_tokens_api::{ LinkTokensApi, LinkTokensApiClient };
mod payment_initiation_uk_only_api;
pub use self::payment_initiation_uk_only_api::{ PaymentInitiationUKOnlyApi, PaymentInitiationUKOnlyApiClient };
mod transactions_api;
pub use self::transactions_api::{ TransactionsApi, TransactionsApiClient };
mod webhooks_api;
pub use self::webhooks_api::{ WebhooksApi, WebhooksApiClient };

pub mod configuration;
pub mod client;
