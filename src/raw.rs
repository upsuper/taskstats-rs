#![allow(dead_code, non_camel_case_types)]

use byteorder::NativeEndian;
use std::os::raw::c_char;
use zerocopy::{AsBytes, FromBytes, U64};

pub const TS_COMM_LEN: usize = 32;
pub const TASKSTATS_GENL_NAME: &[u8] = b"TASKSTATS\0";
pub const TASKSTATS_GENL_VERSION: u8 = 1;

type __u64 = U64<NativeEndian>;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, AsBytes, FromBytes)]
pub struct taskstats_v9 {
    pub version: u16,
    pub _padding_0: u16,
    pub ac_exitcode: u32,
    pub ac_flag: u8,
    pub ac_nice: u8,
    pub _padding_1: u16,
    pub _padding_2: u32,
    pub cpu_count: __u64,
    pub cpu_delay_total: __u64,
    pub blkio_count: __u64,
    pub blkio_delay_total: __u64,
    pub swapin_count: __u64,
    pub swapin_delay_total: __u64,
    pub cpu_run_real_total: __u64,
    pub cpu_run_virtual_total: __u64,
    pub ac_comm: [c_char; TS_COMM_LEN],
    pub ac_sched: u8,
    pub ac_pad: [u8; 3usize],
    pub _padding_3: u32,
    pub ac_uid: u32,
    pub ac_gid: u32,
    pub ac_pid: u32,
    pub ac_ppid: u32,
    pub ac_btime: u32,
    pub _padding_4: u32,
    pub ac_etime: __u64,
    pub ac_utime: __u64,
    pub ac_stime: __u64,
    pub ac_minflt: __u64,
    pub ac_majflt: __u64,
    pub coremem: __u64,
    pub virtmem: __u64,
    pub hiwater_rss: __u64,
    pub hiwater_vm: __u64,
    pub read_char: __u64,
    pub write_char: __u64,
    pub read_syscalls: __u64,
    pub write_syscalls: __u64,
    pub read_bytes: __u64,
    pub write_bytes: __u64,
    pub cancelled_write_bytes: __u64,
    pub nvcsw: __u64,
    pub nivcsw: __u64,
    pub ac_utimescaled: __u64,
    pub ac_stimescaled: __u64,
    pub cpu_scaled_run_real_total: __u64,
    pub freepages_count: __u64,
    pub freepages_delay_total: __u64,
    pub thrashing_count: __u64,
    pub thrashing_delay_total: __u64,
}

type _bindgen_ty_1 = u8;
pub const TASKSTATS_CMD_UNSPEC: _bindgen_ty_1 = 0;
pub const TASKSTATS_CMD_GET: _bindgen_ty_1 = 1;
pub const TASKSTATS_CMD_NEW: _bindgen_ty_1 = 2;
pub const __TASKSTATS_CMD_MAX: _bindgen_ty_1 = 3;

type _bindgen_ty_2 = u16;
pub const TASKSTATS_TYPE_UNSPEC: _bindgen_ty_2 = 0;
pub const TASKSTATS_TYPE_PID: _bindgen_ty_2 = 1;
pub const TASKSTATS_TYPE_TGID: _bindgen_ty_2 = 2;
pub const TASKSTATS_TYPE_STATS: _bindgen_ty_2 = 3;
pub const TASKSTATS_TYPE_AGGR_PID: _bindgen_ty_2 = 4;
pub const TASKSTATS_TYPE_AGGR_TGID: _bindgen_ty_2 = 5;
pub const TASKSTATS_TYPE_NULL: _bindgen_ty_2 = 6;
pub const __TASKSTATS_TYPE_MAX: _bindgen_ty_2 = 7;

