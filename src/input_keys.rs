use crate::key_code::code as key_code_code;
use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut crate::options::options;
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn cmd_mouse_at(
        _: *mut window_pane,
        _: *mut mouse_event,
        _: *mut u_int,
        _: *mut u_int,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn utf8_to_data(_: utf8_char, _: *mut utf8_data);
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn log_get_level() -> libc::c_int;
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
    pub prompt_buffer: *mut utf8_data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: C2RustUnnamed_25,
    pub prompt_saved: *mut utf8_data,
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
    pub saved_cell: grid_cell,
    pub saved_flags: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: *mut crate::screen::screen_sel,
    pub write_list: *mut crate::screen_write::screen_write_collect_line,
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
    pub cached_gc: grid_cell,
    pub cached_active_gc: grid_cell,
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
    pub border_gc: grid_cell,
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
    pub style: grid_cell,
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
    pub cell: grid_cell,
    pub last_cell: grid_cell,
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
/* Entry in the key tree. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_key_entry {
    pub key: key_code,
    pub data: *const libc::c_char,
    pub entry: C2RustUnnamed_33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub rbe_left: *mut input_key_entry,
    pub rbe_right: *mut input_key_entry,
    pub rbe_parent: *mut input_key_entry,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_key_tree {
    pub rbh_root: *mut input_key_entry,
}
unsafe extern "C" fn input_key_tree_RB_INSERT_COLOR(
    mut head: *mut input_key_tree,
    mut elm: *mut input_key_entry,
) {
    let mut parent: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut gparent: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut tmp: *mut input_key_entry = 0 as *mut input_key_entry;
    loop {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
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
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
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
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
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
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
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
    (*(*head).rbh_root).entry.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn input_key_tree_RB_FIND(
    mut head: *mut input_key_tree,
    mut elm: *mut input_key_entry,
) -> *mut input_key_entry {
    let mut tmp: *mut input_key_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = input_key_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut input_key_entry;
}
unsafe extern "C" fn input_key_tree_RB_INSERT(
    mut head: *mut input_key_tree,
    mut elm: *mut input_key_entry,
) -> *mut input_key_entry {
    let mut tmp: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut parent: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = input_key_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut input_key_entry;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).entry.rbe_left = elm
        } else {
            (*parent).entry.rbe_right = elm
        }
    } else {
        (*head).rbh_root = elm
    }
    input_key_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut input_key_entry;
}
unsafe extern "C" fn input_key_tree_RB_MINMAX(
    mut head: *mut input_key_tree,
    mut val: libc::c_int,
) -> *mut input_key_entry {
    let mut tmp: *mut input_key_entry = (*head).rbh_root;
    let mut parent: *mut input_key_entry = 0 as *mut input_key_entry;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else {
            tmp = (*tmp).entry.rbe_right
        }
    }
    return parent;
}
unsafe extern "C" fn input_key_tree_RB_NEXT(mut elm: *mut input_key_entry) -> *mut input_key_entry {
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
#[no_mangle]
pub static mut input_key_tree: input_key_tree = {
    let mut init = input_key_tree {
        rbh_root: 0 as *const input_key_entry as *mut input_key_entry,
    };
    init
};
/* List of default keys, the tree is built from this. */
static mut input_key_defaults: [input_key_entry; 91] = [
    {
        let mut init = input_key_entry {
            key: key_code_code::PASTE_START as libc::c_ulong as key_code,
            data: b"\x1b[200~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::PASTE_END as libc::c_ulong as key_code,
            data: b"\x1b[201~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F1 as libc::c_ulong as key_code,
            data: b"\x1bOP\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F2 as libc::c_ulong as key_code,
            data: b"\x1bOQ\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F3 as libc::c_ulong as key_code,
            data: b"\x1bOR\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F4 as libc::c_ulong as key_code,
            data: b"\x1bOS\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F5 as libc::c_ulong as key_code,
            data: b"\x1b[15~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F6 as libc::c_ulong as key_code,
            data: b"\x1b[17~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F7 as libc::c_ulong as key_code,
            data: b"\x1b[18~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F8 as libc::c_ulong as key_code,
            data: b"\x1b[19~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F9 as libc::c_ulong as key_code,
            data: b"\x1b[20~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F10 as libc::c_ulong as key_code,
            data: b"\x1b[21~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F11 as libc::c_ulong as key_code,
            data: b"\x1b[23~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F12 as libc::c_ulong as key_code,
            data: b"\x1b[24~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F1 as libc::c_ulong as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1b[25~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F2 as libc::c_ulong as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1b[26~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F3 as libc::c_ulong as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1b[28~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F4 as libc::c_ulong as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1b[29~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F5 as libc::c_ulong as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1b[31~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F6 as libc::c_ulong as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1b[32~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F7 as libc::c_ulong as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1b[33~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F8 as libc::c_ulong as libc::c_ulonglong
                | 0x400000000000 as libc::c_ulonglong,
            data: b"\x1b[34~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::IC as libc::c_ulong as key_code,
            data: b"\x1b[2~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::DC as libc::c_ulong as key_code,
            data: b"\x1b[3~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::HOME as libc::c_ulong as key_code,
            data: b"\x1b[1~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::END as libc::c_ulong as key_code,
            data: b"\x1b[4~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::NPAGE as libc::c_ulong as key_code,
            data: b"\x1b[6~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::PPAGE as libc::c_ulong as key_code,
            data: b"\x1b[5~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::BTAB as libc::c_ulong as key_code,
            data: b"\x1b[Z\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::UP as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong,
            data: b"\x1bOA\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::DOWN as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong,
            data: b"\x1bOB\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::RIGHT as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong,
            data: b"\x1bOC\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::LEFT as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong,
            data: b"\x1bOD\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::UP as libc::c_ulong as key_code,
            data: b"\x1b[A\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::DOWN as libc::c_ulong as key_code,
            data: b"\x1b[B\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::RIGHT as libc::c_ulong as key_code,
            data: b"\x1b[C\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::LEFT as libc::c_ulong as key_code,
            data: b"\x1b[D\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_SLASH as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOo\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_STAR as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOj\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_MINUS as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOm\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_SEVEN as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOw\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_EIGHT as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOx\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_NINE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOy\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_PLUS as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOk\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_FOUR as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOt\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_FIVE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOu\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_SIX as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOv\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_ONE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOq\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_TWO as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOr\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_THREE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOs\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_ENTER as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOM\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_ZERO as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOp\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_PERIOD as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
            data: b"\x1bOn\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_SLASH as libc::c_ulong as key_code,
            data: b"/\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_STAR as libc::c_ulong as key_code,
            data: b"*\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_MINUS as libc::c_ulong as key_code,
            data: b"-\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_SEVEN as libc::c_ulong as key_code,
            data: b"7\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_EIGHT as libc::c_ulong as key_code,
            data: b"8\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_NINE as libc::c_ulong as key_code,
            data: b"9\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_PLUS as libc::c_ulong as key_code,
            data: b"+\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_FOUR as libc::c_ulong as key_code,
            data: b"4\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_FIVE as libc::c_ulong as key_code,
            data: b"5\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_SIX as libc::c_ulong as key_code,
            data: b"6\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_ONE as libc::c_ulong as key_code,
            data: b"1\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_TWO as libc::c_ulong as key_code,
            data: b"2\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_THREE as libc::c_ulong as key_code,
            data: b"3\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_ENTER as libc::c_ulong as key_code,
            data: b"\n\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_ZERO as libc::c_ulong as key_code,
            data: b"0\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::KP_PERIOD as libc::c_ulong as key_code,
            data: b".\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F1 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[1;_P\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F2 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[1;_Q\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F3 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[1;_R\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F4 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[1;_S\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F5 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[15;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F6 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[17;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F7 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[18;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F8 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[19;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F9 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[20;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F10 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[21;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F11 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[23;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::F12 as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[24;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::UP as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[1;_A\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::DOWN as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[1;_B\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::RIGHT as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[1;_C\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::LEFT as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[1;_D\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::HOME as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[1;_H\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::END as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[1;_F\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::PPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[5;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::NPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[6;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::IC as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[2;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
    {
        let mut init = input_key_entry {
            key: key_code_code::DC as libc::c_ulong as libc::c_ulonglong
                | 0x10000000000000 as libc::c_ulonglong,
            data: b"\x1b[3;_~\x00" as *const u8 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    },
];
static mut input_key_modifiers: [key_code; 9] = [
    0 as libc::c_int as key_code,
    0 as libc::c_int as key_code,
    0x400000000000 as libc::c_ulonglong,
    0x100000000000 as libc::c_ulonglong | 0x8000000000000 as libc::c_ulonglong,
    0x400000000000 as libc::c_ulonglong
        | 0x100000000000 as libc::c_ulonglong
        | 0x8000000000000 as libc::c_ulonglong,
    0x200000000000 as libc::c_ulonglong,
    0x400000000000 as libc::c_ulonglong | 0x200000000000 as libc::c_ulonglong,
    0x100000000000 as libc::c_ulonglong
        | 0x8000000000000 as libc::c_ulonglong
        | 0x200000000000 as libc::c_ulonglong,
    0x400000000000 as libc::c_ulonglong
        | 0x100000000000 as libc::c_ulonglong
        | 0x8000000000000 as libc::c_ulonglong
        | 0x200000000000 as libc::c_ulonglong,
];
/* Tree of input keys. */
/* Input key comparison function. */
unsafe extern "C" fn input_key_cmp(
    mut ike1: *mut input_key_entry,
    mut ike2: *mut input_key_entry,
) -> libc::c_int {
    if (*ike1).key < (*ike2).key {
        return -(1 as libc::c_int);
    }
    if (*ike1).key > (*ike2).key {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* Look for key in tree. */
unsafe extern "C" fn input_key_get(mut key: key_code) -> *mut input_key_entry {
    let mut entry: input_key_entry = {
        let mut init = input_key_entry {
            key: key,
            data: 0 as *const libc::c_char,
            entry: C2RustUnnamed_33 {
                rbe_left: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_right: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_parent: 0 as *const input_key_entry as *mut input_key_entry,
                rbe_color: 0,
            },
        };
        init
    };
    return input_key_tree_RB_FIND(&mut input_key_tree, &mut entry);
}
/* Split a character into two UTF-8 bytes. */
unsafe extern "C" fn input_key_split2(mut c: u_int, mut dst: *mut u_char) -> size_t {
    if c > 0x7f as libc::c_int as libc::c_uint {
        *dst.offset(0 as libc::c_int as isize) =
            (c >> 6 as libc::c_int | 0xc0 as libc::c_int as libc::c_uint) as u_char;
        *dst.offset(1 as libc::c_int as isize) = (c & 0x3f as libc::c_int as libc::c_uint
            | 0x80 as libc::c_int as libc::c_uint)
            as u_char;
        return 2 as libc::c_int as size_t;
    }
    *dst.offset(0 as libc::c_int as isize) = c as u_char;
    return 1 as libc::c_int as size_t;
}
/* Build input key tree. */
#[no_mangle]
pub unsafe extern "C" fn input_key_build() {
    let mut ike: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut new: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: key_code = 0;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[input_key_entry; 91]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<input_key_entry>() as libc::c_ulong)
    {
        ike = &mut *input_key_defaults.as_mut_ptr().offset(i as isize) as *mut input_key_entry;
        if !(*ike).key & 0x10000000000000 as libc::c_ulonglong != 0 {
            input_key_tree_RB_INSERT(&mut input_key_tree, ike);
        } else {
            j = 2 as libc::c_int as u_int;
            while (j as libc::c_ulong)
                < (::std::mem::size_of::<[key_code; 9]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<key_code>() as libc::c_ulong)
            {
                key = (*ike).key & !(0x10000000000000 as libc::c_ulonglong);
                data = xstrdup((*ike).data);
                *data
                    .offset(strcspn(data, b"_\x00" as *const u8 as *const libc::c_char) as isize) =
                    ('0' as i32 as libc::c_uint).wrapping_add(j) as libc::c_char;
                new = xcalloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<input_key_entry>() as libc::c_ulong,
                ) as *mut input_key_entry;
                (*new).key = key | input_key_modifiers[j as usize];
                (*new).data = data;
                input_key_tree_RB_INSERT(&mut input_key_tree, new);
                j = j.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    ike = input_key_tree_RB_MINMAX(&mut input_key_tree, -(1 as libc::c_int));
    while !ike.is_null() {
        log_debug(
            b"%s: 0x%llx (%s) is %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"input_key_build\x00"))
                .as_ptr(),
            (*ike).key,
            key_string_lookup_key((*ike).key, 1 as libc::c_int),
            (*ike).data,
        );
        ike = input_key_tree_RB_NEXT(ike)
    }
}
/* Translate a key code into an output key sequence for a pane. */
#[no_mangle]
pub unsafe extern "C" fn input_key_pane(
    mut wp: *mut window_pane,
    mut key: key_code,
    mut m: *mut mouse_event,
) -> libc::c_int {
    if log_get_level() != 0 as libc::c_int {
        log_debug(
            b"writing key 0x%llx (%s) to %%%u\x00" as *const u8 as *const libc::c_char,
            key,
            key_string_lookup_key(key, 1 as libc::c_int),
            (*wp).id,
        );
    }
    if key & 0xfffffffffff as libc::c_ulonglong
        >= key_code_code::MOUSE as libc::c_ulong as libc::c_ulonglong
        && (key & 0xfffffffffff as libc::c_ulonglong)
            < key_code_code::BSPACE as libc::c_ulong as libc::c_ulonglong
    {
        if !m.is_null() && (*m).wp != -(1 as libc::c_int) && (*m).wp as u_int == (*wp).id {
            input_key_mouse(wp, m);
        }
        return 0 as libc::c_int;
    }
    return input_key((*wp).screen, (*wp).event, key);
}
/* Translate a key code into an output key sequence. */
#[no_mangle]
pub unsafe extern "C" fn input_key(
    mut s: *mut screen,
    mut bev: *mut bufferevent,
    mut key: key_code,
) -> libc::c_int {
    let mut ike: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut justkey: key_code = 0;
    let mut newkey: key_code = 0;
    let mut outkey: key_code = 0;
    let mut ud: utf8_data = utf8_data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut modifier: libc::c_char = 0;
    /* Mouse keys need a pane. */
    if key & 0xfffffffffff as libc::c_ulonglong
        >= key_code_code::MOUSE as libc::c_ulong as libc::c_ulonglong
        && (key & 0xfffffffffff as libc::c_ulonglong)
            < key_code_code::BSPACE as libc::c_ulong as libc::c_ulonglong
    {
        return 0 as libc::c_int;
    }
    /* Literal keys go as themselves (can't be more than eight bits). */
    if key & 0x1000000000000 as libc::c_ulonglong != 0 {
        ud.data[0 as libc::c_int as usize] = key as u_char;
        bufferevent_write(
            bev,
            &mut *ud.data.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
                as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        return 0 as libc::c_int;
    }
    /* Is this backspace? */
    if key & 0xfffffffffff as libc::c_ulonglong
        == key_code_code::BSPACE as libc::c_ulong as libc::c_ulonglong
    {
        newkey = options_get_number(
            global_options,
            b"backspace\x00" as *const u8 as *const libc::c_char,
        ) as key_code;
        if newkey >= 0x7f as libc::c_int as libc::c_ulonglong {
            newkey = '\u{7f}' as i32 as key_code
        }
        key = newkey
            | key & (0xf00000000000 as libc::c_ulonglong | 0xff000000000000 as libc::c_ulonglong)
    }
    /*
     * If this is a normal 7-bit key, just send it, with a leading escape
     * if necessary. If it is a UTF-8 key, split it and send it.
     */
    justkey = key & !(0x100000000000 as libc::c_ulonglong | 0x8000000000000 as libc::c_ulonglong);
    if justkey <= 0x7f as libc::c_int as libc::c_ulonglong {
        if key & 0x100000000000 as libc::c_ulonglong != 0 {
            bufferevent_write(
                bev,
                b"\x1b\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        ud.data[0 as libc::c_int as usize] = justkey as u_char;
        bufferevent_write(
            bev,
            &mut *ud.data.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut u_char
                as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        return 0 as libc::c_int;
    }
    if justkey > 0x7f as libc::c_int as libc::c_ulonglong
        && justkey < 0x1000000000 as libc::c_ulonglong
    {
        if key & 0x100000000000 as libc::c_ulonglong != 0 {
            bufferevent_write(
                bev,
                b"\x1b\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        utf8_to_data(justkey as utf8_char, &mut ud);
        bufferevent_write(
            bev,
            ud.data.as_mut_ptr() as *const libc::c_void,
            ud.size as size_t,
        );
        return 0 as libc::c_int;
    }
    /*
     * Look up in the tree. If not in application keypad or cursor mode,
     * remove the flags from the key.
     */
    if !(*s).mode & 0x8 as libc::c_int != 0 {
        key &= !(0x2000000000000 as libc::c_ulonglong)
    }
    if !(*s).mode & 0x4 as libc::c_int != 0 {
        key &= !(0x4000000000000 as libc::c_ulonglong)
    }
    ike = input_key_get(key);
    if ike.is_null()
        && key & 0x100000000000 as libc::c_ulonglong != 0
        && !key & 0x8000000000000 as libc::c_ulonglong != 0
    {
        ike = input_key_get(key & !(0x100000000000 as libc::c_ulonglong))
    }
    if ike.is_null() && key & 0x4000000000000 as libc::c_ulonglong != 0 {
        ike = input_key_get(key & !(0x4000000000000 as libc::c_ulonglong))
    }
    if ike.is_null() && key & 0x2000000000000 as libc::c_ulonglong != 0 {
        ike = input_key_get(key & !(0x2000000000000 as libc::c_ulonglong))
    }
    if !ike.is_null() {
        log_debug(
            b"found key 0x%llx: \"%s\"\x00" as *const u8 as *const libc::c_char,
            key,
            (*ike).data,
        );
        if key & 0x100000000000 as libc::c_ulonglong != 0
            && !key & 0x8000000000000 as libc::c_ulonglong != 0
        {
            bufferevent_write(
                bev,
                b"\x1b\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        bufferevent_write(bev, (*ike).data as *const libc::c_void, strlen((*ike).data));
        return 0 as libc::c_int;
    }
    /* No builtin key sequence; construct an extended key sequence. */
    if !(*s).mode & 0x8000 as libc::c_int != 0 {
        if key & 0xf00000000000 as libc::c_ulonglong != 0x200000000000 as libc::c_ulonglong {
            log_debug(
                b"key 0x%llx missing\x00" as *const u8 as *const libc::c_char,
                key,
            );
            return -(1 as libc::c_int);
        } else {
            justkey = key & 0xfffffffffff as libc::c_ulonglong;
            match justkey {
                32 | 50 => {
                    key = 0 as libc::c_int as libc::c_ulonglong
                        | key & !(0xfffffffffff as libc::c_ulonglong)
                }
                124 => {
                    key = 28 as libc::c_int as libc::c_ulonglong
                        | key & !(0xfffffffffff as libc::c_ulonglong)
                }
                54 => {
                    key = 30 as libc::c_int as libc::c_ulonglong
                        | key & !(0xfffffffffff as libc::c_ulonglong)
                }
                45 | 47 => {
                    key = 31 as libc::c_int as libc::c_ulonglong
                        | key & !(0xfffffffffff as libc::c_ulonglong)
                }
                63 => {
                    key = 127 as libc::c_int as libc::c_ulonglong
                        | key & !(0xfffffffffff as libc::c_ulonglong)
                }
                _ => {
                    if justkey >= 'A' as i32 as libc::c_ulonglong
                        && justkey <= '_' as i32 as libc::c_ulonglong
                    {
                        key = justkey.wrapping_sub('A' as i32 as libc::c_ulonglong)
                            | key & !(0xfffffffffff as libc::c_ulonglong)
                    } else if justkey >= 'a' as i32 as libc::c_ulonglong
                        && justkey <= '~' as i32 as libc::c_ulonglong
                    {
                        key = justkey.wrapping_sub(96 as libc::c_int as libc::c_ulonglong)
                            | key & !(0xfffffffffff as libc::c_ulonglong)
                    } else {
                        return 0 as libc::c_int;
                    }
                }
            }
            return input_key(s, bev, key & !(0x200000000000 as libc::c_ulonglong));
        }
    } else {
        outkey = key & 0xfffffffffff as libc::c_ulonglong;
        match key & 0xf00000000000 as libc::c_ulonglong {
            70368744177664 => modifier = '2' as i32 as libc::c_char,
            17592186044416 => modifier = '3' as i32 as libc::c_char,
            87960930222080 => modifier = '4' as i32 as libc::c_char,
            35184372088832 => modifier = '5' as i32 as libc::c_char,
            105553116266496 => modifier = '6' as i32 as libc::c_char,
            52776558133248 => modifier = '7' as i32 as libc::c_char,
            123145302310912 => modifier = '8' as i32 as libc::c_char,
            _ => {
                fatalx(
                    b"invalid key modifiers: %llx\x00" as *const u8 as *const libc::c_char,
                    key,
                );
            }
        }
        xsnprintf(
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"\x1b[%llu;%cu\x00" as *const u8 as *const libc::c_char,
            outkey,
            modifier as libc::c_int,
        );
        bufferevent_write(
            bev,
            tmp.as_mut_ptr() as *const libc::c_void,
            strlen(tmp.as_mut_ptr()),
        );
        return 0 as libc::c_int;
    };
}
/* Get mouse event string. */
#[no_mangle]
pub unsafe extern "C" fn input_key_get_mouse(
    mut s: *mut screen,
    mut m: *mut mouse_event,
    mut x: u_int,
    mut y: u_int,
    mut rbuf: *mut *const libc::c_char,
    mut rlen: *mut size_t,
) -> libc::c_int {
    static mut buf: [libc::c_char; 40] = [0; 40];
    let mut len: size_t = 0;
    *rbuf = 0 as *const libc::c_char;
    *rlen = 0 as libc::c_int as size_t;
    /* If this pane is not in button or all mode, discard motion events. */
    if (*m).b & 32 as libc::c_int as libc::c_uint != 0
        && (*s).mode & (0x40 as libc::c_int | 0x1000 as libc::c_int) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*s).mode & (0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    /*
     * If this event is a release event and not in all mode, discard it.
     * In SGR mode we can tell absolutely because a release is normally
     * shown by the last character. Without SGR, we check if the last
     * buttons was also a release.
     */
    if (*m).sgr_type != ' ' as i32 as libc::c_uint {
        if (*m).sgr_b & 32 as libc::c_int as libc::c_uint != 0
            && (*m).sgr_b & 3 as libc::c_int as libc::c_uint == 3 as libc::c_int as libc::c_uint
            && !(*s).mode & 0x1000 as libc::c_int != 0
        {
            return 0 as libc::c_int;
        }
    } else if (*m).b & 32 as libc::c_int as libc::c_uint != 0
        && (*m).b & 3 as libc::c_int as libc::c_uint == 3 as libc::c_int as libc::c_uint
        && (*m).lb & 3 as libc::c_int as libc::c_uint == 3 as libc::c_int as libc::c_uint
        && !(*s).mode & 0x1000 as libc::c_int != 0
    {
        return 0 as libc::c_int;
    }
    /*
     * Use the SGR (1006) extension only if the application requested it
     * and the underlying terminal also sent the event in this format (this
     * is because an old style mouse release event cannot be converted into
     * the new SGR format, since the released button is unknown). Otherwise
     * pretend that tmux doesn't speak this extension, and fall back to the
     * UTF-8 (1005) extension if the application requested, or to the
     * legacy format.
     */
    if (*m).sgr_type != ' ' as i32 as libc::c_uint && (*s).mode & 0x200 as libc::c_int != 0 {
        len = xsnprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
            b"\x1b[<%u;%u;%u%c\x00" as *const u8 as *const libc::c_char,
            (*m).sgr_b,
            x.wrapping_add(1 as libc::c_int as libc::c_uint),
            y.wrapping_add(1 as libc::c_int as libc::c_uint),
            (*m).sgr_type,
        ) as size_t
    } else if (*s).mode & 0x100 as libc::c_int != 0 {
        if (*m).b > (0x7ff as libc::c_int - 32 as libc::c_int) as libc::c_uint
            || x > (0x7ff as libc::c_int - 33 as libc::c_int) as libc::c_uint
            || y > (0x7ff as libc::c_int - 33 as libc::c_int) as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        len = xsnprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
            b"\x1b[M\x00" as *const u8 as *const libc::c_char,
        ) as size_t;
        len = (len as libc::c_ulong).wrapping_add(input_key_split2(
            (*m).b.wrapping_add(32 as libc::c_int as libc::c_uint),
            &mut *buf.as_mut_ptr().offset(len as isize) as *mut libc::c_char as *mut u_char,
        )) as size_t as size_t;
        len = (len as libc::c_ulong).wrapping_add(input_key_split2(
            x.wrapping_add(33 as libc::c_int as libc::c_uint),
            &mut *buf.as_mut_ptr().offset(len as isize) as *mut libc::c_char as *mut u_char,
        )) as size_t as size_t;
        len = (len as libc::c_ulong).wrapping_add(input_key_split2(
            y.wrapping_add(33 as libc::c_int as libc::c_uint),
            &mut *buf.as_mut_ptr().offset(len as isize) as *mut libc::c_char as *mut u_char,
        )) as size_t as size_t
    } else {
        if (*m).b > 223 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int;
        }
        len = xsnprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
            b"\x1b[M\x00" as *const u8 as *const libc::c_char,
        ) as size_t;
        let fresh0 = len;
        len = len.wrapping_add(1);
        buf[fresh0 as usize] =
            (*m).b.wrapping_add(32 as libc::c_int as libc::c_uint) as libc::c_char;
        let fresh1 = len;
        len = len.wrapping_add(1);
        buf[fresh1 as usize] = x.wrapping_add(33 as libc::c_int as libc::c_uint) as libc::c_char;
        let fresh2 = len;
        len = len.wrapping_add(1);
        buf[fresh2 as usize] = y.wrapping_add(33 as libc::c_int as libc::c_uint) as libc::c_char
    }
    *rbuf = buf.as_mut_ptr();
    *rlen = len;
    return 1 as libc::c_int;
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
/*
 * This file is rather misleadingly named, it contains the code which takes a
 * key code and translates it into something suitable to be sent to the
 * application running in a pane (similar to input.c does in the other
 * direction with output).
 */
/* Translate mouse and output. */
unsafe extern "C" fn input_key_mouse(mut wp: *mut window_pane, mut m: *mut mouse_event) {
    let mut s: *mut screen = (*wp).screen;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    /* Ignore events if no mouse mode or the pane is not visible. */
    if (*m).ignore != 0
        || (*s).mode & (0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int)
            == 0 as libc::c_int
    {
        return;
    }
    if cmd_mouse_at(wp, m, &mut x, &mut y, 0 as libc::c_int) != 0 as libc::c_int {
        return;
    }
    if window_pane_visible(wp) == 0 {
        return;
    }
    if input_key_get_mouse(s, m, x, y, &mut buf, &mut len) == 0 {
        return;
    }
    log_debug(
        b"writing mouse %.*s to %%%u\x00" as *const u8 as *const libc::c_char,
        len as libc::c_int,
        buf,
        (*wp).id,
    );
    bufferevent_write((*wp).event, buf as *const libc::c_void, len);
}
