use ::libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __u_char = libc::c_uchar;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
pub type u_int = __u_int;
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
/*
 * vis - visually encode characters
 */
#[no_mangle]
pub unsafe extern "C" fn vis(
    mut dst: *mut libc::c_char,
    mut c: libc::c_int,
    mut flag: libc::c_int,
    mut nextc: libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    if (c == '\\' as i32 || flag & 0x400i32 == 0i32)
        && (c as u_int <= (127i32 * 2i32 + 1i32) as libc::c_uint
            && c as u_char as libc::c_int & !(0x7fi32) == 0i32
            && (c != '*' as i32 && c != '?' as i32 && c != '[' as i32 && c != '#' as i32
                || flag & 0x100i32 == 0i32)
            && *(*__ctype_b_loc()).offset(c as u_char as libc::c_int as isize) as libc::c_int
                & _ISgraph as libc::c_ushort as libc::c_int
                != 0
            || flag & 0x4i32 == 0i32 && c == ' ' as i32
            || flag & 0x8i32 == 0i32 && c == '\t' as i32
            || flag & 0x10i32 == 0i32 && c == '\n' as i32
            || flag & 0x20i32 != 0
                && (c == '\u{8}' as i32
                    || c == '\u{7}' as i32
                    || c == '\r' as i32
                    || *(*__ctype_b_loc()).offset(c as u_char as libc::c_int as isize)
                        as libc::c_int
                        & _ISgraph as libc::c_ushort as libc::c_int
                        != 0))
    {
        if c == '\"' as i32 && flag & 0x200i32 != 0i32 || c == '\\' as i32 && flag & 0x40i32 == 0i32
        {
            let fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = '\\' as libc::c_char
        }
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = c as libc::c_char;
        *dst = '\u{0}' as libc::c_char;
        return dst;
    }
    if flag & 0x2i32 != 0 {
        match c {
            10 => {
                current_block = 8121314696114196120;
                match current_block {
                    8121314696114196120 => {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\\' as libc::c_char;
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = 'n' as libc::c_char
                    }
                    5892893968624351926 => {
                        let fresh16 = dst;
                        dst = dst.offset(1);
                        *fresh16 = '\\' as libc::c_char;
                        let fresh17 = dst;
                        dst = dst.offset(1);
                        *fresh17 = 's' as libc::c_char
                    }
                    3692292822783516354 => {
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = '\\' as libc::c_char;
                        let fresh15 = dst;
                        dst = dst.offset(1);
                        *fresh15 = 'f' as libc::c_char
                    }
                    13839326668931527616 => {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as libc::c_char;
                        let fresh13 = dst;
                        dst = dst.offset(1);
                        *fresh13 = 't' as libc::c_char
                    }
                    5310961604937924882 => {
                        let fresh10 = dst;
                        dst = dst.offset(1);
                        *fresh10 = '\\' as libc::c_char;
                        let fresh11 = dst;
                        dst = dst.offset(1);
                        *fresh11 = 'v' as libc::c_char
                    }
                    6153397765503504804 => {
                        let fresh8 = dst;
                        dst = dst.offset(1);
                        *fresh8 = '\\' as libc::c_char;
                        let fresh9 = dst;
                        dst = dst.offset(1);
                        *fresh9 = 'a' as libc::c_char
                    }
                    8337753555966048743 => {
                        let fresh6 = dst;
                        dst = dst.offset(1);
                        *fresh6 = '\\' as libc::c_char;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = 'b' as libc::c_char
                    }
                    2667368111120228261 => {
                        let fresh4 = dst;
                        dst = dst.offset(1);
                        *fresh4 = '\\' as libc::c_char;
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = 'r' as libc::c_char
                    }
                    _ => {
                        let fresh18 = dst;
                        dst = dst.offset(1);
                        *fresh18 = '\\' as libc::c_char;
                        let fresh19 = dst;
                        dst = dst.offset(1);
                        *fresh19 = '0' as libc::c_char;
                        if nextc as u_char as libc::c_int >= '0' as i32
                            && nextc as u_char as libc::c_int <= '7' as i32
                        {
                            let fresh20 = dst;
                            dst = dst.offset(1);
                            *fresh20 = '0' as libc::c_char;
                            let fresh21 = dst;
                            dst = dst.offset(1);
                            *fresh21 = '0' as libc::c_char
                        }
                    }
                }
                current_block = 18394067709589936313;
            }
            13 => {
                current_block = 2667368111120228261;
                match current_block {
                    8121314696114196120 => {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\\' as libc::c_char;
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = 'n' as libc::c_char
                    }
                    5892893968624351926 => {
                        let fresh16 = dst;
                        dst = dst.offset(1);
                        *fresh16 = '\\' as libc::c_char;
                        let fresh17 = dst;
                        dst = dst.offset(1);
                        *fresh17 = 's' as libc::c_char
                    }
                    3692292822783516354 => {
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = '\\' as libc::c_char;
                        let fresh15 = dst;
                        dst = dst.offset(1);
                        *fresh15 = 'f' as libc::c_char
                    }
                    13839326668931527616 => {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as libc::c_char;
                        let fresh13 = dst;
                        dst = dst.offset(1);
                        *fresh13 = 't' as libc::c_char
                    }
                    5310961604937924882 => {
                        let fresh10 = dst;
                        dst = dst.offset(1);
                        *fresh10 = '\\' as libc::c_char;
                        let fresh11 = dst;
                        dst = dst.offset(1);
                        *fresh11 = 'v' as libc::c_char
                    }
                    6153397765503504804 => {
                        let fresh8 = dst;
                        dst = dst.offset(1);
                        *fresh8 = '\\' as libc::c_char;
                        let fresh9 = dst;
                        dst = dst.offset(1);
                        *fresh9 = 'a' as libc::c_char
                    }
                    8337753555966048743 => {
                        let fresh6 = dst;
                        dst = dst.offset(1);
                        *fresh6 = '\\' as libc::c_char;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = 'b' as libc::c_char
                    }
                    2667368111120228261 => {
                        let fresh4 = dst;
                        dst = dst.offset(1);
                        *fresh4 = '\\' as libc::c_char;
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = 'r' as libc::c_char
                    }
                    _ => {
                        let fresh18 = dst;
                        dst = dst.offset(1);
                        *fresh18 = '\\' as libc::c_char;
                        let fresh19 = dst;
                        dst = dst.offset(1);
                        *fresh19 = '0' as libc::c_char;
                        if nextc as u_char as libc::c_int >= '0' as i32
                            && nextc as u_char as libc::c_int <= '7' as i32
                        {
                            let fresh20 = dst;
                            dst = dst.offset(1);
                            *fresh20 = '0' as libc::c_char;
                            let fresh21 = dst;
                            dst = dst.offset(1);
                            *fresh21 = '0' as libc::c_char
                        }
                    }
                }
                current_block = 18394067709589936313;
            }
            8 => {
                current_block = 8337753555966048743;
                match current_block {
                    8121314696114196120 => {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\\' as libc::c_char;
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = 'n' as libc::c_char
                    }
                    5892893968624351926 => {
                        let fresh16 = dst;
                        dst = dst.offset(1);
                        *fresh16 = '\\' as libc::c_char;
                        let fresh17 = dst;
                        dst = dst.offset(1);
                        *fresh17 = 's' as libc::c_char
                    }
                    3692292822783516354 => {
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = '\\' as libc::c_char;
                        let fresh15 = dst;
                        dst = dst.offset(1);
                        *fresh15 = 'f' as libc::c_char
                    }
                    13839326668931527616 => {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as libc::c_char;
                        let fresh13 = dst;
                        dst = dst.offset(1);
                        *fresh13 = 't' as libc::c_char
                    }
                    5310961604937924882 => {
                        let fresh10 = dst;
                        dst = dst.offset(1);
                        *fresh10 = '\\' as libc::c_char;
                        let fresh11 = dst;
                        dst = dst.offset(1);
                        *fresh11 = 'v' as libc::c_char
                    }
                    6153397765503504804 => {
                        let fresh8 = dst;
                        dst = dst.offset(1);
                        *fresh8 = '\\' as libc::c_char;
                        let fresh9 = dst;
                        dst = dst.offset(1);
                        *fresh9 = 'a' as libc::c_char
                    }
                    8337753555966048743 => {
                        let fresh6 = dst;
                        dst = dst.offset(1);
                        *fresh6 = '\\' as libc::c_char;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = 'b' as libc::c_char
                    }
                    2667368111120228261 => {
                        let fresh4 = dst;
                        dst = dst.offset(1);
                        *fresh4 = '\\' as libc::c_char;
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = 'r' as libc::c_char
                    }
                    _ => {
                        let fresh18 = dst;
                        dst = dst.offset(1);
                        *fresh18 = '\\' as libc::c_char;
                        let fresh19 = dst;
                        dst = dst.offset(1);
                        *fresh19 = '0' as libc::c_char;
                        if nextc as u_char as libc::c_int >= '0' as i32
                            && nextc as u_char as libc::c_int <= '7' as i32
                        {
                            let fresh20 = dst;
                            dst = dst.offset(1);
                            *fresh20 = '0' as libc::c_char;
                            let fresh21 = dst;
                            dst = dst.offset(1);
                            *fresh21 = '0' as libc::c_char
                        }
                    }
                }
                current_block = 18394067709589936313;
            }
            7 => {
                current_block = 6153397765503504804;
                match current_block {
                    8121314696114196120 => {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\\' as libc::c_char;
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = 'n' as libc::c_char
                    }
                    5892893968624351926 => {
                        let fresh16 = dst;
                        dst = dst.offset(1);
                        *fresh16 = '\\' as libc::c_char;
                        let fresh17 = dst;
                        dst = dst.offset(1);
                        *fresh17 = 's' as libc::c_char
                    }
                    3692292822783516354 => {
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = '\\' as libc::c_char;
                        let fresh15 = dst;
                        dst = dst.offset(1);
                        *fresh15 = 'f' as libc::c_char
                    }
                    13839326668931527616 => {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as libc::c_char;
                        let fresh13 = dst;
                        dst = dst.offset(1);
                        *fresh13 = 't' as libc::c_char
                    }
                    5310961604937924882 => {
                        let fresh10 = dst;
                        dst = dst.offset(1);
                        *fresh10 = '\\' as libc::c_char;
                        let fresh11 = dst;
                        dst = dst.offset(1);
                        *fresh11 = 'v' as libc::c_char
                    }
                    6153397765503504804 => {
                        let fresh8 = dst;
                        dst = dst.offset(1);
                        *fresh8 = '\\' as libc::c_char;
                        let fresh9 = dst;
                        dst = dst.offset(1);
                        *fresh9 = 'a' as libc::c_char
                    }
                    8337753555966048743 => {
                        let fresh6 = dst;
                        dst = dst.offset(1);
                        *fresh6 = '\\' as libc::c_char;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = 'b' as libc::c_char
                    }
                    2667368111120228261 => {
                        let fresh4 = dst;
                        dst = dst.offset(1);
                        *fresh4 = '\\' as libc::c_char;
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = 'r' as libc::c_char
                    }
                    _ => {
                        let fresh18 = dst;
                        dst = dst.offset(1);
                        *fresh18 = '\\' as libc::c_char;
                        let fresh19 = dst;
                        dst = dst.offset(1);
                        *fresh19 = '0' as libc::c_char;
                        if nextc as u_char as libc::c_int >= '0' as i32
                            && nextc as u_char as libc::c_int <= '7' as i32
                        {
                            let fresh20 = dst;
                            dst = dst.offset(1);
                            *fresh20 = '0' as libc::c_char;
                            let fresh21 = dst;
                            dst = dst.offset(1);
                            *fresh21 = '0' as libc::c_char
                        }
                    }
                }
                current_block = 18394067709589936313;
            }
            11 => {
                current_block = 5310961604937924882;
                match current_block {
                    8121314696114196120 => {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\\' as libc::c_char;
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = 'n' as libc::c_char
                    }
                    5892893968624351926 => {
                        let fresh16 = dst;
                        dst = dst.offset(1);
                        *fresh16 = '\\' as libc::c_char;
                        let fresh17 = dst;
                        dst = dst.offset(1);
                        *fresh17 = 's' as libc::c_char
                    }
                    3692292822783516354 => {
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = '\\' as libc::c_char;
                        let fresh15 = dst;
                        dst = dst.offset(1);
                        *fresh15 = 'f' as libc::c_char
                    }
                    13839326668931527616 => {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as libc::c_char;
                        let fresh13 = dst;
                        dst = dst.offset(1);
                        *fresh13 = 't' as libc::c_char
                    }
                    5310961604937924882 => {
                        let fresh10 = dst;
                        dst = dst.offset(1);
                        *fresh10 = '\\' as libc::c_char;
                        let fresh11 = dst;
                        dst = dst.offset(1);
                        *fresh11 = 'v' as libc::c_char
                    }
                    6153397765503504804 => {
                        let fresh8 = dst;
                        dst = dst.offset(1);
                        *fresh8 = '\\' as libc::c_char;
                        let fresh9 = dst;
                        dst = dst.offset(1);
                        *fresh9 = 'a' as libc::c_char
                    }
                    8337753555966048743 => {
                        let fresh6 = dst;
                        dst = dst.offset(1);
                        *fresh6 = '\\' as libc::c_char;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = 'b' as libc::c_char
                    }
                    2667368111120228261 => {
                        let fresh4 = dst;
                        dst = dst.offset(1);
                        *fresh4 = '\\' as libc::c_char;
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = 'r' as libc::c_char
                    }
                    _ => {
                        let fresh18 = dst;
                        dst = dst.offset(1);
                        *fresh18 = '\\' as libc::c_char;
                        let fresh19 = dst;
                        dst = dst.offset(1);
                        *fresh19 = '0' as libc::c_char;
                        if nextc as u_char as libc::c_int >= '0' as i32
                            && nextc as u_char as libc::c_int <= '7' as i32
                        {
                            let fresh20 = dst;
                            dst = dst.offset(1);
                            *fresh20 = '0' as libc::c_char;
                            let fresh21 = dst;
                            dst = dst.offset(1);
                            *fresh21 = '0' as libc::c_char
                        }
                    }
                }
                current_block = 18394067709589936313;
            }
            9 => {
                current_block = 13839326668931527616;
                match current_block {
                    8121314696114196120 => {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\\' as libc::c_char;
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = 'n' as libc::c_char
                    }
                    5892893968624351926 => {
                        let fresh16 = dst;
                        dst = dst.offset(1);
                        *fresh16 = '\\' as libc::c_char;
                        let fresh17 = dst;
                        dst = dst.offset(1);
                        *fresh17 = 's' as libc::c_char
                    }
                    3692292822783516354 => {
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = '\\' as libc::c_char;
                        let fresh15 = dst;
                        dst = dst.offset(1);
                        *fresh15 = 'f' as libc::c_char
                    }
                    13839326668931527616 => {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as libc::c_char;
                        let fresh13 = dst;
                        dst = dst.offset(1);
                        *fresh13 = 't' as libc::c_char
                    }
                    5310961604937924882 => {
                        let fresh10 = dst;
                        dst = dst.offset(1);
                        *fresh10 = '\\' as libc::c_char;
                        let fresh11 = dst;
                        dst = dst.offset(1);
                        *fresh11 = 'v' as libc::c_char
                    }
                    6153397765503504804 => {
                        let fresh8 = dst;
                        dst = dst.offset(1);
                        *fresh8 = '\\' as libc::c_char;
                        let fresh9 = dst;
                        dst = dst.offset(1);
                        *fresh9 = 'a' as libc::c_char
                    }
                    8337753555966048743 => {
                        let fresh6 = dst;
                        dst = dst.offset(1);
                        *fresh6 = '\\' as libc::c_char;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = 'b' as libc::c_char
                    }
                    2667368111120228261 => {
                        let fresh4 = dst;
                        dst = dst.offset(1);
                        *fresh4 = '\\' as libc::c_char;
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = 'r' as libc::c_char
                    }
                    _ => {
                        let fresh18 = dst;
                        dst = dst.offset(1);
                        *fresh18 = '\\' as libc::c_char;
                        let fresh19 = dst;
                        dst = dst.offset(1);
                        *fresh19 = '0' as libc::c_char;
                        if nextc as u_char as libc::c_int >= '0' as i32
                            && nextc as u_char as libc::c_int <= '7' as i32
                        {
                            let fresh20 = dst;
                            dst = dst.offset(1);
                            *fresh20 = '0' as libc::c_char;
                            let fresh21 = dst;
                            dst = dst.offset(1);
                            *fresh21 = '0' as libc::c_char
                        }
                    }
                }
                current_block = 18394067709589936313;
            }
            12 => {
                current_block = 3692292822783516354;
                match current_block {
                    8121314696114196120 => {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\\' as libc::c_char;
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = 'n' as libc::c_char
                    }
                    5892893968624351926 => {
                        let fresh16 = dst;
                        dst = dst.offset(1);
                        *fresh16 = '\\' as libc::c_char;
                        let fresh17 = dst;
                        dst = dst.offset(1);
                        *fresh17 = 's' as libc::c_char
                    }
                    3692292822783516354 => {
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = '\\' as libc::c_char;
                        let fresh15 = dst;
                        dst = dst.offset(1);
                        *fresh15 = 'f' as libc::c_char
                    }
                    13839326668931527616 => {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as libc::c_char;
                        let fresh13 = dst;
                        dst = dst.offset(1);
                        *fresh13 = 't' as libc::c_char
                    }
                    5310961604937924882 => {
                        let fresh10 = dst;
                        dst = dst.offset(1);
                        *fresh10 = '\\' as libc::c_char;
                        let fresh11 = dst;
                        dst = dst.offset(1);
                        *fresh11 = 'v' as libc::c_char
                    }
                    6153397765503504804 => {
                        let fresh8 = dst;
                        dst = dst.offset(1);
                        *fresh8 = '\\' as libc::c_char;
                        let fresh9 = dst;
                        dst = dst.offset(1);
                        *fresh9 = 'a' as libc::c_char
                    }
                    8337753555966048743 => {
                        let fresh6 = dst;
                        dst = dst.offset(1);
                        *fresh6 = '\\' as libc::c_char;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = 'b' as libc::c_char
                    }
                    2667368111120228261 => {
                        let fresh4 = dst;
                        dst = dst.offset(1);
                        *fresh4 = '\\' as libc::c_char;
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = 'r' as libc::c_char
                    }
                    _ => {
                        let fresh18 = dst;
                        dst = dst.offset(1);
                        *fresh18 = '\\' as libc::c_char;
                        let fresh19 = dst;
                        dst = dst.offset(1);
                        *fresh19 = '0' as libc::c_char;
                        if nextc as u_char as libc::c_int >= '0' as i32
                            && nextc as u_char as libc::c_int <= '7' as i32
                        {
                            let fresh20 = dst;
                            dst = dst.offset(1);
                            *fresh20 = '0' as libc::c_char;
                            let fresh21 = dst;
                            dst = dst.offset(1);
                            *fresh21 = '0' as libc::c_char
                        }
                    }
                }
                current_block = 18394067709589936313;
            }
            32 => {
                current_block = 5892893968624351926;
                match current_block {
                    8121314696114196120 => {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\\' as libc::c_char;
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = 'n' as libc::c_char
                    }
                    5892893968624351926 => {
                        let fresh16 = dst;
                        dst = dst.offset(1);
                        *fresh16 = '\\' as libc::c_char;
                        let fresh17 = dst;
                        dst = dst.offset(1);
                        *fresh17 = 's' as libc::c_char
                    }
                    3692292822783516354 => {
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = '\\' as libc::c_char;
                        let fresh15 = dst;
                        dst = dst.offset(1);
                        *fresh15 = 'f' as libc::c_char
                    }
                    13839326668931527616 => {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as libc::c_char;
                        let fresh13 = dst;
                        dst = dst.offset(1);
                        *fresh13 = 't' as libc::c_char
                    }
                    5310961604937924882 => {
                        let fresh10 = dst;
                        dst = dst.offset(1);
                        *fresh10 = '\\' as libc::c_char;
                        let fresh11 = dst;
                        dst = dst.offset(1);
                        *fresh11 = 'v' as libc::c_char
                    }
                    6153397765503504804 => {
                        let fresh8 = dst;
                        dst = dst.offset(1);
                        *fresh8 = '\\' as libc::c_char;
                        let fresh9 = dst;
                        dst = dst.offset(1);
                        *fresh9 = 'a' as libc::c_char
                    }
                    8337753555966048743 => {
                        let fresh6 = dst;
                        dst = dst.offset(1);
                        *fresh6 = '\\' as libc::c_char;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = 'b' as libc::c_char
                    }
                    2667368111120228261 => {
                        let fresh4 = dst;
                        dst = dst.offset(1);
                        *fresh4 = '\\' as libc::c_char;
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = 'r' as libc::c_char
                    }
                    _ => {
                        let fresh18 = dst;
                        dst = dst.offset(1);
                        *fresh18 = '\\' as libc::c_char;
                        let fresh19 = dst;
                        dst = dst.offset(1);
                        *fresh19 = '0' as libc::c_char;
                        if nextc as u_char as libc::c_int >= '0' as i32
                            && nextc as u_char as libc::c_int <= '7' as i32
                        {
                            let fresh20 = dst;
                            dst = dst.offset(1);
                            *fresh20 = '0' as libc::c_char;
                            let fresh21 = dst;
                            dst = dst.offset(1);
                            *fresh21 = '0' as libc::c_char
                        }
                    }
                }
                current_block = 18394067709589936313;
            }
            0 => {
                current_block = 12992004776645797882;
                match current_block {
                    8121314696114196120 => {
                        let fresh2 = dst;
                        dst = dst.offset(1);
                        *fresh2 = '\\' as libc::c_char;
                        let fresh3 = dst;
                        dst = dst.offset(1);
                        *fresh3 = 'n' as libc::c_char
                    }
                    5892893968624351926 => {
                        let fresh16 = dst;
                        dst = dst.offset(1);
                        *fresh16 = '\\' as libc::c_char;
                        let fresh17 = dst;
                        dst = dst.offset(1);
                        *fresh17 = 's' as libc::c_char
                    }
                    3692292822783516354 => {
                        let fresh14 = dst;
                        dst = dst.offset(1);
                        *fresh14 = '\\' as libc::c_char;
                        let fresh15 = dst;
                        dst = dst.offset(1);
                        *fresh15 = 'f' as libc::c_char
                    }
                    13839326668931527616 => {
                        let fresh12 = dst;
                        dst = dst.offset(1);
                        *fresh12 = '\\' as libc::c_char;
                        let fresh13 = dst;
                        dst = dst.offset(1);
                        *fresh13 = 't' as libc::c_char
                    }
                    5310961604937924882 => {
                        let fresh10 = dst;
                        dst = dst.offset(1);
                        *fresh10 = '\\' as libc::c_char;
                        let fresh11 = dst;
                        dst = dst.offset(1);
                        *fresh11 = 'v' as libc::c_char
                    }
                    6153397765503504804 => {
                        let fresh8 = dst;
                        dst = dst.offset(1);
                        *fresh8 = '\\' as libc::c_char;
                        let fresh9 = dst;
                        dst = dst.offset(1);
                        *fresh9 = 'a' as libc::c_char
                    }
                    8337753555966048743 => {
                        let fresh6 = dst;
                        dst = dst.offset(1);
                        *fresh6 = '\\' as libc::c_char;
                        let fresh7 = dst;
                        dst = dst.offset(1);
                        *fresh7 = 'b' as libc::c_char
                    }
                    2667368111120228261 => {
                        let fresh4 = dst;
                        dst = dst.offset(1);
                        *fresh4 = '\\' as libc::c_char;
                        let fresh5 = dst;
                        dst = dst.offset(1);
                        *fresh5 = 'r' as libc::c_char
                    }
                    _ => {
                        let fresh18 = dst;
                        dst = dst.offset(1);
                        *fresh18 = '\\' as libc::c_char;
                        let fresh19 = dst;
                        dst = dst.offset(1);
                        *fresh19 = '0' as libc::c_char;
                        if nextc as u_char as libc::c_int >= '0' as i32
                            && nextc as u_char as libc::c_int <= '7' as i32
                        {
                            let fresh20 = dst;
                            dst = dst.offset(1);
                            *fresh20 = '0' as libc::c_char;
                            let fresh21 = dst;
                            dst = dst.offset(1);
                            *fresh21 = '0' as libc::c_char
                        }
                    }
                }
                current_block = 18394067709589936313;
            }
            _ => {
                current_block = 1118134448028020070;
            }
        }
    } else {
        current_block = 1118134448028020070;
    }
    match current_block {
        1118134448028020070 => {
            if c & 0o177i32 == ' ' as i32
                || flag & 0x1i32 != 0
                || flag & 0x100i32 != 0
                    && (c == '*' as i32 || c == '?' as i32 || c == '[' as i32 || c == '#' as i32)
            {
                let fresh22 = dst;
                dst = dst.offset(1);
                *fresh22 = '\\' as libc::c_char;
                let fresh23 = dst;
                dst = dst.offset(1);
                *fresh23 =
                    ((c as u_char as libc::c_int >> 6i32 & 0o7i32) + '0' as i32) as libc::c_char;
                let fresh24 = dst;
                dst = dst.offset(1);
                *fresh24 =
                    ((c as u_char as libc::c_int >> 3i32 & 0o7i32) + '0' as i32) as libc::c_char;
                let fresh25 = dst;
                dst = dst.offset(1);
                *fresh25 = ((c as u_char as libc::c_int & 0o7i32) + '0' as i32) as libc::c_char
            } else {
                if flag & 0x40i32 == 0i32 {
                    let fresh26 = dst;
                    dst = dst.offset(1);
                    *fresh26 = '\\' as libc::c_char
                }
                if c & 0o200i32 != 0 {
                    c &= 0o177i32;
                    let fresh27 = dst;
                    dst = dst.offset(1);
                    *fresh27 = 'M' as libc::c_char
                }
                if *(*__ctype_b_loc()).offset(c as u_char as libc::c_int as isize) as libc::c_int
                    & _IScntrl as libc::c_ushort as libc::c_int
                    != 0
                {
                    let fresh28 = dst;
                    dst = dst.offset(1);
                    *fresh28 = '^' as libc::c_char;
                    if c == 0o177i32 {
                        let fresh29 = dst;
                        dst = dst.offset(1);
                        *fresh29 = '?' as libc::c_char
                    } else {
                        let fresh30 = dst;
                        dst = dst.offset(1);
                        *fresh30 = (c + '@' as i32) as libc::c_char
                    }
                } else {
                    let fresh31 = dst;
                    dst = dst.offset(1);
                    *fresh31 = '-' as libc::c_char;
                    let fresh32 = dst;
                    dst = dst.offset(1);
                    *fresh32 = c as libc::c_char
                }
            }
        }
        _ => {}
    }
    *dst = '\u{0}' as libc::c_char;
    return dst;
}
/*
 * strvis, strnvis, strvisx - visually encode characters from src into dst
 *
 *	Dst must be 4 times the size of src to account for possible
 *	expansion.  The length of dst, not including the trailing NULL,
 *	is returned.
 *
 *	Strnvis will write no more than siz-1 bytes (and will NULL terminate).
 *	The number of bytes needed to fully encode the string is returned.
 *
 *	Strvisx encodes exactly len bytes from src into dst.
 *	This is useful for encoding a block of data.
 */
