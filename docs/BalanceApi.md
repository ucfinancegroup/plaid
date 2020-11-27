# \BalanceApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_balance**](BalanceApi.md#retrieve_balance) | **post** /accounts/balance/get | Retrieve Balance



## retrieve_balance

> crate::models::RetrieveBalanceResponse retrieve_balance(retrieve_balance_request)
Retrieve Balance

The `/accounts/balance/get` endpoint returns the real-time balance for each of an `Item`’s accounts. It can be used for existing `Item`s that were added via any of Plaid’s other products.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_balance_request** | [**RetrieveBalanceRequest**](RetrieveBalanceRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveBalanceResponse**](RetrieveBalanceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

