use std::ffi::CStr;
use std::mem::size_of;

pub trait Serialize {
    fn len(&self) -> u32;

    /// Serialize `self` into the given buffer `buf`.
    ///
    /// The buffer length must match exactly the value returned from `len()`,
    /// otherwise it may panic.
    fn serialize(&self, buf: &mut [u8]);
}

impl<'a, P: Serialize> Serialize for &'a [P] {
    fn len(&self) -> u32 {
        self.iter().map(|item| item.len()).sum()
    }

    fn serialize(&self, buf: &mut [u8]) {
        let mut remaining = buf;
        for item in self.iter() {
            let item_len = item.len() as usize;
            let (cur, rest) = remaining.split_at_mut(item_len);
            item.serialize(cur);
            remaining = rest;
        }
    }
}

impl<'a> Serialize for &'a CStr {
    fn len(&self) -> u32 {
        self.to_bytes_with_nul().len() as u32
    }

    fn serialize(&self, buf: &mut [u8]) {
        buf.copy_from_slice(self.to_bytes_with_nul());
    }
}

impl Serialize for u32 {
    fn len(&self) -> u32 {
        size_of::<Self>() as u32
    }

    fn serialize(&self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.to_ne_bytes())
    }
}
