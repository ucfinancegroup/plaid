# \AuthApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_auth**](AuthApi.md#retrieve_auth) | **post** /auth/get | Retrieve Auth



## retrieve_auth

> crate::models::RetrieveAuthEftExample retrieve_auth(retrieve_auth_request)
Retrieve Auth

The `/auth/get` endpoint allows you to retrieve the bank account and routing numbers associated with an Item’s checking and savings accounts, along with high-level account data and balances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_auth_request** | [**RetrieveAuthRequest**](RetrieveAuthRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveAuthEftExample**](RetrieveAuthEFTExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

