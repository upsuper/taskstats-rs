use crate::error::Error;
use crate::genetlink::{resolve_family_id, GenericNetlinkMessage};
use crate::netlink::raw::NLM_F_REQUEST;
use crate::netlink::{deserialize_attrs, AttrsIter, NetlinkAttr, NetlinkMessage, Serialize};
use crate::parse::{check_attr_type, drop_attrs, parse_aggr_pid};
use crate::raw::{
    TASKSTATS_CMD_ATTR_PID, TASKSTATS_CMD_GET, TASKSTATS_CMD_NEW, TASKSTATS_GENL_NAME,
    TASKSTATS_GENL_VERSION, TASKSTATS_TYPE_AGGR_PID,
};
use crate::taskstats::Taskstats;
use crate::InvalidMessage;
use libc::pid_t;
use log::trace;
use netlink_sys::{Protocol, Socket};
use once_cell::sync::OnceCell;
use std::ffi::CStr;

const BUFFER_SIZE: usize = 4096;

#[repr(align(4))]
struct Buffer([u8; BUFFER_SIZE]);

/// Taskstats connection
pub struct TaskstatsConnection {
    pub(crate) socket: Socket,
    pub(crate) pid: u32,
    seq: u32,
    buffer: Box<Buffer>,
}

impl TaskstatsConnection {
    /// Creates a new taskstats connection.
    pub fn new() -> Result<Self, Error> {
        let mut socket = Socket::new(Protocol::Generic)?;
        let addr = socket.bind_auto()?;

        Ok(Self {
            socket,
            pid: addr.port_number(),
            seq: 0,
            buffer: Box::new(Buffer([0; BUFFER_SIZE])),
        })
    }

    fn next_seq(&mut self) -> u32 {
        self.seq += 1;
        self.seq
    }

    pub(crate) fn send<T>(&mut self, attr: NetlinkAttr<T>) -> Result<(), Error>
    where
        T: Serialize,
    {
        let msg = NetlinkMessage {
            ty: get_family_id(&self.socket)?,
            flags: NLM_F_REQUEST,
            seq: self.next_seq(),
            pid: self.pid,
            payload: GenericNetlinkMessage {
                cmd: TASKSTATS_CMD_GET,
                version: TASKSTATS_GENL_VERSION,
                payload: &[attr] as &[_],
            },
        };
        let msg_len = msg.len() as usize;
        let buf = &mut self.buffer.as_mut().0[..msg_len];
        msg.serialize(buf);
        self.socket.send(buf, 0)?;
        Ok(())
    }

    pub(crate) fn recv(&mut self) -> Result<AttrsIter<'_>, Error> {
        let buffer = &mut self.buffer.as_mut().0;
        let len = self.socket.recv(buffer, 0)?;
        trace!("recv package length = {}", len);
        let nl_msg = NetlinkMessage::deserialize(&buffer[..len]);
        let genl_msg = GenericNetlinkMessage::deserialize(nl_msg.payload)?;
        match genl_msg.cmd {
            TASKSTATS_CMD_NEW => Ok(deserialize_attrs(genl_msg.payload)),
            cmd => Err(Error::UnexpectedCmd(cmd)),
        }
    }

    /// Returns current aggregated taskstats of the given pid.
    pub fn get_pid_stats(&mut self, pid: pid_t) -> Result<Taskstats, Error> {
        // Send request
        self.send(NetlinkAttr {
            ty: TASKSTATS_CMD_ATTR_PID,
            payload: pid as u32,
        })?;

        // Receive response
        let mut reply = self.recv()?;
        let (ty, value) = reply
            .next()
            .ok_or(Error::NoResult)?
            .map_err(InvalidMessage::Netlink)?;
        check_attr_type(ty, TASKSTATS_TYPE_AGGR_PID)?;
        let (this_pid, result) = parse_aggr_pid(value)?;
        drop_attrs(reply)?;
        if this_pid == pid {
            Ok(result)
        } else {
            Err(Error::UnexpectedPid(pid, this_pid))
        }
    }
}

static FAMILY_ID: OnceCell<u16> = OnceCell::new();

fn get_family_id(socket: &Socket) -> Result<u16, Error> {
    FAMILY_ID
        .get_or_try_init(|| {
            let family_name = CStr::from_bytes_with_nul(TASKSTATS_GENL_NAME).unwrap();
            let family_id = resolve_family_id(socket, family_name)?;
            family_id.ok_or(Error::Unavailable)
        })
        .map(|id| *id)
}
