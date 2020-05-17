use crate::error::InvalidMessage;
use crate::netlink::{deserialize_attrs, deserialize_u32, AttrsIter};
use crate::raw::{taskstats_v9, TASKSTATS_TYPE_PID, TASKSTATS_TYPE_STATS};
use crate::taskstats::Taskstats;
use crate::Error;
use libc::pid_t;
use log::{debug, trace, warn};
use zerocopy::LayoutVerified;

pub(crate) fn parse_aggr_pid(value: &[u8]) -> Result<(pid_t, Taskstats), Error> {
    let mut attrs = deserialize_attrs(value);
    // Parse pid
    let (ty, value) = attrs
        .next()
        .ok_or(InvalidMessage::EmptyAggrPid)?
        .map_err(InvalidMessage::Netlink)?;
    check_attr_type(ty, TASKSTATS_TYPE_PID)?;
    let pid = deserialize_u32(value)?;
    trace!("parsing stats for pid {}", pid);
    // Parse stats
    let (ty, value) = attrs.next_or(InvalidMessage::NoStats)?;
    check_attr_type(ty, TASKSTATS_TYPE_STATS)?;
    let taskstats = parse_taskstats(value)?;
    // Dump remaining attrs if any
    drop_attrs(attrs)?;
    Ok((pid as pid_t, taskstats))
}

fn parse_taskstats(value: &[u8]) -> Result<Taskstats, Error> {
    let mut version: [u8; 2] = [0; 2];
    version.copy_from_slice(&value[..2]);
    let version = u16::from_ne_bytes(version);
    if version < 9 {
        warn!("unrecognized taskstats version: {:?}", value);
        Err(Error::UnsupportedVersion(version))
    } else {
        let stats = LayoutVerified::<_, taskstats_v9>::new(value).unwrap();
        Ok(Taskstats::from(&*stats))
    }
}

pub(crate) fn check_attr_type(ty: u16, expected: u16) -> Result<(), InvalidMessage> {
    if ty == expected {
        Ok(())
    } else {
        Err(InvalidMessage::Attr(expected, ty))
    }
}

pub(crate) fn drop_attrs(attrs: AttrsIter<'_>) -> Result<(), Error> {
    for attr in attrs {
        drop_attr(attr?);
    }
    Ok(())
}

pub(crate) fn drop_attr((ty, value): (u16, &[u8])) {
    debug!("unknown attr {}: {:?}", ty, value);
}
