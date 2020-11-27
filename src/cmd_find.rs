use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type screen_write_collect_line;
    pub type screen_sel;
    pub type screen_titles;
    pub type environ;
    pub type options;
    pub type format_tree;
    pub type input_ctx;
    pub type cmds;
    pub type tty_code;
    pub type format_job_tree;
    pub type control_state;
    pub type cmdq_list;
    pub type tmuxpeer;
    pub type cmdq_item;
    #[no_mangle]
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn environ_find(_: *mut crate::environ::environ, _: *const libc::c_char) -> *mut environ_entry;
    #[no_mangle]
    fn cmdq_error(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn cmdq_get_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_check_marked() -> libc::c_int;
    #[no_mangle]
    fn cmd_mouse_window(_: *mut mouse_event, _: *mut *mut session) -> *mut winlink;
    #[no_mangle]
    fn cmd_mouse_pane(
        _: *mut mouse_event,
        _: *mut *mut session,
        _: *mut *mut winlink,
    ) -> *mut window_pane;
    #[no_mangle]
    fn cmdq_get_event(_: *mut crate::cmd_queue::cmdq_item) -> *mut key_event;
    #[no_mangle]
    fn cmdq_get_current(_: *mut crate::cmd_queue::cmdq_item) -> *mut cmd_find_state;
    #[no_mangle]
    fn server_client_get_pane(_: *mut client) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_tree_RB_MINMAX(_: *mut window_pane_tree, _: libc::c_int) -> *mut window_pane;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_pane_tree_RB_NEXT(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn window_pane_find_by_id_str(_: *const libc::c_char) -> *mut window_pane;
    #[no_mangle]
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_previous_by_number(_: *mut winlink, _: *mut session, _: libc::c_int)
        -> *mut winlink;
    #[no_mangle]
    fn winlink_next_by_number(_: *mut winlink, _: *mut session, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn window_find_by_id_str(_: *const libc::c_char) -> *mut window;
    #[no_mangle]
    fn window_find_string(_: *mut window, _: *const libc::c_char) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_at_index(_: *mut window, _: u_int) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_previous_by_number(
        _: *mut window,
        _: *mut window_pane,
        _: u_int,
    ) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_next_by_number(
        _: *mut window,
        _: *mut window_pane,
        _: u_int,
    ) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_find_right(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_find_left(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_find_down(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_find_up(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    fn window_has_pane(_: *mut window, _: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    fn session_alive(_: *mut session) -> libc::c_int;
    #[no_mangle]
    fn session_find_by_id_str(_: *const libc::c_char) -> *mut session;
    #[no_mangle]
    fn session_find(_: *const libc::c_char) -> *mut session;
    #[no_mangle]
    fn session_has(_: *mut session, _: *mut window) -> libc::c_int;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
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
    pub exit_msgtype: msgtype,
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
pub type msgtype = libc::c_uint;
pub const MSG_WRITE_CLOSE: msgtype = 306;
pub const MSG_WRITE_READY: msgtype = 305;
pub const MSG_WRITE: msgtype = 304;
pub const MSG_WRITE_OPEN: msgtype = 303;
pub const MSG_READ_DONE: msgtype = 302;
pub const MSG_READ: msgtype = 301;
pub const MSG_READ_OPEN: msgtype = 300;
pub const MSG_FLAGS: msgtype = 218;
pub const MSG_EXEC: msgtype = 217;
pub const MSG_WAKEUP: msgtype = 216;
pub const MSG_UNLOCK: msgtype = 215;
pub const MSG_SUSPEND: msgtype = 214;
pub const MSG_OLDSTDOUT: msgtype = 213;
pub const MSG_OLDSTDIN: msgtype = 212;
pub const MSG_OLDSTDERR: msgtype = 211;
pub const MSG_SHUTDOWN: msgtype = 210;
pub const MSG_SHELL: msgtype = 209;
pub const MSG_RESIZE: msgtype = 208;
pub const MSG_READY: msgtype = 207;
pub const MSG_LOCK: msgtype = 206;
pub const MSG_EXITING: msgtype = 205;
pub const MSG_EXITED: msgtype = 204;
pub const MSG_EXIT: msgtype = 203;
pub const MSG_DETACHKILL: msgtype = 202;
pub const MSG_DETACH: msgtype = 201;
pub const MSG_COMMAND: msgtype = 200;
pub const MSG_IDENTIFY_LONGFLAGS: msgtype = 111;
pub const MSG_IDENTIFY_STDOUT: msgtype = 110;
pub const MSG_IDENTIFY_FEATURES: msgtype = 109;
pub const MSG_IDENTIFY_CWD: msgtype = 108;
pub const MSG_IDENTIFY_CLIENTPID: msgtype = 107;
pub const MSG_IDENTIFY_DONE: msgtype = 106;
pub const MSG_IDENTIFY_ENVIRON: msgtype = 105;
pub const MSG_IDENTIFY_STDIN: msgtype = 104;
pub const MSG_IDENTIFY_OLDCWD: msgtype = 103;
pub const MSG_IDENTIFY_TTYNAME: msgtype = 102;
pub const MSG_IDENTIFY_TERM: msgtype = 101;
pub const MSG_IDENTIFY_FLAGS: msgtype = 100;
pub const MSG_VERSION: msgtype = 12;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub type cmd_find_type = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
static mut cmd_find_session_table: [[*const libc::c_char; 2]; 1] =
    [[0 as *const libc::c_char, 0 as *const libc::c_char]];
static mut cmd_find_window_table: [[*const libc::c_char; 2]; 6] = [
    [
        b"{start}\x00" as *const u8 as *const libc::c_char,
        b"^\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{last}\x00" as *const u8 as *const libc::c_char,
        b"!\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{end}\x00" as *const u8 as *const libc::c_char,
        b"$\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{next}\x00" as *const u8 as *const libc::c_char,
        b"+\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{previous}\x00" as *const u8 as *const libc::c_char,
        b"-\x00" as *const u8 as *const libc::c_char,
    ],
    [0 as *const libc::c_char, 0 as *const libc::c_char],
];
static mut cmd_find_pane_table: [[*const libc::c_char; 2]; 16] = [
    [
        b"{last}\x00" as *const u8 as *const libc::c_char,
        b"!\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{next}\x00" as *const u8 as *const libc::c_char,
        b"+\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{previous}\x00" as *const u8 as *const libc::c_char,
        b"-\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{top}\x00" as *const u8 as *const libc::c_char,
        b"top\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{bottom}\x00" as *const u8 as *const libc::c_char,
        b"bottom\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{left}\x00" as *const u8 as *const libc::c_char,
        b"left\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{right}\x00" as *const u8 as *const libc::c_char,
        b"right\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{top-left}\x00" as *const u8 as *const libc::c_char,
        b"top-left\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{top-right}\x00" as *const u8 as *const libc::c_char,
        b"top-right\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{bottom-left}\x00" as *const u8 as *const libc::c_char,
        b"bottom-left\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{bottom-right}\x00" as *const u8 as *const libc::c_char,
        b"bottom-right\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{up-of}\x00" as *const u8 as *const libc::c_char,
        b"{up-of}\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{down-of}\x00" as *const u8 as *const libc::c_char,
        b"{down-of}\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{left-of}\x00" as *const u8 as *const libc::c_char,
        b"{left-of}\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"{right-of}\x00" as *const u8 as *const libc::c_char,
        b"{right-of}\x00" as *const u8 as *const libc::c_char,
    ],
    [0 as *const libc::c_char, 0 as *const libc::c_char],
];
/* Find pane containing client if any. */
unsafe extern "C" fn cmd_find_inside_pane(mut c: *mut client) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    if c.is_null() {
        return 0 as *mut window_pane;
    }
    wp = window_pane_tree_RB_MINMAX(&mut all_window_panes, -(1 as libc::c_int));
    while !wp.is_null() {
        if (*wp).fd != -(1 as libc::c_int)
            && strcmp((*wp).tty.as_mut_ptr(), (*c).ttyname) == 0 as libc::c_int
        {
            break;
        }
        wp = window_pane_tree_RB_NEXT(wp)
    }
    if wp.is_null() {
        envent = environ_find(
            (*c).environ,
            b"TMUX_PANE\x00" as *const u8 as *const libc::c_char,
        );
        if !envent.is_null() {
            wp = window_pane_find_by_id_str((*envent).value)
        }
    }
    if !wp.is_null() {
        log_debug(
            b"%s: got pane %%%u (%s)\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"cmd_find_inside_pane\x00"))
                .as_ptr(),
            (*wp).id,
            (*wp).tty.as_mut_ptr(),
        );
    }
    return wp;
}
/* Is this client better? */
unsafe extern "C" fn cmd_find_client_better(
    mut c: *mut client,
    mut than: *mut client,
) -> libc::c_int {
    if than.is_null() {
        return 1 as libc::c_int;
    }
    return if (*c).activity_time.tv_sec == (*than).activity_time.tv_sec {
        ((*c).activity_time.tv_usec > (*than).activity_time.tv_usec) as libc::c_int
    } else {
        ((*c).activity_time.tv_sec > (*than).activity_time.tv_sec) as libc::c_int
    };
}
/* Find best client for session. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_best_client(mut s: *mut session) -> *mut client {
    let mut c_loop: *mut client = 0 as *mut client;
    let mut c: *mut client = 0 as *mut client;
    if (*s).attached == 0 as libc::c_int as libc::c_uint {
        s = 0 as *mut session
    }
    c = 0 as *mut client;
    c_loop = clients.tqh_first;
    while !c_loop.is_null() {
        if !(*c_loop).session.is_null() {
            if !(!s.is_null() && (*c_loop).session != s) {
                if cmd_find_client_better(c_loop, c) != 0 {
                    c = c_loop
                }
            }
        }
        c_loop = (*c_loop).entry.tqe_next
    }
    return c;
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2015 Nicholas Marriott <nicholas.marriott@gmail.com>
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
/* Is this session better? */
unsafe extern "C" fn cmd_find_session_better(
    mut s: *mut session,
    mut than: *mut session,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut attached: libc::c_int = 0;
    if than.is_null() {
        return 1 as libc::c_int;
    }
    if flags & 0x1 as libc::c_int != 0 {
        attached = ((*than).attached != 0 as libc::c_int as libc::c_uint) as libc::c_int;
        if attached != 0 && (*s).attached == 0 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int;
        } else {
            if attached == 0 && (*s).attached != 0 as libc::c_int as libc::c_uint {
                return 0 as libc::c_int;
            }
        }
    }
    return if (*s).activity_time.tv_sec == (*than).activity_time.tv_sec {
        ((*s).activity_time.tv_usec > (*than).activity_time.tv_usec) as libc::c_int
    } else {
        ((*s).activity_time.tv_sec > (*than).activity_time.tv_sec) as libc::c_int
    };
}
/* Find best session from a list, or all if list is NULL. */
unsafe extern "C" fn cmd_find_best_session(
    mut slist: *mut *mut session,
    mut ssize: u_int,
    mut flags: libc::c_int,
) -> *mut session {
    let mut s_loop: *mut session = 0 as *mut session;
    let mut s: *mut session = 0 as *mut session;
    let mut i: u_int = 0;
    log_debug(
        b"%s: %u sessions to try\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"cmd_find_best_session\x00"))
            .as_ptr(),
        ssize,
    );
    s = 0 as *mut session;
    if !slist.is_null() {
        i = 0 as libc::c_int as u_int;
        while i < ssize {
            if cmd_find_session_better(*slist.offset(i as isize), s, flags) != 0 {
                s = *slist.offset(i as isize)
            }
            i = i.wrapping_add(1)
        }
    } else {
        s_loop = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
        while !s_loop.is_null() {
            if cmd_find_session_better(s_loop, s, flags) != 0 {
                s = s_loop
            }
            s_loop = sessions_RB_NEXT(s_loop)
        }
    }
    return s;
}
/* Find best session and winlink for window. */
unsafe extern "C" fn cmd_find_best_session_with_window(mut fs: *mut cmd_find_state) -> libc::c_int {
    let mut slist: *mut *mut session = 0 as *mut *mut session;
    let mut ssize: u_int = 0;
    let mut s: *mut session = 0 as *mut session;
    log_debug(
        b"%s: window is @%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
            b"cmd_find_best_session_with_window\x00",
        ))
        .as_ptr(),
        (*(*fs).w).id,
    );
    ssize = 0 as libc::c_int as u_int;
    s = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s.is_null() {
        if !(session_has(s, (*fs).w) == 0) {
            slist = xreallocarray(
                slist as *mut libc::c_void,
                ssize.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                ::std::mem::size_of::<*mut session>() as libc::c_ulong,
            ) as *mut *mut session;
            let fresh0 = ssize;
            ssize = ssize.wrapping_add(1);
            let ref mut fresh1 = *slist.offset(fresh0 as isize);
            *fresh1 = s
        }
        s = sessions_RB_NEXT(s)
    }
    if !(ssize == 0 as libc::c_int as libc::c_uint) {
        (*fs).s = cmd_find_best_session(slist, ssize, (*fs).flags);
        if !(*fs).s.is_null() {
            free(slist as *mut libc::c_void);
            return cmd_find_best_winlink_with_window(fs);
        }
    }
    free(slist as *mut libc::c_void);
    return -(1 as libc::c_int);
}
/*
 * Find the best winlink for a window (the current if it contains the window,
 * otherwise the first).
 */
unsafe extern "C" fn cmd_find_best_winlink_with_window(mut fs: *mut cmd_find_state) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wl_loop: *mut winlink = 0 as *mut winlink;
    log_debug(
        b"%s: window is @%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
            b"cmd_find_best_winlink_with_window\x00",
        ))
        .as_ptr(),
        (*(*fs).w).id,
    );
    wl = 0 as *mut winlink;
    if !(*(*fs).s).curw.is_null() && (*(*(*fs).s).curw).window == (*fs).w {
        wl = (*(*fs).s).curw
    } else {
        wl_loop = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
        while !wl_loop.is_null() {
            if (*wl_loop).window == (*fs).w {
                wl = wl_loop;
                break;
            } else {
                wl_loop = winlinks_RB_NEXT(wl_loop)
            }
        }
    }
    if wl.is_null() {
        return -(1 as libc::c_int);
    }
    (*fs).wl = wl;
    (*fs).idx = (*(*fs).wl).idx;
    return 0 as libc::c_int;
}
/* Maps string in table. */
unsafe extern "C" fn cmd_find_map_table(
    mut table: *mut [*const libc::c_char; 2],
    mut s: *const libc::c_char,
) -> *const libc::c_char {
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while !(*table.offset(i as isize))[0 as libc::c_int as usize].is_null() {
        if strcmp(s, (*table.offset(i as isize))[0 as libc::c_int as usize]) == 0 as libc::c_int {
            return (*table.offset(i as isize))[1 as libc::c_int as usize];
        }
        i = i.wrapping_add(1)
    }
    return s;
}
/* Find session from string. Fills in s. */
unsafe extern "C" fn cmd_find_get_session(
    mut fs: *mut cmd_find_state,
    mut session: *const libc::c_char,
) -> libc::c_int {
    let mut s: *mut session = 0 as *mut session;
    let mut s_loop: *mut session = 0 as *mut session;
    let mut c: *mut client = 0 as *mut client;
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"cmd_find_get_session\x00"))
            .as_ptr(),
        session,
    );
    /* Check for session ids starting with $. */
    if *session as libc::c_int == '$' as i32 {
        (*fs).s = session_find_by_id_str(session);
        if (*fs).s.is_null() {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    /* Look for exactly this session. */
    (*fs).s = session_find(session);
    if !(*fs).s.is_null() {
        return 0 as libc::c_int;
    }
    /* Look for as a client. */
    c = cmd_find_client(
        0 as *mut crate::cmd_queue::cmdq_item,
        session,
        1 as libc::c_int,
    );
    if !c.is_null() && !(*c).session.is_null() {
        (*fs).s = (*c).session;
        return 0 as libc::c_int;
    }
    /* Stop now if exact only. */
    if (*fs).flags & 0x10 as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    /* Otherwise look for prefix. */
    s = 0 as *mut session;
    s_loop = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s_loop.is_null() {
        if strncmp(session, (*s_loop).name, strlen(session)) == 0 as libc::c_int {
            if !s.is_null() {
                return -(1 as libc::c_int);
            }
            s = s_loop
        }
        s_loop = sessions_RB_NEXT(s_loop)
    }
    if !s.is_null() {
        (*fs).s = s;
        return 0 as libc::c_int;
    }
    /* Then as a pattern. */
    s = 0 as *mut session;
    s_loop = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s_loop.is_null() {
        if fnmatch(session, (*s_loop).name, 0 as libc::c_int) == 0 as libc::c_int {
            if !s.is_null() {
                return -(1 as libc::c_int);
            }
            s = s_loop
        }
        s_loop = sessions_RB_NEXT(s_loop)
    }
    if !s.is_null() {
        (*fs).s = s;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
/* Find window from string. Fills in s, wl, w. */
unsafe extern "C" fn cmd_find_get_window(
    mut fs: *mut cmd_find_state,
    mut window: *const libc::c_char,
    mut only: libc::c_int,
) -> libc::c_int {
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"cmd_find_get_window\x00"))
            .as_ptr(),
        window,
    );
    /* Check for window ids starting with @. */
    if *window as libc::c_int == '@' as i32 {
        (*fs).w = window_find_by_id_str(window);
        if (*fs).w.is_null() {
            return -(1 as libc::c_int);
        }
        return cmd_find_best_session_with_window(fs);
    }
    /* Not a window id, so use the current session. */
    (*fs).s = (*(*fs).current).s;
    /* We now only need to find the winlink in this session. */
    if cmd_find_get_window_with_session(fs, window) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Otherwise try as a session itself. */
    if only == 0 && cmd_find_get_session(fs, window) == 0 as libc::c_int {
        (*fs).wl = (*(*fs).s).curw;
        (*fs).w = (*(*fs).wl).window;
        if !(*fs).flags & 0x4 as libc::c_int != 0 {
            (*fs).idx = (*(*fs).wl).idx
        }
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
/*
 * Find window from string, assuming it is in given session. Needs s, fills in
 * wl and w.
 */
unsafe extern "C" fn cmd_find_get_window_with_session(
    mut fs: *mut cmd_find_state,
    mut window: *const libc::c_char,
) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut exact: libc::c_int = 0;
    let mut s: *mut session = 0 as *mut session;
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
            b"cmd_find_get_window_with_session\x00",
        ))
        .as_ptr(),
        window,
    );
    exact = (*fs).flags & 0x20 as libc::c_int;
    /*
     * Start with the current window as the default. So if only an index is
     * found, the window will be the current.
     */
    (*fs).wl = (*(*fs).s).curw;
    (*fs).w = (*(*fs).wl).window;
    /* Check for window ids starting with @. */
    if *window as libc::c_int == '@' as i32 {
        (*fs).w = window_find_by_id_str(window);
        if (*fs).w.is_null() || session_has((*fs).s, (*fs).w) == 0 {
            return -(1 as libc::c_int);
        }
        return cmd_find_best_winlink_with_window(fs);
    }
    /* Try as an offset. */
    if exact == 0
        && (*window.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
            || *window.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32)
    {
        if *window.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
            n = strtonum(
                window.offset(1 as libc::c_int as isize),
                1 as libc::c_int as libc::c_longlong,
                2147483647 as libc::c_int as libc::c_longlong,
                0 as *mut *const libc::c_char,
            ) as libc::c_int
        } else {
            n = 1 as libc::c_int
        }
        s = (*fs).s;
        if (*fs).flags & 0x4 as libc::c_int != 0 {
            if *window.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
                if 2147483647 as libc::c_int - (*(*s).curw).idx < n {
                    return -(1 as libc::c_int);
                }
                (*fs).idx = (*(*s).curw).idx + n
            } else {
                if n > (*(*s).curw).idx {
                    return -(1 as libc::c_int);
                }
                (*fs).idx = (*(*s).curw).idx - n
            }
            return 0 as libc::c_int;
        }
        if *window.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
            (*fs).wl = winlink_next_by_number((*s).curw, s, n)
        } else {
            (*fs).wl = winlink_previous_by_number((*s).curw, s, n)
        }
        if !(*fs).wl.is_null() {
            (*fs).idx = (*(*fs).wl).idx;
            (*fs).w = (*(*fs).wl).window;
            return 0 as libc::c_int;
        }
    }
    /* Try special characters. */
    if exact == 0 {
        if strcmp(window, b"!\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            (*fs).wl = (*(*fs).s).lastw.tqh_first;
            if (*fs).wl.is_null() {
                return -(1 as libc::c_int);
            }
            (*fs).idx = (*(*fs).wl).idx;
            (*fs).w = (*(*fs).wl).window;
            return 0 as libc::c_int;
        } else {
            if strcmp(window, b"^\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*fs).wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
                if (*fs).wl.is_null() {
                    return -(1 as libc::c_int);
                }
                (*fs).idx = (*(*fs).wl).idx;
                (*fs).w = (*(*fs).wl).window;
                return 0 as libc::c_int;
            } else {
                if strcmp(window, b"$\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                {
                    (*fs).wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, 1 as libc::c_int);
                    if (*fs).wl.is_null() {
                        return -(1 as libc::c_int);
                    }
                    (*fs).idx = (*(*fs).wl).idx;
                    (*fs).w = (*(*fs).wl).window;
                    return 0 as libc::c_int;
                }
            }
        }
    }
    /* First see if this is a valid window index in this session. */
    if *window.offset(0 as libc::c_int as isize) as libc::c_int != '+' as i32
        && *window.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
    {
        idx = strtonum(
            window,
            0 as libc::c_int as libc::c_longlong,
            2147483647 as libc::c_int as libc::c_longlong,
            &mut errstr,
        ) as libc::c_int;
        if errstr.is_null() {
            (*fs).wl = winlink_find_by_index(&mut (*(*fs).s).windows, idx);
            if !(*fs).wl.is_null() {
                (*fs).idx = (*(*fs).wl).idx;
                (*fs).w = (*(*fs).wl).window;
                return 0 as libc::c_int;
            }
            if (*fs).flags & 0x4 as libc::c_int != 0 {
                (*fs).idx = idx;
                return 0 as libc::c_int;
            }
        }
    }
    /* Look for exact matches, error if more than one. */
    (*fs).wl = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if strcmp(window, (*(*wl).window).name) == 0 as libc::c_int {
            if !(*fs).wl.is_null() {
                return -(1 as libc::c_int);
            }
            (*fs).wl = wl
        }
        wl = winlinks_RB_NEXT(wl)
    }
    if !(*fs).wl.is_null() {
        (*fs).idx = (*(*fs).wl).idx;
        (*fs).w = (*(*fs).wl).window;
        return 0 as libc::c_int;
    }
    /* Stop now if exact only. */
    if exact != 0 {
        return -(1 as libc::c_int);
    }
    /* Try as the start of a window name, error if multiple. */
    (*fs).wl = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if strncmp(window, (*(*wl).window).name, strlen(window)) == 0 as libc::c_int {
            if !(*fs).wl.is_null() {
                return -(1 as libc::c_int);
            }
            (*fs).wl = wl
        }
        wl = winlinks_RB_NEXT(wl)
    }
    if !(*fs).wl.is_null() {
        (*fs).idx = (*(*fs).wl).idx;
        (*fs).w = (*(*fs).wl).window;
        return 0 as libc::c_int;
    }
    /* Now look for pattern matches, again error if multiple. */
    (*fs).wl = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if fnmatch(window, (*(*wl).window).name, 0 as libc::c_int) == 0 as libc::c_int {
            if !(*fs).wl.is_null() {
                return -(1 as libc::c_int);
            }
            (*fs).wl = wl
        }
        wl = winlinks_RB_NEXT(wl)
    }
    if !(*fs).wl.is_null() {
        (*fs).idx = (*(*fs).wl).idx;
        (*fs).w = (*(*fs).wl).window;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
/* Find pane from string. Fills in s, wl, w, wp. */
unsafe extern "C" fn cmd_find_get_pane(
    mut fs: *mut cmd_find_state,
    mut pane: *const libc::c_char,
    mut only: libc::c_int,
) -> libc::c_int {
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"cmd_find_get_pane\x00"))
            .as_ptr(),
        pane,
    );
    /* Check for pane ids starting with %. */
    if *pane as libc::c_int == '%' as i32 {
        (*fs).wp = window_pane_find_by_id_str(pane);
        if (*fs).wp.is_null() {
            return -(1 as libc::c_int);
        }
        (*fs).w = (*(*fs).wp).window;
        return cmd_find_best_session_with_window(fs);
    }
    /* Not a pane id, so try the current session and window. */
    (*fs).s = (*(*fs).current).s;
    (*fs).wl = (*(*fs).current).wl;
    (*fs).idx = (*(*fs).current).idx;
    (*fs).w = (*(*fs).current).w;
    /* We now only need to find the pane in this window. */
    if cmd_find_get_pane_with_window(fs, pane) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Otherwise try as a window itself (this will also try as session). */
    if only == 0 && cmd_find_get_window(fs, pane, 0 as libc::c_int) == 0 as libc::c_int {
        (*fs).wp = (*(*fs).w).active;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
/*
 * Find pane from string, assuming it is in given session. Needs s, fills in wl
 * and w and wp.
 */
unsafe extern "C" fn cmd_find_get_pane_with_session(
    mut fs: *mut cmd_find_state,
    mut pane: *const libc::c_char,
) -> libc::c_int {
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
            b"cmd_find_get_pane_with_session\x00",
        ))
        .as_ptr(),
        pane,
    );
    /* Check for pane ids starting with %. */
    if *pane as libc::c_int == '%' as i32 {
        (*fs).wp = window_pane_find_by_id_str(pane);
        if (*fs).wp.is_null() {
            return -(1 as libc::c_int);
        }
        (*fs).w = (*(*fs).wp).window;
        return cmd_find_best_winlink_with_window(fs);
    }
    /* Otherwise use the current window. */
    (*fs).wl = (*(*fs).s).curw;
    (*fs).idx = (*(*fs).wl).idx;
    (*fs).w = (*(*fs).wl).window;
    /* Now we just need to look up the pane. */
    return cmd_find_get_pane_with_window(fs, pane);
}
/*
 * Find pane from string, assuming it is in the given window. Needs w, fills in
 * wp.
 */
