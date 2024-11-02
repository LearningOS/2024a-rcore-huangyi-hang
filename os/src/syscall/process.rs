//! Process management syscalls
use core::ptr;

use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE}, mm::{translated_byte_buffer, MapPermission, VirtAddr}, task::{
        change_program_brk, current_user_token, exit_current_and_run_next, map_new_area, suspend_current_and_run_next, unmap_old_area, vpn_to_pte, TaskStatus
    }, timer::get_time_us
};
use crate::task::get_taskinfo;
use crate::mm::VPNRange;

#[repr(C)]
#[derive(Debug)]
/// TimeVal
pub struct TimeVal {
    ///
    pub sec: usize,
    ///
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

impl TaskInfo {
    /// new
    pub fn new() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }

    /// increase syscall by id
    pub fn increase_syscall(&mut self, id: usize) {
        self.syscall_times[id] += 1;
    }

    ///
    pub fn get_status(&self) -> TaskStatus{
        self.status
    }
    ///
    pub fn change_status(&mut self, ts: TaskStatus) {
        self.status = ts;
    }
    ///
    pub fn change_time(&mut self, time: usize) {
        self.time = time;
    }
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let mut ts_vec = translated_byte_buffer(current_user_token(), ts as *const u8, core::mem::size_of::<TimeVal>());
    let tem_time = TimeVal {
        sec: get_time_us() / 1000000,
        usec:  get_time_us() % 100000,
    };
    let mut tem_time_ptr = &tem_time as *const TimeVal as *const u8;
    for time_slice in ts_vec.iter_mut() {
        let len = time_slice.len();
        let time_slice_ptr = time_slice.as_mut_ptr();
        unsafe  {
            ptr::copy_nonoverlapping(tem_time_ptr, time_slice_ptr, len);
            tem_time_ptr = tem_time_ptr.wrapping_add(len);
        }

    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let mut ti_vec = translated_byte_buffer(current_user_token(), ti as *const u8, core::mem::size_of::<TaskInfo>());
    let tem_ti =  get_taskinfo();
    let mut tem_ti_ptr = &tem_ti as *const TaskInfo as *const u8;
    for ti_slice in ti_vec.iter_mut() {
        let len = ti_slice.len();
        let ti_slice_ptr = ti_slice.as_mut_ptr();
        unsafe {
            ptr::copy_nonoverlapping(tem_ti_ptr, ti_slice_ptr, len);
            tem_ti_ptr = tem_ti_ptr.wrapping_add(len);
        }
    }
    0
}

// YOUR JOB: Implement mmap.
///
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    const PROT_MASK:usize = 0x7;
    if start % PAGE_SIZE != 0 
        || port & !PROT_MASK != 0 
        || port & PROT_MASK == 0 {
        return -1;
    }
    let start_vpn = VirtAddr::from(start).floor();
    let end_vpn = VirtAddr::from(start + len).ceil();
    let vpns = VPNRange::new(start_vpn, end_vpn);
    for vpn in vpns {
        if let Some(pte) = vpn_to_pte(vpn) {
            if pte.is_valid() {
                return -1;
            }
        }
    }
    // port << 1, because first valid bit of MapPermission is 1th, not 0th
    map_new_area(start_vpn.into(), end_vpn.into(), MapPermission::from_bits_truncate((port << 1) as u8 ) | MapPermission::U);
    0
}

// YOUR JOB: Implement munmap.
///
pub fn sys_munmap(start: usize, len: usize) -> isize {
    if start % PAGE_SIZE != 0 {
        return -1;
    }
    unmap_old_area(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
