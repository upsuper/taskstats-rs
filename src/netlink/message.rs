use super::raw::{nlmsghdr, NLMSG_LENGTH};
use super::Serialize;
use zerocopy::LayoutVerified;

#[derive(Debug)]
pub struct NetlinkMessage<P> {
    pub ty: u16,
    pub flags: u16,
    pub seq: u32,
    pub pid: u32,
    pub payload: P,
}

impl<P: Serialize> Serialize for NetlinkMessage<P> {
    fn len(&self) -> u32 {
        NLMSG_LENGTH(self.payload.len())
    }

    fn serialize(&self, buf: &mut [u8]) {
        let header_len = NLMSG_LENGTH(0) as usize;
        let (header, payload) = buf.split_at_mut(header_len);
        let mut header = LayoutVerified::<_, nlmsghdr>::new(header).expect("invalid buffer");
        header.nlmsg_len = self.len();
        header.nlmsg_type = self.ty;
        header.nlmsg_flags = self.flags;
        header.nlmsg_seq = self.seq;
        header.nlmsg_pid = self.pid;
        self.payload.serialize(payload);
    }
}

impl<'a> NetlinkMessage<&'a [u8]> {
    pub fn deserialize(buf: &'a [u8]) -> Self {
        let header_len = NLMSG_LENGTH(0) as usize;
        let (header, payload) = buf.split_at(header_len);
        let header = LayoutVerified::<_, nlmsghdr>::new(header).expect("invalid buffer");
        Self {
            ty: header.nlmsg_type,
            flags: header.nlmsg_flags,
            seq: header.nlmsg_seq,
            pid: header.nlmsg_pid,
            payload,
        }
    }
}
