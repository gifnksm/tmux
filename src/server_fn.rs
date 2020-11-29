use crate::{
    grid::Cell as GridCell,
    msg::code as msgtype_code,
    tty_code::{code as tty_code_code, Code as TtyCode},
    utf8::Utf8Data,
};
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn ctime_r(__timer: *const time_t, __buf: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_free(bufev: *mut bufferevent);
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn sig2name(_: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn proc_send(
        _: *mut crate::proc::tmuxpeer,
        _: crate::msg::Msgtype,
        _: libc::c_int,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn notify_client(_: *const libc::c_char, _: *mut client);
    #[no_mangle]
    fn notify_session_window(_: *const libc::c_char, _: *mut session, _: *mut window);
    #[no_mangle]
    fn notify_pane(_: *const libc::c_char, _: *mut window_pane);
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
    fn tty_update_client_offset(_: *mut client);
    #[no_mangle]
    fn tty_raw(_: *mut tty, _: *const libc::c_char);
    #[no_mangle]
    fn tty_stop_tty(_: *mut tty);
    #[no_mangle]
    fn tty_term_string(_: *mut tty_term, _: TtyCode) -> *const libc::c_char;
    #[no_mangle]
    fn alerts_check_session(_: *mut session);
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char);
    #[no_mangle]
    fn server_client_remove_pane(_: *mut window_pane);
    #[no_mangle]
    fn recalculate_sizes();
    #[no_mangle]
    fn status_timer_start(_: *mut client);
    #[no_mangle]
    fn winlink_find_by_window(_: *mut winlinks, _: *mut window) -> *mut winlink;
    #[no_mangle]
    fn winlink_remove(_: *mut winlinks, _: *mut winlink);
    #[no_mangle]
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_nputs(
        _: *mut screen_write_ctx,
        _: ssize_t,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn screen_write_linefeed(_: *mut screen_write_ctx, _: libc::c_int, _: u_int);
    #[no_mangle]
    fn screen_write_cursormove(
        _: *mut screen_write_ctx,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_write_scrollregion(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_start_pane(_: *mut screen_write_ctx, _: *mut window_pane, _: *mut screen);
    #[no_mangle]
    fn window_remove_pane(_: *mut window, _: *mut window_pane);
    #[no_mangle]
    fn layout_close_pane(_: *mut window_pane);
    #[no_mangle]
    fn winlink_stack_remove(_: *mut winlink_stack, _: *mut winlink);
    #[no_mangle]
    fn window_unzoom(_: *mut window) -> libc::c_int;
    #[no_mangle]
    fn window_count_panes(_: *mut window) -> u_int;
    #[no_mangle]
    fn session_group_contains(_: *mut session) -> *mut session_group;
    #[no_mangle]
    fn session_has(_: *mut session, _: *mut window) -> libc::c_int;
    #[no_mangle]
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    fn session_destroy(_: *mut session, _: libc::c_int, _: *const libc::c_char);
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn session_update_activity(_: *mut session, _: *mut timeval);
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    fn session_detach(_: *mut session, _: *mut winlink) -> libc::c_int;
    #[no_mangle]
    fn session_attach(
        _: *mut session,
        _: *mut window,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> *mut winlink;
    #[no_mangle]
    fn session_select(_: *mut session, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn session_renumber_windows(_: *mut session);
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct imsg_hdr {
    pub type_0: uint32_t,
    pub len: uint16_t,
    pub flags: uint16_t,
    pub peerid: uint32_t,
    pub pid: uint32_t,
}

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
    pub celldata: *mut crate::grid::CellEntry,
    pub extdsize: u_int,
    pub extddata: *mut crate::grid::ExtdEntry,
    pub flags: libc::c_int,
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
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: C2RustUnnamed_33,
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sessions {
    pub rbh_root: *mut session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[no_mangle]
pub unsafe extern "C" fn server_redraw_client(mut c: *mut client) {
    (*c).flags |= (0x8i32 | 0x10i32 | 0x1000000i32 | 0x400i32 | 0x2000000i32 | 0x20000000i32)
        as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn server_status_client(mut c: *mut client) {
    (*c).flags |= 0x10u64;
}
#[no_mangle]
pub unsafe extern "C" fn server_redraw_session(mut s: *mut session) {
    let mut c: *mut client = 0 as *mut client;
    c = clients.tqh_first;
    while !c.is_null() {
        if (*c).session == s {
            server_redraw_client(c);
        }
        c = (*c).entry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_redraw_session_group(mut s: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_contains(s);
    if sg.is_null() {
        server_redraw_session(s);
    } else {
        s = (*sg).sessions.tqh_first;
        while !s.is_null() {
            server_redraw_session(s);
            s = (*s).gentry.tqe_next
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_status_session(mut s: *mut session) {
    let mut c: *mut client = 0 as *mut client;
    c = clients.tqh_first;
    while !c.is_null() {
        if (*c).session == s {
            server_status_client(c);
        }
        c = (*c).entry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_status_session_group(mut s: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_contains(s);
    if sg.is_null() {
        server_status_session(s);
    } else {
        s = (*sg).sessions.tqh_first;
        while !s.is_null() {
            server_status_session(s);
            s = (*s).gentry.tqe_next
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_redraw_window(mut w: *mut window) {
    let mut c: *mut client = 0 as *mut client;
    c = clients.tqh_first;
    while !c.is_null() {
        if !(*c).session.is_null() && (*(*(*c).session).curw).window == w {
            server_redraw_client(c);
        }
        c = (*c).entry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_redraw_window_borders(mut w: *mut window) {
    let mut c: *mut client = 0 as *mut client;
    c = clients.tqh_first;
    while !c.is_null() {
        if !(*c).session.is_null() && (*(*(*c).session).curw).window == w {
            (*c).flags |= 0x400u64
        }
        c = (*c).entry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_status_window(mut w: *mut window) {
    let mut s: *mut session = 0 as *mut session;
    /*
     * This is slightly different. We want to redraw the status line of any
     * clients containing this window rather than anywhere it is the
     * current window.
     */
    s = sessions_RB_MINMAX(&mut sessions, -(1i32));
    while !s.is_null() {
        if session_has(s, w) != 0 {
            server_status_session(s);
        }
        s = sessions_RB_NEXT(s)
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_lock() {
    let mut c: *mut client = 0 as *mut client;
    c = clients.tqh_first;
    while !c.is_null() {
        if !(*c).session.is_null() {
            server_lock_client(c);
        }
        c = (*c).entry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_lock_session(mut s: *mut session) {
    let mut c: *mut client = 0 as *mut client;
    c = clients.tqh_first;
    while !c.is_null() {
        if (*c).session == s {
            server_lock_client(c);
        }
        c = (*c).entry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_lock_client(mut c: *mut client) {
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    if (*c).flags & 0x2000u64 != 0 {
        return;
    }
    if (*c).flags & 0x40u64 != 0 {
        return;
    }
    cmd = options_get_string(
        (*(*c).session).options,
        b"lock-command\x00" as *const u8 as *const libc::c_char,
    );
    if *cmd as libc::c_int == '\u{0}' as i32
        || strlen(cmd).wrapping_add(1u64)
            > (16384u64).wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong)
    {
        return;
    }
    tty_stop_tty(&mut (*c).tty);
    tty_raw(
        &mut (*c).tty,
        tty_term_string((*c).tty.term, tty_code_code::SMCUP),
    );
    tty_raw(
        &mut (*c).tty,
        tty_term_string((*c).tty.term, tty_code_code::CLEAR),
    );
    tty_raw(
        &mut (*c).tty,
        tty_term_string((*c).tty.term, tty_code_code::E3),
    );
    (*c).flags |= 0x40u64;
    proc_send(
        (*c).peer,
        msgtype_code::LOCK,
        -(1i32),
        cmd as *const libc::c_void,
        strlen(cmd).wrapping_add(1u64),
    );
}
#[no_mangle]
pub unsafe extern "C" fn server_kill_pane(mut wp: *mut window_pane) {
    let mut w: *mut window = (*wp).window;
    if window_count_panes(w) == 1u32 {
        server_kill_window(w, 1i32);
        recalculate_sizes();
    } else {
        server_unzoom_window(w);
        server_client_remove_pane(wp);
        layout_close_pane(wp);
        window_remove_pane(w, wp);
        server_redraw_window(w);
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_kill_window(mut w: *mut window, mut renumber: libc::c_int) {
    let mut s: *mut session = 0 as *mut session;
    let mut s1: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    s = sessions_RB_MINMAX(&mut sessions, -(1i32));
    while !s.is_null() && {
        s1 = sessions_RB_NEXT(s);
        (1i32) != 0
    } {
        if !(session_has(s, w) == 0) {
            server_unzoom_window(w);
            loop {
                wl = winlink_find_by_window(&mut (*s).windows, w);
                if wl.is_null() {
                    break;
                }
                if session_detach(s, wl) != 0 {
                    server_destroy_session_group(s);
                    break;
                } else {
                    server_redraw_session_group(s);
                }
            }
            if renumber != 0 {
                server_renumber_session(s);
            }
        }
        s = s1
    }
    recalculate_sizes();
}
#[no_mangle]
pub unsafe extern "C" fn server_renumber_session(mut s: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    if options_get_number(
        (*s).options,
        b"renumber-windows\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        sg = session_group_contains(s);
        if !sg.is_null() {
            s = (*sg).sessions.tqh_first;
            while !s.is_null() {
                session_renumber_windows(s);
                s = (*s).gentry.tqe_next
            }
        } else {
            session_renumber_windows(s);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_renumber_all() {
    let mut s: *mut session = 0 as *mut session;
    s = sessions_RB_MINMAX(&mut sessions, -(1i32));
    while !s.is_null() {
        server_renumber_session(s);
        s = sessions_RB_NEXT(s)
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_link_window(
    mut src: *mut session,
    mut srcwl: *mut winlink,
    mut dst: *mut session,
    mut dstidx: libc::c_int,
    mut killflag: libc::c_int,
    mut selectflag: libc::c_int,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut dstwl: *mut winlink = 0 as *mut winlink;
    let mut srcsg: *mut session_group = 0 as *mut session_group;
    let mut dstsg: *mut session_group = 0 as *mut session_group;
    srcsg = session_group_contains(src);
    dstsg = session_group_contains(dst);
    if src != dst && !srcsg.is_null() && !dstsg.is_null() && srcsg == dstsg {
        xasprintf(
            cause,
            b"sessions are grouped\x00" as *const u8 as *const libc::c_char,
        );
        return -(1i32);
    }
    dstwl = 0 as *mut winlink;
    if dstidx != -(1i32) {
        dstwl = winlink_find_by_index(&mut (*dst).windows, dstidx)
    }
    if !dstwl.is_null() {
        if (*dstwl).window == (*srcwl).window {
            xasprintf(
                cause,
                b"same index: %d\x00" as *const u8 as *const libc::c_char,
                dstidx,
            );
            return -(1i32);
        }
        if killflag != 0 {
            /*
             * Can't use session_detach as it will destroy session
             * if this makes it empty.
             */
            notify_session_window(
                b"window-unlinked\x00" as *const u8 as *const libc::c_char,
                dst,
                (*dstwl).window,
            );
            (*dstwl).flags &= !(0x1i32 | 0x2i32 | 0x4i32);
            winlink_stack_remove(&mut (*dst).lastw, dstwl);
            winlink_remove(&mut (*dst).windows, dstwl);
            /* Force select/redraw if current. */
            if dstwl == (*dst).curw {
                selectflag = 1i32;
                (*dst).curw = 0 as *mut winlink
            }
        }
    }
    if dstidx == -(1i32) {
        dstidx = (-1i64
            - options_get_number(
                (*dst).options,
                b"base-index\x00" as *const u8 as *const libc::c_char,
            )) as libc::c_int
    }
    dstwl = session_attach(dst, (*srcwl).window, dstidx, cause);
    if dstwl.is_null() {
        return -(1i32);
    }
    if selectflag != 0 {
        session_select(dst, (*dstwl).idx);
    }
    server_redraw_session_group(dst);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn server_unlink_window(mut s: *mut session, mut wl: *mut winlink) {
    if session_detach(s, wl) != 0 {
        server_destroy_session_group(s);
    } else {
        server_redraw_session_group(s);
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_destroy_pane(mut wp: *mut window_pane, mut notify: libc::c_int) {
    let mut w: *mut window = (*wp).window;
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
    let mut t: time_t = 0;
    let mut tim: [libc::c_char; 26] = [0; 26];
    if (*wp).fd != -(1i32) {
        bufferevent_free((*wp).event);
        (*wp).event = 0 as *mut bufferevent;
        close((*wp).fd);
        (*wp).fd = -(1i32)
    }
    if options_get_number(
        (*wp).options,
        b"remain-on-exit\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        if !(*wp).flags & 0x200i32 != 0 {
            return;
        }
        if (*wp).flags & 0x400i32 != 0 {
            return;
        }
        (*wp).flags |= 0x400i32;
        if notify != 0 {
            notify_pane(b"pane-died\x00" as *const u8 as *const libc::c_char, wp);
        }
        screen_write_start_pane(&mut ctx, wp, &mut (*wp).base);
        screen_write_scrollregion(&mut ctx, 0u32, (*(*ctx.s).grid).sy.wrapping_sub(1u32));
        screen_write_cursormove(
            &mut ctx,
            0i32,
            (*(*ctx.s).grid).sy.wrapping_sub(1u32) as libc::c_int,
            0i32,
        );
        screen_write_linefeed(&mut ctx, 1i32, 8u32);
        memcpy(
            &mut gc as *mut GridCell as *mut libc::c_void,
            &grid_default_cell as *const GridCell as *const libc::c_void,
            ::std::mem::size_of::<GridCell>() as libc::c_ulong,
        );
        time(&mut t);
        ctime_r(&mut t, tim.as_mut_ptr());
        tim[strcspn(
            tim.as_mut_ptr(),
            b"\n\x00" as *const u8 as *const libc::c_char,
        ) as usize] = '\u{0}' as libc::c_char;
        if (*wp).status & 0x7fi32 == 0i32 {
            screen_write_nputs(
                &mut ctx as *mut screen_write_ctx,
                -1i64,
                &mut gc as *mut GridCell,
                b"Pane is dead (status %d, %s)\x00" as *const u8 as *const libc::c_char,
                ((*wp).status & 0xff00i32) >> 8i32,
                tim.as_mut_ptr(),
            );
        } else if (((*wp).status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32 {
            screen_write_nputs(
                &mut ctx as *mut screen_write_ctx,
                -1i64,
                &mut gc as *mut GridCell,
                b"Pane is dead (signal %s, %s)\x00" as *const u8 as *const libc::c_char,
                sig2name((*wp).status & 0x7fi32),
                tim.as_mut_ptr(),
            );
        }
        screen_write_stop(&mut ctx);
        (*wp).flags |= 0x1i32;
        return;
    }
    if notify != 0 {
        notify_pane(b"pane-exited\x00" as *const u8 as *const libc::c_char, wp);
    }
    server_unzoom_window(w);
    server_client_remove_pane(wp);
    layout_close_pane(wp);
    window_remove_pane(w, wp);
    if (*w).panes.tqh_first.is_null() {
        server_kill_window(w, 1i32);
    } else {
        server_redraw_window(w);
    };
}
unsafe extern "C" fn server_destroy_session_group(mut s: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut s1: *mut session = 0 as *mut session;
    sg = session_group_contains(s);
    if sg.is_null() {
        server_destroy_session(s);
    } else {
        s = (*sg).sessions.tqh_first;
        while !s.is_null() && {
            s1 = (*s).gentry.tqe_next;
            (1i32) != 0
        } {
            server_destroy_session(s);
            session_destroy(
                s,
                1i32,
                (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                    b"server_destroy_session_group\x00",
                ))
                .as_ptr(),
            );
            s = s1
        }
    };
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
unsafe extern "C" fn server_next_session(mut s: *mut session) -> *mut session {
    let mut s_loop: *mut session = 0 as *mut session;
    let mut s_out: *mut session = 0 as *mut session;
    s_out = 0 as *mut session;
    s_loop = sessions_RB_MINMAX(&mut sessions, -(1i32));
    while !s_loop.is_null() {
        if !(s_loop == s) {
            if s_out.is_null()
                || (if (*s_loop).activity_time.tv_sec == (*s_out).activity_time.tv_sec {
                    ((*s_loop).activity_time.tv_usec < (*s_out).activity_time.tv_usec)
                        as libc::c_int
                } else {
                    ((*s_loop).activity_time.tv_sec < (*s_out).activity_time.tv_sec) as libc::c_int
                }) != 0
            {
                s_out = s_loop
            }
        }
        s_loop = sessions_RB_NEXT(s_loop)
    }
    return s_out;
}
#[no_mangle]
pub unsafe extern "C" fn server_destroy_session(mut s: *mut session) {
    let mut c: *mut client = 0 as *mut client;
    let mut s_new: *mut session = 0 as *mut session;
    if options_get_number(
        (*s).options,
        b"detach-on-destroy\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        s_new = server_next_session(s)
    } else {
        s_new = 0 as *mut session
    }
    c = clients.tqh_first;
    while !c.is_null() {
        if !((*c).session != s) {
            if s_new.is_null() {
                (*c).session = 0 as *mut session;
                (*c).flags |= 0x4u64
            } else {
                (*c).last_session = 0 as *mut session;
                (*c).session = s_new;
                server_client_set_key_table(c, 0 as *const libc::c_char);
                tty_update_client_offset(c);
                status_timer_start(c);
                notify_client(
                    b"client-session-changed\x00" as *const u8 as *const libc::c_char,
                    c,
                );
                session_update_activity(s_new, 0 as *mut timeval);
                gettimeofday(&mut (*s_new).last_attached_time, 0 as *mut libc::c_void);
                server_redraw_client(c);
                alerts_check_session(s_new);
            }
        }
        c = (*c).entry.tqe_next
    }
    recalculate_sizes();
}
#[no_mangle]
pub unsafe extern "C" fn server_check_unattached() {
    let mut s: *mut session = 0 as *mut session;
    /*
     * If any sessions are no longer attached and have destroy-unattached
     * set, collect them.
     */
    s = sessions_RB_MINMAX(&mut sessions, -(1i32));
    while !s.is_null() {
        if !((*s).attached != 0u32) {
            if options_get_number(
                (*s).options,
                b"destroy-unattached\x00" as *const u8 as *const libc::c_char,
            ) != 0
            {
                session_destroy(
                    s,
                    1i32,
                    (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                        b"server_check_unattached\x00",
                    ))
                    .as_ptr(),
                );
            }
        }
        s = sessions_RB_NEXT(s)
    }
}
#[no_mangle]
pub unsafe extern "C" fn server_unzoom_window(mut w: *mut window) {
    if window_unzoom(w) == 0i32 {
        server_redraw_window(w);
    };
}