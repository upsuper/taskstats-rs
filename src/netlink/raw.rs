#![allow(dead_code, non_camel_case_types, non_snake_case)]

use std::mem::size_of;
use std::os::raw::{c_int, c_uint, c_ushort};
use zerocopy::{AsBytes, FromBytes};

type _netlink_type = u32;
pub const NETLINK_ROUTE: _netlink_type = 0;
pub const NETLINK_UNUSED: _netlink_type = 1;
pub const NETLINK_USERSOCK: _netlink_type = 2;
pub const NETLINK_FIREWALL: _netlink_type = 3;
pub const NETLINK_SOCK_DIAG: _netlink_type = 4;
pub const NETLINK_NFLOG: _netlink_type = 5;
pub const NETLINK_XFRM: _netlink_type = 6;
pub const NETLINK_SELINUX: _netlink_type = 7;
pub const NETLINK_ISCSI: _netlink_type = 8;
pub const NETLINK_AUDIT: _netlink_type = 9;
pub const NETLINK_FIB_LOOKUP: _netlink_type = 10;
pub const NETLINK_CONNECTOR: _netlink_type = 11;
pub const NETLINK_NETFILTER: _netlink_type = 12;
pub const NETLINK_IP6_FW: _netlink_type = 13;
pub const NETLINK_DNRTMSG: _netlink_type = 14;
pub const NETLINK_KOBJECT_UEVENT: _netlink_type = 15;
pub const NETLINK_GENERIC: _netlink_type = 16;
pub const NETLINK_SCSITRANSPORT: _netlink_type = 18;
pub const NETLINK_ECRYPTFS: _netlink_type = 19;
pub const NETLINK_RDMA: _netlink_type = 20;
pub const NETLINK_CRYPTO: _netlink_type = 21;
pub const NETLINK_SMC: _netlink_type = 22;

pub const NETLINK_INET_DIAG: _netlink_type = NETLINK_SOCK_DIAG;

pub const MAX_LINKS: usize = 32;

type __kernel_sa_family_t = c_ushort;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_nl {
    pub nl_family: __kernel_sa_family_t,
    pub nl_pad: c_ushort,
    pub nl_pid: u32,
    pub nl_groups: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, AsBytes, FromBytes)]
pub struct nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}

type _nlm_f_type = u16;
pub const NLM_F_REQUEST: _nlm_f_type = 1;
pub const NLM_F_MULTI: _nlm_f_type = 2;
pub const NLM_F_ACK: _nlm_f_type = 4;
pub const NLM_F_ECHO: _nlm_f_type = 8;
pub const NLM_F_DUMP_INTR: _nlm_f_type = 16;
pub const NLM_F_DUMP_FILTERED: _nlm_f_type = 32;
pub const NLM_F_ROOT: _nlm_f_type = 256;
pub const NLM_F_MATCH: _nlm_f_type = 512;
pub const NLM_F_ATOMIC: _nlm_f_type = 1024;
pub const NLM_F_DUMP: _nlm_f_type = 768;
pub const NLM_F_REPLACE: _nlm_f_type = 256;
pub const NLM_F_EXCL: _nlm_f_type = 512;
pub const NLM_F_CREATE: _nlm_f_type = 1024;
pub const NLM_F_APPEND: _nlm_f_type = 2048;
pub const NLM_F_NONREC: _nlm_f_type = 256;
pub const NLM_F_CAPPED: _nlm_f_type = 256;
pub const NLM_F_ACK_TLVS: _nlm_f_type = 512;

pub const NLMSG_ALIGNTO: u32 = 4;
#[inline]
pub const fn NLMSG_ALIGN(len: u32) -> u32 {
    (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1)
}
pub const NLMSG_HDRLEN: u32 = size_of::<nlmsghdr>() as u32;
#[inline]
pub const fn NLMSG_LENGTH(len: u32) -> u32 {
    len + NLMSG_HDRLEN
}
#[inline]
pub const fn NLMSG_SPACE(len: u32) -> u32 {
    NLMSG_ALIGN(NLMSG_LENGTH(len))
}
#[inline]
pub fn NLMSG_DATA(data: &[u8]) -> &[u8] {
    &data[NLMSG_LENGTH(0) as usize..]
}

