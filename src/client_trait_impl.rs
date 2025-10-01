use crate::signup::SigninResponseData;
#[cfg(feature = "client-side")]
use crate::signup::SignupResponseData;

#[cfg(feature = "client-side")]
use super::SigninData;
use super::SignupData;
#[cfg(feature = "client-side")]
use faces_quic_client::{ContentType, IntoBodyReq};
#[cfg(feature = "client-side")]
impl IntoBodyReq for SignupData {
    fn into_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
    fn content_type(&self) -> ContentType {
        ContentType::Json
    }
}
#[cfg(feature = "client-side")]
impl IntoBodyReq for SignupResponseData {
    fn into_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
    fn content_type(&self) -> ContentType {
        ContentType::Json
    }
}
#[cfg(feature = "client-side")]
impl IntoBodyReq for SigninData {
    fn into_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
    fn content_type(&self) -> ContentType {
        ContentType::Json
    }
}
#[cfg(feature = "client-side")]
impl IntoBodyReq for SigninResponseData {
    fn into_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
    fn content_type(&self) -> ContentType {
        ContentType::Json
    }
}
