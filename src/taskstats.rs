use crate::raw::taskstats_v9;

/// Parsed taskstats data
#[derive(Debug)]
#[non_exhaustive]
pub struct Taskstats {
    /// Bytes of read I/O, from `taskstats.read_bytes`
    pub read_bytes: u64,
    /// Bytes of write I/O, from `taskstats.write_bytes`
    pub write_bytes: u64,
    /// Bytes of cancelled write I/O, from `taskstats.cancelled_write_bytes`
    pub cancelled_write_bytes: u64,
}

impl From<&taskstats_v9> for Taskstats {
    fn from(raw: &taskstats_v9) -> Self {
        Self {
            read_bytes: raw.read_bytes.get(),
            write_bytes: raw.write_bytes.get(),
            cancelled_write_bytes: raw.cancelled_write_bytes.get(),
        }
    }
}
