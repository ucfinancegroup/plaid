# \InvestmentsApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_investments_holdings**](InvestmentsApi.md#retrieve_investments_holdings) | **post** /investments/holdings/get | Retrieve Investments Holdings
[**retrieve_investments_transactions**](InvestmentsApi.md#retrieve_investments_transactions) | **post** /investments/transactions/get | Retrieve Investments Transactions



## retrieve_investments_holdings

> crate::models::RetrieveInvestmentsHoldingsResponse retrieve_investments_holdings(retrieve_investments_holdings_request)
Retrieve Investments Holdings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_investments_holdings_request** | [**RetrieveInvestmentsHoldingsRequest**](RetrieveInvestmentsHoldingsRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveInvestmentsHoldingsResponse**](RetrieveInvestmentsHoldingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_investments_transactions

> crate::models::RetrieveInvestmentsTransactionsRequest retrieve_investments_transactions(retrieve_investments_transactions_request)
Retrieve Investments Transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_investments_transactions_request** | [**RetrieveInvestmentsTransactionsRequest**](RetrieveInvestmentsTransactionsRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveInvestmentsTransactionsRequest**](RetrieveInvestmentsTransactionsRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

