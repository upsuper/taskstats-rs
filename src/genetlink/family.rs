use super::raw::{CTRL_ATTR_FAMILY_ID, CTRL_ATTR_FAMILY_NAME, CTRL_CMD_GETFAMILY, GENL_ID_CTRL};
use super::GenericNetlinkMessage;
use crate::netlink::raw::NLM_F_REQUEST;
use crate::netlink::{deserialize_attrs, deserialize_u16, NetlinkAttr, NetlinkMessage, Serialize};
use netlink_sys::{Socket, SocketAddr};
use std::ffi::CStr;
use std::io;
use thiserror::Error;

const BUFFER_SIZE: usize = 512;

pub fn resolve_family_id(socket: &Socket, name: &CStr) -> Result<Option<u16>, FamilyError> {
    let mut addr = SocketAddr::new(0, 0);
    socket.get_address(&mut addr)?;

    let msg = NetlinkMessage {
        ty: GENL_ID_CTRL,
        flags: NLM_F_REQUEST,
        seq: 1,
        pid: addr.port_number(),
        payload: GenericNetlinkMessage {
            cmd: CTRL_CMD_GETFAMILY,
            version: 2,
            payload: &[NetlinkAttr {
                ty: CTRL_ATTR_FAMILY_NAME,
                payload: name,
            }] as &[_],
        },
    };
    let mut buffer = [0; BUFFER_SIZE];
    let msg_len = msg.len() as usize;
    msg.serialize(&mut buffer[..msg_len]);
    socket.send(&buffer[..msg_len], 0)?;

    let reply_len = socket.recv(&mut buffer, 0)?;
    let nl_msg = NetlinkMessage::deserialize(&buffer[..reply_len]);
    let genl_msg = GenericNetlinkMessage::deserialize(nl_msg.payload)?;
    for attr in deserialize_attrs(genl_msg.payload) {
        let (ty, value) = attr?;
        if ty == CTRL_ATTR_FAMILY_ID {
            return Ok(Some(deserialize_u16(value)?));
        }
    }
    Ok(None)
}

#[derive(Debug, Error)]
pub enum FamilyError {
    #[error("io")]
    Io(#[from] io::Error),
    #[error("invalid netlink message")]
    Netlink(#[from] crate::netlink::InvalidBuffer),
    #[error("invalid generic netlink message")]
    GenericNetlink(#[from] super::message::InvalidBuffer),
}
