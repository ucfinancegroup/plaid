# \InstitutionsApi

All URIs are relative to *https://sandbox.plaid.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**retrieve_insitution_list**](InstitutionsApi.md#retrieve_insitution_list) | **post** /institutions/get | Retrieve Insitution List
[**search_institutionby_id**](InstitutionsApi.md#search_institutionby_id) | **post** /institutions/get_by_id | Search Institution by ID
[**search_institutionby_name**](InstitutionsApi.md#search_institutionby_name) | **post** /institutions/search | Search Institution by Name



## retrieve_insitution_list

> crate::models::RetrieveInsitutionListExample retrieve_insitution_list(retrieve_insitution_list_request)
Retrieve Insitution List

To see a full list of supported institutions across all products, use the `/institutions/get` and `/institutions/search` endpoints.   <br /> Use the `count` and `offset` query parameters to retrieve the desired institution data.    <br /> `count`: The total number of Institutions to return, with 0 < count <= 500. <br /> `offset`: The number of Institutions to skip before returning results, with offset >= 0

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**retrieve_insitution_list_request** | [**RetrieveInsitutionListRequest**](RetrieveInsitutionListRequest.md) |  | [required] |

### Return type

[**crate::models::RetrieveInsitutionListExample**](RetrieveInsitutionListExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_institutionby_id

> crate::models::SearchInstitutionbyIdExample search_institutionby_id(search_institutionby_id_request)
Search Institution by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_institutionby_id_request** | [**SearchInstitutionbyIdRequest**](SearchInstitutionbyIdRequest.md) |  | [required] |

### Return type

[**crate::models::SearchInstitutionbyIdExample**](SearchInstitutionbyIDExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_institutionby_name

> crate::models::SearchInstitutionbyNameExample search_institutionby_name(search_institutionby_name_request)
Search Institution by Name

The `/institutions/search` endpoint makes it easy to stay up-to-date with supported institutions and help your users quickly find their institutions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_institutionby_name_request** | [**SearchInstitutionbyNameRequest**](SearchInstitutionbyNameRequest.md) |  | [required] |

### Return type

[**crate::models::SearchInstitutionbyNameExample**](SearchInstitutionbyNameExample.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

