use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type u_char = __u_char;
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
/* $OpenBSD$ */
/*
 * Copyright (c) 2008 Nicholas Marriott <nicholas.marriott@gmail.com>
 * Copyright (c) 2016 Avi Halachmi <avihpit@yahoo.com>
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
unsafe extern "C" fn colour_dist_sq(
    mut R: libc::c_int,
    mut G: libc::c_int,
    mut B: libc::c_int,
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    return (R - r) * (R - r) + (G - g) * (G - g) + (B - b) * (B - b);
}
unsafe extern "C" fn colour_to_6cube(mut v: libc::c_int) -> libc::c_int {
    if v < 48i32 {
        return 0i32;
    }
    if v < 114i32 {
        return 1i32;
    }
    return (v - 35i32) / 40i32;
}
/*
 * Convert an RGB triplet to the xterm(1) 256 colour palette.
 *
 * xterm provides a 6x6x6 colour cube (16 - 231) and 24 greys (232 - 255). We
 * map our RGB colour to the closest in the cube, also work out the closest
 * grey, and use the nearest of the two.
 *
 * Note that the xterm has much lower resolution for darker colours (they are
 * not evenly spread out), so our 6 levels are not evenly spread: 0x0, 0x5f
 * (95), 0x87 (135), 0xaf (175), 0xd7 (215) and 0xff (255). Greys are more
 * evenly spread (8, 18, 28 ... 238).
 */
