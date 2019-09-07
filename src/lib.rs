#![no_std]
#![deny(warnings)]

pub mod ffi;
use crate::ffi as kern;
use bk_primitives as raw;

use core::alloc::{GlobalAlloc, Layout};

#[cfg(all(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "asmjs",
    target_arch = "wasm32"
)))]
const MIN_ALIGN: usize = 8;
#[cfg(all(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "s390x",
    target_arch = "sparc64"
)))]
const MIN_ALIGN: usize = 16;

pub struct BkAllocator;

impl BkAllocator {
    unsafe fn alloc(&self, layout: Layout, flags: i32) -> *mut u8 {
        kern::malloc(
            layout.size() as raw::c_size_t,
            &mut kern::M_DEVBUF[0],
            flags,
        ) as *mut u8
    }
    unsafe fn alloc_aligned(&self, _layout: Layout, _flags: i32) -> *mut u8 {
        unimplemented!();
    }
}

unsafe impl GlobalAlloc for BkAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let flags = kern::M_WAITOK as i32;
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            self.alloc(layout, flags)
        } else {
            self.alloc_aligned(layout, flags)
        }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let flags = kern::M_WAITOK as i32 | kern::M_ZERO as i32;
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            self.alloc(layout, flags)
        } else {
            self.alloc_aligned(layout, flags)
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        kern::free(ptr as *mut raw::c_void, &mut kern::M_DEVBUF[0])
    }
}
