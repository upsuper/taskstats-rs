use crate::error::InvalidMessage;
use crate::netlink::{deserialize_attrs, deserialize_u32, NetlinkAttr};
use crate::parse::{check_attr_type, drop_attr, drop_attrs, parse_aggr_pid};
use crate::raw::{
    TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK, TASKSTATS_CMD_ATTR_REGISTER_CPUMASK,
    TASKSTATS_TYPE_AGGR_PID, TASKSTATS_TYPE_AGGR_TGID, TASKSTATS_TYPE_TGID,
};
use crate::taskstats::Taskstats;
use crate::{Error, TaskstatsConnection};
use libc::pid_t;
use log::{debug, warn};
use std::ffi::CStr;

/// Taskstats listener for stats of exited tasks
pub struct TaskstatsListener {
    conn: Option<TaskstatsConnection>,
    cpu_mask: Box<CStr>,
}

impl TaskstatsListener {
    /// Registers the a taskstats listener on the given CPU mask.
    pub fn register(mut conn: TaskstatsConnection, cpu_mask: Box<CStr>) -> Result<Self, Error> {
        do_register(&mut conn, &cpu_mask)?;
        Ok(Self {
            conn: Some(conn),
            cpu_mask,
        })
    }

    /// Deregisters this taskstats listener and returns the internal connection.
    pub fn deregister(mut self) -> Result<TaskstatsConnection, Error> {
        let mut conn = self.conn.take().unwrap();
        do_deregister(&mut conn, &self.cpu_mask)?;
        // TODO: may need to drain packets received.
        Ok(conn)
    }

    /// Returns the next exit taskstats. It blocks if nothing is available.
    pub fn get_next(&mut self) -> Result<(pid_t, Taskstats), Error> {
        let conn = self.conn.as_mut().unwrap();
        let mut attrs = conn.recv()?;
        // Parse aggregated pid stats
        let (ty, value) = attrs.next_or(Error::UnexpectedEmptyResult)?;
        check_attr_type(ty, TASKSTATS_TYPE_AGGR_PID)?;
        let result = parse_aggr_pid(value)?;
        // Drop aggregated tgid stats if any, as its content doesn't have much useful information,
        // and it isn't very reliable either.
        let next_attr = attrs.next().transpose()?;
        if let Some((ty, value)) = next_attr {
            match ty {
                TASKSTATS_TYPE_AGGR_TGID => {
                    let mut attrs = deserialize_attrs(value);
                    let (ty, value) = attrs.next_or(InvalidMessage::EmptyAggrTgid)?;
                    check_attr_type(ty, TASKSTATS_TYPE_TGID)?;
                    let tgid = deserialize_u32(value)?;
                    debug!("dropping aggr tgid stats for {}", tgid);
                }
                _ => drop_attr((ty, value)),
            }
        }
        // Drop the remaining attributes.
        drop_attrs(attrs)?;
        Ok(result)
    }
}

impl Drop for TaskstatsListener {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.as_mut() {
            match do_deregister(conn, &self.cpu_mask) {
                Ok(()) => {}
                Err(e) => warn!("failed to deregister on {:?}: {}", self.cpu_mask, e),
            }
        }
    }
}

fn do_register(conn: &mut TaskstatsConnection, cpu_mask: &CStr) -> Result<(), Error> {
    conn.send(NetlinkAttr {
        ty: TASKSTATS_CMD_ATTR_REGISTER_CPUMASK,
        payload: cpu_mask,
    })
}

fn do_deregister(conn: &mut TaskstatsConnection, cpu_mask: &CStr) -> Result<(), Error> {
    conn.send(NetlinkAttr {
        ty: TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK,
        payload: cpu_mask,
    })
}
