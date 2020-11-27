# \WebhooksApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fire_webhook_sandbox_only**](WebhooksApi.md#fire_webhook_sandbox_only) | **post** /sandbox/item/fire_webhook | Fire Webhook [Sandbox Only]
[**get_webhook_verification_key**](WebhooksApi.md#get_webhook_verification_key) | **post** /webhook_verification_key/get | Get Webhook Verification Key



## fire_webhook_sandbox_only

> crate::models::FireWebhookSandboxOnlyResponse fire_webhook_sandbox_only(fire_webhook_sandbox_only_request)
Fire Webhook [Sandbox Only]

Fire sandbox webhook for item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fire_webhook_sandbox_only_request** | [**FireWebhookSandboxOnlyRequest**](FireWebhookSandboxOnlyRequest.md) |  | [required] |

### Return type

[**crate::models::FireWebhookSandboxOnlyResponse**](FireWebhookSandboxOnlyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_verification_key

> crate::models::GetWebhookVerificationKeyResponse get_webhook_verification_key(get_webhook_verification_key_request)
Get Webhook Verification Key

Get Webhook Verification Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_webhook_verification_key_request** | [**GetWebhookVerificationKeyRequest**](GetWebhookVerificationKeyRequest.md) |  | [required] |

### Return type

[**crate::models::GetWebhookVerificationKeyResponse**](GetWebhookVerificationKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