unsafe extern "C" fn cmd_find_get_pane_with_window(
    mut fs: *mut cmd_find_state,
    mut pane: *const libc::c_char,
) -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut n: u_int = 0;
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
            b"cmd_find_get_pane_with_window\x00",
        ))
        .as_ptr(),
        pane,
    );
    /* Check for pane ids starting with %. */
    if *pane as libc::c_int == '%' as i32 {
        (*fs).wp = window_pane_find_by_id_str(pane);
        if (*fs).wp.is_null() {
            return -(1 as libc::c_int);
        }
        if (*(*fs).wp).window != (*fs).w {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    /* Try special characters. */
    if strcmp(pane, b"!\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*fs).wp = (*(*fs).w).last;
        if (*fs).wp.is_null() {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    } else {
        if strcmp(pane, b"{up-of}\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            (*fs).wp = window_pane_find_up((*(*fs).current).wp);
            if (*fs).wp.is_null() {
                return -(1 as libc::c_int);
            }
            return 0 as libc::c_int;
        } else {
            if strcmp(pane, b"{down-of}\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*fs).wp = window_pane_find_down((*(*fs).current).wp);
                if (*fs).wp.is_null() {
                    return -(1 as libc::c_int);
                }
                return 0 as libc::c_int;
            } else {
                if strcmp(pane, b"{left-of}\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*fs).wp = window_pane_find_left((*(*fs).current).wp);
                    if (*fs).wp.is_null() {
                        return -(1 as libc::c_int);
                    }
                    return 0 as libc::c_int;
                } else {
                    if strcmp(pane, b"{right-of}\x00" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        (*fs).wp = window_pane_find_right((*(*fs).current).wp);
                        if (*fs).wp.is_null() {
                            return -(1 as libc::c_int);
                        }
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
    /* Try as an offset. */
    if *pane.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
        || *pane.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        if *pane.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
            n = strtonum(
                pane.offset(1 as libc::c_int as isize),
                1 as libc::c_int as libc::c_longlong,
                2147483647 as libc::c_int as libc::c_longlong,
                0 as *mut *const libc::c_char,
            ) as u_int
        } else {
            n = 1 as libc::c_int as u_int
        }
        wp = (*(*fs).current).wp;
        if *pane.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
            (*fs).wp = window_pane_next_by_number((*fs).w, wp, n)
        } else {
            (*fs).wp = window_pane_previous_by_number((*fs).w, wp, n)
        }
        if !(*fs).wp.is_null() {
            return 0 as libc::c_int;
        }
    }
    /* Get pane by index. */
    idx = strtonum(
        pane,
        0 as libc::c_int as libc::c_longlong,
        2147483647 as libc::c_int as libc::c_longlong,
        &mut errstr,
    ) as libc::c_int;
    if errstr.is_null() {
        (*fs).wp = window_pane_at_index((*fs).w, idx as u_int);
        if !(*fs).wp.is_null() {
            return 0 as libc::c_int;
        }
    }
    /* Try as a description. */
    (*fs).wp = window_find_string((*fs).w, pane);
    if !(*fs).wp.is_null() {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
/* Clear state. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_clear_state(mut fs: *mut cmd_find_state, mut flags: libc::c_int) {
    memset(
        fs as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cmd_find_state>() as libc::c_ulong,
    );
    (*fs).flags = flags;
    (*fs).idx = -(1 as libc::c_int);
}
/* Check if state is empty. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_empty_state(mut fs: *mut cmd_find_state) -> libc::c_int {
    if (*fs).s.is_null() && (*fs).wl.is_null() && (*fs).w.is_null() && (*fs).wp.is_null() {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* Check if a state if valid. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_valid_state(mut fs: *mut cmd_find_state) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if (*fs).s.is_null() || (*fs).wl.is_null() || (*fs).w.is_null() || (*fs).wp.is_null() {
        return 0 as libc::c_int;
    }
    if session_alive((*fs).s) == 0 {
        return 0 as libc::c_int;
    }
    wl = winlinks_RB_MINMAX(&mut (*(*fs).s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if (*wl).window == (*fs).w && wl == (*fs).wl {
            break;
        }
        wl = winlinks_RB_NEXT(wl)
    }
    if wl.is_null() {
        return 0 as libc::c_int;
    }
    if (*fs).w != (*(*fs).wl).window {
        return 0 as libc::c_int;
    }
    return window_has_pane((*fs).w, (*fs).wp);
}
/* Copy a state. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_copy_state(
    mut dst: *mut cmd_find_state,
    mut src: *mut cmd_find_state,
) {
    (*dst).s = (*src).s;
    (*dst).wl = (*src).wl;
    (*dst).idx = (*src).idx;
    (*dst).w = (*src).w;
    (*dst).wp = (*src).wp;
}
/* Log the result. */
unsafe extern "C" fn cmd_find_log_state(
    mut prefix: *const libc::c_char,
    mut fs: *mut cmd_find_state,
) {
    if !(*fs).s.is_null() {
        log_debug(
            b"%s: s=$%u %s\x00" as *const u8 as *const libc::c_char,
            prefix,
            (*(*fs).s).id,
            (*(*fs).s).name,
        );
    } else {
        log_debug(
            b"%s: s=none\x00" as *const u8 as *const libc::c_char,
            prefix,
        );
    }
    if !(*fs).wl.is_null() {
        log_debug(
            b"%s: wl=%u %d w=@%u %s\x00" as *const u8 as *const libc::c_char,
            prefix,
            (*(*fs).wl).idx,
            ((*(*fs).wl).window == (*fs).w) as libc::c_int,
            (*(*fs).w).id,
            (*(*fs).w).name,
        );
    } else {
        log_debug(
            b"%s: wl=none\x00" as *const u8 as *const libc::c_char,
            prefix,
        );
    }
    if !(*fs).wp.is_null() {
        log_debug(
            b"%s: wp=%%%u\x00" as *const u8 as *const libc::c_char,
            prefix,
            (*(*fs).wp).id,
        );
    } else {
        log_debug(
            b"%s: wp=none\x00" as *const u8 as *const libc::c_char,
            prefix,
        );
    }
    if (*fs).idx != -(1 as libc::c_int) {
        log_debug(
            b"%s: idx=%d\x00" as *const u8 as *const libc::c_char,
            prefix,
            (*fs).idx,
        );
    } else {
        log_debug(
            b"%s: idx=none\x00" as *const u8 as *const libc::c_char,
            prefix,
        );
    };
}
/* Find state from a session. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_session(
    mut fs: *mut cmd_find_state,
    mut s: *mut session,
    mut flags: libc::c_int,
) {
    cmd_find_clear_state(fs, flags);
    (*fs).s = s;
    (*fs).wl = (*(*fs).s).curw;
    (*fs).w = (*(*fs).wl).window;
    (*fs).wp = (*(*fs).w).active;
    cmd_find_log_state(
        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"cmd_find_from_session\x00"))
            .as_ptr(),
        fs,
    );
}
/* Find state from a winlink. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_winlink(
    mut fs: *mut cmd_find_state,
    mut wl: *mut winlink,
    mut flags: libc::c_int,
) {
    cmd_find_clear_state(fs, flags);
    (*fs).s = (*wl).session;
    (*fs).wl = wl;
    (*fs).w = (*wl).window;
    (*fs).wp = (*(*wl).window).active;
    cmd_find_log_state(
        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"cmd_find_from_winlink\x00"))
            .as_ptr(),
        fs,
    );
}
/* Find state from a session and window. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_session_window(
    mut fs: *mut cmd_find_state,
    mut s: *mut session,
    mut w: *mut window,
    mut flags: libc::c_int,
) -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    (*fs).s = s;
    (*fs).w = w;
    if cmd_find_best_winlink_with_window(fs) != 0 as libc::c_int {
        cmd_find_clear_state(fs, flags);
        return -(1 as libc::c_int);
    }
    (*fs).wp = (*(*fs).w).active;
    cmd_find_log_state(
        (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
            b"cmd_find_from_session_window\x00",
        ))
        .as_ptr(),
        fs,
    );
    return 0 as libc::c_int;
}
/* Find state from a window. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_window(
    mut fs: *mut cmd_find_state,
    mut w: *mut window,
    mut flags: libc::c_int,
) -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    (*fs).w = w;
    if cmd_find_best_session_with_window(fs) != 0 as libc::c_int {
        cmd_find_clear_state(fs, flags);
        return -(1 as libc::c_int);
    }
    if cmd_find_best_winlink_with_window(fs) != 0 as libc::c_int {
        cmd_find_clear_state(fs, flags);
        return -(1 as libc::c_int);
    }
    (*fs).wp = (*(*fs).w).active;
    cmd_find_log_state(
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"cmd_find_from_window\x00"))
            .as_ptr(),
        fs,
    );
    return 0 as libc::c_int;
}
/* Find state from a winlink and pane. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_winlink_pane(
    mut fs: *mut cmd_find_state,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
    mut flags: libc::c_int,
) {
    cmd_find_clear_state(fs, flags);
    (*fs).s = (*wl).session;
    (*fs).wl = wl;
    (*fs).idx = (*(*fs).wl).idx;
    (*fs).w = (*(*fs).wl).window;
    (*fs).wp = wp;
    cmd_find_log_state(
        (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
            b"cmd_find_from_winlink_pane\x00",
        ))
        .as_ptr(),
        fs,
    );
}
/* Find state from a pane. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_pane(
    mut fs: *mut cmd_find_state,
    mut wp: *mut window_pane,
    mut flags: libc::c_int,
) -> libc::c_int {
    if cmd_find_from_window(fs, (*wp).window, flags) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*fs).wp = wp;
    cmd_find_log_state(
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"cmd_find_from_pane\x00"))
            .as_ptr(),
        fs,
    );
    return 0 as libc::c_int;
}
/* Find state from nothing. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_nothing(
    mut fs: *mut cmd_find_state,
    mut flags: libc::c_int,
) -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    (*fs).s = cmd_find_best_session(0 as *mut *mut session, 0 as libc::c_int as u_int, flags);
    if (*fs).s.is_null() {
        cmd_find_clear_state(fs, flags);
        return -(1 as libc::c_int);
    }
    (*fs).wl = (*(*fs).s).curw;
    (*fs).idx = (*(*fs).wl).idx;
    (*fs).w = (*(*fs).wl).window;
    (*fs).wp = (*(*fs).w).active;
    cmd_find_log_state(
        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"cmd_find_from_nothing\x00"))
            .as_ptr(),
        fs,
    );
    return 0 as libc::c_int;
}
/* Find state from mouse. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_mouse(
    mut fs: *mut cmd_find_state,
    mut m: *mut mouse_event,
    mut flags: libc::c_int,
) -> libc::c_int {
    cmd_find_clear_state(fs, flags);
    if (*m).valid == 0 {
        return -(1 as libc::c_int);
    }
    (*fs).wp = cmd_mouse_pane(m, &mut (*fs).s, &mut (*fs).wl);
    if (*fs).wp.is_null() {
        cmd_find_clear_state(fs, flags);
        return -(1 as libc::c_int);
    }
    (*fs).w = (*(*fs).wl).window;
    cmd_find_log_state(
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"cmd_find_from_mouse\x00"))
            .as_ptr(),
        fs,
    );
    return 0 as libc::c_int;
}
/* Find state from client. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_from_client(
    mut fs: *mut cmd_find_state,
    mut c: *mut client,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    /* If no client, treat as from nothing. */
    if c.is_null() {
        return cmd_find_from_nothing(fs, flags);
    }
    /* If this is an attached client, all done. */
    if !(*c).session.is_null() {
        cmd_find_clear_state(fs, flags);
        (*fs).wp = server_client_get_pane(c);
        if (*fs).wp.is_null() {
            cmd_find_from_session(fs, (*c).session, flags);
            return 0 as libc::c_int;
        }
        (*fs).s = (*c).session;
        (*fs).wl = (*(*fs).s).curw;
        (*fs).w = (*(*fs).wl).window;
        cmd_find_log_state(
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"cmd_find_from_client\x00"))
                .as_ptr(),
            fs,
        );
        return 0 as libc::c_int;
    }
    cmd_find_clear_state(fs, flags);
    /*
     * If this is an unattached client running in a pane, we can use that
     * to limit the list of sessions to those containing that pane.
     */
    wp = cmd_find_inside_pane(c);
    if !wp.is_null() {
        /*
         * Don't have a session, or it doesn't have this pane. Try all
         * sessions.
         */
        (*fs).w = (*wp).window; /* use active pane */
        if !(cmd_find_best_session_with_window(fs) != 0 as libc::c_int) {
            (*fs).wl = (*(*fs).s).curw;
            (*fs).w = (*(*fs).wl).window;
            (*fs).wp = (*(*fs).w).active;
            cmd_find_log_state(
                (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"cmd_find_from_client\x00",
                ))
                .as_ptr(),
                fs,
            );
            return 0 as libc::c_int;
        }
    }
    /*
     * The window may have been destroyed but the pane
     * still on all_window_panes due to something else
     * holding a reference.
     */
    /* We can't find the pane so need to guess. */
    return cmd_find_from_nothing(fs, flags);
}
/*
 * Split target into pieces and resolve for the given type. Fills in the given
 * state. Returns 0 on success or -1 on error.
 */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_target(
    mut fs: *mut cmd_find_state,
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut target: *const libc::c_char,
    mut type_0: cmd_find_type,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut m: *mut mouse_event = 0 as *mut mouse_event;
    let mut current: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *const cmd_find_state as *mut cmd_find_state,
        s: 0 as *const session as *mut session,
        wl: 0 as *const winlink as *mut winlink,
        w: 0 as *const window as *mut window,
        wp: 0 as *const window_pane as *mut window_pane,
        idx: 0,
    };
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut period: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 256] = [0; 256];
    let mut session: *const libc::c_char = 0 as *const libc::c_char;
    let mut window: *const libc::c_char = 0 as *const libc::c_char;
    let mut pane: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut window_only: libc::c_int = 0 as libc::c_int;
    let mut pane_only: libc::c_int = 0 as libc::c_int;
    /* Can fail flag implies quiet. */
    if flags & 0x40 as libc::c_int != 0 {
        flags |= 0x2 as libc::c_int
    }
    /* Log the arguments. */
    if type_0 as libc::c_uint == CMD_FIND_PANE as libc::c_int as libc::c_uint {
        s = b"pane\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint == CMD_FIND_WINDOW as libc::c_int as libc::c_uint {
        s = b"window\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint == CMD_FIND_SESSION as libc::c_int as libc::c_uint {
        s = b"session\x00" as *const u8 as *const libc::c_char
    } else {
        s = b"unknown\x00" as *const u8 as *const libc::c_char
    }
    *tmp.as_mut_ptr() = '\u{0}' as i32 as libc::c_char;
    if flags & 0x1 as libc::c_int != 0 {
        strlcat(
            tmp.as_mut_ptr(),
            b"PREFER_UNATTACHED,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if flags & 0x2 as libc::c_int != 0 {
        strlcat(
            tmp.as_mut_ptr(),
            b"QUIET,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if flags & 0x4 as libc::c_int != 0 {
        strlcat(
            tmp.as_mut_ptr(),
            b"WINDOW_INDEX,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if flags & 0x8 as libc::c_int != 0 {
        strlcat(
            tmp.as_mut_ptr(),
            b"DEFAULT_MARKED,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if flags & 0x10 as libc::c_int != 0 {
        strlcat(
            tmp.as_mut_ptr(),
            b"EXACT_SESSION,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if flags & 0x20 as libc::c_int != 0 {
        strlcat(
            tmp.as_mut_ptr(),
            b"EXACT_WINDOW,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if flags & 0x40 as libc::c_int != 0 {
        strlcat(
            tmp.as_mut_ptr(),
            b"CANFAIL,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if *tmp.as_mut_ptr() as libc::c_int != '\u{0}' as i32 {
        tmp[strlen(tmp.as_mut_ptr()).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] =
            '\u{0}' as i32 as libc::c_char
    } else {
        strlcat(
            tmp.as_mut_ptr(),
            b"NONE\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    log_debug(
        b"%s: target %s, type %s, item %p, flags %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr(),
        if target.is_null() {
            b"none\x00" as *const u8 as *const libc::c_char
        } else {
            target
        },
        s,
        item,
        tmp.as_mut_ptr(),
    );
    /* Clear new state. */
    cmd_find_clear_state(fs, flags);
    /* Find current state. */
    if server_check_marked() != 0 && flags & 0x8 as libc::c_int != 0 {
        (*fs).current = &mut marked_pane;
        log_debug(
            b"%s: current is marked pane\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"cmd_find_target\x00"))
                .as_ptr(),
        );
        current_block = 17784502470059252271;
    } else if cmd_find_valid_state(cmdq_get_current(item)) != 0 {
        (*fs).current = cmdq_get_current(item);
        log_debug(
            b"%s: current is from queue\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"cmd_find_target\x00"))
                .as_ptr(),
        );
        current_block = 17784502470059252271;
    } else if cmd_find_from_client(&mut current, cmdq_get_client(item), flags) == 0 as libc::c_int {
        (*fs).current = &mut current;
        log_debug(
            b"%s: current is from client\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"cmd_find_target\x00"))
                .as_ptr(),
        );
        current_block = 17784502470059252271;
    } else {
        if !flags & 0x2 as libc::c_int != 0 {
            cmdq_error(
                item,
                b"no current target\x00" as *const u8 as *const libc::c_char,
            );
        }
        current_block = 10029033417334023838;
    }
    match current_block {
        17784502470059252271 => {
            if cmd_find_valid_state((*fs).current) == 0 {
                fatalx(b"invalid current find state\x00" as *const u8 as *const libc::c_char);
            }
            /* An empty or NULL target is the current. */
            if target.is_null() || *target as libc::c_int == '\u{0}' as i32 {
                current_block = 13645261163415976511;
            } else if strcmp(target, b"=\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(target, b"{mouse}\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                m = &mut (*(cmdq_get_event
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd_queue::cmdq_item,
                    ) -> *mut key_event)(item))
                .m;
                let mut current_block_51: u64;
                match type_0 as libc::c_uint {
                    0 => {
                        (*fs).wp = cmd_mouse_pane(m, &mut (*fs).s, &mut (*fs).wl);
                        if !(*fs).wp.is_null() {
                            (*fs).w = (*(*fs).wl).window;
                            current_block_51 = 9241535491006583629;
                        } else {
                            current_block_51 = 12930510239943809441;
                        }
                    }
                    1 | 2 => {
                        current_block_51 = 12930510239943809441;
                    }
                    _ => {
                        current_block_51 = 9241535491006583629;
                    }
                }
                match current_block_51 {
                    12930510239943809441 =>
                    /* Mouse target is a plain = or {mouse}. */
                    /* FALLTHROUGH */
                    {
                        (*fs).wl = cmd_mouse_window(m, &mut (*fs).s);
                        if (*fs).wl.is_null() && !(*fs).s.is_null() {
                            (*fs).wl = (*(*fs).s).curw
                        }
                        if !(*fs).wl.is_null() {
                            (*fs).w = (*(*fs).wl).window;
                            (*fs).wp = (*(*fs).w).active
                        }
                    }
                    _ => {}
                }
                if (*fs).wp.is_null() {
                    if !flags & 0x2 as libc::c_int != 0 {
                        cmdq_error(
                            item,
                            b"no mouse target\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    current_block = 10029033417334023838;
                } else {
                    current_block = 6635813614084111333;
                }
            } else if strcmp(target, b"~\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(target, b"{marked}\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                if server_check_marked() == 0 {
                    if !flags & 0x2 as libc::c_int != 0 {
                        cmdq_error(
                            item,
                            b"no marked target\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    current_block = 10029033417334023838;
                } else {
                    cmd_find_copy_state(fs, &mut marked_pane);
                    current_block = 6635813614084111333;
                }
            } else {
                /* Marked target is a plain ~ or {marked}. */
                /* Find separators if they exist. */
                copy = xstrdup(target);
                colon = strchr(copy, ':' as i32);
                if !colon.is_null() {
                    let fresh2 = colon;
                    colon = colon.offset(1);
                    *fresh2 = '\u{0}' as i32 as libc::c_char
                }
                if colon.is_null() {
                    period = strchr(copy, '.' as i32)
                } else {
                    period = strchr(colon, '.' as i32)
                }
                if !period.is_null() {
                    let fresh3 = period;
                    period = period.offset(1);
                    *fresh3 = '\u{0}' as i32 as libc::c_char
                }
                /* Set session, window and pane parts. */
                pane = 0 as *const libc::c_char;
                window = pane;
                session = window;
                if !colon.is_null() && !period.is_null() {
                    session = copy;
                    window = colon;
                    window_only = 1 as libc::c_int;
                    pane = period;
                    pane_only = 1 as libc::c_int
                } else if !colon.is_null() && period.is_null() {
                    session = copy;
                    window = colon;
                    window_only = 1 as libc::c_int
                } else if colon.is_null() && !period.is_null() {
                    window = copy;
                    pane = period;
                    pane_only = 1 as libc::c_int
                } else if *copy as libc::c_int == '$' as i32 {
                    session = copy
                } else if *copy as libc::c_int == '@' as i32 {
                    window = copy
                } else if *copy as libc::c_int == '%' as i32 {
                    pane = copy
                } else {
                    match type_0 as libc::c_uint {
                        2 => session = copy,
                        1 => window = copy,
                        0 => pane = copy,
                        _ => {}
                    }
                }
                /* Set exact match flags. */
                if !session.is_null() && *session as libc::c_int == '=' as i32 {
                    session = session.offset(1);
                    (*fs).flags |= 0x10 as libc::c_int
                }
                if !window.is_null() && *window as libc::c_int == '=' as i32 {
                    window = window.offset(1);
                    (*fs).flags |= 0x20 as libc::c_int
                }
                /* Empty is the same as NULL. */
                if !session.is_null() && *session as libc::c_int == '\u{0}' as i32 {
                    session = 0 as *const libc::c_char
                }
                if !window.is_null() && *window as libc::c_int == '\u{0}' as i32 {
                    window = 0 as *const libc::c_char
                }
                if !pane.is_null() && *pane as libc::c_int == '\u{0}' as i32 {
                    pane = 0 as *const libc::c_char
                }
                /* Map though conversion table. */
                if !session.is_null() {
                    session = cmd_find_map_table(cmd_find_session_table.as_mut_ptr(), session)
                }
                if !window.is_null() {
                    window = cmd_find_map_table(cmd_find_window_table.as_mut_ptr(), window)
                }
                if !pane.is_null() {
                    pane = cmd_find_map_table(cmd_find_pane_table.as_mut_ptr(), pane)
                }
                if !session.is_null() || !window.is_null() || !pane.is_null() {
                    log_debug(
                        b"%s: target %s is %s%s%s%s%s%s\x00" as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                            b"cmd_find_target\x00",
                        ))
                        .as_ptr(),
                        target,
                        if session.is_null() {
                            b"\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"session \x00" as *const u8 as *const libc::c_char
                        },
                        if session.is_null() {
                            b"\x00" as *const u8 as *const libc::c_char
                        } else {
                            session
                        },
                        if window.is_null() {
                            b"\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"window \x00" as *const u8 as *const libc::c_char
                        },
                        if window.is_null() {
                            b"\x00" as *const u8 as *const libc::c_char
                        } else {
                            window
                        },
                        if pane.is_null() {
                            b"\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"pane \x00" as *const u8 as *const libc::c_char
                        },
                        if pane.is_null() {
                            b"\x00" as *const u8 as *const libc::c_char
                        } else {
                            pane
                        },
                    );
                }
                /* No pane is allowed if want an index. */
                if !pane.is_null() && flags & 0x4 as libc::c_int != 0 {
                    if !flags & 0x2 as libc::c_int != 0 {
                        cmdq_error(
                            item,
                            b"can\'t specify pane here\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    current_block = 10029033417334023838;
                } else {
                    /* If the session isn't NULL, look it up. */
                    if !session.is_null() {
                        /* This will fill in session. */
                        if cmd_find_get_session(fs, session) != 0 as libc::c_int {
                            if !flags & 0x2 as libc::c_int != 0 {
                                cmdq_error(
                                    item,
                                    b"can\'t find session: %s\x00" as *const u8
                                        as *const libc::c_char,
                                    session,
                                );
                            }
                            current_block = 10029033417334023838;
                        } else if window.is_null() && pane.is_null() {
                            (*fs).wl = (*(*fs).s).curw;
                            (*fs).idx = -(1 as libc::c_int);
                            (*fs).w = (*(*fs).wl).window;
                            (*fs).wp = (*(*fs).w).active;
                            current_block = 6635813614084111333;
                        } else if !window.is_null() && pane.is_null() {
                            /* If window and pane are NULL, use that session's current. */
                            /* If window is present but pane not, find window in session. */
                            /* This will fill in winlink and window. */
                            if cmd_find_get_window_with_session(fs, window) != 0 as libc::c_int {
                                current_block = 6577947231097195512;
                            } else {
                                if !(*fs).wl.is_null() {
                                    /* can be NULL if index only */
                                    (*fs).wp = (*(*(*fs).wl).window).active
                                }
                                current_block = 6635813614084111333;
                            }
                        } else if window.is_null() && !pane.is_null() {
                            /* If pane is present but window not, find pane. */
                            /* This will fill in winlink and window and pane. */
                            if cmd_find_get_pane_with_session(fs, pane) != 0 as libc::c_int {
                                current_block = 12740893567707959592;
                            } else {
                                current_block = 6635813614084111333;
                            }
                        } else if cmd_find_get_window_with_session(fs, window) != 0 as libc::c_int {
                            current_block = 6577947231097195512;
                        } else if cmd_find_get_pane_with_window(fs, pane) != 0 as libc::c_int {
                            current_block = 12740893567707959592;
                        } else {
                            current_block = 6635813614084111333;
                        }
                    } else if !window.is_null() && !pane.is_null() {
                        /*
                         * If window and pane are present, find both in session. This
                         * will fill in winlink and window.
                         */
                        /* This will fill in pane. */
                        /* No session. If window and pane, try them. */
                        /* This will fill in session, winlink and window. */
                        if cmd_find_get_window(fs, window, window_only) != 0 as libc::c_int {
                            current_block = 6577947231097195512;
                        } else if cmd_find_get_pane_with_window(fs, pane) != 0 as libc::c_int {
                            current_block = 12740893567707959592;
                        } else {
                            current_block = 6635813614084111333;
                        }
                    } else if !window.is_null() && pane.is_null() {
                        /* This will fill in pane. */
                        /* If just window is present, try it. */
                        /* This will fill in session, winlink and window. */
                        if cmd_find_get_window(fs, window, window_only) != 0 as libc::c_int {
                            current_block = 6577947231097195512;
                        } else {
                            if !(*fs).wl.is_null() {
                                /* can be NULL if index only */
                                (*fs).wp = (*(*(*fs).wl).window).active
                            }
                            current_block = 6635813614084111333;
                        }
                    } else if window.is_null() && !pane.is_null() {
                        /* If just pane is present, try it. */
                        /* This will fill in session, winlink, window and pane. */
                        if cmd_find_get_pane(fs, pane, pane_only) != 0 as libc::c_int {
                            current_block = 12740893567707959592;
                        } else {
                            current_block = 6635813614084111333;
                        }
                    } else {
                        current_block = 13645261163415976511;
                    }
                    match current_block {
                        10029033417334023838 => {}
                        6635813614084111333 => {}
                        13645261163415976511 => {}
                        _ => {
                            match current_block {
                                6577947231097195512 => {
                                    if !flags & 0x2 as libc::c_int != 0 {
                                        cmdq_error(
                                            item,
                                            b"can\'t find window: %s\x00" as *const u8
                                                as *const libc::c_char,
                                            window,
                                        );
                                    }
                                }
                                _ => {
                                    if !flags & 0x2 as libc::c_int != 0 {
                                        cmdq_error(
                                            item,
                                            b"can\'t find pane: %s\x00" as *const u8
                                                as *const libc::c_char,
                                            pane,
                                        );
                                    }
                                }
                            }
                            current_block = 10029033417334023838;
                        }
                    }
                }
            }
            match current_block {
                10029033417334023838 => {}
                _ => {
                    match current_block {
                        13645261163415976511 => {
                            /* Use the current session. */
                            cmd_find_copy_state(fs, (*fs).current);
                            if flags & 0x4 as libc::c_int != 0 {
                                (*fs).idx = -(1 as libc::c_int)
                            }
                        }
                        _ => {}
                    }
                    (*fs).current = 0 as *mut cmd_find_state;
                    cmd_find_log_state(
                        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                            b"cmd_find_target\x00",
                        ))
                        .as_ptr(),
                        fs,
                    );
                    free(copy as *mut libc::c_void);
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    (*fs).current = 0 as *mut cmd_find_state;
    log_debug(
        b"%s: error\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"cmd_find_target\x00")).as_ptr(),
    );
    free(copy as *mut libc::c_void);
    if flags & 0x40 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
/* Find the current client. */
unsafe extern "C" fn cmd_find_current_client(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut quiet: libc::c_int,
) -> *mut client {
    let mut c: *mut client = 0 as *mut client;
    let mut found: *mut client = 0 as *mut client;
    let mut s: *mut session = 0 as *mut session;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *const cmd_find_state as *mut cmd_find_state,
        s: 0 as *const session as *mut session,
        wl: 0 as *const winlink as *mut winlink,
        w: 0 as *const window as *mut window,
        wp: 0 as *const window_pane as *mut window_pane,
        idx: 0,
    };
    if !item.is_null() {
        c = cmdq_get_client(item)
    }
    if !c.is_null() && !(*c).session.is_null() {
        return c;
    }
    found = 0 as *mut client;
    if !c.is_null() && {
        wp = cmd_find_inside_pane(c);
        !wp.is_null()
    } {
        cmd_find_clear_state(&mut fs, 0x2 as libc::c_int);
        fs.w = (*wp).window;
        if cmd_find_best_session_with_window(&mut fs) == 0 as libc::c_int {
            found = cmd_find_best_client(fs.s)
        }
    } else {
        s = cmd_find_best_session(
            0 as *mut *mut session,
            0 as libc::c_int as u_int,
            0x2 as libc::c_int,
        );
        if !s.is_null() {
            found = cmd_find_best_client(s)
        }
    }
    if found.is_null() && !item.is_null() && quiet == 0 {
        cmdq_error(
            item,
            b"no current client\x00" as *const u8 as *const libc::c_char,
        );
    }
    log_debug(
        b"%s: no target, return %p\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"cmd_find_current_client\x00"))
            .as_ptr(),
        found,
    );
    return found;
}
/* Find the target client or report an error and return NULL. */
#[no_mangle]
pub unsafe extern "C" fn cmd_find_client(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut target: *const libc::c_char,
    mut quiet: libc::c_int,
) -> *mut client {
    let mut c: *mut client = 0 as *mut client;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    /* A NULL argument means the current client. */
    if target.is_null() {
        return cmd_find_current_client(item, quiet);
    }
    copy = xstrdup(target);
    /* Trim a single trailing colon if any. */
    size = strlen(copy);
    if size != 0 as libc::c_int as libc::c_ulong
        && *copy.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            == ':' as i32
    {
        *copy.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
            '\u{0}' as i32 as libc::c_char
    }
    /* Check name and path of each client. */
    c = clients.tqh_first;
    while !c.is_null() {
        if !(*c).session.is_null() {
            if strcmp(copy, (*c).name) == 0 as libc::c_int {
                break;
            }
            if !(*(*c).ttyname as libc::c_int == '\u{0}' as i32) {
                if strcmp(copy, (*c).ttyname) == 0 as libc::c_int {
                    break;
                }
                if !(strncmp(
                    (*c).ttyname,
                    b"/dev/\x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) != 0 as libc::c_int)
                {
                    if strcmp(
                        copy,
                        (*c).ttyname
                            .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                                as isize)
                            .offset(-(1 as libc::c_int as isize)),
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                }
            }
        }
        c = (*c).entry.tqe_next
    }
    /* If no client found, report an error. */
    if c.is_null() && quiet == 0 {
        cmdq_error(
            item,
            b"can\'t find client: %s\x00" as *const u8 as *const libc::c_char,
            copy,
        );
    }
    free(copy as *mut libc::c_void);
    log_debug(
        b"%s: target %s, return %p\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"cmd_find_client\x00")).as_ptr(),
        target,
        c,
    );
    return c;
}
