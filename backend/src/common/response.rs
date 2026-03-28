use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    http::HeaderMap as AxumHeaderMap,
    http::HeaderName,
    http::HeaderValue,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Response {
        let body = serde_json::to_string(&ApiResponse {
            code: 0,
            message: "ok".to_string(),
            data: Some(data),
            request_id: None,
        })
        .unwrap();

        (
            StatusCode::OK,
            AxumHeaderMap::from_iter([("content-type".parse::<HeaderName>().unwrap(), HeaderValue::from_static("application/json"))]),
            body,
        ).into_response()
    }

    pub fn success_with_request_id(data: T, request_id: String) -> Response {
        let body = serde_json::to_string(&ApiResponse {
            code: 0,
            message: "ok".to_string(),
            data: Some(data),
            request_id: Some(request_id),
        })
        .unwrap();

        (
            StatusCode::OK,
            AxumHeaderMap::from_iter([("content-type".parse::<HeaderName>().unwrap(), HeaderValue::from_static("application/json"))]),
            body,
        ).into_response()
    }
}

impl ApiResponse<()> {
    pub fn success_no_data() -> Response {
        let body = serde_json::to_string(&ApiResponse::<()> {
            code: 0,
            message: "ok".to_string(),
            data: None,
            request_id: None,
        })
        .unwrap();

        (
            StatusCode::OK,
            AxumHeaderMap::from_iter([("content-type".parse::<HeaderName>().unwrap(), HeaderValue::from_static("application/json"))]),
            body,
        ).into_response()
    }

    pub fn success_no_data_with_request_id(request_id: String) -> Response {
        let body = serde_json::to_string(&ApiResponse::<()> {
            code: 0,
            message: "ok".to_string(),
            data: None,
            request_id: Some(request_id),
        })
        .unwrap();

        (
            StatusCode::OK,
            AxumHeaderMap::from_iter([("content-type".parse::<HeaderName>().unwrap(), HeaderValue::from_static("application/json"))]),
            body,
        ).into_response()
    }

    pub fn error(code: u16, message: String) -> Response {
        let body = serde_json::to_string(&ApiResponse::<()> {
            code,
            message,
            data: None,
            request_id: None,
        })
        .unwrap();

        (
            StatusCode::OK,
            AxumHeaderMap::from_iter([("content-type".parse::<HeaderName>().unwrap(), HeaderValue::from_static("application/json"))]),
            body,
        ).into_response()
    }

    pub fn error_with_request_id(code: u16, message: String, request_id: String) -> Response {
        let body = serde_json::to_string(&ApiResponse::<()> {
            code,
            message,
            data: None,
            request_id: Some(request_id),
        })
        .unwrap();

        (
            StatusCode::OK,
            AxumHeaderMap::from_iter([("content-type".parse::<HeaderName>().unwrap(), HeaderValue::from_static("application/json"))]),
            body,
        ).into_response()
    }
}

/// Add a header to a Response
pub fn with_etag(response: Response, etag: String) -> Response {
    let mut headers = AxumHeaderMap::new();
    headers.insert(
        HeaderName::from_static("etag"),
        HeaderValue::from_str(&etag).expect("HeaderValue"),
    );
    // Merge headers
    response.into_response()
}

/// Add request ID header to response
pub fn with_request_id(response: Response, request_id: String) -> Response {
    let mut resp = response;
    resp.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&request_id).unwrap_or_else(|_| HeaderValue::from_static("unknown")),
    );
    resp
}