type _nlmsg_type = u32;
pub const NLMSG_NOOP: _nlmsg_type = 1;
pub const NLMSG_ERROR: _nlmsg_type = 2;
pub const NLMSG_DONE: _nlmsg_type = 3;
pub const NLMSG_OVERRUN: _nlmsg_type = 4;
pub const NLMSG_MIN_TYPE: _nlmsg_type = 16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nlmsgerr {
    pub error: c_int,
    pub msg: nlmsghdr,
}

pub type nlmsgerr_attrs = u32;
pub const NLMSGERR_ATTR_UNUSED: nlmsgerr_attrs = 0;
pub const NLMSGERR_ATTR_MSG: nlmsgerr_attrs = 1;
pub const NLMSGERR_ATTR_OFFS: nlmsgerr_attrs = 2;
pub const NLMSGERR_ATTR_COOKIE: nlmsgerr_attrs = 3;
pub const __NLMSGERR_ATTR_MAX: nlmsgerr_attrs = 4;
pub const NLMSGERR_ATTR_MAX: nlmsgerr_attrs = 3;

type _netlink2_type = u32;
pub const NETLINK_ADD_MEMBERSHIP: _netlink2_type = 1;
pub const NETLINK_DROP_MEMBERSHIP: _netlink2_type = 2;
pub const NETLINK_PKTINFO: _netlink2_type = 3;
pub const NETLINK_BROADCAST_ERROR: _netlink2_type = 4;
pub const NETLINK_NO_ENOBUFS: _netlink2_type = 5;
pub const NETLINK_RX_RING: _netlink2_type = 6;
pub const NETLINK_TX_RING: _netlink2_type = 7;
pub const NETLINK_LISTEN_ALL_NSID: _netlink2_type = 8;
pub const NETLINK_LIST_MEMBERSHIPS: _netlink2_type = 9;
pub const NETLINK_CAP_ACK: _netlink2_type = 10;
pub const NETLINK_EXT_ACK: _netlink2_type = 11;
pub const NETLINK_GET_STRICT_CHK: _netlink2_type = 12;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nl_pktinfo {
    pub group: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nl_mmap_req {
    pub nm_block_size: c_uint,
    pub nm_block_nr: c_uint,
    pub nm_frame_size: c_uint,
    pub nm_frame_nr: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nl_mmap_hdr {
    pub nm_status: c_uint,
    pub nm_len: c_uint,
    pub nm_group: u32,
    pub nm_pid: u32,
    pub nm_uid: u32,
    pub nm_gid: u32,
}

pub type nl_mmap_status = u32;
pub const NL_MMAP_STATUS_UNUSED: nl_mmap_status = 0;
pub const NL_MMAP_STATUS_RESERVED: nl_mmap_status = 1;
pub const NL_MMAP_STATUS_VALID: nl_mmap_status = 2;
pub const NL_MMAP_STATUS_COPY: nl_mmap_status = 3;
pub const NL_MMAP_STATUS_SKIP: nl_mmap_status = 4;

type _netlink_connected = u32;
pub const NETLINK_UNCONNECTED: _netlink_connected = 0;
pub const NETLINK_CONNECTED: _netlink_connected = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, AsBytes, FromBytes)]
pub struct nlattr {
    pub nla_len: u16,
    pub nla_type: u16,
}

pub const NLA_F_NESTED: u16 = 1 << 15;
pub const NLA_F_NET_BYTEORDER: u16 = 1 << 14;
pub const NLA_TYPE_MASK: u16 = !(NLA_F_NESTED | NLA_F_NET_BYTEORDER);

