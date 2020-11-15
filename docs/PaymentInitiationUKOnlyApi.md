# \PaymentInitiationUKOnlyApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment**](PaymentInitiationUKOnlyApi.md#create_payment) | **Post** /payment_initiation/payment/create | Create Payment
[**create_payment_recipient**](PaymentInitiationUKOnlyApi.md#create_payment_recipient) | **Post** /payment_initiation/recipient/create | Create Payment Recipient
[**create_payment_token**](PaymentInitiationUKOnlyApi.md#create_payment_token) | **Post** /payment_initiation/payment/token/create | Create Payment Token
[**get_payment**](PaymentInitiationUKOnlyApi.md#get_payment) | **Post** /payment_initiation/payment/get | Get Payment
[**get_payment_recipient**](PaymentInitiationUKOnlyApi.md#get_payment_recipient) | **Post** /payment_initiation/recipient/get | Get Payment Recipient
[**list_payment_recipients**](PaymentInitiationUKOnlyApi.md#list_payment_recipients) | **Post** /payment_initiation/recipient/list | List Payment Recipients
[**list_payments**](PaymentInitiationUKOnlyApi.md#list_payments) | **Post** /payment_initiation/payment/list | List Payments



## create_payment

> create_payment(create_payment_request)
Create Payment

Create and configure a payment by specifying a `recipient_id`, `reference`, `amount`, and `currency` to create a Plaid payment resource. The Plaid API returns a `payment_id` to identify the created payment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_request** | [**CreatePaymentRequest**](CreatePaymentRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_payment_recipient

> create_payment_recipient(create_payment_recipient_request)
Create Payment Recipient

In the sandbox environment, you can use the `/payment_initiation/recipient/create` endpoint to generate recipients. Programmatic recipient creation in the development and production environments can be done after approval by our compliance team.  The endpoint is idempotent: if a developer has already made a request with the same payment details, Plaid will return the same recipient_id. Recipients are scoped per environment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_recipient_request** | [**CreatePaymentRecipientRequest**](CreatePaymentRecipientRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_payment_token

> create_payment_token(create_payment_token_request)
Create Payment Token

In order to actually initiate a Payment, you must use the `payment_id` returned from `payment_initiation/payment/create` and create a `payment_token`.  The `payment_token` will allow for at most one payment initiation. If this attempt fails, the end user aborts the flow, or the token expires, the developer will need to create a new payment token. Creating a new payment token does not require end user input.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_token_request** | [**CreatePaymentTokenRequest**](CreatePaymentTokenRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment

> get_payment(get_payment_request)
Get Payment

The endpoint `payment_initiation/payment/get` takes a `payment_id` and returns all of the payment details for a previously created payment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_payment_request** | [**GetPaymentRequest**](GetPaymentRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_recipient

> get_payment_recipient(get_payment_recipient_request)
Get Payment Recipient

The `/payment_initiation/recipient/get` endpoint allows you to retrieve all of the details for a recipient that you have created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_payment_recipient_request** | [**GetPaymentRecipientRequest**](GetPaymentRecipientRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payment_recipients

> list_payment_recipients(list_payment_recipients_request)
List Payment Recipients

The `/payment_initiation/recipient/list` endpoint allows you to retrieve details for all recipients that you have created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_payment_recipients_request** | [**ListPaymentRecipientsRequest**](ListPaymentRecipientsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payments

> list_payments(list_payments_request)
List Payments

The endpoint `payment_initiation/payment/get` list payment details for all of your previously created payments.  This endpoint accepts `count` and `cursor` as optional parameters in order to support pagination. `count` limits how many payments are returned and can be set between 0 and 200. `cursor` should be a string in RFC 3339 format (i.e. `\"2019-12-06T22:35:49Z\"`). Only payments created before the `cursor` will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_payments_request** | [**ListPaymentsRequest**](ListPaymentsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

