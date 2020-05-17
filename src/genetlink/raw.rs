#![allow(dead_code, non_camel_case_types)]

use crate::netlink::raw::NLMSG_ALIGN;
use std::mem::size_of;
use zerocopy::{AsBytes, FromBytes};

pub const GENL_NAMSIZ: u32 = 16;

pub const GENL_MIN_ID: u32 = 16;
pub const GENL_MAX_ID: u32 = 1023;

#[repr(C)]
#[derive(Debug, Copy, Clone, AsBytes, FromBytes)]
pub struct genlmsghdr {
    pub cmd: u8,
    pub version: u8,
    pub reserved: u16,
}

pub const GENL_HDRLEN: u32 = NLMSG_ALIGN(size_of::<genlmsghdr>() as u32);

pub const GENL_ADMIN_PERM: u32 = 1;
pub const GENL_CMD_CAP_DO: u32 = 2;
pub const GENL_CMD_CAP_DUMP: u32 = 4;
pub const GENL_CMD_CAP_HASPOL: u32 = 8;
pub const GENL_UNS_ADMIN_PERM: u32 = 16;

pub const GENL_ID_CTRL: u16 = 16;
pub const GENL_ID_VFS_DQUOT: u16 = 17;
pub const GENL_ID_PMCRAID: u16 = 18;
pub const GENL_START_ALLOC: u16 = 19;

type _bindgen_ty_2 = u8;
pub const CTRL_CMD_UNSPEC: _bindgen_ty_2 = 0;
pub const CTRL_CMD_NEWFAMILY: _bindgen_ty_2 = 1;
pub const CTRL_CMD_DELFAMILY: _bindgen_ty_2 = 2;
pub const CTRL_CMD_GETFAMILY: _bindgen_ty_2 = 3;
pub const CTRL_CMD_NEWOPS: _bindgen_ty_2 = 4;
pub const CTRL_CMD_DELOPS: _bindgen_ty_2 = 5;
pub const CTRL_CMD_GETOPS: _bindgen_ty_2 = 6;
pub const CTRL_CMD_NEWMCAST_GRP: _bindgen_ty_2 = 7;
pub const CTRL_CMD_DELMCAST_GRP: _bindgen_ty_2 = 8;
pub const CTRL_CMD_GETMCAST_GRP: _bindgen_ty_2 = 9;
pub const __CTRL_CMD_MAX: _bindgen_ty_2 = 10;

type _bindgen_ty_3 = u16;
pub const CTRL_ATTR_UNSPEC: _bindgen_ty_3 = 0;
pub const CTRL_ATTR_FAMILY_ID: _bindgen_ty_3 = 1;
pub const CTRL_ATTR_FAMILY_NAME: _bindgen_ty_3 = 2;
pub const CTRL_ATTR_VERSION: _bindgen_ty_3 = 3;
pub const CTRL_ATTR_HDRSIZE: _bindgen_ty_3 = 4;
pub const CTRL_ATTR_MAXATTR: _bindgen_ty_3 = 5;
pub const CTRL_ATTR_OPS: _bindgen_ty_3 = 6;
pub const CTRL_ATTR_MCAST_GROUPS: _bindgen_ty_3 = 7;
pub const __CTRL_ATTR_MAX: _bindgen_ty_3 = 8;

type _bindgen_ty_4 = u16;
pub const CTRL_ATTR_OP_UNSPEC: _bindgen_ty_4 = 0;
pub const CTRL_ATTR_OP_ID: _bindgen_ty_4 = 1;
pub const CTRL_ATTR_OP_FLAGS: _bindgen_ty_4 = 2;
pub const __CTRL_ATTR_OP_MAX: _bindgen_ty_4 = 3;

type _bindgen_ty_5 = u32;
pub const CTRL_ATTR_MCAST_GRP_UNSPEC: _bindgen_ty_5 = 0;
pub const CTRL_ATTR_MCAST_GRP_NAME: _bindgen_ty_5 = 1;
pub const CTRL_ATTR_MCAST_GRP_ID: _bindgen_ty_5 = 2;
pub const __CTRL_ATTR_MCAST_GRP_MAX: _bindgen_ty_5 = 3;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};
    use std::ptr::null;

    #[test]
    fn bindgen_test_layout_genlmsghdr() {
        assert_eq!(size_of::<genlmsghdr>(), 4usize, "Size of: genlmsghdr");
        assert_eq!(align_of::<genlmsghdr>(), 2usize, "Alignment of genlmsghdr");
        assert_eq!(
            unsafe { &(*(null::<genlmsghdr>())).cmd as *const _ as usize },
            0usize,
            "Offset of field: genlmsghdr::cmd",
        );
        assert_eq!(
            unsafe { &(*(null::<genlmsghdr>())).version as *const _ as usize },
            1usize,
            "Offset of field: genlmsghdr::version",
        );
        assert_eq!(
            unsafe { &(*(null::<genlmsghdr>())).reserved as *const _ as usize },
            2usize,
            "Offset of field: genlmsghdr::reserved",
        );
    }
}
