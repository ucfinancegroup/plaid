# \TransactionsApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_transactions**](TransactionsApi.md#retrieve_transactions) | **post** /transactions/get | Retrieve Transactions



## retrieve_transactions

> crate::models::RetrieveTransactionsResponse retrieve_transactions(retrieve_transactions_request)
Retrieve Transactions

The `/transactions/get` endpoint allows developers to receive user-authorized transaction data for credit and depository-type Accounts. Transaction data is standardized across financial institutions, and in many cases transactions are linked to a clean name, entity type, location, and category. Similarly, account data is standardized and returned with a clean name, number, balance, and other meta information where available.   <br />  You must wait to retrieve transactions after creating an `Item` as it takes a couple seconds for Plaid to grab `transaction` data initially. If you don't, you will receive a `PRODUCT_NOT_READY` error. A good practice is to retrieve the data when you are notified that it's ready via webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_transactions_request** | [**RetrieveTransactionsRequest**](RetrieveTransactionsRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveTransactionsResponse**](RetrieveTransactionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

