use crate::{
    grid::Cell as GridCell,
    utf8::{utf8_state, Utf8Data, Utf8State},
};
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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xstrndup(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn format_skip(_: *const libc::c_char, _: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn screen_write_cursormove(
        _: *mut screen_write_ctx,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_free(_: *mut screen);
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_clearendofline(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_cell(_: *mut screen_write_ctx, _: *const crate::grid::Cell);
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn screen_write_putc(_: *mut screen_write_ctx, _: *const crate::grid::Cell, _: u_char);
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut screen);
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_fast_copy(
        _: *mut screen_write_ctx,
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
    );
    #[no_mangle]
    fn utf8_open(_: *mut Utf8Data, _: u_char) -> crate::utf8::Utf8State;
    #[no_mangle]
    fn utf8_append(_: *mut Utf8Data, _: u_char) -> crate::utf8::Utf8State;
    #[no_mangle]
    fn utf8_set(_: *mut Utf8Data, _: u_char);
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn style_tostring(_: *mut style) -> *const libc::c_char;
    #[no_mangle]
    fn style_parse(
        _: *mut style,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn style_set(_: *mut style, _: *const crate::grid::Cell);
    #[no_mangle]
    fn style_copy(_: *mut style, _: *mut style);
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
    pub linedata: *mut crate::grid::Line,
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
pub type style_align = libc::c_uint;
pub const STYLE_ALIGN_RIGHT: style_align = 3;
pub const STYLE_ALIGN_CENTRE: style_align = 2;
pub const STYLE_ALIGN_LEFT: style_align = 1;
pub const STYLE_ALIGN_DEFAULT: style_align = 0;
pub type style_list = libc::c_uint;
pub const STYLE_LIST_RIGHT_MARKER: style_list = 4;
pub const STYLE_LIST_LEFT_MARKER: style_list = 3;
pub const STYLE_LIST_FOCUS: style_list = 2;
pub const STYLE_LIST_ON: style_list = 1;
pub const STYLE_LIST_OFF: style_list = 0;
pub type style_default_type = libc::c_uint;
pub const STYLE_DEFAULT_POP: style_default_type = 2;
pub const STYLE_DEFAULT_PUSH: style_default_type = 1;
pub const STYLE_DEFAULT_BASE: style_default_type = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct style {
    pub gc: crate::grid::Cell,
    pub ignore: libc::c_int,
    pub fill: libc::c_int,
    pub align: style_align,
    pub list: style_list,
    pub range_type: style_range_type,
    pub range_argument: u_int,
    pub default_type: style_default_type,
}
pub const TOTAL: C2RustUnnamed_33 = 7;
/* $OpenBSD$ */
/*
 * Copyright (c) 2019 Nicholas Marriott <nicholas.marriott@gmail.com>
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
/* Format range. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_range {
    pub index: u_int,
    pub s: *mut screen,
    pub start: u_int,
    pub end: u_int,
    pub type_0: style_range_type,
    pub argument: u_int,
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub tqe_next: *mut format_range,
    pub tqe_prev: *mut *mut format_range,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_ranges {
    pub tqh_first: *mut format_range,
    pub tqh_last: *mut *mut format_range,
}
pub const AFTER: C2RustUnnamed_33 = 6;
pub const LIST_RIGHT: C2RustUnnamed_33 = 5;
pub const LIST_LEFT: C2RustUnnamed_33 = 4;
pub const LIST: C2RustUnnamed_33 = 3;
pub const RIGHT: C2RustUnnamed_33 = 2;
pub const CENTRE: C2RustUnnamed_33 = 1;
pub const LEFT: C2RustUnnamed_33 = 0;
pub type C2RustUnnamed_33 = libc::c_uint;
/* Does this range match this style? */
unsafe extern "C" fn format_is_type(mut fr: *mut format_range, mut sy: *mut style) -> libc::c_int {
    if (*fr).type_0 != (*sy).range_type {
        return 0i32;
    }
    if (*fr).type_0 == STYLE_RANGE_WINDOW && (*fr).argument != (*sy).range_argument {
        return 0i32;
    }
    return 1i32;
}
/* Free a range. */
unsafe extern "C" fn format_free_range(mut frs: *mut format_ranges, mut fr: *mut format_range) {
    if !(*fr).entry.tqe_next.is_null() {
        (*(*fr).entry.tqe_next).entry.tqe_prev = (*fr).entry.tqe_prev
    } else {
        (*frs).tqh_last = (*fr).entry.tqe_prev
    }
    *(*fr).entry.tqe_prev = (*fr).entry.tqe_next;
    free(fr as *mut libc::c_void);
}
/* Fix range positions. */
unsafe extern "C" fn format_update_ranges(
    mut frs: *mut format_ranges,
    mut s: *mut screen,
    mut offset: u_int,
    mut start: u_int,
    mut width: u_int,
) {
    let mut fr: *mut format_range = 0 as *mut format_range;
    let mut fr1: *mut format_range = 0 as *mut format_range;
    if frs.is_null() {
        return;
    }
    fr = (*frs).tqh_first;
    while !fr.is_null() && {
        fr1 = (*fr).entry.tqe_next;
        (1i32) != 0
    } {
        if !((*fr).s != s) {
            if (*fr).end <= start || (*fr).start >= start.wrapping_add(width) {
                format_free_range(frs, fr);
            } else {
                if (*fr).start < start {
                    (*fr).start = start
                }
                if (*fr).end > start.wrapping_add(width) {
                    (*fr).end = start.wrapping_add(width)
                }
                if (*fr).start == (*fr).end {
                    format_free_range(frs, fr);
                } else {
                    (*fr).start = ((*fr).start).wrapping_sub(start);
                    (*fr).end = ((*fr).end).wrapping_sub(start);
                    (*fr).start = ((*fr).start).wrapping_add(offset);
                    (*fr).end = ((*fr).end).wrapping_add(offset)
                }
            }
        }
        fr = fr1
    }
}
/* Draw a part of the format. */
unsafe extern "C" fn format_draw_put(
    mut octx: *mut screen_write_ctx,
    mut ocx: u_int,
    mut ocy: u_int,
    mut s: *mut screen,
    mut frs: *mut format_ranges,
    mut offset: u_int,
    mut start: u_int,
    mut width: u_int,
) {
    /*
     * The offset is how far from the cursor on the target screen; start
     * and width how much to copy from the source screen.
     */
    screen_write_cursormove(
        octx,
        ocx.wrapping_add(offset) as libc::c_int,
        ocy as libc::c_int,
        0i32,
    );
    screen_write_fast_copy(octx, s, start, 0u32, width, 1u32);
    format_update_ranges(frs, s, offset, start, width);
}
/* Draw list part of format. */
unsafe extern "C" fn format_draw_put_list(
    mut octx: *mut screen_write_ctx,
    mut ocx: u_int,
    mut ocy: u_int,
    mut offset: u_int,
    mut width: u_int,
    mut list: *mut screen,
    mut list_left: *mut screen,
    mut list_right: *mut screen,
    mut focus_start: libc::c_int,
    mut focus_end: libc::c_int,
    mut frs: *mut format_ranges,
) {
    let mut start: u_int = 0;
    let mut focus_centre: u_int = 0;
    /* If there is enough space for the list, draw it entirely. */
    if width >= (*list).cx {
        format_draw_put(octx, ocx, ocy, list, frs, offset, 0u32, width);
        return;
    }
    /* The list needs to be trimmed. Try to keep the focus visible. */
    focus_centre = (focus_start + (focus_end - focus_start) / 2i32) as u_int;
    if focus_centre < width.wrapping_div(2u32) {
        start = 0u32
    } else {
        start = focus_centre.wrapping_sub(width.wrapping_div(2u32))
    }
    if start.wrapping_add(width) > (*list).cx {
        start = (*list).cx.wrapping_sub(width)
    }
    /* Draw <> markers at either side if needed. */
    if start != 0u32 && width > (*list_left).cx {
        screen_write_cursormove(
            octx,
            ocx.wrapping_add(offset) as libc::c_int,
            ocy as libc::c_int,
            0i32,
        );
        screen_write_fast_copy(octx, list_left, 0u32, 0u32, (*list_left).cx, 1u32);
        offset = (offset).wrapping_add((*list_left).cx);
        start = (start).wrapping_add((*list_left).cx);
        width = (width).wrapping_sub((*list_left).cx)
    }
    if start.wrapping_add(width) < (*list).cx && width > (*list_right).cx {
        screen_write_cursormove(
            octx,
            ocx.wrapping_add(offset)
                .wrapping_add(width)
                .wrapping_sub((*list_right).cx) as libc::c_int,
            ocy as libc::c_int,
            0i32,
        );
        screen_write_fast_copy(octx, list_right, 0u32, 0u32, (*list_right).cx, 1u32);
        width = (width).wrapping_sub((*list_right).cx)
    }
    /* Draw the list screen itself. */
    format_draw_put(octx, ocx, ocy, list, frs, offset, start, width);
}
/* Draw format with no list. */
unsafe extern "C" fn format_draw_none(
    mut octx: *mut screen_write_ctx,
    mut available: u_int,
    mut ocx: u_int,
    mut ocy: u_int,
    mut left: *mut screen,
    mut centre: *mut screen,
    mut right: *mut screen,
    mut frs: *mut format_ranges,
) {
    let mut width_left: u_int = 0;
    let mut width_centre: u_int = 0;
    let mut width_right: u_int = 0;
    width_left = (*left).cx;
    width_centre = (*centre).cx;
    width_right = (*right).cx;
    /*
     * Try to keep as much of the left and right as possible at the expense
     * of the centre.
     */
    while width_left
        .wrapping_add(width_centre)
        .wrapping_add(width_right)
        > available
    {
        if width_centre > 0u32 {
            width_centre = width_centre.wrapping_sub(1)
        } else if width_right > 0u32 {
            width_right = width_right.wrapping_sub(1)
        } else {
            width_left = width_left.wrapping_sub(1)
        }
    }
    /* Write left. */
    format_draw_put(octx, ocx, ocy, left, frs, 0u32, 0u32, width_left);
    /* Write right at available - width_right. */
    format_draw_put(
        octx,
        ocx,
        ocy,
        right,
        frs,
        available.wrapping_sub(width_right),
        (*right).cx.wrapping_sub(width_right),
        width_right,
    );
    /*
     * Write centre halfway between
     *     width_left
     * and
     *     available - width_right.
     */
    format_draw_put(
        octx,
        ocx,
        ocy,
        centre,
        frs,
        width_left
            .wrapping_add(
                available
                    .wrapping_sub(width_right)
                    .wrapping_sub(width_left)
                    .wrapping_div(2u32),
            )
            .wrapping_sub(width_centre.wrapping_div(2u32)),
        (*centre)
            .cx
            .wrapping_div(2u32)
            .wrapping_sub(width_centre.wrapping_div(2u32)),
        width_centre,
    );
}
/* Draw format with list on the left. */
unsafe extern "C" fn format_draw_left(
    mut octx: *mut screen_write_ctx,
    mut available: u_int,
    mut ocx: u_int,
    mut ocy: u_int,
    mut left: *mut screen,
    mut centre: *mut screen,
    mut right: *mut screen,
    mut list: *mut screen,
    mut list_left: *mut screen,
    mut list_right: *mut screen,
    mut after: *mut screen,
    mut focus_start: libc::c_int,
    mut focus_end: libc::c_int,
    mut frs: *mut format_ranges,
) {
    let mut width_left: u_int = 0;
    let mut width_centre: u_int = 0;
    let mut width_right: u_int = 0;
    let mut width_list: u_int = 0;
    let mut width_after: u_int = 0;
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
    width_left = (*left).cx;
    width_centre = (*centre).cx;
    width_right = (*right).cx;
    width_list = (*list).cx;
    width_after = (*after).cx;
    /*
     * Trim first the centre, then the list, then the right, then after the
     * list, then the left.
     */
    while width_left
        .wrapping_add(width_centre)
        .wrapping_add(width_right)
        .wrapping_add(width_list)
        .wrapping_add(width_after)
        > available
    {
        if width_centre > 0u32 {
            width_centre = width_centre.wrapping_sub(1)
        } else if width_list > 0u32 {
            width_list = width_list.wrapping_sub(1)
        } else if width_right > 0u32 {
            width_right = width_right.wrapping_sub(1)
        } else if width_after > 0u32 {
            width_after = width_after.wrapping_sub(1)
        } else {
            width_left = width_left.wrapping_sub(1)
        }
    }
    /* If there is no list left, pass off to the no list function. */
    if width_list == 0u32 {
        screen_write_start(&mut ctx, left);
        screen_write_fast_copy(&mut ctx, after, 0u32, 0u32, width_after, 1u32);
        screen_write_stop(&mut ctx);
        format_draw_none(octx, available, ocx, ocy, left, centre, right, frs);
        return;
    }
    /* Write left at 0. */
    format_draw_put(octx, ocx, ocy, left, frs, 0u32, 0u32, width_left);
    /* Write right at available - width_right. */
    format_draw_put(
        octx,
        ocx,
        ocy,
        right,
        frs,
        available.wrapping_sub(width_right),
        (*right).cx.wrapping_sub(width_right),
        width_right,
    );
    /* Write after at width_left + width_list. */
    format_draw_put(
        octx,
        ocx,
        ocy,
        after,
        frs,
        width_left.wrapping_add(width_list),
        0u32,
        width_after,
    );
    /*
     * Write centre halfway between
     *     width_left + width_list + width_after
     * and
     *     available - width_right.
     */
    format_draw_put(
        octx,
        ocx,
        ocy,
        centre,
        frs,
        width_left
            .wrapping_add(width_list)
            .wrapping_add(width_after)
            .wrapping_add(
                available
                    .wrapping_sub(width_right)
                    .wrapping_sub(
                        width_left
                            .wrapping_add(width_list)
                            .wrapping_add(width_after),
                    )
                    .wrapping_div(2u32),
            )
            .wrapping_sub(width_centre.wrapping_div(2u32)),
        (*centre)
            .cx
            .wrapping_div(2u32)
            .wrapping_sub(width_centre.wrapping_div(2u32)),
        width_centre,
    );
    /*
     * The list now goes from
     *     width_left
     * to
     *     width_left + width_list.
     * If there is no focus given, keep the left in focus.
     */
    if focus_start == -(1i32) || focus_end == -(1i32) {
        focus_end = 0i32;
        focus_start = focus_end
    }
    format_draw_put_list(
        octx,
        ocx,
        ocy,
        width_left,
        width_list,
        list,
        list_left,
        list_right,
        focus_start,
        focus_end,
        frs,
    );
}
/* Draw format with list in the centre. */
unsafe extern "C" fn format_draw_centre(
    mut octx: *mut screen_write_ctx,
    mut available: u_int,
    mut ocx: u_int,
    mut ocy: u_int,
    mut left: *mut screen,
    mut centre: *mut screen,
    mut right: *mut screen,
    mut list: *mut screen,
    mut list_left: *mut screen,
    mut list_right: *mut screen,
    mut after: *mut screen,
    mut focus_start: libc::c_int,
    mut focus_end: libc::c_int,
    mut frs: *mut format_ranges,
) {
    let mut width_left: u_int = 0;
    let mut width_centre: u_int = 0;
    let mut width_right: u_int = 0;
    let mut width_list: u_int = 0;
    let mut width_after: u_int = 0;
    let mut middle: u_int = 0;
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
    width_left = (*left).cx;
    width_centre = (*centre).cx;
    width_right = (*right).cx;
    width_list = (*list).cx;
    width_after = (*after).cx;
    /*
     * Trim first the list, then after the list, then the centre, then the
     * right, then the left.
     */
    while width_left
        .wrapping_add(width_centre)
        .wrapping_add(width_right)
        .wrapping_add(width_list)
        .wrapping_add(width_after)
        > available
    {
        if width_list > 0u32 {
            width_list = width_list.wrapping_sub(1)
        } else if width_after > 0u32 {
            width_after = width_after.wrapping_sub(1)
        } else if width_centre > 0u32 {
            width_centre = width_centre.wrapping_sub(1)
        } else if width_right > 0u32 {
            width_right = width_right.wrapping_sub(1)
        } else {
            width_left = width_left.wrapping_sub(1)
        }
    }
    /* If there is no list left, pass off to the no list function. */
    if width_list == 0u32 {
        screen_write_start(&mut ctx, centre);
        screen_write_fast_copy(&mut ctx, after, 0u32, 0u32, width_after, 1u32);
        screen_write_stop(&mut ctx);
        format_draw_none(octx, available, ocx, ocy, left, centre, right, frs);
        return;
    }
    /* Write left at 0. */
    format_draw_put(octx, ocx, ocy, left, frs, 0u32, 0u32, width_left);
    /* Write right at available - width_right. */
    format_draw_put(
        octx,
        ocx,
        ocy,
        right,
        frs,
        available.wrapping_sub(width_right),
        (*right).cx.wrapping_sub(width_right),
        width_right,
    );
    /*
     * All three centre sections are offset from the middle of the
     * available space.
     */
    middle = width_left.wrapping_add(
        available
            .wrapping_sub(width_right)
            .wrapping_sub(width_left)
            .wrapping_div(2u32),
    );
    /*
     * Write centre at
     *     middle - width_list / 2 - width_centre.
     */
    format_draw_put(
        octx,
        ocx,
        ocy,
        centre,
        frs,
        middle
            .wrapping_sub(width_list.wrapping_div(2u32))
            .wrapping_sub(width_centre),
        0u32,
        width_centre,
    );
    /*
     * Write after at
     *     middle - width_list / 2 + width_list
     */
    format_draw_put(
        octx,
        ocx,
        ocy,
        after,
        frs,
        middle
            .wrapping_sub(width_list.wrapping_div(2u32))
            .wrapping_add(width_list),
        0u32,
        width_after,
    );
    /*
     * The list now goes from
     *     middle - width_list / 2
     * to
     *     middle + width_list / 2
     * If there is no focus given, keep the centre in focus.
     */
    if focus_start == -(1i32) || focus_end == -(1i32) {
        focus_end = (*list).cx.wrapping_div(2u32) as libc::c_int;
        focus_start = focus_end
    }
    format_draw_put_list(
        octx,
        ocx,
        ocy,
        middle.wrapping_sub(width_list.wrapping_div(2u32)),
        width_list,
        list,
        list_left,
        list_right,
        focus_start,
        focus_end,
        frs,
    );
}
/* Draw format with list on the right. */
unsafe extern "C" fn format_draw_right(
    mut octx: *mut screen_write_ctx,
    mut available: u_int,
    mut ocx: u_int,
    mut ocy: u_int,
    mut left: *mut screen,
    mut centre: *mut screen,
    mut right: *mut screen,
    mut list: *mut screen,
    mut list_left: *mut screen,
    mut list_right: *mut screen,
    mut after: *mut screen,
    mut focus_start: libc::c_int,
    mut focus_end: libc::c_int,
    mut frs: *mut format_ranges,
) {
    let mut width_left: u_int = 0;
    let mut width_centre: u_int = 0;
    let mut width_right: u_int = 0;
    let mut width_list: u_int = 0;
    let mut width_after: u_int = 0;
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
    width_left = (*left).cx;
    width_centre = (*centre).cx;
    width_right = (*right).cx;
    width_list = (*list).cx;
    width_after = (*after).cx;
    /*
     * Trim first the centre, then the list, then the right, then
     * after the list, then the left.
     */
    while width_left
        .wrapping_add(width_centre)
        .wrapping_add(width_right)
        .wrapping_add(width_list)
        .wrapping_add(width_after)
        > available
    {
        if width_centre > 0u32 {
            width_centre = width_centre.wrapping_sub(1)
        } else if width_list > 0u32 {
            width_list = width_list.wrapping_sub(1)
        } else if width_right > 0u32 {
            width_right = width_right.wrapping_sub(1)
        } else if width_after > 0u32 {
            width_after = width_after.wrapping_sub(1)
        } else {
            width_left = width_left.wrapping_sub(1)
        }
    }
    /* If there is no list left, pass off to the no list function. */
    if width_list == 0u32 {
        screen_write_start(&mut ctx, right);
        screen_write_fast_copy(&mut ctx, after, 0u32, 0u32, width_after, 1u32);
        screen_write_stop(&mut ctx);
        format_draw_none(octx, available, ocx, ocy, left, centre, right, frs);
        return;
    }
    /* Write left at 0. */
    format_draw_put(octx, ocx, ocy, left, frs, 0u32, 0u32, width_left);
    /* Write after at available - width_after. */
    format_draw_put(
        octx,
        ocx,
        ocy,
        after,
        frs,
        available.wrapping_sub(width_after),
        (*after).cx.wrapping_sub(width_after),
        width_after,
    );
    /*
     * Write right at
     *     available - width_right - width_list - width_after.
     */
    format_draw_put(
        octx,
        ocx,
        ocy,
        right,
        frs,
        available
            .wrapping_sub(width_right)
            .wrapping_sub(width_list)
            .wrapping_sub(width_after),
        0u32,
        width_right,
    );
    /*
     * Write centre halfway between
     *     width_left
     * and
     *     available - width_right - width_list - width_after.
     */
    format_draw_put(
        octx,
        ocx,
        ocy,
        centre,
        frs,
        width_left
            .wrapping_add(
                available
                    .wrapping_sub(width_right)
                    .wrapping_sub(width_list)
                    .wrapping_sub(width_after)
                    .wrapping_sub(width_left)
                    .wrapping_div(2u32),
            )
            .wrapping_sub(width_centre.wrapping_div(2u32)),
        (*centre)
            .cx
            .wrapping_div(2u32)
            .wrapping_sub(width_centre.wrapping_div(2u32)),
        width_centre,
    );
    /*
     * The list now goes from
     *     available - width_list - width_after
     * to
     *     available - width_after
     * If there is no focus given, keep the right in focus.
     */
    if focus_start == -(1i32) || focus_end == -(1i32) {
        focus_end = 0i32;
        focus_start = focus_end
    }
    format_draw_put_list(
        octx,
        ocx,
        ocy,
        available.wrapping_sub(width_list).wrapping_sub(width_after),
        width_list,
        list,
        list_left,
        list_right,
        focus_start,
        focus_end,
        frs,
    );
}
/* Draw a format to a screen. */
#[no_mangle]
pub unsafe extern "C" fn format_draw(
    mut octx: *mut screen_write_ctx,
    mut base: *const GridCell,
    mut available: u_int,
    mut expanded: *const libc::c_char,
    mut srs: *mut style_ranges,
) {
    let mut current_block: u64;
    let mut current: C2RustUnnamed_33 = LEFT;
    let mut last: C2RustUnnamed_33 = LEFT;
    let mut names: [*const libc::c_char; 7] = [
        b"LEFT\x00" as *const u8 as *const libc::c_char,
        b"CENTRE\x00" as *const u8 as *const libc::c_char,
        b"RIGHT\x00" as *const u8 as *const libc::c_char,
        b"LIST\x00" as *const u8 as *const libc::c_char,
        b"LIST_LEFT\x00" as *const u8 as *const libc::c_char,
        b"LIST_RIGHT\x00" as *const u8 as *const libc::c_char,
        b"AFTER\x00" as *const u8 as *const libc::c_char,
    ];
    let mut size: size_t = strlen(expanded);
    let mut os: *mut screen = (*octx).s;
    let mut s: [screen; 7] = [screen {
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
    }; 7];
    let mut ctx: [screen_write_ctx; 7] = [screen_write_ctx {
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
    }; 7];
    let mut ocx: u_int = (*os).cx;
    let mut ocy: u_int = (*os).cy;
    let mut i: u_int = 0;
    let mut width: [u_int; 7] = [0; 7];
    let mut map: [u_int; 4] = [LEFT, LEFT, CENTRE, RIGHT];
    let mut focus_start: libc::c_int = -(1i32);
    let mut focus_end: libc::c_int = -(1i32);
    let mut list_state: libc::c_int = -(1i32);
    let mut fill: libc::c_int = -(1i32);
    let mut list_align: style_align = STYLE_ALIGN_DEFAULT;
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
    let mut current_default: GridCell = GridCell {
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
    let mut sy: style = style {
        gc: GridCell {
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
        ignore: 0,
        fill: 0,
        align: STYLE_ALIGN_DEFAULT,
        list: STYLE_LIST_OFF,
        range_type: STYLE_RANGE_NONE,
        range_argument: 0,
        default_type: STYLE_DEFAULT_BASE,
    };
    let mut saved_sy: style = style {
        gc: GridCell {
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
        ignore: 0,
        fill: 0,
        align: STYLE_ALIGN_DEFAULT,
        list: STYLE_LIST_OFF,
        range_type: STYLE_RANGE_NONE,
        range_argument: 0,
        default_type: STYLE_DEFAULT_BASE,
    };
    let mut ud: *mut Utf8Data = &mut sy.gc.data;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut more: Utf8State = utf8_state::MORE;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fr: *mut format_range = 0 as *mut format_range;
    let mut fr1: *mut format_range = 0 as *mut format_range;
    let mut frs: format_ranges = format_ranges {
        tqh_first: 0 as *mut format_range,
        tqh_last: 0 as *mut *mut format_range,
    };
    let mut sr: *mut style_range = 0 as *mut style_range;
    memcpy(
        &mut current_default as *mut GridCell as *mut libc::c_void,
        base as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    style_set(&mut sy, &mut current_default);
    frs.tqh_first = 0 as *mut format_range;
    frs.tqh_last = &mut frs.tqh_first;
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"format_draw\x00")).as_ptr(),
        expanded,
    );
    /*
     * We build three screens for left, right, centre alignment, one for
     * the list, one for anything after the list and two for the list left
     * and right markers.
     */
    i = 0u32;
    while i < TOTAL {
        screen_init(
            &mut *s.as_mut_ptr().offset(i as isize),
            size as u_int,
            1u32,
            0u32,
        );
        screen_write_start(
            &mut *ctx.as_mut_ptr().offset(i as isize),
            &mut *s.as_mut_ptr().offset(i as isize),
        );
        screen_write_clearendofline(
            &mut *ctx.as_mut_ptr().offset(i as isize),
            current_default.bg as u_int,
        );
        width[i as usize] = 0u32;
        i = i.wrapping_add(1)
    }
    /*
     * Walk the string and add to the corresponding screens,
     * parsing styles as we go.
     */
    cp = expanded;
    loop {
        if !(*cp as libc::c_int != '\u{0}' as i32) {
            current_block = 10763371041174037105;
            break;
        }
        if *cp.offset(0isize) as libc::c_int != '#' as i32
            || *cp.offset(1isize) as libc::c_int != '[' as i32
            || sy.ignore != 0
        {
            /* See if this is a UTF-8 character. */
            more = utf8_open(ud, *cp as u_char);
            if more == utf8_state::MORE {
                loop {
                    cp = cp.offset(1);
                    if !(*cp as libc::c_int != '\u{0}' as i32 && more == utf8_state::MORE) {
                        break;
                    }
                    more = utf8_append(ud, *cp as u_char)
                }
                if more != utf8_state::DONE {
                    cp = cp.offset(-((*ud).have as libc::c_int as isize))
                }
            }
            /* Not a UTF-8 character - ASCII or not valid. */
            if more != utf8_state::DONE {
                if (*cp as libc::c_int) < 0x20i32 || *cp as libc::c_int > 0x7ei32 {
                    /* Ignore nonprintable characters. */
                    cp = cp.offset(1);
                    continue;
                } else {
                    utf8_set(ud, *cp as u_char);
                    cp = cp.offset(1)
                }
            }
            /* Draw the cell to the current screen. */
            screen_write_cell(&mut *ctx.as_mut_ptr().offset(current as isize), &mut sy.gc);
            width[current as usize] =
                (width[current as usize]).wrapping_add((*ud).width as libc::c_uint)
        } else {
            /* This is a style. Work out where the end is and parse it. */
            end = format_skip(
                cp.offset(2isize),
                b"]\x00" as *const u8 as *const libc::c_char,
            );
            if end.is_null() {
                log_debug(
                    b"%s: no terminating ] at \'%s\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"format_draw\x00"))
                        .as_ptr(),
                    cp.offset(2isize),
                );
                fr = frs.tqh_first;
                while !fr.is_null() && {
                    fr1 = (*fr).entry.tqe_next;
                    (1i32) != 0
                } {
                    format_free_range(&mut frs, fr);
                    fr = fr1
                }
                current_block = 12270534628354781744;
                break;
            } else {
                tmp = xstrndup(
                    cp.offset(2isize),
                    end.wrapping_offset_from(cp.offset(2isize)) as size_t,
                );
                style_copy(&mut saved_sy, &mut sy);
                if style_parse(&mut sy, &mut current_default, tmp) != 0i32 {
                    log_debug(
                        b"%s: invalid style \'%s\'\x00" as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(
                            b"format_draw\x00",
                        ))
                        .as_ptr(),
                        tmp,
                    );
                    free(tmp as *mut libc::c_void);
                    cp = end.offset(1isize)
                } else {
                    log_debug(
                        b"%s: style \'%s\' -> \'%s\'\x00" as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(
                            b"format_draw\x00",
                        ))
                        .as_ptr(),
                        tmp,
                        style_tostring(&mut sy),
                    );
                    free(tmp as *mut libc::c_void);
                    /* If this style has a fill colour, store it for later. */
                    if sy.fill != 8i32 {
                        fill = sy.fill
                    }
                    /* If this style pushed or popped the default, update it. */
                    if sy.default_type == STYLE_DEFAULT_PUSH {
                        memcpy(
                            &mut current_default as *mut GridCell as *mut libc::c_void,
                            &mut saved_sy.gc as *mut GridCell as *const libc::c_void,
                            ::std::mem::size_of::<GridCell>() as libc::c_ulong,
                        );
                        sy.default_type = STYLE_DEFAULT_BASE
                    } else if sy.default_type == STYLE_DEFAULT_POP {
                        memcpy(
                            &mut current_default as *mut GridCell as *mut libc::c_void,
                            base as *const libc::c_void,
                            ::std::mem::size_of::<GridCell>() as libc::c_ulong,
                        );
                        sy.default_type = STYLE_DEFAULT_BASE
                    }
                    /* Check the list state. */
                    match sy.list {
                        1 => {
                            /*
                             * Entering the list, exiting a marker, or exiting the
                             * focus.
                             */
                            if list_state != 0i32 {
                                if !fr.is_null() {
                                    /* abort any region */
                                    free(fr as *mut libc::c_void);
                                    fr = 0 as *mut format_range
                                }
                                list_state = 0i32;
                                list_align = sy.align
                            }
                            /* End the focus if started. */
                            if focus_start != -(1i32) && focus_end == -(1i32) {
                                focus_end = s[LIST as usize].cx as libc::c_int
                            }
                            current = LIST
                        }
                        2 => {
                            /* Entering the focus. */
                            if !(list_state != 0i32) {
                                if focus_start == -(1i32) {
                                    /* focus already started */
                                    focus_start = s[LIST as usize].cx as libc::c_int
                                }
                            }
                        }
                        0 => {
                            /* Exiting or outside the list. */
                            if list_state == 0i32 {
                                if !fr.is_null() {
                                    /* abort any region */
                                    free(fr as *mut libc::c_void);
                                    fr = 0 as *mut format_range
                                }
                                if focus_start != -(1i32) && focus_end == -(1i32) {
                                    focus_end = s[LIST as usize].cx as libc::c_int
                                }
                                map[list_align as usize] = AFTER;
                                if list_align == STYLE_ALIGN_LEFT {
                                    map[STYLE_ALIGN_DEFAULT as usize] = AFTER
                                }
                                list_state = 1i32
                            }
                            current = map[sy.align as usize]
                        }
                        3 => {
                            /* Entering left marker. */
                            if !(list_state != 0i32) {
                                if !(s[LIST_LEFT as usize].cx != 0u32) {
                                    if !fr.is_null() {
                                        /* abort any region */
                                        free(fr as *mut libc::c_void);
                                        fr = 0 as *mut format_range
                                    }
                                    if focus_start != -(1i32) && focus_end == -(1i32) {
                                        focus_end = -(1i32);
                                        focus_start = focus_end
                                    }
                                    current = LIST_LEFT
                                }
                            }
                        }
                        4 => {
                            /* Entering right marker. */
                            if !(list_state != 0i32) {
                                if !(s[LIST_RIGHT as usize].cx != 0u32) {
                                    if !fr.is_null() {
                                        /* abort any region */
                                        free(fr as *mut libc::c_void);
                                        fr = 0 as *mut format_range
                                    }
                                    if focus_start != -(1i32) && focus_end == -(1i32) {
                                        focus_end = -(1i32);
                                        focus_start = focus_end
                                    }
                                    current = LIST_RIGHT
                                }
                            }
                        }
                        _ => {}
                    }
                    if current != last {
                        log_debug(
                            b"%s: change %s -> %s\x00" as *const u8 as *const libc::c_char,
                            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(
                                b"format_draw\x00",
                            ))
                            .as_ptr(),
                            names[last as usize],
                            names[current as usize],
                        );
                        last = current
                    }
                    /*
                     * Check if the range style has changed and if so end the
                     * current range and start a new one if needed.
                     */
                    if !srs.is_null() {
                        if !fr.is_null() && format_is_type(fr, &mut sy) == 0 {
                            if s[current as usize].cx != (*fr).start {
                                (*fr).end = s[current as usize].cx.wrapping_add(1u32);
                                (*fr).entry.tqe_next = 0 as *mut format_range;
                                (*fr).entry.tqe_prev = frs.tqh_last;
                                *frs.tqh_last = fr;
                                frs.tqh_last = &mut (*fr).entry.tqe_next
                            } else {
                                free(fr as *mut libc::c_void);
                            }
                            fr = 0 as *mut format_range
                        }
                        if fr.is_null() && sy.range_type != STYLE_RANGE_NONE {
                            fr = xcalloc(
                                1u64,
                                ::std::mem::size_of::<format_range>() as libc::c_ulong,
                            ) as *mut format_range;
                            (*fr).index = current;
                            (*fr).s = &mut *s.as_mut_ptr().offset(current as isize) as *mut screen;
                            (*fr).start = s[current as usize].cx;
                            (*fr).type_0 = sy.range_type;
                            (*fr).argument = sy.range_argument
                        }
                    }
                    cp = end.offset(1isize)
                }
            }
        }
    }
    match current_block {
        10763371041174037105 => {
            free(fr as *mut libc::c_void);
            i = 0u32;
            while i < TOTAL {
                screen_write_stop(&mut *ctx.as_mut_ptr().offset(i as isize));
                log_debug(
                    b"%s: width %s is %u\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"format_draw\x00"))
                        .as_ptr(),
                    names[i as usize],
                    width[i as usize],
                );
                i = i.wrapping_add(1)
            }
            if focus_start != -(1i32) && focus_end != -(1i32) {
                log_debug(
                    b"%s: focus %d-%d\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"format_draw\x00"))
                        .as_ptr(),
                    focus_start,
                    focus_end,
                );
            }
            fr = frs.tqh_first;
            while !fr.is_null() {
                log_debug(
                    b"%s: range %d|%u is %s %u-%u\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"format_draw\x00"))
                        .as_ptr(),
                    (*fr).type_0,
                    (*fr).argument,
                    names[(*fr).index as usize],
                    (*fr).start,
                    (*fr).end,
                );
                fr = (*fr).entry.tqe_next
            }
            /* Clear the available area. */
            if fill != -(1i32) {
                memcpy(
                    &mut gc as *mut GridCell as *mut libc::c_void,
                    &grid_default_cell as *const GridCell as *const libc::c_void,
                    ::std::mem::size_of::<GridCell>() as libc::c_ulong,
                );
                gc.bg = fill;
                i = 0u32;
                while i < available {
                    screen_write_putc(octx, &mut gc, ' ' as u_char);
                    i = i.wrapping_add(1)
                }
            }
            /*
             * Draw the screens. How they are arranged depends on where the list
             * appears.
             */
            match list_align {
                0 => {
                    /* No list. */
                    format_draw_none(
                        octx,
                        available,
                        ocx,
                        ocy,
                        &mut *s.as_mut_ptr().offset(LEFT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(CENTRE as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(RIGHT as libc::c_int as isize),
                        &mut frs,
                    );
                }
                1 => {
                    /* List is part of the left. */
                    format_draw_left(
                        octx,
                        available,
                        ocx,
                        ocy,
                        &mut *s.as_mut_ptr().offset(LEFT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(CENTRE as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(RIGHT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(LIST as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(LIST_LEFT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(LIST_RIGHT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(AFTER as libc::c_int as isize),
                        focus_start,
                        focus_end,
                        &mut frs,
                    );
                }
                2 => {
                    /* List is part of the centre. */
                    format_draw_centre(
                        octx,
                        available,
                        ocx,
                        ocy,
                        &mut *s.as_mut_ptr().offset(LEFT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(CENTRE as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(RIGHT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(LIST as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(LIST_LEFT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(LIST_RIGHT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(AFTER as libc::c_int as isize),
                        focus_start,
                        focus_end,
                        &mut frs,
                    );
                }
                3 => {
                    /* List is part of the right. */
                    format_draw_right(
                        octx,
                        available,
                        ocx,
                        ocy,
                        &mut *s.as_mut_ptr().offset(LEFT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(CENTRE as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(RIGHT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(LIST as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(LIST_LEFT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(LIST_RIGHT as libc::c_int as isize),
                        &mut *s.as_mut_ptr().offset(AFTER as libc::c_int as isize),
                        focus_start,
                        focus_end,
                        &mut frs,
                    );
                }
                _ => {}
            }
            /* Create ranges to return. */
            fr = frs.tqh_first;
            while !fr.is_null() && {
                fr1 = (*fr).entry.tqe_next;
                (1i32) != 0
            } {
                sr = xcalloc(1u64, ::std::mem::size_of::<style_range>() as libc::c_ulong)
                    as *mut style_range;
                (*sr).type_0 = (*fr).type_0;
                (*sr).argument = (*fr).argument;
                (*sr).start = (*fr).start;
                (*sr).end = (*fr).end;
                (*sr).entry.tqe_next = 0 as *mut style_range;
                (*sr).entry.tqe_prev = (*srs).tqh_last;
                *(*srs).tqh_last = sr;
                (*srs).tqh_last = &mut (*sr).entry.tqe_next;
                log_debug(
                    b"%s: range %d|%u at %u-%u\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"format_draw\x00"))
                        .as_ptr(),
                    (*sr).type_0,
                    (*sr).argument,
                    (*sr).start,
                    (*sr).end,
                );
                format_free_range(&mut frs, fr);
                fr = fr1
            }
        }
        _ => {}
    }
    /* Free the screens. */
    i = 0u32;
    while i < TOTAL {
        screen_free(&mut *s.as_mut_ptr().offset(i as isize));
        i = i.wrapping_add(1)
    }
    /* Restore the original cursor position. */
    screen_write_cursormove(octx, ocx as libc::c_int, ocy as libc::c_int, 0i32);
}
/* Get width, taking #[] into account. */
#[no_mangle]
pub unsafe extern "C" fn format_width(mut expanded: *const libc::c_char) -> u_int {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut width: u_int = 0u32;
    let mut ud: Utf8Data = Utf8Data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut more: Utf8State = utf8_state::MORE;
    cp = expanded;
    while *cp as libc::c_int != '\u{0}' as i32 {
        if *cp.offset(0isize) as libc::c_int == '#' as i32
            && *cp.offset(1isize) as libc::c_int == '[' as i32
        {
            end = format_skip(
                cp.offset(2isize),
                b"]\x00" as *const u8 as *const libc::c_char,
            );
            if end.is_null() {
                return 0u32;
            }
            cp = end.offset(1isize)
        } else {
            more = utf8_open(&mut ud, *cp as u_char);
            if more == utf8_state::MORE {
                loop {
                    cp = cp.offset(1);
                    if !(*cp as libc::c_int != '\u{0}' as i32 && more == utf8_state::MORE) {
                        break;
                    }
                    more = utf8_append(&mut ud, *cp as u_char)
                }
                if more == utf8_state::DONE {
                    width = (width).wrapping_add(ud.width as libc::c_uint)
                } else {
                    cp = cp.offset(-(ud.have as libc::c_int as isize))
                }
            } else if *cp as libc::c_int > 0x1fi32 && (*cp as libc::c_int) < 0x7fi32 {
                width = width.wrapping_add(1);
                cp = cp.offset(1)
            } else {
                cp = cp.offset(1)
            }
        }
    }
    return width;
}
/* Trim on the left, taking #[] into account. */
#[no_mangle]
pub unsafe extern "C" fn format_trim_left(
    mut expanded: *const libc::c_char,
    mut limit: u_int,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *const libc::c_char = expanded;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut width: u_int = 0u32;
    let mut ud: Utf8Data = Utf8Data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut more: Utf8State = utf8_state::MORE;
    copy = xmalloc(strlen(expanded).wrapping_add(1u64)) as *mut libc::c_char;
    out = copy;
    while *cp as libc::c_int != '\u{0}' as i32 {
        if *cp.offset(0isize) as libc::c_int == '#' as i32
            && *cp.offset(1isize) as libc::c_int == '[' as i32
        {
            end = format_skip(
                cp.offset(2isize),
                b"]\x00" as *const u8 as *const libc::c_char,
            );
            if end.is_null() {
                break;
            }
            memcpy(
                out as *mut libc::c_void,
                cp as *const libc::c_void,
                end.offset(1isize).wrapping_offset_from(cp) as libc::c_ulong,
            );
            out = out.offset(end.offset(1isize).wrapping_offset_from(cp));
            cp = end.offset(1isize)
        } else {
            more = utf8_open(&mut ud, *cp as u_char);
            if more == utf8_state::MORE {
                loop {
                    cp = cp.offset(1);
                    if !(*cp as libc::c_int != '\u{0}' as i32 && more == utf8_state::MORE) {
                        break;
                    }
                    more = utf8_append(&mut ud, *cp as u_char)
                }
                if more == utf8_state::DONE {
                    if width.wrapping_add(ud.width as libc::c_uint) <= limit {
                        memcpy(
                            out as *mut libc::c_void,
                            ud.data.as_mut_ptr() as *const libc::c_void,
                            ud.size as libc::c_ulong,
                        );
                        out = out.offset(ud.size as libc::c_int as isize)
                    }
                    width = (width).wrapping_add(ud.width as libc::c_uint)
                } else {
                    cp = cp.offset(-(ud.have as libc::c_int as isize));
                    cp = cp.offset(1)
                }
            } else if *cp as libc::c_int > 0x1fi32 && (*cp as libc::c_int) < 0x7fi32 {
                if width.wrapping_add(1u32) <= limit {
                    let fresh0 = out;
                    out = out.offset(1);
                    *fresh0 = *cp
                }
                width = width.wrapping_add(1);
                cp = cp.offset(1)
            } else {
                cp = cp.offset(1)
            }
        }
    }
    *out = '\u{0}' as libc::c_char;
    return copy;
}
/* Trim on the right, taking #[] into account. */
#[no_mangle]
pub unsafe extern "C" fn format_trim_right(
    mut expanded: *const libc::c_char,
    mut limit: u_int,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *const libc::c_char = expanded;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut width: u_int = 0u32;
    let mut total_width: u_int = 0;
    let mut skip: u_int = 0;
    let mut ud: Utf8Data = Utf8Data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut more: Utf8State = utf8_state::MORE;
    total_width = format_width(expanded);
    if total_width <= limit {
        return xstrdup(expanded);
    }
    skip = total_width.wrapping_sub(limit);
    copy = xmalloc(strlen(expanded).wrapping_add(1u64)) as *mut libc::c_char;
    out = copy;
    while *cp as libc::c_int != '\u{0}' as i32 {
        if *cp.offset(0isize) as libc::c_int == '#' as i32
            && *cp.offset(1isize) as libc::c_int == '[' as i32
        {
            end = format_skip(
                cp.offset(2isize),
                b"]\x00" as *const u8 as *const libc::c_char,
            );
            if end.is_null() {
                break;
            }
            memcpy(
                out as *mut libc::c_void,
                cp as *const libc::c_void,
                end.offset(1isize).wrapping_offset_from(cp) as libc::c_ulong,
            );
            out = out.offset(end.offset(1isize).wrapping_offset_from(cp));
            cp = end.offset(1isize)
        } else {
            more = utf8_open(&mut ud, *cp as u_char);
            if more == utf8_state::MORE {
                loop {
                    cp = cp.offset(1);
                    if !(*cp as libc::c_int != '\u{0}' as i32 && more == utf8_state::MORE) {
                        break;
                    }
                    more = utf8_append(&mut ud, *cp as u_char)
                }
                if more == utf8_state::DONE {
                    if width >= skip {
                        memcpy(
                            out as *mut libc::c_void,
                            ud.data.as_mut_ptr() as *const libc::c_void,
                            ud.size as libc::c_ulong,
                        );
                        out = out.offset(ud.size as libc::c_int as isize)
                    }
                    width = (width).wrapping_add(ud.width as libc::c_uint)
                } else {
                    cp = cp.offset(-(ud.have as libc::c_int as isize));
                    cp = cp.offset(1)
                }
            } else if *cp as libc::c_int > 0x1fi32 && (*cp as libc::c_int) < 0x7fi32 {
                if width >= skip {
                    let fresh1 = out;
                    out = out.offset(1);
                    *fresh1 = *cp
                }
                width = width.wrapping_add(1);
                cp = cp.offset(1)
            } else {
                cp = cp.offset(1)
            }
        }
    }
    *out = '\u{0}' as libc::c_char;
    return copy;
}
