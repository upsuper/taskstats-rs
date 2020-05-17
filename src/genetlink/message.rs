use super::raw::{genlmsghdr, GENL_HDRLEN};
use crate::netlink::Serialize;
use thiserror::Error;
use zerocopy::LayoutVerified;

#[derive(Debug)]
pub struct GenericNetlinkMessage<P> {
    pub cmd: u8,
    pub version: u8,
    pub payload: P,
}

impl<P: Serialize> Serialize for GenericNetlinkMessage<P> {
    fn len(&self) -> u32 {
        GENL_HDRLEN + self.payload.len()
    }

    fn serialize(&self, buf: &mut [u8]) {
        let header_len = GENL_HDRLEN as usize;
        let (header, payload) = buf.split_at_mut(header_len);
        let mut header = LayoutVerified::<_, genlmsghdr>::new(header).expect("invalid buffer");
        header.cmd = self.cmd;
        header.version = self.version;
        header.reserved = 0;
        self.payload.serialize(payload);
    }
}

impl<'a> GenericNetlinkMessage<&'a [u8]> {
    pub fn deserialize(buf: &'a [u8]) -> Result<Self, InvalidBuffer> {
        let header_len = GENL_HDRLEN as usize;
        if buf.len() < header_len {
            return Err(InvalidBuffer::Header(buf.len()));
        }
        let (header, payload) = buf.split_at(header_len);
        let header = LayoutVerified::<_, genlmsghdr>::new(header).expect("invalid buffer");
        Ok(Self {
            cmd: header.cmd,
            version: header.version,
            payload,
        })
    }
}

#[derive(Debug, Error)]
pub enum InvalidBuffer {
    #[error("insufficient buffer for generic netlink header, got buffer size {0}")]
    Header(usize),
}
