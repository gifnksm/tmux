use crate::{ffi, session::Session};
use std::{ffi::c_void, os::raw::c_int};

pub(crate) use ffi::client as Client;

impl Client {
    pub(crate) fn session(&self) -> Option<&Session> {
        unsafe { self.session.as_ref() }
    }

    pub(crate) fn each_clients(f: impl FnMut(&mut Self) -> bool) {
        // FIXME: workaround to satisfy borrow checker
        let mut ctx = EachClientsCtx {
            f: Box::new(f) as _,
        };
        unsafe {
            ffi::glue_clients_foreach_safe(
                &mut ffi::clients,
                &mut ctx as *mut _ as _,
                Some(each_clients),
            );
        }
    }
}

struct EachClientsCtx<'a> {
    f: Box<dyn FnMut(&mut Client) -> bool + 'a>,
}

extern "C" fn each_clients(client: *mut Client, ctx: *mut c_void) -> c_int {
    let session = unsafe { client.as_mut().unwrap() };
    let ctx = unsafe { (ctx as *mut EachClientsCtx).as_mut().unwrap() };
    (ctx.f)(session) as _
}
