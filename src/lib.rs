#![no_std]
#![deny(warnings)]

mod ffi;
use crate::ffi as kern;
use bk_primitives as raw;

use core::alloc::{GlobalAlloc, Layout};

pub struct BkAllocator;

impl BkAllocator {
    unsafe fn alloc(&self, layout: Layout, flags: i32) -> *mut u8 {
        kern::malloc(
            layout.size() as raw::c_size_t,
            &mut kern::M_DEVBUF[0],
            flags,
        ) as *mut u8
    }
}

unsafe impl GlobalAlloc for BkAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let flags = kern::M_WAITOK as i32;
        self.alloc(layout, flags)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let flags = kern::M_WAITOK as i32 | kern::M_ZERO as i32;
        self.alloc(layout, flags)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        kern::free(ptr as *mut raw::c_void, &mut kern::M_DEVBUF[0])
    }
    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        let flags = kern::M_WAITOK as i32;
        kern::realloc(
            ptr as *mut raw::c_void,
            new_size as raw::c_ulong,
            &mut kern::M_DEVBUF[0],
            flags,
        ) as *mut u8
    }
}

#[global_allocator]
static BK_ALLOCATOR: BkAllocator = BkAllocator;
