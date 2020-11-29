use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn notify_window(_: *const libc::c_char, _: *mut window);
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn window_pane_resize(_: *mut window_pane, _: u_int, _: u_int);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
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
#[no_mangle]
pub unsafe extern "C" fn layout_create_cell(mut lcparent: *mut layout_cell) -> *mut layout_cell {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    lc = xmalloc(::std::mem::size_of::<layout_cell>() as libc::c_ulong) as *mut layout_cell;
    (*lc).type_0 = LAYOUT_WINDOWPANE;
    (*lc).parent = lcparent;
    (*lc).cells.tqh_first = 0 as *mut layout_cell;
    (*lc).cells.tqh_last = &mut (*lc).cells.tqh_first;
    (*lc).sx = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*lc).sy = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*lc).xoff = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*lc).yoff = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*lc).wp = 0 as *mut window_pane;
    return lc;
}
#[no_mangle]
pub unsafe extern "C" fn layout_free_cell(mut lc: *mut layout_cell) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    match (*lc).type_0 {
        0 | 1 => {
            while !(*lc).cells.tqh_first.is_null() {
                lcchild = (*lc).cells.tqh_first;
                if !(*lcchild).entry.tqe_next.is_null() {
                    (*(*lcchild).entry.tqe_next).entry.tqe_prev = (*lcchild).entry.tqe_prev
                } else {
                    (*lc).cells.tqh_last = (*lcchild).entry.tqe_prev
                }
                *(*lcchild).entry.tqe_prev = (*lcchild).entry.tqe_next;
                layout_free_cell(lcchild);
            }
        }
        2 => {
            if !(*lc).wp.is_null() {
                (*(*lc).wp).layout_cell = 0 as *mut layout_cell
            }
        }
        _ => {}
    }
    free(lc as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn layout_print_cell(
    mut lc: *mut layout_cell,
    mut hdr: *const libc::c_char,
    mut n: u_int,
) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut type_0: *const libc::c_char = 0 as *const libc::c_char;
    match (*lc).type_0 {
        0 => type_0 = b"LEFTRIGHT\x00" as *const u8 as *const libc::c_char,
        1 => type_0 = b"TOPBOTTOM\x00" as *const u8 as *const libc::c_char,
        2 => type_0 = b"WINDOWPANE\x00" as *const u8 as *const libc::c_char,
        _ => type_0 = b"UNKNOWN\x00" as *const u8 as *const libc::c_char,
    }
    log_debug(
        b"%s:%*s%p type %s [parent %p] wp=%p [%u,%u %ux%u]\x00" as *const u8 as *const libc::c_char,
        hdr,
        n,
        b" \x00" as *const u8 as *const libc::c_char,
        lc,
        type_0,
        (*lc).parent,
        (*lc).wp,
        (*lc).xoff,
        (*lc).yoff,
        (*lc).sx,
        (*lc).sy,
    );
    match (*lc).type_0 {
        0 | 1 => {
            lcchild = (*lc).cells.tqh_first;
            while !lcchild.is_null() {
                layout_print_cell(lcchild, hdr, n.wrapping_add(1u32));
                lcchild = (*lcchild).entry.tqe_next
            }
        }
        2 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_search_by_border(
    mut lc: *mut layout_cell,
    mut x: u_int,
    mut y: u_int,
) -> *mut layout_cell {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut last: *mut layout_cell = 0 as *mut layout_cell;
    lcchild = (*lc).cells.tqh_first;
    while !lcchild.is_null() {
        if x >= (*lcchild).xoff
            && x < (*lcchild).xoff.wrapping_add((*lcchild).sx)
            && y >= (*lcchild).yoff
            && y < (*lcchild).yoff.wrapping_add((*lcchild).sy)
        {
            /* Inside the cell - recurse. */
            return layout_search_by_border(lcchild, x, y);
        }
        if last.is_null() {
            last = lcchild
        } else {
            match (*lc).type_0 {
                0 => {
                    if x < (*lcchild).xoff && x >= (*last).xoff.wrapping_add((*last).sx) {
                        return last;
                    }
                }
                1 => {
                    if y < (*lcchild).yoff && y >= (*last).yoff.wrapping_add((*last).sy) {
                        return last;
                    }
                }
                2 | _ => {}
            }
            last = lcchild
        }
        lcchild = (*lcchild).entry.tqe_next
    }
    return 0 as *mut layout_cell;
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_size(
    mut lc: *mut layout_cell,
    mut sx: u_int,
    mut sy: u_int,
    mut xoff: u_int,
    mut yoff: u_int,
) {
    (*lc).sx = sx;
    (*lc).sy = sy;
    (*lc).xoff = xoff;
    (*lc).yoff = yoff;
}
#[no_mangle]
pub unsafe extern "C" fn layout_make_leaf(mut lc: *mut layout_cell, mut wp: *mut window_pane) {
    (*lc).type_0 = LAYOUT_WINDOWPANE;
    (*lc).cells.tqh_first = 0 as *mut layout_cell;
    (*lc).cells.tqh_last = &mut (*lc).cells.tqh_first;
    (*wp).layout_cell = lc;
    (*lc).wp = wp;
}
#[no_mangle]
pub unsafe extern "C" fn layout_make_node(mut lc: *mut layout_cell, mut type_0: layout_type) {
    if type_0 == LAYOUT_WINDOWPANE {
        fatalx(b"bad layout type\x00" as *const u8 as *const libc::c_char);
    }
    (*lc).type_0 = type_0;
    (*lc).cells.tqh_first = 0 as *mut layout_cell;
    (*lc).cells.tqh_last = &mut (*lc).cells.tqh_first;
    if !(*lc).wp.is_null() {
        (*(*lc).wp).layout_cell = 0 as *mut layout_cell
    }
    (*lc).wp = 0 as *mut window_pane;
}
/* Fix cell offsets for a child cell. */
unsafe extern "C" fn layout_fix_offsets1(mut lc: *mut layout_cell) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    if (*lc).type_0 == LAYOUT_LEFTRIGHT {
        xoff = (*lc).xoff;
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            (*lcchild).xoff = xoff;
            (*lcchild).yoff = (*lc).yoff;
            if (*lcchild).type_0 != LAYOUT_WINDOWPANE {
                layout_fix_offsets1(lcchild);
            }
            xoff = (xoff).wrapping_add((*lcchild).sx.wrapping_add(1u32));
            lcchild = (*lcchild).entry.tqe_next
        }
    } else {
        yoff = (*lc).yoff;
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            (*lcchild).xoff = (*lc).xoff;
            (*lcchild).yoff = yoff;
            if (*lcchild).type_0 != LAYOUT_WINDOWPANE {
                layout_fix_offsets1(lcchild);
            }
            yoff = (yoff).wrapping_add((*lcchild).sy.wrapping_add(1u32));
            lcchild = (*lcchild).entry.tqe_next
        }
    };
}
/* Update cell offsets based on their sizes. */
#[no_mangle]
pub unsafe extern "C" fn layout_fix_offsets(mut w: *mut window) {
    let mut lc: *mut layout_cell = (*w).layout_root;
    (*lc).xoff = 0u32;
    (*lc).yoff = 0u32;
    layout_fix_offsets1(lc);
}
/* Is this a top cell? */
unsafe extern "C" fn layout_cell_is_top(
    mut w: *mut window,
    mut lc: *mut layout_cell,
) -> libc::c_int {
    let mut next: *mut layout_cell = 0 as *mut layout_cell;
    while lc != (*w).layout_root {
        next = (*lc).parent;
        if (*next).type_0 == LAYOUT_TOPBOTTOM && lc != (*next).cells.tqh_first {
            return 0i32;
        }
        lc = next
    }
    return 1i32;
}
/* Is this a bottom cell? */
unsafe extern "C" fn layout_cell_is_bottom(
    mut w: *mut window,
    mut lc: *mut layout_cell,
) -> libc::c_int {
    let mut next: *mut layout_cell = 0 as *mut layout_cell;
    while lc != (*w).layout_root {
        next = (*lc).parent;
        if (*next).type_0 == LAYOUT_TOPBOTTOM
            && lc != *(*((*next).cells.tqh_last as *mut layout_cells)).tqh_last
        {
            return 0i32;
        }
        lc = next
    }
    return 1i32;
}
/*
 * Returns 1 if we need to add an extra line for the pane status line. This is
 * the case for the most upper or lower panes only.
 */
