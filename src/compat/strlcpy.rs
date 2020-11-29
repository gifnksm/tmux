use ::libc;
pub type size_t = libc::c_ulong;
/*	$OpenBSD: strlcpy.c,v 1.10 2005/08/08 08:05:37 espie Exp $	*/
/*
 * Copyright (c) 1998 Todd C. Miller <Todd.Miller@courtesan.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Copy src to string dst of size siz.  At most siz-1 characters
 * will be copied.  Always NUL terminates (unless siz == 0).
 * Returns strlen(src); if retval >= siz, truncation occurred.
 */
#[no_mangle]
pub unsafe extern "C" fn strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut siz: size_t,
) -> libc::c_ulong {
    let mut d: *mut libc::c_char = dst;
    let mut s: *const libc::c_char = src;
    let mut n: size_t = siz;
    /* Copy as many bytes as will fit */
    if n != 0u64 && {
        n = n.wrapping_sub(1);
        (n) != 0u64
    } {
        loop {
            let fresh0 = s;
            s = s.offset(1);
            let fresh1 = d;
            d = d.offset(1);
            *fresh1 = *fresh0;
            if *fresh1 as libc::c_int == 0i32 {
                break;
            }
            n = n.wrapping_sub(1);
            if !(n != 0u64) {
                break;
            }
        }
    }
    /* Not enough room in dst, add NUL and traverse rest of src */
    if n == 0u64 {
        if siz != 0u64 {
            *d = '\u{0}' as libc::c_char
        } /* NUL-terminate dst */
        loop {
            let fresh2 = s;
            s = s.offset(1);
            if !(*fresh2 != 0) {
                break;
            }
        }
    }
    return (s.wrapping_offset_from(src) as libc::c_long - 1i64) as libc::c_ulong;
    /* count does not include NUL */
}
