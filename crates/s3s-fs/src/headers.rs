//! HTTP headers storage and retrieval for object metadata

use crate::fs::InternalInfo;
use s3s::dto::ContentDisposition;
use s3s::dto::ContentEncoding;
use s3s::dto::ContentLanguage;
use s3s::dto::ContentType;

/// Save HTTP headers (Content-Encoding, Content-Type, etc.) to internal info
pub fn save_object_headers(
    info: &mut InternalInfo,
    content_encoding: Option<&ContentEncoding>,
    content_type: Option<&ContentType>,
    content_language: Option<&ContentLanguage>,
    content_disposition: Option<&ContentDisposition>,
) {
    if let Some(val) = content_encoding {
        info.insert("content_encoding".to_owned(), serde_json::Value::String(val.clone()));
    }
    if let Some(val) = content_type {
        info.insert("content_type".to_owned(), serde_json::Value::String(val.clone()));
    }
    if let Some(val) = content_language {
        info.insert("content_language".to_owned(), serde_json::Value::String(val.clone()));
    }
    if let Some(val) = content_disposition {
        info.insert("content_disposition".to_owned(), serde_json::Value::String(val.clone()));
    }
}

/// Load Content-Encoding from internal info
pub fn load_content_encoding(info: &InternalInfo) -> Option<ContentEncoding> {
    info.get("content_encoding")
        .and_then(|v| v.as_str())
        .map(|s| ContentEncoding::from(s))
}

/// Load Content-Type from internal info
pub fn load_content_type(info: &InternalInfo) -> Option<ContentType> {
    info.get("content_type")
        .and_then(|v| v.as_str())
        .map(|s| ContentType::from(s))
}

/// Load Content-Language from internal info
pub fn load_content_language(info: &InternalInfo) -> Option<ContentLanguage> {
    info.get("content_language")
        .and_then(|v| v.as_str())
        .map(|s| ContentLanguage::from(s))
}

/// Load Content-Disposition from internal info
pub fn load_content_disposition(info: &InternalInfo) -> Option<ContentDisposition> {
    info.get("content_disposition")
        .and_then(|v| v.as_str())
        .map(|s| ContentDisposition::from(s))
}