unsafe extern "C" fn layout_add_border(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut status: libc::c_int,
) -> libc::c_int {
    if status == 1i32 {
        return layout_cell_is_top(w, lc);
    }
    if status == 2i32 {
        return layout_cell_is_bottom(w, lc);
    }
    return 0i32;
}
/* Update pane offsets and sizes based on their cells. */
#[no_mangle]
pub unsafe extern "C" fn layout_fix_panes(mut w: *mut window) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut status: libc::c_int = 0;
    status = options_get_number(
        (*w).options,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        lc = (*wp).layout_cell;
        if !lc.is_null() {
            (*wp).xoff = (*lc).xoff;
            (*wp).yoff = (*lc).yoff;
            if layout_add_border(w, lc, status) != 0 {
                if status == 1i32 {
                    (*wp).yoff = (*wp).yoff.wrapping_add(1)
                }
                window_pane_resize(wp, (*lc).sx, (*lc).sy.wrapping_sub(1u32));
            } else {
                window_pane_resize(wp, (*lc).sx, (*lc).sy);
            }
        }
        wp = (*wp).entry.tqe_next
    }
}
/* Count the number of available cells in a layout. */
#[no_mangle]
pub unsafe extern "C" fn layout_count_cells(mut lc: *mut layout_cell) -> u_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut count: u_int = 0;
    match (*lc).type_0 {
        2 => return 1u32,
        0 | 1 => {
            count = 0u32;
            lcchild = (*lc).cells.tqh_first;
            while !lcchild.is_null() {
                count = (count).wrapping_add(layout_count_cells(lcchild));
                lcchild = (*lcchild).entry.tqe_next
            }
            return count;
        }
        _ => {
            fatalx(b"bad layout type\x00" as *const u8 as *const libc::c_char);
        }
    };
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2009 Nicholas Marriott <nicholas.marriott@gmail.com>
 * Copyright (c) 2016 Stephen Kent <smkent@smkent.net>
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
 * The window layout is a tree of cells each of which can be one of: a
 * left-right container for a list of cells, a top-bottom container for a list
 * of cells, or a container for a window pane.
 *
 * Each window has a pointer to the root of its layout tree (containing its
 * panes), every pane has a pointer back to the cell containing it, and each
 * cell a pointer to its parent cell.
 */
