use ::libc;
extern "C" {
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __u_int = libc::c_uint;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub name: *const libc::c_char,
    pub attr: libc::c_int,
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2009 Joshua Elsasser <josh@elsasser.org>
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
pub unsafe extern "C" fn attributes_tostring(mut attr: libc::c_int) -> *const libc::c_char {
    static mut buf: [libc::c_char; 512] = [0; 512];
    let mut len: size_t = 0;
    if attr == 0 as libc::c_int {
        return b"none\x00" as *const u8 as *const libc::c_char;
    }
    len = xsnprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        b"%s%s%s%s%s%s%s%s%s%s%s%s%s%s\x00" as *const u8 as *const libc::c_char,
        if attr & 0x80 as libc::c_int != 0 {
            b"acs,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x1 as libc::c_int != 0 {
            b"bright,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x2 as libc::c_int != 0 {
            b"dim,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x4 as libc::c_int != 0 {
            b"underscore,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x8 as libc::c_int != 0 {
            b"blink,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x10 as libc::c_int != 0 {
            b"reverse,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x20 as libc::c_int != 0 {
            b"hidden,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x40 as libc::c_int != 0 {
            b"italics,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x100 as libc::c_int != 0 {
            b"strikethrough,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x200 as libc::c_int != 0 {
            b"double-underscore,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x400 as libc::c_int != 0 {
            b"curly-underscore,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x800 as libc::c_int != 0 {
            b"dotted-underscore,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x1000 as libc::c_int != 0 {
            b"dashed-underscore,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if attr & 0x2000 as libc::c_int != 0 {
            b"overline,\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    ) as size_t;
    if len > 0 as libc::c_int as libc::c_ulong {
        buf[len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] =
            '\u{0}' as i32 as libc::c_char
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn attributes_fromstring(mut str: *const libc::c_char) -> libc::c_int {
    let delimiters: [libc::c_char; 4] =
        *::std::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b" ,|\x00");
    let mut attr: libc::c_int = 0;
    let mut end: size_t = 0;
    let mut i: u_int = 0;
    let mut table: [C2RustUnnamed; 15] = [
        {
            let mut init = C2RustUnnamed {
                name: b"acs\x00" as *const u8 as *const libc::c_char,
                attr: 0x80 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"bright\x00" as *const u8 as *const libc::c_char,
                attr: 0x1 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"bold\x00" as *const u8 as *const libc::c_char,
                attr: 0x1 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"dim\x00" as *const u8 as *const libc::c_char,
                attr: 0x2 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"underscore\x00" as *const u8 as *const libc::c_char,
                attr: 0x4 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"blink\x00" as *const u8 as *const libc::c_char,
                attr: 0x8 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"reverse\x00" as *const u8 as *const libc::c_char,
                attr: 0x10 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"hidden\x00" as *const u8 as *const libc::c_char,
                attr: 0x20 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"italics\x00" as *const u8 as *const libc::c_char,
                attr: 0x40 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"strikethrough\x00" as *const u8 as *const libc::c_char,
                attr: 0x100 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"double-underscore\x00" as *const u8 as *const libc::c_char,
                attr: 0x200 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"curly-underscore\x00" as *const u8 as *const libc::c_char,
                attr: 0x400 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"dotted-underscore\x00" as *const u8 as *const libc::c_char,
                attr: 0x800 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"dashed-underscore\x00" as *const u8 as *const libc::c_char,
                attr: 0x1000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b"overline\x00" as *const u8 as *const libc::c_char,
                attr: 0x2000 as libc::c_int,
            };
            init
        },
    ];
    if *str as libc::c_int == '\u{0}' as i32
        || strcspn(str, delimiters.as_ptr()) == 0 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    if !strchr(
        delimiters.as_ptr(),
        *str.offset(strlen(str).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int,
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    if strcasecmp(str, b"default\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcasecmp(str, b"none\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    attr = 0 as libc::c_int;
    loop {
        end = strcspn(str, delimiters.as_ptr());
        i = 0 as libc::c_int as u_int;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[C2RustUnnamed; 15]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong)
        {
            if !(end != strlen(table[i as usize].name)) {
                if strncasecmp(str, table[i as usize].name, end) == 0 as libc::c_int {
                    attr |= table[i as usize].attr;
                    break;
                }
            }
            i = i.wrapping_add(1)
        }
        if i as libc::c_ulong
            == (::std::mem::size_of::<[C2RustUnnamed; 15]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong)
        {
            return -(1 as libc::c_int);
        }
        str = str.offset(
            end.wrapping_add(strspn(str.offset(end as isize), delimiters.as_ptr())) as isize,
        );
        if !(*str as libc::c_int != '\u{0}' as i32) {
            break;
        }
    }
    return attr;
}
