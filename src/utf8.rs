use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn __ctype_get_mb_cur_max() -> size_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn mbtowc(__pwc: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    #[no_mangle]
    fn vis(
        _: *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
}
pub type __u_char = libc::c_uchar;
pub type __u_int = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type u_int = __u_int;
pub type ssize_t = __ssize_t;
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
pub type wchar_t = libc::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utf8_data {
    pub data: [u_char; 21],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type utf8_char = u_int;
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
/* $OpenBSD$ */
/*
 * Copyright (c) 2008 Nicholas Marriott <nicholas.marriott@gmail.com>
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utf8_item {
    pub index_entry: C2RustUnnamed_1,
    pub index: u_int,
    pub data_entry: C2RustUnnamed_0,
    pub data: [libc::c_char; 21],
    pub size: u_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub rbe_left: *mut utf8_item,
    pub rbe_right: *mut utf8_item,
    pub rbe_parent: *mut utf8_item,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub rbe_left: *mut utf8_item,
    pub rbe_right: *mut utf8_item,
    pub rbe_parent: *mut utf8_item,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utf8_data_tree {
    pub rbh_root: *mut utf8_item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utf8_index_tree {
    pub rbh_root: *mut utf8_item,
}
unsafe extern "C" fn utf8_data_cmp(
    mut ui1: *mut utf8_item,
    mut ui2: *mut utf8_item,
) -> libc::c_int {
    if ((*ui1).size as libc::c_int) < (*ui2).size as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*ui1).size as libc::c_int > (*ui2).size as libc::c_int {
        return 1 as libc::c_int;
    }
    return memcmp(
        (*ui1).data.as_mut_ptr() as *const libc::c_void,
        (*ui2).data.as_mut_ptr() as *const libc::c_void,
        (*ui1).size as libc::c_ulong,
    );
}
unsafe extern "C" fn utf8_data_tree_RB_FIND(
    mut head: *mut utf8_data_tree,
    mut elm: *mut utf8_item,
) -> *mut utf8_item {
    let mut tmp: *mut utf8_item = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = utf8_data_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).data_entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).data_entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut utf8_item;
}
unsafe extern "C" fn utf8_data_tree_RB_INSERT_COLOR(
    mut head: *mut utf8_data_tree,
    mut elm: *mut utf8_item,
) {
    let mut parent: *mut utf8_item = 0 as *mut utf8_item;
    let mut gparent: *mut utf8_item = 0 as *mut utf8_item;
    let mut tmp: *mut utf8_item = 0 as *mut utf8_item;
    loop {
        parent = (*elm).data_entry.rbe_parent;
        if !(!parent.is_null() && (*parent).data_entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).data_entry.rbe_parent;
        if parent == (*gparent).data_entry.rbe_left {
            tmp = (*gparent).data_entry.rbe_right;
            if !tmp.is_null() && (*tmp).data_entry.rbe_color == 1 as libc::c_int {
                (*tmp).data_entry.rbe_color = 0 as libc::c_int;
                (*parent).data_entry.rbe_color = 0 as libc::c_int;
                (*gparent).data_entry.rbe_color = 1 as libc::c_int;
                elm = gparent
            } else {
                if (*parent).data_entry.rbe_right == elm {
                    tmp = (*parent).data_entry.rbe_right;
                    (*parent).data_entry.rbe_right = (*tmp).data_entry.rbe_left;
                    if !(*parent).data_entry.rbe_right.is_null() {
                        (*(*tmp).data_entry.rbe_left).data_entry.rbe_parent = parent
                    }
                    (*tmp).data_entry.rbe_parent = (*parent).data_entry.rbe_parent;
                    if !(*tmp).data_entry.rbe_parent.is_null() {
                        if parent == (*(*parent).data_entry.rbe_parent).data_entry.rbe_left {
                            (*(*parent).data_entry.rbe_parent).data_entry.rbe_left = tmp
                        } else {
                            (*(*parent).data_entry.rbe_parent).data_entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).data_entry.rbe_left = parent;
                    (*parent).data_entry.rbe_parent = tmp;
                    !(*tmp).data_entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).data_entry.rbe_color = 0 as libc::c_int;
                (*gparent).data_entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).data_entry.rbe_left;
                (*gparent).data_entry.rbe_left = (*tmp).data_entry.rbe_right;
                if !(*gparent).data_entry.rbe_left.is_null() {
                    (*(*tmp).data_entry.rbe_right).data_entry.rbe_parent = gparent
                }
                (*tmp).data_entry.rbe_parent = (*gparent).data_entry.rbe_parent;
                if !(*tmp).data_entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).data_entry.rbe_parent).data_entry.rbe_left {
                        (*(*gparent).data_entry.rbe_parent).data_entry.rbe_left = tmp
                    } else {
                        (*(*gparent).data_entry.rbe_parent).data_entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).data_entry.rbe_right = gparent;
                (*gparent).data_entry.rbe_parent = tmp;
                !(*tmp).data_entry.rbe_parent.is_null();
            }
        } else {
            tmp = (*gparent).data_entry.rbe_left;
            if !tmp.is_null() && (*tmp).data_entry.rbe_color == 1 as libc::c_int {
                (*tmp).data_entry.rbe_color = 0 as libc::c_int;
                (*parent).data_entry.rbe_color = 0 as libc::c_int;
                (*gparent).data_entry.rbe_color = 1 as libc::c_int;
                elm = gparent
            } else {
                if (*parent).data_entry.rbe_left == elm {
                    tmp = (*parent).data_entry.rbe_left;
                    (*parent).data_entry.rbe_left = (*tmp).data_entry.rbe_right;
                    if !(*parent).data_entry.rbe_left.is_null() {
                        (*(*tmp).data_entry.rbe_right).data_entry.rbe_parent = parent
                    }
                    (*tmp).data_entry.rbe_parent = (*parent).data_entry.rbe_parent;
                    if !(*tmp).data_entry.rbe_parent.is_null() {
                        if parent == (*(*parent).data_entry.rbe_parent).data_entry.rbe_left {
                            (*(*parent).data_entry.rbe_parent).data_entry.rbe_left = tmp
                        } else {
                            (*(*parent).data_entry.rbe_parent).data_entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).data_entry.rbe_right = parent;
                    (*parent).data_entry.rbe_parent = tmp;
                    !(*tmp).data_entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).data_entry.rbe_color = 0 as libc::c_int;
                (*gparent).data_entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).data_entry.rbe_right;
                (*gparent).data_entry.rbe_right = (*tmp).data_entry.rbe_left;
                if !(*gparent).data_entry.rbe_right.is_null() {
                    (*(*tmp).data_entry.rbe_left).data_entry.rbe_parent = gparent
                }
                (*tmp).data_entry.rbe_parent = (*gparent).data_entry.rbe_parent;
                if !(*tmp).data_entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).data_entry.rbe_parent).data_entry.rbe_left {
                        (*(*gparent).data_entry.rbe_parent).data_entry.rbe_left = tmp
                    } else {
                        (*(*gparent).data_entry.rbe_parent).data_entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).data_entry.rbe_left = gparent;
                (*gparent).data_entry.rbe_parent = tmp;
                !(*tmp).data_entry.rbe_parent.is_null();
            }
        }
    }
    (*(*head).rbh_root).data_entry.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn utf8_data_tree_RB_INSERT(
    mut head: *mut utf8_data_tree,
    mut elm: *mut utf8_item,
) -> *mut utf8_item {
    let mut tmp: *mut utf8_item = 0 as *mut utf8_item;
    let mut parent: *mut utf8_item = 0 as *mut utf8_item;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = utf8_data_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).data_entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).data_entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).data_entry.rbe_parent = parent;
    (*elm).data_entry.rbe_right = 0 as *mut utf8_item;
    (*elm).data_entry.rbe_left = (*elm).data_entry.rbe_right;
    (*elm).data_entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).data_entry.rbe_left = elm
        } else {
            (*parent).data_entry.rbe_right = elm
        }
    } else {
        (*head).rbh_root = elm
    }
    utf8_data_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut utf8_item;
}
static mut utf8_data_tree: utf8_data_tree = {
    let mut init = utf8_data_tree {
        rbh_root: 0 as *const utf8_item as *mut utf8_item,
    };
    init
};
unsafe extern "C" fn utf8_index_cmp(
    mut ui1: *mut utf8_item,
    mut ui2: *mut utf8_item,
) -> libc::c_int {
    if (*ui1).index < (*ui2).index {
        return -(1 as libc::c_int);
    }
    if (*ui1).index > (*ui2).index {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn utf8_index_tree_RB_FIND(
    mut head: *mut utf8_index_tree,
    mut elm: *mut utf8_item,
) -> *mut utf8_item {
    let mut tmp: *mut utf8_item = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = utf8_index_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).index_entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).index_entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut utf8_item;
}
unsafe extern "C" fn utf8_index_tree_RB_INSERT(
    mut head: *mut utf8_index_tree,
    mut elm: *mut utf8_item,
) -> *mut utf8_item {
    let mut tmp: *mut utf8_item = 0 as *mut utf8_item;
    let mut parent: *mut utf8_item = 0 as *mut utf8_item;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = utf8_index_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).index_entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).index_entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).index_entry.rbe_parent = parent;
    (*elm).index_entry.rbe_right = 0 as *mut utf8_item;
    (*elm).index_entry.rbe_left = (*elm).index_entry.rbe_right;
    (*elm).index_entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).index_entry.rbe_left = elm
        } else {
            (*parent).index_entry.rbe_right = elm
        }
    } else {
        (*head).rbh_root = elm
    }
    utf8_index_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut utf8_item;
}
unsafe extern "C" fn utf8_index_tree_RB_INSERT_COLOR(
    mut head: *mut utf8_index_tree,
    mut elm: *mut utf8_item,
) {
    let mut parent: *mut utf8_item = 0 as *mut utf8_item;
    let mut gparent: *mut utf8_item = 0 as *mut utf8_item;
    let mut tmp: *mut utf8_item = 0 as *mut utf8_item;
    loop {
        parent = (*elm).index_entry.rbe_parent;
        if !(!parent.is_null() && (*parent).index_entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).index_entry.rbe_parent;
        if parent == (*gparent).index_entry.rbe_left {
            tmp = (*gparent).index_entry.rbe_right;
            if !tmp.is_null() && (*tmp).index_entry.rbe_color == 1 as libc::c_int {
                (*tmp).index_entry.rbe_color = 0 as libc::c_int;
                (*parent).index_entry.rbe_color = 0 as libc::c_int;
                (*gparent).index_entry.rbe_color = 1 as libc::c_int;
                elm = gparent
            } else {
                if (*parent).index_entry.rbe_right == elm {
                    tmp = (*parent).index_entry.rbe_right;
                    (*parent).index_entry.rbe_right = (*tmp).index_entry.rbe_left;
                    if !(*parent).index_entry.rbe_right.is_null() {
                        (*(*tmp).index_entry.rbe_left).index_entry.rbe_parent = parent
                    }
                    (*tmp).index_entry.rbe_parent = (*parent).index_entry.rbe_parent;
                    if !(*tmp).index_entry.rbe_parent.is_null() {
                        if parent == (*(*parent).index_entry.rbe_parent).index_entry.rbe_left {
                            (*(*parent).index_entry.rbe_parent).index_entry.rbe_left = tmp
                        } else {
                            (*(*parent).index_entry.rbe_parent).index_entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).index_entry.rbe_left = parent;
                    (*parent).index_entry.rbe_parent = tmp;
                    !(*tmp).index_entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).index_entry.rbe_color = 0 as libc::c_int;
                (*gparent).index_entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).index_entry.rbe_left;
                (*gparent).index_entry.rbe_left = (*tmp).index_entry.rbe_right;
                if !(*gparent).index_entry.rbe_left.is_null() {
                    (*(*tmp).index_entry.rbe_right).index_entry.rbe_parent = gparent
                }
                (*tmp).index_entry.rbe_parent = (*gparent).index_entry.rbe_parent;
                if !(*tmp).index_entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).index_entry.rbe_parent).index_entry.rbe_left {
                        (*(*gparent).index_entry.rbe_parent).index_entry.rbe_left = tmp
                    } else {
                        (*(*gparent).index_entry.rbe_parent).index_entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).index_entry.rbe_right = gparent;
                (*gparent).index_entry.rbe_parent = tmp;
                !(*tmp).index_entry.rbe_parent.is_null();
            }
        } else {
            tmp = (*gparent).index_entry.rbe_left;
            if !tmp.is_null() && (*tmp).index_entry.rbe_color == 1 as libc::c_int {
                (*tmp).index_entry.rbe_color = 0 as libc::c_int;
                (*parent).index_entry.rbe_color = 0 as libc::c_int;
                (*gparent).index_entry.rbe_color = 1 as libc::c_int;
                elm = gparent
            } else {
                if (*parent).index_entry.rbe_left == elm {
                    tmp = (*parent).index_entry.rbe_left;
                    (*parent).index_entry.rbe_left = (*tmp).index_entry.rbe_right;
                    if !(*parent).index_entry.rbe_left.is_null() {
                        (*(*tmp).index_entry.rbe_right).index_entry.rbe_parent = parent
                    }
                    (*tmp).index_entry.rbe_parent = (*parent).index_entry.rbe_parent;
                    if !(*tmp).index_entry.rbe_parent.is_null() {
                        if parent == (*(*parent).index_entry.rbe_parent).index_entry.rbe_left {
                            (*(*parent).index_entry.rbe_parent).index_entry.rbe_left = tmp
                        } else {
                            (*(*parent).index_entry.rbe_parent).index_entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).index_entry.rbe_right = parent;
                    (*parent).index_entry.rbe_parent = tmp;
                    !(*tmp).index_entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).index_entry.rbe_color = 0 as libc::c_int;
                (*gparent).index_entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).index_entry.rbe_right;
                (*gparent).index_entry.rbe_right = (*tmp).index_entry.rbe_left;
                if !(*gparent).index_entry.rbe_right.is_null() {
                    (*(*tmp).index_entry.rbe_left).index_entry.rbe_parent = gparent
                }
                (*tmp).index_entry.rbe_parent = (*gparent).index_entry.rbe_parent;
                if !(*tmp).index_entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).index_entry.rbe_parent).index_entry.rbe_left {
                        (*(*gparent).index_entry.rbe_parent).index_entry.rbe_left = tmp
                    } else {
                        (*(*gparent).index_entry.rbe_parent).index_entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).index_entry.rbe_left = gparent;
                (*gparent).index_entry.rbe_parent = tmp;
                !(*tmp).index_entry.rbe_parent.is_null();
            }
        }
    }
    (*(*head).rbh_root).index_entry.rbe_color = 0 as libc::c_int;
}
static mut utf8_index_tree: utf8_index_tree = {
    let mut init = utf8_index_tree {
        rbh_root: 0 as *const utf8_item as *mut utf8_item,
    };
    init
};
static mut utf8_next_index: u_int = 0;
/* Get a UTF-8 item from data. */
unsafe extern "C" fn utf8_item_by_data(
    mut data: *const libc::c_char,
    mut size: size_t,
) -> *mut utf8_item {
    let mut ui: utf8_item = utf8_item {
        index_entry: C2RustUnnamed_1 {
            rbe_left: 0 as *mut utf8_item,
            rbe_right: 0 as *mut utf8_item,
            rbe_parent: 0 as *mut utf8_item,
            rbe_color: 0,
        },
        index: 0,
        data_entry: C2RustUnnamed_0 {
            rbe_left: 0 as *mut utf8_item,
            rbe_right: 0 as *mut utf8_item,
            rbe_parent: 0 as *mut utf8_item,
            rbe_color: 0,
        },
        data: [0; 21],
        size: 0,
    };
    memcpy(
        ui.data.as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        size,
    );
    ui.size = size as u_char;
    return utf8_data_tree_RB_FIND(&mut utf8_data_tree, &mut ui);
}
/* Get a UTF-8 item from data. */
unsafe extern "C" fn utf8_item_by_index(mut index: u_int) -> *mut utf8_item {
    let mut ui: utf8_item = utf8_item {
        index_entry: C2RustUnnamed_1 {
            rbe_left: 0 as *mut utf8_item,
            rbe_right: 0 as *mut utf8_item,
            rbe_parent: 0 as *mut utf8_item,
            rbe_color: 0,
        },
        index: 0,
        data_entry: C2RustUnnamed_0 {
            rbe_left: 0 as *mut utf8_item,
            rbe_right: 0 as *mut utf8_item,
            rbe_parent: 0 as *mut utf8_item,
            rbe_color: 0,
        },
        data: [0; 21],
        size: 0,
    };
    ui.index = index;
    return utf8_index_tree_RB_FIND(&mut utf8_index_tree, &mut ui);
}
/* Add a UTF-8 item. */
unsafe extern "C" fn utf8_put_item(
    mut data: *const libc::c_char,
    mut size: size_t,
    mut index: *mut u_int,
) -> libc::c_int {
    let mut ui: *mut utf8_item = 0 as *mut utf8_item;
    ui = utf8_item_by_data(data, size);
    if !ui.is_null() {
        *index = (*ui).index;
        log_debug(
            b"%s: found %.*s = %u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"utf8_put_item\x00"))
                .as_ptr(),
            size as libc::c_int,
            data,
            *index,
        );
        return 0 as libc::c_int;
    }
    if utf8_next_index == (0xffffff as libc::c_int + 1 as libc::c_int) as libc::c_uint {
        return -(1 as libc::c_int);
    }
    ui = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<utf8_item>() as libc::c_ulong,
    ) as *mut utf8_item;
    let fresh0 = utf8_next_index;
    utf8_next_index = utf8_next_index.wrapping_add(1);
    (*ui).index = fresh0;
    utf8_index_tree_RB_INSERT(&mut utf8_index_tree, ui);
    memcpy(
        (*ui).data.as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        size,
    );
    (*ui).size = size as u_char;
    utf8_data_tree_RB_INSERT(&mut utf8_data_tree, ui);
    *index = (*ui).index;
    log_debug(
        b"%s: added %.*s = %u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"utf8_put_item\x00")).as_ptr(),
        size as libc::c_int,
        data,
        *index,
    );
    return 0 as libc::c_int;
}
/* Get UTF-8 character from data. */
#[no_mangle]
pub unsafe extern "C" fn utf8_from_data(
    mut ud: *const utf8_data,
    mut uc: *mut utf8_char,
) -> utf8_state {
    let mut current_block: u64;
    let mut index: u_int = 0;
    if (*ud).width as libc::c_int > 2 as libc::c_int {
        fatalx(
            b"invalid UTF-8 width: %u\x00" as *const u8 as *const libc::c_char,
            (*ud).width as libc::c_int,
        );
    }
    if !((*ud).size as libc::c_int > 21 as libc::c_int) {
        if (*ud).size as libc::c_int <= 3 as libc::c_int {
            index = ((*ud).data[2 as libc::c_int as usize] as utf8_char) << 16 as libc::c_int
                | ((*ud).data[1 as libc::c_int as usize] as utf8_char) << 8 as libc::c_int
                | (*ud).data[0 as libc::c_int as usize] as utf8_char;
            current_block = 10879442775620481940;
        } else if utf8_put_item(
            (*ud).data.as_ptr() as *const libc::c_char,
            (*ud).size as size_t,
            &mut index,
        ) != 0 as libc::c_int
        {
            current_block = 8721612281967708268;
        } else {
            current_block = 10879442775620481940;
        }
        match current_block {
            8721612281967708268 => {}
            _ => {
                *uc = ((*ud).size as utf8_char) << 24 as libc::c_int
                    | ((*ud).width as utf8_char).wrapping_add(1 as libc::c_int as libc::c_uint)
                        << 29 as libc::c_int
                    | index;
                log_debug(
                    b"%s: (%d %d %.*s) -> %08x\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"utf8_from_data\x00",
                    ))
                    .as_ptr(),
                    (*ud).width as libc::c_int,
                    (*ud).size as libc::c_int,
                    (*ud).size as libc::c_int,
                    (*ud).data.as_ptr(),
                    *uc,
                );
                return UTF8_DONE;
            }
        }
    }
    if (*ud).width as libc::c_int == 0 as libc::c_int {
        *uc = (0 as libc::c_int as utf8_char) << 24 as libc::c_int
            | (0 as libc::c_int as utf8_char).wrapping_add(1 as libc::c_int as libc::c_uint)
                << 29 as libc::c_int
    } else if (*ud).width as libc::c_int == 1 as libc::c_int {
        *uc = (1 as libc::c_int as utf8_char) << 24 as libc::c_int
            | (1 as libc::c_int as utf8_char).wrapping_add(1 as libc::c_int as libc::c_uint)
                << 29 as libc::c_int
            | 0x20 as libc::c_int as libc::c_uint
    } else {
        *uc = (1 as libc::c_int as utf8_char) << 24 as libc::c_int
            | (1 as libc::c_int as utf8_char).wrapping_add(1 as libc::c_int as libc::c_uint)
                << 29 as libc::c_int
            | 0x2020 as libc::c_int as libc::c_uint
    }
    return UTF8_ERROR;
}
/* Get UTF-8 data from character. */
#[no_mangle]
pub unsafe extern "C" fn utf8_to_data(mut uc: utf8_char, mut ud: *mut utf8_data) {
    let mut ui: *mut utf8_item = 0 as *mut utf8_item;
    let mut index: u_int = 0;
    memset(
        ud as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
    );
    (*ud).have = (uc >> 24 as libc::c_int & 0x1f as libc::c_int as libc::c_uint) as u_char;
    (*ud).size = (*ud).have;
    (*ud).width =
        (uc >> 29 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_uint) as u_char;
    if (*ud).size as libc::c_int <= 3 as libc::c_int {
        (*ud).data[2 as libc::c_int as usize] = (uc >> 16 as libc::c_int) as u_char;
        (*ud).data[1 as libc::c_int as usize] =
            (uc >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as u_char;
        (*ud).data[0 as libc::c_int as usize] = (uc & 0xff as libc::c_int as libc::c_uint) as u_char
    } else {
        index = uc & 0xffffff as libc::c_int as libc::c_uint;
        ui = utf8_item_by_index(index);
        if ui.is_null() {
            memset(
                (*ud).data.as_mut_ptr() as *mut libc::c_void,
                ' ' as i32,
                (*ud).size as libc::c_ulong,
            );
        } else {
            memcpy(
                (*ud).data.as_mut_ptr() as *mut libc::c_void,
                (*ui).data.as_mut_ptr() as *const libc::c_void,
                (*ud).size as libc::c_ulong,
            );
        }
    }
    log_debug(
        b"%s: %08x -> (%d %d %.*s)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"utf8_to_data\x00")).as_ptr(),
        uc,
        (*ud).width as libc::c_int,
        (*ud).size as libc::c_int,
        (*ud).size as libc::c_int,
        (*ud).data.as_mut_ptr(),
    );
}
/* Get UTF-8 character from a single ASCII character. */
#[no_mangle]
pub unsafe extern "C" fn utf8_build_one(mut ch: u_char) -> utf8_char {
    return (1 as libc::c_int as utf8_char) << 24 as libc::c_int
        | (1 as libc::c_int as utf8_char).wrapping_add(1 as libc::c_int as libc::c_uint)
            << 29 as libc::c_int
        | ch as libc::c_uint;
}
/* Set a single character. */
#[no_mangle]
pub unsafe extern "C" fn utf8_set(mut ud: *mut utf8_data, mut ch: u_char) {
    static mut empty: utf8_data = {
        let mut init = utf8_data {
            data: [
                0 as libc::c_int as u_char,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            have: 1 as libc::c_int as u_char,
            size: 1 as libc::c_int as u_char,
            width: 1 as libc::c_int as u_char,
        };
        init
    };
    memcpy(
        ud as *mut libc::c_void,
        &empty as *const utf8_data as *const libc::c_void,
        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
    );
    *(*ud).data.as_mut_ptr() = ch;
}
/* Copy UTF-8 character. */
#[no_mangle]
pub unsafe extern "C" fn utf8_copy(mut to: *mut utf8_data, mut from: *const utf8_data) {
    let mut i: u_int = 0;
    memcpy(
        to as *mut libc::c_void,
        from as *const libc::c_void,
        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
    );
    i = (*to).size as u_int;
    while (i as libc::c_ulong) < ::std::mem::size_of::<[u_char; 21]>() as libc::c_ulong {
        (*to).data[i as usize] = '\u{0}' as i32 as u_char;
        i = i.wrapping_add(1)
    }
}
/* Get width of Unicode character. */
unsafe extern "C" fn utf8_width(mut ud: *mut utf8_data, mut width: *mut libc::c_int) -> utf8_state {
    let mut wc: wchar_t = 0;
    match mbtowc(
        &mut wc,
        (*ud).data.as_mut_ptr() as *const libc::c_char,
        (*ud).size as size_t,
    ) {
        -1 => {
            log_debug(
                b"UTF-8 %.*s, mbtowc() %d\x00" as *const u8 as *const libc::c_char,
                (*ud).size as libc::c_int,
                (*ud).data.as_mut_ptr(),
                *__errno_location(),
            );
            mbtowc(
                0 as *mut wchar_t,
                0 as *const libc::c_char,
                __ctype_get_mb_cur_max(),
            );
            return UTF8_ERROR;
        }
        0 => return UTF8_ERROR,
        _ => {}
    }
    *width = wcwidth(wc);
    if *width >= 0 as libc::c_int && *width <= 0xff as libc::c_int {
        return UTF8_DONE;
    }
    log_debug(
        b"UTF-8 %.*s, wcwidth() %d\x00" as *const u8 as *const libc::c_char,
        (*ud).size as libc::c_int,
        (*ud).data.as_mut_ptr(),
        *width,
    );
    /*
     * Many platforms (particularly and inevitably OS X) have no width for
     * relatively common characters (wcwidth() returns -1); assume width 1
     * in this case. This will be wrong for genuinely nonprintable
     * characters, but they should be rare. We may pass through stuff that
     * ideally we would block, but this is no worse than sending the same
     * to the terminal without tmux.
     */
    if *width < 0 as libc::c_int {
        *width = 1 as libc::c_int;
        return UTF8_DONE;
    }
    return UTF8_ERROR;
}
/*
 * Open UTF-8 sequence.
 *
 * 11000010-11011111 C2-DF start of 2-byte sequence
 * 11100000-11101111 E0-EF start of 3-byte sequence
 * 11110000-11110100 F0-F4 start of 4-byte sequence
 */
#[no_mangle]
pub unsafe extern "C" fn utf8_open(mut ud: *mut utf8_data, mut ch: u_char) -> utf8_state {
    memset(
        ud as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
    );
    if ch as libc::c_int >= 0xc2 as libc::c_int && ch as libc::c_int <= 0xdf as libc::c_int {
        (*ud).size = 2 as libc::c_int as u_char
    } else if ch as libc::c_int >= 0xe0 as libc::c_int && ch as libc::c_int <= 0xef as libc::c_int {
        (*ud).size = 3 as libc::c_int as u_char
    } else if ch as libc::c_int >= 0xf0 as libc::c_int && ch as libc::c_int <= 0xf4 as libc::c_int {
        (*ud).size = 4 as libc::c_int as u_char
    } else {
        return UTF8_ERROR;
    }
    utf8_append(ud, ch);
    return UTF8_MORE;
}
/* Append character to UTF-8, closing if finished. */
#[no_mangle]
pub unsafe extern "C" fn utf8_append(mut ud: *mut utf8_data, mut ch: u_char) -> utf8_state {
    let mut width: libc::c_int = 0;
    if (*ud).have as libc::c_int >= (*ud).size as libc::c_int {
        fatalx(b"UTF-8 character overflow\x00" as *const u8 as *const libc::c_char);
    }
    if (*ud).size as libc::c_ulong > ::std::mem::size_of::<[u_char; 21]>() as libc::c_ulong {
        fatalx(b"UTF-8 character size too large\x00" as *const u8 as *const libc::c_char);
    }
    if (*ud).have as libc::c_int != 0 as libc::c_int
        && ch as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int
    {
        (*ud).width = 0xff as libc::c_int as u_char
    }
    let fresh1 = (*ud).have;
    (*ud).have = (*ud).have.wrapping_add(1);
    (*ud).data[fresh1 as usize] = ch;
    if (*ud).have as libc::c_int != (*ud).size as libc::c_int {
        return UTF8_MORE;
    }
    if (*ud).width as libc::c_int == 0xff as libc::c_int {
        return UTF8_ERROR;
    }
    if utf8_width(ud, &mut width) as libc::c_uint != UTF8_DONE as libc::c_int as libc::c_uint {
        return UTF8_ERROR;
    }
    (*ud).width = width as u_char;
    return UTF8_DONE;
}
/*
 * Encode len characters from src into dst, which is guaranteed to have four
 * bytes available for each character from src (for \abc or UTF-8) plus space
 * for \0.
 */
#[no_mangle]
pub unsafe extern "C" fn utf8_strvis(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut len: size_t,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut ud: utf8_data = utf8_data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut start: *const libc::c_char = dst;
    let mut end: *const libc::c_char = src.offset(len as isize);
    let mut more: utf8_state = UTF8_MORE;
    let mut i: size_t = 0;
    while src < end {
        more = utf8_open(&mut ud, *src as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            loop {
                src = src.offset(1);
                if !(src < end && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint)
                {
                    break;
                }
                more = utf8_append(&mut ud, *src as u_char)
            }
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                /* UTF-8 character finished. */
                i = 0 as libc::c_int as size_t;
                while i < ud.size as libc::c_ulong {
                    let fresh2 = dst;
                    dst = dst.offset(1);
                    *fresh2 = ud.data[i as usize] as libc::c_char;
                    i = i.wrapping_add(1)
                }
                continue;
            } else {
                /* Not a complete, valid UTF-8 character. */
                src = src.offset(-(ud.have as libc::c_int as isize))
            }
        }
        if *src.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32
            && src < end.offset(-(1 as libc::c_int as isize))
        {
            if *(*__ctype_b_loc())
                .offset(*src.offset(1 as libc::c_int as isize) as u_char as libc::c_int as isize)
                as libc::c_int
                & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0
                || *src.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
                || *src.offset(1 as libc::c_int as isize) as libc::c_int == '{' as i32
            {
                let fresh3 = dst;
                dst = dst.offset(1);
                *fresh3 = '\\' as i32 as libc::c_char
            }
            let fresh4 = dst;
            dst = dst.offset(1);
            *fresh4 = '$' as i32 as libc::c_char
        } else if src < end.offset(-(1 as libc::c_int as isize)) {
            dst = vis(
                dst,
                *src.offset(0 as libc::c_int as isize) as libc::c_int,
                flag,
                *src.offset(1 as libc::c_int as isize) as libc::c_int,
            )
        } else if src < end {
            dst = vis(
                dst,
                *src.offset(0 as libc::c_int as isize) as libc::c_int,
                flag,
                '\u{0}' as i32,
            )
        }
        src = src.offset(1)
    }
    *dst = '\u{0}' as i32 as libc::c_char;
    return dst.wrapping_offset_from(start) as libc::c_long as libc::c_int;
}
/* Same as utf8_strvis but allocate the buffer. */
#[no_mangle]
pub unsafe extern "C" fn utf8_stravis(
    mut dst: *mut *mut libc::c_char,
    mut src: *const libc::c_char,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    buf = xreallocarray(
        0 as *mut libc::c_void,
        4 as libc::c_int as size_t,
        strlen(src).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    len = utf8_strvis(buf, src, strlen(src), flag);
    *dst =
        xrealloc(buf as *mut libc::c_void, (len + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    return len;
}
/* Same as utf8_strvis but allocate the buffer. */
#[no_mangle]
pub unsafe extern "C" fn utf8_stravisx(
    mut dst: *mut *mut libc::c_char,
    mut src: *const libc::c_char,
    mut srclen: size_t,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    buf = xreallocarray(
        0 as *mut libc::c_void,
        4 as libc::c_int as size_t,
        srclen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    len = utf8_strvis(buf, src, srclen, flag);
    *dst =
        xrealloc(buf as *mut libc::c_void, (len + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    return len;
}
/* Does this string contain anything that isn't valid UTF-8? */
#[no_mangle]
pub unsafe extern "C" fn utf8_isvalid(mut s: *const libc::c_char) -> libc::c_int {
    let mut ud: utf8_data = utf8_data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut more: utf8_state = UTF8_MORE;
    end = s.offset(strlen(s) as isize);
    while s < end {
        more = utf8_open(&mut ud, *s as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            loop {
                s = s.offset(1);
                if !(s < end && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint) {
                    break;
                }
                more = utf8_append(&mut ud, *s as u_char)
            }
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                continue;
            }
            return 0 as libc::c_int;
        } else {
            if (*s as libc::c_int) < 0x20 as libc::c_int || *s as libc::c_int > 0x7e as libc::c_int
            {
                return 0 as libc::c_int;
            }
            s = s.offset(1)
        }
    }
    return 1 as libc::c_int;
}
/*
 * Sanitize a string, changing any UTF-8 characters to '_'. Caller should free
 * the returned string. Anything not valid printable ASCII or UTF-8 is
 * stripped.
 */
#[no_mangle]
pub unsafe extern "C" fn utf8_sanitize(mut src: *const libc::c_char) -> *mut libc::c_char {
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut more: utf8_state = UTF8_MORE;
    let mut ud: utf8_data = utf8_data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut i: u_int = 0;
    while *src as libc::c_int != '\u{0}' as i32 {
        dst = xreallocarray(
            dst as *mut libc::c_void,
            n.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        ) as *mut libc::c_char;
        more = utf8_open(&mut ud, *src as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            loop {
                src = src.offset(1);
                if !(*src as libc::c_int != '\u{0}' as i32
                    && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint)
                {
                    break;
                }
                more = utf8_append(&mut ud, *src as u_char)
            }
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                dst = xreallocarray(
                    dst as *mut libc::c_void,
                    n.wrapping_add(ud.width as libc::c_ulong),
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                ) as *mut libc::c_char;
                i = 0 as libc::c_int as u_int;
                while i < ud.width as libc::c_uint {
                    let fresh5 = n;
                    n = n.wrapping_add(1);
                    *dst.offset(fresh5 as isize) = '_' as i32 as libc::c_char;
                    i = i.wrapping_add(1)
                }
                continue;
            } else {
                src = src.offset(-(ud.have as libc::c_int as isize))
            }
        }
        if *src as libc::c_int > 0x1f as libc::c_int && (*src as libc::c_int) < 0x7f as libc::c_int
        {
            let fresh6 = n;
            n = n.wrapping_add(1);
            *dst.offset(fresh6 as isize) = *src
        } else {
            let fresh7 = n;
            n = n.wrapping_add(1);
            *dst.offset(fresh7 as isize) = '_' as i32 as libc::c_char
        }
        src = src.offset(1)
    }
    dst = xreallocarray(
        dst as *mut libc::c_void,
        n.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    *dst.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    return dst;
}
/* Get UTF-8 buffer length. */
#[no_mangle]
pub unsafe extern "C" fn utf8_strlen(mut s: *const utf8_data) -> size_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while (*s.offset(i as isize)).size as libc::c_int != 0 as libc::c_int {
        /* nothing */
        i = i.wrapping_add(1)
    }
    return i;
}
/* Get UTF-8 string width. */
#[no_mangle]
pub unsafe extern "C" fn utf8_strwidth(mut s: *const utf8_data, mut n: ssize_t) -> u_int {
    let mut i: ssize_t = 0;
    let mut width: u_int = 0 as libc::c_int as u_int;
    i = 0 as libc::c_int as ssize_t;
    while (*s.offset(i as isize)).size as libc::c_int != 0 as libc::c_int {
        if n != -(1 as libc::c_int) as libc::c_long && n == i {
            break;
        }
        width = (width as libc::c_uint).wrapping_add((*s.offset(i as isize)).width as libc::c_uint)
            as u_int as u_int;
        i += 1
    }
    return width;
}
/*
 * Convert a string into a buffer of UTF-8 characters. Terminated by size == 0.
 * Caller frees.
 */
#[no_mangle]
pub unsafe extern "C" fn utf8_fromcstr(mut src: *const libc::c_char) -> *mut utf8_data {
    let mut dst: *mut utf8_data = 0 as *mut utf8_data;
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut more: utf8_state = UTF8_MORE;
    while *src as libc::c_int != '\u{0}' as i32 {
        dst = xreallocarray(
            dst as *mut libc::c_void,
            n.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
        ) as *mut utf8_data;
        more = utf8_open(&mut *dst.offset(n as isize), *src as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            loop {
                src = src.offset(1);
                if !(*src as libc::c_int != '\u{0}' as i32
                    && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint)
                {
                    break;
                }
                more = utf8_append(&mut *dst.offset(n as isize), *src as u_char)
            }
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                n = n.wrapping_add(1);
                continue;
            } else {
                src = src.offset(-((*dst.offset(n as isize)).have as libc::c_int as isize))
            }
        }
        utf8_set(&mut *dst.offset(n as isize), *src as u_char);
        n = n.wrapping_add(1);
        src = src.offset(1)
    }
    dst = xreallocarray(
        dst as *mut libc::c_void,
        n.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
    ) as *mut utf8_data;
    (*dst.offset(n as isize)).size = 0 as libc::c_int as u_char;
    return dst;
}
/* Convert from a buffer of UTF-8 characters into a string. Caller frees. */
#[no_mangle]
pub unsafe extern "C" fn utf8_tocstr(mut src: *mut utf8_data) -> *mut libc::c_char {
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0 as libc::c_int as size_t;
    while (*src).size as libc::c_int != 0 as libc::c_int {
        dst = xreallocarray(
            dst as *mut libc::c_void,
            n.wrapping_add((*src).size as libc::c_ulong),
            1 as libc::c_int as size_t,
        ) as *mut libc::c_char;
        memcpy(
            dst.offset(n as isize) as *mut libc::c_void,
            (*src).data.as_mut_ptr() as *const libc::c_void,
            (*src).size as libc::c_ulong,
        );
        n = (n as libc::c_ulong).wrapping_add((*src).size as libc::c_ulong) as size_t as size_t;
        src = src.offset(1)
    }
    dst = xreallocarray(
        dst as *mut libc::c_void,
        n.wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as size_t,
    ) as *mut libc::c_char;
    *dst.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    return dst;
}
/* Get width of UTF-8 string. */
#[no_mangle]
pub unsafe extern "C" fn utf8_cstrwidth(mut s: *const libc::c_char) -> u_int {
    let mut tmp: utf8_data = utf8_data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut width: u_int = 0;
    let mut more: utf8_state = UTF8_MORE;
    width = 0 as libc::c_int as u_int;
    while *s as libc::c_int != '\u{0}' as i32 {
        more = utf8_open(&mut tmp, *s as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            loop {
                s = s.offset(1);
                if !(*s as libc::c_int != '\u{0}' as i32
                    && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint)
                {
                    break;
                }
                more = utf8_append(&mut tmp, *s as u_char)
            }
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                width = (width as libc::c_uint).wrapping_add(tmp.width as libc::c_uint) as u_int
                    as u_int;
                continue;
            } else {
                s = s.offset(-(tmp.have as libc::c_int as isize))
            }
        }
        if *s as libc::c_int > 0x1f as libc::c_int && *s as libc::c_int != 0x7f as libc::c_int {
            width = width.wrapping_add(1)
        }
        s = s.offset(1)
    }
    return width;
}
/* Pad UTF-8 string to width on the left. Caller frees. */
#[no_mangle]
pub unsafe extern "C" fn utf8_padcstr(
    mut s: *const libc::c_char,
    mut width: u_int,
) -> *mut libc::c_char {
    let mut slen: size_t = 0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    n = utf8_cstrwidth(s);
    if n >= width {
        return xstrdup(s);
    }
    slen = strlen(s);
    out = xmalloc(
        slen.wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(width.wrapping_sub(n) as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(out as *mut libc::c_void, s as *const libc::c_void, slen);
    i = n;
    while i < width {
        let fresh8 = slen;
        slen = slen.wrapping_add(1);
        *out.offset(fresh8 as isize) = ' ' as i32 as libc::c_char;
        i = i.wrapping_add(1)
    }
    *out.offset(slen as isize) = '\u{0}' as i32 as libc::c_char;
    return out;
}
/* Pad UTF-8 string to width on the right. Caller frees. */
#[no_mangle]
pub unsafe extern "C" fn utf8_rpadcstr(
    mut s: *const libc::c_char,
    mut width: u_int,
) -> *mut libc::c_char {
    let mut slen: size_t = 0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    n = utf8_cstrwidth(s);
    if n >= width {
        return xstrdup(s);
    }
    slen = strlen(s);
    out = xmalloc(
        slen.wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(width.wrapping_sub(n) as libc::c_ulong),
    ) as *mut libc::c_char;
    i = 0 as libc::c_int as u_int;
    while i < width.wrapping_sub(n) {
        *out.offset(i as isize) = ' ' as i32 as libc::c_char;
        i = i.wrapping_add(1)
    }
    memcpy(
        out.offset(i as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        slen,
    );
    *out.offset((i as libc::c_ulong).wrapping_add(slen) as isize) = '\u{0}' as i32 as libc::c_char;
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn utf8_cstrhas(
    mut s: *const libc::c_char,
    mut ud: *const utf8_data,
) -> libc::c_int {
    let mut copy: *mut utf8_data = 0 as *mut utf8_data;
    let mut loop_0: *mut utf8_data = 0 as *mut utf8_data;
    let mut found: libc::c_int = 0 as libc::c_int;
    copy = utf8_fromcstr(s);
    loop_0 = copy;
    while (*loop_0).size as libc::c_int != 0 as libc::c_int {
        if !((*loop_0).size as libc::c_int != (*ud).size as libc::c_int) {
            if memcmp(
                (*loop_0).data.as_mut_ptr() as *const libc::c_void,
                (*ud).data.as_ptr() as *const libc::c_void,
                (*loop_0).size as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                found = 1 as libc::c_int;
                break;
            }
        }
        loop_0 = loop_0.offset(1)
    }
    free(copy as *mut libc::c_void);
    return found;
}