/* Calculate how much size is available to be removed from a cell. */
unsafe extern "C" fn layout_resize_check(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
) -> u_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut available: u_int = 0;
    let mut minimum: u_int = 0;
    let mut status: libc::c_int = 0;
    status = options_get_number(
        (*w).options,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if (*lc).type_0 == LAYOUT_WINDOWPANE {
        /* Space available in this cell only. */
        if type_0 == LAYOUT_LEFTRIGHT {
            available = (*lc).sx;
            minimum = 1u32
        } else {
            available = (*lc).sy;
            if layout_add_border(w, lc, status) != 0 {
                minimum = (1i32 + 1i32) as u_int
            } else {
                minimum = 1u32
            }
        }
        if available > minimum {
            available = (available).wrapping_sub(minimum)
        } else {
            available = 0u32
        }
    } else if (*lc).type_0 == type_0 {
        /* Same type: total of available space in all child cells. */
        available = 0u32;
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            available = (available).wrapping_add(layout_resize_check(w, lcchild, type_0));
            lcchild = (*lcchild).entry.tqe_next
        }
    } else {
        /* Different type: minimum of available space in child cells. */
        minimum = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            available = layout_resize_check(w, lcchild, type_0);
            if available < minimum {
                minimum = available
            }
            lcchild = (*lcchild).entry.tqe_next
        }
        available = minimum
    }
    return available;
}
/*
 * Adjust cell size evenly, including altering its children. This function
 * expects the change to have already been bounded to the space available.
 */
