use crate::{grid::Cell as GridCell, utf8::Utf8Data};
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn options_string_to_style(
        _: *mut crate::options::options,
        _: *const libc::c_char,
        _: *mut crate::format::format_tree,
    ) -> *mut Style;
    #[no_mangle]
    fn colour_tostring(_: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn colour_fromstring(s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn attributes_tostring(_: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn attributes_fromstring(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
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
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_5,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_0,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub ev_io: C2RustUnnamed_3,
    pub ev_signal: C2RustUnnamed_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub ev_signal_next: C2RustUnnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub ev_io_next: C2RustUnnamed_4,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_5 {
    pub ev_next_with_common_timeout: C2RustUnnamed_6,
    pub min_heap_idx: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_8,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_7,
    pub evcb_arg: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
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
    pub exit_type: C2RustUnnamed_29,
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
    pub prompt_mode: C2RustUnnamed_26,
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
    pub entry: C2RustUnnamed_9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
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
    pub entry: C2RustUnnamed_10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
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
    pub grid: *mut crate::grid::Grid,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub rupper: u_int,
    pub rlower: u_int,
    pub mode: libc::c_int,
    pub saved_cx: u_int,
    pub saved_cy: u_int,
    pub saved_grid: *mut crate::grid::Grid,
    pub saved_cell: crate::grid::Cell,
    pub saved_flags: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: *mut crate::screen::screen_sel,
    pub write_list: *mut crate::screen_write::screen_write_collect_line,
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
    pub gentry: C2RustUnnamed_14,
    pub entry: C2RustUnnamed_13,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_14 {
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
    pub entry: C2RustUnnamed_17,
    pub wentry: C2RustUnnamed_16,
    pub sentry: C2RustUnnamed_15,
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
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_17 {
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
    pub alerts_entry: C2RustUnnamed_20,
    pub options: *mut crate::options::options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_19,
    pub entry: C2RustUnnamed_18,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_18 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_19 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_20 {
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
    pub entry: C2RustUnnamed_21,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_21 {
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
    pub modes: C2RustUnnamed_24,
    pub searchstr: *mut libc::c_char,
    pub searchregex: libc::c_int,
    pub written: size_t,
    pub skipped: size_t,
    pub border_gc_set: libc::c_int,
    pub border_gc: crate::grid::Cell,
    pub entry: C2RustUnnamed_23,
    pub tree_entry: C2RustUnnamed_22,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_22 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_23 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_24 {
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
    pub entry: C2RustUnnamed_25,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_25 {
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
pub type C2RustUnnamed_26 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_26 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_26 = 0;
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
    pub entry: C2RustUnnamed_27,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_27 {
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
    pub entry: C2RustUnnamed_28,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_28 {
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
pub type C2RustUnnamed_29 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_29 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_29 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_29 = 0;

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
    pub ranges: crate::style::Ranges,
}

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
    pub entry: C2RustUnnamed_31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_31 {
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
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub rbe_left: *mut client_window,
    pub rbe_right: *mut client_window,
    pub rbe_parent: *mut client_window,
    pub rbe_color: libc::c_int,
}

#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(__nptr, 0 as *mut *mut libc::c_char, 10i32) as libc::c_int;
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2007 Nicholas Marriott <nicholas.marriott@gmail.com>
 * Copyright (c) 2014 Tiago Cunha <tcunha@users.sourceforge.net>
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
/// Style alignment.
pub type Align = libc::c_uint;
pub mod align {
    use super::Align;

    pub const DEFAULT: Align = 0;
    pub const LEFT: Align = 1;
    pub const CENTRE: Align = 2;
    pub const RIGHT: Align = 3;
}

/// Style list.
pub type List = libc::c_uint;
pub mod list {
    use super::List;

    pub const OFF: List = 0;
    pub const ON: List = 1;
    pub const FOCUS: List = 2;
    pub const LEFT_MARKER: List = 3;
    pub const RIGHT_MARKER: List = 4;
}

// Style range.
pub type RangeType = libc::c_uint;
pub mod range_type {
    use super::RangeType;
    pub const NONE: RangeType = 0;
    pub const LEFT: RangeType = 1;
    pub const RIGHT: RangeType = 2;
    pub const WINDOW: RangeType = 3;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Range {
    pub type_0: RangeType,
    pub argument: u_int,
    pub start: u_int,
    pub end: u_int,
    pub entry: C2RustUnnamed_30,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_30 {
    pub tqe_next: *mut Range,
    pub tqe_prev: *mut *mut Range,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ranges {
    pub tqh_first: *mut Range,
    pub tqh_last: *mut *mut Range,
}

/// Style default.
pub type DefaultType = libc::c_uint;
pub mod default_type {
    use super::DefaultType;

    pub const BASE: DefaultType = 0;
    pub const PUSH: DefaultType = 1;
    pub const POP: DefaultType = 2;
}

/// Style option.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Style {
    pub gc: crate::grid::Cell,
    pub ignore: libc::c_int,
    pub fill: libc::c_int,
    pub align: Align,
    pub list: List,
    pub range_type: RangeType,
    pub range_argument: u_int,
    pub default_type: DefaultType,
}

/* Mask for bits not included in style. */
/* Default style. */
static mut style_default: Style = {
    let mut init = Style {
        gc: {
            let mut init = GridCell {
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
        },
        ignore: 0i32,
        fill: 8i32,
        align: align::DEFAULT,
        list: list::OFF,
        range_type: range_type::NONE,
        range_argument: 0u32,
        default_type: default_type::BASE,
    };
    init
};
/*
 * Parse an embedded style of the form "fg=colour,bg=colour,bright,...".  Note
 * that this adds onto the given style, so it must have been initialized
 * already.
 */
#[no_mangle]
pub unsafe extern "C" fn style_parse(
    mut sy: *mut Style,
    mut base: *const GridCell,
    mut in_0: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut saved: Style = Style {
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
        align: align::DEFAULT,
        list: list::OFF,
        range_type: range_type::NONE,
        range_argument: 0,
        default_type: default_type::BASE,
    };
    let delimiters: [libc::c_char; 3] =
        *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b" ,\x00");
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: [libc::c_char; 256] = [0; 256];
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: libc::c_int = 0;
    let mut end: size_t = 0;
    if *in_0 as libc::c_int == '\u{0}' as i32 {
        return 0i32;
    }
    style_copy(&mut saved, sy);
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"style_parse\x00")).as_ptr(),
        in_0,
    );
    's_36: loop {
        while *in_0 as libc::c_int != '\u{0}' as i32
            && !strchr(delimiters.as_ptr(), *in_0 as libc::c_int).is_null()
        {
            in_0 = in_0.offset(1)
        }
        if *in_0 as libc::c_int == '\u{0}' as i32 {
            current_block = 7416055328783156979;
            break;
        }
        end = strcspn(in_0, delimiters.as_ptr());
        if end > (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong).wrapping_sub(1u64)
        {
            current_block = 13061046559241396907;
            break;
        }
        memcpy(
            tmp.as_mut_ptr() as *mut libc::c_void,
            in_0 as *const libc::c_void,
            end,
        );
        tmp[end as usize] = '\u{0}' as libc::c_char;
        log_debug(
            b"%s: %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"style_parse\x00")).as_ptr(),
            tmp.as_mut_ptr(),
        );
        if strcasecmp(
            tmp.as_mut_ptr(),
            b"default\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*sy).gc.fg = (*base).fg;
            (*sy).gc.bg = (*base).bg;
            (*sy).gc.attr = (*base).attr;
            (*sy).gc.flags = (*base).flags
        } else if strcasecmp(
            tmp.as_mut_ptr(),
            b"ignore\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*sy).ignore = 1i32
        } else if strcasecmp(
            tmp.as_mut_ptr(),
            b"noignore\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*sy).ignore = 0i32
        } else if strcasecmp(
            tmp.as_mut_ptr(),
            b"push-default\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*sy).default_type = default_type::PUSH
        } else if strcasecmp(
            tmp.as_mut_ptr(),
            b"pop-default\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*sy).default_type = default_type::POP
        } else if strcasecmp(
            tmp.as_mut_ptr(),
            b"nolist\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*sy).list = list::OFF
        } else if strncasecmp(
            tmp.as_mut_ptr(),
            b"list=\x00" as *const u8 as *const libc::c_char,
            5u64,
        ) == 0i32
        {
            if strcasecmp(
                tmp.as_mut_ptr().offset(5isize),
                b"on\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                (*sy).list = list::ON
            } else if strcasecmp(
                tmp.as_mut_ptr().offset(5isize),
                b"focus\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                (*sy).list = list::FOCUS
            } else if strcasecmp(
                tmp.as_mut_ptr().offset(5isize),
                b"left-marker\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                (*sy).list = list::LEFT_MARKER
            } else {
                if !(strcasecmp(
                    tmp.as_mut_ptr().offset(5isize),
                    b"right-marker\x00" as *const u8 as *const libc::c_char,
                ) == 0i32)
                {
                    current_block = 13061046559241396907;
                    break;
                }
                (*sy).list = list::RIGHT_MARKER
            }
        } else if strcasecmp(
            tmp.as_mut_ptr(),
            b"norange\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*sy).range_type = style_default.range_type;
            (*sy).range_argument = style_default.range_type
        } else if end > 6u64
            && strncasecmp(
                tmp.as_mut_ptr(),
                b"range=\x00" as *const u8 as *const libc::c_char,
                6u64,
            ) == 0i32
        {
            found = strchr(tmp.as_mut_ptr().offset(6isize), '|' as i32);
            if !found.is_null() {
                let fresh0 = found;
                found = found.offset(1);
                *fresh0 = '\u{0}' as libc::c_char;
                if *found as libc::c_int == '\u{0}' as i32 {
                    current_block = 13061046559241396907;
                    break;
                }
                cp = found;
                while *cp as libc::c_int != '\u{0}' as i32 {
                    if *(*__ctype_b_loc()).offset(*cp as u_char as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_ushort as libc::c_int
                        == 0
                    {
                        current_block = 13061046559241396907;
                        break 's_36;
                    }
                    cp = cp.offset(1)
                }
            }
            if strcasecmp(
                tmp.as_mut_ptr().offset(6isize),
                b"left\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                if !found.is_null() {
                    current_block = 13061046559241396907;
                    break;
                }
                (*sy).range_type = range_type::LEFT;
                (*sy).range_argument = 0u32
            } else if strcasecmp(
                tmp.as_mut_ptr().offset(6isize),
                b"right\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                if !found.is_null() {
                    current_block = 13061046559241396907;
                    break;
                }
                (*sy).range_type = range_type::RIGHT;
                (*sy).range_argument = 0u32
            } else if strcasecmp(
                tmp.as_mut_ptr().offset(6isize),
                b"window\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                if found.is_null() {
                    current_block = 13061046559241396907;
                    break;
                }
                (*sy).range_type = range_type::WINDOW;
                (*sy).range_argument = atoi(found) as u_int
            }
        } else if strcasecmp(
            tmp.as_mut_ptr(),
            b"noalign\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*sy).align = style_default.align
        } else if end > 6u64
            && strncasecmp(
                tmp.as_mut_ptr(),
                b"align=\x00" as *const u8 as *const libc::c_char,
                6u64,
            ) == 0i32
        {
            if strcasecmp(
                tmp.as_mut_ptr().offset(6isize),
                b"left\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                (*sy).align = align::LEFT
            } else if strcasecmp(
                tmp.as_mut_ptr().offset(6isize),
                b"centre\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                (*sy).align = align::CENTRE
            } else {
                if !(strcasecmp(
                    tmp.as_mut_ptr().offset(6isize),
                    b"right\x00" as *const u8 as *const libc::c_char,
                ) == 0i32)
                {
                    current_block = 13061046559241396907;
                    break;
                }
                (*sy).align = align::RIGHT
            }
        } else if end > 5u64
            && strncasecmp(
                tmp.as_mut_ptr(),
                b"fill=\x00" as *const u8 as *const libc::c_char,
                5u64,
            ) == 0i32
        {
            value = colour_fromstring(tmp.as_mut_ptr().offset(5isize));
            if value == -(1i32) {
                current_block = 13061046559241396907;
                break;
            }
            (*sy).fill = value
        } else if end > 3u64
            && strncasecmp(
                tmp.as_mut_ptr().offset(1isize),
                b"g=\x00" as *const u8 as *const libc::c_char,
                2u64,
            ) == 0i32
        {
            value = colour_fromstring(tmp.as_mut_ptr().offset(3isize));
            if value == -(1i32) {
                current_block = 13061046559241396907;
                break;
            }
            if *in_0 as libc::c_int == 'f' as i32 || *in_0 as libc::c_int == 'F' as i32 {
                if value != 8i32 {
                    (*sy).gc.fg = value
                } else {
                    (*sy).gc.fg = (*base).fg
                }
            } else {
                if !(*in_0 as libc::c_int == 'b' as i32 || *in_0 as libc::c_int == 'B' as i32) {
                    current_block = 13061046559241396907;
                    break;
                }
                if value != 8i32 {
                    (*sy).gc.bg = value
                } else {
                    (*sy).gc.bg = (*base).bg
                }
            }
        } else if strcasecmp(
            tmp.as_mut_ptr(),
            b"none\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*sy).gc.attr = 0u16
        } else if end > 2u64
            && strncasecmp(
                tmp.as_mut_ptr(),
                b"no\x00" as *const u8 as *const libc::c_char,
                2u64,
            ) == 0i32
        {
            value = attributes_fromstring(tmp.as_mut_ptr().offset(2isize));
            if value == -(1i32) {
                current_block = 13061046559241396907;
                break;
            }
            (*sy).gc.attr = ((*sy).gc.attr as libc::c_int & !value) as u_short
        } else {
            value = attributes_fromstring(tmp.as_mut_ptr());
            if value == -(1i32) {
                current_block = 13061046559241396907;
                break;
            }
            (*sy).gc.attr = ((*sy).gc.attr as libc::c_int | value) as u_short
        }
        in_0 = in_0.offset(
            end.wrapping_add(strspn(in_0.offset(end as isize), delimiters.as_ptr())) as isize,
        );
        if !(*in_0 as libc::c_int != '\u{0}' as i32) {
            current_block = 7416055328783156979;
            break;
        }
    }
    match current_block {
        7416055328783156979 => return 0i32,
        _ => {
            style_copy(sy, &mut saved);
            return -(1i32);
        }
    };
}
/* Convert style to a string. */
#[no_mangle]
pub unsafe extern "C" fn style_tostring(mut sy: *mut Style) -> *const libc::c_char {
    let mut gc: *mut GridCell = &mut (*sy).gc;
    let mut off: libc::c_int = 0i32;
    let mut comma: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    let mut tmp: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    static mut s: [libc::c_char; 256] = [0; 256];
    let mut b: [libc::c_char; 16] = [0; 16];
    *s.as_mut_ptr() = '\u{0}' as libc::c_char;
    if (*sy).list != list::OFF {
        if (*sy).list == list::ON {
            tmp = b"on\x00" as *const u8 as *const libc::c_char
        } else if (*sy).list == list::FOCUS {
            tmp = b"focus\x00" as *const u8 as *const libc::c_char
        } else if (*sy).list == list::LEFT_MARKER {
            tmp = b"left-marker\x00" as *const u8 as *const libc::c_char
        } else if (*sy).list == list::RIGHT_MARKER {
            tmp = b"right-marker\x00" as *const u8 as *const libc::c_char
        }
        off += xsnprintf(
            s.as_mut_ptr().offset(off as isize),
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(off as libc::c_ulong),
            b"%slist=%s\x00" as *const u8 as *const libc::c_char,
            comma,
            tmp,
        );
        comma = b",\x00" as *const u8 as *const libc::c_char
    }
    if (*sy).range_type != range_type::NONE {
        if (*sy).range_type == range_type::LEFT {
            tmp = b"left\x00" as *const u8 as *const libc::c_char
        } else if (*sy).range_type == range_type::RIGHT {
            tmp = b"right\x00" as *const u8 as *const libc::c_char
        } else if (*sy).range_type == range_type::WINDOW {
            snprintf(
                b.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                b"window|%u\x00" as *const u8 as *const libc::c_char,
                (*sy).range_argument,
            );
            tmp = b.as_mut_ptr()
        }
        off += xsnprintf(
            s.as_mut_ptr().offset(off as isize),
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(off as libc::c_ulong),
            b"%srange=%s\x00" as *const u8 as *const libc::c_char,
            comma,
            tmp,
        );
        comma = b",\x00" as *const u8 as *const libc::c_char
    }
    if (*sy).align != align::DEFAULT {
        if (*sy).align == align::LEFT {
            tmp = b"left\x00" as *const u8 as *const libc::c_char
        } else if (*sy).align == align::CENTRE {
            tmp = b"centre\x00" as *const u8 as *const libc::c_char
        } else if (*sy).align == align::RIGHT {
            tmp = b"right\x00" as *const u8 as *const libc::c_char
        }
        off += xsnprintf(
            s.as_mut_ptr().offset(off as isize),
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(off as libc::c_ulong),
            b"%salign=%s\x00" as *const u8 as *const libc::c_char,
            comma,
            tmp,
        );
        comma = b",\x00" as *const u8 as *const libc::c_char
    }
    if (*sy).default_type != default_type::BASE {
        if (*sy).default_type == default_type::PUSH {
            tmp = b"push-default\x00" as *const u8 as *const libc::c_char
        } else if (*sy).default_type == default_type::POP {
            tmp = b"pop-default\x00" as *const u8 as *const libc::c_char
        }
        off += xsnprintf(
            s.as_mut_ptr().offset(off as isize),
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(off as libc::c_ulong),
            b"%s%s\x00" as *const u8 as *const libc::c_char,
            comma,
            tmp,
        );
        comma = b",\x00" as *const u8 as *const libc::c_char
    }
    if (*sy).fill != 8i32 {
        off += xsnprintf(
            s.as_mut_ptr().offset(off as isize),
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(off as libc::c_ulong),
            b"%sfill=%s\x00" as *const u8 as *const libc::c_char,
            comma,
            colour_tostring((*sy).fill),
        );
        comma = b",\x00" as *const u8 as *const libc::c_char
    }
    if (*gc).fg != 8i32 {
        off += xsnprintf(
            s.as_mut_ptr().offset(off as isize),
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(off as libc::c_ulong),
            b"%sfg=%s\x00" as *const u8 as *const libc::c_char,
            comma,
            colour_tostring((*gc).fg),
        );
        comma = b",\x00" as *const u8 as *const libc::c_char
    }
    if (*gc).bg != 8i32 {
        off += xsnprintf(
            s.as_mut_ptr().offset(off as isize),
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(off as libc::c_ulong),
            b"%sbg=%s\x00" as *const u8 as *const libc::c_char,
            comma,
            colour_tostring((*gc).bg),
        );
        comma = b",\x00" as *const u8 as *const libc::c_char
    }
    if (*gc).attr as libc::c_int != 0i32 {
        xsnprintf(
            s.as_mut_ptr().offset(off as isize),
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(off as libc::c_ulong),
            b"%s%s\x00" as *const u8 as *const libc::c_char,
            comma,
            attributes_tostring((*gc).attr as libc::c_int),
        );
        comma = b",\x00" as *const u8 as *const libc::c_char
    }
    if *s.as_mut_ptr() as libc::c_int == '\u{0}' as i32 {
        return b"default\x00" as *const u8 as *const libc::c_char;
    }
    return s.as_mut_ptr();
}
/* Apply a style on top of the given style. */
#[no_mangle]
pub unsafe extern "C" fn style_add(
    mut gc: *mut GridCell,
    mut oo: *mut crate::options::options,
    mut name: *const libc::c_char,
    mut ft: *mut crate::format::format_tree,
) {
    let mut sy: *mut Style = 0 as *mut Style;
    let mut ft0: *mut crate::format::format_tree = 0 as *mut crate::format::format_tree;
    if ft.is_null() {
        ft0 = format_create(
            0 as *mut client,
            0 as *mut crate::cmd_queue::cmdq_item,
            0i32,
            0x4i32,
        );
        ft = ft0
    }
    sy = options_string_to_style(oo, name, ft);
    if sy.is_null() {
        sy = &mut style_default
    }
    if (*sy).gc.fg != 8i32 {
        (*gc).fg = (*sy).gc.fg
    }
    if (*sy).gc.bg != 8i32 {
        (*gc).bg = (*sy).gc.bg
    }
    (*gc).attr = ((*gc).attr as libc::c_int | (*sy).gc.attr as libc::c_int) as u_short;
    if !ft0.is_null() {
        format_free(ft0);
    };
}
/* Apply a style on top of the default style. */
#[no_mangle]
pub unsafe extern "C" fn style_apply(
    mut gc: *mut GridCell,
    mut oo: *mut crate::options::options,
    mut name: *const libc::c_char,
    mut ft: *mut crate::format::format_tree,
) {
    memcpy(
        gc as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    style_add(gc, oo, name, ft);
}
/* Initialize style from cell. */
#[no_mangle]
pub unsafe extern "C" fn style_set(mut sy: *mut Style, mut gc: *const GridCell) {
    memcpy(
        sy as *mut libc::c_void,
        &mut style_default as *mut Style as *const libc::c_void,
        ::std::mem::size_of::<Style>() as libc::c_ulong,
    );
    memcpy(
        &mut (*sy).gc as *mut GridCell as *mut libc::c_void,
        gc as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
}
/* Copy style. */
#[no_mangle]
pub unsafe extern "C" fn style_copy(mut dst: *mut Style, mut src: *mut Style) {
    memcpy(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        ::std::mem::size_of::<Style>() as libc::c_ulong,
    );
}
