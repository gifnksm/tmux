use ::libc;
pub type __u_char = libc::c_uchar;
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
/*
 * unvis - decode characters previously encoded by vis
 */
#[no_mangle]
pub unsafe extern "C" fn unvis(
    mut cp: *mut libc::c_char,
    mut c: libc::c_char,
    mut astate: *mut libc::c_int,
    mut flag: libc::c_int,
) -> libc::c_int {
    if flag & 1i32 != 0 {
        if *astate == 5i32 || *astate == 6i32 {
            *astate = 0i32;
            return 1i32;
        }
        return if *astate == 0i32 { 3i32 } else { -(1i32) };
    }
    match *astate {
        0 => {
            *cp = 0i8;
            if c as libc::c_int == '\\' as i32 {
                *astate = 1i32;
                return 0i32;
            }
            *cp = c;
            return 1i32;
        }
        1 => {
            match c as libc::c_int {
                92 => {
                    *cp = c;
                    *astate = 0i32;
                    return 1i32;
                }
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    *cp = (c as libc::c_int - '0' as i32) as libc::c_char;
                    *astate = 5i32;
                    return 0i32;
                }
                77 => {
                    *cp = 0o200i32 as libc::c_char;
                    *astate = 2i32;
                    return 0i32;
                }
                94 => {
                    *astate = 4i32;
                    return 0i32;
                }
                110 => {
                    *cp = '\n' as libc::c_char;
                    *astate = 0i32;
                    return 1i32;
                }
                114 => {
                    *cp = '\r' as libc::c_char;
                    *astate = 0i32;
                    return 1i32;
                }
                98 => {
                    *cp = '\u{8}' as libc::c_char;
                    *astate = 0i32;
                    return 1i32;
                }
                97 => {
                    *cp = '\u{7}' as libc::c_char;
                    *astate = 0i32;
                    return 1i32;
                }
                118 => {
                    *cp = '\u{b}' as libc::c_char;
                    *astate = 0i32;
                    return 1i32;
                }
                116 => {
                    *cp = '\t' as libc::c_char;
                    *astate = 0i32;
                    return 1i32;
                }
                102 => {
                    *cp = '\u{c}' as libc::c_char;
                    *astate = 0i32;
                    return 1i32;
                }
                115 => {
                    *cp = ' ' as libc::c_char;
                    *astate = 0i32;
                    return 1i32;
                }
                69 => {
                    *cp = '\u{1b}' as libc::c_char;
                    *astate = 0i32;
                    return 1i32;
                }
                10 => {
                    /*
                     * hidden newline
                     */
                    *astate = 0i32;
                    return 3i32;
                }
                36 => {
                    /*
                     * hidden marker
                     */
                    *astate = 0i32;
                    return 3i32;
                }
                _ => {}
            }
            *astate = 0i32;
            return -(1i32);
        }
        2 => {
            if c as libc::c_int == '-' as i32 {
                *astate = 3i32
            } else if c as libc::c_int == '^' as i32 {
                *astate = 4i32
            } else {
                *astate = 0i32;
                return -(1i32);
            }
            return 0i32;
        }
        3 => {
            *astate = 0i32;
            *cp = (*cp as libc::c_int | c as libc::c_int) as libc::c_char;
            return 1i32;
        }
        4 => {
            if c as libc::c_int == '?' as i32 {
                *cp = (*cp as libc::c_int | 0o177i32) as libc::c_char
            } else {
                *cp = (*cp as libc::c_int | c as libc::c_int & 0o37i32) as libc::c_char
            }
            *astate = 0i32;
            return 1i32;
        }
        5 => {
            /* second possible octal digit */
            if c as u_char as libc::c_int >= '0' as i32 && c as u_char as libc::c_int <= '7' as i32
            {
                /*
                 * yes - and maybe a third
                 */
                *cp = (((*cp as libc::c_int) << 3i32) + (c as libc::c_int - '0' as i32))
                    as libc::c_char;
                *astate = 6i32;
                return 0i32;
            }
            /*
             * no - done with current sequence, push back passed char
             */
            *astate = 0i32;
            return 2i32;
        }
        6 => {
            /* third possible octal digit */
            *astate = 0i32;
            if c as u_char as libc::c_int >= '0' as i32 && c as u_char as libc::c_int <= '7' as i32
            {
                *cp = (((*cp as libc::c_int) << 3i32) + (c as libc::c_int - '0' as i32))
                    as libc::c_char;
                return 1i32;
            }
            /*
             * we were done, push back passed char
             */
            return 2i32;
        }
        _ => {
            /*
             * decoder in unknown state - (probably uninitialized)
             */
            *astate = 0i32;
            return -(1i32);
        }
    };
}
/*
 * strunvis - decode src into dst
 *
 *	Number of chars decoded into dst is returned, -1 on error.
 *	Dst is null terminated.
 */
#[no_mangle]
pub unsafe extern "C" fn strunvis(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut start: *mut libc::c_char = dst;
    let mut state: libc::c_int = 0i32;
    loop {
        let fresh0 = src;
        src = src.offset(1);
        c = *fresh0;
        if !(c != 0) {
            break;
        }
        loop {
            match unvis(dst, c, &mut state, 0i32) {
                1 => {
                    dst = dst.offset(1);
                    break;
                }
                2 => dst = dst.offset(1),
                0 | 3 => {
                    break;
                }
                _ => {
                    *dst = '\u{0}' as libc::c_char;
                    return -(1i32);
                }
            }
        }
    }
    if unvis(dst, c, &mut state, 1i32) == 1i32 {
        dst = dst.offset(1)
    }
    *dst = '\u{0}' as libc::c_char;
    return dst.wrapping_offset_from(start) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strnunvis(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut sz: size_t,
) -> ssize_t {
    let mut c: libc::c_char = 0;
    let mut p: libc::c_char = 0;
    let mut start: *mut libc::c_char = dst;
    let mut end: *mut libc::c_char = dst.offset(sz as isize).offset(-(1isize));
    let mut state: libc::c_int = 0i32;
    if sz > 0u64 {
        *end = '\u{0}' as libc::c_char
    }
    loop {
        let fresh1 = src;
        src = src.offset(1);
        c = *fresh1;
        if !(c != 0) {
            break;
        }
        loop {
            match unvis(&mut p, c, &mut state, 0i32) {
                1 => {
                    if dst < end {
                        *dst = p
                    }
                    dst = dst.offset(1);
                    break;
                }
                2 => {
                    if dst < end {
                        *dst = p
                    }
                    dst = dst.offset(1)
                }
                0 | 3 => {
                    break;
                }
                _ => {
                    if dst <= end {
                        *dst = '\u{0}' as libc::c_char
                    }
                    return -1i64;
                }
            }
        }
    }
    if unvis(&mut p, c, &mut state, 1i32) == 1i32 {
        if dst < end {
            *dst = p
        }
        dst = dst.offset(1)
    }
    if dst <= end {
        *dst = '\u{0}' as libc::c_char
    }
    return dst.wrapping_offset_from(start) as libc::c_long;
}
