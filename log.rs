use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn setvbuf(__stream: *mut FILE, __buf: *mut libc::c_char,
               __modes: libc::c_int, __n: size_t) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vasprintf(__ptr: *mut *mut libc::c_char, __f: *const libc::c_char,
                 __arg: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char,
                _: ...) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn event_set_log_callback(cb: event_log_cb);
    #[no_mangle]
    fn stravis(_: *mut *mut libc::c_char, _: *const libc::c_char,
               _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void)
     -> libc::c_int;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type event_log_cb
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *const libc::c_char)
               -> ()>;
/* $OpenBSD$ */
/*
 * Copyright (c) 2007 Nicholas Marriott <nicholas.marriott@gmail.com>
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
static mut log_file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut log_level: libc::c_int = 0;
/* Log callback for libevent. */
unsafe extern "C" fn log_event_cb(mut severity: libc::c_int,
                                  mut msg: *const libc::c_char) {
    log_debug(b"%s\x00" as *const u8 as *const libc::c_char, msg);
}
/* Increment log level. */
#[no_mangle]
pub unsafe extern "C" fn log_add_level() { log_level += 1; }
/* Get log level. */
#[no_mangle]
pub unsafe extern "C" fn log_get_level() -> libc::c_int { return log_level; }
/* Open logging to file. */
#[no_mangle]
pub unsafe extern "C" fn log_open(mut name: *const libc::c_char) {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    if log_level == 0 as libc::c_int { return }
    log_close();
    xasprintf(&mut path as *mut *mut libc::c_char,
              b"tmux-%s-%ld.log\x00" as *const u8 as *const libc::c_char,
              name, getpid() as libc::c_long);
    log_file = fopen(path, b"a\x00" as *const u8 as *const libc::c_char);
    free(path as *mut libc::c_void);
    if log_file.is_null() { return }
    setvbuf(log_file, 0 as *mut libc::c_char, 1 as libc::c_int,
            0 as libc::c_int as size_t);
    event_set_log_callback(Some(log_event_cb as
                                    unsafe extern "C" fn(_: libc::c_int,
                                                         _:
                                                             *const libc::c_char)
                                        -> ()));
}
/* Toggle logging. */
#[no_mangle]
pub unsafe extern "C" fn log_toggle(mut name: *const libc::c_char) {
    if log_level == 0 as libc::c_int {
        log_level = 1 as libc::c_int;
        log_open(name);
        log_debug(b"log opened\x00" as *const u8 as *const libc::c_char);
    } else {
        log_debug(b"log closed\x00" as *const u8 as *const libc::c_char);
        log_level = 0 as libc::c_int;
        log_close();
    };
}
/* Close logging. */
#[no_mangle]
pub unsafe extern "C" fn log_close() {
    if !log_file.is_null() { fclose(log_file); }
    log_file = 0 as *mut FILE;
    event_set_log_callback(None);
}
/* Write a log message. */
unsafe extern "C" fn log_vwrite(mut msg: *const libc::c_char,
                                mut ap: ::std::ffi::VaList) {
    let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    if log_file.is_null() { return }
    if vasprintf(&mut fmt, msg, ap.as_va_list()) == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    if stravis(&mut out, fmt,
               0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int |
                   0x10 as libc::c_int) == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    if fprintf(log_file,
               b"%lld.%06d %s\n\x00" as *const u8 as *const libc::c_char,
               tv.tv_sec as libc::c_longlong, tv.tv_usec as libc::c_int, out)
           == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    fflush(log_file);
    free(out as *mut libc::c_void);
    free(fmt as *mut libc::c_void);
}
/* Log a debug message. */
#[no_mangle]
pub unsafe extern "C" fn log_debug(mut msg: *const libc::c_char,
                                   mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if log_file.is_null() { return }
    ap = args.clone();
    log_vwrite(msg, ap.as_va_list());
}
/* Log a critical error with error string and die. */
#[no_mangle]
pub unsafe extern "C" fn fatal(mut msg: *const libc::c_char, mut args: ...)
 -> ! {
    let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if asprintf(&mut fmt as *mut *mut libc::c_char,
                b"fatal: %s: %s\x00" as *const u8 as *const libc::c_char, msg,
                strerror(*__errno_location())) == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    log_vwrite(fmt, ap.as_va_list());
    exit(1 as libc::c_int);
}
/* Log a critical error and die. */
#[no_mangle]
pub unsafe extern "C" fn fatalx(mut msg: *const libc::c_char, mut args: ...)
 -> ! {
    let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if asprintf(&mut fmt as *mut *mut libc::c_char,
                b"fatal: %s\x00" as *const u8 as *const libc::c_char, msg) ==
           -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    log_vwrite(fmt, ap.as_va_list());
    exit(1 as libc::c_int);
}
