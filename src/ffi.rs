#![allow(warnings, unused_variables, dead_code, improper_ctypes, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use bk_primitives as raw;

pub type u_char = raw::c_uchar;
pub type u_short = raw::c_ushort;
pub type u_int = raw::c_uint;
pub type u_long = raw::c_ulong;
pub const M_NOWAIT: raw::c_uint = 1;
pub const M_WAITOK: raw::c_uint = 2;
pub const M_ZERO: raw::c_uint = 256;
pub const M_NOVM: raw::c_uint = 512;
pub const M_USE_RESERVE: raw::c_uint = 1024;
pub const M_NODUMP: raw::c_uint = 2048;
pub const M_FIRSTFIT: raw::c_uint = 4096;
pub const M_BESTFIT: raw::c_uint = 8192;
pub const M_CONTIG: raw::c_uint = 16384;
pub const M_MAGIC: raw::c_uint = 877983977;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct malloc_type {
    pub ks_next: *mut malloc_type,
    pub ks_magic: u_long,
    pub ks_shortdesc: *const raw::c_char,
    pub ks_handle: *mut raw::c_void,
}

impl Clone for malloc_type {
    fn clone(&self) -> Self {
        *self
    }
}

extern "C" {
    #[link_name = "M_DEVBUF"]
    pub static mut M_DEVBUF: [malloc_type; 1usize];
}

extern "C" {
    pub fn free(addr: *mut raw::c_void, type_: *mut malloc_type);
}
extern "C" {
    pub fn malloc(
        size: raw::c_ulong,
        type_: *mut malloc_type,
        flags: raw::c_int,
    ) -> *mut raw::c_void;
}
extern "C" {
    pub fn realloc(
        addr: *mut raw::c_void,
        size: raw::c_ulong,
        type_: *mut malloc_type,
        flags: raw::c_int,
    ) -> *mut raw::c_void;
}
extern "C" {
    pub fn uprintf(arg1: *const raw::c_char, ...) -> raw::c_int;
}
