# \ItemManagementApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**automated_deposit_webhook**](ItemManagementApi.md#automated_deposit_webhook) | **post** /sandbox/item/set_verification_status | Automated Deposit Webhook
[**create_public_tokenfor_update**](ItemManagementApi.md#create_public_tokenfor_update) | **post** /item/public_token/create | Create Public Token for Update
[**remove_item**](ItemManagementApi.md#remove_item) | **post** /item/remove | Remove Item
[**retrieve_an_items_accounts**](ItemManagementApi.md#retrieve_an_items_accounts) | **post** /accounts/get | Retrieve an Item's Accounts
[**retrieve_item**](ItemManagementApi.md#retrieve_item) | **post** /item/get | Retrieve Item
[**rotate_access_token**](ItemManagementApi.md#rotate_access_token) | **post** /item/access_token/invalidate | Rotate Access Token
[**simulate_itemloginrequired_sandbox_only**](ItemManagementApi.md#simulate_itemloginrequired_sandbox_only) | **post** /sandbox/item/reset_login | Simulate ITEM_LOGIN_REQUIRED [Sandbox Only]
[**update_an_items_webhook**](ItemManagementApi.md#update_an_items_webhook) | **post** /item/webhook/update | Update an Item's Webhook



## automated_deposit_webhook

> crate::models::AutomatedDepositWebhookResponse automated_deposit_webhook(automated_deposit_webhook_request)
Automated Deposit Webhook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**automated_deposit_webhook_request** | [**AutomatedDepositWebhookRequest**](AutomatedDepositWebhookRequest.md) |  | [required] |

### Return type

[**crate::models::AutomatedDepositWebhookResponse**](AutomatedDepositWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_public_tokenfor_update

> crate::models::CreatePublicTokenforUpdateResponse create_public_tokenfor_update(create_public_tokenfor_update_request)
Create Public Token for Update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_public_tokenfor_update_request** | [**CreatePublicTokenforUpdateRequest**](CreatePublicTokenforUpdateRequest.md) |  | [required] |

### Return type

[**crate::models::CreatePublicTokenforUpdateResponse**](CreatePublicTokenforUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_item

> crate::models::RemoveItemResponse remove_item(remove_item_request)
Remove Item

This endpoint allows you to remove an `Item` using its `access_token`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_item_request** | [**RemoveItemRequest**](RemoveItemRequest.md) |  | [required] |

### Return type

[**crate::models::RemoveItemResponse**](RemoveItemResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_an_items_accounts

> crate::models::RetrieveAnItemsAccountsResponse retrieve_an_items_accounts(retrieve_an_items_accounts_request)
Retrieve an Item's Accounts

This endpoint allows you to retrieve all available `Account`s associated with an `Item`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_an_items_accounts_request** | [**RetrieveAnItemsAccountsRequest**](RetrieveAnItemsAccountsRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveAnItemsAccountsResponse**](RetrieveAnItemsAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_item

> crate::models::RetrieveItemResponse retrieve_item(retrieve_item_request)
Retrieve Item

This endpoint allows you to retrieve information about an `Item`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_item_request** | [**RetrieveItemRequest**](RetrieveItemRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveItemResponse**](RetrieveItemResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_access_token

> crate::models::RotateAccessTokenResponse rotate_access_token(rotate_access_token_request)
Rotate Access Token

By default, the `access_token` associated with an `Item` does not expire and should be stored in a persistent, secure manner.   <br /> You can use the POST `/item/access_token/invalidate` endpoint to rotate the `access_token` associated with an `Item`. The endpoint returns a new `access_token` and immediately invalidates the previous `access_token`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rotate_access_token_request** | [**RotateAccessTokenRequest**](RotateAccessTokenRequest.md) |  | [required] |

### Return type

[**crate::models::RotateAccessTokenResponse**](RotateAccessTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## simulate_itemloginrequired_sandbox_only

> crate::models::SimulateItemLoginRequiredSandboxOnlyResponse simulate_itemloginrequired_sandbox_only(simulate_item_login_required_sandbox_only_request)
Simulate ITEM_LOGIN_REQUIRED [Sandbox Only]

An `Item` may transition into an error state in response to changes made by the user or financial institution. The most common scenarios are when a user changes their password or when the financial institution changes their multi-factor authentication flow. [Plaid Link](https://plaid.com/docs/api/#updating-items-via-link) makes it easy to restore a user's `Item` to a good state by having them provide updated credentials and MFA information, if needed.   <br /> In the Sandbox, `Item`s transition to an `ITEM_LOGIN_REQUIRED` error state automatically after 30 days. You can also simulate this event via an API request.   <br /> The `/sandbox/item/reset_login` endpoint allows you put an Item in an `ITEM_LOGIN_REQUIRED` error state. You can then use [Plaid Link update mode](https://plaid.com/docs/api/#updating-items-via-link) to restore the `Item` to a good state.   <br /> An `ITEM_LOGIN_REQUIRED` webhook will be fired after a call to this endpoint, if one is associated with the `Item`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**simulate_item_login_required_sandbox_only_request** | [**SimulateItemLoginRequiredSandboxOnlyRequest**](SimulateItemLoginRequiredSandboxOnlyRequest.md) |  | [required] |

### Return type

[**crate::models::SimulateItemLoginRequiredSandboxOnlyResponse**](SimulateITEM_LOGIN_REQUIREDSandboxOnlyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_an_items_webhook

> crate::models::UpdateAnItemsWebhookResponse update_an_items_webhook(update_an_items_webhook_request)
Update an Item's Webhook

This endpoint allows you to update the webhook url for an `Item`. This request triggers a `WEBHOOK_UPDATE_ACKNOWLEDGED` [webhook](https://plaid.com/docs/api/#item-webhooks).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_an_items_webhook_request** | [**UpdateAnItemsWebhookRequest**](UpdateAnItemsWebhookRequest.md) |  | [required] |

### Return type

[**crate::models::UpdateAnItemsWebhookResponse**](UpdateAnItemsWebhookResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

