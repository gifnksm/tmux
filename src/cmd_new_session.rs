use crate::msg::code as msgtype_code;
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    static mut global_s_options: *mut crate::options::options;
    #[no_mangle]
    fn proc_send(
        _: *mut crate::proc::tmuxpeer,
        _: crate::msg::Msgtype,
        _: libc::c_int,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn cfg_show_causes(_: *mut session);
    #[no_mangle]
    fn format_single(
        _: *mut crate::cmd_queue::cmdq_item,
        _: *const libc::c_char,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn notify_client(_: *const libc::c_char, _: *mut client);
    #[no_mangle]
    fn notify_session(_: *const libc::c_char, _: *mut session);
    #[no_mangle]
    fn options_create(_: *mut crate::options::options) -> *mut crate::options::options;
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
    fn options_set_string(
        _: *mut crate::options::options,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    ) -> *mut crate::options::options_entry;
    #[no_mangle]
    fn environ_create() -> *mut crate::environ::environ;
    #[no_mangle]
    fn environ_put(_: *mut crate::environ::environ, _: *const libc::c_char, _: libc::c_int);
    #[no_mangle]
    fn environ_update(
        _: *mut crate::options::options,
        _: *mut crate::environ::environ,
        _: *mut crate::environ::environ,
    );
    #[no_mangle]
    fn tty_update_client_offset(_: *mut client);
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn args_first_value(
        _: *mut args,
        _: u_char,
        _: *mut *mut crate::arguments::args_value,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn args_next_value(_: *mut *mut crate::arguments::args_value) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_find_from_session(_: *mut cmd_find_state, _: *mut session, _: libc::c_int);
    #[no_mangle]
    fn cmd_get_entry(_: *mut crate::cmd::cmd) -> *const cmd_entry;
    #[no_mangle]
    fn cmd_get_args(_: *mut crate::cmd::cmd) -> *mut args;
    #[no_mangle]
    fn cmd_attach_session(
        _: *mut crate::cmd_queue::cmdq_item,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> cmd_retval;
    #[no_mangle]
    fn cmdq_get_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
    #[no_mangle]
    fn cmdq_get_target(_: *mut crate::cmd_queue::cmdq_item) -> *mut cmd_find_state;
    #[no_mangle]
    fn cmdq_get_current(_: *mut crate::cmd_queue::cmdq_item) -> *mut cmd_find_state;
    #[no_mangle]
    fn cmdq_get_flags(_: *mut crate::cmd_queue::cmdq_item) -> libc::c_int;
    #[no_mangle]
    fn cmdq_insert_hook(
        _: *mut session,
        _: *mut crate::cmd_queue::cmdq_item,
        _: *mut cmd_find_state,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn cmdq_print(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn cmdq_error(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn server_update_socket();
    #[no_mangle]
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char);
    #[no_mangle]
    fn server_client_check_nested(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn server_client_open(_: *mut client, _: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn server_client_get_cwd(_: *mut client, _: *mut session) -> *const libc::c_char;
    #[no_mangle]
    fn server_client_set_flags(_: *mut client, _: *const libc::c_char);
    #[no_mangle]
    fn status_timer_start(_: *mut client);
    #[no_mangle]
    fn recalculate_sizes();
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn server_redraw_client(_: *mut client);
    #[no_mangle]
    fn session_find(_: *const libc::c_char) -> *mut session;
    #[no_mangle]
    fn session_create(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *mut crate::environ::environ,
        _: *mut crate::options::options,
        _: *mut termios,
    ) -> *mut session;
    #[no_mangle]
    fn session_destroy(_: *mut session, _: libc::c_int, _: *const libc::c_char);
    #[no_mangle]
    fn session_check_name(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn session_update_activity(_: *mut session, _: *mut timeval);
    #[no_mangle]
    fn session_select(_: *mut session, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn session_group_contains(_: *mut session) -> *mut session_group;
    #[no_mangle]
    fn session_group_find(_: *const libc::c_char) -> *mut session_group;
    #[no_mangle]
    fn session_group_new(_: *const libc::c_char) -> *mut session_group;
    #[no_mangle]
    fn session_group_add(_: *mut session_group, _: *mut session);
    #[no_mangle]
    fn session_group_synchronize_to(_: *mut session);
    #[no_mangle]
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn spawn_window(_: *mut spawn_context, _: *mut *mut libc::c_char) -> *mut winlink;
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
    pub saved_cell: grid_cell,
    pub saved_flags: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: *mut crate::screen::screen_sel,
    pub write_list: *mut crate::screen_write::screen_write_collect_line,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grid_cell {
    pub data: crate::utf8::Utf8Data,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
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
    pub args: C2RustUnnamed_34,
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
pub struct C2RustUnnamed_34 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spawn_context {
    pub item: *mut crate::cmd_queue::cmdq_item,
    pub s: *mut session,
    pub wl: *mut winlink,
    pub tc: *mut client,
    pub wp0: *mut window_pane,
    pub lc: *mut layout_cell,
    pub name: *const libc::c_char,
    pub argv: *mut *mut libc::c_char,
    pub argc: libc::c_int,
    pub environ: *mut crate::environ::environ,
    pub idx: libc::c_int,
    pub cwd: *const libc::c_char,
    pub flags: libc::c_int,
}
#[no_mangle]
pub static mut cmd_new_session_entry: cmd_entry = {
    {
        let mut init =
                cmd_entry{name:
                              b"new-session\x00" as *const u8 as
                                  *const libc::c_char,
                          alias:
                              b"new\x00" as *const u8 as *const libc::c_char,
                          args:
                              {
                                  let mut init =
                                      C2RustUnnamed_34{template:
                                                           b"Ac:dDe:EF:f:n:Ps:t:x:Xy:\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                       lower:
                                                           0 as libc::c_int,
                                                       upper:
                                                           -(1 as
                                                                 libc::c_int),};
                                  init
                              },
                          usage:
                              b"[-AdDEPX] [-c start-directory] [-e environment] [-F format] [-f flags] [-n window-name] [-s session-name] [-t target-session] [-x width] [-y height] [command]\x00"
                                  as *const u8 as *const libc::c_char,
                          source:
                              cmd_entry_flag{flag: 0,
                                             type_0: CMD_FIND_PANE,
                                             flags: 0,},
                          target:
                              {
                                  let mut init =
                                      cmd_entry_flag{flag:
                                                         't' as i32 as
                                                             libc::c_char,
                                                     type_0: CMD_FIND_SESSION,
                                                     flags:
                                                         0x40 as
                                                             libc::c_int,};
                                  init
                              },
                          flags: 0x1 as libc::c_int,
                          exec:
                              Some(cmd_new_session_exec as
                                       unsafe extern "C" fn(_: *mut crate::cmd::cmd,
                                                            _: *mut crate::cmd_queue::cmdq_item)
                                           -> cmd_retval),};
        init
    }
};
#[no_mangle]
pub static mut cmd_has_session_entry: cmd_entry = {
    {
        let mut init = cmd_entry {
            name: b"has-session\x00" as *const u8 as *const libc::c_char,
            alias: b"has\x00" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed_34 {
                    template: b"t:\x00" as *const u8 as *const libc::c_char,
                    lower: 0 as libc::c_int,
                    upper: 0 as libc::c_int,
                };
                init
            },
            usage: b"[-t target-session]\x00" as *const u8 as *const libc::c_char,
            source: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            target: {
                let mut init = cmd_entry_flag {
                    flag: 't' as i32 as libc::c_char,
                    type_0: CMD_FIND_SESSION,
                    flags: 0 as libc::c_int,
                };
                init
            },
            flags: 0 as libc::c_int,
            exec: Some(
                cmd_new_session_exec
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd::cmd,
                        _: *mut crate::cmd_queue::cmdq_item,
                    ) -> cmd_retval,
            ),
        };
        init
    }
};
unsafe extern "C" fn cmd_new_session_exec(
    mut self_0: *mut crate::cmd::cmd,
    mut item: *mut crate::cmd_queue::cmdq_item,
) -> cmd_retval {
    let mut current_block: u64;
    let mut args: *mut args = cmd_get_args(self_0);
    let mut current: *mut cmd_find_state = cmdq_get_current(item);
    let mut target: *mut cmd_find_state = cmdq_get_target(item);
    let mut c: *mut client = cmdq_get_client(item);
    let mut s: *mut session = 0 as *mut session;
    let mut as_0: *mut session = 0 as *mut session;
    let mut groupwith: *mut session = 0 as *mut session;
    let mut env: *mut crate::environ::environ = 0 as *mut crate::environ::environ;
    let mut oo: *mut crate::options::options = 0 as *mut crate::options::options;
    let mut tio: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut tiop: *mut termios = 0 as *mut termios;
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut template: *const libc::c_char = 0 as *const libc::c_char;
    let mut group: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut add: *const libc::c_char = 0 as *const libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut detached: libc::c_int = 0;
    let mut already_attached: libc::c_int = 0;
    let mut is_control: libc::c_int = 0 as libc::c_int;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut dsx: u_int = 0;
    let mut dsy: u_int = 0;
    let mut sc: spawn_context = spawn_context {
        item: 0 as *mut crate::cmd_queue::cmdq_item,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        tc: 0 as *mut client,
        wp0: 0 as *mut window_pane,
        lc: 0 as *mut layout_cell,
        name: 0 as *const libc::c_char,
        argv: 0 as *mut *mut libc::c_char,
        argc: 0,
        environ: 0 as *mut crate::environ::environ,
        idx: 0,
        cwd: 0 as *const libc::c_char,
        flags: 0,
    };
    let mut retval: cmd_retval = CMD_RETURN_NORMAL;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    let mut value: *mut crate::arguments::args_value = 0 as *mut crate::arguments::args_value;
    if cmd_get_entry(self_0) == &cmd_has_session_entry as *const cmd_entry {
        /*
         * cmd_find_target() will fail if the session cannot be found,
         * so always return success here.
         */
        return CMD_RETURN_NORMAL;
    }
    if args_has(args, 't' as i32 as u_char) != 0
        && ((*args).argc != 0 as libc::c_int || args_has(args, 'n' as i32 as u_char) != 0)
    {
        cmdq_error(
            item,
            b"command or window name given with target\x00" as *const u8 as *const libc::c_char,
        );
        return CMD_RETURN_ERROR;
    }
    tmp = args_get(args, 's' as i32 as u_char);
    if !tmp.is_null() {
        name = format_single(
            item,
            tmp,
            c,
            0 as *mut session,
            0 as *mut winlink,
            0 as *mut window_pane,
        );
        newname = session_check_name(name);
        free(name as *mut libc::c_void);
    }
    if args_has(args, 'A' as i32 as u_char) != 0 {
        if !newname.is_null() {
            as_0 = session_find(newname)
        } else {
            as_0 = (*target).s
        }
        if !as_0.is_null() {
            retval = cmd_attach_session(
                item,
                (*as_0).name,
                args_has(args, 'D' as i32 as u_char),
                args_has(args, 'X' as i32 as u_char),
                0 as libc::c_int,
                0 as *const libc::c_char,
                args_has(args, 'E' as i32 as u_char),
                args_get(args, 'f' as i32 as u_char),
            );
            free(newname as *mut libc::c_void);
            return retval;
        }
    }
    if !newname.is_null() && !session_find(newname).is_null() {
        cmdq_error(
            item,
            b"duplicate session: %s\x00" as *const u8 as *const libc::c_char,
            newname,
        );
    } else {
        /* Is this going to be part of a session group? */
        group = args_get(args, 't' as i32 as u_char);
        if !group.is_null() {
            groupwith = (*target).s;
            if groupwith.is_null() {
                sg = session_group_find(group)
            } else {
                sg = session_group_contains(groupwith)
            }
            if !sg.is_null() {
                prefix = xstrdup((*sg).name)
            } else if !groupwith.is_null() {
                prefix = xstrdup((*groupwith).name)
            } else {
                prefix = session_check_name(group)
            }
        }
        /* Set -d if no client. */
        detached = args_has(args, 'd' as i32 as u_char);
        if c.is_null() {
            detached = 1 as libc::c_int
        } else if (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
            is_control = 1 as libc::c_int
        }
        /* Is this client already attached? */
        already_attached = 0 as libc::c_int;
        if !c.is_null() && !(*c).session.is_null() {
            already_attached = 1 as libc::c_int
        }
        /* Get the new session working directory. */
        tmp = args_get(args, 'c' as i32 as u_char);
        if !tmp.is_null() {
            cwd = format_single(
                item,
                tmp,
                c,
                0 as *mut session,
                0 as *mut winlink,
                0 as *mut window_pane,
            )
        } else {
            cwd = xstrdup(server_client_get_cwd(c, 0 as *mut session))
        }
        /*
         * If this is a new client, check for nesting and save the termios
         * settings (part of which is used for new windows in this session).
         *
         * tcgetattr() is used rather than using tty.tio since if the client is
         * detached, tty_open won't be called. It must be done before opening
         * the terminal as that calls tcsetattr() to prepare for tmux taking
         * over.
         */
        if detached == 0
            && already_attached == 0
            && (*c).fd != -(1 as libc::c_int)
            && !(*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0
        {
            if server_client_check_nested(cmdq_get_client(item)) != 0 {
                cmdq_error(
                    item,
                    b"sessions should be nested with care, unset $TMUX to force\x00" as *const u8
                        as *const libc::c_char,
                );
                current_block = 16572900422509204778;
            } else {
                if tcgetattr((*c).fd, &mut tio) != 0 as libc::c_int {
                    fatal(b"tcgetattr failed\x00" as *const u8 as *const libc::c_char);
                }
                tiop = &mut tio;
                current_block = 13303144130133872306;
            }
        } else {
            tiop = 0 as *mut termios;
            current_block = 13303144130133872306;
        }
        match current_block {
            16572900422509204778 => {}
            _ =>
            /* Open the terminal if necessary. */
            {
                if detached == 0 && already_attached == 0 {
                    if server_client_open(c, &mut cause) != 0 as libc::c_int {
                        cmdq_error(
                            item,
                            b"open terminal failed: %s\x00" as *const u8 as *const libc::c_char,
                            cause,
                        );
                        free(cause as *mut libc::c_void);
                        current_block = 16572900422509204778;
                    } else {
                        current_block = 307447392441238883;
                    }
                } else {
                    current_block = 307447392441238883;
                }
                match current_block {
                    16572900422509204778 => {}
                    _ =>
                    /* Get default session size. */
                    {
                        if args_has(args, 'x' as i32 as u_char) != 0 {
                            tmp = args_get(args, 'x' as i32 as u_char);
                            if strcmp(tmp, b"-\x00" as *const u8 as *const libc::c_char)
                                == 0 as libc::c_int
                            {
                                if !c.is_null() {
                                    dsx = (*c).tty.sx
                                } else {
                                    dsx = 80 as libc::c_int as u_int
                                }
                                current_block = 11052029508375673978;
                            } else {
                                dsx = strtonum(
                                    tmp,
                                    1 as libc::c_int as libc::c_longlong,
                                    (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                        as libc::c_longlong,
                                    &mut errstr,
                                ) as u_int;
                                if !errstr.is_null() {
                                    cmdq_error(
                                        item,
                                        b"width %s\x00" as *const u8 as *const libc::c_char,
                                        errstr,
                                    );
                                    current_block = 16572900422509204778;
                                } else {
                                    current_block = 11052029508375673978;
                                }
                            }
                        } else {
                            dsx = 80 as libc::c_int as u_int;
                            current_block = 11052029508375673978;
                        }
                        match current_block {
                            16572900422509204778 => {}
                            _ => {
                                if args_has(args, 'y' as i32 as u_char) != 0 {
                                    tmp = args_get(args, 'y' as i32 as u_char);
                                    if strcmp(tmp, b"-\x00" as *const u8 as *const libc::c_char)
                                        == 0 as libc::c_int
                                    {
                                        if !c.is_null() {
                                            dsy = (*c).tty.sy
                                        } else {
                                            dsy = 24 as libc::c_int as u_int
                                        }
                                        current_block = 1852451392920375136;
                                    } else {
                                        dsy = strtonum(
                                            tmp,
                                            1 as libc::c_int as libc::c_longlong,
                                            (32767 as libc::c_int * 2 as libc::c_int
                                                + 1 as libc::c_int)
                                                as libc::c_longlong,
                                            &mut errstr,
                                        ) as u_int;
                                        if !errstr.is_null() {
                                            cmdq_error(
                                                item,
                                                b"height %s\x00" as *const u8
                                                    as *const libc::c_char,
                                                errstr,
                                            );
                                            current_block = 16572900422509204778;
                                        } else {
                                            current_block = 1852451392920375136;
                                        }
                                    }
                                } else {
                                    dsy = 24 as libc::c_int as u_int;
                                    current_block = 1852451392920375136;
                                }
                                match current_block {
                                    16572900422509204778 => {}
                                    _ => {
                                        /* Find new session size. */
                                        if detached == 0 && is_control == 0 {
                                            sx = (*c).tty.sx;
                                            sy = (*c).tty.sy;
                                            if sy > 0 as libc::c_int as libc::c_uint
                                                && options_get_number(
                                                    global_s_options,
                                                    b"status\x00" as *const u8
                                                        as *const libc::c_char,
                                                ) != 0
                                            {
                                                sy = sy.wrapping_sub(1)
                                            }
                                        } else {
                                            tmp = options_get_string(
                                                global_s_options,
                                                b"default-size\x00" as *const u8
                                                    as *const libc::c_char,
                                            );
                                            if sscanf(
                                                tmp,
                                                b"%ux%u\x00" as *const u8 as *const libc::c_char,
                                                &mut sx as *mut u_int,
                                                &mut sy as *mut u_int,
                                            ) != 2 as libc::c_int
                                            {
                                                sx = dsx;
                                                sy = dsy
                                            } else {
                                                if args_has(args, 'x' as i32 as u_char) != 0 {
                                                    sx = dsx
                                                }
                                                if args_has(args, 'y' as i32 as u_char) != 0 {
                                                    sy = dsy
                                                }
                                            }
                                        }
                                        if sx == 0 as libc::c_int as libc::c_uint {
                                            sx = 1 as libc::c_int as u_int
                                        }
                                        if sy == 0 as libc::c_int as libc::c_uint {
                                            sy = 1 as libc::c_int as u_int
                                        }
                                        /* Create the new session. */
                                        oo = options_create(global_s_options);
                                        if args_has(args, 'x' as i32 as u_char) != 0
                                            || args_has(args, 'y' as i32 as u_char) != 0
                                        {
                                            if args_has(args, 'x' as i32 as u_char) == 0 {
                                                dsx = sx
                                            }
                                            if args_has(args, 'y' as i32 as u_char) == 0 {
                                                dsy = sy
                                            }
                                            options_set_string(
                                                oo,
                                                b"default-size\x00" as *const u8
                                                    as *const libc::c_char,
                                                0 as libc::c_int,
                                                b"%ux%u\x00" as *const u8 as *const libc::c_char,
                                                dsx,
                                                dsy,
                                            );
                                        }
                                        env = environ_create();
                                        if !c.is_null() && args_has(args, 'E' as i32 as u_char) == 0
                                        {
                                            environ_update(global_s_options, (*c).environ, env);
                                        }
                                        add = args_first_value(
                                            args,
                                            'e' as i32 as u_char,
                                            &mut value,
                                        );
                                        while !add.is_null() {
                                            environ_put(env, add, 0 as libc::c_int);
                                            add = args_next_value(&mut value)
                                        }
                                        s = session_create(prefix, newname, cwd, env, oo, tiop);
                                        /* Spawn the initial window. */
                                        memset(
                                            &mut sc as *mut spawn_context as *mut libc::c_void,
                                            0 as libc::c_int,
                                            ::std::mem::size_of::<spawn_context>() as libc::c_ulong,
                                        );
                                        sc.item = item;
                                        sc.s = s;
                                        sc.name = args_get(args, 'n' as i32 as u_char);
                                        sc.argc = (*args).argc;
                                        sc.argv = (*args).argv;
                                        sc.idx = -(1 as libc::c_int);
                                        sc.cwd = args_get(args, 'c' as i32 as u_char);
                                        sc.flags = 0 as libc::c_int;
                                        if spawn_window(&mut sc, &mut cause).is_null() {
                                            session_destroy(
                                                s,
                                                0 as libc::c_int,
                                                (*::std::mem::transmute::<
                                                    &[u8; 21],
                                                    &[libc::c_char; 21],
                                                >(
                                                    b"cmd_new_session_exec\x00"
                                                ))
                                                .as_ptr(),
                                            );
                                            cmdq_error(
                                                item,
                                                b"create window failed: %s\x00" as *const u8
                                                    as *const libc::c_char,
                                                cause,
                                            );
                                            free(cause as *mut libc::c_void);
                                        } else {
                                            /*
                                             * If a target session is given, this is to be part of a session group,
                                             * so add it to the group and synchronize.
                                             */
                                            if !group.is_null() {
                                                if sg.is_null() {
                                                    if !groupwith.is_null() {
                                                        sg = session_group_new((*groupwith).name);
                                                        session_group_add(sg, groupwith);
                                                    } else {
                                                        sg = session_group_new(group)
                                                    }
                                                }
                                                session_group_add(sg, s);
                                                session_group_synchronize_to(s);
                                                session_select(
                                                    s,
                                                    (*winlinks_RB_MINMAX(
                                                        &mut (*s).windows,
                                                        -(1 as libc::c_int),
                                                    ))
                                                    .idx,
                                                );
                                            }
                                            notify_session(
                                                b"session-created\x00" as *const u8
                                                    as *const libc::c_char,
                                                s,
                                            );
                                            /*
                                             * Set the client to the new session. If a command client exists, it is
                                             * taking this session and needs to get msgtype_code::READY and stay around.
                                             */
                                            if detached == 0 {
                                                if args_has(args, 'f' as i32 as u_char) != 0 {
                                                    server_client_set_flags(
                                                        c,
                                                        args_get(args, 'f' as i32 as u_char),
                                                    );
                                                }
                                                if already_attached == 0 {
                                                    if !(*c).flags
                                                        & 0x2000 as libc::c_int as libc::c_ulong
                                                        != 0
                                                    {
                                                        proc_send(
                                                            (*c).peer,
                                                            msgtype_code::READY,
                                                            -(1 as libc::c_int),
                                                            0 as *const libc::c_void,
                                                            0 as libc::c_int as size_t,
                                                        );
                                                    }
                                                } else if !(*c).session.is_null() {
                                                    (*c).last_session = (*c).session
                                                }
                                                (*c).session = s;
                                                if !cmdq_get_flags(item) & 0x1 as libc::c_int != 0 {
                                                    server_client_set_key_table(
                                                        c,
                                                        0 as *const libc::c_char,
                                                    );
                                                }
                                                tty_update_client_offset(c);
                                                status_timer_start(c);
                                                notify_client(
                                                    b"client-session-changed\x00" as *const u8
                                                        as *const libc::c_char,
                                                    c,
                                                );
                                                session_update_activity(s, 0 as *mut timeval);
                                                gettimeofday(
                                                    &mut (*s).last_attached_time,
                                                    0 as *mut libc::c_void,
                                                );
                                                server_redraw_client(c);
                                            }
                                            recalculate_sizes();
                                            server_update_socket();
                                            /*
                                             * If there are still configuration file errors to display, put the new
                                             * session's current window into more mode and display them now.
                                             */
                                            if cfg_finished != 0 {
                                                cfg_show_causes(s);
                                            }
                                            /* Print if requested. */
                                            if args_has(args, 'P' as i32 as u_char) != 0 {
                                                template = args_get(args, 'F' as i32 as u_char);
                                                if template.is_null() {
                                                    template = b"#{session_name}:\x00" as *const u8
                                                        as *const libc::c_char
                                                }
                                                cp = format_single(
                                                    item,
                                                    template,
                                                    c,
                                                    s,
                                                    (*s).curw,
                                                    0 as *mut window_pane,
                                                );
                                                cmdq_print(
                                                    item,
                                                    b"%s\x00" as *const u8 as *const libc::c_char,
                                                    cp,
                                                );
                                                free(cp as *mut libc::c_void);
                                            }
                                            if detached == 0 {
                                                (*c).flags |= 0x80 as libc::c_int as libc::c_ulong
                                            }
                                            if args_has(args, 'd' as i32 as u_char) == 0 {
                                                cmd_find_from_session(current, s, 0 as libc::c_int);
                                            }
                                            cmd_find_from_session(&mut fs, s, 0 as libc::c_int);
                                            cmdq_insert_hook(
                                                s,
                                                item,
                                                &mut fs as *mut cmd_find_state,
                                                b"after-new-session\x00" as *const u8
                                                    as *const libc::c_char,
                                            );
                                            free(cwd as *mut libc::c_void);
                                            free(newname as *mut libc::c_void);
                                            free(prefix as *mut libc::c_void);
                                            return CMD_RETURN_NORMAL;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(cwd as *mut libc::c_void);
    free(newname as *mut libc::c_void);
    free(prefix as *mut libc::c_void);
    return CMD_RETURN_ERROR;
}
