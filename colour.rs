use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char,
                   _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strtonum(_: *const libc::c_char, _: libc::c_longlong,
                _: libc::c_longlong, _: *mut *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char,
                 _: ...) -> libc::c_int;
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
unsafe extern "C" fn colour_dist_sq(mut R: libc::c_int, mut G: libc::c_int,
                                    mut B: libc::c_int, mut r: libc::c_int,
                                    mut g: libc::c_int, mut b: libc::c_int)
 -> libc::c_int {
    return (R - r) * (R - r) + (G - g) * (G - g) + (B - b) * (B - b);
}
unsafe extern "C" fn colour_to_6cube(mut v: libc::c_int) -> libc::c_int {
    if v < 48 as libc::c_int { return 0 as libc::c_int }
    if v < 114 as libc::c_int { return 1 as libc::c_int }
    return (v - 35 as libc::c_int) / 40 as libc::c_int;
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
pub unsafe extern "C" fn colour_find_rgb(mut r: u_char, mut g: u_char,
                                         mut b: u_char) -> libc::c_int {
    static mut q2c: [libc::c_int; 6] =
        [0 as libc::c_int, 0x5f as libc::c_int, 0x87 as libc::c_int,
         0xaf as libc::c_int, 0xd7 as libc::c_int, 0xff as libc::c_int];
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
    if cr == r as libc::c_int && cg == g as libc::c_int &&
           cb == b as libc::c_int {
        return 16 as libc::c_int + 36 as libc::c_int * qr +
                   6 as libc::c_int * qg + qb | 0x1000000 as libc::c_int
    }
    /* Work out the closest grey (average of RGB). */
    grey_avg =
        (r as libc::c_int + g as libc::c_int + b as libc::c_int) /
            3 as libc::c_int;
    if grey_avg > 238 as libc::c_int {
        grey_idx = 23 as libc::c_int
    } else { grey_idx = (grey_avg - 3 as libc::c_int) / 10 as libc::c_int }
    grey = 8 as libc::c_int + 10 as libc::c_int * grey_idx;
    /* Is grey or 6x6x6 colour closest? */
    d =
        colour_dist_sq(cr, cg, cb, r as libc::c_int, g as libc::c_int,
                       b as libc::c_int);
    if colour_dist_sq(grey, grey, grey, r as libc::c_int, g as libc::c_int,
                      b as libc::c_int) < d {
        idx = 232 as libc::c_int + grey_idx
    } else {
        idx =
            16 as libc::c_int + 36 as libc::c_int * qr + 6 as libc::c_int * qg
                + qb
    }
    return idx | 0x1000000 as libc::c_int;
}
/* Join RGB into a colour. */
#[no_mangle]
pub unsafe extern "C" fn colour_join_rgb(mut r: u_char, mut g: u_char,
                                         mut b: u_char) -> libc::c_int {
    return (r as libc::c_int & 0xff as libc::c_int) << 16 as libc::c_int |
               (g as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int |
               b as libc::c_int & 0xff as libc::c_int |
               0x2000000 as libc::c_int;
}
/* Split colour into RGB. */
#[no_mangle]
pub unsafe extern "C" fn colour_split_rgb(mut c: libc::c_int,
                                          mut r: *mut u_char,
                                          mut g: *mut u_char,
                                          mut b: *mut u_char) {
    *r = (c >> 16 as libc::c_int & 0xff as libc::c_int) as u_char;
    *g = (c >> 8 as libc::c_int & 0xff as libc::c_int) as u_char;
    *b = (c & 0xff as libc::c_int) as u_char;
}
/* Convert colour to a string. */
#[no_mangle]
pub unsafe extern "C" fn colour_tostring(mut c: libc::c_int)
 -> *const libc::c_char {
    static mut s: [libc::c_char; 32] = [0; 32];
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    if c & 0x2000000 as libc::c_int != 0 {
        colour_split_rgb(c, &mut r, &mut g, &mut b);
        xsnprintf(s.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong,
                  b"#%02x%02x%02x\x00" as *const u8 as *const libc::c_char,
                  r as libc::c_int, g as libc::c_int, b as libc::c_int);
        return s.as_mut_ptr()
    }
    if c & 0x1000000 as libc::c_int != 0 {
        xsnprintf(s.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 32]>() as
                      libc::c_ulong,
                  b"colour%u\x00" as *const u8 as *const libc::c_char,
                  c & 0xff as libc::c_int);
        return s.as_mut_ptr()
    }
    match c {
        0 => { return b"black\x00" as *const u8 as *const libc::c_char }
        1 => { return b"red\x00" as *const u8 as *const libc::c_char }
        2 => { return b"green\x00" as *const u8 as *const libc::c_char }
        3 => { return b"yellow\x00" as *const u8 as *const libc::c_char }
        4 => { return b"blue\x00" as *const u8 as *const libc::c_char }
        5 => { return b"magenta\x00" as *const u8 as *const libc::c_char }
        6 => { return b"cyan\x00" as *const u8 as *const libc::c_char }
        7 => { return b"white\x00" as *const u8 as *const libc::c_char }
        8 => { return b"default\x00" as *const u8 as *const libc::c_char }
        9 => { return b"terminal\x00" as *const u8 as *const libc::c_char }
        90 => {
            return b"brightblack\x00" as *const u8 as *const libc::c_char
        }
        91 => { return b"brightred\x00" as *const u8 as *const libc::c_char }
        92 => {
            return b"brightgreen\x00" as *const u8 as *const libc::c_char
        }
        93 => {
            return b"brightyellow\x00" as *const u8 as *const libc::c_char
        }
        94 => { return b"brightblue\x00" as *const u8 as *const libc::c_char }
        95 => {
            return b"brightmagenta\x00" as *const u8 as *const libc::c_char
        }
        96 => { return b"brightcyan\x00" as *const u8 as *const libc::c_char }
        97 => {
            return b"brightwhite\x00" as *const u8 as *const libc::c_char
        }
        _ => { }
    }
    return b"invalid\x00" as *const u8 as *const libc::c_char;
}
/* Convert colour from string. */
#[no_mangle]
pub unsafe extern "C" fn colour_fromstring(mut s: *const libc::c_char)
 -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    if *s as libc::c_int == '#' as i32 &&
           strlen(s) == 7 as libc::c_int as libc::c_ulong {
        cp = s.offset(1 as libc::c_int as isize);
        while *(*__ctype_b_loc()).offset(*cp as u_char as libc::c_int as
                                             isize) as libc::c_int &
                  _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            cp = cp.offset(1)
        }
        if *cp as libc::c_int != '\u{0}' as i32 { return -(1 as libc::c_int) }
        n =
            sscanf(s.offset(1 as libc::c_int as isize),
                   b"%2hhx%2hhx%2hhx\x00" as *const u8 as *const libc::c_char,
                   &mut r as *mut u_char, &mut g as *mut u_char,
                   &mut b as *mut u_char);
        if n != 3 as libc::c_int { return -(1 as libc::c_int) }
        return colour_join_rgb(r, g, b)
    }
    if strncasecmp(s, b"colour\x00" as *const u8 as *const libc::c_char,
                   (::std::mem::size_of::<[libc::c_char; 7]>() as
                        libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong)) ==
           0 as libc::c_int {
        n =
            strtonum(s.offset(::std::mem::size_of::<[libc::c_char; 7]>() as
                                  libc::c_ulong as
                                  isize).offset(-(1 as libc::c_int as isize)),
                     0 as libc::c_int as libc::c_longlong,
                     255 as libc::c_int as libc::c_longlong, &mut errstr) as
                libc::c_int;
        if !errstr.is_null() { return -(1 as libc::c_int) }
        return n | 0x1000000 as libc::c_int
    }
    if strncasecmp(s, b"color\x00" as *const u8 as *const libc::c_char,
                   (::std::mem::size_of::<[libc::c_char; 6]>() as
                        libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong)) ==
           0 as libc::c_int {
        n =
            strtonum(s.offset(::std::mem::size_of::<[libc::c_char; 6]>() as
                                  libc::c_ulong as
                                  isize).offset(-(1 as libc::c_int as isize)),
                     0 as libc::c_int as libc::c_longlong,
                     255 as libc::c_int as libc::c_longlong, &mut errstr) as
                libc::c_int;
        if !errstr.is_null() { return -(1 as libc::c_int) }
        return n | 0x1000000 as libc::c_int
    }
    if strcasecmp(s, b"default\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return 8 as libc::c_int
    }
    if strcasecmp(s, b"terminal\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return 9 as libc::c_int
    }
    if strcasecmp(s, b"black\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"0\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 0 as libc::c_int
    }
    if strcasecmp(s, b"red\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"1\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 1 as libc::c_int
    }
    if strcasecmp(s, b"green\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"2\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 2 as libc::c_int
    }
    if strcasecmp(s, b"yellow\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"3\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 3 as libc::c_int
    }
    if strcasecmp(s, b"blue\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"4\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 4 as libc::c_int
    }
    if strcasecmp(s, b"magenta\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"5\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 5 as libc::c_int
    }
    if strcasecmp(s, b"cyan\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"6\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 6 as libc::c_int
    }
    if strcasecmp(s, b"white\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"7\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 7 as libc::c_int
    }
    if strcasecmp(s, b"brightblack\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int ||
           strcmp(s, b"90\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 90 as libc::c_int
    }
    if strcasecmp(s, b"brightred\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"91\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 91 as libc::c_int
    }
    if strcasecmp(s, b"brightgreen\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int ||
           strcmp(s, b"92\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 92 as libc::c_int
    }
    if strcasecmp(s, b"brightyellow\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int ||
           strcmp(s, b"93\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 93 as libc::c_int
    }
    if strcasecmp(s, b"brightblue\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"94\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 94 as libc::c_int
    }
    if strcasecmp(s, b"brightmagenta\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int ||
           strcmp(s, b"95\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 95 as libc::c_int
    }
    if strcasecmp(s, b"brightcyan\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int ||
           strcmp(s, b"96\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 96 as libc::c_int
    }
    if strcasecmp(s, b"brightwhite\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int ||
           strcmp(s, b"97\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 97 as libc::c_int
    }
    return -(1 as libc::c_int);
}
/* Convert 256 colour to RGB colour. */
#[no_mangle]
pub unsafe extern "C" fn colour_256toRGB(mut c: libc::c_int) -> libc::c_int {
    static mut table: [libc::c_int; 256] =
        [0 as libc::c_int, 0x800000 as libc::c_int, 0x8000 as libc::c_int,
         0x808000 as libc::c_int, 0x80 as libc::c_int,
         0x800080 as libc::c_int, 0x8080 as libc::c_int,
         0xc0c0c0 as libc::c_int, 0x808080 as libc::c_int,
         0xff0000 as libc::c_int, 0xff00 as libc::c_int,
         0xffff00 as libc::c_int, 0xff as libc::c_int,
         0xff00ff as libc::c_int, 0xffff as libc::c_int,
         0xffffff as libc::c_int, 0 as libc::c_int, 0x5f as libc::c_int,
         0x87 as libc::c_int, 0xaf as libc::c_int, 0xd7 as libc::c_int,
         0xff as libc::c_int, 0x5f00 as libc::c_int, 0x5f5f as libc::c_int,
         0x5f87 as libc::c_int, 0x5faf as libc::c_int, 0x5fd7 as libc::c_int,
         0x5fff as libc::c_int, 0x8700 as libc::c_int, 0x875f as libc::c_int,
         0x8787 as libc::c_int, 0x87af as libc::c_int, 0x87d7 as libc::c_int,
         0x87ff as libc::c_int, 0xaf00 as libc::c_int, 0xaf5f as libc::c_int,
         0xaf87 as libc::c_int, 0xafaf as libc::c_int, 0xafd7 as libc::c_int,
         0xafff as libc::c_int, 0xd700 as libc::c_int, 0xd75f as libc::c_int,
         0xd787 as libc::c_int, 0xd7af as libc::c_int, 0xd7d7 as libc::c_int,
         0xd7ff as libc::c_int, 0xff00 as libc::c_int, 0xff5f as libc::c_int,
         0xff87 as libc::c_int, 0xffaf as libc::c_int, 0xffd7 as libc::c_int,
         0xffff as libc::c_int, 0x5f0000 as libc::c_int,
         0x5f005f as libc::c_int, 0x5f0087 as libc::c_int,
         0x5f00af as libc::c_int, 0x5f00d7 as libc::c_int,
         0x5f00ff as libc::c_int, 0x5f5f00 as libc::c_int,
         0x5f5f5f as libc::c_int, 0x5f5f87 as libc::c_int,
         0x5f5faf as libc::c_int, 0x5f5fd7 as libc::c_int,
         0x5f5fff as libc::c_int, 0x5f8700 as libc::c_int,
         0x5f875f as libc::c_int, 0x5f8787 as libc::c_int,
         0x5f87af as libc::c_int, 0x5f87d7 as libc::c_int,
         0x5f87ff as libc::c_int, 0x5faf00 as libc::c_int,
         0x5faf5f as libc::c_int, 0x5faf87 as libc::c_int,
         0x5fafaf as libc::c_int, 0x5fafd7 as libc::c_int,
         0x5fafff as libc::c_int, 0x5fd700 as libc::c_int,
         0x5fd75f as libc::c_int, 0x5fd787 as libc::c_int,
         0x5fd7af as libc::c_int, 0x5fd7d7 as libc::c_int,
         0x5fd7ff as libc::c_int, 0x5fff00 as libc::c_int,
         0x5fff5f as libc::c_int, 0x5fff87 as libc::c_int,
         0x5fffaf as libc::c_int, 0x5fffd7 as libc::c_int,
         0x5fffff as libc::c_int, 0x870000 as libc::c_int,
         0x87005f as libc::c_int, 0x870087 as libc::c_int,
         0x8700af as libc::c_int, 0x8700d7 as libc::c_int,
         0x8700ff as libc::c_int, 0x875f00 as libc::c_int,
         0x875f5f as libc::c_int, 0x875f87 as libc::c_int,
         0x875faf as libc::c_int, 0x875fd7 as libc::c_int,
         0x875fff as libc::c_int, 0x878700 as libc::c_int,
         0x87875f as libc::c_int, 0x878787 as libc::c_int,
         0x8787af as libc::c_int, 0x8787d7 as libc::c_int,
         0x8787ff as libc::c_int, 0x87af00 as libc::c_int,
         0x87af5f as libc::c_int, 0x87af87 as libc::c_int,
         0x87afaf as libc::c_int, 0x87afd7 as libc::c_int,
         0x87afff as libc::c_int, 0x87d700 as libc::c_int,
         0x87d75f as libc::c_int, 0x87d787 as libc::c_int,
         0x87d7af as libc::c_int, 0x87d7d7 as libc::c_int,
         0x87d7ff as libc::c_int, 0x87ff00 as libc::c_int,
         0x87ff5f as libc::c_int, 0x87ff87 as libc::c_int,
         0x87ffaf as libc::c_int, 0x87ffd7 as libc::c_int,
         0x87ffff as libc::c_int, 0xaf0000 as libc::c_int,
         0xaf005f as libc::c_int, 0xaf0087 as libc::c_int,
         0xaf00af as libc::c_int, 0xaf00d7 as libc::c_int,
         0xaf00ff as libc::c_int, 0xaf5f00 as libc::c_int,
         0xaf5f5f as libc::c_int, 0xaf5f87 as libc::c_int,
         0xaf5faf as libc::c_int, 0xaf5fd7 as libc::c_int,
         0xaf5fff as libc::c_int, 0xaf8700 as libc::c_int,
         0xaf875f as libc::c_int, 0xaf8787 as libc::c_int,
         0xaf87af as libc::c_int, 0xaf87d7 as libc::c_int,
         0xaf87ff as libc::c_int, 0xafaf00 as libc::c_int,
         0xafaf5f as libc::c_int, 0xafaf87 as libc::c_int,
         0xafafaf as libc::c_int, 0xafafd7 as libc::c_int,
         0xafafff as libc::c_int, 0xafd700 as libc::c_int,
         0xafd75f as libc::c_int, 0xafd787 as libc::c_int,
         0xafd7af as libc::c_int, 0xafd7d7 as libc::c_int,
         0xafd7ff as libc::c_int, 0xafff00 as libc::c_int,
         0xafff5f as libc::c_int, 0xafff87 as libc::c_int,
         0xafffaf as libc::c_int, 0xafffd7 as libc::c_int,
         0xafffff as libc::c_int, 0xd70000 as libc::c_int,
         0xd7005f as libc::c_int, 0xd70087 as libc::c_int,
         0xd700af as libc::c_int, 0xd700d7 as libc::c_int,
         0xd700ff as libc::c_int, 0xd75f00 as libc::c_int,
         0xd75f5f as libc::c_int, 0xd75f87 as libc::c_int,
         0xd75faf as libc::c_int, 0xd75fd7 as libc::c_int,
         0xd75fff as libc::c_int, 0xd78700 as libc::c_int,
         0xd7875f as libc::c_int, 0xd78787 as libc::c_int,
         0xd787af as libc::c_int, 0xd787d7 as libc::c_int,
         0xd787ff as libc::c_int, 0xd7af00 as libc::c_int,
         0xd7af5f as libc::c_int, 0xd7af87 as libc::c_int,
         0xd7afaf as libc::c_int, 0xd7afd7 as libc::c_int,
         0xd7afff as libc::c_int, 0xd7d700 as libc::c_int,
         0xd7d75f as libc::c_int, 0xd7d787 as libc::c_int,
         0xd7d7af as libc::c_int, 0xd7d7d7 as libc::c_int,
         0xd7d7ff as libc::c_int, 0xd7ff00 as libc::c_int,
         0xd7ff5f as libc::c_int, 0xd7ff87 as libc::c_int,
         0xd7ffaf as libc::c_int, 0xd7ffd7 as libc::c_int,
         0xd7ffff as libc::c_int, 0xff0000 as libc::c_int,
         0xff005f as libc::c_int, 0xff0087 as libc::c_int,
         0xff00af as libc::c_int, 0xff00d7 as libc::c_int,
         0xff00ff as libc::c_int, 0xff5f00 as libc::c_int,
         0xff5f5f as libc::c_int, 0xff5f87 as libc::c_int,
         0xff5faf as libc::c_int, 0xff5fd7 as libc::c_int,
         0xff5fff as libc::c_int, 0xff8700 as libc::c_int,
         0xff875f as libc::c_int, 0xff8787 as libc::c_int,
         0xff87af as libc::c_int, 0xff87d7 as libc::c_int,
         0xff87ff as libc::c_int, 0xffaf00 as libc::c_int,
         0xffaf5f as libc::c_int, 0xffaf87 as libc::c_int,
         0xffafaf as libc::c_int, 0xffafd7 as libc::c_int,
         0xffafff as libc::c_int, 0xffd700 as libc::c_int,
         0xffd75f as libc::c_int, 0xffd787 as libc::c_int,
         0xffd7af as libc::c_int, 0xffd7d7 as libc::c_int,
         0xffd7ff as libc::c_int, 0xffff00 as libc::c_int,
         0xffff5f as libc::c_int, 0xffff87 as libc::c_int,
         0xffffaf as libc::c_int, 0xffffd7 as libc::c_int,
         0xffffff as libc::c_int, 0x80808 as libc::c_int,
         0x121212 as libc::c_int, 0x1c1c1c as libc::c_int,
         0x262626 as libc::c_int, 0x303030 as libc::c_int,
         0x3a3a3a as libc::c_int, 0x444444 as libc::c_int,
         0x4e4e4e as libc::c_int, 0x585858 as libc::c_int,
         0x626262 as libc::c_int, 0x6c6c6c as libc::c_int,
         0x767676 as libc::c_int, 0x808080 as libc::c_int,
         0x8a8a8a as libc::c_int, 0x949494 as libc::c_int,
         0x9e9e9e as libc::c_int, 0xa8a8a8 as libc::c_int,
         0xb2b2b2 as libc::c_int, 0xbcbcbc as libc::c_int,
         0xc6c6c6 as libc::c_int, 0xd0d0d0 as libc::c_int,
         0xdadada as libc::c_int, 0xe4e4e4 as libc::c_int,
         0xeeeeee as libc::c_int];
    return table[(c & 0xff as libc::c_int) as usize] |
               0x2000000 as libc::c_int;
}
/* Convert 256 colour to 16 colour. */
#[no_mangle]
pub unsafe extern "C" fn colour_256to16(mut c: libc::c_int) -> libc::c_int {
    static mut table: [libc::c_char; 256] =
        [0 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
         2 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
         4 as libc::c_int as libc::c_char, 5 as libc::c_int as libc::c_char,
         6 as libc::c_int as libc::c_char, 7 as libc::c_int as libc::c_char,
         8 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 11 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 13 as libc::c_int as libc::c_char,
         14 as libc::c_int as libc::c_char, 15 as libc::c_int as libc::c_char,
         0 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
         4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         2 as libc::c_int as libc::c_char, 6 as libc::c_int as libc::c_char,
         4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
         6 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
         2 as libc::c_int as libc::c_char, 6 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         14 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 14 as libc::c_int as libc::c_char,
         1 as libc::c_int as libc::c_char, 5 as libc::c_int as libc::c_char,
         4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         3 as libc::c_int as libc::c_char, 8 as libc::c_int as libc::c_char,
         4 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
         6 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
         2 as libc::c_int as libc::c_char, 6 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         14 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 14 as libc::c_int as libc::c_char,
         1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
         5 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
         5 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
         8 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         2 as libc::c_int as libc::c_char, 2 as libc::c_int as libc::c_char,
         2 as libc::c_int as libc::c_char, 6 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         14 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 14 as libc::c_int as libc::c_char,
         1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
         1 as libc::c_int as libc::c_char, 5 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
         1 as libc::c_int as libc::c_char, 5 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
         1 as libc::c_int as libc::c_char, 5 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         3 as libc::c_int as libc::c_char, 3 as libc::c_int as libc::c_char,
         3 as libc::c_int as libc::c_char, 7 as libc::c_int as libc::c_char,
         12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         14 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 14 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         13 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         13 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         13 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         13 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         11 as libc::c_int as libc::c_char, 11 as libc::c_int as libc::c_char,
         11 as libc::c_int as libc::c_char, 11 as libc::c_int as libc::c_char,
         7 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char,
         10 as libc::c_int as libc::c_char, 14 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 13 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 13 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 13 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 13 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 9 as libc::c_int as libc::c_char,
         9 as libc::c_int as libc::c_char, 13 as libc::c_int as libc::c_char,
         11 as libc::c_int as libc::c_char, 11 as libc::c_int as libc::c_char,
         11 as libc::c_int as libc::c_char, 11 as libc::c_int as libc::c_char,
         11 as libc::c_int as libc::c_char, 15 as libc::c_int as libc::c_char,
         0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
         0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
         0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
         8 as libc::c_int as libc::c_char, 8 as libc::c_int as libc::c_char,
         8 as libc::c_int as libc::c_char, 8 as libc::c_int as libc::c_char,
         8 as libc::c_int as libc::c_char, 8 as libc::c_int as libc::c_char,
         7 as libc::c_int as libc::c_char, 7 as libc::c_int as libc::c_char,
         7 as libc::c_int as libc::c_char, 7 as libc::c_int as libc::c_char,
         7 as libc::c_int as libc::c_char, 7 as libc::c_int as libc::c_char,
         15 as libc::c_int as libc::c_char, 15 as libc::c_int as libc::c_char,
         15 as libc::c_int as libc::c_char, 15 as libc::c_int as libc::c_char,
         15 as libc::c_int as libc::c_char,
         15 as libc::c_int as libc::c_char];
    return table[(c & 0xff as libc::c_int) as usize] as libc::c_int;
}
