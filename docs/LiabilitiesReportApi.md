# \LiabilitiesReportApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_liabilities_report**](LiabilitiesReportApi.md#create_liabilities_report) | **Post** /liabilities_report/create | Create Liabilities Report
[**remove_liabilities_report**](LiabilitiesReportApi.md#remove_liabilities_report) | **Post** /liabilities_report/remove | Remove Liabilities Report
[**retrieve_liabilities_report**](LiabilitiesReportApi.md#retrieve_liabilities_report) | **Post** /liabilities_report/get | Retrieve Liabilities Report



## create_liabilities_report

> crate::models::CreateLiabilitiesReportExample create_liabilities_report(create_liabilities_report_request)
Create Liabilities Report

With your desired `access_tokens` in hand, all you need to do to create a Liabilities Report is to call the `/liabilities_report/create` endpoint.  When creating a Liabilities Report, the only required fields are your `client_id`, `secret`, an `array of access_tokens` (one for each Item to be included in the Report), and the number of `days_requested` which determines the duration of transaction history to be included.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_liabilities_report_request** | [**CreateLiabilitiesReportRequest**](CreateLiabilitiesReportRequest.md) |  | [required] |

### Return type

[**crate::models::CreateLiabilitiesReportExample**](CreateLiabilitiesReportExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_liabilities_report

> crate::models::RemoveLiabilitiesReportExample remove_liabilities_report(remove_liabilities_report_request)
Remove Liabilities Report

The `/liabilities_report/remove` endpoint allows you to remove a Liabilities Report. Removing a Liabilities Report invalidates its `liabilities_report_token`, meaning you will no longer be able to use it to access report data. Removing a Liabilities Report does not affect the underlying Items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_liabilities_report_request** | [**RemoveLiabilitiesReportRequest**](RemoveLiabilitiesReportRequest.md) |  | [required] |

### Return type

[**crate::models::RemoveLiabilitiesReportExample**](RemoveLiabilitiesReportExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrieve_liabilities_report

> crate::models::RetrieveLiabilitiesReportExample retrieve_liabilities_report(retrieve_liabilities_report_request)
Retrieve Liabilities Report

You can retrieve your Liabilities Report in JSON by calling `/liabilities_report/get`.  <br /> Just like retrieving `transaction` data, you need to wait several seconds to retrieve a liabilities report after you create one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_liabilities_report_request** | [**RetrieveLiabilitiesReportRequest**](RetrieveLiabilitiesReportRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveLiabilitiesReportExample**](RetrieveLiabilitiesReportExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

