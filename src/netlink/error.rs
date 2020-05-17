use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidBuffer {
    #[error("insufficient buffer for attr header, got buffer size {0}")]
    AttrHeader(usize),
    #[error("incomplete payload, expect buffer size {0} got {1}")]
    AttrPayload(usize, usize),
    #[error("expect u16, got buffer size {0}")]
    U16(usize),
    #[error("expect u32, got buffer size {0}")]
    U32(usize),
}
