use super::error::InvalidBuffer;
use super::raw::{nlattr, NLA_ALIGN};
use super::Serialize;
use std::mem::size_of;
use zerocopy::LayoutVerified;

#[derive(Debug)]
pub struct NetlinkAttr<P> {
    pub ty: u16,
    pub payload: P,
}

impl<P: Serialize> Serialize for NetlinkAttr<P> {
    fn len(&self) -> u32 {
        let len = size_of::<nlattr>() as u32 + self.payload.len();
        NLA_ALIGN(len as u16) as u32
    }

    fn serialize(&self, buf: &mut [u8]) {
        let attr_len = size_of::<nlattr>();
        let (attr, payload) = buf.split_at_mut(attr_len);
        let mut attr = LayoutVerified::<_, nlattr>::new(attr).expect("invalid buffer");
        let payload_len = self.payload.len() as usize;
        attr.nla_len = attr_len as u16 + payload_len as u16;
        attr.nla_type = self.ty;
        self.payload.serialize(&mut payload[..payload_len]);
    }
}

pub fn deserialize_attrs(buf: &[u8]) -> AttrsIter<'_> {
    AttrsIter { buf }
}

pub struct AttrsIter<'a> {
    buf: &'a [u8],
}

impl<'a> Iterator for AttrsIter<'a> {
    type Item = Result<(u16, &'a [u8]), InvalidBuffer>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            return None;
        }
        let attr_len = size_of::<nlattr>();
        if self.buf.len() < attr_len {
            return Some(Err(InvalidBuffer::AttrHeader(self.buf.len())));
        }
        let (attr, payload) = self.buf.split_at(attr_len);
        let attr = LayoutVerified::<_, nlattr>::new(attr).unwrap();
        let aligned_attr_len = NLA_ALIGN(attr.nla_len) as usize;
        if self.buf.len() < aligned_attr_len {
            return Some(Err(InvalidBuffer::AttrPayload(
                aligned_attr_len,
                self.buf.len(),
            )));
        }
        let payload_len = attr.nla_len as usize - attr_len;
        self.buf = &self.buf[aligned_attr_len..];
        Some(Ok((attr.nla_type, &payload[..payload_len])))
    }
}

impl<'a> AttrsIter<'a> {
    pub fn next_or<E>(&mut self, err: E) -> Result<(u16, &'a [u8]), E>
    where
        E: From<InvalidBuffer>,
    {
        self.next().transpose()?.ok_or(err)
    }
}
