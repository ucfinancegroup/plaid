# \IdentityApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_identity**](IdentityApi.md#retrieve_identity) | **post** /identity/get | Retrieve Identity



## retrieve_identity

> crate::models::RetrieveIdentityExample retrieve_identity(retrieve_identity_request)
Retrieve Identity

The `/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_identity_request** | [**RetrieveIdentityRequest**](RetrieveIdentityRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveIdentityExample**](RetrieveIdentityExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

