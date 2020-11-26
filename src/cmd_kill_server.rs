use ::libc;
extern "C" {
    pub type cmd;
    pub type cmdq_item;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn cmd_get_entry(_: *mut cmd) -> *const cmd_entry;
}
pub type __pid_t = libc::c_int;
pub type cmd_find_type = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type cmd_retval = libc::c_int;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: C2RustUnnamed,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item) -> cmd_retval>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[no_mangle]
pub static mut cmd_kill_server_entry: cmd_entry = unsafe {
    {
        let mut init = cmd_entry {
            name: b"kill-server\x00" as *const u8 as *const libc::c_char,
            alias: 0 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed {
                    template: b"\x00" as *const u8 as *const libc::c_char,
                    lower: 0 as libc::c_int,
                    upper: 0 as libc::c_int,
                };
                init
            },
            usage: b"\x00" as *const u8 as *const libc::c_char,
            source: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            target: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            flags: 0 as libc::c_int,
            exec: Some(
                cmd_kill_server_exec
                    as unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item) -> cmd_retval,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut cmd_start_server_entry: cmd_entry = unsafe {
    {
        let mut init = cmd_entry {
            name: b"start-server\x00" as *const u8 as *const libc::c_char,
            alias: b"start\x00" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed {
                    template: b"\x00" as *const u8 as *const libc::c_char,
                    lower: 0 as libc::c_int,
                    upper: 0 as libc::c_int,
                };
                init
            },
            usage: b"\x00" as *const u8 as *const libc::c_char,
            source: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            target: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            flags: 0x1 as libc::c_int,
            exec: Some(
                cmd_kill_server_exec
                    as unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item) -> cmd_retval,
            ),
        };
        init
    }
};
/* $OpenBSD$ */
/*
 * Copyright (c) 2007 Nicholas Marriott <nicholas.marriott@gmail.com>
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
/*
 * Kill the server and do nothing else.
 */
unsafe extern "C" fn cmd_kill_server_exec(
    mut self_0: *mut cmd,
    mut _item: *mut cmdq_item,
) -> cmd_retval {
    if cmd_get_entry(self_0) == &cmd_kill_server_entry as *const cmd_entry {
        kill(getpid(), 15 as libc::c_int);
    }
    return CMD_RETURN_NORMAL;
}
