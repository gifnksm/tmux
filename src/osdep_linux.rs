use ::libc;
extern "C" {
    pub type event_base;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn event_init() -> *mut event_base;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn readlink(__path: *const libc::c_char, __buf: *mut libc::c_char, __len: size_t) -> ssize_t;
    #[no_mangle]
    fn tcgetpgrp(__fd: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
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
/* $OpenBSD$ */
/*
 * Copyright (c) 2009 Nicholas Marriott <nicholas.marriott@gmail.com>
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
pub unsafe extern "C" fn osdep_get_name(
    mut fd: libc::c_int,
    mut _tty: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut ch: libc::c_int = 0;
    let mut pgrp: pid_t = 0;
    pgrp = tcgetpgrp(fd);
    if pgrp == -(1i32) {
        return 0 as *mut libc::c_char;
    }
    xasprintf(
        &mut path as *mut *mut libc::c_char,
        b"/proc/%lld/cmdline\x00" as *const u8 as *const libc::c_char,
        pgrp as libc::c_longlong,
    );
    f = fopen(path, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        free(path as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    free(path as *mut libc::c_void);
    len = 0u64;
    buf = 0 as *mut libc::c_char;
    loop {
        ch = fgetc(f);
        if !(ch != -(1i32)) {
            break;
        }
        if ch == '\u{0}' as i32 {
            break;
        }
        buf = xrealloc(buf as *mut libc::c_void, len.wrapping_add(2u64)) as *mut libc::c_char;
        let fresh0 = len;
        len = len.wrapping_add(1);
        *buf.offset(fresh0 as isize) = ch as libc::c_char
    }
    if !buf.is_null() {
        *buf.offset(len as isize) = '\u{0}' as libc::c_char
    }
    fclose(f);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn osdep_get_cwd(mut fd: libc::c_int) -> *mut libc::c_char {
    static mut target: [libc::c_char; 4097] = [0; 4097];
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pgrp: pid_t = 0;
    let mut sid: pid_t = 0;
    let mut n: ssize_t = 0;
    pgrp = tcgetpgrp(fd);
    if pgrp == -(1i32) {
        return 0 as *mut libc::c_char;
    }
    xasprintf(
        &mut path as *mut *mut libc::c_char,
        b"/proc/%lld/cwd\x00" as *const u8 as *const libc::c_char,
        pgrp as libc::c_longlong,
    );
    n = readlink(path, target.as_mut_ptr(), 4096u64);
    free(path as *mut libc::c_void);
    if n == -1i64 && ioctl(fd, 0x5429u64, &mut sid as *mut pid_t) != -(1i32) {
        xasprintf(
            &mut path as *mut *mut libc::c_char,
            b"/proc/%lld/cwd\x00" as *const u8 as *const libc::c_char,
            sid as libc::c_longlong,
        );
        n = readlink(path, target.as_mut_ptr(), 4096u64);
        free(path as *mut libc::c_void);
    }
    if n > 0i64 {
        target[n as usize] = '\u{0}' as libc::c_char;
        return target.as_mut_ptr();
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn osdep_event_init() -> *mut event_base {
    let mut base: *mut event_base = 0 as *mut event_base;
    /* On Linux, epoll doesn't work on /dev/null (yes, really). */
    setenv(
        b"EVENT_NOEPOLL\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        1i32,
    );
    base = event_init();
    unsetenv(b"EVENT_NOEPOLL\x00" as *const u8 as *const libc::c_char);
    return base;
}