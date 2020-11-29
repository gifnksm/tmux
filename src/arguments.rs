use ::libc;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    static mut BSDoptind: libc::c_int;
    #[no_mangle]
    static mut BSDoptreset: libc::c_int;
    #[no_mangle]
    static mut BSDoptarg: *mut libc::c_char;
    #[no_mangle]
    fn BSDgetopt(
        _: libc::c_int,
        _: *const *mut libc::c_char,
        _: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xvasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn cmd_copy_argv(_: libc::c_int, _: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
    #[no_mangle]
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char);
    #[no_mangle]
    fn utf8_stravis(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __u_char = libc::c_uchar;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;

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
    pub rbh_root: *mut args_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_entry {
    pub flag: u_char,
    pub values: args_values,
    pub count: u_int,
    pub entry: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed {
    pub rbe_left: *mut args_entry,
    pub rbe_right: *mut args_entry,
    pub rbe_parent: *mut args_entry,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_values {
    pub tqh_first: *mut args_value,
    pub tqh_last: *mut *mut args_value,
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2010 Nicholas Marriott <nicholas.marriott@gmail.com>
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
 * Manipulate command arguments.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args_value {
    pub value: *mut libc::c_char,
    pub entry: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut args_value,
    pub tqe_prev: *mut *mut args_value,
}
unsafe extern "C" fn args_tree_RB_MINMAX(
    mut head: *mut args_tree,
    mut val: libc::c_int,
) -> *mut args_entry {
    let mut tmp: *mut args_entry = (*head).rbh_root;
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else {
            tmp = (*tmp).entry.rbe_right
        }
    }
    return parent;
}
unsafe extern "C" fn args_tree_RB_REMOVE(
    mut head: *mut args_tree,
    mut elm: *mut args_entry,
) -> *mut args_entry {
    let mut current_block: u64;
    let mut child: *mut args_entry = 0 as *mut args_entry;
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    let mut old: *mut args_entry = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut args_entry = 0 as *mut args_entry;
        elm = (*elm).entry.rbe_right;
        loop {
            left = (*elm).entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() {
            (*child).entry.rbe_parent = parent
        }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child
            } else {
                (*parent).entry.rbe_right = child
            }
        } else {
            (*head).rbh_root = child
        }
        if (*elm).entry.rbe_parent == old {
            parent = elm
        }
        (*elm).entry = (*old).entry;
        if !(*old).entry.rbe_parent.is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm
            } else {
                (*(*old).entry.rbe_parent).entry.rbe_right = elm
            }
        } else {
            (*head).rbh_root = elm
        }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !(*old).entry.rbe_right.is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 15378387224937501455;
    }
    match current_block {
        7226443171521532240 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() {
                (*child).entry.rbe_parent = parent
            }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child
                } else {
                    (*parent).entry.rbe_right = child
                }
            } else {
                (*head).rbh_root = child
            }
        }
        _ => {}
    }
    if color == 0i32 {
        args_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn args_tree_RB_REMOVE_COLOR(
    mut head: *mut args_tree,
    mut parent: *mut args_entry,
    mut elm: *mut args_entry,
) {
    let mut tmp: *mut args_entry = 0 as *mut args_entry;
    while (elm.is_null() || (*elm).entry.rbe_color == 0i32) && elm != (*head).rbh_root {
        if (*parent).entry.rbe_left == elm {
            tmp = (*parent).entry.rbe_right;
            if (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 1i32;
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*parent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                tmp = (*parent).entry.rbe_right
            }
            if ((*tmp).entry.rbe_left.is_null() || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32)
            {
                (*tmp).entry.rbe_color = 1i32;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32
                {
                    let mut oleft: *mut args_entry = 0 as *mut args_entry;
                    oleft = (*tmp).entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).entry.rbe_color = 0i32
                    }
                    (*tmp).entry.rbe_color = 1i32;
                    oleft = (*tmp).entry.rbe_left;
                    (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                    if !(*tmp).entry.rbe_left.is_null() {
                        (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp
                    }
                    (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oleft).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft
                        }
                    } else {
                        (*head).rbh_root = oleft
                    }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    !(*oleft).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_right
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0i32;
                if !(*tmp).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_color = 0i32
                }
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*parent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).entry.rbe_left;
            if (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 1i32;
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*parent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                tmp = (*parent).entry.rbe_left
            }
            if ((*tmp).entry.rbe_left.is_null() || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32)
            {
                (*tmp).entry.rbe_color = 1i32;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_left.is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32
                {
                    let mut oright: *mut args_entry = 0 as *mut args_entry;
                    oright = (*tmp).entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).entry.rbe_color = 0i32
                    }
                    (*tmp).entry.rbe_color = 1i32;
                    oright = (*tmp).entry.rbe_right;
                    (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                    if !(*tmp).entry.rbe_right.is_null() {
                        (*(*oright).entry.rbe_left).entry.rbe_parent = tmp
                    }
                    (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oright).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oright
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oright
                        }
                    } else {
                        (*head).rbh_root = oright
                    }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    !(*oright).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_left
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0i32;
                if !(*tmp).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_color = 0i32
                }
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*parent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).entry.rbe_color = 0i32
    };
}
unsafe extern "C" fn args_tree_RB_NEXT(mut elm: *mut args_entry) -> *mut args_entry {
    if !(*elm).entry.rbe_right.is_null() {
        elm = (*elm).entry.rbe_right;
        while !(*elm).entry.rbe_left.is_null() {
            elm = (*elm).entry.rbe_left
        }
    } else if !(*elm).entry.rbe_parent.is_null() && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
    {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null()
            && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
        {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
unsafe extern "C" fn args_tree_RB_INSERT(
    mut head: *mut args_tree,
    mut elm: *mut args_entry,
) -> *mut args_entry {
    let mut tmp: *mut args_entry = 0 as *mut args_entry;
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = args_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut args_entry;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1i32;
    if !parent.is_null() {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else {
            (*parent).entry.rbe_right = elm
        }
    } else {
        (*head).rbh_root = elm
    }
    args_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut args_entry;
}
unsafe extern "C" fn args_tree_RB_INSERT_COLOR(mut head: *mut args_tree, mut elm: *mut args_entry) {
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    let mut gparent: *mut args_entry = 0 as *mut args_entry;
    let mut tmp: *mut args_entry = 0 as *mut args_entry;
    loop {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1i32) {
            break;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
                elm = gparent
            } else {
                if (*parent).entry.rbe_right == elm {
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !(*parent).entry.rbe_right.is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
                tmp = (*gparent).entry.rbe_left;
                (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*gparent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = gparent
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_right = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
            }
        } else {
            tmp = (*gparent).entry.rbe_left;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
                elm = gparent
            } else {
                if (*parent).entry.rbe_left == elm {
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !(*parent).entry.rbe_left.is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
                tmp = (*gparent).entry.rbe_right;
                (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*gparent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = gparent
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_left = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0i32;
}
unsafe extern "C" fn args_tree_RB_FIND(
    mut head: *mut args_tree,
    mut elm: *mut args_entry,
) -> *mut args_entry {
    let mut tmp: *mut args_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = args_cmp(elm, tmp);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut args_entry;
}
/* Arguments tree comparison function. */
unsafe extern "C" fn args_cmp(mut a1: *mut args_entry, mut a2: *mut args_entry) -> libc::c_int {
    return (*a1).flag as libc::c_int - (*a2).flag as libc::c_int;
}
/* Find a flag in the arguments tree. */
unsafe extern "C" fn args_find(mut args: *mut args, mut flag: u_char) -> *mut args_entry {
    let mut entry: args_entry = args_entry {
        flag: 0,
        values: args_values {
            tqh_first: 0 as *mut args_value,
            tqh_last: 0 as *mut *mut args_value,
        },
        count: 0,
        entry: C2RustUnnamed {
            rbe_left: 0 as *mut args_entry,
            rbe_right: 0 as *mut args_entry,
            rbe_parent: 0 as *mut args_entry,
            rbe_color: 0,
        },
    };
    entry.flag = flag;
    return args_tree_RB_FIND(&mut (*args).tree, &mut entry);
}
/* Parse an argv and argc into a new argument set. */
#[no_mangle]
pub unsafe extern "C" fn args_parse(
    mut template: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut args {
    let mut args: *mut args = 0 as *mut args;
    let mut opt: libc::c_int = 0;
    args = xcalloc(1u64, ::std::mem::size_of::<args>() as libc::c_ulong) as *mut args;
    BSDoptreset = 1i32;
    BSDoptind = 1i32;
    BSDoptarg = 0 as *mut libc::c_char;
    loop {
        opt = BSDgetopt(argc, argv, template);
        if !(opt != -(1i32)) {
            break;
        }
        if opt < 0i32 {
            continue;
        }
        if opt == '?' as i32 || strchr(template, opt).is_null() {
            args_free(args);
            return 0 as *mut args;
        }
        args_set(args, opt as u_char, BSDoptarg);
        BSDoptarg = 0 as *mut libc::c_char
    }
    argc -= BSDoptind;
    argv = argv.offset(BSDoptind as isize);
    (*args).argc = argc;
    (*args).argv = cmd_copy_argv(argc, argv);
    return args;
}
/* Free an arguments set. */
#[no_mangle]
pub unsafe extern "C" fn args_free(mut args: *mut args) {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    let mut entry1: *mut args_entry = 0 as *mut args_entry;
    let mut value: *mut args_value = 0 as *mut args_value;
    let mut value1: *mut args_value = 0 as *mut args_value;
    cmd_free_argv((*args).argc, (*args).argv);
    entry = args_tree_RB_MINMAX(&mut (*args).tree, -(1i32));
    while !entry.is_null() && {
        entry1 = args_tree_RB_NEXT(entry);
        (1i32) != 0
    } {
        args_tree_RB_REMOVE(&mut (*args).tree, entry);
        value = (*entry).values.tqh_first;
        while !value.is_null() && {
            value1 = (*value).entry.tqe_next;
            (1i32) != 0
        } {
            if !(*value).entry.tqe_next.is_null() {
                (*(*value).entry.tqe_next).entry.tqe_prev = (*value).entry.tqe_prev
            } else {
                (*entry).values.tqh_last = (*value).entry.tqe_prev
            }
            *(*value).entry.tqe_prev = (*value).entry.tqe_next;
            free((*value).value as *mut libc::c_void);
            free(value as *mut libc::c_void);
            value = value1
        }
        free(entry as *mut libc::c_void);
        entry = entry1
    }
    free(args as *mut libc::c_void);
}
/* Add to string. */
unsafe extern "C" fn args_print_add(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slen: size_t = 0;
    ap = args.clone();
    slen = xvasprintf(&mut s, fmt, ap.as_va_list()) as size_t;
    *len = (*len).wrapping_add(slen);
    *buf = xrealloc(*buf as *mut libc::c_void, *len) as *mut libc::c_char;
    strlcat(*buf, s, *len);
    free(s as *mut libc::c_void);
}
/* Add value to string. */
unsafe extern "C" fn args_print_add_value(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut entry: *mut args_entry,
    mut value: *mut args_value,
) {
    let mut escaped: *mut libc::c_char = 0 as *mut libc::c_char;
    if **buf as libc::c_int != '\u{0}' as i32 {
        args_print_add(
            buf,
            len,
            b" -%c \x00" as *const u8 as *const libc::c_char,
            (*entry).flag as libc::c_int,
        );
    } else {
        args_print_add(
            buf,
            len,
            b"-%c \x00" as *const u8 as *const libc::c_char,
            (*entry).flag as libc::c_int,
        );
    }
    escaped = args_escape((*value).value);
    args_print_add(
        buf,
        len,
        b"%s\x00" as *const u8 as *const libc::c_char,
        escaped,
    );
    free(escaped as *mut libc::c_void);
}
/* Add argument to string. */
unsafe extern "C" fn args_print_add_argument(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut argument: *const libc::c_char,
) {
    let mut escaped: *mut libc::c_char = 0 as *mut libc::c_char;
    if **buf as libc::c_int != '\u{0}' as i32 {
        args_print_add(buf, len, b" \x00" as *const u8 as *const libc::c_char);
    }
    escaped = args_escape(argument);
    args_print_add(
        buf,
        len,
        b"%s\x00" as *const u8 as *const libc::c_char,
        escaped,
    );
    free(escaped as *mut libc::c_void);
}
/* Print a set of arguments. */
#[no_mangle]
pub unsafe extern "C" fn args_print(mut args: *mut args) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: u_int = 0;
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    let mut value: *mut args_value = 0 as *mut args_value;
    len = 1u64;
    buf = xcalloc(1u64, len) as *mut libc::c_char;
    /* Process the flags first. */
    entry = args_tree_RB_MINMAX(&mut (*args).tree, -(1i32));
    while !entry.is_null() {
        if (*entry).values.tqh_first.is_null() {
            if *buf as libc::c_int == '\u{0}' as i32 {
                args_print_add(
                    &mut buf as *mut *mut libc::c_char,
                    &mut len as *mut size_t,
                    b"-\x00" as *const u8 as *const libc::c_char,
                );
            }
            j = 0u32;
            while j < (*entry).count {
                args_print_add(
                    &mut buf as *mut *mut libc::c_char,
                    &mut len as *mut size_t,
                    b"%c\x00" as *const u8 as *const libc::c_char,
                    (*entry).flag as libc::c_int,
                );
                j = j.wrapping_add(1)
            }
        }
        entry = args_tree_RB_NEXT(entry)
    }
    /* Then the flags with arguments. */
    entry = args_tree_RB_MINMAX(&mut (*args).tree, -(1i32));
    while !entry.is_null() {
        value = (*entry).values.tqh_first;
        while !value.is_null() {
            args_print_add_value(&mut buf, &mut len, entry, value);
            value = (*value).entry.tqe_next
        }
        entry = args_tree_RB_NEXT(entry)
    }
    /* And finally the argument vector. */
    i = 0i32;
    while i < (*args).argc {
        args_print_add_argument(&mut buf, &mut len, *(*args).argv.offset(i as isize));
        i += 1
    }
    return buf;
}
/* Escape an argument. */
#[no_mangle]
pub unsafe extern "C" fn args_escape(mut s: *const libc::c_char) -> *mut libc::c_char {
    static mut dquoted: [libc::c_char; 8] =
        unsafe { *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b" #\';${}\x00") };
    static mut squoted: [libc::c_char; 3] =
        unsafe { *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b" \"\x00") };
    let mut escaped: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: libc::c_int = 0;
    let mut quotes: libc::c_int = 0i32;
    if *s as libc::c_int == '\u{0}' as i32 {
        xasprintf(
            &mut result as *mut *mut libc::c_char,
            b"\'\'\x00" as *const u8 as *const libc::c_char,
        );
        return result;
    }
    if *s.offset(strcspn(s, dquoted.as_ptr()) as isize) as libc::c_int != '\u{0}' as i32 {
        quotes = '\"' as i32
    } else if *s.offset(strcspn(s, squoted.as_ptr()) as isize) as libc::c_int != '\u{0}' as i32 {
        quotes = '\'' as i32
    }
    if *s.offset(0isize) as libc::c_int != ' ' as i32
        && *s.offset(1isize) as libc::c_int == '\u{0}' as i32
        && (quotes != 0i32 || *s.offset(0isize) as libc::c_int == '~' as i32)
    {
        xasprintf(
            &mut escaped as *mut *mut libc::c_char,
            b"\\%c\x00" as *const u8 as *const libc::c_char,
            *s.offset(0isize) as libc::c_int,
        );
        return escaped;
    }
    flags = 0x1i32 | 0x2i32 | 0x8i32 | 0x10i32;
    if quotes == '\"' as i32 {
        flags |= 0x200i32
    }
    utf8_stravis(&mut escaped, s, flags);
    if quotes == '\'' as i32 {
        xasprintf(
            &mut result as *mut *mut libc::c_char,
            b"\'%s\'\x00" as *const u8 as *const libc::c_char,
            escaped,
        );
    } else if quotes == '\"' as i32 {
        if *escaped as libc::c_int == '~' as i32 {
            xasprintf(
                &mut result as *mut *mut libc::c_char,
                b"\"\\%s\"\x00" as *const u8 as *const libc::c_char,
                escaped,
            );
        } else {
            xasprintf(
                &mut result as *mut *mut libc::c_char,
                b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                escaped,
            );
        }
    } else if *escaped as libc::c_int == '~' as i32 {
        xasprintf(
            &mut result as *mut *mut libc::c_char,
            b"\\%s\x00" as *const u8 as *const libc::c_char,
            escaped,
        );
    } else {
        result = xstrdup(escaped)
    }
    free(escaped as *mut libc::c_void);
    return result;
}
/* Return if an argument is present. */
#[no_mangle]
pub unsafe extern "C" fn args_has(mut args: *mut args, mut flag: u_char) -> libc::c_int {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    entry = args_find(args, flag);
    if entry.is_null() {
        return 0i32;
    }
    return (*entry).count as libc::c_int;
}
/* Set argument value in the arguments tree. */
#[no_mangle]
pub unsafe extern "C" fn args_set(
    mut args: *mut args,
    mut flag: u_char,
    mut s: *const libc::c_char,
) {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    let mut value: *mut args_value = 0 as *mut args_value;
    entry = args_find(args, flag);
    if entry.is_null() {
        entry =
            xcalloc(1u64, ::std::mem::size_of::<args_entry>() as libc::c_ulong) as *mut args_entry;
        (*entry).flag = flag;
        (*entry).count = 1u32;
        (*entry).values.tqh_first = 0 as *mut args_value;
        (*entry).values.tqh_last = &mut (*entry).values.tqh_first;
        args_tree_RB_INSERT(&mut (*args).tree, entry);
    } else {
        (*entry).count = (*entry).count.wrapping_add(1)
    }
    if !s.is_null() {
        value =
            xcalloc(1u64, ::std::mem::size_of::<args_value>() as libc::c_ulong) as *mut args_value;
        (*value).value = xstrdup(s);
        (*value).entry.tqe_next = 0 as *mut args_value;
        (*value).entry.tqe_prev = (*entry).values.tqh_last;
        *(*entry).values.tqh_last = value;
        (*entry).values.tqh_last = &mut (*value).entry.tqe_next
    };
}
/* Get argument value. Will be NULL if it isn't present. */
#[no_mangle]
pub unsafe extern "C" fn args_get(mut args: *mut args, mut flag: u_char) -> *const libc::c_char {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    entry = args_find(args, flag);
    if entry.is_null() {
        return 0 as *const libc::c_char;
    }
    if (*entry).values.tqh_first.is_null() {
        return 0 as *const libc::c_char;
    }
    return (**(*((*entry).values.tqh_last as *mut args_values)).tqh_last).value;
}
/* Get first argument. */
#[no_mangle]
pub unsafe extern "C" fn args_first(
    mut args: *mut args,
    mut entry: *mut *mut args_entry,
) -> u_char {
    *entry = args_tree_RB_MINMAX(&mut (*args).tree, -(1i32));
    if (*entry).is_null() {
        return 0u8;
    }
    return (**entry).flag;
}
/* Get next argument. */
#[no_mangle]
pub unsafe extern "C" fn args_next(mut entry: *mut *mut args_entry) -> u_char {
    *entry = args_tree_RB_NEXT(*entry);
    if (*entry).is_null() {
        return 0u8;
    }
    return (**entry).flag;
}
/* Get first value in argument. */
#[no_mangle]
pub unsafe extern "C" fn args_first_value(
    mut args: *mut args,
    mut flag: u_char,
    mut value: *mut *mut args_value,
) -> *const libc::c_char {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    entry = args_find(args, flag);
    if entry.is_null() {
        return 0 as *const libc::c_char;
    }
    *value = (*entry).values.tqh_first;
    if (*value).is_null() {
        return 0 as *const libc::c_char;
    }
    return (**value).value;
}
/* Get next value in argument. */
#[no_mangle]
pub unsafe extern "C" fn args_next_value(mut value: *mut *mut args_value) -> *const libc::c_char {
    if (*value).is_null() {
        return 0 as *const libc::c_char;
    }
    *value = (**value).entry.tqe_next;
    if (*value).is_null() {
        return 0 as *const libc::c_char;
    }
    return (**value).value;
}
/* Convert an argument value to a number. */
#[no_mangle]
pub unsafe extern "C" fn args_strtonum(
    mut args: *mut args,
    mut flag: u_char,
    mut minval: libc::c_longlong,
    mut maxval: libc::c_longlong,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_longlong {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ll: libc::c_longlong = 0;
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    let mut value: *mut args_value = 0 as *mut args_value;
    entry = args_find(args, flag);
    if entry.is_null() {
        *cause = xstrdup(b"missing\x00" as *const u8 as *const libc::c_char);
        return 0i64;
    }
    value = *(*((*entry).values.tqh_last as *mut args_values)).tqh_last;
    ll = strtonum((*value).value, minval, maxval, &mut errstr);
    if !errstr.is_null() {
        *cause = xstrdup(errstr);
        return 0i64;
    }
    *cause = 0 as *mut libc::c_char;
    return ll;
}
/* Convert an argument to a number which may be a percentage. */
#[no_mangle]
pub unsafe extern "C" fn args_percentage(
    mut args: *mut args,
    mut flag: u_char,
    mut minval: libc::c_longlong,
    mut maxval: libc::c_longlong,
    mut curval: libc::c_longlong,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_longlong {
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    entry = args_find(args, flag);
    if entry.is_null() {
        *cause = xstrdup(b"missing\x00" as *const u8 as *const libc::c_char);
        return 0i64;
    }
    value = (**(*((*entry).values.tqh_last as *mut args_values)).tqh_last).value;
    return args_string_percentage(value, minval, maxval, curval, cause);
}
/* Convert a string to a number which may be a percentage. */
#[no_mangle]
pub unsafe extern "C" fn args_string_percentage(
    mut value: *const libc::c_char,
    mut minval: libc::c_longlong,
    mut maxval: libc::c_longlong,
    mut curval: libc::c_longlong,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_longlong {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ll: libc::c_longlong = 0;
    let mut valuelen: size_t = strlen(value);
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    if *value.offset(valuelen.wrapping_sub(1u64) as isize) as libc::c_int == '%' as i32 {
        copy = xstrdup(value);
        *copy.offset(valuelen.wrapping_sub(1u64) as isize) = '\u{0}' as libc::c_char;
        ll = strtonum(copy, 0i64, 100i64, &mut errstr);
        free(copy as *mut libc::c_void);
        if !errstr.is_null() {
            *cause = xstrdup(errstr);
            return 0i64;
        }
        ll = curval * ll / 100i64;
        if ll < minval {
            *cause = xstrdup(b"too small\x00" as *const u8 as *const libc::c_char);
            return 0i64;
        }
        if ll > maxval {
            *cause = xstrdup(b"too large\x00" as *const u8 as *const libc::c_char);
            return 0i64;
        }
    } else {
        ll = strtonum(value, minval, maxval, &mut errstr);
        if !errstr.is_null() {
            *cause = xstrdup(errstr);
            return 0i64;
        }
    }
    *cause = 0 as *mut libc::c_char;
    return ll;
}