#[no_mangle]
pub unsafe extern "C" fn layout_resize_adjust(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut change: libc::c_int,
) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    /* Adjust the cell size. */
    if type_0 == LAYOUT_LEFTRIGHT {
        (*lc).sx = ((*lc).sx).wrapping_add(change as libc::c_uint)
    } else {
        (*lc).sy = ((*lc).sy).wrapping_add(change as libc::c_uint)
    }
    /* If this is a leaf cell, that is all that is necessary. */
    if type_0 == LAYOUT_WINDOWPANE {
        return;
    }
    /* Child cell runs in a different direction. */
    if (*lc).type_0 != type_0 {
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            layout_resize_adjust(w, lcchild, type_0, change);
            lcchild = (*lcchild).entry.tqe_next
        }
        return;
    }
    /*
     * Child cell runs in the same direction. Adjust each child equally
     * until no further change is possible.
     */
    while change != 0i32 {
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            if change == 0i32 {
                break;
            }
            if change > 0i32 {
                layout_resize_adjust(w, lcchild, type_0, 1i32);
                change -= 1
            } else if layout_resize_check(w, lcchild, type_0) > 0u32 {
                layout_resize_adjust(w, lcchild, type_0, -(1i32));
                change += 1
            }
            lcchild = (*lcchild).entry.tqe_next
        }
    }
}
/* Destroy a cell and redistribute the space. */
#[no_mangle]
pub unsafe extern "C" fn layout_destroy_cell(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut lcroot: *mut *mut layout_cell,
) {
    let mut lcother: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    /*
     * If no parent, this is the last pane so window close is imminent and
     * there is no need to resize anything.
     */
    lcparent = (*lc).parent;
    if lcparent.is_null() {
        layout_free_cell(lc);
        *lcroot = 0 as *mut layout_cell;
        return;
    }
    /* Merge the space into the previous or next cell. */
    if lc == (*lcparent).cells.tqh_first {
        lcother = (*lc).entry.tqe_next
    } else {
        lcother = *(*((*lc).entry.tqe_prev as *mut layout_cells)).tqh_last
    }
    if !lcother.is_null() && (*lcparent).type_0 == LAYOUT_LEFTRIGHT {
        layout_resize_adjust(
            w,
            lcother,
            (*lcparent).type_0,
            (*lc).sx.wrapping_add(1u32) as libc::c_int,
        );
    } else if !lcother.is_null() {
        layout_resize_adjust(
            w,
            lcother,
            (*lcparent).type_0,
            (*lc).sy.wrapping_add(1u32) as libc::c_int,
        );
    }
    /* Remove this from the parent's list. */
    if !(*lc).entry.tqe_next.is_null() {
        (*(*lc).entry.tqe_next).entry.tqe_prev = (*lc).entry.tqe_prev
    } else {
        (*lcparent).cells.tqh_last = (*lc).entry.tqe_prev
    }
    *(*lc).entry.tqe_prev = (*lc).entry.tqe_next;
    layout_free_cell(lc);
    /*
     * If the parent now has one cell, remove the parent from the tree and
     * replace it by that cell.
     */
    lc = (*lcparent).cells.tqh_first;
    if (*lc).entry.tqe_next.is_null() {
        if !(*lc).entry.tqe_next.is_null() {
            (*(*lc).entry.tqe_next).entry.tqe_prev = (*lc).entry.tqe_prev
        } else {
            (*lcparent).cells.tqh_last = (*lc).entry.tqe_prev
        }
        *(*lc).entry.tqe_prev = (*lc).entry.tqe_next;
        (*lc).parent = (*lcparent).parent;
        if (*lc).parent.is_null() {
            (*lc).xoff = 0u32;
            (*lc).yoff = 0u32;
            *lcroot = lc
        } else {
            (*lc).entry.tqe_next = (*lcparent).entry.tqe_next;
            if !(*lc).entry.tqe_next.is_null() {
                (*(*lc).entry.tqe_next).entry.tqe_prev = &mut (*lc).entry.tqe_next
            } else {
                (*(*lc).parent).cells.tqh_last = &mut (*lc).entry.tqe_next
            }
            (*lc).entry.tqe_prev = (*lcparent).entry.tqe_prev;
            *(*lc).entry.tqe_prev = lc
        }
        layout_free_cell(lcparent);
    };
}
#[no_mangle]
pub unsafe extern "C" fn layout_init(mut w: *mut window, mut wp: *mut window_pane) {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    layout_set_size(lc, (*w).sx, (*w).sy, 0u32, 0u32);
    layout_make_leaf(lc, wp);
    layout_fix_panes(w);
}
#[no_mangle]
pub unsafe extern "C" fn layout_free(mut w: *mut window) {
    layout_free_cell((*w).layout_root);
}
/* Resize the entire layout after window resize. */
#[no_mangle]
pub unsafe extern "C" fn layout_resize(mut w: *mut window, mut sx: u_int, mut sy: u_int) {
    let mut lc: *mut layout_cell = (*w).layout_root;
    let mut xlimit: libc::c_int = 0;
    let mut ylimit: libc::c_int = 0;
    let mut xchange: libc::c_int = 0;
    let mut ychange: libc::c_int = 0;
    /*
     * Adjust horizontally. Do not attempt to reduce the layout lower than
     * the minimum (more than the amount returned by layout_resize_check).
     *
     * This can mean that the window size is smaller than the total layout
     * size: redrawing this is handled at a higher level, but it does leave
     * a problem with growing the window size here: if the current size is
     * < the minimum, growing proportionately by adding to each pane is
     * wrong as it would keep the layout size larger than the window size.
     * Instead, spread the difference between the minimum and the new size
     * out proportionately - this should leave the layout fitting the new
     * window size.
     */
    xchange = sx.wrapping_sub((*lc).sx) as libc::c_int;
    xlimit = layout_resize_check(w, lc, LAYOUT_LEFTRIGHT) as libc::c_int;
    if xchange < 0i32 && xchange < -xlimit {
        xchange = -xlimit
    }
    if xlimit == 0i32 {
        if sx <= (*lc).sx {
            /* lc->sx is minimum possible */
            xchange = 0i32
        } else {
            xchange = sx.wrapping_sub((*lc).sx) as libc::c_int
        }
    }
    if xchange != 0i32 {
        layout_resize_adjust(w, lc, LAYOUT_LEFTRIGHT, xchange);
    }
    /* Adjust vertically in a similar fashion. */
    ychange = sy.wrapping_sub((*lc).sy) as libc::c_int;
    ylimit = layout_resize_check(w, lc, LAYOUT_TOPBOTTOM) as libc::c_int;
    if ychange < 0i32 && ychange < -ylimit {
        ychange = -ylimit
    }
    if ylimit == 0i32 {
        if sy <= (*lc).sy {
            /* lc->sy is minimum possible */
            ychange = 0i32
        } else {
            ychange = sy.wrapping_sub((*lc).sy) as libc::c_int
        }
    }
    if ychange != 0i32 {
        layout_resize_adjust(w, lc, LAYOUT_TOPBOTTOM, ychange);
    }
    /* Fix cell offsets. */
    layout_fix_offsets(w);
    layout_fix_panes(w);
}
/* Resize a pane to an absolute size. */
#[no_mangle]
pub unsafe extern "C" fn layout_resize_pane_to(
    mut wp: *mut window_pane,
    mut type_0: layout_type,
    mut new_size: u_int,
) {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    let mut change: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    lc = (*wp).layout_cell;
    /* Find next parent of the same type. */
    lcparent = (*lc).parent;
    while !lcparent.is_null() && (*lcparent).type_0 != type_0 {
        lc = lcparent;
        lcparent = (*lc).parent
    }
    if lcparent.is_null() {
        return;
    }
    /* Work out the size adjustment. */
    if type_0 == LAYOUT_LEFTRIGHT {
        size = (*lc).sx as libc::c_int
    } else {
        size = (*lc).sy as libc::c_int
    }
    if lc == *(*((*lcparent).cells.tqh_last as *mut layout_cells)).tqh_last {
        change = (size as libc::c_uint).wrapping_sub(new_size) as libc::c_int
    } else {
        change = new_size.wrapping_sub(size as libc::c_uint) as libc::c_int
    }
    /* Resize the pane. */
    layout_resize_pane(wp, type_0, change, 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn layout_resize_layout(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut change: libc::c_int,
    mut opposite: libc::c_int,
) {
    let mut needed: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    /* Grow or shrink the cell. */
    needed = change;
    while needed != 0i32 {
        if change > 0i32 {
            size = layout_resize_pane_grow(w, lc, type_0, needed, opposite);
            needed -= size
        } else {
            size = layout_resize_pane_shrink(w, lc, type_0, needed);
            needed += size
        }
        if size == 0i32 {
            break;
        }
    }
    /* Fix cell offsets. */
    layout_fix_offsets(w);
    layout_fix_panes(w);
    notify_window(
        b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
        w,
    );
}
/* Resize a single pane within the layout. */
#[no_mangle]
pub unsafe extern "C" fn layout_resize_pane(
    mut wp: *mut window_pane,
    mut type_0: layout_type,
    mut change: libc::c_int,
    mut opposite: libc::c_int,
) {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    lc = (*wp).layout_cell;
    /* Find next parent of the same type. */
    lcparent = (*lc).parent;
    while !lcparent.is_null() && (*lcparent).type_0 != type_0 {
        lc = lcparent;
        lcparent = (*lc).parent
    }
    if lcparent.is_null() {
        return;
    }
    /* If this is the last cell, move back one. */
    if lc == *(*((*lcparent).cells.tqh_last as *mut layout_cells)).tqh_last {
        lc = *(*((*lc).entry.tqe_prev as *mut layout_cells)).tqh_last
    }
    layout_resize_layout((*wp).window, lc, type_0, change, opposite);
}
/* Helper function to grow pane. */
unsafe extern "C" fn layout_resize_pane_grow(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut needed: libc::c_int,
    mut opposite: libc::c_int,
) -> libc::c_int {
    let mut lcadd: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcremove: *mut layout_cell = 0 as *mut layout_cell;
    let mut size: u_int = 0u32;
    /* Growing. Always add to the current cell. */
    lcadd = lc;
    /* Look towards the tail for a suitable cell for reduction. */
    lcremove = (*lc).entry.tqe_next;
    while !lcremove.is_null() {
        size = layout_resize_check(w, lcremove, type_0);
        if size > 0u32 {
            break;
        }
        lcremove = (*lcremove).entry.tqe_next
    }
    /* If none found, look towards the head. */
    if opposite != 0 && lcremove.is_null() {
        lcremove = *(*((*lc).entry.tqe_prev as *mut layout_cells)).tqh_last;
        while !lcremove.is_null() {
            size = layout_resize_check(w, lcremove, type_0);
            if size > 0u32 {
                break;
            }
            lcremove = *(*((*lcremove).entry.tqe_prev as *mut layout_cells)).tqh_last
        }
    }
    if lcremove.is_null() {
        return 0i32;
    }
    /* Change the cells. */
    if size > needed as u_int {
        size = needed as u_int
    }
    layout_resize_adjust(w, lcadd, type_0, size as libc::c_int);
    layout_resize_adjust(w, lcremove, type_0, size.wrapping_neg() as libc::c_int);
    return size as libc::c_int;
}
/* Helper function to shrink pane. */
unsafe extern "C" fn layout_resize_pane_shrink(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut needed: libc::c_int,
) -> libc::c_int {
    let mut lcadd: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcremove: *mut layout_cell = 0 as *mut layout_cell;
    let mut size: u_int = 0;
    /* Shrinking. Find cell to remove from by walking towards head. */
    lcremove = lc;
    loop {
        size = layout_resize_check(w, lcremove, type_0);
        if size != 0u32 {
            break;
        }
        lcremove = *(*((*lcremove).entry.tqe_prev as *mut layout_cells)).tqh_last;
        if lcremove.is_null() {
            break;
        }
    }
    if lcremove.is_null() {
        return 0i32;
    }
    /* And add onto the next cell (from the original cell). */
    lcadd = (*lc).entry.tqe_next;
    if lcadd.is_null() {
        return 0i32;
    }
    /* Change the cells. */
    if size > -needed as u_int {
        size = -needed as u_int
    }
    layout_resize_adjust(w, lcadd, type_0, size as libc::c_int);
    layout_resize_adjust(w, lcremove, type_0, size.wrapping_neg() as libc::c_int);
    return size as libc::c_int;
}
/* Assign window pane to newly split cell. */
#[no_mangle]
pub unsafe extern "C" fn layout_assign_pane(mut lc: *mut layout_cell, mut wp: *mut window_pane) {
    layout_make_leaf(lc, wp);
    layout_fix_panes((*wp).window);
}
/* Calculate the new pane size for resized parent. */
unsafe extern "C" fn layout_new_pane_size(
    mut w: *mut window,
    mut previous: u_int,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut size: u_int,
    mut count_left: u_int,
    mut size_left: u_int,
) -> u_int {
    let mut new_size: u_int = 0;
    let mut min: u_int = 0;
    let mut max: u_int = 0;
    let mut available: u_int = 0;
    /* If this is the last cell, it can take all of the remaining size. */
    if count_left == 1u32 {
        return size_left;
    }
    /* How much is available in this parent? */
    available = layout_resize_check(w, lc, type_0);
    /*
     * Work out the minimum size of this cell and the new size
     * proportionate to the previous size.
     */
    min = ((1i32 + 1i32) as libc::c_uint).wrapping_mul(count_left.wrapping_sub(1u32));
    if type_0 == LAYOUT_LEFTRIGHT {
        if (*lc).sx.wrapping_sub(available) > min {
            min = (*lc).sx.wrapping_sub(available)
        }
        new_size = (*lc).sx.wrapping_mul(size).wrapping_div(previous)
    } else {
        if (*lc).sy.wrapping_sub(available) > min {
            min = (*lc).sy.wrapping_sub(available)
        }
        new_size = (*lc).sy.wrapping_mul(size).wrapping_div(previous)
    }
    /* Check against the maximum and minimum size. */
    max = size_left.wrapping_sub(min);
    if new_size > max {
        new_size = max
    }
    if new_size < 1u32 {
        new_size = 1u32
    }
    return new_size;
}
/* Check if the cell and all its children can be resized to a specific size. */
unsafe extern "C" fn layout_set_size_check(
    mut w: *mut window,
    mut lc: *mut layout_cell,
    mut type_0: layout_type,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut new_size: u_int = 0;
    let mut available: u_int = 0;
    let mut previous: u_int = 0;
    let mut count: u_int = 0;
    let mut idx: u_int = 0;
    /* Cells with no children must just be bigger than minimum. */
    if (*lc).type_0 == LAYOUT_WINDOWPANE {
        return (size >= 1i32) as libc::c_int;
    }
    available = size as u_int;
    /* Count number of children. */
    count = 0u32;
    lcchild = (*lc).cells.tqh_first;
    while !lcchild.is_null() {
        count = count.wrapping_add(1);
        lcchild = (*lcchild).entry.tqe_next
    }
    /* Check new size will work for each child. */
    if (*lc).type_0 == type_0 {
        if available < count.wrapping_mul(2u32).wrapping_sub(1u32) {
            return 0i32;
        }
        if type_0 == LAYOUT_LEFTRIGHT {
            previous = (*lc).sx
        } else {
            previous = (*lc).sy
        }
        idx = 0u32;
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            new_size = layout_new_pane_size(
                w,
                previous,
                lcchild,
                type_0,
                size as u_int,
                count.wrapping_sub(idx),
                available,
            );
            if idx == count.wrapping_sub(1u32) {
                if new_size > available {
                    return 0i32;
                }
                available = (available).wrapping_sub(new_size)
            } else {
                if new_size.wrapping_add(1u32) > available {
                    return 0i32;
                }
                available = (available).wrapping_sub(new_size.wrapping_add(1u32))
            }
            if layout_set_size_check(w, lcchild, type_0, new_size as libc::c_int) == 0 {
                return 0i32;
            }
            idx = idx.wrapping_add(1);
            lcchild = (*lcchild).entry.tqe_next
        }
    } else {
        lcchild = (*lc).cells.tqh_first;
        while !lcchild.is_null() {
            if !((*lcchild).type_0 == LAYOUT_WINDOWPANE) {
                if layout_set_size_check(w, lcchild, type_0, size) == 0 {
                    return 0i32;
                }
            }
            lcchild = (*lcchild).entry.tqe_next
        }
    }
    return 1i32;
}
/* Resize all child cells to fit within the current cell. */
unsafe extern "C" fn layout_resize_child_cells(mut w: *mut window, mut lc: *mut layout_cell) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut previous: u_int = 0;
    let mut available: u_int = 0;
    let mut count: u_int = 0;
    let mut idx: u_int = 0;
    if (*lc).type_0 == LAYOUT_WINDOWPANE {
        return;
    }
    /* What is the current size used? */
    count = 0u32;
    previous = 0u32;
    lcchild = (*lc).cells.tqh_first;
    while !lcchild.is_null() {
        count = count.wrapping_add(1);
        if (*lc).type_0 == LAYOUT_LEFTRIGHT {
            previous = (previous).wrapping_add((*lcchild).sx)
        } else if (*lc).type_0 == LAYOUT_TOPBOTTOM {
            previous = (previous).wrapping_add((*lcchild).sy)
        }
        lcchild = (*lcchild).entry.tqe_next
    }
    previous = (previous).wrapping_add(count.wrapping_sub(1u32));
    /* And how much is available? */
    available = 0u32;
    if (*lc).type_0 == LAYOUT_LEFTRIGHT {
        available = (*lc).sx
    } else if (*lc).type_0 == LAYOUT_TOPBOTTOM {
        available = (*lc).sy
    }
    /* Resize children into the new size. */
    idx = 0u32;
    lcchild = (*lc).cells.tqh_first;
    while !lcchild.is_null() {
        if (*lc).type_0 == LAYOUT_TOPBOTTOM {
            (*lcchild).sx = (*lc).sx;
            (*lcchild).xoff = (*lc).xoff
        } else {
            (*lcchild).sx = layout_new_pane_size(
                w,
                previous,
                lcchild,
                (*lc).type_0,
                (*lc).sx,
                count.wrapping_sub(idx),
                available,
            );
            available = (available).wrapping_sub((*lcchild).sx.wrapping_add(1u32))
        }
        if (*lc).type_0 == LAYOUT_LEFTRIGHT {
            (*lcchild).sy = (*lc).sy
        } else {
            (*lcchild).sy = layout_new_pane_size(
                w,
                previous,
                lcchild,
                (*lc).type_0,
                (*lc).sy,
                count.wrapping_sub(idx),
                available,
            );
            available = (available).wrapping_sub((*lcchild).sy.wrapping_add(1u32))
        }
        layout_resize_child_cells(w, lcchild);
        idx = idx.wrapping_add(1);
        lcchild = (*lcchild).entry.tqe_next
    }
}
/*
 * Split a pane into two. size is a hint, or -1 for default half/half
 * split. This must be followed by layout_assign_pane before much else happens!
 */