#[no_mangle]
pub unsafe extern "C" fn strvis(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    start = dst;
    loop {
        c = *src;
        if !(c != 0) {
            break;
        }
        src = src.offset(1);
        dst = vis(dst, c as libc::c_int, flag, *src as libc::c_int)
    }
    *dst = '\u{0}' as libc::c_char;
    return dst.wrapping_offset_from(start) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn strnvis(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut siz: size_t,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tbuf: [libc::c_char; 5] = [0; 5];
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = 0i32;
    start = dst;
    end = start.offset(siz as isize).offset(-(1isize));
    loop {
        c = *src as libc::c_int;
        if !(c != 0 && dst < end) {
            break;
        }
        if (c == '\\' as i32 || flag & 0x400i32 == 0i32)
            && (c as u_int <= (127i32 * 2i32 + 1i32) as libc::c_uint
                && c as u_char as libc::c_int & !(0x7fi32) == 0i32
                && (c != '*' as i32 && c != '?' as i32 && c != '[' as i32 && c != '#' as i32
                    || flag & 0x100i32 == 0i32)
                && *(*__ctype_b_loc()).offset(c as u_char as libc::c_int as isize) as libc::c_int
                    & _ISgraph as libc::c_ushort as libc::c_int
                    != 0
                || flag & 0x4i32 == 0i32 && c == ' ' as i32
                || flag & 0x8i32 == 0i32 && c == '\t' as i32
                || flag & 0x10i32 == 0i32 && c == '\n' as i32
                || flag & 0x20i32 != 0
                    && (c == '\u{8}' as i32
                        || c == '\u{7}' as i32
                        || c == '\r' as i32
                        || *(*__ctype_b_loc()).offset(c as u_char as libc::c_int as isize)
                            as libc::c_int
                            & _ISgraph as libc::c_ushort as libc::c_int
                            != 0))
        {
            if c == '\"' as i32 && flag & 0x200i32 != 0i32
                || c == '\\' as i32 && flag & 0x40i32 == 0i32
            {
                /* need space for the extra '\\' */
                if dst.offset(1isize) >= end {
                    i = 2i32;
                    break;
                } else {
                    let fresh33 = dst;
                    dst = dst.offset(1);
                    *fresh33 = '\\' as libc::c_char
                }
            }
            i = 1i32;
            let fresh34 = dst;
            dst = dst.offset(1);
            *fresh34 = c as libc::c_char;
            src = src.offset(1)
        } else {
            src = src.offset(1);
            i = vis(tbuf.as_mut_ptr(), c, flag, *src as libc::c_int)
                .wrapping_offset_from(tbuf.as_mut_ptr()) as libc::c_int;
            if dst.offset(i as isize) <= end {
                memcpy(
                    dst as *mut libc::c_void,
                    tbuf.as_mut_ptr() as *const libc::c_void,
                    i as libc::c_ulong,
                );
                dst = dst.offset(i as isize)
            } else {
                src = src.offset(-1);
                break;
            }
        }
    }
    if siz > 0u64 {
        *dst = '\u{0}' as libc::c_char
    }
    if dst.offset(i as isize) > end {
        loop
        /* adjust return value for truncation */
        {
            c = *src as libc::c_int;
            if !(c != 0) {
                break;
            }
            src = src.offset(1);
            dst = dst.offset(
                vis(tbuf.as_mut_ptr(), c, flag, *src as libc::c_int)
                    .wrapping_offset_from(tbuf.as_mut_ptr()),
            )
        }
    }
    return dst.wrapping_offset_from(start) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stravis(
    mut outp: *mut *mut libc::c_char,
    mut src: *const libc::c_char,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut serrno: libc::c_int = 0;
    buf = calloc(4u64, strlen(src).wrapping_add(1u64)) as *mut libc::c_char;
    if buf.is_null() {
        return -(1i32);
    }
    len = strvis(buf, src, flag);
    serrno = *__errno_location();
    *outp = realloc(buf as *mut libc::c_void, (len + 1i32) as libc::c_ulong) as *mut libc::c_char;
    if (*outp).is_null() {
        *outp = buf;
        *__errno_location() = serrno
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn strvisx(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut len: size_t,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    start = dst;
    while len > 1u64 {
        c = *src;
        src = src.offset(1);
        dst = vis(dst, c as libc::c_int, flag, *src as libc::c_int);
        len = len.wrapping_sub(1)
    }
    if len != 0 {
        dst = vis(dst, *src as libc::c_int, flag, '\u{0}' as i32)
    }
    *dst = '\u{0}' as libc::c_char;
    return dst.wrapping_offset_from(start) as libc::c_int;
}
