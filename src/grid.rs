use crate::utf8::{Utf8Char, Utf8Data};
use ::libc;

extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn malloc_trim(__pad: size_t) -> libc::c_int;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn colour_split_rgb(_: libc::c_int, _: *mut u_char, _: *mut u_char, _: *mut u_char);
    #[no_mangle]
    fn utf8_from_data(_: *const Utf8Data, _: *mut Utf8Char) -> crate::utf8::Utf8State;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn utf8_set(_: *mut Utf8Data, _: u_char);
    #[no_mangle]
    fn utf8_to_data(_: Utf8Char, _: *mut Utf8Data);
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn utf8_build_one(_: u_char) -> Utf8Char;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grid {
    pub flags: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub hscrolled: u_int,
    pub hsize: u_int,
    pub hlimit: u_int,
    pub linedata: *mut Line,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub mask: u_int,
    pub code: u_int,
}
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
/*
 * Grid data. This is the basic data structure that represents what is shown on
 * screen.
 *
 * A grid is a grid of cells (struct Cell). Lines are not allocated until
 * cells in that line are written to. The grid is split into history and
 * viewable data with the history starting at row (line) 0 and extending to
 * (hsize - 1); from hsize to hsize + (sy - 1) is the viewable data. All
 * functions in this file work on absolute coordinates, grid-view.c has
 * functions which work on the screen data.
 */
/// Grid cell data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Cell {
    pub data: crate::utf8::Utf8Data,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}

/// Grid extended cell entry.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ExtdEntry {
    pub data: crate::utf8::Utf8Char,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}

/// Grid cell entry.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CellEntry {
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

/// Grid line.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Line {
    pub cellused: u_int,
    pub cellsize: u_int,
    pub celldata: *mut CellEntry,
    pub extdsize: u_int,
    pub extddata: *mut ExtdEntry,
    pub flags: libc::c_int,
}

