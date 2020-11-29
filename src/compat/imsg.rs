use ::libc;
extern "C" {
    #[no_mangle]
    fn recvmsg(__fd: libc::c_int, __message: *mut msghdr, __flags: libc::c_int) -> ssize_t;
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
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn getdtablesize() -> libc::c_int;
    #[no_mangle]
    fn freezero(_: *mut libc::c_void, _: size_t);
    #[no_mangle]
    fn getdtablecount() -> libc::c_int;
    #[no_mangle]
    fn msgbuf_write(_: *mut msgbuf) -> libc::c_int;
    #[no_mangle]
    fn msgbuf_clear(_: *mut msgbuf);
    #[no_mangle]
    fn msgbuf_init(_: *mut msgbuf);
    #[no_mangle]
    fn ibuf_free(_: *mut ibuf);
    #[no_mangle]
    fn ibuf_close(_: *mut msgbuf, _: *mut ibuf);
    #[no_mangle]
    fn ibuf_add(_: *mut ibuf, _: *const libc::c_void, _: size_t) -> libc::c_int;
    #[no_mangle]
    fn ibuf_dynamic(_: size_t, _: size_t) -> *mut ibuf;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
pub type uint16_t = __uint16_t;
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
pub struct ibuf_read {
    pub buf: [libc::c_uchar; 65535],
    pub rptr: *mut libc::c_uchar,
    pub wpos: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imsg_fd {
    pub entry: C2RustUnnamed_2,
    pub fd: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut imsg_fd,
    pub tqe_prev: *mut *mut imsg_fd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imsgbuf {
    pub fds: C2RustUnnamed_3,
    pub r: ibuf_read,
    pub w: msgbuf,
    pub fd: libc::c_int,
    pub pid: pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub tqh_first: *mut imsg_fd,
    pub tqh_last: *mut *mut imsg_fd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imsg_hdr {
    pub type_0: uint32_t,
    pub len: uint16_t,
    pub flags: uint16_t,
    pub peerid: uint32_t,
    pub pid: uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imsg {
    pub hdr: imsg_hdr,
    pub fd: libc::c_int,
    pub data: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_4 {
    pub hdr: cmsghdr,
    pub buf: [libc::c_char; 24],
}
#[inline]
unsafe extern "C" fn __cmsg_nxthdr(
    mut __mhdr: *mut msghdr,
    mut __cmsg: *mut cmsghdr,
) -> *mut cmsghdr {
    if (*__cmsg).cmsg_len < ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
        return 0 as *mut cmsghdr;
    }
    __cmsg = (__cmsg as *mut libc::c_uchar).offset(
        ((*__cmsg)
            .cmsg_len
            .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
            .wrapping_sub(1u64)
            & !(::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_sub(1u64))
            as isize,
    ) as *mut cmsghdr;
    if __cmsg.offset(1isize) as *mut libc::c_uchar
        > ((*__mhdr).msg_control as *mut libc::c_uchar).offset((*__mhdr).msg_controllen as isize)
        || (__cmsg as *mut libc::c_uchar).offset(
            ((*__cmsg)
                .cmsg_len
                .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1u64)
                & !(::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_sub(1u64))
                as isize,
        ) > ((*__mhdr).msg_control as *mut libc::c_uchar)
            .offset((*__mhdr).msg_controllen as isize)
    {
        return 0 as *mut cmsghdr;
    }
    return __cmsg;
}
/*	$OpenBSD: imsg.c,v 1.16 2017/12/14 09:27:44 kettenis Exp $	*/
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
#[no_mangle]
pub static mut imsg_fd_overhead: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn imsg_init(mut ibuf: *mut imsgbuf, mut fd: libc::c_int) {
    msgbuf_init(&mut (*ibuf).w);
    memset(
        &mut (*ibuf).r as *mut ibuf_read as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<ibuf_read>() as libc::c_ulong,
    );
    (*ibuf).fd = fd;
    (*ibuf).w.fd = fd;
    (*ibuf).pid = getpid();
    (*ibuf).fds.tqh_first = 0 as *mut imsg_fd;
    (*ibuf).fds.tqh_last = &mut (*ibuf).fds.tqh_first;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_read(mut ibuf: *mut imsgbuf) -> ssize_t {
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
    let mut cmsgbuf: C2RustUnnamed_4 = C2RustUnnamed_4 {
        hdr: cmsghdr {
            cmsg_len: 0,
            cmsg_level: 0,
            cmsg_type: 0,
            __cmsg_data: [],
        },
    };
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    let mut n: ssize_t = -1i64;
    let mut fd: libc::c_int = 0;
    let mut ifd: *mut imsg_fd = 0 as *mut imsg_fd;
    memset(
        &mut msg as *mut msghdr as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<msghdr>() as libc::c_ulong,
    );
    memset(
        &mut cmsgbuf as *mut C2RustUnnamed_4 as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong,
    );
    iov.iov_base = (*ibuf).r.buf.as_mut_ptr().offset((*ibuf).r.wpos as isize) as *mut libc::c_void;
    iov.iov_len = (::std::mem::size_of::<[libc::c_uchar; 65535]>() as libc::c_ulong)
        .wrapping_sub((*ibuf).r.wpos);
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1u64;
    msg.msg_control = &mut cmsgbuf.buf as *mut [libc::c_char; 24] as *mut libc::c_void;
    msg.msg_controllen = ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong;
    ifd = calloc(1u64, ::std::mem::size_of::<imsg_fd>() as libc::c_ulong) as *mut imsg_fd;
    if ifd.is_null() {
        return -1i64;
    }
    loop {
        if getdtablecount()
            + imsg_fd_overhead
            + ((::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1u64)
                & !(::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_sub(1u64))
            .wrapping_add(
                (::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(1u64)
                    & !(::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_sub(1u64),
            )
            .wrapping_sub(
                ((0u64)
                    .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(1u64)
                    & !(::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_sub(1u64))
                .wrapping_add(
                    (::std::mem::size_of::<cmsghdr>() as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_sub(1u64)
                        & !(::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_sub(1u64),
                ),
            )
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int
            >= getdtablesize()
        {
            *__errno_location() = 11i32;
            free(ifd as *mut libc::c_void);
            return -1i64;
        }
        n = recvmsg((*ibuf).fd, &mut msg, 0i32);
        if n == -1i64 {
            if !(*__errno_location() == 4i32) {
                break;
            }
        } else {
            (*ibuf).r.wpos = ((*ibuf).r.wpos).wrapping_add(n as libc::c_ulong);
            cmsg = if msg.msg_controllen >= ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else {
                0 as *mut cmsghdr
            };
            while !cmsg.is_null() {
                if (*cmsg).cmsg_level == 1i32 && (*cmsg).cmsg_type == SCM_RIGHTS as libc::c_int {
                    let mut i: libc::c_int = 0;
                    let mut j: libc::c_int = 0;
                    /*
                     * We only accept one file descriptor.  Due to C
                     * padding rules, our control buffer might contain
                     * more than one fd, and we must close them.
                     */
                    j = ((cmsg as *mut libc::c_char)
                        .offset((*cmsg).cmsg_len as isize)
                        .wrapping_offset_from((*cmsg).__cmsg_data.as_mut_ptr() as *mut libc::c_char)
                        as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        as libc::c_int;
                    i = 0i32;
                    while i < j {
                        fd = *((*cmsg).__cmsg_data.as_mut_ptr() as *mut libc::c_int)
                            .offset(i as isize);
                        if !ifd.is_null() {
                            (*ifd).fd = fd;
                            (*ifd).entry.tqe_next = 0 as *mut imsg_fd;
                            (*ifd).entry.tqe_prev = (*ibuf).fds.tqh_last;
                            *(*ibuf).fds.tqh_last = ifd;
                            (*ibuf).fds.tqh_last = &mut (*ifd).entry.tqe_next;
                            ifd = 0 as *mut imsg_fd
                        } else {
                            close(fd);
                        }
                        i += 1
                    }
                }
                cmsg = __cmsg_nxthdr(&mut msg, cmsg)
                /* we do not handle other ctl data level */
            }
            break;
        }
    }
    free(ifd as *mut libc::c_void);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_get(mut ibuf: *mut imsgbuf, mut imsg: *mut imsg) -> ssize_t {
    let mut av: size_t = 0;
    let mut left: size_t = 0;
    let mut datalen: size_t = 0;
    av = (*ibuf).r.wpos;
    if ::std::mem::size_of::<imsg_hdr>() as libc::c_ulong > av {
        return 0i64;
    }
    memcpy(
        &mut (*imsg).hdr as *mut imsg_hdr as *mut libc::c_void,
        (*ibuf).r.buf.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<imsg_hdr>() as libc::c_ulong,
    );
    if ((*imsg).hdr.len as libc::c_ulong) < ::std::mem::size_of::<imsg_hdr>() as libc::c_ulong
        || (*imsg).hdr.len as libc::c_int > 16384i32
    {
        *__errno_location() = 34i32;
        return -1i64;
    }
    if (*imsg).hdr.len as libc::c_ulong > av {
        return 0i64;
    }
    datalen = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong);
    (*ibuf).r.rptr = (*ibuf)
        .r
        .buf
        .as_mut_ptr()
        .offset(::std::mem::size_of::<imsg_hdr>() as isize);
    if datalen == 0u64 {
        (*imsg).data = 0 as *mut libc::c_void
    } else {
        (*imsg).data = malloc(datalen);
        if (*imsg).data.is_null() {
            return -1i64;
        }
    }
    if (*imsg).hdr.flags as libc::c_int & 1i32 != 0 {
        (*imsg).fd = imsg_get_fd(ibuf)
    } else {
        (*imsg).fd = -(1i32)
    }
    memcpy((*imsg).data, (*ibuf).r.rptr as *const libc::c_void, datalen);
    if ((*imsg).hdr.len as libc::c_ulong) < av {
        left = av.wrapping_sub((*imsg).hdr.len as libc::c_ulong);
        memmove(
            &mut (*ibuf).r.buf as *mut [libc::c_uchar; 65535] as *mut libc::c_void,
            (*ibuf)
                .r
                .buf
                .as_mut_ptr()
                .offset((*imsg).hdr.len as libc::c_int as isize) as *const libc::c_void,
            left,
        );
        (*ibuf).r.wpos = left
    } else {
        (*ibuf).r.wpos = 0u64
    }
    return datalen.wrapping_add(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_compose(
    mut ibuf: *mut imsgbuf,
    mut type_0: uint32_t,
    mut peerid: uint32_t,
    mut pid: pid_t,
    mut fd: libc::c_int,
    mut data: *const libc::c_void,
    mut datalen: uint16_t,
) -> libc::c_int {
    let mut wbuf: *mut ibuf = 0 as *mut ibuf;
    wbuf = imsg_create(ibuf, type_0, peerid, pid, datalen);
    if wbuf.is_null() {
        return -(1i32);
    }
    if imsg_add(wbuf, data, datalen) == -(1i32) {
        return -(1i32);
    }
    (*wbuf).fd = fd;
    imsg_close(ibuf, wbuf);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_composev(
    mut ibuf: *mut imsgbuf,
    mut type_0: uint32_t,
    mut peerid: uint32_t,
    mut pid: pid_t,
    mut fd: libc::c_int,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
) -> libc::c_int {
    let mut wbuf: *mut ibuf = 0 as *mut ibuf;
    let mut i: libc::c_int = 0;
    let mut datalen: libc::c_int = 0i32;
    i = 0i32;
    while i < iovcnt {
        datalen = (datalen as libc::c_ulong).wrapping_add((*iov.offset(i as isize)).iov_len)
            as libc::c_int;
        i += 1
    }
    wbuf = imsg_create(ibuf, type_0, peerid, pid, datalen as uint16_t);
    if wbuf.is_null() {
        return -(1i32);
    }
    i = 0i32;
    while i < iovcnt {
        if imsg_add(
            wbuf,
            (*iov.offset(i as isize)).iov_base,
            (*iov.offset(i as isize)).iov_len as uint16_t,
        ) == -(1i32)
        {
            return -(1i32);
        }
        i += 1
    }
    (*wbuf).fd = fd;
    imsg_close(ibuf, wbuf);
    return 1i32;
}
/* ARGSUSED */
#[no_mangle]
pub unsafe extern "C" fn imsg_create(
    mut ibuf: *mut imsgbuf,
    mut type_0: uint32_t,
    mut peerid: uint32_t,
    mut pid: pid_t,
    mut datalen: uint16_t,
) -> *mut ibuf {
    let mut wbuf: *mut ibuf = 0 as *mut ibuf;
    let mut hdr: imsg_hdr = imsg_hdr {
        type_0: 0,
        len: 0,
        flags: 0,
        peerid: 0,
        pid: 0,
    };
    datalen = (datalen as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong) as uint16_t;
    if datalen as libc::c_int > 16384i32 {
        *__errno_location() = 34i32;
        return 0 as *mut ibuf;
    }
    hdr.type_0 = type_0;
    hdr.flags = 0u16;
    hdr.peerid = peerid;
    hdr.pid = pid as uint32_t;
    if hdr.pid == 0u32 {
        hdr.pid = (*ibuf).pid as uint32_t
    }
    wbuf = ibuf_dynamic(datalen as size_t, 16384u64);
    if wbuf.is_null() {
        return 0 as *mut ibuf;
    }
    if imsg_add(
        wbuf,
        &mut hdr as *mut imsg_hdr as *const libc::c_void,
        ::std::mem::size_of::<imsg_hdr>() as uint16_t,
    ) == -(1i32)
    {
        return 0 as *mut ibuf;
    }
    return wbuf;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_add(
    mut msg: *mut ibuf,
    mut data: *const libc::c_void,
    mut datalen: uint16_t,
) -> libc::c_int {
    if datalen != 0 {
        if ibuf_add(msg, data, datalen as size_t) == -(1i32) {
            ibuf_free(msg);
            return -(1i32);
        }
    }
    return datalen as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_close(mut ibuf: *mut imsgbuf, mut msg: *mut ibuf) {
    let mut hdr: *mut imsg_hdr = 0 as *mut imsg_hdr;
    hdr = (*msg).buf as *mut imsg_hdr;
    (*hdr).flags = ((*hdr).flags as libc::c_int & !(1i32)) as uint16_t;
    if (*msg).fd != -(1i32) {
        (*hdr).flags = ((*hdr).flags as libc::c_int | 1i32) as uint16_t
    }
    (*hdr).len = (*msg).wpos as uint16_t;
    ibuf_close(&mut (*ibuf).w, msg);
}
#[no_mangle]
pub unsafe extern "C" fn imsg_free(mut imsg: *mut imsg) {
    freezero(
        (*imsg).data,
        ((*imsg).hdr.len as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong),
    );
}
unsafe extern "C" fn imsg_get_fd(mut ibuf: *mut imsgbuf) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut ifd: *mut imsg_fd = 0 as *mut imsg_fd;
    ifd = (*ibuf).fds.tqh_first;
    if ifd.is_null() {
        return -(1i32);
    }
    fd = (*ifd).fd;
    if !(*ifd).entry.tqe_next.is_null() {
        (*(*ifd).entry.tqe_next).entry.tqe_prev = (*ifd).entry.tqe_prev
    } else {
        (*ibuf).fds.tqh_last = (*ifd).entry.tqe_prev
    }
    *(*ifd).entry.tqe_prev = (*ifd).entry.tqe_next;
    free(ifd as *mut libc::c_void);
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_flush(mut ibuf: *mut imsgbuf) -> libc::c_int {
    while (*ibuf).w.queued != 0 {
        if msgbuf_write(&mut (*ibuf).w) <= 0i32 {
            return -(1i32);
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_clear(mut ibuf: *mut imsgbuf) {
    let mut fd: libc::c_int = 0;
    msgbuf_clear(&mut (*ibuf).w);
    loop {
        fd = imsg_get_fd(ibuf);
        if !(fd != -(1i32)) {
            break;
        }
        close(fd);
    }
}
