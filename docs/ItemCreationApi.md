# \ItemCreationApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_item_sandbox_only**](ItemCreationApi.md#create_item_sandbox_only) | **post** /sandbox/public_token/create | Create Item [Sandbox Only]
[**exchange_token**](ItemCreationApi.md#exchange_token) | **post** /item/public_token/exchange | Exchange Token



## create_item_sandbox_only

> create_item_sandbox_only(create_item_sandbox_only_request)
Create Item [Sandbox Only]

Creates an `Item` in the `sandbox` environment. You will get back a `public_token` and a `request_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_item_sandbox_only_request** | [**CreateItemSandboxOnlyRequest**](CreateItemSandboxOnlyRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchange_token

> crate::models::ExchangeTokenExample exchange_token(exchange_token_request)
Exchange Token

Goes through the exchange token process where the `public_token` is exchanged for an `access_token`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_token_request** | [**ExchangeTokenRequest**](ExchangeTokenRequest.md) |  | [required] |

### Return type

[**crate::models::ExchangeTokenExample**](ExchangeTokenExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

