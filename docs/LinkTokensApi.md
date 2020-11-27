# \LinkTokensApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_link_token**](LinkTokensApi.md#create_link_token) | **post** /link/token/create | Create Link Token



## create_link_token

> crate::models::CreateLinkTokenResponse create_link_token(create_link_token_request)
Create Link Token

Creates a link token with options. The link token can then be used to initialize Plaid Link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_link_token_request** | [**CreateLinkTokenRequest**](CreateLinkTokenRequest.md) |  | [required] |

### Return type

[**crate::models::CreateLinkTokenResponse**](CreateLinkTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

