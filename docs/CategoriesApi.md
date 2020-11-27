# \CategoriesApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_categories**](CategoriesApi.md#retrieve_categories) | **post** /categories/get | Retrieve Categories



## retrieve_categories

> crate::models::RetrieveCategoriesExample retrieve_categories(body)
Retrieve Categories

This endpoint allows you to get to get detailed information on categories returned by Plaid. This endpoint does not require authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::RetrieveCategoriesExample**](RetrieveCategoriesExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

