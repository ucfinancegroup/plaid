# Rust API client for plaid

A collection of Plaid API endpoints for the `sandbox` environment. Each endpoint request comes with an example request & response. It also contains 'use cases' for each product.  
<br />
Before you begin, please set your `client_id` and `secret_key` variables in the Sandbox environment. You can find them in your Plaid [dashboard](https://dashboard.plaid.com/account/keys). Set the variables by clicking on the 'eye' icon in the top-right corner of the screen.

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *https://sandbox.plaid.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AssetsApi* | [**create_asset_report**](docs/AssetsApi.md#create_asset_report) | **Post** /asset_report/create | Create Asset Report
*AssetsApi* | [**create_audit_copy**](docs/AssetsApi.md#create_audit_copy) | **Post** /asset_report/audit_copy/create | Create Audit Copy
*AssetsApi* | [**refresh_asset_report**](docs/AssetsApi.md#refresh_asset_report) | **Post** /asset_report/refresh | Refresh Asset Report
*AssetsApi* | [**remove_asset_report**](docs/AssetsApi.md#remove_asset_report) | **Post** /asset_report/remove | Remove Asset Report
*AssetsApi* | [**remove_audit_copy**](docs/AssetsApi.md#remove_audit_copy) | **Post** /asset_report/audit_copy/remove | Remove Audit Copy
*AssetsApi* | [**retrievean_asset_report_json**](docs/AssetsApi.md#retrievean_asset_report_json) | **Post** /asset_report/get | Retrieve an Asset Report (JSON)
*AssetsApi* | [**retrievean_asset_report_pdf**](docs/AssetsApi.md#retrievean_asset_report_pdf) | **Post** /asset_report/pdf/get | Retrieve an Asset Report (PDF)
*AuthApi* | [**retrieve_auth**](docs/AuthApi.md#retrieve_auth) | **Post** /auth/get | Retrieve Auth
*BalanceApi* | [**retrieve_balance**](docs/BalanceApi.md#retrieve_balance) | **Post** /accounts/balance/get | Retrieve Balance
*CategoriesApi* | [**retrieve_categories**](docs/CategoriesApi.md#retrieve_categories) | **Post** /categories/get | Retrieve Categories
*IdentityApi* | [**retrieve_identity**](docs/IdentityApi.md#retrieve_identity) | **Post** /identity/get | Retrieve Identity
*IncomeApi* | [**retrieve_income**](docs/IncomeApi.md#retrieve_income) | **Post** /income/get | Retrieve Income
*InstitutionsApi* | [**retrieve_insitution_list**](docs/InstitutionsApi.md#retrieve_insitution_list) | **Post** /institutions/get | Retrieve Insitution List
*InstitutionsApi* | [**search_institutionby_id**](docs/InstitutionsApi.md#search_institutionby_id) | **Post** /institutions/get_by_id | Search Institution by ID
*InstitutionsApi* | [**search_institutionby_name**](docs/InstitutionsApi.md#search_institutionby_name) | **Post** /institutions/search | Search Institution by Name
*InvestmentsApi* | [**retrieve_investments_holdings**](docs/InvestmentsApi.md#retrieve_investments_holdings) | **Post** /investments/holdings/get | Retrieve Investments Holdings
*InvestmentsApi* | [**retrieve_investments_transactions**](docs/InvestmentsApi.md#retrieve_investments_transactions) | **Post** /investments/transactions/get | Retrieve Investments Transactions
*ItemCreationApi* | [**create_item_sandbox_only**](docs/ItemCreationApi.md#create_item_sandbox_only) | **Post** /sandbox/public_token/create | Create Item [Sandbox Only]
*ItemCreationApi* | [**exchange_token**](docs/ItemCreationApi.md#exchange_token) | **Post** /item/public_token/exchange | Exchange Token
*ItemManagementApi* | [**automated_deposit_webhook**](docs/ItemManagementApi.md#automated_deposit_webhook) | **Post** /sandbox/item/set_verification_status | Automated Deposit Webhook
*ItemManagementApi* | [**create_public_tokenfor_update**](docs/ItemManagementApi.md#create_public_tokenfor_update) | **Post** /item/public_token/create | Create Public Token for Update
*ItemManagementApi* | [**remove_item**](docs/ItemManagementApi.md#remove_item) | **Post** /item/remove | Remove Item
*ItemManagementApi* | [**retrieve_an_items_accounts**](docs/ItemManagementApi.md#retrieve_an_items_accounts) | **Post** /accounts/get | Retrieve an Item's Accounts
*ItemManagementApi* | [**retrieve_item**](docs/ItemManagementApi.md#retrieve_item) | **Post** /item/get | Retrieve Item
*ItemManagementApi* | [**rotate_access_token**](docs/ItemManagementApi.md#rotate_access_token) | **Post** /item/access_token/invalidate | Rotate Access Token
*ItemManagementApi* | [**simulate_itemloginrequired_sandbox_only**](docs/ItemManagementApi.md#simulate_itemloginrequired_sandbox_only) | **Post** /sandbox/item/reset_login | Simulate ITEM_LOGIN_REQUIRED [Sandbox Only]
*ItemManagementApi* | [**update_an_items_webhook**](docs/ItemManagementApi.md#update_an_items_webhook) | **Post** /item/webhook/update | Update an Item's Webhook
*LiabilitiesApi* | [**retrieve_liabilities**](docs/LiabilitiesApi.md#retrieve_liabilities) | **Post** /liabilities/get | Retrieve Liabilities
*LiabilitiesReportApi* | [**create_liabilities_report**](docs/LiabilitiesReportApi.md#create_liabilities_report) | **Post** /liabilities_report/create | Create Liabilities Report
*LiabilitiesReportApi* | [**remove_liabilities_report**](docs/LiabilitiesReportApi.md#remove_liabilities_report) | **Post** /liabilities_report/remove | Remove Liabilities Report
*LiabilitiesReportApi* | [**retrieve_liabilities_report**](docs/LiabilitiesReportApi.md#retrieve_liabilities_report) | **Post** /liabilities_report/get | Retrieve Liabilities Report
*LinkTokensApi* | [**create_link_token**](docs/LinkTokensApi.md#create_link_token) | **Post** /link/token/create | Create Link Token
*PaymentInitiationUKOnlyApi* | [**create_payment**](docs/PaymentInitiationUKOnlyApi.md#create_payment) | **Post** /payment_initiation/payment/create | Create Payment
*PaymentInitiationUKOnlyApi* | [**create_payment_recipient**](docs/PaymentInitiationUKOnlyApi.md#create_payment_recipient) | **Post** /payment_initiation/recipient/create | Create Payment Recipient
*PaymentInitiationUKOnlyApi* | [**create_payment_token**](docs/PaymentInitiationUKOnlyApi.md#create_payment_token) | **Post** /payment_initiation/payment/token/create | Create Payment Token
*PaymentInitiationUKOnlyApi* | [**get_payment**](docs/PaymentInitiationUKOnlyApi.md#get_payment) | **Post** /payment_initiation/payment/get | Get Payment
*PaymentInitiationUKOnlyApi* | [**get_payment_recipient**](docs/PaymentInitiationUKOnlyApi.md#get_payment_recipient) | **Post** /payment_initiation/recipient/get | Get Payment Recipient
*PaymentInitiationUKOnlyApi* | [**list_payment_recipients**](docs/PaymentInitiationUKOnlyApi.md#list_payment_recipients) | **Post** /payment_initiation/recipient/list | List Payment Recipients
*PaymentInitiationUKOnlyApi* | [**list_payments**](docs/PaymentInitiationUKOnlyApi.md#list_payments) | **Post** /payment_initiation/payment/list | List Payments
*TransactionsApi* | [**retrieve_transactions**](docs/TransactionsApi.md#retrieve_transactions) | **Post** /transactions/get | Retrieve Transactions
*WebhooksApi* | [**fire_webhook_sandbox_only**](docs/WebhooksApi.md#fire_webhook_sandbox_only) | **Post** /sandbox/item/fire_webhook | Fire Webhook [Sandbox Only]
*WebhooksApi* | [**get_webhook_verification_key**](docs/WebhooksApi.md#get_webhook_verification_key) | **Post** /webhook_verification_key/get | Get Webhook Verification Key


## Documentation For Models

 - [Account](docs/Account.md)
 - [Account5](docs/Account5.md)
 - [Account6](docs/Account6.md)
 - [Address](docs/Address.md)
 - [Address1](docs/Address1.md)
 - [Address2](docs/Address2.md)
 - [Address3](docs/Address3.md)
 - [Amount](docs/Amount.md)
 - [Apr](docs/Apr.md)
 - [AutomatedDepositWebhookRequest](docs/AutomatedDepositWebhookRequest.md)
 - [Balances](docs/Balances.md)
 - [Balances5](docs/Balances5.md)
 - [Balances6](docs/Balances6.md)
 - [Category](docs/Category.md)
 - [CreateAssetReportExample](docs/CreateAssetReportExample.md)
 - [CreateAssetReportRequest](docs/CreateAssetReportRequest.md)
 - [CreateAuditCopyExample](docs/CreateAuditCopyExample.md)
 - [CreateAuditCopyRequest](docs/CreateAuditCopyRequest.md)
 - [CreateItemCustomSandboxOnlyRequest](docs/CreateItemCustomSandboxOnlyRequest.md)
 - [CreateItemSandboxOnlyRequest](docs/CreateItemSandboxOnlyRequest.md)
 - [CreateLiabilitiesReportExample](docs/CreateLiabilitiesReportExample.md)
 - [CreateLiabilitiesReportRequest](docs/CreateLiabilitiesReportRequest.md)
 - [CreateLinkTokenOAuthRequest](docs/CreateLinkTokenOAuthRequest.md)
 - [CreateLinkTokenPaymentInitiationRequest](docs/CreateLinkTokenPaymentInitiationRequest.md)
 - [CreateLinkTokenRequest](docs/CreateLinkTokenRequest.md)
 - [CreateLinkTokenUpdateModeRequest](docs/CreateLinkTokenUpdateModeRequest.md)
 - [CreatePaymentRecipientRequest](docs/CreatePaymentRecipientRequest.md)
 - [CreatePaymentRequest](docs/CreatePaymentRequest.md)
 - [CreatePaymentTokenRequest](docs/CreatePaymentTokenRequest.md)
 - [CreatePublicTokenforUpdateRequest](docs/CreatePublicTokenforUpdateRequest.md)
 - [Credential](docs/Credential.md)
 - [Credit](docs/Credit.md)
 - [Data](docs/Data.md)
 - [Data2](docs/Data2.md)
 - [Eft](docs/Eft.md)
 - [Email](docs/Email.md)
 - [ExchangeTokenExample](docs/ExchangeTokenExample.md)
 - [ExchangeTokenRequest](docs/ExchangeTokenRequest.md)
 - [FireWebhookSandboxOnlyRequest](docs/FireWebhookSandboxOnlyRequest.md)
 - [GetPaymentRecipientRequest](docs/GetPaymentRecipientRequest.md)
 - [GetPaymentRequest](docs/GetPaymentRequest.md)
 - [GetWebhookVerificationKeyRequest](docs/GetWebhookVerificationKeyRequest.md)
 - [HistoricalBalance](docs/HistoricalBalance.md)
 - [Identity](docs/Identity.md)
 - [Income](docs/Income.md)
 - [IncomeStream](docs/IncomeStream.md)
 - [Institution](docs/Institution.md)
 - [InterestRate](docs/InterestRate.md)
 - [Item](docs/Item.md)
 - [Item7](docs/Item7.md)
 - [Item8](docs/Item8.md)
 - [Liability](docs/Liability.md)
 - [ListPaymentRecipientsRequest](docs/ListPaymentRecipientsRequest.md)
 - [ListPaymentsRequest](docs/ListPaymentsRequest.md)
 - [LoanStatus](docs/LoanStatus.md)
 - [Location](docs/Location.md)
 - [Mortgage](docs/Mortgage.md)
 - [Numbers](docs/Numbers.md)
 - [Options](docs/Options.md)
 - [Options1](docs/Options1.md)
 - [Options2](docs/Options2.md)
 - [Owner](docs/Owner.md)
 - [Owner1](docs/Owner1.md)
 - [PaymentInitiation](docs/PaymentInitiation.md)
 - [PaymentMeta](docs/PaymentMeta.md)
 - [PhoneNumber](docs/PhoneNumber.md)
 - [PropertyAddress](docs/PropertyAddress.md)
 - [PslfStatus](docs/PslfStatus.md)
 - [RefreshAssetReportExample](docs/RefreshAssetReportExample.md)
 - [RefreshAssetReportRequest](docs/RefreshAssetReportRequest.md)
 - [RemoveAssetReportExample](docs/RemoveAssetReportExample.md)
 - [RemoveAssetReportRequest](docs/RemoveAssetReportRequest.md)
 - [RemoveAuditCopyExample](docs/RemoveAuditCopyExample.md)
 - [RemoveAuditCopyRequest](docs/RemoveAuditCopyRequest.md)
 - [RemoveItemExample](docs/RemoveItemExample.md)
 - [RemoveItemRequest](docs/RemoveItemRequest.md)
 - [RemoveLiabilitiesReportExample](docs/RemoveLiabilitiesReportExample.md)
 - [RemoveLiabilitiesReportRequest](docs/RemoveLiabilitiesReportRequest.md)
 - [RepaymentPlan](docs/RepaymentPlan.md)
 - [Report](docs/Report.md)
 - [Report1](docs/Report1.md)
 - [RetrieveAnItemsAccountsExample](docs/RetrieveAnItemsAccountsExample.md)
 - [RetrieveAnItemsAccountsRequest](docs/RetrieveAnItemsAccountsRequest.md)
 - [RetrieveAuthEftExample](docs/RetrieveAuthEftExample.md)
 - [RetrieveAuthRequest](docs/RetrieveAuthRequest.md)
 - [RetrieveBalanceExample](docs/RetrieveBalanceExample.md)
 - [RetrieveBalanceRequest](docs/RetrieveBalanceRequest.md)
 - [RetrieveCategoriesExample](docs/RetrieveCategoriesExample.md)
 - [RetrieveIdentityExample](docs/RetrieveIdentityExample.md)
 - [RetrieveIdentityRequest](docs/RetrieveIdentityRequest.md)
 - [RetrieveIncomeExample](docs/RetrieveIncomeExample.md)
 - [RetrieveIncomeRequest](docs/RetrieveIncomeRequest.md)
 - [RetrieveInsitutionListExample](docs/RetrieveInsitutionListExample.md)
 - [RetrieveInsitutionListRequest](docs/RetrieveInsitutionListRequest.md)
 - [RetrieveInvestmentsHoldingsRequest](docs/RetrieveInvestmentsHoldingsRequest.md)
 - [RetrieveInvestmentsTransactionsRequest](docs/RetrieveInvestmentsTransactionsRequest.md)
 - [RetrieveItemExample](docs/RetrieveItemExample.md)
 - [RetrieveItemRequest](docs/RetrieveItemRequest.md)
 - [RetrieveLiabilitiesReportExample](docs/RetrieveLiabilitiesReportExample.md)
 - [RetrieveLiabilitiesReportRequest](docs/RetrieveLiabilitiesReportRequest.md)
 - [RetrieveLiabilitiesRequest](docs/RetrieveLiabilitiesRequest.md)
 - [RetrieveTransactionsExample](docs/RetrieveTransactionsExample.md)
 - [RetrieveTransactionsRequest](docs/RetrieveTransactionsRequest.md)
 - [RetrieveanAssetReportJsoNwithInsights](docs/RetrieveanAssetReportJsoNwithInsights.md)
 - [RetrieveanAssetReportJsonRequest](docs/RetrieveanAssetReportJsonRequest.md)
 - [RetrieveanAssetReportPdfRequest](docs/RetrieveanAssetReportPdfRequest.md)
 - [RotateAccessTokenExample](docs/RotateAccessTokenExample.md)
 - [RotateAccessTokenRequest](docs/RotateAccessTokenRequest.md)
 - [SearchInstitutionbyIdExample](docs/SearchInstitutionbyIdExample.md)
 - [SearchInstitutionbyIdRequest](docs/SearchInstitutionbyIdRequest.md)
 - [SearchInstitutionbyNameExample](docs/SearchInstitutionbyNameExample.md)
 - [SearchInstitutionbyNameRequest](docs/SearchInstitutionbyNameRequest.md)
 - [ServicerAddress](docs/ServicerAddress.md)
 - [SimulateItemLoginRequiredSandboxOnlyExample](docs/SimulateItemLoginRequiredSandboxOnlyExample.md)
 - [SimulateItemLoginRequiredSandboxOnlyRequest](docs/SimulateItemLoginRequiredSandboxOnlyRequest.md)
 - [Student](docs/Student.md)
 - [Transaction](docs/Transaction.md)
 - [Transaction1](docs/Transaction1.md)
 - [Transaction2](docs/Transaction2.md)
 - [UpdateAnItemsWebhookExample](docs/UpdateAnItemsWebhookExample.md)
 - [UpdateAnItemsWebhookRequest](docs/UpdateAnItemsWebhookRequest.md)
 - [User](docs/User.md)
 - [User4](docs/User4.md)
 - [User5](docs/User5.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


