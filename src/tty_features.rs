use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn tty_term_apply(_: *mut tty_term, _: *const libc::c_char, _: libc::c_int);
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
/* $OpenBSD$ */
/*
 * Copyright (c) 2020 Nicholas Marriott <nicholas.marriott@gmail.com>
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
 * Still hardcoded:
 * - mouse (under kmous capability);
 * - default colours (under AX or op capabilities);
 * - AIX colours (under colors >= 16);
 * - alternate escape (if terminal is VT100-like).
 *
 * Also:
 * - DECFRA uses a flag instead of capabilities;
 * - UTF-8 is a separate flag on the client; needed for unattached clients.
 */
/* A named terminal feature. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_feature {
    pub name: *const libc::c_char,
    pub capabilities: *mut *const libc::c_char,
    pub flags: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub name: *const libc::c_char,
    pub version: u_int,
    pub features: *const libc::c_char,
}
/* Terminal has xterm(1) title setting. */
static mut tty_feature_title_capabilities: [*const libc::c_char; 3] = [
    b"tsl=\\E]0;\x00" as *const u8 as *const libc::c_char,
    b"fsl=\\a\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_title: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"title\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_title_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/* Terminal can set the clipboard with OSC 52. */
static mut tty_feature_clipboard_capabilities: [*const libc::c_char; 2] = [
    b"Ms=\\E]52;%p1%s;%p2%s\\a\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_clipboard: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"clipboard\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_clipboard_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/*
 * Terminal supports RGB colour. This replaces setab and setaf also since
 * terminals with RGB have versions that do not allow setting colours from the
 * 256 palette.
 */
static mut tty_feature_rgb_capabilities: [*const libc::c_char; 6] = [
    b"AX\x00" as *const u8 as *const libc::c_char,
    b"setrgbf=\\E[38;2;%p1%d;%p2%d;%p3%dm\x00" as *const u8 as *const libc::c_char,
    b"setrgbb=\\E[48;2;%p1%d;%p2%d;%p3%dm\x00" as *const u8 as *const libc::c_char,
    b"setab=\\E[%?%p1%{8}%<%t4%p1%d%e%p1%{16}%<%t10%p1%{8}%-%d%e48;5;%p1%d%;m\x00" as *const u8
        as *const libc::c_char,
    b"setaf=\\E[%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m\x00" as *const u8
        as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_rgb: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"RGB\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_rgb_capabilities.as_ptr() as *mut _,
            flags: 0x1i32 | 0x10i32,
        };
        init
    }
};
/* Terminal supports 256 colours. */
static mut tty_feature_256_capabilities: [*const libc::c_char; 4] = [
    b"AX\x00" as *const u8 as *const libc::c_char,
    b"setab=\\E[%?%p1%{8}%<%t4%p1%d%e%p1%{16}%<%t10%p1%{8}%-%d%e48;5;%p1%d%;m\x00" as *const u8
        as *const libc::c_char,
    b"setaf=\\E[%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m\x00" as *const u8
        as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_256: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"256\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_256_capabilities.as_ptr() as *mut _,
            flags: 0x1i32,
        };
        init
    }
};
/* Terminal supports overline. */
static mut tty_feature_overline_capabilities: [*const libc::c_char; 2] = [
    b"Smol=\\E[53m\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_overline: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"overline\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_overline_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/* Terminal supports underscore styles. */
static mut tty_feature_usstyle_capabilities: [*const libc::c_char; 4] = [
    b"Smulx=\\E[4::%p1%dm\x00" as *const u8 as *const libc::c_char,
    b"Setulc=\\E[58::2::%p1%{65536}%/%d::%p1%{256}%/%{255}%&%d::%p1%{255}%&%d%;m\x00" as *const u8
        as *const libc::c_char,
    b"ol=\\E[59m\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_usstyle: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"usstyle\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_usstyle_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/* Terminal supports bracketed paste. */
static mut tty_feature_bpaste_capabilities: [*const libc::c_char; 3] = [
    b"Enbp=\\E[?2004h\x00" as *const u8 as *const libc::c_char,
    b"Dsbp=\\E[?2004l\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_bpaste: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"bpaste\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_bpaste_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/* Terminal supports focus reporting. */
static mut tty_feature_focus_capabilities: [*const libc::c_char; 3] = [
    b"Enfcs=\\E[?1004h\x00" as *const u8 as *const libc::c_char,
    b"Dsfcs=\\E[?1004l\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_focus: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"focus\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_focus_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/* Terminal supports cursor styles. */
static mut tty_feature_cstyle_capabilities: [*const libc::c_char; 3] = [
    b"Ss=\\E[%p1%d q\x00" as *const u8 as *const libc::c_char,
    b"Se=\\E[2 q\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_cstyle: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"cstyle\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_cstyle_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/* Terminal supports cursor colours. */
static mut tty_feature_ccolour_capabilities: [*const libc::c_char; 3] = [
    b"Cs=\\E]12;%p1%s\\a\x00" as *const u8 as *const libc::c_char,
    b"Cr=\\E]112\\a\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_ccolour: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"ccolour\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_ccolour_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/* Terminal supports strikethrough. */
static mut tty_feature_strikethrough_capabilities: [*const libc::c_char; 2] = [
    b"smxx=\\E[9m\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_strikethrough: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"strikethrough\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_strikethrough_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/* Terminal supports synchronized updates. */
static mut tty_feature_sync_capabilities: [*const libc::c_char; 2] = [
    b"Sync=\\EP=%p1%ds\\E\\\\\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_sync: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"sync\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_sync_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/* Terminal supports extended keys. */
static mut tty_feature_extkeys_capabilities: [*const libc::c_char; 3] = [
    b"Eneks=\\E[>4;1m\x00" as *const u8 as *const libc::c_char,
    b"Dseks=\\E[>4m\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_extkeys: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"extkeys\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_extkeys_capabilities.as_ptr() as *mut _,
            flags: 0i32,
        };
        init
    }
};
/* Terminal supports DECSLRM margins. */
static mut tty_feature_margins_capabilities: [*const libc::c_char; 5] = [
    b"Enmg=\\E[?69h\x00" as *const u8 as *const libc::c_char,
    b"Dsmg=\\E[?69l\x00" as *const u8 as *const libc::c_char,
    b"Clmg=\\E[s\x00" as *const u8 as *const libc::c_char,
    b"Cmg=\\E[%i%p1%d;%p2%ds\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut tty_feature_margins: tty_feature = unsafe {
    {
        let mut init = tty_feature {
            name: b"margins\x00" as *const u8 as *const libc::c_char,
            capabilities: tty_feature_margins_capabilities.as_ptr() as *mut _,
            flags: 0x4i32,
        };
        init
    }
};
/* Terminal supports DECFRA rectangle fill. */
static mut tty_feature_rectfill: tty_feature = {
    let mut init = tty_feature {
        name: b"rectfill\x00" as *const u8 as *const libc::c_char,
        capabilities: 0 as *mut *const libc::c_char,
        flags: 0x8i32,
    };
    init
};
/* Available terminal features. */
static mut tty_features: [*const tty_feature; 15] = unsafe {
    [
        &tty_feature_256 as *const tty_feature,
        &tty_feature_bpaste as *const tty_feature,
        &tty_feature_ccolour as *const tty_feature,
        &tty_feature_clipboard as *const tty_feature,
        &tty_feature_cstyle as *const tty_feature,
        &tty_feature_extkeys as *const tty_feature,
        &tty_feature_focus as *const tty_feature,
        &tty_feature_margins as *const tty_feature,
        &tty_feature_overline as *const tty_feature,
        &tty_feature_rectfill as *const tty_feature,
        &tty_feature_rgb as *const tty_feature,
        &tty_feature_strikethrough as *const tty_feature,
        &tty_feature_sync as *const tty_feature,
        &tty_feature_title as *const tty_feature,
        &tty_feature_usstyle as *const tty_feature,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn tty_add_features(
    mut feat: *mut libc::c_int,
    mut s: *const libc::c_char,
    mut separators: *const libc::c_char,
) {
    let mut tf: *const tty_feature = 0 as *const tty_feature;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut loop_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    log_debug(
        b"adding terminal features %s\x00" as *const u8 as *const libc::c_char,
        s,
    );
    copy = xstrdup(s);
    loop_0 = copy;
    loop {
        next = strsep(&mut loop_0, separators);
        if next.is_null() {
            break;
        }
        i = 0u32;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[*const tty_feature; 15]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*const tty_feature>() as libc::c_ulong)
        {
            tf = tty_features[i as usize];
            if strcasecmp((*tf).name, next) == 0i32 {
                break;
            }
            i = i.wrapping_add(1)
        }
        if i as libc::c_ulong
            == (::std::mem::size_of::<[*const tty_feature; 15]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*const tty_feature>() as libc::c_ulong)
        {
            log_debug(
                b"unknown terminal feature: %s\x00" as *const u8 as *const libc::c_char,
                next,
            );
            break;
        } else if !*feat & (1i32) << i != 0 {
            log_debug(
                b"adding terminal feature: %s\x00" as *const u8 as *const libc::c_char,
                (*tf).name,
            );
            *feat |= (1i32) << i
        }
    }
    free(copy as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tty_get_features(mut feat: libc::c_int) -> *const libc::c_char {
    let mut tf: *const tty_feature = 0 as *const tty_feature;
    static mut s: [libc::c_char; 512] = [0; 512];
    let mut i: u_int = 0;
    *s.as_mut_ptr() = '\u{0}' as libc::c_char;
    i = 0u32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*const tty_feature; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const tty_feature>() as libc::c_ulong)
    {
        if !(!feat & (1i32) << i != 0) {
            tf = tty_features[i as usize];
            strlcat(
                s.as_mut_ptr(),
                (*tf).name,
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            );
            strlcat(
                s.as_mut_ptr(),
                b",\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
            );
        }
        i = i.wrapping_add(1)
    }
    if *s.as_mut_ptr() as libc::c_int != '\u{0}' as i32 {
        s[strlen(s.as_mut_ptr()).wrapping_sub(1u64) as usize] = '\u{0}' as libc::c_char
    }
    return s.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn tty_apply_features(
    mut term: *mut tty_term,
    mut feat: libc::c_int,
) -> libc::c_int {
    let mut tf: *const tty_feature = 0 as *const tty_feature;
    let mut capability: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut i: u_int = 0;
    if feat == 0i32 {
        return 0i32;
    }
    log_debug(
        b"applying terminal features: %s\x00" as *const u8 as *const libc::c_char,
        tty_get_features(feat),
    );
    i = 0u32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*const tty_feature; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const tty_feature>() as libc::c_ulong)
    {
        if !((*term).features & (1i32) << i != 0 || !feat & (1i32) << i != 0) {
            tf = tty_features[i as usize];
            log_debug(
                b"applying terminal feature: %s\x00" as *const u8 as *const libc::c_char,
                (*tf).name,
            );
            if !(*tf).capabilities.is_null() {
                capability = (*tf).capabilities;
                while !(*capability).is_null() {
                    log_debug(
                        b"adding capability: %s\x00" as *const u8 as *const libc::c_char,
                        *capability,
                    );
                    tty_term_apply(term, *capability, 1i32);
                    capability = capability.offset(1)
                }
            }
            (*term).flags |= (*tf).flags
        }
        i = i.wrapping_add(1)
    }
    if (*term).features | feat == (*term).features {
        return 0i32;
    }
    (*term).features |= feat;
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn tty_default_features(
    mut feat: *mut libc::c_int,
    mut name: *const libc::c_char,
    mut version: u_int,
) {
    static mut table: [C2RustUnnamed_32; 5] = [
        {
            let mut init =
                 C2RustUnnamed_32{name:
                                      b"mintty\x00" as *const u8 as
                                          *const libc::c_char,
                                  version: 0,
                                  features:
                                      b"256,RGB,bpaste,clipboard,strikethrough,title,ccolour,cstyle,extkeys,margins,overline,usstyle\x00"
                                          as *const u8 as
                                          *const libc::c_char,};
            init
        },
        {
            let mut init =
                 C2RustUnnamed_32{name:
                                      b"tmux\x00" as *const u8 as
                                          *const libc::c_char,
                                  version: 0,
                                  features:
                                      b"256,RGB,bpaste,clipboard,strikethrough,title,ccolour,cstyle,focus,overline,usstyle\x00"
                                          as *const u8 as
                                          *const libc::c_char,};
            init
        },
        {
            let mut init = C2RustUnnamed_32 {
                name: b"rxvt-unicode\x00" as *const u8 as *const libc::c_char,
                version: 0,
                features: b"256,bpaste,ccolour,cstyle,title\x00" as *const u8
                    as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_32 {
                name: b"iTerm2\x00" as *const u8 as *const libc::c_char,
                version: 0,
                features:
                    b"256,RGB,bpaste,clipboard,strikethrough,title,cstyle,extkeys,margins,sync\x00"
                        as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 C2RustUnnamed_32{name:
                                      b"XTerm\x00" as *const u8 as
                                          *const libc::c_char,
                                  version: 0,
                                  features:
                                      b"256,RGB,bpaste,clipboard,strikethrough,title,ccolour,cstyle,extkeys,focus,margins,rectfill\x00"
                                          as *const u8 as
                                          *const libc::c_char,};
            init
        },
    ];
    let mut i: u_int = 0;
    i = 0u32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[C2RustUnnamed_32; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_32>() as libc::c_ulong)
    {
        if !(strcmp(table[i as usize].name, name) != 0i32) {
            if !(version != 0u32 && version < table[i as usize].version) {
                tty_add_features(
                    feat,
                    table[i as usize].features,
                    b",\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        i = i.wrapping_add(1)
    }
}
