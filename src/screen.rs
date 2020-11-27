use ::libc;
extern "C" {
    pub type screen_write_collect_line;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn grid_empty_line(_: *mut grid, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_create(_: u_int, _: u_int, _: u_int) -> *mut grid;
    #[no_mangle]
    fn grid_destroy(_: *mut grid);
    #[no_mangle]
    fn grid_adjust_lines(_: *mut grid, _: u_int);
    #[no_mangle]
    fn grid_clear_lines(_: *mut grid, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_duplicate_lines(_: *mut grid, _: u_int, _: *mut grid, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_reflow(_: *mut grid, _: u_int);
    #[no_mangle]
    fn grid_wrap_position(_: *mut grid, _: u_int, _: u_int, _: *mut u_int, _: *mut u_int);
    #[no_mangle]
    fn grid_unwrap_position(_: *mut grid, _: *mut u_int, _: *mut u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_clear(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_delete_lines(_: *mut grid, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_make_list(_: *mut screen);
    #[no_mangle]
    fn screen_write_free_list(_: *mut screen);
    #[no_mangle]
    fn utf8_copy(_: *mut utf8_data, _: *const utf8_data);
    #[no_mangle]
    fn utf8_isvalid(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn utf8_stravis(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
pub type bitstr_t = libc::c_uchar;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct screen {
    pub title: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub titles: *mut screen_titles,
    pub grid: *mut grid,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub rupper: u_int,
    pub rlower: u_int,
    pub mode: libc::c_int,
    pub saved_cx: u_int,
    pub saved_cy: u_int,
    pub saved_grid: *mut grid,
    pub saved_cell: grid_cell,
    pub saved_flags: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: *mut screen_sel,
    pub write_list: *mut crate::screen_write::screen_write_collect_line,
}
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
/* Selected area in screen. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct screen_sel {
    pub hidden: libc::c_int,
    pub rectangle: libc::c_int,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grid_cell {
    pub data: utf8_data,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utf8_data {
    pub data: [u_char; 21],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grid {
    pub flags: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub hscrolled: u_int,
    pub hsize: u_int,
    pub hlimit: u_int,
    pub linedata: *mut grid_line,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct grid_line {
    pub cellused: u_int,
    pub cellsize: u_int,
    pub celldata: *mut grid_cell_entry,
    pub extdsize: u_int,
    pub extddata: *mut grid_extd_entry,
    pub flags: libc::c_int,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct grid_extd_entry {
    pub data: utf8_char,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}
pub type utf8_char = u_int;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub c2rust_unnamed: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
    pub offset: u_int,
    pub data: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct screen_titles {
    pub tqh_first: *mut screen_title_entry,
    pub tqh_last: *mut *mut screen_title_entry,
}
/* Entry on title stack. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct screen_title_entry {
    pub text: *mut libc::c_char,
    pub entry: C2RustUnnamed_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut screen_title_entry,
    pub tqe_prev: *mut *mut screen_title_entry,
}
/* Free titles stack. */
unsafe extern "C" fn screen_free_titles(mut s: *mut screen) {
    let mut title_entry: *mut screen_title_entry = 0 as *mut screen_title_entry;
    if (*s).titles.is_null() {
        return;
    }
    loop {
        title_entry = (*(*s).titles).tqh_first;
        if title_entry.is_null() {
            break;
        }
        if !(*title_entry).entry.tqe_next.is_null() {
            (*(*title_entry).entry.tqe_next).entry.tqe_prev = (*title_entry).entry.tqe_prev
        } else {
            (*(*s).titles).tqh_last = (*title_entry).entry.tqe_prev
        }
        *(*title_entry).entry.tqe_prev = (*title_entry).entry.tqe_next;
        free((*title_entry).text as *mut libc::c_void);
        free(title_entry as *mut libc::c_void);
    }
    free((*s).titles as *mut libc::c_void);
    (*s).titles = 0 as *mut screen_titles;
}
/* Create a new screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_init(
    mut s: *mut screen,
    mut sx: u_int,
    mut sy: u_int,
    mut hlimit: u_int,
) {
    (*s).grid = grid_create(sx, sy, hlimit);
    (*s).saved_grid = 0 as *mut grid;
    (*s).title = xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    (*s).titles = 0 as *mut screen_titles;
    (*s).path = 0 as *mut libc::c_char;
    (*s).cstyle = 0 as libc::c_int as u_int;
    (*s).ccolour = xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    (*s).tabs = 0 as *mut bitstr_t;
    (*s).sel = 0 as *mut screen_sel;
    (*s).write_list = 0 as *mut crate::screen_write::screen_write_collect_line;
    screen_reinit(s);
}
/* Reinitialise screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_reinit(mut s: *mut screen) {
    (*s).cx = 0 as libc::c_int as u_int;
    (*s).cy = 0 as libc::c_int as u_int;
    (*s).rupper = 0 as libc::c_int as u_int;
    (*s).rlower = (*(*s).grid)
        .sy
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    (*s).mode = 0x1 as libc::c_int | 0x10 as libc::c_int;
    if !(*s).saved_grid.is_null() {
        screen_alternate_off(s, 0 as *mut grid_cell, 0 as libc::c_int);
    }
    (*s).saved_cx = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    (*s).saved_cy = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    screen_reset_tabs(s);
    grid_clear_lines(
        (*s).grid,
        (*(*s).grid).hsize,
        (*(*s).grid).sy,
        8 as libc::c_int as u_int,
    );
    screen_clear_selection(s);
    screen_free_titles(s);
}
/* Destroy a screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_free(mut s: *mut screen) {
    free((*s).sel as *mut libc::c_void);
    free((*s).tabs as *mut libc::c_void);
    free((*s).path as *mut libc::c_void);
    free((*s).title as *mut libc::c_void);
    free((*s).ccolour as *mut libc::c_void);
    if !(*s).write_list.is_null() {
        screen_write_free_list(s);
    }
    if !(*s).saved_grid.is_null() {
        grid_destroy((*s).saved_grid);
    }
    grid_destroy((*s).grid);
    screen_free_titles(s);
}
/* Reset tabs to default, eight spaces apart. */
#[no_mangle]
pub unsafe extern "C" fn screen_reset_tabs(mut s: *mut screen) {
    let mut i: u_int = 0;
    free((*s).tabs as *mut libc::c_void);
    (*s).tabs = calloc(
        ((*(*s).grid)
            .sx
            .wrapping_add(7 as libc::c_int as libc::c_uint)
            >> 3 as libc::c_int) as size_t,
        ::std::mem::size_of::<bitstr_t>() as libc::c_ulong,
    ) as *mut bitstr_t;
    if (*s).tabs.is_null() {
        fatal(b"bit_alloc failed\x00" as *const u8 as *const libc::c_char);
    }
    i = 8 as libc::c_int as u_int;
    while i < (*(*s).grid).sx {
        let ref mut fresh0 = *(*s).tabs.offset((i >> 3 as libc::c_int) as isize);
        *fresh0 = (*fresh0 as libc::c_int
            | (1 as libc::c_int) << (i & 0x7 as libc::c_int as libc::c_uint))
            as bitstr_t;
        i = (i as libc::c_uint).wrapping_add(8 as libc::c_int as libc::c_uint) as u_int as u_int
    }
}
/* Set screen cursor style. */
#[no_mangle]
pub unsafe extern "C" fn screen_set_cursor_style(mut s: *mut screen, mut style: u_int) {
    if style <= 6 as libc::c_int as libc::c_uint {
        (*s).cstyle = style
    };
}
/* Set screen cursor colour. */
#[no_mangle]
pub unsafe extern "C" fn screen_set_cursor_colour(
    mut s: *mut screen,
    mut colour: *const libc::c_char,
) {
    free((*s).ccolour as *mut libc::c_void);
    (*s).ccolour = xstrdup(colour);
}
/* Set screen title. */
#[no_mangle]
pub unsafe extern "C" fn screen_set_title(
    mut s: *mut screen,
    mut title: *const libc::c_char,
) -> libc::c_int {
    if utf8_isvalid(title) == 0 {
        return 0 as libc::c_int;
    }
    free((*s).title as *mut libc::c_void);
    (*s).title = xstrdup(title);
    return 1 as libc::c_int;
}
/* Set screen path. */
#[no_mangle]
pub unsafe extern "C" fn screen_set_path(mut s: *mut screen, mut path: *const libc::c_char) {
    free((*s).path as *mut libc::c_void);
    utf8_stravis(
        &mut (*s).path,
        path,
        0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int,
    );
}
/* Push the current title onto the stack. */
#[no_mangle]
pub unsafe extern "C" fn screen_push_title(mut s: *mut screen) {
    let mut title_entry: *mut screen_title_entry = 0 as *mut screen_title_entry;
    if (*s).titles.is_null() {
        (*s).titles =
            xmalloc(::std::mem::size_of::<screen_titles>() as libc::c_ulong) as *mut screen_titles;
        (*(*s).titles).tqh_first = 0 as *mut screen_title_entry;
        (*(*s).titles).tqh_last = &mut (*(*s).titles).tqh_first
    }
    title_entry = xmalloc(::std::mem::size_of::<screen_title_entry>() as libc::c_ulong)
        as *mut screen_title_entry;
    (*title_entry).text = xstrdup((*s).title);
    (*title_entry).entry.tqe_next = (*(*s).titles).tqh_first;
    if !(*title_entry).entry.tqe_next.is_null() {
        (*(*(*s).titles).tqh_first).entry.tqe_prev = &mut (*title_entry).entry.tqe_next
    } else {
        (*(*s).titles).tqh_last = &mut (*title_entry).entry.tqe_next
    }
    (*(*s).titles).tqh_first = title_entry;
    (*title_entry).entry.tqe_prev = &mut (*(*s).titles).tqh_first;
}
/*
 * Pop a title from the stack and set it as the screen title. If the stack is
 * empty, do nothing.
 */
#[no_mangle]
pub unsafe extern "C" fn screen_pop_title(mut s: *mut screen) {
    let mut title_entry: *mut screen_title_entry = 0 as *mut screen_title_entry;
    if (*s).titles.is_null() {
        return;
    }
    title_entry = (*(*s).titles).tqh_first;
    if !title_entry.is_null() {
        screen_set_title(s, (*title_entry).text);
        if !(*title_entry).entry.tqe_next.is_null() {
            (*(*title_entry).entry.tqe_next).entry.tqe_prev = (*title_entry).entry.tqe_prev
        } else {
            (*(*s).titles).tqh_last = (*title_entry).entry.tqe_prev
        }
        *(*title_entry).entry.tqe_prev = (*title_entry).entry.tqe_next;
        free((*title_entry).text as *mut libc::c_void);
        free(title_entry as *mut libc::c_void);
    };
}
/* Resize screen with options. */
#[no_mangle]
pub unsafe extern "C" fn screen_resize_cursor(
    mut s: *mut screen,
    mut sx: u_int,
    mut sy: u_int,
    mut reflow: libc::c_int,
    mut eat_empty: libc::c_int,
    mut cursor: libc::c_int,
) {
    let mut cx: u_int = (*s).cx;
    let mut cy: u_int = (*(*s).grid).hsize.wrapping_add((*s).cy);
    if !(*s).write_list.is_null() {
        screen_write_free_list(s);
    }
    log_debug(
        b"%s: new size %ux%u, now %ux%u (cursor %u,%u = %u,%u)\x00" as *const u8
            as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"screen_resize_cursor\x00"))
            .as_ptr(),
        sx,
        sy,
        (*(*s).grid).sx,
        (*(*s).grid).sy,
        (*s).cx,
        (*s).cy,
        cx,
        cy,
    );
    if sx < 1 as libc::c_int as libc::c_uint {
        sx = 1 as libc::c_int as u_int
    }
    if sy < 1 as libc::c_int as libc::c_uint {
        sy = 1 as libc::c_int as u_int
    }
    if sx != (*(*s).grid).sx {
        (*(*s).grid).sx = sx;
        screen_reset_tabs(s);
    } else {
        reflow = 0 as libc::c_int
    }
    if sy != (*(*s).grid).sy {
        screen_resize_y(s, sy, eat_empty, &mut cy);
    }
    if reflow != 0 {
        screen_reflow(s, sx, &mut cx, &mut cy, cursor);
    }
    if cy >= (*(*s).grid).hsize {
        (*s).cx = cx;
        (*s).cy = cy.wrapping_sub((*(*s).grid).hsize)
    } else {
        (*s).cx = 0 as libc::c_int as u_int;
        (*s).cy = 0 as libc::c_int as u_int
    }
    log_debug(
        b"%s: cursor finished at %u,%u = %u,%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"screen_resize_cursor\x00"))
            .as_ptr(),
        (*s).cx,
        (*s).cy,
        cx,
        cy,
    );
    if !(*s).write_list.is_null() {
        screen_write_make_list(s);
    };
}
/* Resize screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_resize(
    mut s: *mut screen,
    mut sx: u_int,
    mut sy: u_int,
    mut reflow: libc::c_int,
) {
    screen_resize_cursor(s, sx, sy, reflow, 1 as libc::c_int, 1 as libc::c_int);
}
unsafe extern "C" fn screen_resize_y(
    mut s: *mut screen,
    mut sy: u_int,
    mut eat_empty: libc::c_int,
    mut cy: *mut u_int,
) {
    let mut gd: *mut grid = (*s).grid;
    let mut needed: u_int = 0;
    let mut available: u_int = 0;
    let mut oldy: u_int = 0;
    let mut i: u_int = 0;
    if sy == 0 as libc::c_int as libc::c_uint {
        fatalx(b"zero size\x00" as *const u8 as *const libc::c_char);
    }
    oldy = (*(*s).grid).sy;
    /*
     * When resizing:
     *
     * If the height is decreasing, delete lines from the bottom until
     * hitting the cursor, then push lines from the top into the history.
     *
     * When increasing, pull as many lines as possible from scrolled
     * history (not explicitly cleared from view) to the top, then fill the
     * remaining with blanks at the bottom.
     */
    /* Size decreasing. */
    if sy < oldy {
        needed = oldy.wrapping_sub(sy);
        /* Delete as many lines as possible from the bottom. */
        if eat_empty != 0 {
            available = oldy
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub((*s).cy);
            if available > 0 as libc::c_int as libc::c_uint {
                if available > needed {
                    available = needed
                }
                grid_view_delete_lines(
                    gd,
                    oldy.wrapping_sub(available),
                    available,
                    8 as libc::c_int as u_int,
                );
            }
            needed = (needed as libc::c_uint).wrapping_sub(available) as u_int as u_int
        }
        /*
         * Now just increase the history size, if possible, to take
         * over the lines which are left. If history is off, delete
         * lines from the top.
         */
        available = (*s).cy;
        if (*gd).flags & 0x1 as libc::c_int != 0 {
            (*gd).hscrolled =
                ((*gd).hscrolled as libc::c_uint).wrapping_add(needed) as u_int as u_int;
            (*gd).hsize = ((*gd).hsize as libc::c_uint).wrapping_add(needed) as u_int as u_int
        } else if needed > 0 as libc::c_int as libc::c_uint
            && available > 0 as libc::c_int as libc::c_uint
        {
            if available > needed {
                available = needed
            }
            grid_view_delete_lines(
                gd,
                0 as libc::c_int as u_int,
                available,
                8 as libc::c_int as u_int,
            );
            *cy = (*cy as libc::c_uint).wrapping_sub(available) as u_int as u_int
        }
    }
    /* Resize line array. */
    grid_adjust_lines(gd, (*gd).hsize.wrapping_add(sy));
    /* Size increasing. */
    if sy > oldy {
        needed = sy.wrapping_sub(oldy);
        /*
         * Try to pull as much as possible out of scrolled history, if
         * is is enabled.
         */
        available = (*gd).hscrolled;
        if (*gd).flags & 0x1 as libc::c_int != 0 && available > 0 as libc::c_int as libc::c_uint {
            if available > needed {
                available = needed
            }
            (*gd).hscrolled =
                ((*gd).hscrolled as libc::c_uint).wrapping_sub(available) as u_int as u_int;
            (*gd).hsize = ((*gd).hsize as libc::c_uint).wrapping_sub(available) as u_int as u_int
        } else {
            available = 0 as libc::c_int as u_int
        }
        needed = (needed as libc::c_uint).wrapping_sub(available) as u_int as u_int;
        /* Then fill the rest in with blanks. */
        i = (*gd).hsize.wrapping_add(sy).wrapping_sub(needed);
        while i < (*gd).hsize.wrapping_add(sy) {
            grid_empty_line(gd, i, 8 as libc::c_int as u_int);
            i = i.wrapping_add(1)
        }
    }
    /* Set the new size, and reset the scroll region. */
    (*gd).sy = sy;
    (*s).rupper = 0 as libc::c_int as u_int;
    (*s).rlower = (*(*s).grid)
        .sy
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
}
/* Set selection. */
#[no_mangle]
pub unsafe extern "C" fn screen_set_selection(
    mut s: *mut screen,
    mut sx: u_int,
    mut sy: u_int,
    mut ex: u_int,
    mut ey: u_int,
    mut rectangle: u_int,
    mut modekeys: libc::c_int,
    mut gc: *mut grid_cell,
) {
    if (*s).sel.is_null() {
        (*s).sel = xcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<screen_sel>() as libc::c_ulong,
        ) as *mut screen_sel
    }
    memcpy(
        &mut (*(*s).sel).cell as *mut grid_cell as *mut libc::c_void,
        gc as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    (*(*s).sel).hidden = 0 as libc::c_int;
    (*(*s).sel).rectangle = rectangle as libc::c_int;
    (*(*s).sel).modekeys = modekeys;
    (*(*s).sel).sx = sx;
    (*(*s).sel).sy = sy;
    (*(*s).sel).ex = ex;
    (*(*s).sel).ey = ey;
}
/* Clear selection. */
#[no_mangle]
pub unsafe extern "C" fn screen_clear_selection(mut s: *mut screen) {
    free((*s).sel as *mut libc::c_void);
    (*s).sel = 0 as *mut screen_sel;
}
/* Hide selection. */
#[no_mangle]
pub unsafe extern "C" fn screen_hide_selection(mut s: *mut screen) {
    if !(*s).sel.is_null() {
        (*(*s).sel).hidden = 1 as libc::c_int
    };
}
/* Check if cell in selection. */
#[no_mangle]
pub unsafe extern "C" fn screen_check_selection(
    mut s: *mut screen,
    mut px: u_int,
    mut py: u_int,
) -> libc::c_int {
    let mut sel: *mut screen_sel = (*s).sel;
    let mut xx: u_int = 0;
    if sel.is_null() || (*sel).hidden != 0 {
        return 0 as libc::c_int;
    }
    if (*sel).rectangle != 0 {
        if (*sel).sy < (*sel).ey {
            /* start line < end line -- downward selection. */
            if py < (*sel).sy || py > (*sel).ey {
                return 0 as libc::c_int;
            }
        } else if (*sel).sy > (*sel).ey {
            /* start line > end line -- upward selection. */
            if py > (*sel).sy || py < (*sel).ey {
                return 0 as libc::c_int;
            }
        } else if py != (*sel).sy {
            return 0 as libc::c_int;
        }
        /* starting line == ending line. */
        /*
         * Need to include the selection start row, but not the cursor
         * row, which means the selection changes depending on which
         * one is on the left.
         */
        if (*sel).ex < (*sel).sx {
            /* Cursor (ex) is on the left. */
            if px < (*sel).ex {
                return 0 as libc::c_int;
            }
            if px > (*sel).sx {
                return 0 as libc::c_int;
            }
        } else {
            /* Selection start (sx) is on the left. */
            if px < (*sel).sx {
                return 0 as libc::c_int;
            }
            if px > (*sel).ex {
                return 0 as libc::c_int;
            }
        }
    } else if (*sel).sy < (*sel).ey {
        /*
         * Like emacs, keep the top-left-most character, and drop the
         * bottom-right-most, regardless of copy direction.
         */
        /* starting line < ending line -- downward selection. */
        if py < (*sel).sy || py > (*sel).ey {
            return 0 as libc::c_int;
        }
        if py == (*sel).sy && px < (*sel).sx {
            return 0 as libc::c_int;
        }
        if (*sel).modekeys == 0 as libc::c_int {
            xx = if (*sel).ex == 0 as libc::c_int as libc::c_uint {
                0 as libc::c_int as libc::c_uint
            } else {
                (*sel).ex.wrapping_sub(1 as libc::c_int as libc::c_uint)
            }
        } else {
            xx = (*sel).ex
        }
        if py == (*sel).ey && px > xx {
            return 0 as libc::c_int;
        }
    } else if (*sel).sy > (*sel).ey {
        /* starting line > ending line -- upward selection. */
        if py > (*sel).sy || py < (*sel).ey {
            return 0 as libc::c_int;
        }
        if py == (*sel).ey && px < (*sel).ex {
            return 0 as libc::c_int;
        }
        if (*sel).modekeys == 0 as libc::c_int {
            xx = (*sel).sx.wrapping_sub(1 as libc::c_int as libc::c_uint)
        } else {
            xx = (*sel).sx
        }
        if py == (*sel).sy && ((*sel).sx == 0 as libc::c_int as libc::c_uint || px > xx) {
            return 0 as libc::c_int;
        }
    } else {
        /* starting line == ending line. */
        if py != (*sel).sy {
            return 0 as libc::c_int;
        }
        if (*sel).ex < (*sel).sx {
            /* cursor (ex) is on the left */
            if (*sel).modekeys == 0 as libc::c_int {
                xx = (*sel).sx.wrapping_sub(1 as libc::c_int as libc::c_uint)
            } else {
                xx = (*sel).sx
            }
            if px > xx || px < (*sel).ex {
                return 0 as libc::c_int;
            }
        } else {
            /* selection start (sx) is on the left */
            if (*sel).modekeys == 0 as libc::c_int {
                xx = if (*sel).ex == 0 as libc::c_int as libc::c_uint {
                    0 as libc::c_int as libc::c_uint
                } else {
                    (*sel).ex.wrapping_sub(1 as libc::c_int as libc::c_uint)
                }
            } else {
                xx = (*sel).ex
            }
            if px < (*sel).sx || px > xx {
                return 0 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}
/* Get selected grid cell. */
#[no_mangle]
pub unsafe extern "C" fn screen_select_cell(
    mut s: *mut screen,
    mut dst: *mut grid_cell,
    mut src: *const grid_cell,
) {
    if (*s).sel.is_null() || (*(*s).sel).hidden != 0 {
        return;
    }
    memcpy(
        dst as *mut libc::c_void,
        &mut (*(*s).sel).cell as *mut grid_cell as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    utf8_copy(&mut (*dst).data, &(*src).data);
    (*dst).attr = ((*dst).attr as libc::c_int & !(0x80 as libc::c_int)) as u_short;
    (*dst).attr =
        ((*dst).attr as libc::c_int | (*src).attr as libc::c_int & 0x80 as libc::c_int) as u_short;
    (*dst).flags = (*src).flags;
}
/* Reflow wrapped lines. */
unsafe extern "C" fn screen_reflow(
    mut s: *mut screen,
    mut new_x: u_int,
    mut cx: *mut u_int,
    mut cy: *mut u_int,
    mut cursor: libc::c_int,
) {
    let mut wx: u_int = 0;
    let mut wy: u_int = 0;
    if cursor != 0 {
        grid_wrap_position((*s).grid, *cx, *cy, &mut wx, &mut wy);
        log_debug(
            b"%s: cursor %u,%u is %u,%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"screen_reflow\x00"))
                .as_ptr(),
            *cx,
            *cy,
            wx,
            wy,
        );
    }
    grid_reflow((*s).grid, new_x);
    if cursor != 0 {
        grid_unwrap_position((*s).grid, cx, cy, wx, wy);
        log_debug(
            b"%s: new cursor is %u,%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"screen_reflow\x00"))
                .as_ptr(),
            *cx,
            *cy,
        );
    } else {
        *cx = 0 as libc::c_int as u_int;
        *cy = (*(*s).grid).hsize
    };
}
/*
 * Enter alternative screen mode. A copy of the visible screen is saved and the
 * history is not updated.
 */
#[no_mangle]
pub unsafe extern "C" fn screen_alternate_on(
    mut s: *mut screen,
    mut gc: *mut grid_cell,
    mut cursor: libc::c_int,
) {
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    if !(*s).saved_grid.is_null() {
        return;
    }
    sx = (*(*s).grid).sx;
    sy = (*(*s).grid).sy;
    (*s).saved_grid = grid_create(sx, sy, 0 as libc::c_int as u_int);
    grid_duplicate_lines(
        (*s).saved_grid,
        0 as libc::c_int as u_int,
        (*s).grid,
        (*(*s).grid).hsize,
        sy,
    );
    if cursor != 0 {
        (*s).saved_cx = (*s).cx;
        (*s).saved_cy = (*s).cy
    }
    memcpy(
        &mut (*s).saved_cell as *mut grid_cell as *mut libc::c_void,
        gc as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    grid_view_clear(
        (*s).grid,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
        sx,
        sy,
        8 as libc::c_int as u_int,
    );
    (*s).saved_flags = (*(*s).grid).flags;
    (*(*s).grid).flags &= !(0x1 as libc::c_int);
}
/* Exit alternate screen mode and restore the copied grid. */
#[no_mangle]
pub unsafe extern "C" fn screen_alternate_off(
    mut s: *mut screen,
    mut gc: *mut grid_cell,
    mut cursor: libc::c_int,
) {
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    /*
     * Restore the cursor position and cell. This happens even if not
     * currently in the alternate screen.
     */
    if cursor != 0
        && (*s).saved_cx
            != (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
        && (*s).saved_cy
            != (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
    {
        (*s).cx = (*s).saved_cx;
        if (*s).cx
            > (*(*s).grid)
                .sx
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            (*s).cx = (*(*s).grid)
                .sx
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        }
        (*s).cy = (*s).saved_cy;
        if (*s).cy
            > (*(*s).grid)
                .sy
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            (*s).cy = (*(*s).grid)
                .sy
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        }
        if !gc.is_null() {
            memcpy(
                gc as *mut libc::c_void,
                &mut (*s).saved_cell as *mut grid_cell as *const libc::c_void,
                ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
            );
        }
    }
    if (*s).saved_grid.is_null() {
        return;
    }
    sx = (*(*s).grid).sx;
    sy = (*(*s).grid).sy;
    /*
     * If the current size is bigger, temporarily resize to the old size
     * before copying back.
     */
    if sy > (*(*s).saved_grid).sy {
        screen_resize(s, sx, (*(*s).saved_grid).sy, 1 as libc::c_int);
    }
    /* Restore the saved grid. */
    grid_duplicate_lines(
        (*s).grid,
        (*(*s).grid).hsize,
        (*s).saved_grid,
        0 as libc::c_int as u_int,
        sy,
    );
    /*
     * Turn history back on (so resize can use it) and then resize back to
     * the current size.
     */
    if (*s).saved_flags & 0x1 as libc::c_int != 0 {
        (*(*s).grid).flags |= 0x1 as libc::c_int
    }
    if sy > (*(*s).saved_grid).sy || sx != (*(*s).saved_grid).sx {
        screen_resize(s, sx, sy, 1 as libc::c_int);
    }
    grid_destroy((*s).saved_grid);
    (*s).saved_grid = 0 as *mut grid;
}
