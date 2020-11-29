use crate::{grid::Cell as GridCell, utf8::Utf8Data};
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_set(
        _: *mut event,
        _: libc::c_int,
        _: libc::c_short,
        _: Option<
            unsafe extern "C" fn(_: libc::c_int, _: libc::c_short, _: *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
    );
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut screen);
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_puts(
        _: *mut screen_write_ctx,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn screen_write_putc(_: *mut screen_write_ctx, _: *const crate::grid::Cell, _: u_char);
    #[no_mangle]
    fn screen_write_cursormove(
        _: *mut screen_write_ctx,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_write_clearscreen(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_free(_: *mut screen);
    #[no_mangle]
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int);
    #[no_mangle]
    fn window_pane_reset_mode(_: *mut window_pane);
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_4,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
    pub ev_io: C2RustUnnamed_2,
    pub ev_signal: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub ev_signal_next: C2RustUnnamed_1,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub ev_io_next: C2RustUnnamed_3,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_4 {
    pub ev_next_with_common_timeout: C2RustUnnamed_5,
    pub min_heap_idx: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_7,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_6,
    pub evcb_arg: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_6 {
    pub evcb_callback:
        Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_short, _: *mut libc::c_void) -> ()>,
    pub evcb_selfcb:
        Option<unsafe extern "C" fn(_: *mut event_callback, _: *mut libc::c_void) -> ()>,
    pub evcb_evfinalize: Option<unsafe extern "C" fn(_: *mut event, _: *mut libc::c_void) -> ()>,
    pub evcb_cbfinalize:
        Option<unsafe extern "C" fn(_: *mut event_callback, _: *mut libc::c_void) -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bufferevent {
    pub ev_base: *mut event_base,
    pub be_ops: *const bufferevent_ops,
    pub ev_read: event,
    pub ev_write: event,
    pub input: *mut evbuffer,
    pub output: *mut evbuffer,
    pub wm_read: event_watermark,
    pub wm_write: event_watermark,
    pub readcb: bufferevent_data_cb,
    pub writecb: bufferevent_data_cb,
    pub errorcb: bufferevent_event_cb,
    pub cbarg: *mut libc::c_void,
    pub timeout_read: timeval,
    pub timeout_write: timeval,
    pub enabled: libc::c_short,
}
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short, _: *mut libc::c_void) -> ()>;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void) -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type bitstr_t = libc::c_uchar;

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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct client {
    pub name: *const libc::c_char,
    pub peer: *mut crate::proc::tmuxpeer,
    pub queue: *mut crate::cmd_queue::cmdq_list,
    pub windows: client_windows,
    pub control_state: *mut crate::control::control_state,
    pub pause_age: u_int,
    pub pid: pid_t,
    pub fd: libc::c_int,
    pub out_fd: libc::c_int,
    pub event: event,
    pub retval: libc::c_int,
    pub creation_time: timeval,
    pub activity_time: timeval,
    pub environ: *mut crate::environ::environ,
    pub jobs: *mut crate::format::format_job_tree,
    pub title: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub term_name: *mut libc::c_char,
    pub term_features: libc::c_int,
    pub term_type: *mut libc::c_char,
    pub ttyname: *mut libc::c_char,
    pub tty: tty,
    pub written: size_t,
    pub discarded: size_t,
    pub redraw: size_t,
    pub repeat_timer: event,
    pub click_timer: event,
    pub click_button: u_int,
    pub click_event: mouse_event,
    pub status: status_line,
    pub flags: uint64_t,
    pub exit_type: C2RustUnnamed_28,
    pub exit_msgtype: crate::msg::Msgtype,
    pub exit_session: *mut libc::c_char,
    pub exit_message: *mut libc::c_char,
    pub keytable: *mut key_table,
    pub redraw_panes: uint64_t,
    pub message_ignore_styles: libc::c_int,
    pub message_string: *mut libc::c_char,
    pub message_timer: event,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut crate::utf8::Utf8Data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: C2RustUnnamed_25,
    pub prompt_saved: *mut crate::utf8::Utf8Data,
    pub prompt_flags: libc::c_int,
    pub session: *mut session,
    pub last_session: *mut session,
    pub references: libc::c_int,
    pub pan_window: *mut libc::c_void,
    pub pan_ox: u_int,
    pub pan_oy: u_int,
    pub overlay_check: overlay_check_cb,
    pub overlay_mode: overlay_mode_cb,
    pub overlay_draw: overlay_draw_cb,
    pub overlay_key: overlay_key_cb,
    pub overlay_free: overlay_free_cb,
    pub overlay_data: *mut libc::c_void,
    pub overlay_timer: event,
    pub files: client_files,
    pub entry: C2RustUnnamed_8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_files {
    pub rbh_root: *mut client_file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_file {
    pub c: *mut client,
    pub references: libc::c_int,
    pub stream: libc::c_int,
    pub path: *mut libc::c_char,
    pub buffer: *mut evbuffer,
    pub event: *mut bufferevent,
    pub fd: libc::c_int,
    pub error: libc::c_int,
    pub closed: libc::c_int,
    pub cb: client_file_cb,
    pub data: *mut libc::c_void,
    pub entry: C2RustUnnamed_9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub rbe_left: *mut client_file,
    pub rbe_right: *mut client_file,
    pub rbe_parent: *mut client_file,
    pub rbe_color: libc::c_int,
}
pub type client_file_cb = Option<
    unsafe extern "C" fn(
        _: *mut client,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut evbuffer,
        _: *mut libc::c_void,
    ) -> (),
>;
pub type overlay_free_cb = Option<unsafe extern "C" fn(_: *mut client) -> ()>;
pub type overlay_key_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut key_event) -> libc::c_int>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_event {
    pub key: key_code,
    pub m: mouse_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mouse_event {
    pub valid: libc::c_int,
    pub ignore: libc::c_int,
    pub key: key_code,
    pub statusat: libc::c_int,
    pub statuslines: u_int,
    pub x: u_int,
    pub y: u_int,
    pub b: u_int,
    pub lx: u_int,
    pub ly: u_int,
    pub lb: u_int,
    pub ox: u_int,
    pub oy: u_int,
    pub s: libc::c_int,
    pub w: libc::c_int,
    pub wp: libc::c_int,
    pub sgr_type: u_int,
    pub sgr_b: u_int,
}
pub type key_code = libc::c_ulonglong;
pub type overlay_draw_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut screen_redraw_ctx) -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct screen_redraw_ctx {
    pub c: *mut client,
    pub statuslines: u_int,
    pub statustop: libc::c_int,
    pub pane_status: libc::c_int,
    pub pane_lines: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ox: u_int,
    pub oy: u_int,
}
pub type overlay_mode_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut u_int, _: *mut u_int) -> *mut screen>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct screen {
    pub title: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub titles: *mut crate::screen::screen_titles,
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
    pub saved_cell: crate::grid::Cell,
    pub saved_flags: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: *mut crate::screen::screen_sel,
    pub write_list: *mut crate::screen_write::screen_write_collect_line,
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
    pub c2rust_unnamed: C2RustUnnamed_10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_10 {
    pub offset: u_int,
    pub data: C2RustUnnamed_11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type overlay_check_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: u_int, _: u_int) -> libc::c_int>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct session {
    pub id: u_int,
    pub name: *mut libc::c_char,
    pub cwd: *const libc::c_char,
    pub creation_time: timeval,
    pub last_attached_time: timeval,
    pub activity_time: timeval,
    pub last_activity_time: timeval,
    pub lock_timer: event,
    pub curw: *mut winlink,
    pub lastw: winlink_stack,
    pub windows: winlinks,
    pub statusat: libc::c_int,
    pub statuslines: u_int,
    pub options: *mut crate::options::options,
    pub flags: libc::c_int,
    pub attached: u_int,
    pub tio: *mut termios,
    pub environ: *mut crate::environ::environ,
    pub references: libc::c_int,
    pub gentry: C2RustUnnamed_13,
    pub entry: C2RustUnnamed_12,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_12 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct winlink {
    pub idx: libc::c_int,
    pub session: *mut session,
    pub window: *mut window,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_16,
    pub wentry: C2RustUnnamed_15,
    pub sentry: C2RustUnnamed_14,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_15 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_16 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window {
    pub id: u_int,
    pub latest: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub name_event: event,
    pub name_time: timeval,
    pub alerts_timer: event,
    pub offset_timer: event,
    pub activity_time: timeval,
    pub active: *mut window_pane,
    pub last: *mut window_pane,
    pub panes: window_panes,
    pub lastlayout: libc::c_int,
    pub layout_root: *mut layout_cell,
    pub saved_layout_root: *mut layout_cell,
    pub old_layout: *mut libc::c_char,
    pub sx: u_int,
    pub sy: u_int,
    pub xpixel: u_int,
    pub ypixel: u_int,
    pub new_sx: u_int,
    pub new_sy: u_int,
    pub new_xpixel: u_int,
    pub new_ypixel: u_int,
    pub flags: libc::c_int,
    pub alerts_queued: libc::c_int,
    pub alerts_entry: C2RustUnnamed_19,
    pub options: *mut crate::options::options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_18,
    pub entry: C2RustUnnamed_17,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_17 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_18 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_19 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct layout_cell {
    pub type_0: layout_type,
    pub parent: *mut layout_cell,
    pub sx: u_int,
    pub sy: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub wp: *mut window_pane,
    pub cells: layout_cells,
    pub entry: C2RustUnnamed_20,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_20 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_pane {
    pub id: u_int,
    pub active_point: u_int,
    pub window: *mut window,
    pub options: *mut crate::options::options,
    pub layout_cell: *mut layout_cell,
    pub saved_layout_cell: *mut layout_cell,
    pub sx: u_int,
    pub sy: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub flags: libc::c_int,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub shell: *mut libc::c_char,
    pub cwd: *mut libc::c_char,
    pub pid: pid_t,
    pub tty: [libc::c_char; 32],
    pub status: libc::c_int,
    pub fd: libc::c_int,
    pub event: *mut bufferevent,
    pub offset: window_pane_offset,
    pub base_offset: size_t,
    pub resize_timer: event,
    pub force_timer: event,
    pub ictx: *mut crate::input::input_ctx,
    pub cached_gc: crate::grid::Cell,
    pub cached_active_gc: crate::grid::Cell,
    pub palette: *mut libc::c_int,
    pub pipe_fd: libc::c_int,
    pub pipe_event: *mut bufferevent,
    pub pipe_offset: window_pane_offset,
    pub screen: *mut screen,
    pub base: screen,
    pub status_screen: screen,
    pub status_size: size_t,
    pub modes: C2RustUnnamed_23,
    pub searchstr: *mut libc::c_char,
    pub searchregex: libc::c_int,
    pub written: size_t,
    pub skipped: size_t,
    pub border_gc_set: libc::c_int,
    pub border_gc: crate::grid::Cell,
    pub entry: C2RustUnnamed_22,
    pub tree_entry: C2RustUnnamed_21,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_21 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_22 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_23 {
    pub tqh_first: *mut window_mode_entry,
    pub tqh_last: *mut *mut window_mode_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_mode_entry {
    pub wp: *mut window_pane,
    pub swp: *mut window_pane,
    pub mode: *const window_mode,
    pub data: *mut libc::c_void,
    pub screen: *mut screen,
    pub prefix: u_int,
    pub entry: C2RustUnnamed_24,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_24 {
    pub tqe_next: *mut window_mode_entry,
    pub tqe_prev: *mut *mut window_mode_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_mode {
    pub name: *const libc::c_char,
    pub default_format: *const libc::c_char,
    pub init: Option<
        unsafe extern "C" fn(
            _: *mut window_mode_entry,
            _: *mut cmd_find_state,
            _: *mut args,
        ) -> *mut screen,
    >,
    pub free: Option<unsafe extern "C" fn(_: *mut window_mode_entry) -> ()>,
    pub resize: Option<unsafe extern "C" fn(_: *mut window_mode_entry, _: u_int, _: u_int) -> ()>,
    pub key: Option<
        unsafe extern "C" fn(
            _: *mut window_mode_entry,
            _: *mut client,
            _: *mut session,
            _: *mut winlink,
            _: key_code,
            _: *mut mouse_event,
        ) -> (),
    >,
    pub key_table: Option<unsafe extern "C" fn(_: *mut window_mode_entry) -> *const libc::c_char>,
    pub command: Option<
        unsafe extern "C" fn(
            _: *mut window_mode_entry,
            _: *mut client,
            _: *mut session,
            _: *mut winlink,
            _: *mut args,
            _: *mut mouse_event,
        ) -> (),
    >,
    pub formats: Option<
        unsafe extern "C" fn(_: *mut window_mode_entry, _: *mut crate::format::format_tree) -> (),
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_find_state {
    pub flags: libc::c_int,
    pub current: *mut cmd_find_state,
    pub s: *mut session,
    pub wl: *mut winlink,
    pub w: *mut window,
    pub wp: *mut window_pane,
    pub idx: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_pane_offset {
    pub used: size_t,
}
pub type layout_type = libc::c_uint;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type C2RustUnnamed_25 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_25 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_25 = 0;
pub type prompt_free_cb = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type prompt_input_cb = Option<
    unsafe extern "C" fn(
        _: *mut client,
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub default_key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed_26,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_26 {
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
    pub entry: C2RustUnnamed_27,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_27 {
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
pub type C2RustUnnamed_28 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_28 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_28 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_28 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_line {
    pub timer: event,
    pub screen: screen,
    pub active: *mut screen,
    pub references: libc::c_int,
    pub style: crate::grid::Cell,
    pub entries: [status_line_entry; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_line_entry {
    pub expanded: *mut libc::c_char,
    pub ranges: style_ranges,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct style_ranges {
    pub tqh_first: *mut style_range,
    pub tqh_last: *mut *mut style_range,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct style_range {
    pub type_0: style_range_type,
    pub argument: u_int,
    pub start: u_int,
    pub end: u_int,
    pub entry: C2RustUnnamed_29,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_29 {
    pub tqe_next: *mut style_range,
    pub tqe_prev: *mut *mut style_range,
}
pub type style_range_type = libc::c_uint;
pub const STYLE_RANGE_WINDOW: style_range_type = 3;
pub const STYLE_RANGE_RIGHT: style_range_type = 2;
pub const STYLE_RANGE_LEFT: style_range_type = 1;
pub const STYLE_RANGE_NONE: style_range_type = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty {
    pub client: *mut client,
    pub start_timer: event,
    pub sx: u_int,
    pub sy: u_int,
    pub xpixel: u_int,
    pub ypixel: u_int,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub oflag: libc::c_int,
    pub oox: u_int,
    pub ooy: u_int,
    pub osx: u_int,
    pub osy: u_int,
    pub mode: libc::c_int,
    pub rlower: u_int,
    pub rupper: u_int,
    pub rleft: u_int,
    pub rright: u_int,
    pub event_in: event,
    pub in_0: *mut evbuffer,
    pub event_out: event,
    pub out: *mut evbuffer,
    pub timer: event,
    pub discarded: size_t,
    pub tio: termios,
    pub cell: crate::grid::Cell,
    pub last_cell: crate::grid::Cell,
    pub flags: libc::c_int,
    pub term: *mut tty_term,
    pub mouse_last_x: u_int,
    pub mouse_last_y: u_int,
    pub mouse_last_b: u_int,
    pub mouse_drag_flag: libc::c_int,
    pub mouse_drag_update: Option<unsafe extern "C" fn(_: *mut client, _: *mut mouse_event) -> ()>,
    pub mouse_drag_release: Option<unsafe extern "C" fn(_: *mut client, _: *mut mouse_event) -> ()>,
    pub key_timer: event,
    pub key_tree: *mut tty_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub tty: *mut tty,
    pub features: libc::c_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut crate::tty_term::tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_30,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_30 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_windows {
    pub rbh_root: *mut client_window,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_window {
    pub window: u_int,
    pub pane: *mut window_pane,
    pub entry: C2RustUnnamed_31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_31 {
    pub rbe_left: *mut client_window,
    pub rbe_right: *mut client_window,
    pub rbe_parent: *mut client_window,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct screen_write_ctx {
    pub wp: *mut window_pane,
    pub s: *mut screen,
    pub flags: libc::c_int,
    pub init_ctx_cb: screen_write_init_ctx_cb,
    pub arg: *mut libc::c_void,
    pub item: *mut crate::screen_write::screen_write_collect_item,
    pub scrolled: u_int,
    pub bg: u_int,
    pub cells: u_int,
    pub written: u_int,
    pub skipped: u_int,
}
pub type screen_write_init_ctx_cb =
    Option<unsafe extern "C" fn(_: *mut screen_write_ctx, _: *mut tty_ctx) -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_ctx {
    pub s: *mut screen,
    pub redraw_cb: tty_ctx_redraw_cb,
    pub set_client_cb: tty_ctx_set_client_cb,
    pub arg: *mut libc::c_void,
    pub cell: *const crate::grid::Cell,
    pub wrapped: libc::c_int,
    pub num: u_int,
    pub ptr: *mut libc::c_void,
    pub ocx: u_int,
    pub ocy: u_int,
    pub orupper: u_int,
    pub orlower: u_int,
    pub xoff: u_int,
    pub yoff: u_int,
    pub rxoff: u_int,
    pub ryoff: u_int,
    pub sx: u_int,
    pub sy: u_int,
    pub bg: u_int,
    pub defaults: crate::grid::Cell,
    pub palette: *mut libc::c_int,
    pub bigger: libc::c_int,
    pub wox: u_int,
    pub woy: u_int,
    pub wsx: u_int,
    pub wsy: u_int,
}
pub type tty_ctx_set_client_cb =
    Option<unsafe extern "C" fn(_: *mut tty_ctx, _: *mut client) -> libc::c_int>;
pub type tty_ctx_redraw_cb = Option<unsafe extern "C" fn(_: *const tty_ctx) -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_clock_mode_data {
    pub screen: screen,
    pub tim: time_t,
    pub timer: event,
}
#[no_mangle]
pub static mut window_clock_mode: window_mode = {
    {
        let mut init = window_mode {
            name: b"clock-mode\x00" as *const u8 as *const libc::c_char,
            default_format: 0 as *const libc::c_char,
            init: Some(
                window_clock_init
                    as unsafe extern "C" fn(
                        _: *mut window_mode_entry,
                        _: *mut cmd_find_state,
                        _: *mut args,
                    ) -> *mut screen,
            ),
            free: Some(window_clock_free as unsafe extern "C" fn(_: *mut window_mode_entry) -> ()),
            resize: Some(
                window_clock_resize
                    as unsafe extern "C" fn(_: *mut window_mode_entry, _: u_int, _: u_int) -> (),
            ),
            key: Some(
                window_clock_key
                    as unsafe extern "C" fn(
                        _: *mut window_mode_entry,
                        _: *mut client,
                        _: *mut session,
                        _: *mut winlink,
                        _: key_code,
                        _: *mut mouse_event,
                    ) -> (),
            ),
            key_table: None,
            command: None,
            formats: None,
        };
        init
    }
};
#[no_mangle]
pub static mut window_clock_table: [[[libc::c_char; 5]; 5]; 14] = [
    [
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ],
        [
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
        ],
    ],
    [
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
        [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ],
    ],
];
unsafe extern "C" fn window_clock_timer_callback(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut wme: *mut window_mode_entry = arg as *mut window_mode_entry;
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_clock_mode_data = (*wme).data as *mut window_clock_mode_data;
    let mut now: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut then: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut t: time_t = 0;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 1 as libc::c_int as __time_t,
            tv_usec: 0,
        };
        init
    };
    event_del(&mut (*data).timer);
    event_add(&mut (*data).timer, &mut tv);
    if (*wp).modes.tqh_first != wme {
        return;
    }
    t = time(0 as *mut time_t);
    gmtime_r(&mut t, &mut now);
    gmtime_r(&mut (*data).tim, &mut then);
    if now.tm_min == then.tm_min {
        return;
    }
    (*data).tim = t;
    window_clock_draw_screen(wme);
    (*wp).flags |= 0x1 as libc::c_int;
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2009 Nicholas Marriott <nicholas.marriott@gmail.com>
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
unsafe extern "C" fn window_clock_init(
    mut wme: *mut window_mode_entry,
    mut _fs: *mut cmd_find_state,
    mut _args: *mut args,
) -> *mut screen {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_clock_mode_data = 0 as *mut window_clock_mode_data;
    let mut s: *mut screen = 0 as *mut screen;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 1 as libc::c_int as __time_t,
            tv_usec: 0,
        };
        init
    };
    data = xmalloc(::std::mem::size_of::<window_clock_mode_data>() as libc::c_ulong)
        as *mut window_clock_mode_data;
    (*wme).data = data as *mut libc::c_void;
    (*data).tim = time(0 as *mut time_t);
    event_set(
        &mut (*data).timer,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            window_clock_timer_callback
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        wme as *mut libc::c_void,
    );
    event_add(&mut (*data).timer, &mut tv);
    s = &mut (*data).screen;
    screen_init(
        s,
        (*(*wp).base.grid).sx,
        (*(*wp).base.grid).sy,
        0 as libc::c_int as u_int,
    );
    (*s).mode &= !(0x1 as libc::c_int);
    window_clock_draw_screen(wme);
    return s;
}
unsafe extern "C" fn window_clock_free(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_clock_mode_data = (*wme).data as *mut window_clock_mode_data;
    event_del(&mut (*data).timer);
    screen_free(&mut (*data).screen);
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn window_clock_resize(
    mut wme: *mut window_mode_entry,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut data: *mut window_clock_mode_data = (*wme).data as *mut window_clock_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    screen_resize(s, sx, sy, 0 as libc::c_int);
    window_clock_draw_screen(wme);
}
unsafe extern "C" fn window_clock_key(
    mut wme: *mut window_mode_entry,
    mut _c: *mut client,
    mut _s: *mut session,
    mut _wl: *mut winlink,
    mut _key: key_code,
    mut _m: *mut mouse_event,
) {
    window_pane_reset_mode((*wme).wp);
}
unsafe extern "C" fn window_clock_draw_screen(mut wme: *mut window_mode_entry) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_clock_mode_data = (*wme).data as *mut window_clock_mode_data;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut colour: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut s: *mut screen = &mut (*data).screen;
    let mut gc: GridCell = GridCell {
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
    let mut tim: [libc::c_char; 64] = [0; 64];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: time_t = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut idx: u_int = 0;
    colour = options_get_number(
        (*(*wp).window).options,
        b"clock-mode-colour\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    style = options_get_number(
        (*(*wp).window).options,
        b"clock-mode-style\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    screen_write_start(&mut ctx, s);
    t = time(0 as *mut time_t);
    tm = localtime(&mut t);
    if style == 0 as libc::c_int {
        strftime(
            tim.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%l:%M \x00" as *const u8 as *const libc::c_char,
            localtime(&mut t),
        );
        if (*tm).tm_hour >= 12 as libc::c_int {
            strlcat(
                tim.as_mut_ptr(),
                b"PM\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            );
        } else {
            strlcat(
                tim.as_mut_ptr(),
                b"AM\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            );
        }
    } else {
        strftime(
            tim.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%H:%M\x00" as *const u8 as *const libc::c_char,
            tm,
        );
    }
    screen_write_clearscreen(&mut ctx, 8 as libc::c_int as u_int);
    if ((*(*s).grid).sx as libc::c_ulong)
        < (6 as libc::c_int as libc::c_ulong).wrapping_mul(strlen(tim.as_mut_ptr()))
        || (*(*s).grid).sy < 6 as libc::c_int as libc::c_uint
    {
        if (*(*s).grid).sx as libc::c_ulong >= strlen(tim.as_mut_ptr())
            && (*(*s).grid).sy != 0 as libc::c_int as libc::c_uint
        {
            x = ((*(*s).grid)
                .sx
                .wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_sub(
                    strlen(tim.as_mut_ptr()).wrapping_div(2 as libc::c_int as libc::c_ulong),
                ) as u_int;
            y = (*(*s).grid)
                .sy
                .wrapping_div(2 as libc::c_int as libc::c_uint);
            screen_write_cursormove(
                &mut ctx,
                x as libc::c_int,
                y as libc::c_int,
                0 as libc::c_int,
            );
            memcpy(
                &mut gc as *mut GridCell as *mut libc::c_void,
                &grid_default_cell as *const GridCell as *const libc::c_void,
                ::std::mem::size_of::<GridCell>() as libc::c_ulong,
            );
            gc.flags = (gc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
            gc.fg = colour;
            screen_write_puts(
                &mut ctx as *mut screen_write_ctx,
                &mut gc as *mut GridCell,
                b"%s\x00" as *const u8 as *const libc::c_char,
                tim.as_mut_ptr(),
            );
        }
        screen_write_stop(&mut ctx);
        return;
    }
    x = ((*(*s).grid)
        .sx
        .wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_ulong)
        .wrapping_sub((3 as libc::c_int as libc::c_ulong).wrapping_mul(strlen(tim.as_mut_ptr())))
        as u_int;
    y = (*(*s).grid)
        .sy
        .wrapping_div(2 as libc::c_int as libc::c_uint)
        .wrapping_sub(3 as libc::c_int as libc::c_uint);
    memcpy(
        &mut gc as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    gc.flags = (gc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
    gc.bg = colour;
    let mut current_block_47: u64;
    ptr = tim.as_mut_ptr();
    while *ptr as libc::c_int != '\u{0}' as i32 {
        if *ptr as libc::c_int >= '0' as i32 && *ptr as libc::c_int <= '9' as i32 {
            idx = (*ptr as libc::c_int - '0' as i32) as u_int;
            current_block_47 = 12997042908615822766;
        } else if *ptr as libc::c_int == ':' as i32 {
            idx = 10 as libc::c_int as u_int;
            current_block_47 = 12997042908615822766;
        } else if *ptr as libc::c_int == 'A' as i32 {
            idx = 11 as libc::c_int as u_int;
            current_block_47 = 12997042908615822766;
        } else if *ptr as libc::c_int == 'P' as i32 {
            idx = 12 as libc::c_int as u_int;
            current_block_47 = 12997042908615822766;
        } else if *ptr as libc::c_int == 'M' as i32 {
            idx = 13 as libc::c_int as u_int;
            current_block_47 = 12997042908615822766;
        } else {
            x = (x as libc::c_uint).wrapping_add(6 as libc::c_int as libc::c_uint) as u_int
                as u_int;
            current_block_47 = 18377268871191777778;
        }
        match current_block_47 {
            12997042908615822766 => {
                j = 0 as libc::c_int as u_int;
                while j < 5 as libc::c_int as libc::c_uint {
                    i = 0 as libc::c_int as u_int;
                    while i < 5 as libc::c_int as libc::c_uint {
                        screen_write_cursormove(
                            &mut ctx,
                            x.wrapping_add(i) as libc::c_int,
                            y.wrapping_add(j) as libc::c_int,
                            0 as libc::c_int,
                        );
                        if window_clock_table[idx as usize][j as usize][i as usize] != 0 {
                            screen_write_putc(&mut ctx, &mut gc, ' ' as i32 as u_char);
                        }
                        i = i.wrapping_add(1)
                    }
                    j = j.wrapping_add(1)
                }
                x = (x as libc::c_uint).wrapping_add(6 as libc::c_int as libc::c_uint) as u_int
                    as u_int
            }
            _ => {}
        }
        ptr = ptr.offset(1)
    }
    screen_write_stop(&mut ctx);
}
