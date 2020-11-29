use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type re_dfa_t;
    #[no_mangle]
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn regfree(__preg: *mut regex_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type __u_int = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type u_int = __u_int;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2019 Nicholas Marriott <nicholas.marriott@gmail.com>
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
unsafe extern "C" fn regsub_copy(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut text: *const libc::c_char,
    mut start: size_t,
    mut end: size_t,
) {
    let mut add: size_t = end.wrapping_sub(start);
    *buf = xrealloc(
        *buf as *mut libc::c_void,
        (*len).wrapping_add(add).wrapping_add(1u64),
    ) as *mut libc::c_char;
    memcpy(
        (*buf).offset(*len as isize) as *mut libc::c_void,
        text.offset(start as isize) as *const libc::c_void,
        add,
    );
    *len = (*len).wrapping_add(add);
}
unsafe extern "C" fn regsub_expand(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut with: *const libc::c_char,
    mut text: *const libc::c_char,
    mut m: *mut regmatch_t,
    mut n: u_int,
) {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: u_int = 0;
    let mut current_block_5: u64;
    cp = with;
    while *cp as libc::c_int != '\u{0}' as i32 {
        if *cp as libc::c_int == '\\' as i32 {
            cp = cp.offset(1);
            if *cp as libc::c_int >= '0' as i32 && *cp as libc::c_int <= '9' as i32 {
                i = (*cp as libc::c_int - '0' as i32) as u_int;
                if i < n && (*m.offset(i as isize)).rm_so != (*m.offset(i as isize)).rm_eo {
                    regsub_copy(
                        buf,
                        len,
                        text,
                        (*m.offset(i as isize)).rm_so as size_t,
                        (*m.offset(i as isize)).rm_eo as size_t,
                    );
                    current_block_5 = 16559507199688588974;
                } else {
                    current_block_5 = 13109137661213826276;
                }
            } else {
                current_block_5 = 13109137661213826276;
            }
        } else {
            current_block_5 = 13109137661213826276;
        }
        match current_block_5 {
            13109137661213826276 => {
                *buf = xrealloc(*buf as *mut libc::c_void, (*len).wrapping_add(2u64))
                    as *mut libc::c_char;
                let fresh0 = *len;
                *len = (*len).wrapping_add(1);
                *(*buf).offset(fresh0 as isize) = *cp
            }
            _ => {}
        }
        cp = cp.offset(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn regsub(
    mut pattern: *const libc::c_char,
    mut with: *const libc::c_char,
    mut text: *const libc::c_char,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut r: regex_t = regex_t {
        buffer: 0 as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *mut libc::c_char,
        translate: 0 as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut m: [regmatch_t; 10] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 10];
    let mut start: ssize_t = 0;
    let mut end: ssize_t = 0;
    let mut last: ssize_t = 0;
    let mut len: ssize_t = 0i64;
    let mut empty: libc::c_int = 0i32;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    if *text as libc::c_int == '\u{0}' as i32 {
        return xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    }
    if regcomp(&mut r, pattern, flags) != 0i32 {
        return 0 as *mut libc::c_char;
    }
    start = 0i64;
    last = 0i64;
    end = strlen(text) as ssize_t;
    while start <= end {
        if regexec(
            &mut r,
            text.offset(start as isize),
            (::std::mem::size_of::<[regmatch_t; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<regmatch_t>() as libc::c_ulong),
            m.as_mut_ptr(),
            0i32,
        ) != 0i32
        {
            regsub_copy(
                &mut buf,
                &mut len as *mut ssize_t as *mut size_t,
                text,
                start as size_t,
                end as size_t,
            );
            break;
        } else {
            /*
             * Append any text not part of this match (from the end of the
             * last match).
             */
            regsub_copy(
                &mut buf,
                &mut len as *mut ssize_t as *mut size_t,
                text,
                last as size_t,
                (m[0usize].rm_so as libc::c_long + start) as size_t,
            );
            /*
             * If the last match was empty and this one isn't (it is either
             * later or has matched text), expand this match. If it is
             * empty, move on one character and try again from there.
             */
            if empty != 0
                || start + m[0usize].rm_so as libc::c_long != last
                || m[0usize].rm_so != m[0usize].rm_eo
            {
                regsub_expand(
                    &mut buf,
                    &mut len as *mut ssize_t as *mut size_t,
                    with,
                    text.offset(start as isize),
                    m.as_mut_ptr(),
                    (::std::mem::size_of::<[regmatch_t; 10]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<regmatch_t>() as libc::c_ulong)
                        as u_int,
                );
                last = start + m[0usize].rm_eo as libc::c_long;
                start += m[0usize].rm_eo as libc::c_long;
                empty = 0i32
            } else {
                last = start + m[0usize].rm_eo as libc::c_long;
                start += (m[0usize].rm_eo + 1i32) as libc::c_long;
                empty = 1i32
            }
            /* Stop now if anchored to start. */
            if !(*pattern as libc::c_int == '^' as i32) {
                continue;
            }
            regsub_copy(
                &mut buf,
                &mut len as *mut ssize_t as *mut size_t,
                text,
                start as size_t,
                end as size_t,
            );
            break;
        }
    }
    *buf.offset(len as isize) = '\u{0}' as libc::c_char;
    regfree(&mut r);
    return buf;
}
