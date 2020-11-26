use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn getprogname() -> *const libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
/*
 * Copyright (c) 1987, 1993, 1994
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/* OPENBSD ORIGINAL: lib/libc/stdlib/getopt.c */
#[no_mangle]
pub static mut BSDopterr: libc::c_int = 1 as libc::c_int;
/* if error message should be printed */
#[no_mangle]
pub static mut BSDoptind: libc::c_int = 1 as libc::c_int;
/* index into parent argv vector */
#[no_mangle]
pub static mut BSDoptopt: libc::c_int = 0;
/* character checked for validity */
#[no_mangle]
pub static mut BSDoptreset: libc::c_int = 0;
/* reset getopt */
#[no_mangle]
pub static mut BSDoptarg: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/*
 * getopt --
 *	Parse argc/argv argument vector.
 */
#[no_mangle]
pub unsafe extern "C" fn BSDgetopt(
    mut nargc: libc::c_int,
    mut nargv: *const *mut libc::c_char,
    mut ostr: *const libc::c_char,
) -> libc::c_int {
    static mut place: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char; /* option letter processing */
    let mut oli: *mut libc::c_char = 0 as *mut libc::c_char; /* option letter list index */
    if ostr.is_null() {
        return -(1 as libc::c_int);
    } /* option letter okay? */
    if BSDoptreset != 0 || *place == 0 {
        /* update scanning pointer */
        BSDoptreset = 0 as libc::c_int;
        if BSDoptind >= nargc || {
            place = *nargv.offset(BSDoptind as isize);
            (*place as libc::c_int) != '-' as i32
        } {
            place = b"\x00" as *const u8 as *const libc::c_char;
            return -(1 as libc::c_int);
        }
        if *place.offset(1 as libc::c_int as isize) as libc::c_int != 0 && {
            place = place.offset(1);
            (*place as libc::c_int) == '-' as i32
        } {
            /* found "--" */
            if *place.offset(1 as libc::c_int as isize) != 0 {
                return '?' as i32;
            }
            BSDoptind += 1;
            place = b"\x00" as *const u8 as *const libc::c_char;
            return -(1 as libc::c_int);
        }
    }
    let fresh0 = place;
    place = place.offset(1);
    BSDoptopt = *fresh0 as libc::c_int;
    if BSDoptopt == ':' as i32 || {
        oli = strchr(ostr, BSDoptopt);
        oli.is_null()
    } {
        /*
         * if the user didn't specify '-' as an option,
         * assume it means -1.
         */
        if BSDoptopt == '-' as i32 {
            return -(1 as libc::c_int);
        }
        if *place == 0 {
            BSDoptind += 1
        }
        if BSDopterr != 0 && *ostr as libc::c_int != ':' as i32 {
            fprintf(
                stderr,
                b"%s: unknown option -- %c\n\x00" as *const u8 as *const libc::c_char,
                getprogname(),
                BSDoptopt,
            );
        }
        return '?' as i32;
    }
    oli = oli.offset(1);
    if *oli as libc::c_int != ':' as i32 {
        /* don't need argument */
        BSDoptarg = 0 as *mut libc::c_char;
        if *place == 0 {
            BSDoptind += 1
        }
    } else {
        /* need an argument */
        if *place != 0 {
            /* no white space */
            BSDoptarg = place as *mut libc::c_char
        } else {
            BSDoptind += 1;
            if nargc <= BSDoptind {
                /* no arg */
                place = b"\x00" as *const u8 as *const libc::c_char;
                if *ostr as libc::c_int == ':' as i32 {
                    return ':' as i32;
                }
                if BSDopterr != 0 {
                    fprintf(
                        stderr,
                        b"%s: option requires an argument -- %c\n\x00" as *const u8
                            as *const libc::c_char,
                        getprogname(),
                        BSDoptopt,
                    );
                }
                return '?' as i32;
            } else {
                /* white space */
                BSDoptarg = *nargv.offset(BSDoptind as isize)
            }
        }
        place = b"\x00" as *const u8 as *const libc::c_char;
        BSDoptind += 1
    }
    return BSDoptopt;
    /* dump back option letter */
}
