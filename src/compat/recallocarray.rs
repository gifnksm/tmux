use ::libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    #[no_mangle]
    fn getpagesize() -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn recallocarray(
    mut ptr: *mut libc::c_void,
    mut oldnmemb: size_t,
    mut newnmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut oldsize: size_t = 0;
    let mut newsize: size_t = 0;
    let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if ptr.is_null() {
        return calloc(newnmemb, size);
    }
    if (newnmemb >= (1u64) << (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(4u64)
        || size >= (1u64) << (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(4u64))
        && newnmemb > 0u64
        && (18446744073709551615u64).wrapping_div(newnmemb) < size
    {
        *__errno_location() = 12i32;
        return 0 as *mut libc::c_void;
    }
    newsize = newnmemb.wrapping_mul(size);
    if (oldnmemb >= (1u64) << (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(4u64)
        || size >= (1u64) << (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(4u64))
        && oldnmemb > 0u64
        && (18446744073709551615u64).wrapping_div(oldnmemb) < size
    {
        *__errno_location() = 22i32;
        return 0 as *mut libc::c_void;
    }
    oldsize = oldnmemb.wrapping_mul(size);
    /*
     * Don't bother too much if we're shrinking just a bit,
     * we do not shrink for series of small steps, oh well.
     */
    if newsize <= oldsize {
        let mut d: size_t = oldsize.wrapping_sub(newsize);
        if d < oldsize.wrapping_div(2u64) && d < getpagesize() as size_t {
            memset(
                (ptr as *mut libc::c_char).offset(newsize as isize) as *mut libc::c_void,
                0i32,
                d,
            );
            return ptr;
        }
    }
    newptr = malloc(newsize);
    if newptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    if newsize > oldsize {
        memcpy(newptr, ptr, oldsize);
        memset(
            (newptr as *mut libc::c_char).offset(oldsize as isize) as *mut libc::c_void,
            0i32,
            newsize.wrapping_sub(oldsize),
        );
    } else {
        memcpy(newptr, ptr, newsize);
    }
    explicit_bzero(ptr, oldsize);
    free(ptr);
    return newptr;
}
