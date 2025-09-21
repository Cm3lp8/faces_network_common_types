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
