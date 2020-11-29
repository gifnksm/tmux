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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn format_create(
        _: *mut client,
        _: *mut crate::cmd_queue::cmdq_item,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut crate::format::format_tree;
    #[no_mangle]
    fn format_free(_: *mut crate::format::format_tree);
    #[no_mangle]
    fn format_expand_time(
        _: *mut crate::format::format_tree,
        _: *const libc::c_char,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn format_create_defaults(
        _: *mut crate::cmd_queue::cmdq_item,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut crate::format::format_tree;
    #[no_mangle]
    fn format_defaults(
        _: *mut crate::format::format_tree,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    );
    #[no_mangle]
    fn format_draw(
        _: *mut screen_write_ctx,
        _: *const crate::grid::Cell,
        _: u_int,
        _: *const libc::c_char,
        _: *mut style_ranges,
    );
    #[no_mangle]
    fn options_get_string(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn tty_window_offset(
        _: *mut tty,
        _: *mut u_int,
        _: *mut u_int,
        _: *mut u_int,
        _: *mut u_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn tty_reset(_: *mut tty);
    #[no_mangle]
    fn tty_cursor(_: *mut tty, _: u_int, _: u_int);
    #[no_mangle]
    fn tty_cell(
        _: *mut tty,
        _: *const crate::grid::Cell,
        _: *const crate::grid::Cell,
        _: *mut libc::c_int,
    );
    #[no_mangle]
    fn tty_update_mode(_: *mut tty, _: libc::c_int, _: *mut screen);
    #[no_mangle]
    fn tty_draw_line(
        _: *mut tty,
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: *const crate::grid::Cell,
        _: *mut libc::c_int,
    );
    #[no_mangle]
    fn tty_sync_start(_: *mut tty);
    #[no_mangle]
    fn tty_default_colours(_: *mut crate::grid::Cell, _: *mut window_pane);
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_is_marked(_: *mut session, _: *mut winlink, _: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn server_client_get_pane(_: *mut client) -> *mut window_pane;
    #[no_mangle]
    fn status_line_size(_: *mut client) -> u_int;
    #[no_mangle]
    fn status_redraw(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn status_message_redraw(_: *mut client) -> libc::c_int;
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn grid_compare(_: *mut grid, _: *mut grid) -> libc::c_int;
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut screen);
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_cursormove(
        _: *mut screen_write_ctx,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_write_cell(_: *mut screen_write_ctx, _: *const crate::grid::Cell);
    #[no_mangle]
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    #[no_mangle]
    fn screen_free(_: *mut screen);
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn status_prompt_redraw(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn utf8_set(_: *mut Utf8Data, _: u_char);
    #[no_mangle]
    fn utf8_copy(_: *mut Utf8Data, _: *const Utf8Data);
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn style_apply(
        _: *mut crate::grid::Cell,
        _: *mut crate::options::options,
        _: *const libc::c_char,
        _: *mut crate::format::format_tree,
    );
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
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub const SCREEN_REDRAW_BORDER: screen_redraw_border_type = 2;
pub type screen_redraw_border_type = libc::c_uint;
pub const SCREEN_REDRAW_INSIDE: screen_redraw_border_type = 1;
pub const SCREEN_REDRAW_OUTSIDE: screen_redraw_border_type = 0;
static mut screen_redraw_double_borders: [Utf8Data; 13] = unsafe {
    [
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 0u8,
                           width: 0u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\x91\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\x90\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\x94\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\x97\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\x9a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\x9d\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\xa6\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\xa9\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\xa0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\xa3\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\xac\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xc2\xb7\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 2u8,
                           width: 1u8,};
            init
        },
    ]
};
static mut screen_redraw_heavy_borders: [Utf8Data; 13] = unsafe {
    [
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 0u8,
                           width: 0u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x94\x83\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x94\x81\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x94\x93\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x94\x8f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x94\x97\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x94\x9b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x94\xb3\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x94\xbb\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x94\xa3\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x94\xab\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xe2\x95\x8b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 3u8,
                           width: 1u8,};
            init
        },
        {
            let mut init =
                 Utf8Data{data:
                               *::std::mem::transmute::<&[u8; 21],
                                                        &mut [u_char; 21]>(b"\xc2\xb7\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                           have: 0u8,
                           size: 2u8,
                           width: 1u8,};
            init
        },
    ]
};
/* Get cell border character. */
unsafe extern "C" fn screen_redraw_border_set(
    mut wp: *mut window_pane,
    mut pane_lines: libc::c_int,
    mut cell_type: libc::c_int,
    mut gc: *mut GridCell,
) {
    let mut idx: u_int = 0;
    match pane_lines {
        4 => {
            if cell_type == 12i32 {
                (*gc).attr = ((*gc).attr as libc::c_int | 0x80i32) as u_short;
                utf8_set(
                    &mut (*gc).data,
                    (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b" xqlkmjwvtun~\x00"))
                        [12usize] as u_char,
                );
            } else {
                (*gc).attr = ((*gc).attr as libc::c_int & !(0x80i32)) as u_short;
                if !wp.is_null() && window_pane_index(wp, &mut idx) == 0i32 {
                    utf8_set(
                        &mut (*gc).data,
                        ('0' as libc::c_uint).wrapping_add(idx.wrapping_rem(10u32)) as u_char,
                    );
                } else {
                    utf8_set(&mut (*gc).data, '*' as u_char);
                }
            }
        }
        1 => {
            (*gc).attr = ((*gc).attr as libc::c_int & !(0x80i32)) as u_short;
            utf8_copy(
                &mut (*gc).data,
                &*screen_redraw_double_borders
                    .as_ptr()
                    .offset(cell_type as isize),
            );
        }
        2 => {
            (*gc).attr = ((*gc).attr as libc::c_int & !(0x80i32)) as u_short;
            utf8_copy(
                &mut (*gc).data,
                &*screen_redraw_heavy_borders
                    .as_ptr()
                    .offset(cell_type as isize),
            );
        }
        3 => {
            (*gc).attr = ((*gc).attr as libc::c_int & !(0x80i32)) as u_short;
            utf8_set(
                &mut (*gc).data,
                (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b" |-+++++++++.\x00"))
                    [cell_type as usize] as u_char,
            );
        }
        _ => {
            (*gc).attr = ((*gc).attr as libc::c_int | 0x80i32) as u_short;
            utf8_set(
                &mut (*gc).data,
                (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b" xqlkmjwvtun~\x00"))
                    [cell_type as usize] as u_char,
            );
        }
    };
}
/* Return if window has only two panes. */
unsafe extern "C" fn screen_redraw_two_panes(
    mut w: *mut window,
    mut direction: libc::c_int,
) -> libc::c_int {
    let mut wp: *mut window_pane = 0 as *mut window_pane; /* one pane */
    wp = (*(*w).panes.tqh_first).entry.tqe_next; /* more than two panes */
    if wp.is_null() {
        return 0i32;
    }
    if !(*wp).entry.tqe_next.is_null() {
        return 0i32;
    }
    if direction == 0i32 && (*wp).xoff == 0u32 {
        return 0i32;
    }
    if direction == 1i32 && (*wp).yoff == 0u32 {
        return 0i32;
    }
    return 1i32;
}
/* Check if cell is on the border of a pane. */
unsafe extern "C" fn screen_redraw_pane_border(
    mut wp: *mut window_pane,
    mut px: u_int,
    mut py: u_int,
    mut pane_status: libc::c_int,
) -> screen_redraw_border_type {
    let mut ex: u_int = (*wp).xoff.wrapping_add((*wp).sx);
    let mut ey: u_int = (*wp).yoff.wrapping_add((*wp).sy);
    /* Inside pane. */
    if px >= (*wp).xoff && px < ex && py >= (*wp).yoff && py < ey {
        return SCREEN_REDRAW_INSIDE;
    }
    /* Left/right borders. */
    if pane_status == 0i32 {
        if screen_redraw_two_panes((*wp).window, 0i32) != 0 {
            if (*wp).xoff == 0u32 && px == (*wp).sx && py <= (*wp).sy.wrapping_div(2u32) {
                return SCREEN_REDRAW_BORDER;
            }
            if (*wp).xoff != 0u32
                && px == (*wp).xoff.wrapping_sub(1u32)
                && py > (*wp).sy.wrapping_div(2u32)
            {
                return SCREEN_REDRAW_BORDER;
            }
        } else if ((*wp).yoff == 0u32 || py >= (*wp).yoff.wrapping_sub(1u32)) && py <= ey {
            if (*wp).xoff != 0u32 && px == (*wp).xoff.wrapping_sub(1u32) {
                return SCREEN_REDRAW_BORDER;
            }
            if px == ex {
                return SCREEN_REDRAW_BORDER;
            }
        }
    } else if ((*wp).yoff == 0u32 || py >= (*wp).yoff.wrapping_sub(1u32)) && py <= ey {
        if (*wp).xoff != 0u32 && px == (*wp).xoff.wrapping_sub(1u32) {
            return SCREEN_REDRAW_BORDER;
        }
        if px == ex {
            return SCREEN_REDRAW_BORDER;
        }
    }
    /* Top/bottom borders. */
    if pane_status == 0i32 {
        if screen_redraw_two_panes((*wp).window, 1i32) != 0 {
            if (*wp).yoff == 0u32 && py == (*wp).sy && px <= (*wp).sx.wrapping_div(2u32) {
                return SCREEN_REDRAW_BORDER;
            }
            if (*wp).yoff != 0u32
                && py == (*wp).yoff.wrapping_sub(1u32)
                && px > (*wp).sx.wrapping_div(2u32)
            {
                return SCREEN_REDRAW_BORDER;
            }
        } else if ((*wp).xoff == 0u32 || px >= (*wp).xoff.wrapping_sub(1u32)) && px <= ex {
            if (*wp).yoff != 0u32 && py == (*wp).yoff.wrapping_sub(1u32) {
                return SCREEN_REDRAW_BORDER;
            }
            if py == ey {
                return SCREEN_REDRAW_BORDER;
            }
        }
    } else if pane_status == 1i32 {
        if ((*wp).xoff == 0u32 || px >= (*wp).xoff.wrapping_sub(1u32)) && px <= ex {
            if (*wp).yoff != 0u32 && py == (*wp).yoff.wrapping_sub(1u32) {
                return SCREEN_REDRAW_BORDER;
            }
        }
    } else if ((*wp).xoff == 0u32 || px >= (*wp).xoff.wrapping_sub(1u32)) && px <= ex {
        if py == ey {
            return SCREEN_REDRAW_BORDER;
        }
    }
    /* Outside pane. */
    return SCREEN_REDRAW_OUTSIDE;
}
/* Check if a cell is on a border. */
unsafe extern "C" fn screen_redraw_cell_border(
    mut c: *mut client,
    mut px: u_int,
    mut py: u_int,
    mut pane_status: libc::c_int,
) -> libc::c_int {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    /* Outside the window? */
    if px > (*w).sx || py > (*w).sy {
        return 0i32;
    }
    /* On the window border? */
    if px == (*w).sx || py == (*w).sy {
        return 1i32;
    }
    /* Check all the panes. */
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if !(window_pane_visible(wp) == 0) {
            match screen_redraw_pane_border(wp, px, py, pane_status) {
                1 => return 0i32,
                2 => return 1i32,
                0 | _ => {}
            }
        }
        wp = (*wp).entry.tqe_next
    }
    return 0i32;
}
/* Work out type of border cell from surrounding cells. */
unsafe extern "C" fn screen_redraw_type_of_cell(
    mut c: *mut client,
    mut px: u_int,
    mut py: u_int,
    mut pane_status: libc::c_int,
) -> libc::c_int {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut sx: u_int = (*w).sx;
    let mut sy: u_int = (*w).sy;
    let mut borders: libc::c_int = 0i32;
    /* Is this outside the window? */
    if px > sx || py > sy {
        return 12i32;
    }
    /*
     * Construct a bitmask of whether the cells to the left (bit 4), right,
     * top, and bottom (bit 1) of this cell are borders.
     */
    if px == 0u32 || screen_redraw_cell_border(c, px.wrapping_sub(1u32), py, pane_status) != 0 {
        borders |= 8i32
    }
    if px <= sx && screen_redraw_cell_border(c, px.wrapping_add(1u32), py, pane_status) != 0 {
        borders |= 4i32
    }
    if pane_status == 1i32 {
        if py != 0u32 && screen_redraw_cell_border(c, px, py.wrapping_sub(1u32), pane_status) != 0 {
            borders |= 2i32
        }
        if screen_redraw_cell_border(c, px, py.wrapping_add(1u32), pane_status) != 0 {
            borders |= 1i32
        }
    } else if pane_status == 2i32 {
        if py == 0u32 || screen_redraw_cell_border(c, px, py.wrapping_sub(1u32), pane_status) != 0 {
            borders |= 2i32
        }
        if py != sy.wrapping_sub(1u32)
            && screen_redraw_cell_border(c, px, py.wrapping_add(1u32), pane_status) != 0
        {
            borders |= 1i32
        }
    } else {
        if py == 0u32 || screen_redraw_cell_border(c, px, py.wrapping_sub(1u32), pane_status) != 0 {
            borders |= 2i32
        }
        if screen_redraw_cell_border(c, px, py.wrapping_add(1u32), pane_status) != 0 {
            borders |= 1i32
        }
    }
    /*
     * Figure out what kind of border this cell is. Only one bit set
     * doesn't make sense (can't have a border cell with no others
     * connected).
     */
    match borders {
        15 => {
            /* 1111, left right top bottom */
            return 11i32;
        }
        14 => {
            /* 1110, left right top */
            return 8i32;
        }
        13 => {
            /* 1101, left right bottom */
            return 7i32;
        }
        12 => {
            /* 1100, left right */
            return 2i32;
        }
        11 => {
            /* 1011, left top bottom */
            return 10i32;
        }
        10 => {
            /* 1010, left top */
            return 6i32;
        }
        9 => {
            /* 1001, left bottom */
            return 4i32;
        }
        7 => {
            /* 0111, right top bottom */
            return 9i32;
        }
        6 => {
            /* 0110, right top */
            return 5i32;
        }
        5 => {
            /* 0101, right bottom */
            return 3i32;
        }
        3 => {
            /* 0011, top bottom */
            return 1i32;
        }
        _ => {}
    }
    return 12i32;
}
/* Check if cell inside a pane. */
unsafe extern "C" fn screen_redraw_check_cell(
    mut c: *mut client,
    mut px: u_int,
    mut py: u_int,
    mut pane_status: libc::c_int,
    mut wpp: *mut *mut window_pane,
) -> libc::c_int {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut active: *mut window_pane = 0 as *mut window_pane;
    let mut border: libc::c_int = 0;
    let mut right: u_int = 0;
    let mut line: u_int = 0;
    *wpp = 0 as *mut window_pane;
    if px > (*w).sx || py > (*w).sy {
        return 12i32;
    }
    if px == (*w).sx || py == (*w).sy {
        /* window border */
        return screen_redraw_type_of_cell(c, px, py, pane_status);
    }
    if pane_status != 0i32 {
        wp = server_client_get_pane(c);
        active = wp;
        loop {
            if !(window_pane_visible(wp) == 0) {
                if pane_status == 1i32 {
                    line = (*wp).yoff.wrapping_sub(1u32)
                } else {
                    line = (*wp).yoff.wrapping_add((*wp).sy)
                }
                right = ((*wp).xoff.wrapping_add(2u32) as libc::c_ulong)
                    .wrapping_add((*wp).status_size)
                    .wrapping_sub(1u64) as u_int;
                if py == line && px >= (*wp).xoff.wrapping_add(2u32) && px <= right {
                    return 0i32;
                }
            }
            wp = (*wp).entry.tqe_next;
            if wp.is_null() {
                wp = (*w).panes.tqh_first
            }
            if !(wp != active) {
                break;
            }
        }
    }
    wp = server_client_get_pane(c);
    active = wp;
    loop {
        if !(window_pane_visible(wp) == 0) {
            *wpp = wp;
            /*
             * If definitely inside, return. If not on border, skip.
             * Otherwise work out the cell.
             */
            border = screen_redraw_pane_border(wp, px, py, pane_status) as libc::c_int;
            if border == SCREEN_REDRAW_INSIDE as libc::c_int {
                return 0i32;
            }
            if !(border == SCREEN_REDRAW_OUTSIDE as libc::c_int) {
                return screen_redraw_type_of_cell(c, px, py, pane_status);
            }
        }
        wp = (*wp).entry.tqe_next;
        if wp.is_null() {
            wp = (*w).panes.tqh_first
        }
        if !(wp != active) {
            break;
        }
    }
    return 12i32;
}
/* Check if the border of a particular pane. */
unsafe extern "C" fn screen_redraw_check_is(
    mut px: u_int,
    mut py: u_int,
    mut pane_status: libc::c_int,
    mut wp: *mut window_pane,
) -> libc::c_int {
    let mut border: screen_redraw_border_type = SCREEN_REDRAW_OUTSIDE;
    border = screen_redraw_pane_border(wp, px, py, pane_status);
    if border == SCREEN_REDRAW_BORDER {
        return 1i32;
    }
    return 0i32;
}
/* Update pane status. */
unsafe extern "C" fn screen_redraw_make_pane_status(
    mut c: *mut client,
    mut wp: *mut window_pane,
    mut rctx: *mut screen_redraw_ctx,
    mut pane_lines: libc::c_int,
) -> libc::c_int {
    let mut w: *mut window = (*wp).window;
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
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut ft: *mut crate::format::format_tree = 0 as *mut crate::format::format_tree;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pane_status: libc::c_int = (*rctx).pane_status;
    let mut width: u_int = 0;
    let mut i: u_int = 0;
    let mut cell_type: u_int = 0;
    let mut top: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
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
    let mut old: screen = screen {
        title: 0 as *mut libc::c_char,
        path: 0 as *mut libc::c_char,
        titles: 0 as *mut crate::screen::screen_titles,
        grid: 0 as *mut grid,
        cx: 0,
        cy: 0,
        cstyle: 0,
        ccolour: 0 as *mut libc::c_char,
        rupper: 0,
        rlower: 0,
        mode: 0,
        saved_cx: 0,
        saved_cy: 0,
        saved_grid: 0 as *mut grid,
        saved_cell: GridCell {
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
        },
        saved_flags: 0,
        tabs: 0 as *mut bitstr_t,
        sel: 0 as *mut crate::screen::screen_sel,
        write_list: 0 as *mut crate::screen_write::screen_write_collect_line,
    };
    ft = format_create(
        c,
        0 as *mut crate::cmd_queue::cmdq_item,
        (0x80000000u32 | (*wp).id) as libc::c_int,
        0x1i32,
    );
    format_defaults(ft, c, (*c).session, (*(*c).session).curw, wp);
    if wp == server_client_get_pane(c) {
        style_apply(
            &mut gc,
            (*w).options,
            b"pane-active-border-style\x00" as *const u8 as *const libc::c_char,
            ft,
        );
    } else {
        style_apply(
            &mut gc,
            (*w).options,
            b"pane-border-style\x00" as *const u8 as *const libc::c_char,
            ft,
        );
    }
    fmt = options_get_string(
        (*w).options,
        b"pane-border-format\x00" as *const u8 as *const libc::c_char,
    );
    expanded = format_expand_time(ft, fmt);
    if (*wp).sx < 4u32 {
        width = 0u32;
        (*wp).status_size = width as size_t
    } else {
        width = (*wp).sx.wrapping_sub(4u32);
        (*wp).status_size = width as size_t
    }
    memcpy(
        &mut old as *mut screen as *mut libc::c_void,
        &mut (*wp).status_screen as *mut screen as *const libc::c_void,
        ::std::mem::size_of::<screen>() as libc::c_ulong,
    );
    screen_init(&mut (*wp).status_screen, width, 1u32, 0u32);
    (*wp).status_screen.mode = 0i32;
    screen_write_start(&mut ctx, &mut (*wp).status_screen);
    if (*rctx).statustop != 0 {
        top = (*rctx).statuslines
    } else {
        top = 0u32
    }
    i = 0u32;
    while i < width {
        px = (*wp).xoff.wrapping_add(2u32).wrapping_add(i);
        if (*rctx).pane_status == 1i32 {
            py = top.wrapping_add((*wp).yoff).wrapping_sub(1u32)
        } else {
            py = top.wrapping_add((*wp).yoff).wrapping_add((*wp).sy)
        }
        cell_type = screen_redraw_type_of_cell(c, px, py, pane_status) as u_int;
        screen_redraw_border_set(wp, pane_lines, cell_type as libc::c_int, &mut gc);
        screen_write_cell(&mut ctx, &mut gc);
        i = i.wrapping_add(1)
    }
    gc.attr = (gc.attr as libc::c_int & !(0x80i32)) as u_short;
    screen_write_cursormove(&mut ctx, 0i32, 0i32, 0i32);
    format_draw(&mut ctx, &mut gc, width, expanded, 0 as *mut style_ranges);
    screen_write_stop(&mut ctx);
    free(expanded as *mut libc::c_void);
    format_free(ft);
    if grid_compare((*wp).status_screen.grid, old.grid) == 0i32 {
        screen_free(&mut old);
        return 0i32;
    }
    screen_free(&mut old);
    return 1i32;
}
/* Draw pane status. */
unsafe extern "C" fn screen_redraw_draw_pane_status(mut ctx: *mut screen_redraw_ctx) {
    let mut c: *mut client = (*ctx).c;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut s: *mut screen = 0 as *mut screen;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut width: u_int = 0;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    let mut size: u_int = 0;
    log_debug(
        b"%s: %s @%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
            b"screen_redraw_draw_pane_status\x00",
        ))
        .as_ptr(),
        (*c).name,
        (*w).id,
    );
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if !(window_pane_visible(wp) == 0) {
            s = &mut (*wp).status_screen;
            size = (*wp).status_size as u_int;
            if (*ctx).pane_status == 1i32 {
                yoff = (*wp).yoff.wrapping_sub(1u32)
            } else {
                yoff = (*wp).yoff.wrapping_add((*wp).sy)
            }
            xoff = (*wp).xoff.wrapping_add(2u32);
            if !(xoff.wrapping_add(size) <= (*ctx).ox
                || xoff >= (*ctx).ox.wrapping_add((*ctx).sx)
                || yoff < (*ctx).oy
                || yoff >= (*ctx).oy.wrapping_add((*ctx).sy))
            {
                if xoff >= (*ctx).ox && xoff.wrapping_add(size) <= (*ctx).ox.wrapping_add((*ctx).sx)
                {
                    /* All visible. */
                    i = 0u32;
                    x = xoff.wrapping_sub((*ctx).ox);
                    width = size
                } else if xoff < (*ctx).ox
                    && xoff.wrapping_add(size) > (*ctx).ox.wrapping_add((*ctx).sx)
                {
                    /* Both left and right not visible. */
                    i = (*ctx).ox;
                    x = 0u32;
                    width = (*ctx).sx
                } else if xoff < (*ctx).ox {
                    /* Left not visible. */
                    i = (*ctx).ox.wrapping_sub(xoff);
                    x = 0u32;
                    width = size.wrapping_sub(i)
                } else {
                    /* Right not visible. */
                    i = 0u32;
                    x = xoff.wrapping_sub((*ctx).ox);
                    width = size.wrapping_sub(x)
                }
                if (*ctx).statustop != 0 {
                    yoff = (yoff).wrapping_add((*ctx).statuslines)
                }
                tty_draw_line(
                    tty,
                    s,
                    i,
                    0u32,
                    width,
                    x,
                    yoff.wrapping_sub((*ctx).oy),
                    &grid_default_cell,
                    0 as *mut libc::c_int,
                );
            }
        }
        wp = (*wp).entry.tqe_next
    }
    tty_cursor(tty, 0u32, 0u32);
}
/* Update status line and change flags if unchanged. */
unsafe extern "C" fn screen_redraw_update(
    mut c: *mut client,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut wo: *mut crate::options::options = (*w).options;
    let mut redraw: libc::c_int = 0;
    let mut lines: libc::c_int = 0;
    let mut ctx: screen_redraw_ctx = screen_redraw_ctx {
        c: 0 as *mut client,
        statuslines: 0,
        statustop: 0,
        pane_status: 0,
        pane_lines: 0,
        sx: 0,
        sy: 0,
        ox: 0,
        oy: 0,
    };
    if !(*c).message_string.is_null() {
        redraw = status_message_redraw(c)
    } else if !(*c).prompt_string.is_null() {
        redraw = status_prompt_redraw(c)
    } else {
        redraw = status_redraw(c)
    }
    if redraw == 0 && !flags & 0x1000000i32 != 0 {
        flags &= !(0x10i32)
    }
    if (*c).overlay_draw.is_some() {
        flags |= 0x2000000i32
    }
    if options_get_number(
        wo,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) != 0i64
    {
        screen_redraw_set_context(c, &mut ctx);
        lines = options_get_number(
            wo,
            b"pane-border-lines\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        redraw = 0i32;
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if screen_redraw_make_pane_status(c, wp, &mut ctx, lines) != 0 {
                redraw = 1i32
            }
            wp = (*wp).entry.tqe_next
        }
        if redraw != 0 {
            flags |= 0x400i32
        }
    }
    return flags;
}
/* Set up redraw context. */
unsafe extern "C" fn screen_redraw_set_context(
    mut c: *mut client,
    mut ctx: *mut screen_redraw_ctx,
) {
    let mut s: *mut session = (*c).session;
    let mut oo: *mut crate::options::options = (*s).options;
    let mut w: *mut window = (*(*s).curw).window;
    let mut wo: *mut crate::options::options = (*w).options;
    let mut lines: u_int = 0;
    memset(
        ctx as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<screen_redraw_ctx>() as libc::c_ulong,
    );
    (*ctx).c = c;
    lines = status_line_size(c);
    if !(*c).message_string.is_null() || !(*c).prompt_string.is_null() {
        lines = if lines == 0u32 { 1u32 } else { lines }
    }
    if lines != 0u32
        && options_get_number(
            oo,
            b"status-position\x00" as *const u8 as *const libc::c_char,
        ) == 0i64
    {
        (*ctx).statustop = 1i32
    }
    (*ctx).statuslines = lines;
    (*ctx).pane_status = options_get_number(
        wo,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    (*ctx).pane_lines = options_get_number(
        wo,
        b"pane-border-lines\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    tty_window_offset(
        &mut (*c).tty,
        &mut (*ctx).ox,
        &mut (*ctx).oy,
        &mut (*ctx).sx,
        &mut (*ctx).sy,
    );
    log_debug(
        b"%s: %s @%u ox=%u oy=%u sx=%u sy=%u %u/%d\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
            b"screen_redraw_set_context\x00",
        ))
        .as_ptr(),
        (*c).name,
        (*w).id,
        (*ctx).ox,
        (*ctx).oy,
        (*ctx).sx,
        (*ctx).sy,
        (*ctx).statuslines,
        (*ctx).statustop,
    );
}
/* Redraw entire screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_redraw_screen(mut c: *mut client) {
    let mut ctx: screen_redraw_ctx = screen_redraw_ctx {
        c: 0 as *mut client,
        statuslines: 0,
        statustop: 0,
        pane_status: 0,
        pane_lines: 0,
        sx: 0,
        sy: 0,
        ox: 0,
        oy: 0,
    };
    let mut flags: libc::c_int = 0;
    if (*c).flags & 0x40u64 != 0 {
        return;
    }
    flags = screen_redraw_update(c, (*c).flags as libc::c_int);
    if flags & (0x8i32 | 0x10i32 | 0x1000000i32 | 0x400i32 | 0x2000000i32 | 0x20000000i32) == 0i32 {
        return;
    }
    screen_redraw_set_context(c, &mut ctx);
    tty_update_mode(&mut (*c).tty, (*c).tty.mode, 0 as *mut screen);
    tty_sync_start(&mut (*c).tty);
    if flags & (0x8i32 | 0x400i32) != 0 {
        log_debug(
            b"%s: redrawing borders\x00" as *const u8 as *const libc::c_char,
            (*c).name,
        );
        if ctx.pane_status != 0i32 {
            screen_redraw_draw_pane_status(&mut ctx);
        }
        screen_redraw_draw_borders(&mut ctx);
    }
    if flags & 0x8i32 != 0 {
        log_debug(
            b"%s: redrawing panes\x00" as *const u8 as *const libc::c_char,
            (*c).name,
        );
        screen_redraw_draw_panes(&mut ctx);
    }
    if ctx.statuslines != 0u32 && flags & (0x10i32 | 0x1000000i32) != 0 {
        log_debug(
            b"%s: redrawing status\x00" as *const u8 as *const libc::c_char,
            (*c).name,
        );
        screen_redraw_draw_status(&mut ctx);
    }
    if (*c).overlay_draw.is_some() && flags & 0x2000000i32 != 0 {
        log_debug(
            b"%s: redrawing overlay\x00" as *const u8 as *const libc::c_char,
            (*c).name,
        );
        (*c).overlay_draw.expect("non-null function pointer")(c, &mut ctx);
    }
    tty_reset(&mut (*c).tty);
}
/* Redraw a single pane. */
#[no_mangle]
pub unsafe extern "C" fn screen_redraw_pane(mut c: *mut client, mut wp: *mut window_pane) {
    let mut ctx: screen_redraw_ctx = screen_redraw_ctx {
        c: 0 as *mut client,
        statuslines: 0,
        statustop: 0,
        pane_status: 0,
        pane_lines: 0,
        sx: 0,
        sy: 0,
        ox: 0,
        oy: 0,
    };
    if (*c).overlay_draw.is_some() || window_pane_visible(wp) == 0 {
        return;
    }
    screen_redraw_set_context(c, &mut ctx);
    tty_update_mode(&mut (*c).tty, (*c).tty.mode, 0 as *mut screen);
    tty_sync_start(&mut (*c).tty);
    screen_redraw_draw_pane(&mut ctx, wp);
    tty_reset(&mut (*c).tty);
}
/* Get border cell style. */
unsafe extern "C" fn screen_redraw_draw_borders_style(
    mut ctx: *mut screen_redraw_ctx,
    mut x: u_int,
    mut y: u_int,
    mut wp: *mut window_pane,
) -> *const GridCell {
    let mut c: *mut client = (*ctx).c;
    let mut s: *mut session = (*c).session;
    let mut w: *mut window = (*(*s).curw).window;
    let mut active: *mut window_pane = server_client_get_pane(c);
    let mut oo: *mut crate::options::options = (*w).options;
    let mut ft: *mut crate::format::format_tree = 0 as *mut crate::format::format_tree;
    if (*wp).border_gc_set != 0 {
        return &mut (*wp).border_gc;
    }
    (*wp).border_gc_set = 1i32;
    ft = format_create_defaults(0 as *mut crate::cmd_queue::cmdq_item, c, s, (*s).curw, wp);
    if screen_redraw_check_is(x, y, (*ctx).pane_status, active) != 0 {
        style_apply(
            &mut (*wp).border_gc,
            oo,
            b"pane-active-border-style\x00" as *const u8 as *const libc::c_char,
            ft,
        );
    } else {
        style_apply(
            &mut (*wp).border_gc,
            oo,
            b"pane-border-style\x00" as *const u8 as *const libc::c_char,
            ft,
        );
    }
    format_free(ft);
    return &mut (*wp).border_gc;
}
/* Draw a border cell. */
unsafe extern "C" fn screen_redraw_draw_borders_cell(
    mut ctx: *mut screen_redraw_ctx,
    mut i: u_int,
    mut j: u_int,
) {
    let mut c: *mut client = (*ctx).c;
    let mut s: *mut session = (*c).session;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut cell_type: u_int = 0;
    let mut x: u_int = (*ctx).ox.wrapping_add(i);
    let mut y: u_int = (*ctx).oy.wrapping_add(j);
    let mut pane_status: libc::c_int = (*ctx).pane_status;
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
    let mut tmp: *const GridCell = 0 as *const GridCell;
    if (*c).overlay_check.is_some()
        && (*c).overlay_check.expect("non-null function pointer")(c, x, y) == 0
    {
        return;
    }
    cell_type = screen_redraw_check_cell(c, x, y, pane_status, &mut wp) as u_int;
    if cell_type == 0u32 {
        return;
    }
    if wp.is_null() {
        memcpy(
            &mut gc as *mut GridCell as *mut libc::c_void,
            &grid_default_cell as *const GridCell as *const libc::c_void,
            ::std::mem::size_of::<GridCell>() as libc::c_ulong,
        );
    } else {
        tmp = screen_redraw_draw_borders_style(ctx, x, y, wp);
        if tmp.is_null() {
            return;
        }
        memcpy(
            &mut gc as *mut GridCell as *mut libc::c_void,
            tmp as *const libc::c_void,
            ::std::mem::size_of::<GridCell>() as libc::c_ulong,
        );
        if server_is_marked(s, (*s).curw, marked_pane.wp) != 0
            && screen_redraw_check_is(x, y, pane_status, marked_pane.wp) != 0
        {
            gc.attr = (gc.attr as libc::c_int ^ 0x10i32) as u_short
        }
    }
    screen_redraw_border_set(wp, (*ctx).pane_lines, cell_type as libc::c_int, &mut gc);
    if (*ctx).statustop != 0 {
        tty_cursor(tty, i, (*ctx).statuslines.wrapping_add(j));
    } else {
        tty_cursor(tty, i, j);
    }
    tty_cell(tty, &mut gc, &grid_default_cell, 0 as *mut libc::c_int);
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
/* Draw the borders. */
unsafe extern "C" fn screen_redraw_draw_borders(mut ctx: *mut screen_redraw_ctx) {
    let mut c: *mut client = (*ctx).c;
    let mut s: *mut session = (*c).session;
    let mut w: *mut window = (*(*s).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    log_debug(
        b"%s: %s @%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
            b"screen_redraw_draw_borders\x00",
        ))
        .as_ptr(),
        (*c).name,
        (*w).id,
    );
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        (*wp).border_gc_set = 0i32;
        wp = (*wp).entry.tqe_next
    }
    j = 0u32;
    while j < (*c).tty.sy.wrapping_sub((*ctx).statuslines) {
        i = 0u32;
        while i < (*c).tty.sx {
            screen_redraw_draw_borders_cell(ctx, i, j);
            i = i.wrapping_add(1)
        }
        j = j.wrapping_add(1)
    }
}
/* Draw the panes. */
unsafe extern "C" fn screen_redraw_draw_panes(mut ctx: *mut screen_redraw_ctx) {
    let mut c: *mut client = (*ctx).c;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    log_debug(
        b"%s: %s @%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"screen_redraw_draw_panes\x00"))
            .as_ptr(),
        (*c).name,
        (*w).id,
    );
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if window_pane_visible(wp) != 0 {
            screen_redraw_draw_pane(ctx, wp);
        }
        wp = (*wp).entry.tqe_next
    }
}
/* Draw the status line. */
unsafe extern "C" fn screen_redraw_draw_status(mut ctx: *mut screen_redraw_ctx) {
    let mut c: *mut client = (*ctx).c;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut s: *mut screen = (*c).status.active;
    let mut i: u_int = 0;
    let mut y: u_int = 0;
    log_debug(
        b"%s: %s @%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
            b"screen_redraw_draw_status\x00",
        ))
        .as_ptr(),
        (*c).name,
        (*w).id,
    );
    if (*ctx).statustop != 0 {
        y = 0u32
    } else {
        y = (*c).tty.sy.wrapping_sub((*ctx).statuslines)
    }
    i = 0u32;
    while i < (*ctx).statuslines {
        tty_draw_line(
            tty,
            s,
            0u32,
            i,
            (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32),
            0u32,
            y.wrapping_add(i),
            &grid_default_cell,
            0 as *mut libc::c_int,
        );
        i = i.wrapping_add(1)
    }
}
/* Draw one pane. */
unsafe extern "C" fn screen_redraw_draw_pane(
    mut ctx: *mut screen_redraw_ctx,
    mut wp: *mut window_pane,
) {
    let mut c: *mut client = (*ctx).c;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut s: *mut screen = 0 as *mut screen;
    let mut defaults: GridCell = GridCell {
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
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut top: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut width: u_int = 0;
    log_debug(
        b"%s: %s @%u %%%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"screen_redraw_draw_pane\x00"))
            .as_ptr(),
        (*c).name,
        (*w).id,
        (*wp).id,
    );
    if (*wp).xoff.wrapping_add((*wp).sx) <= (*ctx).ox
        || (*wp).xoff >= (*ctx).ox.wrapping_add((*ctx).sx)
    {
        return;
    }
    if (*ctx).statustop != 0 {
        top = (*ctx).statuslines
    } else {
        top = 0u32
    }
    s = (*wp).screen;
    j = 0u32;
    while j < (*wp).sy {
        if !((*wp).yoff.wrapping_add(j) < (*ctx).oy
            || (*wp).yoff.wrapping_add(j) >= (*ctx).oy.wrapping_add((*ctx).sy))
        {
            y = top
                .wrapping_add((*wp).yoff)
                .wrapping_add(j)
                .wrapping_sub((*ctx).oy);
            if (*wp).xoff >= (*ctx).ox
                && (*wp).xoff.wrapping_add((*wp).sx) <= (*ctx).ox.wrapping_add((*ctx).sx)
            {
                /* All visible. */
                i = 0u32;
                x = (*wp).xoff.wrapping_sub((*ctx).ox);
                width = (*wp).sx
            } else if (*wp).xoff < (*ctx).ox
                && (*wp).xoff.wrapping_add((*wp).sx) > (*ctx).ox.wrapping_add((*ctx).sx)
            {
                /* Both left and right not visible. */
                i = (*ctx).ox;
                x = 0u32;
                width = (*ctx).sx
            } else if (*wp).xoff < (*ctx).ox {
                /* Left not visible. */
                i = (*ctx).ox.wrapping_sub((*wp).xoff);
                x = 0u32;
                width = (*wp).sx.wrapping_sub(i)
            } else {
                /* Right not visible. */
                i = 0u32;
                x = (*wp).xoff.wrapping_sub((*ctx).ox);
                width = (*ctx).sx.wrapping_sub(x)
            }
            log_debug(
                b"%s: %s %%%u line %u,%u at %u,%u, width %u\x00" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"screen_redraw_draw_pane\x00",
                ))
                .as_ptr(),
                (*c).name,
                (*wp).id,
                i,
                j,
                x,
                y,
                width,
            );
            tty_default_colours(&mut defaults, wp);
            tty_draw_line(tty, s, i, j, width, x, y, &mut defaults, (*wp).palette);
        }
        j = j.wrapping_add(1)
    }
}