#[no_mangle]
pub unsafe extern "C" fn colour_find_rgb(
    mut r: u_char,
    mut g: u_char,
    mut b: u_char,
) -> libc::c_int {
    static mut q2c: [libc::c_int; 6] = [0i32, 0x5fi32, 0x87i32, 0xafi32, 0xd7i32, 0xffi32];
    let mut qr: libc::c_int = 0;
    let mut qg: libc::c_int = 0;
    let mut qb: libc::c_int = 0;
    let mut cr: libc::c_int = 0;
    let mut cg: libc::c_int = 0;
    let mut cb: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut grey_avg: libc::c_int = 0;
    let mut grey_idx: libc::c_int = 0;
    let mut grey: libc::c_int = 0;
    /* Map RGB to 6x6x6 cube. */
    qr = colour_to_6cube(r as libc::c_int);
    cr = q2c[qr as usize];
    qg = colour_to_6cube(g as libc::c_int);
    cg = q2c[qg as usize];
    qb = colour_to_6cube(b as libc::c_int);
    cb = q2c[qb as usize];
    /* If we have hit the colour exactly, return early. */
    if cr == r as libc::c_int && cg == g as libc::c_int && cb == b as libc::c_int {
        return 16i32 + 36i32 * qr + 6i32 * qg + qb | 0x1000000i32;
    }
    /* Work out the closest grey (average of RGB). */
    grey_avg = (r as libc::c_int + g as libc::c_int + b as libc::c_int) / 3i32;
    if grey_avg > 238i32 {
        grey_idx = 23i32
    } else {
        grey_idx = (grey_avg - 3i32) / 10i32
    }
    grey = 8i32 + 10i32 * grey_idx;
    /* Is grey or 6x6x6 colour closest? */
    d = colour_dist_sq(
        cr,
        cg,
        cb,
        r as libc::c_int,
        g as libc::c_int,
        b as libc::c_int,
    );
    if colour_dist_sq(
        grey,
        grey,
        grey,
        r as libc::c_int,
        g as libc::c_int,
        b as libc::c_int,
    ) < d
    {
        idx = 232i32 + grey_idx
    } else {
        idx = 16i32 + 36i32 * qr + 6i32 * qg + qb
    }
    return idx | 0x1000000i32;
}
/* Join RGB into a colour. */
#[no_mangle]
pub unsafe extern "C" fn colour_join_rgb(
    mut r: u_char,
    mut g: u_char,
    mut b: u_char,
) -> libc::c_int {
    return (r as libc::c_int & 0xffi32) << 16i32
        | (g as libc::c_int & 0xffi32) << 8i32
        | b as libc::c_int & 0xffi32
        | 0x2000000i32;
}
/* Split colour into RGB. */
#[no_mangle]
pub unsafe extern "C" fn colour_split_rgb(
    mut c: libc::c_int,
    mut r: *mut u_char,
    mut g: *mut u_char,
    mut b: *mut u_char,
) {
    *r = (c >> 16i32 & 0xffi32) as u_char;
    *g = (c >> 8i32 & 0xffi32) as u_char;
    *b = (c & 0xffi32) as u_char;
}
/* Convert colour to a string. */
#[no_mangle]
pub unsafe extern "C" fn colour_tostring(mut c: libc::c_int) -> *const libc::c_char {
    static mut s: [libc::c_char; 32] = [0; 32];
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    if c & 0x2000000i32 != 0 {
        colour_split_rgb(c, &mut r, &mut g, &mut b);
        xsnprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"#%02x%02x%02x\x00" as *const u8 as *const libc::c_char,
            r as libc::c_int,
            g as libc::c_int,
            b as libc::c_int,
        );
        return s.as_mut_ptr();
    }
    if c & 0x1000000i32 != 0 {
        xsnprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"colour%u\x00" as *const u8 as *const libc::c_char,
            c & 0xffi32,
        );
        return s.as_mut_ptr();
    }
    match c {
        0 => return b"black\x00" as *const u8 as *const libc::c_char,
        1 => return b"red\x00" as *const u8 as *const libc::c_char,
        2 => return b"green\x00" as *const u8 as *const libc::c_char,
        3 => return b"yellow\x00" as *const u8 as *const libc::c_char,
        4 => return b"blue\x00" as *const u8 as *const libc::c_char,
        5 => return b"magenta\x00" as *const u8 as *const libc::c_char,
        6 => return b"cyan\x00" as *const u8 as *const libc::c_char,
        7 => return b"white\x00" as *const u8 as *const libc::c_char,
        8 => return b"default\x00" as *const u8 as *const libc::c_char,
        9 => return b"terminal\x00" as *const u8 as *const libc::c_char,
        90 => return b"brightblack\x00" as *const u8 as *const libc::c_char,
        91 => return b"brightred\x00" as *const u8 as *const libc::c_char,
        92 => return b"brightgreen\x00" as *const u8 as *const libc::c_char,
        93 => return b"brightyellow\x00" as *const u8 as *const libc::c_char,
        94 => return b"brightblue\x00" as *const u8 as *const libc::c_char,
        95 => return b"brightmagenta\x00" as *const u8 as *const libc::c_char,
        96 => return b"brightcyan\x00" as *const u8 as *const libc::c_char,
        97 => return b"brightwhite\x00" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return b"invalid\x00" as *const u8 as *const libc::c_char;
}
/* Convert colour from string. */
#[no_mangle]
pub unsafe extern "C" fn colour_fromstring(mut s: *const libc::c_char) -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    if *s as libc::c_int == '#' as i32 && strlen(s) == 7u64 {
        cp = s.offset(1isize);
        while *(*__ctype_b_loc()).offset(*cp as u_char as libc::c_int as isize) as libc::c_int
            & _ISxdigit as libc::c_ushort as libc::c_int
            != 0
        {
            cp = cp.offset(1)
        }
        if *cp as libc::c_int != '\u{0}' as i32 {
            return -(1i32);
        }
        n = sscanf(
            s.offset(1isize),
            b"%2hhx%2hhx%2hhx\x00" as *const u8 as *const libc::c_char,
            &mut r as *mut u_char,
            &mut g as *mut u_char,
            &mut b as *mut u_char,
        );
        if n != 3i32 {
            return -(1i32);
        }
        return colour_join_rgb(r, g, b);
    }
    if strncasecmp(
        s,
        b"colour\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong).wrapping_sub(1u64),
    ) == 0i32
    {
        n = strtonum(
            s.offset(::std::mem::size_of::<[libc::c_char; 7]>() as isize)
                .offset(-(1isize)),
            0i64,
            255i64,
            &mut errstr,
        ) as libc::c_int;
        if !errstr.is_null() {
            return -(1i32);
        }
        return n | 0x1000000i32;
    }
    if strncasecmp(
        s,
        b"color\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong).wrapping_sub(1u64),
    ) == 0i32
    {
        n = strtonum(
            s.offset(::std::mem::size_of::<[libc::c_char; 6]>() as isize)
                .offset(-(1isize)),
            0i64,
            255i64,
            &mut errstr,
        ) as libc::c_int;
        if !errstr.is_null() {
            return -(1i32);
        }
        return n | 0x1000000i32;
    }
    if strcasecmp(s, b"default\x00" as *const u8 as *const libc::c_char) == 0i32 {
        return 8i32;
    }
    if strcasecmp(s, b"terminal\x00" as *const u8 as *const libc::c_char) == 0i32 {
        return 9i32;
    }
    if strcasecmp(s, b"black\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"0\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 0i32;
    }
    if strcasecmp(s, b"red\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"1\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 1i32;
    }
    if strcasecmp(s, b"green\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"2\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 2i32;
    }
    if strcasecmp(s, b"yellow\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"3\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 3i32;
    }
    if strcasecmp(s, b"blue\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"4\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 4i32;
    }
    if strcasecmp(s, b"magenta\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"5\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 5i32;
    }
    if strcasecmp(s, b"cyan\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"6\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 6i32;
    }
    if strcasecmp(s, b"white\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"7\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 7i32;
    }
    if strcasecmp(s, b"brightblack\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"90\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 90i32;
    }
    if strcasecmp(s, b"brightred\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"91\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 91i32;
    }
    if strcasecmp(s, b"brightgreen\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"92\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 92i32;
    }
    if strcasecmp(s, b"brightyellow\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"93\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 93i32;
    }
    if strcasecmp(s, b"brightblue\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"94\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 94i32;
    }
    if strcasecmp(s, b"brightmagenta\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"95\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 95i32;
    }
    if strcasecmp(s, b"brightcyan\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"96\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 96i32;
    }
    if strcasecmp(s, b"brightwhite\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(s, b"97\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        return 97i32;
    }
    return -(1i32);
}
/* Convert 256 colour to RGB colour. */
#[no_mangle]
pub unsafe extern "C" fn colour_256toRGB(mut c: libc::c_int) -> libc::c_int {
    static mut table: [libc::c_int; 256] = [
        0i32,
        0x800000i32,
        0x8000i32,
        0x808000i32,
        0x80i32,
        0x800080i32,
        0x8080i32,
        0xc0c0c0i32,
        0x808080i32,
        0xff0000i32,
        0xff00i32,
        0xffff00i32,
        0xffi32,
        0xff00ffi32,
        0xffffi32,
        0xffffffi32,
        0i32,
        0x5fi32,
        0x87i32,
        0xafi32,
        0xd7i32,
        0xffi32,
        0x5f00i32,
        0x5f5fi32,
        0x5f87i32,
        0x5fafi32,
        0x5fd7i32,
        0x5fffi32,
        0x8700i32,
        0x875fi32,
        0x8787i32,
        0x87afi32,
        0x87d7i32,
        0x87ffi32,
        0xaf00i32,
        0xaf5fi32,
        0xaf87i32,
        0xafafi32,
        0xafd7i32,
        0xafffi32,
        0xd700i32,
        0xd75fi32,
        0xd787i32,
        0xd7afi32,
        0xd7d7i32,
        0xd7ffi32,
        0xff00i32,
        0xff5fi32,
        0xff87i32,
        0xffafi32,
        0xffd7i32,
        0xffffi32,
        0x5f0000i32,
        0x5f005fi32,
        0x5f0087i32,
        0x5f00afi32,
        0x5f00d7i32,
        0x5f00ffi32,
        0x5f5f00i32,
        0x5f5f5fi32,
        0x5f5f87i32,
        0x5f5fafi32,
        0x5f5fd7i32,
        0x5f5fffi32,
        0x5f8700i32,
        0x5f875fi32,
        0x5f8787i32,
        0x5f87afi32,
        0x5f87d7i32,
        0x5f87ffi32,
        0x5faf00i32,
        0x5faf5fi32,
        0x5faf87i32,
        0x5fafafi32,
        0x5fafd7i32,
        0x5fafffi32,
        0x5fd700i32,
        0x5fd75fi32,
        0x5fd787i32,
        0x5fd7afi32,
        0x5fd7d7i32,
        0x5fd7ffi32,
        0x5fff00i32,
        0x5fff5fi32,
        0x5fff87i32,
        0x5fffafi32,
        0x5fffd7i32,
        0x5fffffi32,
        0x870000i32,
        0x87005fi32,
        0x870087i32,
        0x8700afi32,
        0x8700d7i32,
        0x8700ffi32,
        0x875f00i32,
        0x875f5fi32,
        0x875f87i32,
        0x875fafi32,
        0x875fd7i32,
        0x875fffi32,
        0x878700i32,
        0x87875fi32,
        0x878787i32,
        0x8787afi32,
        0x8787d7i32,
        0x8787ffi32,
        0x87af00i32,
        0x87af5fi32,
        0x87af87i32,
        0x87afafi32,
        0x87afd7i32,
        0x87afffi32,
        0x87d700i32,
        0x87d75fi32,
        0x87d787i32,
        0x87d7afi32,
        0x87d7d7i32,
        0x87d7ffi32,
        0x87ff00i32,
        0x87ff5fi32,
        0x87ff87i32,
        0x87ffafi32,
        0x87ffd7i32,
        0x87ffffi32,
        0xaf0000i32,
        0xaf005fi32,
        0xaf0087i32,
        0xaf00afi32,
        0xaf00d7i32,
        0xaf00ffi32,
        0xaf5f00i32,
        0xaf5f5fi32,
        0xaf5f87i32,
        0xaf5fafi32,
        0xaf5fd7i32,
        0xaf5fffi32,
        0xaf8700i32,
        0xaf875fi32,
        0xaf8787i32,
        0xaf87afi32,
        0xaf87d7i32,
        0xaf87ffi32,
        0xafaf00i32,
        0xafaf5fi32,
        0xafaf87i32,
        0xafafafi32,
        0xafafd7i32,
        0xafafffi32,
        0xafd700i32,
        0xafd75fi32,
        0xafd787i32,
        0xafd7afi32,
        0xafd7d7i32,
        0xafd7ffi32,
        0xafff00i32,
        0xafff5fi32,
        0xafff87i32,
        0xafffafi32,
        0xafffd7i32,
        0xafffffi32,
        0xd70000i32,
        0xd7005fi32,
        0xd70087i32,
        0xd700afi32,
        0xd700d7i32,
        0xd700ffi32,
        0xd75f00i32,
        0xd75f5fi32,
        0xd75f87i32,
        0xd75fafi32,
        0xd75fd7i32,
        0xd75fffi32,
        0xd78700i32,
        0xd7875fi32,
        0xd78787i32,
        0xd787afi32,
        0xd787d7i32,
        0xd787ffi32,
        0xd7af00i32,
        0xd7af5fi32,
        0xd7af87i32,
        0xd7afafi32,
        0xd7afd7i32,
        0xd7afffi32,
        0xd7d700i32,
        0xd7d75fi32,
        0xd7d787i32,
        0xd7d7afi32,
        0xd7d7d7i32,
        0xd7d7ffi32,
        0xd7ff00i32,
        0xd7ff5fi32,
        0xd7ff87i32,
        0xd7ffafi32,
        0xd7ffd7i32,
        0xd7ffffi32,
        0xff0000i32,
        0xff005fi32,
        0xff0087i32,
        0xff00afi32,
        0xff00d7i32,
        0xff00ffi32,
        0xff5f00i32,
        0xff5f5fi32,
        0xff5f87i32,
        0xff5fafi32,
        0xff5fd7i32,
        0xff5fffi32,
        0xff8700i32,
        0xff875fi32,
        0xff8787i32,
        0xff87afi32,
        0xff87d7i32,
        0xff87ffi32,
        0xffaf00i32,
        0xffaf5fi32,
        0xffaf87i32,
        0xffafafi32,
        0xffafd7i32,
        0xffafffi32,
        0xffd700i32,
        0xffd75fi32,
        0xffd787i32,
        0xffd7afi32,
        0xffd7d7i32,
        0xffd7ffi32,
        0xffff00i32,
        0xffff5fi32,
        0xffff87i32,
        0xffffafi32,
        0xffffd7i32,
        0xffffffi32,
        0x80808i32,
        0x121212i32,
        0x1c1c1ci32,
        0x262626i32,
        0x303030i32,
        0x3a3a3ai32,
        0x444444i32,
        0x4e4e4ei32,
        0x585858i32,
        0x626262i32,
        0x6c6c6ci32,
        0x767676i32,
        0x808080i32,
        0x8a8a8ai32,
        0x949494i32,
        0x9e9e9ei32,
        0xa8a8a8i32,
        0xb2b2b2i32,
        0xbcbcbci32,
        0xc6c6c6i32,
        0xd0d0d0i32,
        0xdadadai32,
        0xe4e4e4i32,
        0xeeeeeei32,
    ];
    return table[(c & 0xffi32) as usize] | 0x2000000i32;
}
/* Convert 256 colour to 16 colour. */
#[no_mangle]
pub unsafe extern "C" fn colour_256to16(mut c: libc::c_int) -> libc::c_int {
    static mut table: [libc::c_char; 256] = [
        0i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8, 10i8, 11i8, 12i8, 13i8, 14i8, 15i8, 0i8,
        4i8, 4i8, 4i8, 12i8, 12i8, 2i8, 6i8, 4i8, 4i8, 12i8, 12i8, 2i8, 2i8, 6i8, 4i8, 12i8, 12i8,
        2i8, 2i8, 2i8, 6i8, 12i8, 12i8, 10i8, 10i8, 10i8, 10i8, 14i8, 12i8, 10i8, 10i8, 10i8, 10i8,
        10i8, 14i8, 1i8, 5i8, 4i8, 4i8, 12i8, 12i8, 3i8, 8i8, 4i8, 4i8, 12i8, 12i8, 2i8, 2i8, 6i8,
        4i8, 12i8, 12i8, 2i8, 2i8, 2i8, 6i8, 12i8, 12i8, 10i8, 10i8, 10i8, 10i8, 14i8, 12i8, 10i8,
        10i8, 10i8, 10i8, 10i8, 14i8, 1i8, 1i8, 5i8, 4i8, 12i8, 12i8, 1i8, 1i8, 5i8, 4i8, 12i8,
        12i8, 3i8, 3i8, 8i8, 4i8, 12i8, 12i8, 2i8, 2i8, 2i8, 6i8, 12i8, 12i8, 10i8, 10i8, 10i8,
        10i8, 14i8, 12i8, 10i8, 10i8, 10i8, 10i8, 10i8, 14i8, 1i8, 1i8, 1i8, 5i8, 12i8, 12i8, 1i8,
        1i8, 1i8, 5i8, 12i8, 12i8, 1i8, 1i8, 1i8, 5i8, 12i8, 12i8, 3i8, 3i8, 3i8, 7i8, 12i8, 12i8,
        10i8, 10i8, 10i8, 10i8, 14i8, 12i8, 10i8, 10i8, 10i8, 10i8, 10i8, 14i8, 9i8, 9i8, 9i8, 9i8,
        13i8, 12i8, 9i8, 9i8, 9i8, 9i8, 13i8, 12i8, 9i8, 9i8, 9i8, 9i8, 13i8, 12i8, 9i8, 9i8, 9i8,
        9i8, 13i8, 12i8, 11i8, 11i8, 11i8, 11i8, 7i8, 12i8, 10i8, 10i8, 10i8, 10i8, 10i8, 14i8,
        9i8, 9i8, 9i8, 9i8, 9i8, 13i8, 9i8, 9i8, 9i8, 9i8, 9i8, 13i8, 9i8, 9i8, 9i8, 9i8, 9i8,
        13i8, 9i8, 9i8, 9i8, 9i8, 9i8, 13i8, 9i8, 9i8, 9i8, 9i8, 9i8, 13i8, 11i8, 11i8, 11i8, 11i8,
        11i8, 15i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 8i8, 8i8, 8i8, 8i8, 8i8, 8i8, 7i8, 7i8, 7i8, 7i8,
        7i8, 7i8, 15i8, 15i8, 15i8, 15i8, 15i8, 15i8,
    ];
    return table[(c & 0xffi32) as usize] as libc::c_int;
}
