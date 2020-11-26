use ::libc;
extern "C" {
    #[no_mangle]
    fn sendmsg(__fd: libc::c_int, __message: *const msghdr, __flags: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn freezero(_: *mut libc::c_void, _: size_t);
    #[no_mangle]
    fn recallocarray(_: *mut libc::c_void, _: size_t, _: size_t, _: size_t) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __caddr_t = *mut libc::c_char;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type caddr_t = __caddr_t;
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}
pub type C2RustUnnamed = libc::c_uint;
pub const SCM_CREDENTIALS: C2RustUnnamed = 2;
pub const SCM_RIGHTS: C2RustUnnamed = 1;
pub type uint32_t = __uint32_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibuf {
    pub entry: C2RustUnnamed_0,
    pub buf: *mut libc::c_uchar,
    pub size: size_t,
    pub max: size_t,
    pub wpos: size_t,
    pub rpos: size_t,
    pub fd: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut ibuf,
    pub tqe_prev: *mut *mut ibuf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgbuf {
    pub bufs: C2RustUnnamed_1,
    pub queued: uint32_t,
    pub fd: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub tqh_first: *mut ibuf,
    pub tqh_last: *mut *mut ibuf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_2 {
    pub hdr: cmsghdr,
    pub buf: [libc::c_char; 24],
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_open(mut len: size_t) -> *mut ibuf {
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    buf = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<ibuf>() as libc::c_ulong,
    ) as *mut ibuf;
    if buf.is_null() {
        return 0 as *mut ibuf;
    }
    (*buf).buf = malloc(len) as *mut libc::c_uchar;
    if (*buf).buf.is_null() {
        free(buf as *mut libc::c_void);
        return 0 as *mut ibuf;
    }
    (*buf).max = len;
    (*buf).size = (*buf).max;
    (*buf).fd = -(1 as libc::c_int);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_dynamic(mut len: size_t, mut max: size_t) -> *mut ibuf {
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    if max < len {
        return 0 as *mut ibuf;
    }
    buf = ibuf_open(len);
    if buf.is_null() {
        return 0 as *mut ibuf;
    }
    if max > 0 as libc::c_int as libc::c_ulong {
        (*buf).max = max
    }
    return buf;
}
/*	$OpenBSD: imsg-buffer.c,v 1.12 2019/01/20 02:50:03 bcook Exp $	*/
/*
 * Copyright (c) 2003, 2004 Henning Brauer <henning@openbsd.org>
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
unsafe extern "C" fn ibuf_realloc(mut buf: *mut ibuf, mut len: size_t) -> libc::c_int {
    let mut b: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* on static buffers max is eq size and so the following fails */
    if (*buf).wpos.wrapping_add(len) > (*buf).max {
        *__errno_location() = 34 as libc::c_int;
        return -(1 as libc::c_int);
    }
    b = recallocarray(
        (*buf).buf as *mut libc::c_void,
        (*buf).size,
        (*buf).wpos.wrapping_add(len),
        1 as libc::c_int as size_t,
    ) as *mut libc::c_uchar;
    if b.is_null() {
        return -(1 as libc::c_int);
    }
    (*buf).buf = b;
    (*buf).size = (*buf).wpos.wrapping_add(len);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_add(
    mut buf: *mut ibuf,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    if (*buf).wpos.wrapping_add(len) > (*buf).size {
        if ibuf_realloc(buf, len) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    memcpy(
        (*buf).buf.offset((*buf).wpos as isize) as *mut libc::c_void,
        data,
        len,
    );
    (*buf).wpos = ((*buf).wpos as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_reserve(mut buf: *mut ibuf, mut len: size_t) -> *mut libc::c_void {
    let mut b: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*buf).wpos.wrapping_add(len) > (*buf).size {
        if ibuf_realloc(buf, len) == -(1 as libc::c_int) {
            return 0 as *mut libc::c_void;
        }
    }
    b = (*buf).buf.offset((*buf).wpos as isize) as *mut libc::c_void;
    (*buf).wpos = ((*buf).wpos as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_seek(
    mut buf: *mut ibuf,
    mut pos: size_t,
    mut len: size_t,
) -> *mut libc::c_void {
    /* only allowed to seek in already written parts */
    if pos.wrapping_add(len) > (*buf).wpos {
        return 0 as *mut libc::c_void;
    }
    return (*buf).buf.offset(pos as isize) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_size(mut buf: *mut ibuf) -> size_t {
    return (*buf).wpos;
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_left(mut buf: *mut ibuf) -> size_t {
    return (*buf).max.wrapping_sub((*buf).wpos);
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_close(mut msgbuf: *mut msgbuf, mut buf: *mut ibuf) {
    ibuf_enqueue(msgbuf, buf);
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_write(mut msgbuf: *mut msgbuf) -> libc::c_int {
    let mut iov: [iovec; 1024] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 1024];
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut n: ssize_t = 0;
    memset(
        &mut iov as *mut [iovec; 1024] as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[iovec; 1024]>() as libc::c_ulong,
    );
    buf = (*msgbuf).bufs.tqh_first;
    while !buf.is_null() {
        if i >= 1024 as libc::c_int as libc::c_uint {
            break;
        }
        iov[i as usize].iov_base = (*buf).buf.offset((*buf).rpos as isize) as *mut libc::c_void;
        iov[i as usize].iov_len = (*buf).wpos.wrapping_sub((*buf).rpos);
        i = i.wrapping_add(1);
        buf = (*buf).entry.tqe_next
    }
    's_76: {
        loop {
            n = writev((*msgbuf).fd, iov.as_mut_ptr(), i as libc::c_int);
            if n == -(1 as libc::c_int) as libc::c_long {
                if *__errno_location() == 4 as libc::c_int {
                    continue;
                }
                if *__errno_location() == 105 as libc::c_int {
                    *__errno_location() = 11 as libc::c_int
                }
                return -(1 as libc::c_int);
            } else {
                break 's_76;
            }
        }
    }
    if n == 0 as libc::c_int as libc::c_long {
        /* connection closed */
        *__errno_location() = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    msgbuf_drain(msgbuf, n as size_t);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ibuf_free(mut buf: *mut ibuf) {
    if buf.is_null() {
        return;
    }
    freezero((*buf).buf as *mut libc::c_void, (*buf).size);
    free(buf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn msgbuf_init(mut msgbuf: *mut msgbuf) {
    (*msgbuf).queued = 0 as libc::c_int as uint32_t;
    (*msgbuf).fd = -(1 as libc::c_int);
    (*msgbuf).bufs.tqh_first = 0 as *mut ibuf;
    (*msgbuf).bufs.tqh_last = &mut (*msgbuf).bufs.tqh_first;
}
#[no_mangle]
pub unsafe extern "C" fn msgbuf_drain(mut msgbuf: *mut msgbuf, mut n: size_t) {
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    let mut next: *mut ibuf = 0 as *mut ibuf;
    buf = (*msgbuf).bufs.tqh_first;
    while !buf.is_null() && n > 0 as libc::c_int as libc::c_ulong {
        next = (*buf).entry.tqe_next;
        if (*buf).rpos.wrapping_add(n) >= (*buf).wpos {
            n = (n as libc::c_ulong).wrapping_sub((*buf).wpos.wrapping_sub((*buf).rpos)) as size_t
                as size_t;
            ibuf_dequeue(msgbuf, buf);
        } else {
            (*buf).rpos = ((*buf).rpos as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            n = 0 as libc::c_int as size_t
        }
        buf = next
    }
}
#[no_mangle]
pub unsafe extern "C" fn msgbuf_clear(mut msgbuf: *mut msgbuf) {
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    loop {
        buf = (*msgbuf).bufs.tqh_first;
        if buf.is_null() {
            break;
        }
        ibuf_dequeue(msgbuf, buf);
    }
}
#[no_mangle]
pub unsafe extern "C" fn msgbuf_write(mut msgbuf: *mut msgbuf) -> libc::c_int {
    let mut iov: [iovec; 1024] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 1024];
    let mut buf: *mut ibuf = 0 as *mut ibuf;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut n: ssize_t = 0;
    let mut msg: msghdr = msghdr {
        msg_name: 0 as *mut libc::c_void,
        msg_namelen: 0,
        msg_iov: 0 as *mut iovec,
        msg_iovlen: 0,
        msg_control: 0 as *mut libc::c_void,
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut cmsg: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut cmsgbuf: C2RustUnnamed_2 = C2RustUnnamed_2 {
        hdr: cmsghdr {
            cmsg_len: 0,
            cmsg_level: 0,
            cmsg_type: 0,
            __cmsg_data: [],
        },
    };
    memset(
        &mut iov as *mut [iovec; 1024] as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[iovec; 1024]>() as libc::c_ulong,
    );
    memset(
        &mut msg as *mut msghdr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<msghdr>() as libc::c_ulong,
    );
    memset(
        &mut cmsgbuf as *mut C2RustUnnamed_2 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong,
    );
    buf = (*msgbuf).bufs.tqh_first;
    while !buf.is_null() {
        if i >= 1024 as libc::c_int as libc::c_uint {
            break;
        }
        iov[i as usize].iov_base = (*buf).buf.offset((*buf).rpos as isize) as *mut libc::c_void;
        iov[i as usize].iov_len = (*buf).wpos.wrapping_sub((*buf).rpos);
        i = i.wrapping_add(1);
        if (*buf).fd != -(1 as libc::c_int) {
            break;
        }
        buf = (*buf).entry.tqe_next
    }
    msg.msg_iov = iov.as_mut_ptr();
    msg.msg_iovlen = i as size_t;
    if !buf.is_null() && (*buf).fd != -(1 as libc::c_int) {
        msg.msg_control =
            &mut cmsgbuf.buf as *mut [libc::c_char; 24] as caddr_t as *mut libc::c_void;
        msg.msg_controllen = ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong;
        cmsg = if msg.msg_controllen >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
            msg.msg_control as *mut cmsghdr
        } else {
            0 as *mut cmsghdr
        };
        (*cmsg).cmsg_len = ((::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
        (*cmsg).cmsg_level = 1 as libc::c_int;
        (*cmsg).cmsg_type = SCM_RIGHTS as libc::c_int;
        *((*cmsg).__cmsg_data.as_mut_ptr() as *mut libc::c_int) = (*buf).fd
    }
    's_145: {
        loop {
            n = sendmsg((*msgbuf).fd, &mut msg, 0 as libc::c_int);
            if n == -(1 as libc::c_int) as libc::c_long {
                if *__errno_location() == 4 as libc::c_int {
                    continue;
                }
                if *__errno_location() == 105 as libc::c_int {
                    *__errno_location() = 11 as libc::c_int
                }
                return -(1 as libc::c_int);
            } else {
                break 's_145;
            }
        }
    }
    if n == 0 as libc::c_int as libc::c_long {
        /* connection closed */
        *__errno_location() = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    /*
     * assumption: fd got sent if sendmsg sent anything
     * this works because fds are passed one at a time
     */
    if !buf.is_null() && (*buf).fd != -(1 as libc::c_int) {
        close((*buf).fd);
        (*buf).fd = -(1 as libc::c_int)
    }
    msgbuf_drain(msgbuf, n as size_t);
    return 1 as libc::c_int;
}
unsafe extern "C" fn ibuf_enqueue(mut msgbuf: *mut msgbuf, mut buf: *mut ibuf) {
    (*buf).entry.tqe_next = 0 as *mut ibuf;
    (*buf).entry.tqe_prev = (*msgbuf).bufs.tqh_last;
    *(*msgbuf).bufs.tqh_last = buf;
    (*msgbuf).bufs.tqh_last = &mut (*buf).entry.tqe_next;
    (*msgbuf).queued = (*msgbuf).queued.wrapping_add(1);
}
unsafe extern "C" fn ibuf_dequeue(mut msgbuf: *mut msgbuf, mut buf: *mut ibuf) {
    if !(*buf).entry.tqe_next.is_null() {
        (*(*buf).entry.tqe_next).entry.tqe_prev = (*buf).entry.tqe_prev
    } else {
        (*msgbuf).bufs.tqh_last = (*buf).entry.tqe_prev
    }
    *(*buf).entry.tqe_prev = (*buf).entry.tqe_next;
    if (*buf).fd != -(1 as libc::c_int) {
        close((*buf).fd);
    }
    (*msgbuf).queued = (*msgbuf).queued.wrapping_sub(1);
    ibuf_free(buf);
}
