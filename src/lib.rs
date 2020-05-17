//! This crate provides a high-level encapsulation of Linux's
//! [per-task statistics interface](https://www.kernel.org/doc/html/latest/accounting/taskstats.html).
//!
//! Currently only limited information is exposed in `Taskstats` struct,
//! but new field can be easily added for future version.
//! If there are any other fields in `taskstats` struct useful to you,
//! feel free to submit PR to add them.
//!
//! ## Examples
//!
//! Query aggregated taskstats of a task:
//! ```no_run
//! # use taskstats::{TaskstatsConnection, TaskstatsListener};
//! # use std::ffi::CString;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut conn = TaskstatsConnection::new()?;
//! let stats = conn.get_pid_stats(1)?;
//! // ...
//! # Ok(())
//! # }
//! ```
//!
//! Listen to taskstats of exited tasks:
//! ```no_run
//! # use taskstats::{TaskstatsConnection, TaskstatsListener};
//! # use std::ffi::CString;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let conn = TaskstatsConnection::new()?;
//! let cpu_mask = CString::new("0-7\0".to_owned()).unwrap().into_boxed_c_str();
//! let mut listener = TaskstatsListener::register(conn, cpu_mask)?;
//! loop {
//!     let (pid, stats) = listener.get_next()?;
//!     // ...
//! }
//! # }
//! ```
//!
//! ## Note
//!
//! This crate only exposes stats for pid / task, but not tgid / process,
//! because the kernel doesn't really pass much useful information for tgid.
//! It is recommended that you rely on pid stats and aggregate for process manually.

mod conn;
mod error;
mod genetlink;
mod listener;
mod netlink;
mod parse;
mod raw;
mod taskstats;

pub use self::conn::TaskstatsConnection;
pub use self::error::{Error, InvalidMessage};
pub use self::listener::TaskstatsListener;
pub use self::taskstats::Taskstats;