pub const NLA_HDRLEN: u16 = NLA_ALIGN(size_of::<nlattr>() as u16);
pub const NLA_ALIGNTO: u16 = 4;
pub const fn NLA_ALIGN(len: u16) -> u16 {
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nla_bitfield32 {
    pub value: u32,
    pub selector: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};
    use std::ptr::null;

    #[test]
    fn bindgen_test_layout_sockaddr_nl() {
        assert_eq!(size_of::<sockaddr_nl>(), 12usize, "Size of: sockaddr_nl");
        assert_eq!(
            align_of::<sockaddr_nl>(),
            4usize,
            "Alignment of sockaddr_nl"
        );
        assert_eq!(
            unsafe { &(*(null::<sockaddr_nl>())).nl_family as *const _ as usize },
            0usize,
            "Offset of field: sockaddr_nl::nl_family"
        );
        assert_eq!(
            unsafe { &(*(null::<sockaddr_nl>())).nl_pad as *const _ as usize },
            2usize,
            "Offset of field: sockaddr_nl::nl_pad"
        );
        assert_eq!(
            unsafe { &(*(null::<sockaddr_nl>())).nl_pid as *const _ as usize },
            4usize,
            "Offset of field: sockaddr_nl::nl_pid"
        );
        assert_eq!(
            unsafe { &(*(null::<sockaddr_nl>())).nl_groups as *const _ as usize },
            8usize,
            "Offset of field: sockaddr_nl::nl_groups"
        );
    }

    #[test]
    fn bindgen_test_layout_nlmsghdr() {
        assert_eq!(size_of::<nlmsghdr>(), 16usize, "Size of: nlmsghdr");
        assert_eq!(align_of::<nlmsghdr>(), 4usize, "Alignment of nlmsghdr");
        assert_eq!(
            unsafe { &(*(null::<nlmsghdr>())).nlmsg_len as *const _ as usize },
            0usize,
            "Offset of field: nlmsghdr::nlmsg_len"
        );
        assert_eq!(
            unsafe { &(*(null::<nlmsghdr>())).nlmsg_type as *const _ as usize },
            4usize,
            "Offset of field: nlmsghdr::nlmsg_type"
        );
        assert_eq!(
            unsafe { &(*(null::<nlmsghdr>())).nlmsg_flags as *const _ as usize },
            6usize,
            "Offset of field: nlmsghdr::nlmsg_flags"
        );
        assert_eq!(
            unsafe { &(*(null::<nlmsghdr>())).nlmsg_seq as *const _ as usize },
            8usize,
            "Offset of field: nlmsghdr::nlmsg_seq"
        );
        assert_eq!(
            unsafe { &(*(null::<nlmsghdr>())).nlmsg_pid as *const _ as usize },
            12usize,
            "Offset of field: nlmsghdr::nlmsg_pid"
        );
    }

    #[test]
    fn bindgen_test_layout_nlmsgerr() {
        assert_eq!(size_of::<nlmsgerr>(), 20usize, "Size of: nlmsgerr");
        assert_eq!(align_of::<nlmsgerr>(), 4usize, "Alignment of nlmsgerr");
        assert_eq!(
            unsafe { &(*(null::<nlmsgerr>())).error as *const _ as usize },
            0usize,
            "Offset of field: nlmsgerr::error"
        );
        assert_eq!(
            unsafe { &(*(null::<nlmsgerr>())).msg as *const _ as usize },
            4usize,
            "Offset of field: nlmsgerr::msg"
        );
    }

    #[test]
    fn bindgen_test_layout_nl_pktinfo() {
        assert_eq!(size_of::<nl_pktinfo>(), 4usize, "Size of: nl_pktinfo");
        assert_eq!(align_of::<nl_pktinfo>(), 4usize, "Alignment of nl_pktinfo");
        assert_eq!(
            unsafe { &(*(null::<nl_pktinfo>())).group as *const _ as usize },
            0usize,
            "Offset of field: nl_pktinfo::group"
        );
    }

    #[test]
    fn bindgen_test_layout_nl_mmap_req() {
        assert_eq!(size_of::<nl_mmap_req>(), 16usize, "Size of: nl_mmap_req");
        assert_eq!(
            align_of::<nl_mmap_req>(),
            4usize,
            "Alignment of nl_mmap_req"
        );
        assert_eq!(
            unsafe { &(*(null::<nl_mmap_req>())).nm_block_size as *const _ as usize },
            0usize,
            "Offset of field: nl_mmap_req::nm_block_size"
        );
        assert_eq!(
            unsafe { &(*(null::<nl_mmap_req>())).nm_block_nr as *const _ as usize },
            4usize,
            "Offset of field: nl_mmap_req::nm_block_nr"
        );
        assert_eq!(
            unsafe { &(*(null::<nl_mmap_req>())).nm_frame_size as *const _ as usize },
            8usize,
            "Offset of field: nl_mmap_req::nm_frame_size"
        );
        assert_eq!(
            unsafe { &(*(null::<nl_mmap_req>())).nm_frame_nr as *const _ as usize },
            12usize,
            "Offset of field: nl_mmap_req::nm_frame_nr"
        );
    }

    #[test]
    fn bindgen_test_layout_nl_mmap_hdr() {
        assert_eq!(size_of::<nl_mmap_hdr>(), 24usize, "Size of: nl_mmap_hdr");
        assert_eq!(
            align_of::<nl_mmap_hdr>(),
            4usize,
            "Alignment of nl_mmap_hdr"
        );
        assert_eq!(
            unsafe { &(*(null::<nl_mmap_hdr>())).nm_status as *const _ as usize },
            0usize,
            "Offset of field: nl_mmap_hdr::nm_status"
        );
        assert_eq!(
            unsafe { &(*(null::<nl_mmap_hdr>())).nm_len as *const _ as usize },
            4usize,
            "Offset of field: nl_mmap_hdr::nm_len"
        );
        assert_eq!(
            unsafe { &(*(null::<nl_mmap_hdr>())).nm_group as *const _ as usize },
            8usize,
            "Offset of field: nl_mmap_hdr::nm_group"
        );
        assert_eq!(
            unsafe { &(*(null::<nl_mmap_hdr>())).nm_pid as *const _ as usize },
            12usize,
            "Offset of field: nl_mmap_hdr::nm_pid"
        );
        assert_eq!(
            unsafe { &(*(null::<nl_mmap_hdr>())).nm_uid as *const _ as usize },
            16usize,
            "Offset of field: nl_mmap_hdr::nm_uid"
        );
        assert_eq!(
            unsafe { &(*(null::<nl_mmap_hdr>())).nm_gid as *const _ as usize },
            20usize,
            "Offset of field: nl_mmap_hdr::nm_gid"
        );
    }

    #[test]
    fn bindgen_test_layout_nlattr() {
        assert_eq!(size_of::<nlattr>(), 4usize, "Size of: nlattr");
        assert_eq!(align_of::<nlattr>(), 2usize, "Alignment of nlattr");
        assert_eq!(
            unsafe { &(*(null::<nlattr>())).nla_len as *const _ as usize },
            0usize,
            "Offset of field: nlattr::nla_len"
        );
        assert_eq!(
            unsafe { &(*(null::<nlattr>())).nla_type as *const _ as usize },
            2usize,
            "Offset of field: nlattr::nla_type"
        );
    }

    #[test]
    fn bindgen_test_layout_nla_bitfield32() {
        assert_eq!(
            size_of::<nla_bitfield32>(),
            8usize,
            "Size of: nla_bitfield32"
        );
        assert_eq!(
            align_of::<nla_bitfield32>(),
            4usize,
            "Alignment of nla_bitfield32"
        );
        assert_eq!(
            unsafe { &(*(null::<nla_bitfield32>())).value as *const _ as usize },
            0usize,
            "Offset of field: nla_bitfield32::value"
        );
        assert_eq!(
            unsafe { &(*(null::<nla_bitfield32>())).selector as *const _ as usize },
            4usize,
            "Offset of field: nla_bitfield32::selector"
        );
    }
}
