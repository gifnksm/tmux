use ::libc;
extern "C" {
    pub type stat;
    pub type dirent;
    #[no_mangle]
    fn glob(__pattern: *const libc::c_char, __flags: libc::c_int,
            __errfunc:
                Option<unsafe extern "C" fn(_: *const libc::c_char,
                                            _: libc::c_int) -> libc::c_int>,
            __pglob: *mut glob_t) -> libc::c_int;
    #[no_mangle]
    fn globfree(__pglob: *mut glob_t);
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    /*
 * Copyright (c) 2017 Nicholas Marriott <nicholas.marriott@gmail.com>
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
    fn fatal(_: *const libc::c_char, _: ...);
}
pub type __pid_t = libc::c_int;
pub type __size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                               -> *mut dirent>,
    pub gl_opendir: Option<unsafe extern "C" fn(_: *const libc::c_char)
                               -> *mut libc::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                              _: *mut stat) -> libc::c_int>,
    pub gl_stat: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                             _: *mut stat) -> libc::c_int>,
}
#[no_mangle]
pub unsafe extern "C" fn getdtablecount() -> libc::c_int {
    let mut path: [libc::c_char; 4096] = [0; 4096];
    let mut g: glob_t =
        glob_t{gl_pathc: 0,
               gl_pathv: 0 as *mut *mut libc::c_char,
               gl_offs: 0,
               gl_flags: 0,
               gl_closedir: None,
               gl_readdir: None,
               gl_opendir: None,
               gl_lstat: None,
               gl_stat: None,};
    let mut n: libc::c_int = 0 as libc::c_int;
    if snprintf(path.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as
                    libc::c_ulong,
                b"/proc/%ld/fd/*\x00" as *const u8 as *const libc::c_char,
                getpid() as libc::c_long) < 0 as libc::c_int {
        fatal(b"snprintf overflow\x00" as *const u8 as *const libc::c_char);
    }
    if glob(path.as_mut_ptr(), 0 as libc::c_int, None, &mut g) ==
           0 as libc::c_int {
        n = g.gl_pathc as libc::c_int
    }
    globfree(&mut g);
    return n;
}