/* Default grid cell data. */
#[no_mangle]
pub static mut grid_default_cell: Cell = {
    let mut init = Cell {
        data: {
            let mut init = Utf8Data {
                data: [
                    ' ' as u_char,
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
                have: 0u8,
                size: 1u8,
                width: 1u8,
            };
            init
        },
        attr: 0u16,
        flags: 0u8,
        fg: 8i32,
        bg: 8i32,
        us: 0i32,
    };
    init
};
/*
 * Padding grid cell data. Padding cells are the only zero width cell that
 * appears in the grid - because of this, they are always extended cells.
 */
static mut grid_padding_cell: Cell = {
    let mut init = Cell {
        data: {
            let mut init = Utf8Data {
                data: [
                    '!' as u_char,
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
                have: 0u8,
                size: 0u8,
                width: 0u8,
            };
            init
        },
        attr: 0u16,
        flags: 0x4u8,
        fg: 8i32,
        bg: 8i32,
        us: 0i32,
    };
    init
};
/* Cleared grid cell data. */
static mut grid_cleared_cell: Cell = {
    let mut init = Cell {
        data: {
            let mut init = Utf8Data {
                data: [
                    ' ' as u_char,
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
                have: 0u8,
                size: 1u8,
                width: 1u8,
            };
            init
        },
        attr: 0u16,
        flags: 0x40u8,
        fg: 8i32,
        bg: 8i32,
        us: 0i32,
    };
    init
};
static mut grid_cleared_entry: CellEntry = {
    let mut init = CellEntry {
        flags: 0x40u8,
        c2rust_unnamed: C2RustUnnamed {
            data: {
                let mut init = C2RustUnnamed_0 {
                    attr: 0u8,
                    fg: 8u8,
                    bg: 8u8,
                    data: ' ' as u_char,
                };
                init
            },
        },
    };
    init
};
/* Store cell in entry. */
unsafe extern "C" fn grid_store_cell(mut gce: *mut CellEntry, mut gc: *const Cell, mut c: u_char) {
    (*gce).flags = ((*gc).flags as libc::c_int & !(0x40i32)) as u_char;
    (*gce).c2rust_unnamed.data.fg = ((*gc).fg & 0xffi32) as u_char;
    if (*gc).fg & 0x1000000i32 != 0 {
        (*gce).flags = ((*gce).flags as libc::c_int | 0x1i32) as u_char
    }
    (*gce).c2rust_unnamed.data.bg = ((*gc).bg & 0xffi32) as u_char;
    if (*gc).bg & 0x1000000i32 != 0 {
        (*gce).flags = ((*gce).flags as libc::c_int | 0x2i32) as u_char
    }
    (*gce).c2rust_unnamed.data.attr = (*gc).attr as u_char;
    (*gce).c2rust_unnamed.data.data = c;
}
/* Check if a cell should be an extended cell. */
unsafe extern "C" fn grid_need_extended_cell(
    mut gce: *const CellEntry,
    mut gc: *const Cell,
) -> libc::c_int {
    if (*gce).flags as libc::c_int & 0x8i32 != 0 {
        return 1i32;
    }
    if (*gc).attr as libc::c_int > 0xffi32 {
        return 1i32;
    }
    if (*gc).data.size as libc::c_int != 1i32 || (*gc).data.width as libc::c_int != 1i32 {
        return 1i32;
    }
    if (*gc).fg & 0x2000000i32 != 0 || (*gc).bg & 0x2000000i32 != 0 {
        return 1i32;
    }
    if (*gc).us != 0i32 {
        /* only supports 256 or RGB */
        return 1i32;
    }
    return 0i32;
}
/* Get an extended cell. */
unsafe extern "C" fn grid_get_extended_cell(
    mut gl: *mut Line,
    mut gce: *mut CellEntry,
    mut flags: libc::c_int,
) {
    let mut at: u_int = (*gl).extdsize.wrapping_add(1u32);
    (*gl).extddata = xreallocarray(
        (*gl).extddata as *mut libc::c_void,
        at as size_t,
        ::std::mem::size_of::<ExtdEntry>() as libc::c_ulong,
    ) as *mut ExtdEntry;
    (*gl).extdsize = at;
    (*gce).c2rust_unnamed.offset = at.wrapping_sub(1u32);
    (*gce).flags = (flags | 0x8i32) as u_char;
}
/* Set cell as extended. */
unsafe extern "C" fn grid_extended_cell(
    mut gl: *mut Line,
    mut gce: *mut CellEntry,
    mut gc: *const Cell,
) -> *mut ExtdEntry {
    let mut gee: *mut ExtdEntry = 0 as *mut ExtdEntry;
    let mut flags: libc::c_int = (*gc).flags as libc::c_int & !(0x40i32);
    let mut uc: Utf8Char = 0;
    if !((*gce).flags as libc::c_int) & 0x8i32 != 0 {
        grid_get_extended_cell(gl, gce, flags);
    } else if (*gce).c2rust_unnamed.offset >= (*gl).extdsize {
        fatalx(b"offset too big\x00" as *const u8 as *const libc::c_char);
    }
    (*gl).flags |= 0x2i32;
    utf8_from_data(&(*gc).data, &mut uc);
    gee = &mut *(*gl).extddata.offset((*gce).c2rust_unnamed.offset as isize) as *mut ExtdEntry;
    (*gee).data = uc;
    (*gee).attr = (*gc).attr;
    (*gee).flags = flags as u_char;
    (*gee).fg = (*gc).fg;
    (*gee).bg = (*gc).bg;
    (*gee).us = (*gc).us;
    return gee;
}
/* Free up unused extended cells. */
unsafe extern "C" fn grid_compact_line(mut gl: *mut Line) {
    let mut new_extdsize: libc::c_int = 0i32;
    let mut new_extddata: *mut ExtdEntry = 0 as *mut ExtdEntry;
    let mut gce: *mut CellEntry = 0 as *mut CellEntry;
    let mut gee: *mut ExtdEntry = 0 as *mut ExtdEntry;
    let mut px: u_int = 0;
    let mut idx: u_int = 0;
    if (*gl).extdsize == 0u32 {
        return;
    }
    px = 0u32;
    while px < (*gl).cellsize {
        gce = &mut *(*gl).celldata.offset(px as isize) as *mut CellEntry;
        if (*gce).flags as libc::c_int & 0x8i32 != 0 {
            new_extdsize += 1
        }
        px = px.wrapping_add(1)
    }
    if new_extdsize == 0i32 {
        free((*gl).extddata as *mut libc::c_void);
        (*gl).extddata = 0 as *mut ExtdEntry;
        (*gl).extdsize = 0u32;
        return;
    }
    new_extddata = xreallocarray(
        0 as *mut libc::c_void,
        new_extdsize as size_t,
        ::std::mem::size_of::<ExtdEntry>() as libc::c_ulong,
    ) as *mut ExtdEntry;
    idx = 0u32;
    px = 0u32;
    while px < (*gl).cellsize {
        gce = &mut *(*gl).celldata.offset(px as isize) as *mut CellEntry;
        if (*gce).flags as libc::c_int & 0x8i32 != 0 {
            gee = &mut *(*gl).extddata.offset((*gce).c2rust_unnamed.offset as isize)
                as *mut ExtdEntry;
            memcpy(
                &mut *new_extddata.offset(idx as isize) as *mut ExtdEntry as *mut libc::c_void,
                gee as *const libc::c_void,
                ::std::mem::size_of::<ExtdEntry>() as libc::c_ulong,
            );
            let fresh0 = idx;
            idx = idx.wrapping_add(1);
            (*gce).c2rust_unnamed.offset = fresh0
        }
        px = px.wrapping_add(1)
    }
    free((*gl).extddata as *mut libc::c_void);
    (*gl).extddata = new_extddata;
    (*gl).extdsize = new_extdsize as u_int;
}
/* Get line data. */
#[no_mangle]
pub unsafe extern "C" fn grid_get_line(mut gd: *mut grid, mut line: u_int) -> *mut Line {
    return &mut *(*gd).linedata.offset(line as isize) as *mut Line;
}
/* Adjust number of lines. */
#[no_mangle]
pub unsafe extern "C" fn grid_adjust_lines(mut gd: *mut grid, mut lines: u_int) {
    (*gd).linedata = xreallocarray(
        (*gd).linedata as *mut libc::c_void,
        lines as size_t,
        ::std::mem::size_of::<Line>() as libc::c_ulong,
    ) as *mut Line;
}
/* Copy default into a cell. */
unsafe extern "C" fn grid_clear_cell(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut bg: u_int,
) {
    let mut gl: *mut Line = &mut *(*gd).linedata.offset(py as isize) as *mut Line;
    let mut gce: *mut CellEntry = &mut *(*gl).celldata.offset(px as isize) as *mut CellEntry;
    let mut gee: *mut ExtdEntry = 0 as *mut ExtdEntry;
    memcpy(
        gce as *mut libc::c_void,
        &grid_cleared_entry as *const CellEntry as *const libc::c_void,
        ::std::mem::size_of::<CellEntry>() as libc::c_ulong,
    );
    if bg != 8u32 {
        if bg & 0x2000000u32 != 0 {
            grid_get_extended_cell(gl, gce, (*gce).flags as libc::c_int);
            gee = grid_extended_cell(gl, gce, &grid_cleared_cell);
            (*gee).bg = bg as libc::c_int
        } else {
            if bg & 0x1000000u32 != 0 {
                (*gce).flags = ((*gce).flags as libc::c_int | 0x2i32) as u_char
            }
            (*gce).c2rust_unnamed.data.bg = bg as u_char
        }
    };
}
/* Check grid y position. */
unsafe extern "C" fn grid_check_y(
    mut gd: *mut grid,
    mut from: *const libc::c_char,
    mut py: u_int,
) -> libc::c_int {
    if py >= (*gd).hsize.wrapping_add((*gd).sy) {
        log_debug(
            b"%s: y out of range: %u\x00" as *const u8 as *const libc::c_char,
            from,
            py,
        );
        return -(1i32);
    }
    return 0i32;
}
/* Check if two styles are (visibly) the same. */
#[no_mangle]
pub unsafe extern "C" fn grid_cells_look_equal(
    mut gc1: *const Cell,
    mut gc2: *const Cell,
) -> libc::c_int {
    if (*gc1).fg != (*gc2).fg || (*gc1).bg != (*gc2).bg {
        return 0i32;
    }
    if (*gc1).attr as libc::c_int != (*gc2).attr as libc::c_int
        || (*gc1).flags as libc::c_int != (*gc2).flags as libc::c_int
    {
        return 0i32;
    }
    return 1i32;
}
/* Compare grid cells. Return 1 if equal, 0 if not. */
#[no_mangle]
pub unsafe extern "C" fn grid_cells_equal(
    mut gc1: *const Cell,
    mut gc2: *const Cell,
) -> libc::c_int {
    if grid_cells_look_equal(gc1, gc2) == 0 {
        return 0i32;
    }
    if (*gc1).data.width as libc::c_int != (*gc2).data.width as libc::c_int {
        return 0i32;
    }
    if (*gc1).data.size as libc::c_int != (*gc2).data.size as libc::c_int {
        return 0i32;
    }
    return (memcmp(
        (*gc1).data.data.as_ptr() as *const libc::c_void,
        (*gc2).data.data.as_ptr() as *const libc::c_void,
        (*gc1).data.size as libc::c_ulong,
    ) == 0i32) as libc::c_int;
}
/* Free one line. */
unsafe extern "C" fn grid_free_line(mut gd: *mut grid, mut py: u_int) {
    free((*(*gd).linedata.offset(py as isize)).celldata as *mut libc::c_void);
    let ref mut fresh1 = (*(*gd).linedata.offset(py as isize)).celldata;
    *fresh1 = 0 as *mut CellEntry;
    free((*(*gd).linedata.offset(py as isize)).extddata as *mut libc::c_void);
    let ref mut fresh2 = (*(*gd).linedata.offset(py as isize)).extddata;
    *fresh2 = 0 as *mut ExtdEntry;
}
/* Free several lines. */
unsafe extern "C" fn grid_free_lines(mut gd: *mut grid, mut py: u_int, mut ny: u_int) {
    let mut yy: u_int = 0;
    yy = py;
    while yy < py.wrapping_add(ny) {
        grid_free_line(gd, yy);
        yy = yy.wrapping_add(1)
    }
    malloc_trim(0u64);
}
/* Create a new grid. */
#[no_mangle]
pub unsafe extern "C" fn grid_create(mut sx: u_int, mut sy: u_int, mut hlimit: u_int) -> *mut grid {
    let mut gd: *mut grid = 0 as *mut grid;
    gd = xmalloc(::std::mem::size_of::<grid>() as libc::c_ulong) as *mut grid;
    (*gd).sx = sx;
    (*gd).sy = sy;
    if hlimit != 0u32 {
        (*gd).flags = 0x1i32
    } else {
        (*gd).flags = 0i32
    }
    (*gd).hscrolled = 0u32;
    (*gd).hsize = 0u32;
    (*gd).hlimit = hlimit;
    if (*gd).sy != 0u32 {
        (*gd).linedata = xcalloc(
            (*gd).sy as size_t,
            ::std::mem::size_of::<Line>() as libc::c_ulong,
        ) as *mut Line
    } else {
        (*gd).linedata = 0 as *mut Line
    }
    return gd;
}
/* Destroy grid. */
#[no_mangle]
pub unsafe extern "C" fn grid_destroy(mut gd: *mut grid) {
    grid_free_lines(gd, 0u32, (*gd).hsize.wrapping_add((*gd).sy));
    free((*gd).linedata as *mut libc::c_void);
    free(gd as *mut libc::c_void);
}
/* Compare grids. */
#[no_mangle]
pub unsafe extern "C" fn grid_compare(mut ga: *mut grid, mut gb: *mut grid) -> libc::c_int {
    let mut gla: *mut Line = 0 as *mut Line;
    let mut glb: *mut Line = 0 as *mut Line;
    let mut gca: Cell = Cell {
        data: Utf8Data {
            data: [0; 21],
            have: 0,
            size: 0,
            width: 0,
        },
        attr: 0,
        flags: 0,
        fg: 0,
        bg: 0,
        us: 0,
    };
    let mut gcb: Cell = Cell {
        data: Utf8Data {
            data: [0; 21],
            have: 0,
            size: 0,
            width: 0,
        },
        attr: 0,
        flags: 0,
        fg: 0,
        bg: 0,
        us: 0,
    };
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    if (*ga).sx != (*gb).sx || (*ga).sy != (*gb).sy {
        return 1i32;
    }
    yy = 0u32;
    while yy < (*ga).sy {
        gla = &mut *(*ga).linedata.offset(yy as isize) as *mut Line;
        glb = &mut *(*gb).linedata.offset(yy as isize) as *mut Line;
        if (*gla).cellsize != (*glb).cellsize {
            return 1i32;
        }
        xx = 0u32;
        while xx < (*gla).cellsize {
            grid_get_cell(ga, xx, yy, &mut gca);
            grid_get_cell(gb, xx, yy, &mut gcb);
            if grid_cells_equal(&mut gca, &mut gcb) == 0 {
                return 1i32;
            }
            xx = xx.wrapping_add(1)
        }
        yy = yy.wrapping_add(1)
    }
    return 0i32;
}
/* Trim lines from the history. */
unsafe extern "C" fn grid_trim_history(mut gd: *mut grid, mut ny: u_int) {
    grid_free_lines(gd, 0u32, ny);
    memmove(
        &mut *(*gd).linedata.offset(0isize) as *mut Line as *mut libc::c_void,
        &mut *(*gd).linedata.offset(ny as isize) as *mut Line as *const libc::c_void,
        ((*gd).hsize.wrapping_add((*gd).sy).wrapping_sub(ny) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Line>() as libc::c_ulong),
    );
}
/*
 * Collect lines from the history if at the limit. Free the top (oldest) 10%
 * and shift up.
 */
#[no_mangle]
pub unsafe extern "C" fn grid_collect_history(mut gd: *mut grid) {
    let mut ny: u_int = 0;
    if (*gd).hsize == 0u32 || (*gd).hsize < (*gd).hlimit {
        return;
    }
    ny = (*gd).hlimit.wrapping_div(10u32);
    if ny < 1u32 {
        ny = 1u32
    }
    if ny > (*gd).hsize {
        ny = (*gd).hsize
    }
    /*
     * Free the lines from 0 to ny then move the remaining lines over
     * them.
     */
    grid_trim_history(gd, ny);
    (*gd).hsize = ((*gd).hsize).wrapping_sub(ny);
    if (*gd).hscrolled > (*gd).hsize {
        (*gd).hscrolled = (*gd).hsize
    };
}
/* Remove lines from the bottom of the history. */
#[no_mangle]
pub unsafe extern "C" fn grid_remove_history(mut gd: *mut grid, mut ny: u_int) {
    let mut yy: u_int = 0;
    if ny > (*gd).hsize {
        return;
    }
    yy = 0u32;
    while yy < ny {
        grid_free_line(
            gd,
            (*gd)
                .hsize
                .wrapping_add((*gd).sy)
                .wrapping_sub(1u32)
                .wrapping_sub(yy),
        );
        yy = yy.wrapping_add(1)
    }
    (*gd).hsize = ((*gd).hsize).wrapping_sub(ny);
}
/*
 * Scroll the entire visible screen, moving one line into the history. Just
 * allocate a new line at the bottom and move the history size indicator.
 */
#[no_mangle]
pub unsafe extern "C" fn grid_scroll_history(mut gd: *mut grid, mut bg: u_int) {
    let mut yy: u_int = 0;
    yy = (*gd).hsize.wrapping_add((*gd).sy);
    (*gd).linedata = xreallocarray(
        (*gd).linedata as *mut libc::c_void,
        yy.wrapping_add(1u32) as size_t,
        ::std::mem::size_of::<Line>() as libc::c_ulong,
    ) as *mut Line;
    grid_empty_line(gd, yy, bg);
    (*gd).hscrolled = (*gd).hscrolled.wrapping_add(1);
    grid_compact_line(&mut *(*gd).linedata.offset((*gd).hsize as isize));
    (*gd).hsize = (*gd).hsize.wrapping_add(1);
}
/* Clear the history. */
#[no_mangle]
pub unsafe extern "C" fn grid_clear_history(mut gd: *mut grid) {
    grid_trim_history(gd, (*gd).hsize);
    (*gd).hscrolled = 0u32;
    (*gd).hsize = 0u32;
    (*gd).linedata = xreallocarray(
        (*gd).linedata as *mut libc::c_void,
        (*gd).sy as size_t,
        ::std::mem::size_of::<Line>() as libc::c_ulong,
    ) as *mut Line;
}
/* Scroll a region up, moving the top line into the history. */
#[no_mangle]
pub unsafe extern "C" fn grid_scroll_history_region(
    mut gd: *mut grid,
    mut upper: u_int,
    mut lower: u_int,
    mut bg: u_int,
) {
    let mut gl_history: *mut Line = 0 as *mut Line;
    let mut gl_upper: *mut Line = 0 as *mut Line;
    let mut yy: u_int = 0;
    /* Create a space for a new line. */
    yy = (*gd).hsize.wrapping_add((*gd).sy);
    (*gd).linedata = xreallocarray(
        (*gd).linedata as *mut libc::c_void,
        yy.wrapping_add(1u32) as size_t,
        ::std::mem::size_of::<Line>() as libc::c_ulong,
    ) as *mut Line;
    /* Move the entire screen down to free a space for this line. */
    gl_history = &mut *(*gd).linedata.offset((*gd).hsize as isize) as *mut Line;
    memmove(
        gl_history.offset(1isize) as *mut libc::c_void,
        gl_history as *const libc::c_void,
        ((*gd).sy as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Line>() as libc::c_ulong),
    );
    /* Adjust the region and find its start and end. */
    upper = upper.wrapping_add(1);
    gl_upper = &mut *(*gd).linedata.offset(upper as isize) as *mut Line;
    lower = lower.wrapping_add(1);
    /* Move the line into the history. */
    memcpy(
        gl_history as *mut libc::c_void,
        gl_upper as *const libc::c_void,
        ::std::mem::size_of::<Line>() as libc::c_ulong,
    );
    /* Then move the region up and clear the bottom line. */
    memmove(
        gl_upper as *mut libc::c_void,
        gl_upper.offset(1isize) as *const libc::c_void,
        (lower.wrapping_sub(upper) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Line>() as libc::c_ulong),
    );
    grid_empty_line(gd, lower, bg);
    /* Move the history offset down over the line. */
    (*gd).hscrolled = (*gd).hscrolled.wrapping_add(1);
    (*gd).hsize = (*gd).hsize.wrapping_add(1);
}
/* Expand line to fit to cell. */
unsafe extern "C" fn grid_expand_line(
    mut gd: *mut grid,
    mut py: u_int,
    mut sx: u_int,
    mut bg: u_int,
) {
    let mut gl: *mut Line = 0 as *mut Line;
    let mut xx: u_int = 0;
    gl = &mut *(*gd).linedata.offset(py as isize) as *mut Line;
    if sx <= (*gl).cellsize {
        return;
    }
    if sx < (*gd).sx.wrapping_div(4u32) {
        sx = (*gd).sx.wrapping_div(4u32)
    } else if sx < (*gd).sx.wrapping_div(2u32) {
        sx = (*gd).sx.wrapping_div(2u32)
    } else if (*gd).sx > sx {
        sx = (*gd).sx
    }
    (*gl).celldata = xreallocarray(
        (*gl).celldata as *mut libc::c_void,
        sx as size_t,
        ::std::mem::size_of::<CellEntry>() as libc::c_ulong,
    ) as *mut CellEntry;
    xx = (*gl).cellsize;
    while xx < sx {
        grid_clear_cell(gd, xx, py, bg);
        xx = xx.wrapping_add(1)
    }
    (*gl).cellsize = sx;
}
/* Empty a line and set background colour if needed. */
#[no_mangle]
pub unsafe extern "C" fn grid_empty_line(mut gd: *mut grid, mut py: u_int, mut bg: u_int) {
    memset(
        &mut *(*gd).linedata.offset(py as isize) as *mut Line as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<Line>() as libc::c_ulong,
    );
    if !(bg == 8u32 || bg == 9u32) {
        grid_expand_line(gd, py, (*gd).sx, bg);
    };
}
/* Peek at grid line. */
#[no_mangle]
pub unsafe extern "C" fn grid_peek_line(mut gd: *mut grid, mut py: u_int) -> *const Line {
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"grid_peek_line\x00")).as_ptr(),
        py,
    ) != 0i32
    {
        return 0 as *const Line;
    }
    return &mut *(*gd).linedata.offset(py as isize) as *mut Line;
}
/* Get cell from line. */
unsafe extern "C" fn grid_get_cell1(mut gl: *mut Line, mut px: u_int, mut gc: *mut Cell) {
    let mut gce: *mut CellEntry = &mut *(*gl).celldata.offset(px as isize) as *mut CellEntry;
    let mut gee: *mut ExtdEntry = 0 as *mut ExtdEntry;
    if (*gce).flags as libc::c_int & 0x8i32 != 0 {
        if (*gce).c2rust_unnamed.offset >= (*gl).extdsize {
            memcpy(
                gc as *mut libc::c_void,
                &grid_default_cell as *const Cell as *const libc::c_void,
                ::std::mem::size_of::<Cell>() as libc::c_ulong,
            );
        } else {
            gee = &mut *(*gl).extddata.offset((*gce).c2rust_unnamed.offset as isize)
                as *mut ExtdEntry;
            (*gc).flags = (*gee).flags;
            (*gc).attr = (*gee).attr;
            (*gc).fg = (*gee).fg;
            (*gc).bg = (*gee).bg;
            (*gc).us = (*gee).us;
            utf8_to_data((*gee).data, &mut (*gc).data);
        }
        return;
    }
    (*gc).flags = ((*gce).flags as libc::c_int & !(0x1i32 | 0x2i32)) as u_char;
    (*gc).attr = (*gce).c2rust_unnamed.data.attr as u_short;
    (*gc).fg = (*gce).c2rust_unnamed.data.fg as libc::c_int;
    if (*gce).flags as libc::c_int & 0x1i32 != 0 {
        (*gc).fg |= 0x1000000i32
    }
    (*gc).bg = (*gce).c2rust_unnamed.data.bg as libc::c_int;
    if (*gce).flags as libc::c_int & 0x2i32 != 0 {
        (*gc).bg |= 0x1000000i32
    }
    (*gc).us = 0i32;
    utf8_set(&mut (*gc).data, (*gce).c2rust_unnamed.data.data);
}
/* Get cell for reading. */
#[no_mangle]
pub unsafe extern "C" fn grid_get_cell(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut gc: *mut Cell,
) {
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"grid_get_cell\x00")).as_ptr(),
        py,
    ) != 0i32
        || px >= (*(*gd).linedata.offset(py as isize)).cellsize
    {
        memcpy(
            gc as *mut libc::c_void,
            &grid_default_cell as *const Cell as *const libc::c_void,
            ::std::mem::size_of::<Cell>() as libc::c_ulong,
        );
    } else {
        grid_get_cell1(&mut *(*gd).linedata.offset(py as isize), px, gc);
    };
}
/* Set cell at position. */
#[no_mangle]
pub unsafe extern "C" fn grid_set_cell(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut gc: *const Cell,
) {
    let mut gl: *mut Line = 0 as *mut Line;
    let mut gce: *mut CellEntry = 0 as *mut CellEntry;
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"grid_set_cell\x00")).as_ptr(),
        py,
    ) != 0i32
    {
        return;
    }
    grid_expand_line(gd, py, px.wrapping_add(1u32), 8u32);
    gl = &mut *(*gd).linedata.offset(py as isize) as *mut Line;
    if px.wrapping_add(1u32) > (*gl).cellused {
        (*gl).cellused = px.wrapping_add(1u32)
    }
    gce = &mut *(*gl).celldata.offset(px as isize) as *mut CellEntry;
    if grid_need_extended_cell(gce, gc) != 0 {
        grid_extended_cell(gl, gce, gc);
    } else {
        grid_store_cell(gce, gc, (*gc).data.data[0usize]);
    };
}
/* Set padding at position. */
#[no_mangle]
pub unsafe extern "C" fn grid_set_padding(mut gd: *mut grid, mut px: u_int, mut py: u_int) {
    grid_set_cell(gd, px, py, &grid_padding_cell);
}
/* Set cells at position. */
#[no_mangle]
pub unsafe extern "C" fn grid_set_cells(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut gc: *const Cell,
    mut s: *const libc::c_char,
    mut slen: size_t,
) {
    let mut gl: *mut Line = 0 as *mut Line;
    let mut gce: *mut CellEntry = 0 as *mut CellEntry;
    let mut gee: *mut ExtdEntry = 0 as *mut ExtdEntry;
    let mut i: u_int = 0;
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"grid_set_cells\x00")).as_ptr(),
        py,
    ) != 0i32
    {
        return;
    }
    grid_expand_line(
        gd,
        py,
        (px as libc::c_ulong).wrapping_add(slen) as u_int,
        8u32,
    );
    gl = &mut *(*gd).linedata.offset(py as isize) as *mut Line;
    if (px as libc::c_ulong).wrapping_add(slen) > (*gl).cellused as libc::c_ulong {
        (*gl).cellused = (px as libc::c_ulong).wrapping_add(slen) as u_int
    }
    i = 0u32;
    while (i as libc::c_ulong) < slen {
        gce = &mut *(*gl).celldata.offset(px.wrapping_add(i) as isize) as *mut CellEntry;
        if grid_need_extended_cell(gce, gc) != 0 {
            gee = grid_extended_cell(gl, gce, gc);
            (*gee).data = utf8_build_one(*s.offset(i as isize) as u_char)
        } else {
            grid_store_cell(gce, gc, *s.offset(i as isize) as u_char);
        }
        i = i.wrapping_add(1)
    }
}
/* Clear area. */
#[no_mangle]
pub unsafe extern "C" fn grid_clear(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut gl: *mut Line = 0 as *mut Line; /* default bg first */
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut ox: u_int = 0;
    let mut sx: u_int = 0;
    if nx == 0u32 || ny == 0u32 {
        return;
    }
    if px == 0u32 && nx == (*gd).sx {
        grid_clear_lines(gd, py, ny, bg);
        return;
    }
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"grid_clear\x00")).as_ptr(),
        py,
    ) != 0i32
    {
        return;
    }
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"grid_clear\x00")).as_ptr(),
        py.wrapping_add(ny).wrapping_sub(1u32),
    ) != 0i32
    {
        return;
    }
    let mut current_block_21: u64;
    yy = py;
    while yy < py.wrapping_add(ny) {
        gl = &mut *(*gd).linedata.offset(yy as isize) as *mut Line;
        sx = (*gd).sx;
        if sx > (*gl).cellsize {
            sx = (*gl).cellsize
        }
        ox = nx;
        if bg == 8u32 || bg == 9u32 {
            if px > sx {
                current_block_21 = 11812396948646013369;
            } else {
                if px.wrapping_add(nx) > sx {
                    ox = sx.wrapping_sub(px)
                }
                current_block_21 = 2838571290723028321;
            }
        } else {
            current_block_21 = 2838571290723028321;
        }
        match current_block_21 {
            2838571290723028321 => {
                grid_expand_line(gd, yy, px.wrapping_add(ox), 8u32);
                xx = px;
                while xx < px.wrapping_add(ox) {
                    grid_clear_cell(gd, xx, yy, bg);
                    xx = xx.wrapping_add(1)
                }
            }
            _ => {}
        }
        yy = yy.wrapping_add(1)
    }
}
/* Clear lines. This just frees and truncates the lines. */
#[no_mangle]
pub unsafe extern "C" fn grid_clear_lines(
    mut gd: *mut grid,
    mut py: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut yy: u_int = 0;
    if ny == 0u32 {
        return;
    }
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"grid_clear_lines\x00"))
            .as_ptr(),
        py,
    ) != 0i32
    {
        return;
    }
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"grid_clear_lines\x00"))
            .as_ptr(),
        py.wrapping_add(ny).wrapping_sub(1u32),
    ) != 0i32
    {
        return;
    }
    yy = py;
    while yy < py.wrapping_add(ny) {
        grid_free_line(gd, yy);
        grid_empty_line(gd, yy, bg);
        yy = yy.wrapping_add(1)
    }
    if py != 0u32 {
        (*(*gd).linedata.offset(py.wrapping_sub(1u32) as isize)).flags &= !(0x1i32)
    };
}
/* Move a group of lines. */
#[no_mangle]
pub unsafe extern "C" fn grid_move_lines(
    mut gd: *mut grid,
    mut dy: u_int,
    mut py: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut yy: u_int = 0;
    if ny == 0u32 || py == dy {
        return;
    }
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"grid_move_lines\x00")).as_ptr(),
        py,
    ) != 0i32
    {
        return;
    }
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"grid_move_lines\x00")).as_ptr(),
        py.wrapping_add(ny).wrapping_sub(1u32),
    ) != 0i32
    {
        return;
    }
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"grid_move_lines\x00")).as_ptr(),
        dy,
    ) != 0i32
    {
        return;
    }
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"grid_move_lines\x00")).as_ptr(),
        dy.wrapping_add(ny).wrapping_sub(1u32),
    ) != 0i32
    {
        return;
    }
    /* Free any lines which are being replaced. */
    yy = dy;
    while yy < dy.wrapping_add(ny) {
        if !(yy >= py && yy < py.wrapping_add(ny)) {
            grid_free_line(gd, yy);
        }
        yy = yy.wrapping_add(1)
    }
    if dy != 0u32 {
        (*(*gd).linedata.offset(dy.wrapping_sub(1u32) as isize)).flags &= !(0x1i32)
    }
    memmove(
        &mut *(*gd).linedata.offset(dy as isize) as *mut Line as *mut libc::c_void,
        &mut *(*gd).linedata.offset(py as isize) as *mut Line as *const libc::c_void,
        (ny as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Line>() as libc::c_ulong),
    );
    /*
     * Wipe any lines that have been moved (without freeing them - they are
     * still present).
     */
    yy = py;
    while yy < py.wrapping_add(ny) {
        if yy < dy || yy >= dy.wrapping_add(ny) {
            grid_empty_line(gd, yy, bg);
        }
        yy = yy.wrapping_add(1)
    }
    if py != 0u32 && (py < dy || py >= dy.wrapping_add(ny)) {
        (*(*gd).linedata.offset(py.wrapping_sub(1u32) as isize)).flags &= !(0x1i32)
    };
}
/* Move a group of cells. */
#[no_mangle]
pub unsafe extern "C" fn grid_move_cells(
    mut gd: *mut grid,
    mut dx: u_int,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut gl: *mut Line = 0 as *mut Line;
    let mut xx: u_int = 0;
    if nx == 0u32 || px == dx {
        return;
    }
    if grid_check_y(
        gd,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"grid_move_cells\x00")).as_ptr(),
        py,
    ) != 0i32
    {
        return;
    }
    gl = &mut *(*gd).linedata.offset(py as isize) as *mut Line;
    grid_expand_line(gd, py, px.wrapping_add(nx), 8u32);
    grid_expand_line(gd, py, dx.wrapping_add(nx), 8u32);
    memmove(
        &mut *(*gl).celldata.offset(dx as isize) as *mut CellEntry as *mut libc::c_void,
        &mut *(*gl).celldata.offset(px as isize) as *mut CellEntry as *const libc::c_void,
        (nx as libc::c_ulong).wrapping_mul(::std::mem::size_of::<CellEntry>() as libc::c_ulong),
    );
    if dx.wrapping_add(nx) > (*gl).cellused {
        (*gl).cellused = dx.wrapping_add(nx)
    }
    /* Wipe any cells that have been moved. */
    xx = px;
    while xx < px.wrapping_add(nx) {
        if !(xx >= dx && xx < dx.wrapping_add(nx)) {
            grid_clear_cell(gd, xx, py, bg);
        }
        xx = xx.wrapping_add(1)
    }
}
/* Get ANSI foreground sequence. */
unsafe extern "C" fn grid_string_cells_fg(
    mut gc: *const Cell,
    mut values: *mut libc::c_int,
) -> size_t {
    let mut n: size_t = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    n = 0u64;
    if (*gc).fg & 0x1000000i32 != 0 {
        let fresh3 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh3 as isize) = 38i32;
        let fresh4 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh4 as isize) = 5i32;
        let fresh5 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh5 as isize) = (*gc).fg & 0xffi32
    } else if (*gc).fg & 0x2000000i32 != 0 {
        let fresh6 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh6 as isize) = 38i32;
        let fresh7 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh7 as isize) = 2i32;
        colour_split_rgb((*gc).fg, &mut r, &mut g, &mut b);
        let fresh8 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh8 as isize) = r as libc::c_int;
        let fresh9 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh9 as isize) = g as libc::c_int;
        let fresh10 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh10 as isize) = b as libc::c_int
    } else {
        match (*gc).fg {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                let fresh11 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh11 as isize) = (*gc).fg + 30i32
            }
            8 => {
                let fresh12 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh12 as isize) = 39i32
            }
            90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 => {
                let fresh13 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh13 as isize) = (*gc).fg
            }
            _ => {}
        }
    }
    return n;
}
/* Get ANSI background sequence. */
unsafe extern "C" fn grid_string_cells_bg(
    mut gc: *const Cell,
    mut values: *mut libc::c_int,
) -> size_t {
    let mut n: size_t = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    n = 0u64;
    if (*gc).bg & 0x1000000i32 != 0 {
        let fresh14 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh14 as isize) = 48i32;
        let fresh15 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh15 as isize) = 5i32;
        let fresh16 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh16 as isize) = (*gc).bg & 0xffi32
    } else if (*gc).bg & 0x2000000i32 != 0 {
        let fresh17 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh17 as isize) = 48i32;
        let fresh18 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh18 as isize) = 2i32;
        colour_split_rgb((*gc).bg, &mut r, &mut g, &mut b);
        let fresh19 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh19 as isize) = r as libc::c_int;
        let fresh20 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh20 as isize) = g as libc::c_int;
        let fresh21 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh21 as isize) = b as libc::c_int
    } else {
        match (*gc).bg {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                let fresh22 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh22 as isize) = (*gc).bg + 40i32
            }
            8 => {
                let fresh23 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh23 as isize) = 49i32
            }
            90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 => {
                let fresh24 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh24 as isize) = (*gc).bg + 10i32
            }
            _ => {}
        }
    }
    return n;
}
/*
 * Returns ANSI code to set particular attributes (colour, bold and so on)
 * given a current state.
 */
