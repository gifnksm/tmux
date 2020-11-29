use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_w_options: *mut crate::options::options;
    #[no_mangle]
    fn notify_window(_: *const libc::c_char, _: *mut window);
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
    fn tty_update_window_offset(_: *mut window);
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn status_line_size(_: *mut client) -> u_int;
    #[no_mangle]
    fn status_update_cache(_: *mut session);
    #[no_mangle]
    fn server_redraw_window(_: *mut window);
    #[no_mangle]
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    #[no_mangle]
    fn window_zoom(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn window_resize(_: *mut window, _: u_int, _: u_int, _: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn layout_resize(_: *mut window, _: u_int, _: u_int);
    #[no_mangle]
    fn window_unzoom(_: *mut window) -> libc::c_int;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    fn session_has(_: *mut session, _: *mut window) -> libc::c_int;
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
pub struct windows {
    pub rbh_root: *mut window,
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
#[no_mangle]
pub unsafe extern "C" fn resize_window(
    mut w: *mut window,
    mut sx: u_int,
    mut sy: u_int,
    mut xpixel: libc::c_int,
    mut ypixel: libc::c_int,
) {
    let mut zoomed: libc::c_int = 0;
    /* Check size limits. */
    if sx < 1u32 {
        sx = 1u32
    }
    if sx > 10000u32 {
        sx = 10000u32
    }
    if sy < 1u32 {
        sy = 1u32
    }
    if sy > 10000u32 {
        sy = 10000u32
    }
    /* If the window is zoomed, unzoom. */
    zoomed = (*w).flags & 0x8i32;
    if zoomed != 0 {
        window_unzoom(w);
    }
    /* Resize the layout first. */
    layout_resize(w, sx, sy);
    /* Resize the window, it can be no smaller than the layout. */
    if sx < (*(*w).layout_root).sx {
        sx = (*(*w).layout_root).sx
    }
    if sy < (*(*w).layout_root).sy {
        sy = (*(*w).layout_root).sy
    }
    window_resize(w, sx, sy, xpixel, ypixel);
    log_debug(
        b"%s: @%u resized to %u,%u; layout %u,%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"resize_window\x00")).as_ptr(),
        (*w).id,
        sx,
        sy,
        (*(*w).layout_root).sx,
        (*(*w).layout_root).sy,
    );
    /* Restore the window zoom state. */
    if zoomed != 0 {
        window_zoom((*w).active);
    }
    tty_update_window_offset(w);
    server_redraw_window(w);
    notify_window(
        b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
        w,
    );
    (*w).flags &= !(0x20i32);
}
unsafe extern "C" fn ignore_client_size(mut c: *mut client) -> libc::c_int {
    let mut loop_0: *mut client = 0 as *mut client;
    if (*c).session.is_null() {
        return 1i32;
    }
    if (*c).flags & (0x200i32 | 0x40i32 | 0x4i32) as libc::c_ulong != 0 {
        return 1i32;
    }
    if (*c).flags & 0x20000u64 != 0 {
        /*
         * Ignore flagged clients if there are any attached clients
         * that aren't flagged.
         */
        loop_0 = clients.tqh_first;
        while !loop_0.is_null() {
            if !(*loop_0).session.is_null() {
                if !((*loop_0).flags & (0x200i32 | 0x40i32 | 0x4i32) as libc::c_ulong != 0) {
                    if !(*loop_0).flags & 0x20000u64 != 0 {
                        return 1i32;
                    }
                }
            }
            loop_0 = (*loop_0).entry.tqe_next
        }
    }
    if (*c).flags & 0x2000u64 != 0 && !(*c).flags & 0x400000u64 != 0 {
        return 1i32;
    }
    return 0i32;
}
unsafe extern "C" fn clients_with_window(mut w: *mut window) -> u_int {
    let mut loop_0: *mut client = 0 as *mut client;
    let mut n: u_int = 0u32;
    loop_0 = clients.tqh_first;
    while !loop_0.is_null() {
        if !(ignore_client_size(loop_0) != 0 || session_has((*loop_0).session, w) == 0) {
            n = n.wrapping_add(1);
            if n > 1u32 {
                break;
            }
        }
        loop_0 = (*loop_0).entry.tqe_next
    }
    return n;
}
unsafe extern "C" fn clients_calculate_size(
    mut type_0: libc::c_int,
    mut current: libc::c_int,
    mut s: *mut session,
    mut w: *mut window,
    mut skip_client: Option<
        unsafe extern "C" fn(
            _: *mut client,
            _: libc::c_int,
            _: libc::c_int,
            _: *mut session,
            _: *mut window,
        ) -> libc::c_int,
    >,
    mut sx: *mut u_int,
    mut sy: *mut u_int,
    mut xpixel: *mut u_int,
    mut ypixel: *mut u_int,
) -> libc::c_int {
    let mut loop_0: *mut client = 0 as *mut client;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut n: u_int = 0u32;
    /* Manual windows do not have their size changed based on a client. */
    if type_0 == 2i32 {
        return 0i32;
    }
    /*
     * Start comparing with 0 for largest and UINT_MAX for smallest or
     * latest.
     */
    if type_0 == 0i32 {
        *sy = 0u32;
        *sx = *sy
    } else {
        *sy = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
        *sx = *sy
    }
    *ypixel = 0u32;
    *xpixel = *ypixel;
    /*
     * For latest, count the number of clients with this window. We only
     * care if there is more than one.
     */
    if type_0 == 3i32 {
        n = clients_with_window(w)
    }
    /* Loop over the clients and work out the size. */
    loop_0 = clients.tqh_first;
    while !loop_0.is_null() {
        if !(ignore_client_size(loop_0) != 0) {
            if !(skip_client.expect("non-null function pointer")(loop_0, type_0, current, s, w)
                != 0)
            {
                /*
                 * If there are multiple clients attached, only accept the
                 * latest client; otherwise let the only client be chosen as
                 * for smallest.
                 */
                if !(type_0 == 3i32 && n > 1u32 && loop_0 != (*w).latest as *mut client) {
                    /* Work out this client's size. */
                    cx = (*loop_0).tty.sx;
                    cy = (*loop_0).tty.sy.wrapping_sub(status_line_size(loop_0));
                    /*
                     * If it is larger or smaller than the best so far, update the
                     * new size.
                     */
                    if type_0 == 0i32 {
                        if cx > *sx {
                            *sx = cx
                        }
                        if cy > *sy {
                            *sy = cy
                        }
                    } else {
                        if cx < *sx {
                            *sx = cx
                        }
                        if cy < *sy {
                            *sy = cy
                        }
                    }
                    if (*loop_0).tty.xpixel > *xpixel && (*loop_0).tty.ypixel > *ypixel {
                        *xpixel = (*loop_0).tty.xpixel;
                        *ypixel = (*loop_0).tty.ypixel
                    }
                }
            }
        }
        loop_0 = (*loop_0).entry.tqe_next
    }
    /* Return whether a suitable size was found. */
    if type_0 == 0i32 {
        return (*sx != 0u32 && *sy != 0u32) as libc::c_int;
    }
    return (*sx != (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32)
        && *sy != (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32)) as libc::c_int;
}
unsafe extern "C" fn default_window_size_skip_client(
    mut loop_0: *mut client,
    mut type_0: libc::c_int,
    mut _current: libc::c_int,
    mut s: *mut session,
    mut w: *mut window,
) -> libc::c_int {
    /*
     * Latest checks separately, so do not check here. Otherwise only
     * include clients where the session contains the window or where the
     * session is the given session.
     */
    if type_0 == 3i32 {
        return 0i32;
    }
    if !w.is_null() && session_has((*loop_0).session, w) == 0 {
        return 1i32;
    }
    if w.is_null() && (*loop_0).session != s {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn default_window_size(
    mut c: *mut client,
    mut s: *mut session,
    mut w: *mut window,
    mut sx: *mut u_int,
    mut sy: *mut u_int,
    mut xpixel: *mut u_int,
    mut ypixel: *mut u_int,
    mut type_0: libc::c_int,
) {
    let mut current_block: u64;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    /* Get type if not provided. */
    if type_0 == -(1i32) {
        type_0 = options_get_number(
            global_w_options,
            b"window-size\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int
    }
    /*
     * Latest clients can use the given client if suitable. If there is no
     * client and no window, use the default size as for manual type.
     */
    if type_0 == 3i32 {
        if !c.is_null() && ignore_client_size(c) == 0 {
            *sx = (*c).tty.sx;
            *sy = (*c).tty.sy.wrapping_sub(status_line_size(c));
            *xpixel = (*c).tty.xpixel;
            *ypixel = (*c).tty.ypixel;
            current_block = 8381394993143607130;
        } else {
            if w.is_null() {
                type_0 = 2i32
            }
            current_block = 2868539653012386629;
        }
    } else {
        current_block = 2868539653012386629;
    }
    match current_block {
        2868539653012386629 => {
            /*
             * Look for a client to base the size on. If none exists (or the type
             * is manual), use the default-size option.
             */
            if clients_calculate_size(
                type_0,
                0i32,
                s,
                w,
                Some(
                    default_window_size_skip_client
                        as unsafe extern "C" fn(
                            _: *mut client,
                            _: libc::c_int,
                            _: libc::c_int,
                            _: *mut session,
                            _: *mut window,
                        ) -> libc::c_int,
                ),
                sx,
                sy,
                xpixel,
                ypixel,
            ) == 0
            {
                value = options_get_string(
                    (*s).options,
                    b"default-size\x00" as *const u8 as *const libc::c_char,
                );
                if sscanf(
                    value,
                    b"%ux%u\x00" as *const u8 as *const libc::c_char,
                    sx,
                    sy,
                ) != 2i32
                {
                    *sx = 80u32;
                    *sy = 24u32
                }
            }
        }
        _ => {}
    }
    /* Make sure the limits are enforced. */
    if *sx < 1u32 {
        *sx = 1u32
    }
    if *sx > 10000u32 {
        *sx = 10000u32
    }
    if *sy < 1u32 {
        *sy = 1u32
    }
    if *sy > 10000u32 {
        *sy = 10000u32
    };
}
unsafe extern "C" fn recalculate_size_skip_client(
    mut loop_0: *mut client,
    mut _type_0: libc::c_int,
    mut current: libc::c_int,
    mut _s: *mut session,
    mut w: *mut window,
) -> libc::c_int {
    /*
     * If the current flag is set, then skip any client where this window
     * is not the current window - this is used for aggressive-resize.
     * Otherwise skip any session that doesn't contain the window.
     */
    if current != 0 {
        return ((*(*(*loop_0).session).curw).window != w) as libc::c_int;
    }
    return (session_has((*loop_0).session, w) == 0i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn recalculate_size(mut w: *mut window, mut now: libc::c_int) {
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut xpixel: u_int = 0u32;
    let mut ypixel: u_int = 0u32;
    let mut type_0: libc::c_int = 0;
    let mut current: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    /*
     * Do not attempt to resize windows which have no pane, they must be on
     * the way to destruction.
     */
    if (*w).active.is_null() {
        return;
    }
    log_debug(
        b"%s: @%u is %u,%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"recalculate_size\x00"))
            .as_ptr(),
        (*w).id,
        (*w).sx,
        (*w).sy,
    );
    /*
     * Type is manual, smallest, largest, latest. Current is the
     * aggressive-resize option (do not resize based on clients where the
     * window is not the current window).
     */
    type_0 = options_get_number(
        (*w).options,
        b"window-size\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    current = options_get_number(
        (*w).options,
        b"aggressive-resize\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    /* Look for a suitable client and get the new size. */
    changed = clients_calculate_size(
        type_0,
        current,
        0 as *mut session,
        w,
        Some(
            recalculate_size_skip_client
                as unsafe extern "C" fn(
                    _: *mut client,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: *mut session,
                    _: *mut window,
                ) -> libc::c_int,
        ),
        &mut sx,
        &mut sy,
        &mut xpixel,
        &mut ypixel,
    );
    /*
     * Make sure the size has actually changed. If the window has already
     * got a resize scheduled, then use the new size; otherwise the old.
     */
    if (*w).flags & 0x20i32 != 0 {
        if now == 0 && changed != 0 && (*w).new_sx == sx && (*w).new_sy == sy {
            changed = 0i32
        }
    } else if now == 0 && changed != 0 && (*w).sx == sx && (*w).sy == sy {
        changed = 0i32
    }
    /*
     * If the size hasn't changed, update the window offset but not the
     * size.
     */
    if changed == 0 {
        tty_update_window_offset(w);
        return;
    }
    /*
     * If the now flag is set or if the window is sized manually, change
     * the size immediately. Otherwise set the flag and it will be done
     * later.
     */
    log_debug(
        b"%s: @%u new size %u,%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"recalculate_size\x00"))
            .as_ptr(),
        (*w).id,
        sx,
        sy,
    );
    if now != 0 || type_0 == 2i32 {
        resize_window(w, sx, sy, xpixel as libc::c_int, ypixel as libc::c_int);
    } else {
        (*w).new_sx = sx;
        (*w).new_sy = sy;
        (*w).new_xpixel = xpixel;
        (*w).new_ypixel = ypixel;
        (*w).flags |= 0x20i32;
        tty_update_window_offset(w);
    };
}
#[no_mangle]
pub unsafe extern "C" fn recalculate_sizes() {
    recalculate_sizes_now(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn recalculate_sizes_now(mut now: libc::c_int) {
    let mut s: *mut session = 0 as *mut session;
    let mut c: *mut client = 0 as *mut client;
    let mut w: *mut window = 0 as *mut window;
    /*
     * Clear attached count and update saved status line information for
     * each session.
     */
    s = sessions_RB_MINMAX(&mut sessions, -(1i32));
    while !s.is_null() {
        (*s).attached = 0u32;
        status_update_cache(s);
        s = sessions_RB_NEXT(s)
    }
    /*
     * Increment attached count and check the status line size for each
     * client.
     */
    c = clients.tqh_first;
    while !c.is_null() {
        s = (*c).session;
        if !s.is_null() && (*c).flags & (0x200i32 | 0x40i32 | 0x4i32) as libc::c_ulong == 0 {
            (*s).attached = (*s).attached.wrapping_add(1)
        }
        if !(ignore_client_size(c) != 0) {
            if (*c).tty.sy <= (*s).statuslines || (*c).flags & 0x2000u64 != 0 {
                (*c).flags |= 0x800000u64
            } else {
                (*c).flags &= !(0x800000i32) as libc::c_ulong
            }
        }
        c = (*c).entry.tqe_next
    }
    /* Walk each window and adjust the size. */
    w = windows_RB_MINMAX(&mut windows, -(1i32));
    while !w.is_null() {
        recalculate_size(w, now);
        w = windows_RB_NEXT(w)
    }
}
