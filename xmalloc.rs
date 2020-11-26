use ::libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn reallocarray(__ptr: *mut libc::c_void, __nmemb: size_t, __size: size_t)
        -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn recallocarray(_: *mut libc::c_void, _: size_t, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
/* $OpenBSD$ */
/*
 * Author: Tatu Ylonen <ylo@cs.hut.fi>
 * Copyright (c) 1995 Tatu Ylonen <ylo@cs.hut.fi>, Espoo, Finland
 *                    All rights reserved
 * Versions of malloc and friends that check their results, and never return
 * failure (they call fatalx if they encounter an error).
 *
 * As far as I am concerned, the code I have written for this software
 * can be used freely for any purpose.  Any derived versions of this
 * software must be clearly marked as such, and if the derived work is
 * incompatible with the protocol description in the RFC file, it must be
 * called by a name other than "ssh" or "Secure Shell".
 */
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 as libc::c_int as libc::c_ulong {
        fatalx(b"xmalloc: zero size\x00" as *const u8 as *const libc::c_char);
    }
    ptr = malloc(size);
    if ptr.is_null() {
        fatalx(
            b"xmalloc: allocating %zu bytes: %s\x00" as *const u8 as *const libc::c_char,
            size,
            strerror(*__errno_location()),
        );
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xcalloc(mut nmemb: size_t, mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 as libc::c_int as libc::c_ulong || nmemb == 0 as libc::c_int as libc::c_ulong {
        fatalx(b"xcalloc: zero size\x00" as *const u8 as *const libc::c_char);
    }
    ptr = calloc(nmemb, size);
    if ptr.is_null() {
        fatalx(
            b"xcalloc: allocating %zu * %zu bytes: %s\x00" as *const u8 as *const libc::c_char,
            nmemb,
            size,
            strerror(*__errno_location()),
        );
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return xreallocarray(ptr, 1 as libc::c_int as size_t, size);
}
#[no_mangle]
pub unsafe extern "C" fn xreallocarray(
    mut ptr: *mut libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut new_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if nmemb == 0 as libc::c_int as libc::c_ulong || size == 0 as libc::c_int as libc::c_ulong {
        fatalx(b"xreallocarray: zero size\x00" as *const u8 as *const libc::c_char);
    }
    new_ptr = reallocarray(ptr, nmemb, size);
    if new_ptr.is_null() {
        fatalx(
            b"xreallocarray: allocating %zu * %zu bytes: %s\x00" as *const u8
                as *const libc::c_char,
            nmemb,
            size,
            strerror(*__errno_location()),
        );
    }
    return new_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xrecallocarray(
    mut ptr: *mut libc::c_void,
    mut oldnmemb: size_t,
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut new_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if nmemb == 0 as libc::c_int as libc::c_ulong || size == 0 as libc::c_int as libc::c_ulong {
        fatalx(b"xrecallocarray: zero size\x00" as *const u8 as *const libc::c_char);
    }
    new_ptr = recallocarray(ptr, oldnmemb, nmemb, size);
    if new_ptr.is_null() {
        fatalx(
            b"xrecallocarray: allocating %zu * %zu bytes: %s\x00" as *const u8
                as *const libc::c_char,
            nmemb,
            size,
            strerror(*__errno_location()),
        );
    }
    return new_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strdup(str);
    if cp.is_null() {
        fatalx(
            b"xstrdup: %s\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    return cp;
}
#[no_mangle]
pub unsafe extern "C" fn xstrndup(
    mut str: *const libc::c_char,
    mut maxlen: size_t,
) -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = strndup(str, maxlen);
    if cp.is_null() {
        fatalx(
            b"xstrndup: %s\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    return cp;
}
#[no_mangle]
pub unsafe extern "C" fn xasprintf(
    mut ret: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut i: libc::c_int = 0;
    ap = args.clone();
    i = xvasprintf(ret, fmt, ap.as_va_list());
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn xvasprintf(
    mut ret: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = vasprintf(ret, fmt, ap.as_va_list());
    if i == -(1 as libc::c_int) {
        fatalx(
            b"xasprintf: %s\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn xsnprintf(
    mut str: *mut libc::c_char,
    mut len: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut i: libc::c_int = 0;
    ap = args.clone();
    i = xvsnprintf(str, len, fmt, ap.as_va_list());
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn xvsnprintf(
    mut str: *mut libc::c_char,
    mut len: size_t,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if len > 2147483647 as libc::c_int as libc::c_ulong {
        fatalx(b"xsnprintf: len > INT_MAX\x00" as *const u8 as *const libc::c_char);
    }
    i = vsnprintf(str, len, fmt, ap.as_va_list());
    if i < 0 as libc::c_int || i >= len as libc::c_int {
        fatalx(b"xsnprintf: overflow\x00" as *const u8 as *const libc::c_char);
    }
    return i;
}