unsafe extern "C" fn grid_string_cells_code(
    mut lastgc: *const Cell,
    mut gc: *const Cell,
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut escape_c0: libc::c_int,
) {
    let mut oldc: [libc::c_int; 64] = [0; 64];
    let mut newc: [libc::c_int; 64] = [0; 64];
    let mut s: [libc::c_int; 128] = [0; 128];
    let mut noldc: size_t = 0;
    let mut nnewc: size_t = 0;
    let mut n: size_t = 0;
    let mut i: size_t = 0;
    let mut attr: u_int = (*gc).attr as u_int;
    let mut lastattr: u_int = (*lastgc).attr as u_int;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut attrs: [C2RustUnnamed_1; 13] = [
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x1u32,
                code: 1u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x2u32,
                code: 2u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x40u32,
                code: 3u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x4u32,
                code: 4u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x8u32,
                code: 5u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x10u32,
                code: 7u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x20u32,
                code: 8u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x100u32,
                code: 9u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x200u32,
                code: 42u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x400u32,
                code: 43u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x800u32,
                code: 44u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x1000u32,
                code: 45u32,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x2000u32,
                code: 53u32,
            };
            init
        },
    ];
    n = 0u64;
    /* If any attribute is removed, begin with 0. */
    i = 0u64;
    while i
        < (::std::mem::size_of::<[C2RustUnnamed_1; 13]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
    {
        if attr & attrs[i as usize].mask == 0 && lastattr & attrs[i as usize].mask != 0 {
            let fresh25 = n;
            n = n.wrapping_add(1);
            s[fresh25 as usize] = 0i32;
            lastattr &= 0x80u32;
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    /* For each attribute that is newly set, add its code. */
    i = 0u64;
    while i
        < (::std::mem::size_of::<[C2RustUnnamed_1; 13]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
    {
        if attr & attrs[i as usize].mask != 0 && lastattr & attrs[i as usize].mask == 0 {
            let fresh26 = n;
            n = n.wrapping_add(1);
            s[fresh26 as usize] = attrs[i as usize].code as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    /* Write the attributes. */
    *buf = '\u{0}' as libc::c_char;
    if n > 0u64 {
        if escape_c0 != 0 {
            strlcat(buf, b"\\033[\x00" as *const u8 as *const libc::c_char, len);
        } else {
            strlcat(buf, b"\x1b[\x00" as *const u8 as *const libc::c_char, len);
        }
        i = 0u64;
        while i < n {
            if s[i as usize] < 10i32 {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    s[i as usize],
                );
            } else {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d:%d\x00" as *const u8 as *const libc::c_char,
                    s[i as usize] / 10i32,
                    s[i as usize] % 10i32,
                );
            }
            strlcat(buf, tmp.as_mut_ptr(), len);
            if i.wrapping_add(1u64) < n {
                strlcat(buf, b";\x00" as *const u8 as *const libc::c_char, len);
            }
            i = i.wrapping_add(1)
        }
        strlcat(buf, b"m\x00" as *const u8 as *const libc::c_char, len);
    }
    /* If the foreground colour changed, write its parameters. */
    nnewc = grid_string_cells_fg(gc, newc.as_mut_ptr());
    noldc = grid_string_cells_fg(lastgc, oldc.as_mut_ptr());
    if nnewc != noldc
        || memcmp(
            newc.as_mut_ptr() as *const libc::c_void,
            oldc.as_mut_ptr() as *const libc::c_void,
            nnewc.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) != 0i32
        || n != 0u64 && s[0usize] == 0i32
    {
        if escape_c0 != 0 {
            strlcat(buf, b"\\033[\x00" as *const u8 as *const libc::c_char, len);
        } else {
            strlcat(buf, b"\x1b[\x00" as *const u8 as *const libc::c_char, len);
        }
        i = 0u64;
        while i < nnewc {
            if i.wrapping_add(1u64) < nnewc {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d;\x00" as *const u8 as *const libc::c_char,
                    newc[i as usize],
                );
            } else {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    newc[i as usize],
                );
            }
            strlcat(buf, tmp.as_mut_ptr(), len);
            i = i.wrapping_add(1)
        }
        strlcat(buf, b"m\x00" as *const u8 as *const libc::c_char, len);
    }
    /* If the background colour changed, append its parameters. */
    nnewc = grid_string_cells_bg(gc, newc.as_mut_ptr());
    noldc = grid_string_cells_bg(lastgc, oldc.as_mut_ptr());
    if nnewc != noldc
        || memcmp(
            newc.as_mut_ptr() as *const libc::c_void,
            oldc.as_mut_ptr() as *const libc::c_void,
            nnewc.wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) != 0i32
        || n != 0u64 && s[0usize] == 0i32
    {
        if escape_c0 != 0 {
            strlcat(buf, b"\\033[\x00" as *const u8 as *const libc::c_char, len);
        } else {
            strlcat(buf, b"\x1b[\x00" as *const u8 as *const libc::c_char, len);
        }
        i = 0u64;
        while i < nnewc {
            if i.wrapping_add(1u64) < nnewc {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d;\x00" as *const u8 as *const libc::c_char,
                    newc[i as usize],
                );
            } else {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    newc[i as usize],
                );
            }
            strlcat(buf, tmp.as_mut_ptr(), len);
            i = i.wrapping_add(1)
        }
        strlcat(buf, b"m\x00" as *const u8 as *const libc::c_char, len);
    }
    /* Append shift in/shift out if needed. */
    if attr & 0x80u32 != 0 && lastattr & 0x80u32 == 0 {
        if escape_c0 != 0 {
            strlcat(buf, b"\\016\x00" as *const u8 as *const libc::c_char, len);
        /* SO */
        } else {
            strlcat(buf, b"\x0e\x00" as *const u8 as *const libc::c_char, len);
        }
        /* SO */
    } /* SI */
    if attr & 0x80u32 == 0 && lastattr & 0x80u32 != 0 {
        if escape_c0 != 0 {
            strlcat(buf, b"\\017\x00" as *const u8 as *const libc::c_char, len);
        } else {
            strlcat(buf, b"\x0f\x00" as *const u8 as *const libc::c_char, len);
        }
        /* SI */
    };
}
/* Convert cells into a string. */
#[no_mangle]
pub unsafe extern "C" fn grid_string_cells(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut lastgc: *mut *mut Cell,
    mut with_codes: libc::c_int,
    mut escape_c0: libc::c_int,
    mut trim: libc::c_int,
) -> *mut libc::c_char {
    let mut gc: Cell = Cell {
        data: Utf8Data {
            data: [0; 21],
            have: 0,
            size: 0,
            width: 0,
        },
        attr: 0,
        flags: 0,
        fg: 0,
        bg: 0,
        us: 0,
    };
    static mut lastgc1: Cell = Cell {
        data: Utf8Data {
            data: [0; 21],
            have: 0,
            size: 0,
            width: 0,
        },
        attr: 0,
        flags: 0,
        fg: 0,
        bg: 0,
        us: 0,
    };
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: [libc::c_char; 128] = [0; 128];
    let mut len: size_t = 0;
    let mut off: size_t = 0;
    let mut size: size_t = 0;
    let mut codelen: size_t = 0;
    let mut xx: u_int = 0;
    let mut gl: *const Line = 0 as *const Line;
    if !lastgc.is_null() && (*lastgc).is_null() {
        memcpy(
            &mut lastgc1 as *mut Cell as *mut libc::c_void,
            &grid_default_cell as *const Cell as *const libc::c_void,
            ::std::mem::size_of::<Cell>() as libc::c_ulong,
        );
        *lastgc = &mut lastgc1
    }
    len = 128u64;
    buf = xmalloc(len) as *mut libc::c_char;
    off = 0u64;
    gl = grid_peek_line(gd, py);
    xx = px;
    while xx < px.wrapping_add(nx) {
        if gl.is_null() || xx >= (*gl).cellsize {
            break;
        }
        grid_get_cell(gd, xx, py, &mut gc);
        if !(gc.flags as libc::c_int & 0x4i32 != 0) {
            if with_codes != 0 {
                grid_string_cells_code(
                    *lastgc,
                    &mut gc,
                    code.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    escape_c0,
                );
                codelen = strlen(code.as_mut_ptr());
                memcpy(
                    *lastgc as *mut libc::c_void,
                    &mut gc as *mut Cell as *const libc::c_void,
                    ::std::mem::size_of::<Cell>() as libc::c_ulong,
                );
            } else {
                codelen = 0u64
            }
            data = gc.data.data.as_mut_ptr() as *const libc::c_char;
            size = gc.data.size as size_t;
            if escape_c0 != 0 && size == 1u64 && *data as libc::c_int == '\\' as i32 {
                data = b"\\\\\x00" as *const u8 as *const libc::c_char;
                size = 2u64
            }
            while len
                < off
                    .wrapping_add(size)
                    .wrapping_add(codelen)
                    .wrapping_add(1u64)
            {
                buf = xreallocarray(buf as *mut libc::c_void, 2u64, len) as *mut libc::c_char;
                len = (len).wrapping_mul(2u64)
            }
            if codelen != 0u64 {
                memcpy(
                    buf.offset(off as isize) as *mut libc::c_void,
                    code.as_mut_ptr() as *const libc::c_void,
                    codelen,
                );
                off = (off).wrapping_add(codelen)
            }
            memcpy(
                buf.offset(off as isize) as *mut libc::c_void,
                data as *const libc::c_void,
                size,
            );
            off = (off).wrapping_add(size)
        }
        xx = xx.wrapping_add(1)
    }
    if trim != 0 {
        while off > 0u64
            && *buf.offset(off.wrapping_sub(1u64) as isize) as libc::c_int == ' ' as i32
        {
            off = off.wrapping_sub(1)
        }
    }
    *buf.offset(off as isize) = '\u{0}' as libc::c_char;
    return buf;
}
/*
 * Duplicate a set of lines between two grids. Both source and destination
 * should be big enough.
 */
