# \AssetsApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_asset_report**](AssetsApi.md#create_asset_report) | **post** /asset_report/create | Create Asset Report
[**create_audit_copy**](AssetsApi.md#create_audit_copy) | **post** /asset_report/audit_copy/create | Create Audit Copy
[**refresh_asset_report**](AssetsApi.md#refresh_asset_report) | **post** /asset_report/refresh | Refresh Asset Report
[**remove_asset_report**](AssetsApi.md#remove_asset_report) | **post** /asset_report/remove | Remove Asset Report
[**remove_audit_copy**](AssetsApi.md#remove_audit_copy) | **post** /asset_report/audit_copy/remove | Remove Audit Copy
[**retrievean_asset_report_json**](AssetsApi.md#retrievean_asset_report_json) | **post** /asset_report/get | Retrieve an Asset Report (JSON)
[**retrievean_asset_report_pdf**](AssetsApi.md#retrievean_asset_report_pdf) | **post** /asset_report/pdf/get | Retrieve an Asset Report (PDF)



## create_asset_report

> crate::models::CreateAssetReportExample create_asset_report(create_asset_report_request)
Create Asset Report

With your desired `access_tokens` in hand, all you need to do to create an Asset Report is to call the `/asset_report/create` endpoint.  When creating an Asset Report, the only required fields are your `client_id`, `secret`, an `array of access_tokens` (one for each Item to be included in the Report), and the number of `days_requested` which determines the duration of transaction history to be included.  `options` is optional!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_asset_report_request** | [**CreateAssetReportRequest**](CreateAssetReportRequest.md) |  | [required] |

### Return type

[**crate::models::CreateAssetReportExample**](CreateAssetReportExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_audit_copy

> crate::models::CreateAuditCopyExample create_audit_copy(create_audit_copy_request)
Create Audit Copy

Plaid can provide an **Audit Copy** of any Asset Report directly to a participating third party on your behalf.   An Audit Copy contains the same underlying data as the Asset Report. To grant access to an Audit Copy, you’ll create an `audit_copy_token` for it and then pass that token to the third party who needs access. Each third party has its own `auditor_id`, for example `fannie_mae`. You’ll need to create a separate Audit Copy for each third party to whom you want to grant access to the report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_audit_copy_request** | [**CreateAuditCopyRequest**](CreateAuditCopyRequest.md) |  | [required] |

### Return type

[**crate::models::CreateAuditCopyExample**](CreateAuditCopyExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_asset_report

> crate::models::RefreshAssetReportExample refresh_asset_report(refresh_asset_report_request)
Refresh Asset Report

Create a new Asset Report based on the old one, but with the most recent data available from the financial institution(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_asset_report_request** | [**RefreshAssetReportRequest**](RefreshAssetReportRequest.md) |  | [required] |

### Return type

[**crate::models::RefreshAssetReportExample**](RefreshAssetReportExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_asset_report

> crate::models::RemoveAssetReportExample remove_asset_report(remove_asset_report_request)
Remove Asset Report

The `/asset_report/remove` endpoint allows you to remove an Asset Report. Removing an Asset Report invalidates its `asset_report_token`, meaning you will no longer be able to use it to access report data or create new Audit Copies. Removing an Asset Report does not affect the underlying Items, but does also invalidate any `audit_copy_token`s associated with the Asset Report. In other words, removing an Asset Report also cascade-removes its Audit Copies.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_asset_report_request** | [**RemoveAssetReportRequest**](RemoveAssetReportRequest.md) |  | [required] |

### Return type

[**crate::models::RemoveAssetReportExample**](RemoveAssetReportExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_audit_copy

> crate::models::RemoveAuditCopyExample remove_audit_copy(remove_audit_copy_request)
Remove Audit Copy

The `/asset_report/audit_copy/remove` endpoint allows you to remove an Audit Copy. Removing an Audit Copy invalidates the `audit_copy_token` associated with it, meaning both you and any third parties holding the token will no longer be able to use it to access report data. `Item`s associated with the Asset Report, the Asset Report itself and other Audit Copies of it are not affected and will remain accessible after removing the given Audit Copy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_audit_copy_request** | [**RemoveAuditCopyRequest**](RemoveAuditCopyRequest.md) |  | [required] |

### Return type

[**crate::models::RemoveAuditCopyExample**](RemoveAuditCopyExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrievean_asset_report_json

> crate::models::RetrieveanAssetReportJsoNwithInsights retrievean_asset_report_json(retrievean_asset_report_json_request)
Retrieve an Asset Report (JSON)

You can retrieve your Asset Report in multiple formats (PDF and JSON), determined by the endpoint you call.  JSON: `/asset_report/get`   PDF: `/asset_report/pdf/get`   <br /> Just like retrieving `transaction` data, you need to wait several seconds to retrieve an assets report after you create one. A good practice is to retrieve the data when you are notified that it's ready via webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrievean_asset_report_json_request** | [**RetrieveanAssetReportJsonRequest**](RetrieveanAssetReportJsonRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveanAssetReportJsoNwithInsights**](RetrieveanAssetReportJSONwithInsights.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## retrievean_asset_report_pdf

> retrievean_asset_report_pdf(retrievean_asset_report_pdf_request)
Retrieve an Asset Report (PDF)

You can retrieve your Asset Report in multiple formats (PDF and JSON), determined by the endpoint you call.  JSON: `/asset_report/get`   PDF: `/asset_report/pdf/get`   <br /> Just like retrieving `transaction` data, you need to wait several seconds to retrieve an assets report after you create one. A good practice is to retrieve the data when you are notified that it's ready via webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrievean_asset_report_pdf_request** | [**RetrieveanAssetReportPdfRequest**](RetrieveanAssetReportPdfRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

