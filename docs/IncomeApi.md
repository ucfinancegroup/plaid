# \IncomeApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_income**](IncomeApi.md#retrieve_income) | **post** /income/get | Retrieve Income



## retrieve_income

> crate::models::RetrieveIncomeResponse retrieve_income(retrieve_income_request)
Retrieve Income

The `/income/get` endpoint allows you to retrieve information pertaining to a `Item`’s income. In addition to the annual income, detailed information will be provided for each contributing income stream (or job). Details on each of these fields can be found below.   <br /> Just like retrieving `transaction` data, you need to wait several seconds to retrieve `income` data after `Item` creation. A good practice is to retrieve the data when you are notified that it's ready via webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_income_request** | [**RetrieveIncomeRequest**](RetrieveIncomeRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveIncomeResponse**](RetrieveIncomeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