type _bindgen_ty_3 = u16;
pub const TASKSTATS_CMD_ATTR_UNSPEC: _bindgen_ty_3 = 0;
pub const TASKSTATS_CMD_ATTR_PID: _bindgen_ty_3 = 1;
pub const TASKSTATS_CMD_ATTR_TGID: _bindgen_ty_3 = 2;
pub const TASKSTATS_CMD_ATTR_REGISTER_CPUMASK: _bindgen_ty_3 = 3;
pub const TASKSTATS_CMD_ATTR_DEREGISTER_CPUMASK: _bindgen_ty_3 = 4;
pub const __TASKSTATS_CMD_ATTR_MAX: _bindgen_ty_3 = 5;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};
    use std::ptr::null;

    #[test]
    fn bindgen_test_layout_taskstats_v9() {
        assert_eq!(size_of::<taskstats_v9>(), 344usize, "Size of: taskstats");
        assert_eq!(align_of::<taskstats_v9>(), 4usize, "Alignment of taskstats");
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).version as *const _ as usize },
            0usize,
            "Offset of field: taskstats::version"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_exitcode as *const _ as usize },
            4usize,
            "Offset of field: taskstats::ac_exitcode"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_flag as *const _ as usize },
            8usize,
            "Offset of field: taskstats::ac_flag"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_nice as *const _ as usize },
            9usize,
            "Offset of field: taskstats::ac_nice"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).cpu_count as *const _ as usize },
            16usize,
            "Offset of field: taskstats::cpu_count"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).cpu_delay_total as *const _ as usize },
            24usize,
            "Offset of field: taskstats::cpu_delay_total"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).blkio_count as *const _ as usize },
            32usize,
            "Offset of field: taskstats::blkio_count"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).blkio_delay_total as *const _ as usize },
            40usize,
            "Offset of field: taskstats::blkio_delay_total"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).swapin_count as *const _ as usize },
            48usize,
            "Offset of field: taskstats::swapin_count"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).swapin_delay_total as *const _ as usize },
            56usize,
            "Offset of field: taskstats::swapin_delay_total"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).cpu_run_real_total as *const _ as usize },
            64usize,
            "Offset of field: taskstats::cpu_run_real_total"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).cpu_run_virtual_total as *const _ as usize },
            72usize,
            "Offset of field: taskstats::cpu_run_virtual_total"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_comm as *const _ as usize },
            80usize,
            "Offset of field: taskstats::ac_comm"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_sched as *const _ as usize },
            112usize,
            "Offset of field: taskstats::ac_sched"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_pad as *const _ as usize },
            113usize,
            "Offset of field: taskstats::ac_pad"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_uid as *const _ as usize },
            120usize,
            "Offset of field: taskstats::ac_uid"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_gid as *const _ as usize },
            124usize,
            "Offset of field: taskstats::ac_gid"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_pid as *const _ as usize },
            128usize,
            "Offset of field: taskstats::ac_pid"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_ppid as *const _ as usize },
            132usize,
            "Offset of field: taskstats::ac_ppid"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_btime as *const _ as usize },
            136usize,
            "Offset of field: taskstats::ac_btime"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_etime as *const _ as usize },
            144usize,
            "Offset of field: taskstats::ac_etime"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_utime as *const _ as usize },
            152usize,
            "Offset of field: taskstats::ac_utime"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_stime as *const _ as usize },
            160usize,
            "Offset of field: taskstats::ac_stime"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_minflt as *const _ as usize },
            168usize,
            "Offset of field: taskstats::ac_minflt"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_majflt as *const _ as usize },
            176usize,
            "Offset of field: taskstats::ac_majflt"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).coremem as *const _ as usize },
            184usize,
            "Offset of field: taskstats::coremem"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).virtmem as *const _ as usize },
            192usize,
            "Offset of field: taskstats::virtmem"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).hiwater_rss as *const _ as usize },
            200usize,
            "Offset of field: taskstats::hiwater_rss"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).hiwater_vm as *const _ as usize },
            208usize,
            "Offset of field: taskstats::hiwater_vm"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).read_char as *const _ as usize },
            216usize,
            "Offset of field: taskstats::read_char"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).write_char as *const _ as usize },
            224usize,
            "Offset of field: taskstats::write_char"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).read_syscalls as *const _ as usize },
            232usize,
            "Offset of field: taskstats::read_syscalls"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).write_syscalls as *const _ as usize },
            240usize,
            "Offset of field: taskstats::write_syscalls"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).read_bytes as *const _ as usize },
            248usize,
            "Offset of field: taskstats::read_bytes"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).write_bytes as *const _ as usize },
            256usize,
            "Offset of field: taskstats::write_bytes"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).cancelled_write_bytes as *const _ as usize },
            264usize,
            "Offset of field: taskstats::cancelled_write_bytes"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).nvcsw as *const _ as usize },
            272usize,
            "Offset of field: taskstats::nvcsw"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).nivcsw as *const _ as usize },
            280usize,
            "Offset of field: taskstats::nivcsw"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_utimescaled as *const _ as usize },
            288usize,
            "Offset of field: taskstats::ac_utimescaled"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).ac_stimescaled as *const _ as usize },
            296usize,
            "Offset of field: taskstats::ac_stimescaled"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).cpu_scaled_run_real_total as *const _ as usize },
            304usize,
            "Offset of field: taskstats::cpu_scaled_run_real_total"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).freepages_count as *const _ as usize },
            312usize,
            "Offset of field: taskstats::freepages_count"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).freepages_delay_total as *const _ as usize },
            320usize,
            "Offset of field: taskstats::freepages_delay_total"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).thrashing_count as *const _ as usize },
            328usize,
            "Offset of field: taskstats::thrashing_count"
        );
        assert_eq!(
            unsafe { &(*(null::<taskstats_v9>())).thrashing_delay_total as *const _ as usize },
            336usize,
            "Offset of field: taskstats::thrashing_delay_total"
        );
    }
}
