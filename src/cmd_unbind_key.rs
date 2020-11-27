use ::libc;
extern "C" {
    pub type args_entry;
    pub type cmds;
    pub type cmd;
    pub type cmdq_item;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_get_args(_: *mut crate::cmd::cmd) -> *mut args;
    #[no_mangle]
    fn cmdq_error(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn key_bindings_get_table(_: *const libc::c_char, _: libc::c_int) -> *mut key_table;
    #[no_mangle]
    fn key_bindings_remove(_: *const libc::c_char, _: key_code);
    #[no_mangle]
    fn key_bindings_remove_table(_: *const libc::c_char);
    #[no_mangle]
    fn key_string_lookup_string(_: *const libc::c_char) -> key_code;
}
pub type __u_char = libc::c_uchar;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
pub type u_int = __u_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_tree {
    pub rbh_root: *mut crate::arguments::args_entry,
}
pub type key_code = libc::c_ulonglong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub default_key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub note: *const libc::c_char,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_list {
    pub references: libc::c_int,
    pub group: u_int,
    pub list: *mut crate::cmd::cmds,
}
pub type cmd_find_type = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type cmd_retval = libc::c_int;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: C2RustUnnamed_1,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<
        unsafe extern "C" fn(
            _: *mut crate::cmd::cmd,
            _: *mut crate::cmd_queue::cmdq_item,
        ) -> cmd_retval,
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[no_mangle]
pub static mut cmd_unbind_key_entry: cmd_entry = {
    {
        let mut init = cmd_entry {
            name: b"unbind-key\x00" as *const u8 as *const libc::c_char,
            alias: b"unbind\x00" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed_1 {
                    template: b"anqT:\x00" as *const u8 as *const libc::c_char,
                    lower: 0 as libc::c_int,
                    upper: 1 as libc::c_int,
                };
                init
            },
            usage: b"[-anq] [-T key-table] key\x00" as *const u8 as *const libc::c_char,
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
            flags: 0x4 as libc::c_int,
            exec: Some(
                cmd_unbind_key_exec
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd::cmd,
                        _: *mut crate::cmd_queue::cmdq_item,
                    ) -> cmd_retval,
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
 * Unbind key from command.
 */
unsafe extern "C" fn cmd_unbind_key_exec(
    mut self_0: *mut crate::cmd::cmd,
    mut item: *mut crate::cmd_queue::cmdq_item,
) -> cmd_retval {
    let mut args: *mut args = cmd_get_args(self_0);
    let mut key: key_code = 0;
    let mut tablename: *const libc::c_char = 0 as *const libc::c_char;
    let mut quiet: libc::c_int = args_has(args, 'q' as i32 as u_char);
    if args_has(args, 'a' as i32 as u_char) != 0 {
        if (*args).argc != 0 as libc::c_int {
            if quiet == 0 {
                cmdq_error(
                    item,
                    b"key given with -a\x00" as *const u8 as *const libc::c_char,
                );
            }
            return CMD_RETURN_ERROR;
        }
        tablename = args_get(args, 'T' as i32 as u_char);
        if tablename.is_null() {
            if args_has(args, 'n' as i32 as u_char) != 0 {
                tablename = b"root\x00" as *const u8 as *const libc::c_char
            } else {
                tablename = b"prefix\x00" as *const u8 as *const libc::c_char
            }
        }
        if key_bindings_get_table(tablename, 0 as libc::c_int).is_null() {
            if quiet == 0 {
                cmdq_error(
                    item,
                    b"table %s doesn\'t exist\x00" as *const u8 as *const libc::c_char,
                    tablename,
                );
            }
            return CMD_RETURN_ERROR;
        }
        key_bindings_remove_table(tablename);
        return CMD_RETURN_NORMAL;
    }
    if (*args).argc != 1 as libc::c_int {
        if quiet == 0 {
            cmdq_error(item, b"missing key\x00" as *const u8 as *const libc::c_char);
        }
        return CMD_RETURN_ERROR;
    }
    key = key_string_lookup_string(*(*args).argv.offset(0 as libc::c_int as isize));
    if key == 0xff000000000 as libc::c_ulonglong || key == 0xfe000000000 as libc::c_ulonglong {
        if quiet == 0 {
            cmdq_error(
                item,
                b"unknown key: %s\x00" as *const u8 as *const libc::c_char,
                *(*args).argv.offset(0 as libc::c_int as isize),
            );
        }
        return CMD_RETURN_ERROR;
    }
    if args_has(args, 'T' as i32 as u_char) != 0 {
        tablename = args_get(args, 'T' as i32 as u_char);
        if key_bindings_get_table(tablename, 0 as libc::c_int).is_null() {
            if quiet == 0 {
                cmdq_error(
                    item,
                    b"table %s doesn\'t exist\x00" as *const u8 as *const libc::c_char,
                    tablename,
                );
            }
            return CMD_RETURN_ERROR;
        }
    } else if args_has(args, 'n' as i32 as u_char) != 0 {
        tablename = b"root\x00" as *const u8 as *const libc::c_char
    } else {
        tablename = b"prefix\x00" as *const u8 as *const libc::c_char
    }
    key_bindings_remove(tablename, key);
    return CMD_RETURN_NORMAL;
}
