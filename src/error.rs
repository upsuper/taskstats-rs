use crate::genetlink::FamilyError;
use libc::pid_t;
use std::io;
use thiserror::Error;

/// Taskstats error
#[derive(Debug, Error)]
pub enum Error {
    #[error("io")]
    Io(#[from] io::Error),
    #[error("failed to resolve family id")]
    Family(#[from] FamilyError),
    #[error("taskstats service unavailable")]
    Unavailable,
    #[error("unknown command {0}")]
    UnexpectedCmd(u8),
    #[error("invalid taskstats message")]
    InvalidMessage(#[from] InvalidMessage),
    #[error("taskstats version too low: {0}")]
    UnsupportedVersion(u16),
    #[error("unexpected pid, expected {0} got {1}")]
    UnexpectedPid(pid_t, pid_t),
    #[error("unexpected empty result from listener")]
    UnexpectedEmptyResult,
    #[error("no taskstats received")]
    NoResult,
}

/// Invalid taskstats message
#[derive(Debug, Error)]
pub enum InvalidMessage {
    #[error("invalid netlink message")]
    Netlink(#[from] crate::netlink::InvalidBuffer),
    #[error("invalid generic netlink message")]
    GenericNetlink(#[from] crate::genetlink::InvalidBuffer),
    #[error("unexpected attribute, expected {0} got {1}")]
    Attr(u16, u16),
    #[error("expected pid attribute, found nothing")]
    EmptyAggrPid,
    #[error("expected tgid attribute, found nothing")]
    EmptyAggrTgid,
    #[error("expected stats attribute, found nothing")]
    NoStats,
}

impl From<crate::netlink::InvalidBuffer> for Error {
    fn from(inner: crate::netlink::InvalidBuffer) -> Self {
        Error::InvalidMessage(inner.into())
    }
}

impl From<crate::genetlink::InvalidBuffer> for Error {
    fn from(inner: crate::genetlink::InvalidBuffer) -> Self {
        Error::InvalidMessage(inner.into())
    }
}
