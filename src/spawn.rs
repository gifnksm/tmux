use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn _exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn bufferevent_free(bufev: *mut bufferevent);
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    #[no_mangle]
    fn closefrom(_: libc::c_int);
    #[no_mangle]
    fn fdforkpty(
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_char,
        _: *mut termios,
        _: *mut winsize,
    ) -> pid_t;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn proc_clear_signals(_: *mut crate::proc::tmuxproc, _: libc::c_int);
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
    fn notify_session_window(_: *const libc::c_char, _: *mut session, _: *mut window);
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
    fn options_set_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
        _: libc::c_longlong,
    ) -> *mut crate::options::options_entry;
    #[no_mangle]
    fn environ_free(_: *mut crate::environ::environ);
    #[no_mangle]
    fn environ_copy(_: *mut crate::environ::environ, _: *mut crate::environ::environ);
    #[no_mangle]
    fn environ_find(_: *mut crate::environ::environ, _: *const libc::c_char) -> *mut environ_entry;
    #[no_mangle]
    fn environ_set(
        _: *mut crate::environ::environ,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn environ_push(_: *mut crate::environ::environ);
    #[no_mangle]
    fn environ_log(_: *mut crate::environ::environ, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn environ_for_session(_: *mut session, _: libc::c_int) -> *mut crate::environ::environ;
    #[no_mangle]
    fn cmd_log_argv(_: libc::c_int, _: *mut *mut libc::c_char, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn cmd_copy_argv(_: libc::c_int, _: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
    #[no_mangle]
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char);
    #[no_mangle]
    fn cmd_stringify_argv(_: libc::c_int, _: *mut *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn cmdq_get_name(_: *mut crate::cmd_queue::cmdq_item) -> *const libc::c_char;
    #[no_mangle]
    fn cmdq_get_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
    #[no_mangle]
    fn cmdq_get_target(_: *mut crate::cmd_queue::cmdq_item) -> *mut cmd_find_state;
    #[no_mangle]
    static mut server_proc: *mut crate::proc::tmuxproc;
    #[no_mangle]
    fn server_client_get_cwd(_: *mut client, _: *mut session) -> *const libc::c_char;
    #[no_mangle]
    fn server_client_remove_pane(_: *mut window_pane);
    #[no_mangle]
    fn default_window_size(
        _: *mut client,
        _: *mut session,
        _: *mut window,
        _: *mut u_int,
        _: *mut u_int,
        _: *mut u_int,
        _: *mut u_int,
        _: libc::c_int,
    );
    #[no_mangle]
    static mut global_options: *mut crate::options::options;
    #[no_mangle]
    static mut ptm_fd: libc::c_int;
    #[no_mangle]
    fn checkshell(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn find_home() -> *const libc::c_char;
    #[no_mangle]
    fn screen_reinit(_: *mut crate::screen::Screen);
    #[no_mangle]
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_add(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_set_window(_: *mut winlink, _: *mut window);
    #[no_mangle]
    fn winlink_remove(_: *mut winlinks, _: *mut winlink);
    #[no_mangle]
    fn winlink_stack_remove(_: *mut winlink_stack, _: *mut winlink);
    #[no_mangle]
    fn window_create(_: u_int, _: u_int, _: u_int, _: u_int) -> *mut window;
    #[no_mangle]
    fn window_pane_set_event(_: *mut window_pane);
    #[no_mangle]
    fn window_set_active_pane(_: *mut window, _: *mut window_pane, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn window_add_pane(
        _: *mut window,
        _: *mut window_pane,
        _: u_int,
        _: libc::c_int,
    ) -> *mut window_pane;
    #[no_mangle]
    fn window_remove_pane(_: *mut window, _: *mut window_pane);
    #[no_mangle]
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    #[no_mangle]
    fn window_destroy_panes(_: *mut window);
    #[no_mangle]
    fn window_pane_resize(_: *mut window_pane, _: u_int, _: u_int);
    #[no_mangle]
    fn window_pane_reset_mode_all(_: *mut window_pane);
    #[no_mangle]
    fn input_free(_: *mut crate::input::input_ctx);
    #[no_mangle]
    fn default_window_name(_: *mut window) -> *mut libc::c_char;
    #[no_mangle]
    fn session_select(_: *mut session, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn layout_init(_: *mut window, _: *mut window_pane);
    #[no_mangle]
    fn layout_free(_: *mut window);
    #[no_mangle]
    fn layout_assign_pane(_: *mut layout_cell, _: *mut window_pane);
    #[no_mangle]
    fn layout_close_pane(_: *mut window_pane);
    #[no_mangle]
    fn session_group_synchronize_from(_: *mut session);
    #[no_mangle]
    fn log_close();
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
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;

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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
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
pub type overlay_mode_cb = Option<
    unsafe extern "C" fn(
        _: *mut client,
        _: *mut u_int,
        _: *mut u_int,
    ) -> *mut crate::screen::Screen,
>;

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
    pub screen: *mut crate::screen::Screen,
    pub base: crate::screen::Screen,
    pub status_screen: crate::screen::Screen,
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
    pub screen: *mut crate::screen::Screen,
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
        ) -> *mut crate::screen::Screen,
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
    pub screen: crate::screen::Screen,
    pub active: *mut crate::screen::Screen,
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
/*
 * Set up the environment and create a new window and pane or a new pane.
 *
 * We need to set up the following items:
 *
 * - history limit, comes from the session;
 *
 * - base index, comes from the session;
 *
 * - current working directory, may be specified - if it isn't it comes from
 *   either the client or the session;
 *
 * - PATH variable, comes from the client if any, otherwise from the session
 *   environment;
 *
 * - shell, comes from default-shell;
 *
 * - termios, comes from the session;
 *
 * - remaining environment, comes from the session.
 */
unsafe extern "C" fn spawn_log(mut from: *const libc::c_char, mut sc: *mut spawn_context) {
    let mut s: *mut session = (*sc).s;
    let mut wl: *mut winlink = (*sc).wl;
    let mut wp0: *mut window_pane = (*sc).wp0;
    let mut name: *const libc::c_char = cmdq_get_name((*sc).item);
    let mut tmp: [libc::c_char; 128] = [0; 128];
    log_debug(
        b"%s: %s, flags=%#x\x00" as *const u8 as *const libc::c_char,
        from,
        name,
        (*sc).flags,
    );
    if !wl.is_null() && !wp0.is_null() {
        xsnprintf(
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"wl=%d wp0=%%%u\x00" as *const u8 as *const libc::c_char,
            (*wl).idx,
            (*wp0).id,
        );
    } else if !wl.is_null() {
        xsnprintf(
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"wl=%d wp0=none\x00" as *const u8 as *const libc::c_char,
            (*wl).idx,
        );
    } else if !wp0.is_null() {
        xsnprintf(
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"wl=none wp0=%%%u\x00" as *const u8 as *const libc::c_char,
            (*wp0).id,
        );
    } else {
        xsnprintf(
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"wl=none wp0=none\x00" as *const u8 as *const libc::c_char,
        );
    }
    log_debug(
        b"%s: s=$%u %s idx=%d\x00" as *const u8 as *const libc::c_char,
        from,
        (*s).id,
        tmp.as_mut_ptr(),
        (*sc).idx,
    );
    log_debug(
        b"%s: name=%s\x00" as *const u8 as *const libc::c_char,
        from,
        if (*sc).name.is_null() {
            b"none\x00" as *const u8 as *const libc::c_char
        } else {
            (*sc).name
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn spawn_window(
    mut sc: *mut spawn_context,
    mut cause: *mut *mut libc::c_char,
) -> *mut winlink {
    let mut item: *mut crate::cmd_queue::cmdq_item = (*sc).item;
    let mut c: *mut client = cmdq_get_client(item);
    let mut s: *mut session = (*sc).s;
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut idx: libc::c_int = (*sc).idx;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut xpixel: u_int = 0;
    let mut ypixel: u_int = 0;
    spawn_log(
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"spawn_window\x00")).as_ptr(),
        sc,
    );
    /*
     * If the window already exists, we are respawning, so destroy all the
     * panes except one.
     */
    if (*sc).flags & 0x4i32 != 0 {
        w = (*(*sc).wl).window;
        if !(*sc).flags & 0x1i32 != 0 {
            wp = (*w).panes.tqh_first;
            while !wp.is_null() {
                if (*wp).fd != -(1i32) {
                    break;
                }
                wp = (*wp).entry.tqe_next
            }
            if !wp.is_null() {
                xasprintf(
                    cause,
                    b"window %s:%d still active\x00" as *const u8 as *const libc::c_char,
                    (*s).name,
                    (*(*sc).wl).idx,
                );
                return 0 as *mut winlink;
            }
        }
        (*sc).wp0 = (*w).panes.tqh_first;
        if !(*(*sc).wp0).entry.tqe_next.is_null() {
            (*(*(*sc).wp0).entry.tqe_next).entry.tqe_prev = (*(*sc).wp0).entry.tqe_prev
        } else {
            (*w).panes.tqh_last = (*(*sc).wp0).entry.tqe_prev
        }
        *(*(*sc).wp0).entry.tqe_prev = (*(*sc).wp0).entry.tqe_next;
        layout_free(w);
        window_destroy_panes(w);
        (*(*sc).wp0).entry.tqe_next = (*w).panes.tqh_first;
        if !(*(*sc).wp0).entry.tqe_next.is_null() {
            (*(*w).panes.tqh_first).entry.tqe_prev = &mut (*(*sc).wp0).entry.tqe_next
        } else {
            (*w).panes.tqh_last = &mut (*(*sc).wp0).entry.tqe_next
        }
        (*w).panes.tqh_first = (*sc).wp0;
        (*(*sc).wp0).entry.tqe_prev = &mut (*w).panes.tqh_first;
        window_pane_resize((*sc).wp0, (*w).sx, (*w).sy);
        layout_init(w, (*sc).wp0);
        window_set_active_pane(w, (*sc).wp0, 0i32);
    }
    /*
     * Otherwise we have no window so we will need to create one. First
     * check if the given index already exists and destroy it if so.
     */
    if !(*sc).flags & 0x4i32 != 0 && idx != -(1i32) {
        wl = winlink_find_by_index(&mut (*s).windows, idx);
        if !wl.is_null() && !(*sc).flags & 0x1i32 != 0 {
            xasprintf(
                cause,
                b"index %d in use\x00" as *const u8 as *const libc::c_char,
                idx,
            );
            return 0 as *mut winlink;
        }
        if !wl.is_null() {
            /*
             * Can't use session_detach as it will destroy session
             * if this makes it empty.
             */
            (*wl).flags &= !(0x1i32 | 0x2i32 | 0x4i32);
            notify_session_window(
                b"window-unlinked\x00" as *const u8 as *const libc::c_char,
                s,
                (*wl).window,
            );
            winlink_stack_remove(&mut (*s).lastw, wl);
            winlink_remove(&mut (*s).windows, wl);
            if (*s).curw == wl {
                (*s).curw = 0 as *mut winlink;
                (*sc).flags &= !(0x2i32)
            }
        }
    }
    /* Then create a window if needed. */
    if !(*sc).flags & 0x4i32 != 0 {
        if idx == -(1i32) {
            idx = (-1i64
                - options_get_number(
                    (*s).options,
                    b"base-index\x00" as *const u8 as *const libc::c_char,
                )) as libc::c_int
        }
        (*sc).wl = winlink_add(&mut (*s).windows, idx);
        if (*sc).wl.is_null() {
            xasprintf(
                cause,
                b"couldn\'t add window %d\x00" as *const u8 as *const libc::c_char,
                idx,
            );
            return 0 as *mut winlink;
        }
        default_window_size(
            (*sc).tc,
            s,
            0 as *mut window,
            &mut sx,
            &mut sy,
            &mut xpixel,
            &mut ypixel,
            -(1i32),
        );
        w = window_create(sx, sy, xpixel, ypixel);
        if w.is_null() {
            winlink_remove(&mut (*s).windows, (*sc).wl);
            xasprintf(
                cause,
                b"couldn\'t create window %d\x00" as *const u8 as *const libc::c_char,
                idx,
            );
            return 0 as *mut winlink;
        }
        if (*s).curw.is_null() {
            (*s).curw = (*sc).wl
        }
        (*(*sc).wl).session = s;
        (*w).latest = (*sc).tc as *mut libc::c_void;
        winlink_set_window((*sc).wl, w);
    } else {
        w = 0 as *mut window
    }
    (*sc).flags |= 0x10i32;
    /* Spawn the pane. */
    wp = spawn_pane(sc, cause);
    if wp.is_null() {
        if !(*sc).flags & 0x4i32 != 0 {
            winlink_remove(&mut (*s).windows, (*sc).wl);
        }
        return 0 as *mut winlink;
    }
    /* Set the name of the new window. */
    if !(*sc).flags & 0x4i32 != 0 {
        if !(*sc).name.is_null() {
            (*w).name = format_single(
                item,
                (*sc).name,
                c,
                s,
                0 as *mut winlink,
                0 as *mut window_pane,
            );
            options_set_number(
                (*w).options,
                b"automatic-rename\x00" as *const u8 as *const libc::c_char,
                0i64,
            );
        } else {
            (*w).name = xstrdup(default_window_name(w))
        }
    }
    /* Switch to the new window if required. */
    if !(*sc).flags & 0x2i32 != 0 {
        session_select(s, (*(*sc).wl).idx);
    }
    /* Fire notification if new window. */
    if !(*sc).flags & 0x4i32 != 0 {
        notify_session_window(
            b"window-linked\x00" as *const u8 as *const libc::c_char,
            s,
            w,
        );
    }
    session_group_synchronize_from(s);
    return (*sc).wl;
}
#[no_mangle]
pub unsafe extern "C" fn spawn_pane(
    mut sc: *mut spawn_context,
    mut cause: *mut *mut libc::c_char,
) -> *mut window_pane {
    let mut item: *mut crate::cmd_queue::cmdq_item = (*sc).item;
    let mut target: *mut cmd_find_state = cmdq_get_target(item);
    let mut c: *mut client = cmdq_get_client(item);
    let mut s: *mut session = (*sc).s;
    let mut w: *mut window = (*(*sc).wl).window;
    let mut new_wp: *mut window_pane = 0 as *mut window_pane;
    let mut child: *mut crate::environ::environ = 0 as *mut crate::environ::environ;
    let mut ee: *mut environ_entry = 0 as *mut environ_entry;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argvp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argv0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cwd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut idx: u_int = 0;
    let mut now: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut hlimit: u_int = 0;
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    let mut key: key_code = 0;
    spawn_log(
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"spawn_pane\x00")).as_ptr(),
        sc,
    );
    /*
     * Work out the current working directory. If respawning, use
     * the pane's stored one unless specified.
     */
    if !(*sc).cwd.is_null() {
        cwd = format_single(
            item,
            (*sc).cwd,
            c,
            (*target).s,
            0 as *mut winlink,
            0 as *mut window_pane,
        )
    } else if !(*sc).flags & 0x4i32 != 0 {
        cwd = xstrdup(server_client_get_cwd(c, (*target).s))
    } else {
        cwd = 0 as *mut libc::c_char
    }
    /*
     * If we are respawning then get rid of the old process. Otherwise
     * either create a new cell or assign to the one we are given.
     */
    hlimit = options_get_number(
        (*s).options,
        b"history-limit\x00" as *const u8 as *const libc::c_char,
    ) as u_int;
    if (*sc).flags & 0x4i32 != 0 {
        if (*(*sc).wp0).fd != -(1i32) && !(*sc).flags & 0x1i32 != 0 {
            window_pane_index((*sc).wp0, &mut idx);
            xasprintf(
                cause,
                b"pane %s:%d.%u still active\x00" as *const u8 as *const libc::c_char,
                (*s).name,
                (*(*sc).wl).idx,
                idx,
            );
            free(cwd as *mut libc::c_void);
            return 0 as *mut window_pane;
        }
        if (*(*sc).wp0).fd != -(1i32) {
            bufferevent_free((*(*sc).wp0).event);
            close((*(*sc).wp0).fd);
        }
        window_pane_reset_mode_all((*sc).wp0);
        screen_reinit(&mut (*(*sc).wp0).base);
        input_free((*(*sc).wp0).ictx);
        (*(*sc).wp0).ictx = 0 as *mut crate::input::input_ctx;
        new_wp = (*sc).wp0;
        (*new_wp).flags &= !(0x200i32 | 0x400i32)
    } else if (*sc).lc.is_null() {
        new_wp = window_add_pane(w, 0 as *mut window_pane, hlimit, (*sc).flags);
        layout_init(w, new_wp);
    } else {
        new_wp = window_add_pane(w, (*sc).wp0, hlimit, (*sc).flags);
        layout_assign_pane((*sc).lc, new_wp);
    }
    /*
     * Now we have a pane with nothing running in it ready for the new process.
     * Work out the command and arguments and store the working directory.
     */
    if (*sc).argc == 0i32 && !(*sc).flags & 0x4i32 != 0 {
        cmd = options_get_string(
            (*s).options,
            b"default-command\x00" as *const u8 as *const libc::c_char,
        );
        if !cmd.is_null() && *cmd as libc::c_int != '\u{0}' as i32 {
            argc = 1i32;
            argv = &mut cmd as *mut *const libc::c_char as *mut *mut libc::c_char
        } else {
            argc = 0i32;
            argv = 0 as *mut *mut libc::c_char
        }
    } else {
        argc = (*sc).argc;
        argv = (*sc).argv
    }
    if !cwd.is_null() {
        free((*new_wp).cwd as *mut libc::c_void);
        (*new_wp).cwd = cwd
    }
    /*
     * Replace the stored arguments if there are new ones. If not, the
     * existing ones will be used (they will only exist for respawn).
     */
    if argc > 0i32 {
        cmd_free_argv((*new_wp).argc, (*new_wp).argv);
        (*new_wp).argc = argc;
        (*new_wp).argv = cmd_copy_argv(argc, argv)
    }
    /* Create an environment for this pane. */
    child = environ_for_session(s, 0i32);
    if !(*sc).environ.is_null() {
        environ_copy((*sc).environ, child);
    }
    environ_set(
        child,
        b"TMUX_PANE\x00" as *const u8 as *const libc::c_char,
        0i32,
        b"%%%u\x00" as *const u8 as *const libc::c_char,
        (*new_wp).id,
    );
    /*
     * Then the PATH environment variable. The session one is replaced from
     * the client if there is one because otherwise running "tmux new
     * myprogram" wouldn't work if myprogram isn't in the session's path.
     */
    if !c.is_null() && (*c).session.is_null() {
        /* only unattached clients */
        ee = environ_find(
            (*c).environ,
            b"PATH\x00" as *const u8 as *const libc::c_char,
        );
        if !ee.is_null() {
            environ_set(
                child,
                b"PATH\x00" as *const u8 as *const libc::c_char,
                0i32,
                b"%s\x00" as *const u8 as *const libc::c_char,
                (*ee).value,
            );
        }
    }
    if environ_find(child, b"PATH\x00" as *const u8 as *const libc::c_char).is_null() {
        environ_set(
            child,
            b"PATH\x00" as *const u8 as *const libc::c_char,
            0i32,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"/usr/bin:/bin\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* Then the shell. If respawning, use the old one. */
    if !(*sc).flags & 0x4i32 != 0 {
        tmp = options_get_string(
            (*s).options,
            b"default-shell\x00" as *const u8 as *const libc::c_char,
        );
        if checkshell(tmp) == 0 {
            tmp = b"/bin/sh\x00" as *const u8 as *const libc::c_char
        }
        free((*new_wp).shell as *mut libc::c_void);
        (*new_wp).shell = xstrdup(tmp)
    }
    environ_set(
        child,
        b"SHELL\x00" as *const u8 as *const libc::c_char,
        0i32,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*new_wp).shell,
    );
    /* Log the arguments we are going to use. */
    log_debug(
        b"%s: shell=%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"spawn_pane\x00")).as_ptr(),
        (*new_wp).shell,
    );
    if (*new_wp).argc != 0i32 {
        cp = cmd_stringify_argv((*new_wp).argc, (*new_wp).argv);
        log_debug(
            b"%s: cmd=%s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"spawn_pane\x00")).as_ptr(),
            cp,
        );
        free(cp as *mut libc::c_void);
    }
    if !cwd.is_null() {
        log_debug(
            b"%s: cwd=%s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"spawn_pane\x00")).as_ptr(),
            cwd,
        );
    }
    cmd_log_argv(
        (*new_wp).argc,
        (*new_wp).argv,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"spawn_pane\x00")).as_ptr(),
    );
    environ_log(
        child,
        b"%s: environment \x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"spawn_pane\x00")).as_ptr(),
    );
    /* Initialize the window size. */
    memset(
        &mut ws as *mut winsize as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<winsize>() as libc::c_ulong,
    );
    ws.ws_col = (*(*new_wp).base.grid).sx as libc::c_ushort;
    ws.ws_row = (*(*new_wp).base.grid).sy as libc::c_ushort;
    ws.ws_xpixel = (*w).xpixel.wrapping_mul(ws.ws_col as libc::c_uint) as libc::c_ushort;
    ws.ws_ypixel = (*w).ypixel.wrapping_mul(ws.ws_row as libc::c_uint) as libc::c_ushort;
    /* Block signals until fork has completed. */
    sigfillset(&mut set);
    sigprocmask(0i32, &mut set, &mut oldset);
    /* If the command is empty, don't fork a child process. */
    if (*sc).flags & 0x40i32 != 0 {
        (*new_wp).flags |= 0x800i32;
        (*new_wp).base.mode &= !(0x1i32);
        (*new_wp).base.mode |= 0x4000i32
    } else {
        /* Fork the new process. */
        (*new_wp).pid = fdforkpty(
            ptm_fd,
            &mut (*new_wp).fd,
            (*new_wp).tty.as_mut_ptr(),
            0 as *mut termios,
            &mut ws,
        );
        if (*new_wp).pid == -(1i32) {
            xasprintf(
                cause,
                b"fork failed: %s\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            (*new_wp).fd = -(1i32);
            if !(*sc).flags & 0x4i32 != 0 {
                server_client_remove_pane(new_wp);
                layout_close_pane(new_wp);
                window_remove_pane(w, new_wp);
            }
            sigprocmask(2i32, &mut oldset, 0 as *mut sigset_t);
            environ_free(child);
            return 0 as *mut window_pane;
        }
        /* In the parent process, everything is done now. */
        if !((*new_wp).pid != 0i32) {
            /*
             * Child process. Change to the working directory or home if that
             * fails.
             */
            if chdir((*new_wp).cwd) != 0i32 {
                tmp = find_home();
                if tmp.is_null() || chdir(tmp) != 0i32 {
                    chdir(b"/\x00" as *const u8 as *const libc::c_char);
                }
            }
            /*
             * Update terminal escape characters from the session if available and
             * force VERASE to tmux's backspace.
             */
            if tcgetattr(0i32, &mut now) != 0i32 {
                _exit(1i32);
            }
            if !(*s).tio.is_null() {
                memcpy(
                    now.c_cc.as_mut_ptr() as *mut libc::c_void,
                    (*(*s).tio).c_cc.as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<[cc_t; 32]>() as libc::c_ulong,
                );
            }
            key = options_get_number(
                global_options,
                b"backspace\x00" as *const u8 as *const libc::c_char,
            ) as key_code;
            if key >= 0x7fu64 {
                now.c_cc[2usize] = '\u{7f}' as cc_t
            } else {
                now.c_cc[2usize] = key as cc_t
            }
            now.c_iflag |= 0o40000u32;
            if tcsetattr(0i32, 0i32, &mut now) != 0i32 {
                _exit(1i32);
            }
            /* Clean up file descriptors and signals and update the environment. */
            closefrom(2i32 + 1i32);
            proc_clear_signals(server_proc, 1i32);
            sigprocmask(2i32, &mut oldset, 0 as *mut sigset_t);
            log_close();
            environ_push(child);
            /*
             * If given multiple arguments, use execvp(). Copy the arguments to
             * ensure they end in a NULL.
             */
            if (*new_wp).argc != 0i32 && (*new_wp).argc != 1i32 {
                argvp = cmd_copy_argv((*new_wp).argc, (*new_wp).argv);
                execvp(*argvp.offset(0isize), argvp as *const *mut libc::c_char);
                _exit(1i32);
            }
            /*
             * If one argument, pass it to $SHELL -c. Otherwise create a login
             * shell.
             */
            cp = strrchr((*new_wp).shell, '/' as i32);
            if (*new_wp).argc == 1i32 {
                tmp = *(*new_wp).argv.offset(0isize);
                if !cp.is_null() && *cp.offset(1isize) as libc::c_int != '\u{0}' as i32 {
                    xasprintf(
                        &mut argv0 as *mut *mut libc::c_char,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        cp.offset(1isize),
                    );
                } else {
                    xasprintf(
                        &mut argv0 as *mut *mut libc::c_char,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        (*new_wp).shell,
                    );
                }
                execl(
                    (*new_wp).shell,
                    argv0,
                    b"-c\x00" as *const u8 as *const libc::c_char,
                    tmp,
                    0 as *mut libc::c_char,
                );
                _exit(1i32);
            }
            if !cp.is_null() && *cp.offset(1isize) as libc::c_int != '\u{0}' as i32 {
                xasprintf(
                    &mut argv0 as *mut *mut libc::c_char,
                    b"-%s\x00" as *const u8 as *const libc::c_char,
                    cp.offset(1isize),
                );
            } else {
                xasprintf(
                    &mut argv0 as *mut *mut libc::c_char,
                    b"-%s\x00" as *const u8 as *const libc::c_char,
                    (*new_wp).shell,
                );
            }
            execl((*new_wp).shell, argv0, 0 as *mut libc::c_char);
            _exit(1i32);
        }
    }
    (*new_wp).flags &= !(0x100i32);
    sigprocmask(2i32, &mut oldset, 0 as *mut sigset_t);
    window_pane_set_event(new_wp);
    environ_free(child);
    if (*sc).flags & 0x4i32 != 0 {
        return new_wp;
    }
    if !(*sc).flags & 0x2i32 != 0 || (*w).active.is_null() {
        if (*sc).flags & 0x10i32 != 0 {
            window_set_active_pane(w, new_wp, 0i32);
        } else {
            window_set_active_pane(w, new_wp, 1i32);
        }
    }
    if !(*sc).flags & 0x10i32 != 0 {
        notify_window(
            b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
            w,
        );
    }
    return new_wp;
}
