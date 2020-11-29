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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn cmd_get_args(_: *mut crate::cmd::cmd) -> *mut args;
    #[no_mangle]
    fn cmdq_get_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
    #[no_mangle]
    fn cmdq_continue(_: *mut crate::cmd_queue::cmdq_item);
    #[no_mangle]
    fn cmdq_error(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: ...);
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
pub type cmd_find_type = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type cmd_retval = libc::c_int;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: C2RustUnnamed_32,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<
        unsafe extern "C" fn(
            _: *mut crate::cmd::cmd,
            _: *mut crate::cmd_queue::cmdq_item,
        ) -> cmd_retval,
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_channel {
    pub name: *const libc::c_char,
    pub locked: libc::c_int,
    pub woken: libc::c_int,
    pub waiters: C2RustUnnamed_36,
    pub lockers: C2RustUnnamed_34,
    pub entry: C2RustUnnamed_33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub rbe_left: *mut wait_channel,
    pub rbe_right: *mut wait_channel,
    pub rbe_parent: *mut wait_channel,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_34 {
    pub tqh_first: *mut wait_item,
    pub tqh_last: *mut *mut wait_item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_item {
    pub item: *mut crate::cmd_queue::cmdq_item,
    pub entry: C2RustUnnamed_35,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_35 {
    pub tqe_next: *mut wait_item,
    pub tqe_prev: *mut *mut wait_item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_36 {
    pub tqh_first: *mut wait_item,
    pub tqh_last: *mut *mut wait_item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_channels {
    pub rbh_root: *mut wait_channel,
}
#[no_mangle]
pub static mut cmd_wait_for_entry: cmd_entry = {
    {
        let mut init = cmd_entry {
            name: b"wait-for\x00" as *const u8 as *const libc::c_char,
            alias: b"wait\x00" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed_32 {
                    template: b"LSU\x00" as *const u8 as *const libc::c_char,
                    lower: 1i32,
                    upper: 1i32,
                };
                init
            },
            usage: b"[-L|-S|-U] channel\x00" as *const u8 as *const libc::c_char,
            source: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            target: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            flags: 0i32,
            exec: Some(
                cmd_wait_for_exec
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd::cmd,
                        _: *mut crate::cmd_queue::cmdq_item,
                    ) -> cmd_retval,
            ),
        };
        init
    }
};
static mut wait_channels: wait_channels = {
    let mut init = wait_channels {
        rbh_root: 0 as *mut wait_channel,
    };
    init
};
unsafe extern "C" fn wait_channels_RB_MINMAX(
    mut head: *mut wait_channels,
    mut val: libc::c_int,
) -> *mut wait_channel {
    let mut tmp: *mut wait_channel = (*head).rbh_root;
    let mut parent: *mut wait_channel = 0 as *mut wait_channel;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else {
            tmp = (*tmp).entry.rbe_right
        }
    }
    return parent;
}
unsafe extern "C" fn wait_channels_RB_FIND(
    mut head: *mut wait_channels,
    mut elm: *mut wait_channel,
) -> *mut wait_channel {
    let mut tmp: *mut wait_channel = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = wait_channel_cmp(elm, tmp);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut wait_channel;
}
unsafe extern "C" fn wait_channels_RB_REMOVE(
    mut head: *mut wait_channels,
    mut elm: *mut wait_channel,
) -> *mut wait_channel {
    let mut current_block: u64;
    let mut child: *mut wait_channel = 0 as *mut wait_channel;
    let mut parent: *mut wait_channel = 0 as *mut wait_channel;
    let mut old: *mut wait_channel = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut wait_channel = 0 as *mut wait_channel;
        elm = (*elm).entry.rbe_right;
        loop {
            left = (*elm).entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() {
            (*child).entry.rbe_parent = parent
        }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child
            } else {
                (*parent).entry.rbe_right = child
            }
        } else {
            (*head).rbh_root = child
        }
        if (*elm).entry.rbe_parent == old {
            parent = elm
        }
        (*elm).entry = (*old).entry;
        if !(*old).entry.rbe_parent.is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm
            } else {
                (*(*old).entry.rbe_parent).entry.rbe_right = elm
            }
        } else {
            (*head).rbh_root = elm
        }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !(*old).entry.rbe_right.is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 12861663777764394879;
    }
    match current_block {
        7226443171521532240 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() {
                (*child).entry.rbe_parent = parent
            }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child
                } else {
                    (*parent).entry.rbe_right = child
                }
            } else {
                (*head).rbh_root = child
            }
        }
        _ => {}
    }
    if color == 0i32 {
        wait_channels_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn wait_channels_RB_REMOVE_COLOR(
    mut head: *mut wait_channels,
    mut parent: *mut wait_channel,
    mut elm: *mut wait_channel,
) {
    let mut tmp: *mut wait_channel = 0 as *mut wait_channel;
    while (elm.is_null() || (*elm).entry.rbe_color == 0i32) && elm != (*head).rbh_root {
        if (*parent).entry.rbe_left == elm {
            tmp = (*parent).entry.rbe_right;
            if (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 1i32;
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
                tmp = (*parent).entry.rbe_right
            }
            if ((*tmp).entry.rbe_left.is_null() || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32)
            {
                (*tmp).entry.rbe_color = 1i32;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32
                {
                    let mut oleft: *mut wait_channel = 0 as *mut wait_channel;
                    oleft = (*tmp).entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).entry.rbe_color = 0i32
                    }
                    (*tmp).entry.rbe_color = 1i32;
                    oleft = (*tmp).entry.rbe_left;
                    (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                    if !(*tmp).entry.rbe_left.is_null() {
                        (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp
                    }
                    (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oleft).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft
                        }
                    } else {
                        (*head).rbh_root = oleft
                    }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    !(*oleft).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_right
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0i32;
                if !(*tmp).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_color = 0i32
                }
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
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).entry.rbe_left;
            if (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 1i32;
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
                tmp = (*parent).entry.rbe_left
            }
            if ((*tmp).entry.rbe_left.is_null() || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32)
            {
                (*tmp).entry.rbe_color = 1i32;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_left.is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32
                {
                    let mut oright: *mut wait_channel = 0 as *mut wait_channel;
                    oright = (*tmp).entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).entry.rbe_color = 0i32
                    }
                    (*tmp).entry.rbe_color = 1i32;
                    oright = (*tmp).entry.rbe_right;
                    (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                    if !(*tmp).entry.rbe_right.is_null() {
                        (*(*oright).entry.rbe_left).entry.rbe_parent = tmp
                    }
                    (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oright).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oright
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oright
                        }
                    } else {
                        (*head).rbh_root = oright
                    }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    !(*oright).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_left
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0i32;
                if !(*tmp).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_color = 0i32
                }
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
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).entry.rbe_color = 0i32
    };
}
unsafe extern "C" fn wait_channels_RB_INSERT(
    mut head: *mut wait_channels,
    mut elm: *mut wait_channel,
) -> *mut wait_channel {
    let mut tmp: *mut wait_channel = 0 as *mut wait_channel;
    let mut parent: *mut wait_channel = 0 as *mut wait_channel;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = wait_channel_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut wait_channel;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1i32;
    if !parent.is_null() {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else {
            (*parent).entry.rbe_right = elm
        }
    } else {
        (*head).rbh_root = elm
    }
    wait_channels_RB_INSERT_COLOR(head, elm);
    return 0 as *mut wait_channel;
}
unsafe extern "C" fn wait_channels_RB_INSERT_COLOR(
    mut head: *mut wait_channels,
    mut elm: *mut wait_channel,
) {
    let mut parent: *mut wait_channel = 0 as *mut wait_channel;
    let mut gparent: *mut wait_channel = 0 as *mut wait_channel;
    let mut tmp: *mut wait_channel = 0 as *mut wait_channel;
    loop {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1i32) {
            break;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
    (*(*head).rbh_root).entry.rbe_color = 0i32;
}
unsafe extern "C" fn wait_channels_RB_NEXT(mut elm: *mut wait_channel) -> *mut wait_channel {
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
unsafe extern "C" fn wait_channel_cmp(
    mut wc1: *mut wait_channel,
    mut wc2: *mut wait_channel,
) -> libc::c_int {
    return strcmp((*wc1).name, (*wc2).name);
}
unsafe extern "C" fn cmd_wait_for_add(mut name: *const libc::c_char) -> *mut wait_channel {
    let mut wc: *mut wait_channel = 0 as *mut wait_channel;
    wc = xmalloc(::std::mem::size_of::<wait_channel>() as libc::c_ulong) as *mut wait_channel;
    (*wc).name = xstrdup(name);
    (*wc).locked = 0i32;
    (*wc).woken = 0i32;
    (*wc).waiters.tqh_first = 0 as *mut wait_item;
    (*wc).waiters.tqh_last = &mut (*wc).waiters.tqh_first;
    (*wc).lockers.tqh_first = 0 as *mut wait_item;
    (*wc).lockers.tqh_last = &mut (*wc).lockers.tqh_first;
    wait_channels_RB_INSERT(&mut wait_channels, wc);
    log_debug(
        b"add wait channel %s\x00" as *const u8 as *const libc::c_char,
        (*wc).name,
    );
    return wc;
}
unsafe extern "C" fn cmd_wait_for_remove(mut wc: *mut wait_channel) {
    if (*wc).locked != 0 {
        return;
    }
    if !(*wc).waiters.tqh_first.is_null() || (*wc).woken == 0 {
        return;
    }
    log_debug(
        b"remove wait channel %s\x00" as *const u8 as *const libc::c_char,
        (*wc).name,
    );
    wait_channels_RB_REMOVE(&mut wait_channels, wc);
    free((*wc).name as *mut libc::c_void);
    free(wc as *mut libc::c_void);
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2013 Nicholas Marriott <nicholas.marriott@gmail.com>
 * Copyright (c) 2013 Thiago de Arruda <tpadilha84@gmail.com>
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
 * Block or wake a client on a named wait channel.
 */
unsafe extern "C" fn cmd_wait_for_exec(
    mut self_0: *mut crate::cmd::cmd,
    mut item: *mut crate::cmd_queue::cmdq_item,
) -> cmd_retval {
    let mut args: *mut args = cmd_get_args(self_0);
    let mut name: *const libc::c_char = *(*args).argv.offset(0isize);
    let mut wc: *mut wait_channel = 0 as *mut wait_channel;
    let mut wc0: wait_channel = wait_channel {
        name: 0 as *const libc::c_char,
        locked: 0,
        woken: 0,
        waiters: C2RustUnnamed_36 {
            tqh_first: 0 as *mut wait_item,
            tqh_last: 0 as *mut *mut wait_item,
        },
        lockers: C2RustUnnamed_34 {
            tqh_first: 0 as *mut wait_item,
            tqh_last: 0 as *mut *mut wait_item,
        },
        entry: C2RustUnnamed_33 {
            rbe_left: 0 as *mut wait_channel,
            rbe_right: 0 as *mut wait_channel,
            rbe_parent: 0 as *mut wait_channel,
            rbe_color: 0,
        },
    };
    wc0.name = name;
    wc = wait_channels_RB_FIND(&mut wait_channels, &mut wc0);
    if args_has(args, 'S' as u_char) != 0 {
        return cmd_wait_for_signal(item, name, wc);
    }
    if args_has(args, 'L' as u_char) != 0 {
        return cmd_wait_for_lock(item, name, wc);
    }
    if args_has(args, 'U' as u_char) != 0 {
        return cmd_wait_for_unlock(item, name, wc);
    }
    return cmd_wait_for_wait(item, name, wc);
}
unsafe extern "C" fn cmd_wait_for_signal(
    mut _item: *mut crate::cmd_queue::cmdq_item,
    mut name: *const libc::c_char,
    mut wc: *mut wait_channel,
) -> cmd_retval {
    let mut wi: *mut wait_item = 0 as *mut wait_item;
    let mut wi1: *mut wait_item = 0 as *mut wait_item;
    if wc.is_null() {
        wc = cmd_wait_for_add(name)
    }
    if (*wc).waiters.tqh_first.is_null() && (*wc).woken == 0 {
        log_debug(
            b"signal wait channel %s, no waiters\x00" as *const u8 as *const libc::c_char,
            (*wc).name,
        );
        (*wc).woken = 1i32;
        return CMD_RETURN_NORMAL;
    }
    log_debug(
        b"signal wait channel %s, with waiters\x00" as *const u8 as *const libc::c_char,
        (*wc).name,
    );
    wi = (*wc).waiters.tqh_first;
    while !wi.is_null() && {
        wi1 = (*wi).entry.tqe_next;
        (1i32) != 0
    } {
        cmdq_continue((*wi).item);
        if !(*wi).entry.tqe_next.is_null() {
            (*(*wi).entry.tqe_next).entry.tqe_prev = (*wi).entry.tqe_prev
        } else {
            (*wc).waiters.tqh_last = (*wi).entry.tqe_prev
        }
        *(*wi).entry.tqe_prev = (*wi).entry.tqe_next;
        free(wi as *mut libc::c_void);
        wi = wi1
    }
    cmd_wait_for_remove(wc);
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn cmd_wait_for_wait(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut name: *const libc::c_char,
    mut wc: *mut wait_channel,
) -> cmd_retval {
    let mut c: *mut client = cmdq_get_client(item);
    let mut wi: *mut wait_item = 0 as *mut wait_item;
    if c.is_null() {
        cmdq_error(
            item,
            b"not able to wait\x00" as *const u8 as *const libc::c_char,
        );
        return CMD_RETURN_ERROR;
    }
    if wc.is_null() {
        wc = cmd_wait_for_add(name)
    }
    if (*wc).woken != 0 {
        log_debug(
            b"wait channel %s already woken (%p)\x00" as *const u8 as *const libc::c_char,
            (*wc).name,
            c,
        );
        cmd_wait_for_remove(wc);
        return CMD_RETURN_NORMAL;
    }
    log_debug(
        b"wait channel %s not woken (%p)\x00" as *const u8 as *const libc::c_char,
        (*wc).name,
        c,
    );
    wi = xcalloc(1u64, ::std::mem::size_of::<wait_item>() as libc::c_ulong) as *mut wait_item;
    (*wi).item = item;
    (*wi).entry.tqe_next = 0 as *mut wait_item;
    (*wi).entry.tqe_prev = (*wc).waiters.tqh_last;
    *(*wc).waiters.tqh_last = wi;
    (*wc).waiters.tqh_last = &mut (*wi).entry.tqe_next;
    return CMD_RETURN_WAIT;
}
unsafe extern "C" fn cmd_wait_for_lock(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut name: *const libc::c_char,
    mut wc: *mut wait_channel,
) -> cmd_retval {
    let mut wi: *mut wait_item = 0 as *mut wait_item;
    if cmdq_get_client(item).is_null() {
        cmdq_error(
            item,
            b"not able to lock\x00" as *const u8 as *const libc::c_char,
        );
        return CMD_RETURN_ERROR;
    }
    if wc.is_null() {
        wc = cmd_wait_for_add(name)
    }
    if (*wc).locked != 0 {
        wi = xcalloc(1u64, ::std::mem::size_of::<wait_item>() as libc::c_ulong) as *mut wait_item;
        (*wi).item = item;
        (*wi).entry.tqe_next = 0 as *mut wait_item;
        (*wi).entry.tqe_prev = (*wc).lockers.tqh_last;
        *(*wc).lockers.tqh_last = wi;
        (*wc).lockers.tqh_last = &mut (*wi).entry.tqe_next;
        return CMD_RETURN_WAIT;
    }
    (*wc).locked = 1i32;
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn cmd_wait_for_unlock(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut name: *const libc::c_char,
    mut wc: *mut wait_channel,
) -> cmd_retval {
    let mut wi: *mut wait_item = 0 as *mut wait_item;
    if wc.is_null() || (*wc).locked == 0 {
        cmdq_error(
            item,
            b"channel %s not locked\x00" as *const u8 as *const libc::c_char,
            name,
        );
        return CMD_RETURN_ERROR;
    }
    wi = (*wc).lockers.tqh_first;
    if !wi.is_null() {
        cmdq_continue((*wi).item);
        if !(*wi).entry.tqe_next.is_null() {
            (*(*wi).entry.tqe_next).entry.tqe_prev = (*wi).entry.tqe_prev
        } else {
            (*wc).lockers.tqh_last = (*wi).entry.tqe_prev
        }
        *(*wi).entry.tqe_prev = (*wi).entry.tqe_next;
        free(wi as *mut libc::c_void);
    } else {
        (*wc).locked = 0i32;
        cmd_wait_for_remove(wc);
    }
    return CMD_RETURN_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_wait_for_flush() {
    let mut wc: *mut wait_channel = 0 as *mut wait_channel;
    let mut wc1: *mut wait_channel = 0 as *mut wait_channel;
    let mut wi: *mut wait_item = 0 as *mut wait_item;
    let mut wi1: *mut wait_item = 0 as *mut wait_item;
    wc = wait_channels_RB_MINMAX(&mut wait_channels, -(1i32));
    while !wc.is_null() && {
        wc1 = wait_channels_RB_NEXT(wc);
        (1i32) != 0
    } {
        wi = (*wc).waiters.tqh_first;
        while !wi.is_null() && {
            wi1 = (*wi).entry.tqe_next;
            (1i32) != 0
        } {
            cmdq_continue((*wi).item);
            if !(*wi).entry.tqe_next.is_null() {
                (*(*wi).entry.tqe_next).entry.tqe_prev = (*wi).entry.tqe_prev
            } else {
                (*wc).waiters.tqh_last = (*wi).entry.tqe_prev
            }
            *(*wi).entry.tqe_prev = (*wi).entry.tqe_next;
            free(wi as *mut libc::c_void);
            wi = wi1
        }
        (*wc).woken = 1i32;
        wi = (*wc).lockers.tqh_first;
        while !wi.is_null() && {
            wi1 = (*wi).entry.tqe_next;
            (1i32) != 0
        } {
            cmdq_continue((*wi).item);
            if !(*wi).entry.tqe_next.is_null() {
                (*(*wi).entry.tqe_next).entry.tqe_prev = (*wi).entry.tqe_prev
            } else {
                (*wc).lockers.tqh_last = (*wi).entry.tqe_prev
            }
            *(*wi).entry.tqe_prev = (*wi).entry.tqe_next;
            free(wi as *mut libc::c_void);
            wi = wi1
        }
        (*wc).locked = 0i32;
        cmd_wait_for_remove(wc);
        wc = wc1
    }
}
