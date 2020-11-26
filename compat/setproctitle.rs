use ::libc;
extern "C" {
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                 _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn getprogname() -> *const libc::c_char;
    #[no_mangle]
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
/*
 * Copyright (c) 2016 Nicholas Marriott <nicholas.marriott@gmail.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF MIND, USE, DATA OR PROFITS, WHETHER
 * IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
#[no_mangle]
pub unsafe extern "C" fn setproctitle(mut fmt: *const libc::c_char,
                                      mut args: ...) {
    let mut title: [libc::c_char; 16] = [0; 16];
    let mut name: [libc::c_char; 16] = [0; 16];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    let mut used: libc::c_int = 0;
    ap = args.clone();
    vsnprintf(title.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
              fmt, ap.as_va_list());
    used =
        snprintf(name.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                 b"%s: %s\x00" as *const u8 as *const libc::c_char,
                 getprogname(), title.as_mut_ptr());
    if used >=
           ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as
               libc::c_int {
        cp = strrchr(name.as_mut_ptr(), ' ' as i32);
        if !cp.is_null() { *cp = '\u{0}' as i32 as libc::c_char }
    }
    prctl(15 as libc::c_int, name.as_mut_ptr());
}