#[no_mangle]
pub unsafe extern "C" fn grid_duplicate_lines(
    mut dst: *mut grid,
    mut dy: u_int,
    mut src: *mut grid,
    mut sy: u_int,
    mut ny: u_int,
) {
    let mut dstl: *mut Line = 0 as *mut Line;
    let mut srcl: *mut Line = 0 as *mut Line;
    let mut yy: u_int = 0;
    if dy.wrapping_add(ny) > (*dst).hsize.wrapping_add((*dst).sy) {
        ny = (*dst).hsize.wrapping_add((*dst).sy).wrapping_sub(dy)
    }
    if sy.wrapping_add(ny) > (*src).hsize.wrapping_add((*src).sy) {
        ny = (*src).hsize.wrapping_add((*src).sy).wrapping_sub(sy)
    }
    grid_free_lines(dst, dy, ny);
    yy = 0u32;
    while yy < ny {
        srcl = &mut *(*src).linedata.offset(sy as isize) as *mut Line;
        dstl = &mut *(*dst).linedata.offset(dy as isize) as *mut Line;
        memcpy(
            dstl as *mut libc::c_void,
            srcl as *const libc::c_void,
            ::std::mem::size_of::<Line>() as libc::c_ulong,
        );
        if (*srcl).cellsize != 0u32 {
            (*dstl).celldata = xreallocarray(
                0 as *mut libc::c_void,
                (*srcl).cellsize as size_t,
                ::std::mem::size_of::<CellEntry>() as libc::c_ulong,
            ) as *mut CellEntry;
            memcpy(
                (*dstl).celldata as *mut libc::c_void,
                (*srcl).celldata as *const libc::c_void,
                ((*srcl).cellsize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<CellEntry>() as libc::c_ulong),
            );
        } else {
            (*dstl).celldata = 0 as *mut CellEntry
        }
        if (*srcl).extdsize != 0u32 {
            (*dstl).extdsize = (*srcl).extdsize;
            (*dstl).extddata = xreallocarray(
                0 as *mut libc::c_void,
                (*dstl).extdsize as size_t,
                ::std::mem::size_of::<ExtdEntry>() as libc::c_ulong,
            ) as *mut ExtdEntry;
            memcpy(
                (*dstl).extddata as *mut libc::c_void,
                (*srcl).extddata as *const libc::c_void,
                ((*dstl).extdsize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<ExtdEntry>() as libc::c_ulong),
            );
        }
        sy = sy.wrapping_add(1);
        dy = dy.wrapping_add(1);
        yy = yy.wrapping_add(1)
    }
}
/* Mark line as dead. */
unsafe extern "C" fn grid_reflow_dead(mut gl: *mut Line) {
    memset(
        gl as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<Line>() as libc::c_ulong,
    );
    (*gl).flags = 0x4i32;
}
/* Add lines, return the first new one. */
unsafe extern "C" fn grid_reflow_add(mut gd: *mut grid, mut n: u_int) -> *mut Line {
    let mut gl: *mut Line = 0 as *mut Line;
    let mut sy: u_int = (*gd).sy.wrapping_add(n);
    (*gd).linedata = xreallocarray(
        (*gd).linedata as *mut libc::c_void,
        sy as size_t,
        ::std::mem::size_of::<Line>() as libc::c_ulong,
    ) as *mut Line;
    gl = &mut *(*gd).linedata.offset((*gd).sy as isize) as *mut Line;
    memset(
        gl as *mut libc::c_void,
        0i32,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Line>() as libc::c_ulong),
    );
    (*gd).sy = sy;
    return gl;
}
/* Move a line across. */
unsafe extern "C" fn grid_reflow_move(mut gd: *mut grid, mut from: *mut Line) -> *mut Line {
    let mut to: *mut Line = 0 as *mut Line;
    to = grid_reflow_add(gd, 1u32);
    memcpy(
        to as *mut libc::c_void,
        from as *const libc::c_void,
        ::std::mem::size_of::<Line>() as libc::c_ulong,
    );
    grid_reflow_dead(from);
    return to;
}
/* Join line below onto this one. */
unsafe extern "C" fn grid_reflow_join(
    mut target: *mut grid,
    mut gd: *mut grid,
    mut sx: u_int,
    mut yy: u_int,
    mut width: u_int,
    mut already: libc::c_int,
) {
    let mut gl: *mut Line = 0 as *mut Line;
    let mut from: *mut Line = 0 as *mut Line;
    let mut gc: Cell = Cell {
        data: Utf8Data {
            data: [0; 21],
            have: 0,
            size: 0,
            width: 0,
        },
        attr: 0,
        flags: 0,
        fg: 0,
        bg: 0,
        us: 0,
    };
    let mut lines: u_int = 0;
    let mut left: u_int = 0;
    let mut i: u_int = 0;
    let mut to: u_int = 0;
    let mut line: u_int = 0;
    let mut want: u_int = 0u32;
    let mut at: u_int = 0;
    let mut wrapped: libc::c_int = 1i32;
    /*
     * Add a new target line.
     */
    if already == 0 {
        to = (*target).sy;
        gl = grid_reflow_move(target, &mut *(*gd).linedata.offset(yy as isize))
    } else {
        to = (*target).sy.wrapping_sub(1u32);
        gl = &mut *(*target).linedata.offset(to as isize) as *mut Line
    }
    at = (*gl).cellused;
    /*
     * Loop until no more to consume or the target line is full.
     */
    lines = 0u32;
    /*
     * If this is now the last line, there is nothing more to be
     * done.
     */
    while !(yy.wrapping_add(1u32).wrapping_add(lines) == (*gd).hsize.wrapping_add((*gd).sy)) {
        line = yy.wrapping_add(1u32).wrapping_add(lines);
        /* If the next line is empty, skip it. */
        if !(*(*gd).linedata.offset(line as isize)).flags & 0x1i32 != 0 {
            wrapped = 0i32
        }
        if (*(*gd).linedata.offset(line as isize)).cellused == 0u32 {
            if wrapped == 0 {
                break;
            }
            lines = lines.wrapping_add(1)
        } else {
            /*
             * Is the destination line now full? Copy the first character
             * separately because we need to leave "from" set to the last
             * line if this line is full.
             */
            grid_get_cell1(&mut *(*gd).linedata.offset(line as isize), 0u32, &mut gc);
            if width.wrapping_add(gc.data.width as libc::c_uint) > sx {
                break;
            }
            width = (width).wrapping_add(gc.data.width as libc::c_uint);
            grid_set_cell(target, at, to, &mut gc);
            at = at.wrapping_add(1);
            /* Join as much more as possible onto the current line. */
            from = &mut *(*gd).linedata.offset(line as isize) as *mut Line;
            want = 1u32;
            while want < (*from).cellused {
                grid_get_cell1(from, want, &mut gc);
                if width.wrapping_add(gc.data.width as libc::c_uint) > sx {
                    break;
                }
                width = (width).wrapping_add(gc.data.width as libc::c_uint);
                grid_set_cell(target, at, to, &mut gc);
                at = at.wrapping_add(1);
                want = want.wrapping_add(1)
            }
            lines = lines.wrapping_add(1);
            /*
             * If this line wasn't wrapped or we didn't consume the entire
             * line, don't try to join any further lines.
             */
            if wrapped == 0 || want != (*from).cellused || width == sx {
                break;
            }
        }
    }
    if lines == 0u32 {
        return;
    }
    /*
     * If we didn't consume the entire final line, then remove what we did
     * consume. If we consumed the entire line and it wasn't wrapped,
     * remove the wrap flag from this line.
     */
    left = (*from).cellused.wrapping_sub(want);
    if left != 0u32 {
        grid_move_cells(gd, 0u32, want, yy.wrapping_add(lines), left, 8u32);
        (*from).cellused = left;
        (*from).cellsize = (*from).cellused;
        lines = lines.wrapping_sub(1)
    } else if wrapped == 0 {
        (*gl).flags &= !(0x1i32)
    }
    /* Remove the lines that were completely consumed. */
    i = yy.wrapping_add(1u32);
    while i < yy.wrapping_add(1u32).wrapping_add(lines) {
        free((*(*gd).linedata.offset(i as isize)).celldata as *mut libc::c_void);
        free((*(*gd).linedata.offset(i as isize)).extddata as *mut libc::c_void);
        grid_reflow_dead(&mut *(*gd).linedata.offset(i as isize));
        i = i.wrapping_add(1)
    }
    /* Adjust scroll position. */
    if (*gd).hscrolled > to.wrapping_add(lines) {
        (*gd).hscrolled = ((*gd).hscrolled).wrapping_sub(lines)
    } else if (*gd).hscrolled > to {
        (*gd).hscrolled = to
    };
}
/* Split this line into several new ones */
unsafe extern "C" fn grid_reflow_split(
    mut target: *mut grid,
    mut gd: *mut grid,
    mut sx: u_int,
    mut yy: u_int,
    mut at: u_int,
) {
    let mut gl: *mut Line = &mut *(*gd).linedata.offset(yy as isize) as *mut Line;
    let mut first: *mut Line = 0 as *mut Line;
    let mut gc: Cell = Cell {
        data: Utf8Data {
            data: [0; 21],
            have: 0,
            size: 0,
            width: 0,
        },
        attr: 0,
        flags: 0,
        fg: 0,
        bg: 0,
        us: 0,
    };
    let mut line: u_int = 0;
    let mut lines: u_int = 0;
    let mut width: u_int = 0;
    let mut i: u_int = 0;
    let mut xx: u_int = 0;
    let mut used: u_int = (*gl).cellused;
    let mut flags: libc::c_int = (*gl).flags;
    /* How many lines do we need to insert? We know we need at least two. */
    if !(*gl).flags & 0x2i32 != 0 {
        lines = (1u32).wrapping_add((*gl).cellused.wrapping_sub(1u32).wrapping_div(sx))
    } else {
        lines = 2u32;
        width = 0u32;
        i = at;
        while i < used {
            grid_get_cell1(gl, i, &mut gc);
            if width.wrapping_add(gc.data.width as libc::c_uint) > sx {
                lines = lines.wrapping_add(1);
                width = 0u32
            }
            width = (width).wrapping_add(gc.data.width as libc::c_uint);
            i = i.wrapping_add(1)
        }
    }
    /* Insert new lines. */
    line = (*target).sy.wrapping_add(1u32);
    first = grid_reflow_add(target, lines);
    /* Copy sections from the original line. */
    width = 0u32;
    xx = 0u32;
    i = at;
    while i < used {
        grid_get_cell1(gl, i, &mut gc);
        if width.wrapping_add(gc.data.width as libc::c_uint) > sx {
            (*(*target).linedata.offset(line as isize)).flags |= 0x1i32;
            line = line.wrapping_add(1);
            width = 0u32;
            xx = 0u32
        }
        width = (width).wrapping_add(gc.data.width as libc::c_uint);
        grid_set_cell(target, xx, line, &mut gc);
        xx = xx.wrapping_add(1);
        i = i.wrapping_add(1)
    }
    if flags & 0x1i32 != 0 {
        (*(*target).linedata.offset(line as isize)).flags |= 0x1i32
    }
    /* Move the remainder of the original line. */
    (*gl).cellused = at;
    (*gl).cellsize = (*gl).cellused;
    (*gl).flags |= 0x1i32;
    memcpy(
        first as *mut libc::c_void,
        gl as *const libc::c_void,
        ::std::mem::size_of::<Line>() as libc::c_ulong,
    );
    grid_reflow_dead(gl);
    /* Adjust the scroll position. */
    if yy <= (*gd).hscrolled {
        (*gd).hscrolled = ((*gd).hscrolled).wrapping_add(lines.wrapping_sub(1u32))
    }
    /*
     * If the original line had the wrapped flag and there is still space
     * in the last new line, try to join with the next lines.
     */
    if width < sx && flags & 0x1i32 != 0 {
        grid_reflow_join(target, gd, sx, yy, width, 1i32);
    };
}
/* Reflow lines on grid to new width. */
#[no_mangle]
pub unsafe extern "C" fn grid_reflow(mut gd: *mut grid, mut sx: u_int) {
    let mut target: *mut grid = 0 as *mut grid;
    let mut gl: *mut Line = 0 as *mut Line;
    let mut gc: Cell = Cell {
        data: Utf8Data {
            data: [0; 21],
            have: 0,
            size: 0,
            width: 0,
        },
        attr: 0,
        flags: 0,
        fg: 0,
        bg: 0,
        us: 0,
    };
    let mut yy: u_int = 0;
    let mut width: u_int = 0;
    let mut i: u_int = 0;
    let mut at: u_int = 0;
    /*
     * Create a destination grid. This is just used as a container for the
     * line data and may not be fully valid.
     */
    target = grid_create((*gd).sx, 0u32, 0u32);
    /*
     * Loop over each source line.
     */
    yy = 0u32;
    while yy < (*gd).hsize.wrapping_add((*gd).sy) {
        gl = &mut *(*gd).linedata.offset(yy as isize) as *mut Line;
        if !((*gl).flags & 0x4i32 != 0) {
            /*
             * Work out the width of this line. at is the point at which
             * the available width is hit, and width is the full line
             * width.
             */
            width = 0u32;
            at = width;
            if !(*gl).flags & 0x2i32 != 0 {
                width = (*gl).cellused;
                if width > sx {
                    at = sx
                } else {
                    at = width
                }
            } else {
                i = 0u32;
                while i < (*gl).cellused {
                    grid_get_cell1(gl, i, &mut gc);
                    if at == 0u32 && width.wrapping_add(gc.data.width as libc::c_uint) > sx {
                        at = i
                    }
                    width = (width).wrapping_add(gc.data.width as libc::c_uint);
                    i = i.wrapping_add(1)
                }
            }
            /*
             * If the line is exactly right, just move it across
             * unchanged.
             */
            if width == sx {
                grid_reflow_move(target, gl);
            } else if width > sx {
                grid_reflow_split(target, gd, sx, yy, at);
            } else if (*gl).flags & 0x1i32 != 0 {
                grid_reflow_join(target, gd, sx, yy, width, 0i32);
            } else {
                grid_reflow_move(target, gl);
            }
        }
        yy = yy.wrapping_add(1)
    }
    /*
     * If the line is too big, it needs to be split, whether or not
     * it was previously wrapped.
     */
    /*
     * If the line was previously wrapped, join as much as possible
     * of the next line.
     */
    /*
     * Replace the old grid with the new.
     */
    if (*target).sy < (*gd).sy {
        grid_reflow_add(target, (*gd).sy.wrapping_sub((*target).sy));
    }
    (*gd).hsize = (*target).sy.wrapping_sub((*gd).sy);
    if (*gd).hscrolled > (*gd).hsize {
        (*gd).hscrolled = (*gd).hsize
    }
    free((*gd).linedata as *mut libc::c_void);
    (*gd).linedata = (*target).linedata;
    free(target as *mut libc::c_void);
}
/* Convert to position based on wrapped lines. */
#[no_mangle]
pub unsafe extern "C" fn grid_wrap_position(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut wx: *mut u_int,
    mut wy: *mut u_int,
) {
    let mut ax: u_int = 0u32;
    let mut ay: u_int = 0u32;
    let mut yy: u_int = 0;
    yy = 0u32;
    while yy < py {
        if (*(*gd).linedata.offset(yy as isize)).flags & 0x1i32 != 0 {
            ax = (ax).wrapping_add((*(*gd).linedata.offset(yy as isize)).cellused)
        } else {
            ax = 0u32;
            ay = ay.wrapping_add(1)
        }
        yy = yy.wrapping_add(1)
    }
    if px >= (*(*gd).linedata.offset(yy as isize)).cellused {
        ax = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32)
    } else {
        ax = (ax).wrapping_add(px)
    }
    *wx = ax;
    *wy = ay;
}
/* Convert position based on wrapped lines back. */
#[no_mangle]
pub unsafe extern "C" fn grid_unwrap_position(
    mut gd: *mut grid,
    mut px: *mut u_int,
    mut py: *mut u_int,
    mut wx: u_int,
    mut wy: u_int,
) {
    let mut yy: u_int = 0;
    let mut ay: u_int = 0u32;
    yy = 0u32;
    while yy < (*gd).hsize.wrapping_add((*gd).sy).wrapping_sub(1u32) {
        if ay == wy {
            break;
        }
        if !(*(*gd).linedata.offset(yy as isize)).flags & 0x1i32 != 0 {
            ay = ay.wrapping_add(1)
        }
        yy = yy.wrapping_add(1)
    }
    /*
     * yy is now 0 on the unwrapped line which contains wx. Walk forwards
     * until we find the end or the line now containing wx.
     */
    if wx == (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32) {
        while (*(*gd).linedata.offset(yy as isize)).flags & 0x1i32 != 0 {
            yy = yy.wrapping_add(1)
        }
        wx = (*(*gd).linedata.offset(yy as isize)).cellused
    } else {
        while (*(*gd).linedata.offset(yy as isize)).flags & 0x1i32 != 0 {
            if wx < (*(*gd).linedata.offset(yy as isize)).cellused {
                break;
            }
            wx = (wx).wrapping_sub((*(*gd).linedata.offset(yy as isize)).cellused);
            yy = yy.wrapping_add(1)
        }
    }
    *px = wx;
    *py = yy;
}
/* Get length of line. */
#[no_mangle]
pub unsafe extern "C" fn grid_line_length(mut gd: *mut grid, mut py: u_int) -> u_int {
    let mut gc: Cell = Cell {
        data: Utf8Data {
            data: [0; 21],
            have: 0,
            size: 0,
            width: 0,
        },
        attr: 0,
        flags: 0,
        fg: 0,
        bg: 0,
        us: 0,
    };
    let mut px: u_int = 0;
    px = (*grid_get_line(gd, py)).cellsize;
    if px > (*gd).sx {
        px = (*gd).sx
    }
    while px > 0u32 {
        grid_get_cell(gd, px.wrapping_sub(1u32), py, &mut gc);
        if gc.flags as libc::c_int & 0x4i32 != 0
            || gc.data.size as libc::c_int != 1i32
            || *gc.data.data.as_mut_ptr() as libc::c_int != ' ' as i32
        {
            break;
        }
        px = px.wrapping_sub(1)
    }
    return px;
}
