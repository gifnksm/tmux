use crate::grid::Cell as GridCell;
use ::libc;

extern "C" {
    #[no_mangle]
    fn grid_collect_history(_: *mut grid);
    #[no_mangle]
    fn grid_scroll_history(_: *mut grid, _: u_int);
    #[no_mangle]
    fn grid_scroll_history_region(_: *mut grid, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut crate::grid::Cell);
    #[no_mangle]
    fn grid_set_cell(_: *mut grid, _: u_int, _: u_int, _: *const crate::grid::Cell);
    #[no_mangle]
    fn grid_set_padding(_: *mut grid, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_set_cells(
        _: *mut grid,
        _: u_int,
        _: u_int,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
        _: size_t,
    );
    #[no_mangle]
    fn grid_get_line(_: *mut grid, _: u_int) -> *mut grid_line;
    #[no_mangle]
    fn grid_clear(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_move_lines(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_move_cells(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_string_cells(
        _: *mut grid,
        _: u_int,
        _: u_int,
        _: u_int,
        _: *mut *mut GridCell,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
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
    pub data: crate::utf8::Utf8Char,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}

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
/* Get cell. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_get_cell(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut gc: *mut GridCell,
) {
    grid_get_cell(gd, px, (*gd).hsize.wrapping_add(py), gc);
}
/* Set cell. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_set_cell(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut gc: *const GridCell,
) {
    grid_set_cell(gd, px, (*gd).hsize.wrapping_add(py), gc);
}
/* Set padding. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_set_padding(mut gd: *mut grid, mut px: u_int, mut py: u_int) {
    grid_set_padding(gd, px, (*gd).hsize.wrapping_add(py));
}
/* Set cells. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_set_cells(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut gc: *const GridCell,
    mut s: *const libc::c_char,
    mut slen: size_t,
) {
    grid_set_cells(gd, px, (*gd).hsize.wrapping_add(py), gc, s, slen);
}
/* Clear into history. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_clear_history(mut gd: *mut grid, mut bg: u_int) {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut yy: u_int = 0;
    let mut last: u_int = 0;
    /* Find the last used line. */
    last = 0 as libc::c_int as u_int;
    yy = 0 as libc::c_int as u_int;
    while yy < (*gd).sy {
        gl = grid_get_line(gd, (*gd).hsize.wrapping_add(yy));
        if (*gl).cellused != 0 as libc::c_int as libc::c_uint {
            last = yy.wrapping_add(1 as libc::c_int as libc::c_uint)
        }
        yy = yy.wrapping_add(1)
    }
    if last == 0 as libc::c_int as libc::c_uint {
        grid_view_clear(
            gd,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
            (*gd).sx,
            (*gd).sy,
            bg,
        );
        return;
    }
    /* Scroll the lines into the history. */
    yy = 0 as libc::c_int as u_int;
    while yy < last {
        grid_collect_history(gd);
        grid_scroll_history(gd, bg);
        yy = yy.wrapping_add(1)
    }
    if last < (*gd).sy {
        grid_view_clear(
            gd,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
            (*gd).sx,
            (*gd).sy.wrapping_sub(last),
            bg,
        );
    }
    (*gd).hscrolled = 0 as libc::c_int as u_int;
}
/* Clear area. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_clear(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    px = px;
    py = (*gd).hsize.wrapping_add(py);
    grid_clear(gd, px, py, nx, ny, bg);
}
/* Scroll region up. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_scroll_region_up(
    mut gd: *mut grid,
    mut rupper: u_int,
    mut rlower: u_int,
    mut bg: u_int,
) {
    if (*gd).flags & 0x1 as libc::c_int != 0 {
        grid_collect_history(gd);
        if rupper == 0 as libc::c_int as libc::c_uint
            && rlower == (*gd).sy.wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            grid_scroll_history(gd, bg);
        } else {
            rupper = (*gd).hsize.wrapping_add(rupper);
            rlower = (*gd).hsize.wrapping_add(rlower);
            grid_scroll_history_region(gd, rupper, rlower, bg);
        }
    } else {
        rupper = (*gd).hsize.wrapping_add(rupper);
        rlower = (*gd).hsize.wrapping_add(rlower);
        grid_move_lines(
            gd,
            rupper,
            rupper.wrapping_add(1 as libc::c_int as libc::c_uint),
            rlower.wrapping_sub(rupper),
            bg,
        );
    };
}
/* Scroll region down. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_scroll_region_down(
    mut gd: *mut grid,
    mut rupper: u_int,
    mut rlower: u_int,
    mut bg: u_int,
) {
    rupper = (*gd).hsize.wrapping_add(rupper);
    rlower = (*gd).hsize.wrapping_add(rlower);
    grid_move_lines(
        gd,
        rupper.wrapping_add(1 as libc::c_int as libc::c_uint),
        rupper,
        rlower.wrapping_sub(rupper),
        bg,
    );
}
/* Insert lines. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_insert_lines(
    mut gd: *mut grid,
    mut py: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut sy: u_int = 0;
    py = (*gd).hsize.wrapping_add(py);
    sy = (*gd).hsize.wrapping_add((*gd).sy);
    grid_move_lines(
        gd,
        py.wrapping_add(ny),
        py,
        sy.wrapping_sub(py).wrapping_sub(ny),
        bg,
    );
}
/* Insert lines in region. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_insert_lines_region(
    mut gd: *mut grid,
    mut rlower: u_int,
    mut py: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut ny2: u_int = 0;
    rlower = (*gd).hsize.wrapping_add(rlower);
    py = (*gd).hsize.wrapping_add(py);
    ny2 = rlower
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_sub(py)
        .wrapping_sub(ny);
    grid_move_lines(
        gd,
        rlower
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(ny2),
        py,
        ny2,
        bg,
    );
    grid_clear(
        gd,
        0 as libc::c_int as u_int,
        py.wrapping_add(ny2),
        (*gd).sx,
        ny.wrapping_sub(ny2),
        bg,
    );
}
/* Delete lines. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_delete_lines(
    mut gd: *mut grid,
    mut py: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut sy: u_int = 0;
    py = (*gd).hsize.wrapping_add(py);
    sy = (*gd).hsize.wrapping_add((*gd).sy);
    grid_move_lines(
        gd,
        py,
        py.wrapping_add(ny),
        sy.wrapping_sub(py).wrapping_sub(ny),
        bg,
    );
    grid_clear(
        gd,
        0 as libc::c_int as u_int,
        sy.wrapping_sub(ny),
        (*gd).sx,
        py.wrapping_add(ny).wrapping_sub(sy.wrapping_sub(ny)),
        bg,
    );
}
/* Delete lines inside scroll region. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_delete_lines_region(
    mut gd: *mut grid,
    mut rlower: u_int,
    mut py: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut ny2: u_int = 0;
    rlower = (*gd).hsize.wrapping_add(rlower);
    py = (*gd).hsize.wrapping_add(py);
    ny2 = rlower
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_sub(py)
        .wrapping_sub(ny);
    grid_move_lines(gd, py, py.wrapping_add(ny), ny2, bg);
    grid_clear(
        gd,
        0 as libc::c_int as u_int,
        py.wrapping_add(ny2),
        (*gd).sx,
        ny.wrapping_sub(ny2),
        bg,
    );
}
/* Insert characters. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_insert_cells(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut sx: u_int = 0;
    px = px;
    py = (*gd).hsize.wrapping_add(py);
    sx = (*gd).sx;
    if px >= sx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        grid_clear(
            gd,
            px,
            py,
            1 as libc::c_int as u_int,
            1 as libc::c_int as u_int,
            bg,
        );
    } else {
        grid_move_cells(
            gd,
            px.wrapping_add(nx),
            px,
            py,
            sx.wrapping_sub(px).wrapping_sub(nx),
            bg,
        );
    };
}
/* Delete characters. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_delete_cells(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut sx: u_int = 0;
    px = px;
    py = (*gd).hsize.wrapping_add(py);
    sx = (*gd).sx;
    grid_move_cells(
        gd,
        px,
        px.wrapping_add(nx),
        py,
        sx.wrapping_sub(px).wrapping_sub(nx),
        bg,
    );
    grid_clear(
        gd,
        sx.wrapping_sub(nx),
        py,
        px.wrapping_add(nx).wrapping_sub(sx.wrapping_sub(nx)),
        1 as libc::c_int as u_int,
        bg,
    );
}
/* Convert cells into a string. */
#[no_mangle]
pub unsafe extern "C" fn grid_view_string_cells(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
) -> *mut libc::c_char {
    px = px;
    py = (*gd).hsize.wrapping_add(py);
    return grid_string_cells(
        gd,
        px,
        py,
        nx,
        0 as *mut *mut GridCell,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
