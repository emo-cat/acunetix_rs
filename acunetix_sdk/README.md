# Rust API client for openapi

Awvs12 client api
[http://swagger.io](http://swagger.io) or on
[irc.freenode.net, #swagger](http://swagger.io/irc/).



## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://127.0.0.1:13443/api/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**add_target**](docs/DefaultApi.md#add_target) | **POST** /targets | Add a new target to the scan list
*DefaultApi* | [**delete_scan**](docs/DefaultApi.md#delete_scan) | **DELETE** /scans/{scanid} | delete scan by scanid
*DefaultApi* | [**delete_target**](docs/DefaultApi.md#delete_target) | **DELETE** /targets/{targetid} | get target by id
*DefaultApi* | [**get_info**](docs/DefaultApi.md#get_info) | **GET** /info | get awvs info
*DefaultApi* | [**get_me**](docs/DefaultApi.md#get_me) | **GET** /me | get user info
*DefaultApi* | [**get_scan_detail**](docs/DefaultApi.md#get_scan_detail) | **GET** /scans/{scanid} | get scan status by scanid
*DefaultApi* | [**get_scan_profiles**](docs/DefaultApi.md#get_scan_profiles) | **GET** /scanning_profiles | get scan profile
*DefaultApi* | [**get_scan_reports**](docs/DefaultApi.md#get_scan_reports) | **GET** /reports/{scanid} | get scan reports by scanid
*DefaultApi* | [**get_scan_stat**](docs/DefaultApi.md#get_scan_stat) | **GET** /scans/{scanid}/results/{sessionid}/statistics | get stat by scanid,sessionid
*DefaultApi* | [**get_scans**](docs/DefaultApi.md#get_scans) | **GET** /scans | get scan list
*DefaultApi* | [**get_target**](docs/DefaultApi.md#get_target) | **GET** /targets/{targetid} | get target by id
*DefaultApi* | [**get_target_config**](docs/DefaultApi.md#get_target_config) | **GET** /targets/{targetid}/configuration | get target by id
*DefaultApi* | [**get_targets**](docs/DefaultApi.md#get_targets) | **GET** /targets | get all targets
*DefaultApi* | [**get_vuln**](docs/DefaultApi.md#get_vuln) | **GET** /scans/{scanid}/results/{sessionid}/vulnerabilities | get results by scanid,sessionid
*DefaultApi* | [**login**](docs/DefaultApi.md#login) | **POST** /me/login | login
*DefaultApi* | [**set_target_config**](docs/DefaultApi.md#set_target_config) | **PATCH** /targets/{targetid}/configuration | get target by id
*DefaultApi* | [**start_scan**](docs/DefaultApi.md#start_scan) | **POST** /scans | start scan by scanid
*DefaultApi* | [**stop_scan**](docs/DefaultApi.md#stop_scan) | **POST** /scans/{scanid}/abort | stop scan by scanid


## Documentation For Models

 - [ApiResponse](docs/ApiResponse.md)
 - [Info](docs/Info.md)
 - [License](docs/License.md)
 - [LicenseExtra](docs/LicenseExtra.md)
 - [LicenseLimit](docs/LicenseLimit.md)
 - [LoginReq](docs/LoginReq.md)
 - [Me](docs/Me.md)
 - [Pagination](docs/Pagination.md)
 - [Profile](docs/Profile.md)
 - [Profiles](docs/Profiles.md)
 - [Scan](docs/Scan.md)
 - [ScanApp](docs/ScanApp.md)
 - [ScanAppHost](docs/ScanAppHost.md)
 - [ScanAppHostTargetInfo](docs/ScanAppHostTargetInfo.md)
 - [ScanAppHostWebScanStatus](docs/ScanAppHostWebScanStatus.md)
 - [ScanAppMessage](docs/ScanAppMessage.md)
 - [ScanAppTargetInfo](docs/ScanAppTargetInfo.md)
 - [ScanAppVuln](docs/ScanAppVuln.md)
 - [ScanAppWvs](docs/ScanAppWvs.md)
 - [ScanAppWvsMain](docs/ScanAppWvsMain.md)
 - [ScanAppWvsMainStatus](docs/ScanAppWvsMainStatus.md)
 - [ScanDetail](docs/ScanDetail.md)
 - [ScanDetailCurrentSession](docs/ScanDetailCurrentSession.md)
 - [ScanStat](docs/ScanStat.md)
 - [ScanStatServerityCounts](docs/ScanStatServerityCounts.md)
 - [Scans](docs/Scans.md)
 - [ScansItem](docs/ScansItem.md)
 - [Schedual](docs/Schedual.md)
 - [Target](docs/Target.md)
 - [TargetConfig](docs/TargetConfig.md)
 - [TargetConfigAuthentication](docs/TargetConfigAuthentication.md)
 - [TargetConfigCustomCookies](docs/TargetConfigCustomCookies.md)
 - [TargetConfigLogin](docs/TargetConfigLogin.md)
 - [TargetConfigLoginCredentials](docs/TargetConfigLoginCredentials.md)
 - [TargetConfigProxy](docs/TargetConfigProxy.md)
 - [TargetConfigSshCredentials](docs/TargetConfigSshCredentials.md)
 - [TargetDel](docs/TargetDel.md)
 - [TargetDetail](docs/TargetDetail.md)
 - [TargetDetailAllOf](docs/TargetDetailAllOf.md)
 - [TargetResp](docs/TargetResp.md)
 - [TargetRespAllOf](docs/TargetRespAllOf.md)
 - [TargetsResp](docs/TargetsResp.md)
 - [UpdateInfo](docs/UpdateInfo.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

apiteam@swagger.io
