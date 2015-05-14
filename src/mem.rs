#[allow(dead_code)]

use libc::{c_void, size_t, c_int};

#[no_mangle]
pub unsafe fn __aeabi_memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
    memmove(dest, src, n as size_t)
}

#[no_mangle]
pub unsafe fn __aeabi_memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    memset(s, c, n as size_t)
}

#[no_mangle]
pub extern fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int {
    unsafe { memalign(memptr, alignment, size) }
}

extern {

    fn memmove(dest: *mut u8, src: *const u8, n: size_t) -> *mut u8;
    fn memset(memptr: *mut u8, c: c_int, n: size_t) -> *mut u8;
    fn memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;

}
