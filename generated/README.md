# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.1.0
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AuthControllerApi* | [**signup**](docs/AuthControllerApi.md#signup) | **POST** /auth/signup | 
*UserControllersApi* | [**get_users**](docs/UserControllersApi.md#get_users) | **GET** /users/users | 


## Documentation For Models

 - [AuthResponse](docs/AuthResponse.md)
 - [NewUser](docs/NewUser.md)
 - [Signup200Response](docs/Signup200Response.md)
 - [SignupRequest](docs/SignupRequest.md)
 - [UserResponse](docs/UserResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



