use super::error::InvalidBuffer;
use std::convert::TryInto;

pub fn deserialize_u16(buf: &[u8]) -> Result<u16, InvalidBuffer> {
    buf.try_into()
        .map(u16::from_ne_bytes)
        .map_err(|_| InvalidBuffer::U16(buf.len()))
}

pub fn deserialize_u32(buf: &[u8]) -> Result<u32, InvalidBuffer> {
    buf.try_into()
        .map(u32::from_ne_bytes)
        .map_err(|_| InvalidBuffer::U32(buf.len()))
}
