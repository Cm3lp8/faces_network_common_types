use bytes::Bytes;

const HEADER_LEN: usize = 16;
const MAGIC_START_LEN: usize = 8;
const VALUE_LEN: usize = 4;
const MAGIC_START: [u8; MAGIC_START_LEN] = *b"FACEFACE";

type Width = u32;
type Height = u32;

pub trait VerifyEncodedAnimationData {
    fn verify_header_at_offset(&self, offset: usize) -> Result<(), EncDecResErr>;
    fn compare_payload_len(&self, given_bytes_len: usize) -> bool;
}

impl VerifyEncodedAnimationData for &[u8] {
    fn compare_payload_len(&self, given_bytes_len: usize) -> bool {
        true
    }
    /// Give an offset if blob is wrapped in a serialized struct
    /// data is encoded in a vec : 3 first bytes describe animation payload len
    fn verify_header_at_offset(&self, offset: usize) -> Result<(), EncDecResErr> {
        if self.len() <= HEADER_LEN + offset {
            return Err(EncDecResErr::BlobTooShort(self.len()));
        }
        let hdr_start = offset;

        let hdr = &self[hdr_start..hdr_start + HEADER_LEN];

        match &hdr[..MAGIC_START_LEN] {
            b"FACEFACE" => {}
            _ => {
                return Err(EncDecResErr::WrongPrefix("Should be FACEFACE".to_string()));
            }
        }

        Ok(())
    }
}
pub struct EncodeConfig;

impl EncodeConfig {
    pub fn create_encoder_destination_buffer(frame_width: u32, frame_height: u32) -> Vec<u8> {
        let mut output = MAGIC_START.to_vec();

        output.extend_from_slice(&frame_width.to_le_bytes());
        output.extend_from_slice(&frame_height.to_le_bytes());
        output
    }
}

pub struct DecodeConfig;

impl DecodeConfig {
    pub fn split_header_and_data_into_bytes_buffer(bytes: Vec<u8>) -> (Header, Payload) {
        let mut header = bytes::Bytes::from_owner(bytes);

        let data = header.split_off(HEADER_LEN);

        (Header(header), Payload(data))
    }
}

pub struct Payload(pub Bytes);
pub struct Header(Bytes);

impl Header {
    pub fn get_frame_size(&self) -> Result<(Width, Height), EncDecResErr> {
        if self.0.len() >= HEADER_LEN && &self.0[..MAGIC_START_LEN] == &MAGIC_START {
            let width_bytes: [u8; VALUE_LEN] = self
                .0
                .get(MAGIC_START_LEN..MAGIC_START_LEN + VALUE_LEN)
                .ok_or(EncDecResErr::FailedToReadFrameDimensionsInStream(
                    "Failed to read  animation payload width ".to_string(),
                ))?
                .try_into()
                .map_err(|e| {
                    EncDecResErr::FailedToReadFrameDimensionsInStream(
                        "Failed to read animation payload width len".to_string(),
                    )
                })?;
            let height_bytes = self
                .0
                .get(MAGIC_START_LEN + VALUE_LEN..MAGIC_START_LEN + VALUE_LEN + VALUE_LEN)
                .ok_or(EncDecResErr::FailedToReadFrameDimensionsInStream(
                    "Failed to read payload bytes len".to_string(),
                ))?
                .try_into()
                .map_err(|e| {
                    EncDecResErr::FailedToReadFrameDimensionsInStream(
                        "Failed to read animation payload height len".to_string(),
                    )
                })?;
            return Ok((
                u32::from_le_bytes(width_bytes),
                u32::from_le_bytes(height_bytes),
            ));
        } else {
            Err(EncDecResErr::FailedToReadFrameDimensionsInStream(
                "Failed to read animation payload".to_string(),
            ))
        }
    }
}

pub enum EncDecResErr {
    FailedToReadFrameDimensionsInStream(String),
    BlobTooShort(usize),
    WrongPrefix(String),
}

impl std::fmt::Display for EncDecResErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToReadFrameDimensionsInStream(error) => {
                write!(
                    f,
                    "EncDecResErr FailedToReadFrameDimensionsInStream [{:?}]",
                    error
                )
            }
            Self::WrongPrefix(error) => {
                write!(f, "EncDecResErr WrongPrefix : [{:?}]", error)
            }
            Self::BlobTooShort(blob_len) => {
                write!(f, "EncDecResErr BlobTooShort [{:?}] bytes", blob_len)
            }
        }
    }
}
