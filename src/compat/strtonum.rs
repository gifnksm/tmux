use ::libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct errval {
    pub errstr: *const libc::c_char,
    pub err: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn strtonum(
    mut numstr: *const libc::c_char,
    mut minval: libc::c_longlong,
    mut maxval: libc::c_longlong,
    mut errstrp: *mut *const libc::c_char,
) -> libc::c_longlong {
    let mut ll: libc::c_longlong = 0i64;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error: libc::c_int = 0i32;
    let mut ev: [errval; 4] = [
        {
            let mut init = errval {
                errstr: 0 as *const libc::c_char,
                err: 0i32,
            };
            init
        },
        {
            let mut init = errval {
                errstr: b"invalid\x00" as *const u8 as *const libc::c_char,
                err: 22i32,
            };
            init
        },
        {
            let mut init = errval {
                errstr: b"too small\x00" as *const u8 as *const libc::c_char,
                err: 34i32,
            };
            init
        },
        {
            let mut init = errval {
                errstr: b"too large\x00" as *const u8 as *const libc::c_char,
                err: 34i32,
            };
            init
        },
    ];
    ev[0usize].err = *__errno_location();
    *__errno_location() = 0i32;
    if minval > maxval {
        error = 1i32
    } else {
        ll = strtoll(numstr, &mut ep, 10i32);
        if numstr == ep as *const libc::c_char || *ep as libc::c_int != '\u{0}' as i32 {
            error = 1i32
        } else if ll == -(9223372036854775807i64) - 1i64 && *__errno_location() == 34i32
            || ll < minval
        {
            error = 2i32
        } else if ll == 9223372036854775807i64 && *__errno_location() == 34i32 || ll > maxval {
            error = 3i32
        }
    }
    if !errstrp.is_null() {
        *errstrp = ev[error as usize].errstr
    }
    *__errno_location() = ev[error as usize].err;
    if error != 0 {
        ll = 0i64
    }
    return ll;
}