#[no_mangle]
pub unsafe extern "C" fn layout_split_pane(
    mut wp: *mut window_pane,
    mut type_0: layout_type,
    mut size: libc::c_int,
    mut flags: libc::c_int,
) -> *mut layout_cell {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcparent: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcnew: *mut layout_cell = 0 as *mut layout_cell;
    let mut lc1: *mut layout_cell = 0 as *mut layout_cell;
    let mut lc2: *mut layout_cell = 0 as *mut layout_cell;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    let mut size1: u_int = 0;
    let mut size2: u_int = 0;
    let mut minimum: u_int = 0;
    let mut new_size: u_int = 0;
    let mut saved_size: u_int = 0;
    let mut resize_first: u_int = 0u32;
    let mut full_size: libc::c_int = flags & 0x20i32;
    let mut status: libc::c_int = 0;
    /*
     * If full_size is specified, add a new cell at the top of the window
     * layout. Otherwise, split the cell for the current pane.
     */
    if full_size != 0 {
        lc = (*(*wp).window).layout_root
    } else {
        lc = (*wp).layout_cell
    }
    status = options_get_number(
        (*(*wp).window).options,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    /* Copy the old cell size. */
    sx = (*lc).sx;
    sy = (*lc).sy;
    xoff = (*lc).xoff;
    yoff = (*lc).yoff;
    /* Check there is enough space for the two new panes. */
    match type_0 {
        0 => {
            if sx < (1i32 * 2i32 + 1i32) as libc::c_uint {
                return 0 as *mut layout_cell;
            }
        }
        1 => {
            if layout_add_border((*wp).window, lc, status) != 0 {
                minimum = (1i32 * 2i32 + 2i32) as u_int
            } else {
                minimum = (1i32 * 2i32 + 1i32) as u_int
            }
            if sy < minimum {
                return 0 as *mut layout_cell;
            }
        }
        _ => {
            fatalx(b"bad layout type\x00" as *const u8 as *const libc::c_char);
        }
    }
    /*
     * Calculate new cell sizes. size is the target size or -1 for middle
     * split, size1 is the size of the top/left and size2 the bottom/right.
     */
    if type_0 == LAYOUT_LEFTRIGHT {
        saved_size = sx
    } else {
        saved_size = sy
    }
    if size < 0i32 {
        size2 = saved_size
            .wrapping_add(1u32)
            .wrapping_div(2u32)
            .wrapping_sub(1u32)
    } else if flags & 0x8i32 != 0 {
        size2 = saved_size
            .wrapping_sub(size as libc::c_uint)
            .wrapping_sub(1u32)
    } else {
        size2 = size as u_int
    }
    if size2 < 1u32 {
        size2 = 1u32
    } else if size2 > saved_size.wrapping_sub(2u32) {
        size2 = saved_size.wrapping_sub(2u32)
    }
    size1 = saved_size.wrapping_sub(1u32).wrapping_sub(size2);
    /* Which size are we using? */
    if flags & 0x8i32 != 0 {
        new_size = size2
    } else {
        new_size = size1
    }
    /* Confirm there is enough space for full size pane. */
    if full_size != 0
        && layout_set_size_check((*wp).window, lc, type_0, new_size as libc::c_int) == 0
    {
        return 0 as *mut layout_cell;
    }
    if !(*lc).parent.is_null() && (*(*lc).parent).type_0 == type_0 {
        /*
         * If the parent exists and is of the same type as the split,
         * create a new cell and insert it after this one.
         */
        lcparent = (*lc).parent;
        lcnew = layout_create_cell(lcparent);
        if flags & 0x8i32 != 0 {
            (*lcnew).entry.tqe_prev = (*lc).entry.tqe_prev;
            (*lcnew).entry.tqe_next = lc;
            *(*lc).entry.tqe_prev = lcnew;
            (*lc).entry.tqe_prev = &mut (*lcnew).entry.tqe_next
        } else {
            (*lcnew).entry.tqe_next = (*lc).entry.tqe_next;
            if !(*lcnew).entry.tqe_next.is_null() {
                (*(*lcnew).entry.tqe_next).entry.tqe_prev = &mut (*lcnew).entry.tqe_next
            } else {
                (*lcparent).cells.tqh_last = &mut (*lcnew).entry.tqe_next
            }
            (*lc).entry.tqe_next = lcnew;
            (*lcnew).entry.tqe_prev = &mut (*lc).entry.tqe_next
        }
    } else if full_size != 0 && (*lc).parent.is_null() && (*lc).type_0 == type_0 {
        /*
         * If the new full size pane is the same type as the root
         * split, insert the new pane under the existing root cell
         * instead of creating a new root cell. The existing layout
         * must be resized before inserting the new cell.
         */
        if (*lc).type_0 == LAYOUT_LEFTRIGHT {
            (*lc).sx = new_size;
            layout_resize_child_cells((*wp).window, lc);
            (*lc).sx = saved_size
        } else if (*lc).type_0 == LAYOUT_TOPBOTTOM {
            (*lc).sy = new_size;
            layout_resize_child_cells((*wp).window, lc);
            (*lc).sy = saved_size
        }
        resize_first = 1u32;
        /* Create the new cell. */
        lcnew = layout_create_cell(lc);
        size = saved_size.wrapping_sub(1u32).wrapping_sub(new_size) as libc::c_int;
        if (*lc).type_0 == LAYOUT_LEFTRIGHT {
            layout_set_size(lcnew, size as u_int, sy, 0u32, 0u32);
        } else if (*lc).type_0 == LAYOUT_TOPBOTTOM {
            layout_set_size(lcnew, sx, size as u_int, 0u32, 0u32);
        }
        if flags & 0x8i32 != 0 {
            (*lcnew).entry.tqe_next = (*lc).cells.tqh_first;
            if !(*lcnew).entry.tqe_next.is_null() {
                (*(*lc).cells.tqh_first).entry.tqe_prev = &mut (*lcnew).entry.tqe_next
            } else {
                (*lc).cells.tqh_last = &mut (*lcnew).entry.tqe_next
            }
            (*lc).cells.tqh_first = lcnew;
            (*lcnew).entry.tqe_prev = &mut (*lc).cells.tqh_first
        } else {
            (*lcnew).entry.tqe_next = 0 as *mut layout_cell;
            (*lcnew).entry.tqe_prev = (*lc).cells.tqh_last;
            *(*lc).cells.tqh_last = lcnew;
            (*lc).cells.tqh_last = &mut (*lcnew).entry.tqe_next
        }
    } else {
        /*
         * Otherwise create a new parent and insert it.
         */
        /* Create and insert the replacement parent. */
        lcparent = layout_create_cell((*lc).parent);
        layout_make_node(lcparent, type_0);
        layout_set_size(lcparent, sx, sy, xoff, yoff);
        if (*lc).parent.is_null() {
            (*(*wp).window).layout_root = lcparent
        } else {
            (*lcparent).entry.tqe_next = (*lc).entry.tqe_next;
            if !(*lcparent).entry.tqe_next.is_null() {
                (*(*lcparent).entry.tqe_next).entry.tqe_prev = &mut (*lcparent).entry.tqe_next
            } else {
                (*(*lc).parent).cells.tqh_last = &mut (*lcparent).entry.tqe_next
            }
            (*lcparent).entry.tqe_prev = (*lc).entry.tqe_prev;
            *(*lcparent).entry.tqe_prev = lcparent
        }
        /* Insert the old cell. */
        (*lc).parent = lcparent;
        (*lc).entry.tqe_next = (*lcparent).cells.tqh_first;
        if !(*lc).entry.tqe_next.is_null() {
            (*(*lcparent).cells.tqh_first).entry.tqe_prev = &mut (*lc).entry.tqe_next
        } else {
            (*lcparent).cells.tqh_last = &mut (*lc).entry.tqe_next
        }
        (*lcparent).cells.tqh_first = lc;
        (*lc).entry.tqe_prev = &mut (*lcparent).cells.tqh_first;
        /* Create the new child cell. */
        lcnew = layout_create_cell(lcparent);
        if flags & 0x8i32 != 0 {
            (*lcnew).entry.tqe_next = (*lcparent).cells.tqh_first;
            if !(*lcnew).entry.tqe_next.is_null() {
                (*(*lcparent).cells.tqh_first).entry.tqe_prev = &mut (*lcnew).entry.tqe_next
            } else {
                (*lcparent).cells.tqh_last = &mut (*lcnew).entry.tqe_next
            }
            (*lcparent).cells.tqh_first = lcnew;
            (*lcnew).entry.tqe_prev = &mut (*lcparent).cells.tqh_first
        } else {
            (*lcnew).entry.tqe_next = 0 as *mut layout_cell;
            (*lcnew).entry.tqe_prev = (*lcparent).cells.tqh_last;
            *(*lcparent).cells.tqh_last = lcnew;
            (*lcparent).cells.tqh_last = &mut (*lcnew).entry.tqe_next
        }
    }
    if flags & 0x8i32 != 0 {
        lc1 = lcnew;
        lc2 = lc
    } else {
        lc1 = lc;
        lc2 = lcnew
    }
    /*
     * Set new cell sizes. size1 is the size of the top/left and size2 the
     * bottom/right.
     */
    if resize_first == 0 && type_0 == LAYOUT_LEFTRIGHT {
        layout_set_size(lc1, size1, sy, xoff, yoff);
        layout_set_size(
            lc2,
            size2,
            sy,
            xoff.wrapping_add((*lc1).sx).wrapping_add(1u32),
            yoff,
        );
    } else if resize_first == 0 && type_0 == LAYOUT_TOPBOTTOM {
        layout_set_size(lc1, sx, size1, xoff, yoff);
        layout_set_size(
            lc2,
            sx,
            size2,
            xoff,
            yoff.wrapping_add((*lc1).sy).wrapping_add(1u32),
        );
    }
    if full_size != 0 {
        if resize_first == 0 {
            layout_resize_child_cells((*wp).window, lc);
        }
        layout_fix_offsets((*wp).window);
    } else {
        layout_make_leaf(lc, wp);
    }
    return lcnew;
}
/* Destroy the cell associated with a pane. */
#[no_mangle]
pub unsafe extern "C" fn layout_close_pane(mut wp: *mut window_pane) {
    let mut w: *mut window = (*wp).window;
    /* Remove the cell. */
    layout_destroy_cell(w, (*wp).layout_cell, &mut (*w).layout_root);
    /* Fix pane offsets and sizes. */
    if !(*w).layout_root.is_null() {
        layout_fix_offsets(w);
        layout_fix_panes(w);
    }
    notify_window(
        b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
        w,
    );
}
#[no_mangle]
pub unsafe extern "C" fn layout_spread_cell(
    mut w: *mut window,
    mut parent: *mut layout_cell,
) -> libc::c_int {
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut number: u_int = 0;
    let mut each: u_int = 0;
    let mut size: u_int = 0;
    let mut this: u_int = 0;
    let mut change: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    number = 0u32;
    lc = (*parent).cells.tqh_first;
    while !lc.is_null() {
        number = number.wrapping_add(1);
        lc = (*lc).entry.tqe_next
    }
    if number <= 1u32 {
        return 0i32;
    }
    status = options_get_number(
        (*w).options,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if (*parent).type_0 == LAYOUT_LEFTRIGHT {
        size = (*parent).sx
    } else if (*parent).type_0 == LAYOUT_TOPBOTTOM {
        if layout_add_border(w, parent, status) != 0 {
            size = (*parent).sy.wrapping_sub(1u32)
        } else {
            size = (*parent).sy
        }
    } else {
        return 0i32;
    }
    if size < number.wrapping_sub(1u32) {
        return 0i32;
    }
    each = size
        .wrapping_sub(number.wrapping_sub(1u32))
        .wrapping_div(number);
    if each == 0u32 {
        return 0i32;
    }
    changed = 0i32;
    lc = (*parent).cells.tqh_first;
    while !lc.is_null() {
        if (*lc).entry.tqe_next.is_null() {
            each = size.wrapping_sub(
                each.wrapping_add(1u32)
                    .wrapping_mul(number.wrapping_sub(1u32)),
            )
        }
        change = 0i32;
        if (*parent).type_0 == LAYOUT_LEFTRIGHT {
            change = each.wrapping_sub((*lc).sx) as libc::c_int;
            layout_resize_adjust(w, lc, LAYOUT_LEFTRIGHT, change);
        } else if (*parent).type_0 == LAYOUT_TOPBOTTOM {
            if layout_add_border(w, lc, status) != 0 {
                this = each.wrapping_add(1u32)
            } else {
                this = each
            }
            change = this.wrapping_sub((*lc).sy) as libc::c_int;
            layout_resize_adjust(w, lc, LAYOUT_TOPBOTTOM, change);
        }
        if change != 0i32 {
            changed = 1i32
        }
        lc = (*lc).entry.tqe_next
    }
    return changed;
}
#[no_mangle]
pub unsafe extern "C" fn layout_spread_out(mut wp: *mut window_pane) {
    let mut parent: *mut layout_cell = 0 as *mut layout_cell;
    let mut w: *mut window = (*wp).window;
    parent = (*(*wp).layout_cell).parent;
    if parent.is_null() {
        return;
    }
    loop {
        if layout_spread_cell(w, parent) != 0 {
            layout_fix_offsets(w);
            layout_fix_panes(w);
            break;
        } else {
            parent = (*parent).parent;
            if parent.is_null() {
                break;
            }
        }
    }
}
