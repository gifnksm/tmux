use crate::{
    grid::{Cell as GridCell, Grid, Line as GridLine},
    tty_code::{code as tty_code_code, Code as TtyCode},
    utf8::Utf8Data,
};
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __b64_ntop(
        _: *const libc::c_uchar,
        _: size_t,
        _: *mut libc::c_char,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    #[no_mangle]
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut crate::options::options;
    #[no_mangle]
    fn setblocking(_: libc::c_int, _: libc::c_int);
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
    fn tty_term_apply_overrides(_: *mut tty_term);
    #[no_mangle]
    fn tty_term_create(
        _: *mut tty,
        _: *mut libc::c_char,
        _: *mut libc::c_int,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> *mut tty_term;
    #[no_mangle]
    fn tty_term_free(_: *mut tty_term);
    #[no_mangle]
    fn tty_term_has(_: *mut tty_term, _: TtyCode) -> libc::c_int;
    #[no_mangle]
    fn tty_term_string(_: *mut tty_term, _: TtyCode) -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_string1(_: *mut tty_term, _: TtyCode, _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_string2(
        _: *mut tty_term,
        _: TtyCode,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_string3(
        _: *mut tty_term,
        _: TtyCode,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_ptr1(_: *mut tty_term, _: TtyCode, _: *const libc::c_void) -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_ptr2(
        _: *mut tty_term,
        _: TtyCode,
        _: *const libc::c_void,
        _: *const libc::c_void,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_number(_: *mut tty_term, _: TtyCode) -> libc::c_int;
    #[no_mangle]
    fn tty_term_flag(_: *mut tty_term, _: TtyCode) -> libc::c_int;
    #[no_mangle]
    fn tty_apply_features(_: *mut tty_term, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tty_acs_needed(_: *mut tty) -> libc::c_int;
    #[no_mangle]
    fn tty_acs_get(_: *mut tty, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn tty_acs_reverse_get(_: *mut tty, _: *const libc::c_char, _: size_t) -> libc::c_int;
    #[no_mangle]
    fn tty_keys_build(_: *mut tty);
    #[no_mangle]
    fn tty_keys_free(_: *mut tty);
    #[no_mangle]
    fn tty_keys_next(_: *mut tty) -> libc::c_int;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn server_client_lost(_: *mut client);
    #[no_mangle]
    fn server_client_get_pane(_: *mut client) -> *mut window_pane;
    #[no_mangle]
    fn status_line_size(_: *mut client) -> u_int;
    #[no_mangle]
    fn colour_find_rgb(_: u_char, _: u_char, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn colour_split_rgb(_: libc::c_int, _: *mut u_char, _: *mut u_char, _: *mut u_char);
    #[no_mangle]
    fn colour_256toRGB(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn colour_256to16(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn grid_cells_equal(_: *const crate::grid::Cell, _: *const crate::grid::Cell) -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn utf8_set(_: *mut Utf8Data, _: u_char);
    #[no_mangle]
    fn screen_select_cell(_: *mut screen, _: *mut crate::grid::Cell, _: *const crate::grid::Cell);
    #[no_mangle]
    fn grid_view_get_cell(_: *mut crate::grid::Grid, _: u_int, _: u_int, _: *mut crate::grid::Cell);
    #[no_mangle]
    fn grid_get_line(_: *mut crate::grid::Grid, _: u_int) -> *mut GridLine;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn style_add(
        _: *mut crate::grid::Cell,
        _: *mut crate::options::options,
        _: *const libc::c_char,
        _: *mut crate::format::format_tree,
    );
    #[no_mangle]
    fn evbuffer_read(buffer: *mut evbuffer, fd: libc::c_int, howmuch: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_write(buffer: *mut evbuffer, fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_add(buf: *mut evbuffer, data: *const libc::c_void, datlen: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_free(buf: *mut evbuffer);
    #[no_mangle]
    fn evbuffer_new() -> *mut evbuffer;
    #[no_mangle]
    fn event_set(
        _: *mut event,
        _: libc::c_int,
        _: libc::c_short,
        _: Option<
            unsafe extern "C" fn(_: libc::c_int, _: libc::c_short, _: *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
    );
    #[no_mangle]
    fn event_initialized(ev: *const event) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;

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
pub type bitstr_t = libc::c_uchar;

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
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_6,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_1,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_1 {
    pub ev_io: C2RustUnnamed_4,
    pub ev_signal: C2RustUnnamed_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub ev_signal_next: C2RustUnnamed_3,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub ev_io_next: C2RustUnnamed_5,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_6 {
    pub ev_next_with_common_timeout: C2RustUnnamed_7,
    pub min_heap_idx: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_9,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_8,
    pub evcb_arg: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_8 {
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
pub struct C2RustUnnamed_9 {
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
    pub entry: C2RustUnnamed_10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
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
    pub entry: C2RustUnnamed_11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
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
static mut tty_log_fd: libc::c_int = -(1i32);
#[no_mangle]
pub unsafe extern "C" fn tty_create_log() {
    let mut name: [libc::c_char; 64] = [0; 64];
    xsnprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"tmux-out-%ld.log\x00" as *const u8 as *const libc::c_char,
        getpid() as libc::c_long,
    );
    tty_log_fd = open(name.as_mut_ptr(), 0o1i32 | 0o100i32 | 0o1000i32, 0o644i32);
    if tty_log_fd != -(1i32) && fcntl(tty_log_fd, 2i32, 1i32) == -(1i32) {
        fatal(b"fcntl failed\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_init(mut tty: *mut tty, mut c: *mut client) -> libc::c_int {
    if isatty((*c).fd) == 0 {
        return -(1i32);
    }
    memset(
        tty as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<tty>() as libc::c_ulong,
    );
    (*tty).client = c;
    (*tty).cstyle = 0u32;
    (*tty).ccolour = xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    if tcgetattr((*c).fd, &mut (*tty).tio) != 0i32 {
        return -(1i32);
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn tty_resize(mut tty: *mut tty) {
    let mut c: *mut client = (*tty).client;
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut xpixel: u_int = 0;
    let mut ypixel: u_int = 0;
    if ioctl((*c).fd, 0x5413u64, &mut ws as *mut winsize) != -(1i32) {
        sx = ws.ws_col as u_int;
        if sx == 0u32 {
            sx = 80u32;
            xpixel = 0u32
        } else {
            xpixel = (ws.ws_xpixel as libc::c_uint).wrapping_div(sx)
        }
        sy = ws.ws_row as u_int;
        if sy == 0u32 {
            sy = 24u32;
            ypixel = 0u32
        } else {
            ypixel = (ws.ws_ypixel as libc::c_uint).wrapping_div(sy)
        }
    } else {
        sx = 80u32;
        sy = 24u32;
        xpixel = 0u32;
        ypixel = 0u32
    }
    log_debug(
        b"%s: %s now %ux%u (%ux%u)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"tty_resize\x00")).as_ptr(),
        (*c).name,
        sx,
        sy,
        xpixel,
        ypixel,
    );
    tty_set_size(tty, sx, sy, xpixel, ypixel);
    tty_invalidate(tty);
}
#[no_mangle]
pub unsafe extern "C" fn tty_set_size(
    mut tty: *mut tty,
    mut sx: u_int,
    mut sy: u_int,
    mut xpixel: u_int,
    mut ypixel: u_int,
) {
    (*tty).sx = sx;
    (*tty).sy = sy;
    (*tty).xpixel = xpixel;
    (*tty).ypixel = ypixel;
}
unsafe extern "C" fn tty_read_callback(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    let mut name: *const libc::c_char = (*c).name;
    let mut size: size_t = evbuffer_get_length((*tty).in_0);
    let mut nread: libc::c_int = 0;
    nread = evbuffer_read((*tty).in_0, (*c).fd, -(1i32));
    if nread == 0i32 || nread == -(1i32) {
        if nread == 0i32 {
            log_debug(
                b"%s: read closed\x00" as *const u8 as *const libc::c_char,
                name,
            );
        } else {
            log_debug(
                b"%s: read error: %s\x00" as *const u8 as *const libc::c_char,
                name,
                strerror(*__errno_location()),
            );
        }
        event_del(&mut (*tty).event_in);
        server_client_lost((*tty).client);
        return;
    }
    log_debug(
        b"%s: read %d bytes (already %zu)\x00" as *const u8 as *const libc::c_char,
        name,
        nread,
        size,
    );
    while tty_keys_next(tty) != 0 {}
}
unsafe extern "C" fn tty_timer_callback(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 100000i64,
        };
        init
    };
    log_debug(
        b"%s: %zu discarded\x00" as *const u8 as *const libc::c_char,
        (*c).name,
        (*tty).discarded,
    );
    (*c).flags |= (0x8i32 | 0x10i32 | 0x1000000i32 | 0x400i32 | 0x2000000i32 | 0x20000000i32)
        as libc::c_ulong;
    (*c).discarded = ((*c).discarded).wrapping_add((*tty).discarded);
    if (*tty).discarded
        < (1u32).wrapping_add((*tty).sx.wrapping_mul((*tty).sy).wrapping_div(8u32)) as libc::c_ulong
    {
        (*tty).flags &= !(0x80i32);
        tty_invalidate(tty);
        return;
    }
    (*tty).discarded = 0u64;
    event_add(&mut (*tty).timer, &mut tv);
}
unsafe extern "C" fn tty_block_maybe(mut tty: *mut tty) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut size: size_t = evbuffer_get_length((*tty).out);
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 100000i64,
        };
        init
    };
    if size
        < (1u32).wrapping_add((*tty).sx.wrapping_mul((*tty).sy).wrapping_mul(8u32)) as libc::c_ulong
    {
        return 0i32;
    }
    if (*tty).flags & 0x80i32 != 0 {
        return 1i32;
    }
    (*tty).flags |= 0x80i32;
    log_debug(
        b"%s: can\'t keep up, %zu discarded\x00" as *const u8 as *const libc::c_char,
        (*c).name,
        size,
    );
    evbuffer_drain((*tty).out, size);
    (*c).discarded = ((*c).discarded).wrapping_add(size);
    (*tty).discarded = 0u64;
    event_add(&mut (*tty).timer, &mut tv);
    return 1i32;
}
unsafe extern "C" fn tty_write_callback(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    let mut size: size_t = evbuffer_get_length((*tty).out);
    let mut nwrite: libc::c_int = 0;
    nwrite = evbuffer_write((*tty).out, (*c).fd);
    if nwrite == -(1i32) {
        return;
    }
    log_debug(
        b"%s: wrote %d bytes (of %zu)\x00" as *const u8 as *const libc::c_char,
        (*c).name,
        nwrite,
        size,
    );
    if (*c).redraw > 0u64 {
        if nwrite as size_t >= (*c).redraw {
            (*c).redraw = 0u64
        } else {
            (*c).redraw = ((*c).redraw).wrapping_sub(nwrite as libc::c_ulong)
        }
        log_debug(
            b"%s: waiting for redraw, %zu bytes left\x00" as *const u8 as *const libc::c_char,
            (*c).name,
            (*c).redraw,
        );
    } else if tty_block_maybe(tty) != 0 {
        return;
    }
    if evbuffer_get_length((*tty).out) != 0u64 {
        event_add(&mut (*tty).event_out, 0 as *const timeval);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_open(
    mut tty: *mut tty,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    (*tty).term = tty_term_create(tty, (*c).term_name, &mut (*c).term_features, (*c).fd, cause);
    if (*tty).term.is_null() {
        tty_close(tty);
        return -(1i32);
    }
    (*tty).flags |= 0x20i32;
    (*tty).flags &= !(0x1i32 | 0x2i32 | 0x80i32 | 0x4i32);
    event_set(
        &mut (*tty).event_in,
        (*c).fd,
        (0x10i32 | 0x2i32) as libc::c_short,
        Some(
            tty_read_callback
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        tty as *mut libc::c_void,
    );
    (*tty).in_0 = evbuffer_new();
    if (*tty).in_0.is_null() {
        fatal(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    event_set(
        &mut (*tty).event_out,
        (*c).fd,
        0x4i16,
        Some(
            tty_write_callback
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        tty as *mut libc::c_void,
    );
    (*tty).out = evbuffer_new();
    if (*tty).out.is_null() {
        fatal(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    event_set(
        &mut (*tty).timer,
        -(1i32),
        0i16,
        Some(
            tty_timer_callback
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        tty as *mut libc::c_void,
    );
    tty_start_tty(tty);
    tty_keys_build(tty);
    return 0i32;
}
unsafe extern "C" fn tty_start_timer_callback(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    log_debug(
        b"%s: start timer fired\x00" as *const u8 as *const libc::c_char,
        (*c).name,
    );
    if (*tty).flags & (0x100i32 | 0x200i32) == 0i32 {
        tty_update_features(tty);
    }
    (*tty).flags |= 0x100i32 | 0x200i32;
}
#[no_mangle]
pub unsafe extern "C" fn tty_start_tty(mut tty: *mut tty) {
    let mut c: *mut client = (*tty).client;
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
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 1i64,
            tv_usec: 0,
        };
        init
    };
    setblocking((*c).fd, 0i32);
    event_add(&mut (*tty).event_in, 0 as *const timeval);
    memcpy(
        &mut tio as *mut termios as *mut libc::c_void,
        &mut (*tty).tio as *mut termios as *const libc::c_void,
        ::std::mem::size_of::<termios>() as libc::c_ulong,
    );
    tio.c_iflag &= !(0o2000i32 | 0o10000i32 | 0o400i32 | 0o100i32 | 0o200i32 | 0o20000i32 | 0o40i32)
        as libc::c_uint;
    tio.c_iflag |= 0o1u32;
    tio.c_oflag &= !(0o1i32 | 0o4i32 | 0o10i32 | 0o40i32) as libc::c_uint;
    tio.c_lflag &= !(0o100000i32
        | 0o2i32
        | 0o10i32
        | 0o20i32
        | 0o100i32
        | 0o1000i32
        | 0o2000i32
        | 0o4000i32
        | 0o1i32) as libc::c_uint;
    tio.c_cc[6usize] = 1u8;
    tio.c_cc[5usize] = 0u8;
    if tcsetattr((*c).fd, 0i32, &mut tio) == 0i32 {
        tcflush((*c).fd, 2i32);
    }
    tty_putcode(tty, tty_code_code::SMCUP);
    tty_putcode(tty, tty_code_code::SMKX);
    tty_putcode(tty, tty_code_code::CLEAR);
    if tty_acs_needed(tty) != 0 {
        log_debug(
            b"%s: using capabilities for ACS\x00" as *const u8 as *const libc::c_char,
            (*c).name,
        );
        tty_putcode(tty, tty_code_code::ENACS);
    } else {
        log_debug(
            b"%s: using UTF-8 for ACS\x00" as *const u8 as *const libc::c_char,
            (*c).name,
        );
    }
    tty_putcode(tty, tty_code_code::CNORM);
    if tty_term_has((*tty).term, tty_code_code::KMOUS) != 0 {
        tty_puts(
            tty,
            b"\x1b[?1000l\x1b[?1002l\x1b[?1003l\x00" as *const u8 as *const libc::c_char,
        );
        tty_puts(
            tty,
            b"\x1b[?1006l\x1b[?1005l\x00" as *const u8 as *const libc::c_char,
        );
    }
    event_set(
        &mut (*tty).start_timer,
        -(1i32),
        0i16,
        Some(
            tty_start_timer_callback
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        tty as *mut libc::c_void,
    );
    event_add(&mut (*tty).start_timer, &mut tv);
    (*tty).flags |= 0x10i32;
    tty_invalidate(tty);
    if *(*tty).ccolour as libc::c_int != '\u{0}' as i32 {
        tty_force_cursor_colour(tty, b"\x00" as *const u8 as *const libc::c_char);
    }
    (*tty).mouse_drag_flag = 0i32;
    (*tty).mouse_drag_update = None;
    (*tty).mouse_drag_release = None;
}
#[no_mangle]
pub unsafe extern "C" fn tty_send_requests(mut tty: *mut tty) {
    if !(*tty).flags & 0x10i32 != 0 {
        return;
    }
    if (*(*tty).term).flags & 0x20i32 != 0 {
        if !(*tty).flags & 0x100i32 != 0 {
            tty_puts(tty, b"\x1b[>c\x00" as *const u8 as *const libc::c_char);
        }
        if !(*tty).flags & 0x200i32 != 0 {
            tty_puts(tty, b"\x1b[>q\x00" as *const u8 as *const libc::c_char);
        }
    } else {
        (*tty).flags |= 0x100i32 | 0x200i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_stop_tty(mut tty: *mut tty) {
    let mut c: *mut client = (*tty).client;
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    if (*tty).flags & 0x10i32 == 0 {
        return;
    }
    (*tty).flags &= !(0x10i32);
    event_del(&mut (*tty).start_timer);
    event_del(&mut (*tty).timer);
    (*tty).flags &= !(0x80i32);
    event_del(&mut (*tty).event_in);
    event_del(&mut (*tty).event_out);
    /*
     * Be flexible about error handling and try not kill the server just
     * because the fd is invalid. Things like ssh -t can easily leave us
     * with a dead tty.
     */
    if ioctl((*c).fd, 0x5413u64, &mut ws as *mut winsize) == -(1i32) {
        return;
    }
    if tcsetattr((*c).fd, 0i32, &mut (*tty).tio) == -(1i32) {
        return;
    }
    tty_raw(
        tty,
        tty_term_string2(
            (*tty).term,
            tty_code_code::CSR,
            0i32,
            ws.ws_row as libc::c_int - 1i32,
        ),
    );
    if tty_acs_needed(tty) != 0 {
        tty_raw(tty, tty_term_string((*tty).term, tty_code_code::RMACS));
    }
    tty_raw(tty, tty_term_string((*tty).term, tty_code_code::SGR0));
    tty_raw(tty, tty_term_string((*tty).term, tty_code_code::RMKX));
    tty_raw(tty, tty_term_string((*tty).term, tty_code_code::CLEAR));
    if tty_term_has((*tty).term, tty_code_code::SS) != 0 && (*tty).cstyle != 0u32 {
        if tty_term_has((*tty).term, tty_code_code::SE) != 0 {
            tty_raw(tty, tty_term_string((*tty).term, tty_code_code::SE));
        } else {
            tty_raw(tty, tty_term_string1((*tty).term, tty_code_code::SS, 0i32));
        }
    }
    if (*tty).mode & 0x400i32 != 0 {
        tty_raw(tty, tty_term_string((*tty).term, tty_code_code::DSBP));
    }
    if *(*tty).ccolour as libc::c_int != '\u{0}' as i32 {
        tty_raw(tty, tty_term_string((*tty).term, tty_code_code::CR));
    }
    tty_raw(tty, tty_term_string((*tty).term, tty_code_code::CNORM));
    if tty_term_has((*tty).term, tty_code_code::KMOUS) != 0 {
        tty_raw(
            tty,
            b"\x1b[?1000l\x1b[?1002l\x1b[?1003l\x00" as *const u8 as *const libc::c_char,
        );
        tty_raw(
            tty,
            b"\x1b[?1006l\x1b[?1005l\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*(*tty).term).flags & 0x20i32 != 0 {
        tty_raw(tty, b"\x1b[?7727l\x00" as *const u8 as *const libc::c_char);
    }
    tty_raw(tty, tty_term_string((*tty).term, tty_code_code::DSFCS));
    tty_raw(tty, tty_term_string((*tty).term, tty_code_code::DSEKS));
    if (*(*tty).term).flags & 0x4i32 != 0 {
        tty_raw(tty, tty_term_string((*tty).term, tty_code_code::DSMG));
    }
    tty_raw(tty, tty_term_string((*tty).term, tty_code_code::RMCUP));
    setblocking((*c).fd, 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn tty_close(mut tty: *mut tty) {
    if event_initialized(&mut (*tty).key_timer) != 0 {
        event_del(&mut (*tty).key_timer);
    }
    tty_stop_tty(tty);
    if (*tty).flags & 0x20i32 != 0 {
        evbuffer_free((*tty).in_0);
        event_del(&mut (*tty).event_in);
        evbuffer_free((*tty).out);
        event_del(&mut (*tty).event_out);
        tty_term_free((*tty).term);
        tty_keys_free(tty);
        (*tty).flags &= !(0x20i32)
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_free(mut tty: *mut tty) {
    tty_close(tty);
    free((*tty).ccolour as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tty_update_features(mut tty: *mut tty) {
    let mut c: *mut client = (*tty).client;
    if tty_apply_features((*tty).term, (*c).term_features) != 0 {
        tty_term_apply_overrides((*tty).term);
    }
    if (*(*tty).term).flags & 0x4i32 != 0 {
        tty_putcode(tty, tty_code_code::ENMG);
    }
    if options_get_number(
        global_options,
        b"extended-keys\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        tty_puts(tty, tty_term_string((*tty).term, tty_code_code::ENEKS));
    }
    if options_get_number(
        global_options,
        b"focus-events\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        tty_puts(tty, tty_term_string((*tty).term, tty_code_code::ENFCS));
    }
    if (*(*tty).term).flags & 0x20i32 != 0 {
        tty_puts(tty, b"\x1b[?7727h\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_raw(mut tty: *mut tty, mut s: *const libc::c_char) {
    let mut c: *mut client = (*tty).client;
    let mut n: ssize_t = 0;
    let mut slen: ssize_t = 0;
    let mut i: u_int = 0;
    slen = strlen(s) as ssize_t;
    i = 0u32;
    while i < 5u32 {
        n = write((*c).fd, s as *const libc::c_void, slen as size_t);
        if n >= 0i64 {
            s = s.offset(n as isize);
            slen -= n;
            if slen == 0i64 {
                break;
            }
        } else if n == -1i64 && *__errno_location() != 11i32 {
            break;
        }
        usleep(100u32);
        i = i.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode(mut tty: *mut tty, mut code: TtyCode) {
    tty_puts(tty, tty_term_string((*tty).term, code));
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode1(mut tty: *mut tty, mut code: TtyCode, mut a: libc::c_int) {
    if a < 0i32 {
        return;
    }
    tty_puts(tty, tty_term_string1((*tty).term, code, a));
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode2(
    mut tty: *mut tty,
    mut code: TtyCode,
    mut a: libc::c_int,
    mut b: libc::c_int,
) {
    if a < 0i32 || b < 0i32 {
        return;
    }
    tty_puts(tty, tty_term_string2((*tty).term, code, a, b));
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode3(
    mut tty: *mut tty,
    mut code: TtyCode,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) {
    if a < 0i32 || b < 0i32 || c < 0i32 {
        return;
    }
    tty_puts(tty, tty_term_string3((*tty).term, code, a, b, c));
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode_ptr1(
    mut tty: *mut tty,
    mut code: TtyCode,
    mut a: *const libc::c_void,
) {
    if !a.is_null() {
        tty_puts(tty, tty_term_ptr1((*tty).term, code, a));
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode_ptr2(
    mut tty: *mut tty,
    mut code: TtyCode,
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) {
    if !a.is_null() && !b.is_null() {
        tty_puts(tty, tty_term_ptr2((*tty).term, code, a, b));
    };
}
unsafe extern "C" fn tty_add(mut tty: *mut tty, mut buf: *const libc::c_char, mut len: size_t) {
    let mut c: *mut client = (*tty).client;
    if (*tty).flags & 0x80i32 != 0 {
        (*tty).discarded = ((*tty).discarded).wrapping_add(len);
        return;
    }
    evbuffer_add((*tty).out, buf as *const libc::c_void, len);
    log_debug(
        b"%s: %.*s\x00" as *const u8 as *const libc::c_char,
        (*c).name,
        len as libc::c_int,
        buf,
    );
    (*c).written = ((*c).written).wrapping_add(len);
    if tty_log_fd != -(1i32) {
        write(tty_log_fd, buf as *const libc::c_void, len);
    }
    if (*tty).flags & 0x10i32 != 0 {
        event_add(&mut (*tty).event_out, 0 as *const timeval);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_puts(mut tty: *mut tty, mut s: *const libc::c_char) {
    if *s as libc::c_int != '\u{0}' as i32 {
        tty_add(tty, s, strlen(s));
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putc(mut tty: *mut tty, mut ch: u_char) {
    let mut acs: *const libc::c_char = 0 as *const libc::c_char;
    if (*(*tty).term).flags & 0x2i32 != 0
        && ch as libc::c_int >= 0x20i32
        && ch as libc::c_int != 0x7fi32
        && (*tty).cy == (*tty).sy.wrapping_sub(1u32)
        && (*tty).cx.wrapping_add(1u32) >= (*tty).sx
    {
        return;
    }
    if (*tty).cell.attr as libc::c_int & 0x80i32 != 0 {
        acs = tty_acs_get(tty, ch);
        if !acs.is_null() {
            tty_add(tty, acs, strlen(acs));
        } else {
            tty_add(tty, &mut ch as *mut u_char as *const libc::c_char, 1u64);
        }
    } else {
        tty_add(tty, &mut ch as *mut u_char as *const libc::c_char, 1u64);
    }
    if ch as libc::c_int >= 0x20i32 && ch as libc::c_int != 0x7fi32 {
        if (*tty).cx >= (*tty).sx {
            (*tty).cx = 1u32;
            if (*tty).cy != (*tty).rlower {
                (*tty).cy = (*tty).cy.wrapping_add(1)
            }
            /*
             * On !am terminals, force the cursor position to where
             * we think it should be after a line wrap - this means
             * it works on sensible terminals as well.
             */
            if (*(*tty).term).flags & 0x2i32 != 0 {
                tty_putcode2(
                    tty,
                    tty_code_code::CUP,
                    (*tty).cy as libc::c_int,
                    (*tty).cx as libc::c_int,
                );
            }
        } else {
            (*tty).cx = (*tty).cx.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putn(
    mut tty: *mut tty,
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut width: u_int,
) {
    if (*(*tty).term).flags & 0x2i32 != 0
        && (*tty).cy == (*tty).sy.wrapping_sub(1u32)
        && ((*tty).cx as libc::c_ulong).wrapping_add(len) >= (*tty).sx as libc::c_ulong
    {
        len = (*tty).sx.wrapping_sub((*tty).cx).wrapping_sub(1u32) as size_t
    }
    tty_add(tty, buf as *const libc::c_char, len);
    if (*tty).cx.wrapping_add(width) > (*tty).sx {
        (*tty).cx = (*tty).cx.wrapping_add(width).wrapping_sub((*tty).sx);
        if (*tty).cx <= (*tty).sx {
            (*tty).cy = (*tty).cy.wrapping_add(1)
        } else {
            (*tty).cy = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
            (*tty).cx = (*tty).cy
        }
    } else {
        (*tty).cx = ((*tty).cx).wrapping_add(width)
    };
}
unsafe extern "C" fn tty_set_italics(mut tty: *mut tty) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if tty_term_has((*tty).term, tty_code_code::SITM) != 0 {
        s = options_get_string(
            global_options,
            b"default-terminal\x00" as *const u8 as *const libc::c_char,
        );
        if strcmp(s, b"screen\x00" as *const u8 as *const libc::c_char) != 0i32
            && strncmp(s, b"screen-\x00" as *const u8 as *const libc::c_char, 7u64) != 0i32
        {
            tty_putcode(tty, tty_code_code::SITM);
            return;
        }
    }
    tty_putcode(tty, tty_code_code::SMSO);
}
#[no_mangle]
pub unsafe extern "C" fn tty_set_title(mut tty: *mut tty, mut title: *const libc::c_char) {
    if tty_term_has((*tty).term, tty_code_code::TSL) == 0
        || tty_term_has((*tty).term, tty_code_code::FSL) == 0
    {
        return;
    }
    tty_putcode(tty, tty_code_code::TSL);
    tty_puts(tty, title);
    tty_putcode(tty, tty_code_code::FSL);
}
unsafe extern "C" fn tty_force_cursor_colour(mut tty: *mut tty, mut ccolour: *const libc::c_char) {
    if *ccolour as libc::c_int == '\u{0}' as i32 {
        tty_putcode(tty, tty_code_code::CR);
    } else {
        tty_putcode_ptr1(tty, tty_code_code::CS, ccolour as *const libc::c_void);
    }
    free((*tty).ccolour as *mut libc::c_void);
    (*tty).ccolour = xstrdup(ccolour);
}
#[no_mangle]
pub unsafe extern "C" fn tty_update_mode(
    mut tty: *mut tty,
    mut mode: libc::c_int,
    mut s: *mut screen,
) {
    let mut c: *mut client = (*tty).client;
    let mut changed: libc::c_int = 0;
    if !s.is_null() && strcmp((*s).ccolour, (*tty).ccolour) != 0i32 {
        tty_force_cursor_colour(tty, (*s).ccolour);
    }
    if (*tty).flags & 0x1i32 != 0 {
        mode &= !(0x1i32)
    }
    changed = mode ^ (*tty).mode;
    if changed != 0i32 {
        log_debug(
            b"%s: update mode %x to %x\x00" as *const u8 as *const libc::c_char,
            (*c).name,
            (*tty).mode,
            mode,
        );
    }
    if changed & 0x80i32 != 0 {
        if tty_term_has((*tty).term, tty_code_code::CVVIS) != 0 {
            tty_putcode(tty, tty_code_code::CVVIS);
        } else {
            tty_putcode(tty, tty_code_code::CNORM);
        }
        changed |= 0x1i32
    }
    if changed & 0x1i32 != 0 {
        if mode & 0x1i32 != 0 {
            tty_putcode(tty, tty_code_code::CNORM);
        } else {
            tty_putcode(tty, tty_code_code::CIVIS);
        }
    }
    if !s.is_null() && (*tty).cstyle != (*s).cstyle {
        if tty_term_has((*tty).term, tty_code_code::SS) != 0 {
            if (*s).cstyle == 0u32 && tty_term_has((*tty).term, tty_code_code::SE) != 0 {
                tty_putcode(tty, tty_code_code::SE);
            } else {
                tty_putcode1(tty, tty_code_code::SS, (*s).cstyle as libc::c_int);
            }
        }
        (*tty).cstyle = (*s).cstyle
    }
    if changed & (0x20i32 | 0x40i32 | 0x1000i32) != 0
        && tty_term_has((*tty).term, tty_code_code::KMOUS) != 0
    {
        if mode & (0x20i32 | 0x40i32 | 0x1000i32) == 0i32 {
            tty_puts(tty, b"\x1b[?1006l\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x20i32 != 0 && !mode & 0x20i32 != 0 {
            tty_puts(tty, b"\x1b[?1000l\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x40i32 != 0 && !mode & 0x40i32 != 0 {
            tty_puts(tty, b"\x1b[?1002l\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x1000i32 != 0 && !mode & 0x1000i32 != 0 {
            tty_puts(tty, b"\x1b[?1003l\x00" as *const u8 as *const libc::c_char);
        }
        if mode & (0x20i32 | 0x40i32 | 0x1000i32) != 0 {
            tty_puts(tty, b"\x1b[?1006h\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x20i32 != 0 && mode & 0x20i32 != 0 {
            tty_puts(tty, b"\x1b[?1000h\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x40i32 != 0 && mode & 0x40i32 != 0 {
            tty_puts(tty, b"\x1b[?1002h\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x1000i32 != 0 && mode & 0x1000i32 != 0 {
            tty_puts(tty, b"\x1b[?1003h\x00" as *const u8 as *const libc::c_char);
        }
    }
    if changed & 0x400i32 != 0 {
        if mode & 0x400i32 != 0 {
            tty_putcode(tty, tty_code_code::ENBP);
        } else {
            tty_putcode(tty, tty_code_code::DSBP);
        }
    }
    (*tty).mode = mode;
}
unsafe extern "C" fn tty_emulate_repeat(
    mut tty: *mut tty,
    mut code: TtyCode,
    mut code1: TtyCode,
    mut n: u_int,
) {
    if tty_term_has((*tty).term, code) != 0 {
        tty_putcode1(tty, code, n as libc::c_int);
    } else {
        loop {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if !(fresh0 > 0u32) {
                break;
            }
            tty_putcode(tty, code1);
        }
    };
}
unsafe extern "C" fn tty_repeat_space(mut tty: *mut tty, mut n: u_int) {
    static mut s: [libc::c_char; 500] = [0; 500];
    if *s.as_mut_ptr() as libc::c_int != ' ' as i32 {
        memset(
            s.as_mut_ptr() as *mut libc::c_void,
            ' ' as i32,
            ::std::mem::size_of::<[libc::c_char; 500]>() as libc::c_ulong,
        );
    }
    while n as libc::c_ulong > ::std::mem::size_of::<[libc::c_char; 500]>() as libc::c_ulong {
        tty_putn(
            tty,
            s.as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 500]>() as libc::c_ulong,
            ::std::mem::size_of::<[libc::c_char; 500]>() as u_int,
        );
        n = (n as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<[libc::c_char; 500]>() as libc::c_ulong)
            as u_int
    }
    if n != 0u32 {
        tty_putn(tty, s.as_mut_ptr() as *const libc::c_void, n as size_t, n);
    };
}
/* Is this window bigger than the terminal? */
#[no_mangle]
pub unsafe extern "C" fn tty_window_bigger(mut tty: *mut tty) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    return ((*tty).sx < (*w).sx || (*tty).sy.wrapping_sub(status_line_size(c)) < (*w).sy)
        as libc::c_int;
}
/* What offset should this window be drawn at? */
#[no_mangle]
pub unsafe extern "C" fn tty_window_offset(
    mut tty: *mut tty,
    mut ox: *mut u_int,
    mut oy: *mut u_int,
    mut sx: *mut u_int,
    mut sy: *mut u_int,
) -> libc::c_int {
    *ox = (*tty).oox;
    *oy = (*tty).ooy;
    *sx = (*tty).osx;
    *sy = (*tty).osy;
    return (*tty).oflag;
}
/* What offset should this window be drawn at? */
unsafe extern "C" fn tty_window_offset1(
    mut tty: *mut tty,
    mut ox: *mut u_int,
    mut oy: *mut u_int,
    mut sx: *mut u_int,
    mut sy: *mut u_int,
) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = server_client_get_pane(c);
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut lines: u_int = 0;
    lines = status_line_size(c);
    if (*tty).sx >= (*w).sx && (*tty).sy.wrapping_sub(lines) >= (*w).sy {
        *ox = 0u32;
        *oy = 0u32;
        *sx = (*w).sx;
        *sy = (*w).sy;
        (*c).pan_window = 0 as *mut libc::c_void;
        return 0i32;
    }
    *sx = (*tty).sx;
    *sy = (*tty).sy.wrapping_sub(lines);
    if (*c).pan_window == w as *mut libc::c_void {
        if *sx >= (*w).sx {
            (*c).pan_ox = 0u32
        } else if (*c).pan_ox.wrapping_add(*sx) > (*w).sx {
            (*c).pan_ox = (*w).sx.wrapping_sub(*sx)
        }
        *ox = (*c).pan_ox;
        if *sy >= (*w).sy {
            (*c).pan_oy = 0u32
        } else if (*c).pan_oy.wrapping_add(*sy) > (*w).sy {
            (*c).pan_oy = (*w).sy.wrapping_sub(*sy)
        }
        *oy = (*c).pan_oy;
        return 1i32;
    }
    if !(*(*wp).screen).mode & 0x1i32 != 0 {
        *ox = 0u32;
        *oy = 0u32
    } else {
        cx = (*wp).xoff.wrapping_add((*(*wp).screen).cx);
        cy = (*wp).yoff.wrapping_add((*(*wp).screen).cy);
        if cx < *sx {
            *ox = 0u32
        } else if cx > (*w).sx.wrapping_sub(*sx) {
            *ox = (*w).sx.wrapping_sub(*sx)
        } else {
            *ox = cx.wrapping_sub((*sx).wrapping_div(2u32))
        }
        if cy < *sy {
            *oy = 0u32
        } else if cy > (*w).sy.wrapping_sub(*sy) {
            *oy = (*w).sy.wrapping_sub(*sy)
        } else {
            *oy = cy.wrapping_sub((*sy).wrapping_div(2u32))
        }
    }
    (*c).pan_window = 0 as *mut libc::c_void;
    return 1i32;
}
/* Update stored offsets for a window and redraw if necessary. */
#[no_mangle]
pub unsafe extern "C" fn tty_update_window_offset(mut w: *mut window) {
    let mut c: *mut client = 0 as *mut client;
    c = clients.tqh_first;
    while !c.is_null() {
        if !(*c).session.is_null() && (*(*(*c).session).curw).window == w {
            tty_update_client_offset(c);
        }
        c = (*c).entry.tqe_next
    }
}
/* Update stored offsets for a client and redraw if necessary. */
#[no_mangle]
pub unsafe extern "C" fn tty_update_client_offset(mut c: *mut client) {
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    if !(*c).flags & 0x1u64 != 0 {
        return;
    }
    (*c).tty.oflag = tty_window_offset1(&mut (*c).tty, &mut ox, &mut oy, &mut sx, &mut sy);
    if ox == (*c).tty.oox && oy == (*c).tty.ooy && sx == (*c).tty.osx && sy == (*c).tty.osy {
        return;
    }
    log_debug(
        b"%s: %s offset has changed (%u,%u %ux%u -> %u,%u %ux%u)\x00" as *const u8
            as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"tty_update_client_offset\x00"))
            .as_ptr(),
        (*c).name,
        (*c).tty.oox,
        (*c).tty.ooy,
        (*c).tty.osx,
        (*c).tty.osy,
        ox,
        oy,
        sx,
        sy,
    );
    (*c).tty.oox = ox;
    (*c).tty.ooy = oy;
    (*c).tty.osx = sx;
    (*c).tty.osy = sy;
    (*c).flags |= (0x8i32 | 0x10i32) as libc::c_ulong;
}
/* Get a palette entry. */
unsafe extern "C" fn tty_get_palette(
    mut palette: *mut libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut new: libc::c_int = 0;
    if palette.is_null() {
        return -(1i32);
    }
    new = -(1i32);
    if c < 8i32 {
        new = *palette.offset(c as isize)
    } else if c >= 90i32 && c <= 97i32 {
        new = *palette.offset((8i32 + c - 90i32) as isize)
    } else if c & 0x1000000i32 != 0 {
        new = *palette.offset((c & !(0x1000000i32)) as isize)
    }
    if new == 0i32 {
        return -(1i32);
    }
    return new;
}
/*
 * Is the region large enough to be worth redrawing once later rather than
 * probably several times now? Currently yes if it is more than 50% of the
 * pane.
 */
unsafe extern "C" fn tty_large_region(mut _tty: *mut tty, mut ctx: *const tty_ctx) -> libc::c_int {
    return ((*ctx).orlower.wrapping_sub((*ctx).orupper) >= (*ctx).sy.wrapping_div(2u32))
        as libc::c_int;
}
/*
 * Return if BCE is needed but the terminal doesn't have it - it'll need to be
 * emulated.
 */
unsafe extern "C" fn tty_fake_bce(
    mut tty: *const tty,
    mut gc: *const GridCell,
    mut bg: u_int,
) -> libc::c_int {
    if tty_term_flag((*tty).term, tty_code_code::BCE) != 0 {
        return 0i32;
    }
    if !(bg == 8u32 || bg == 9u32) || !((*gc).bg == 8i32 || (*gc).bg == 9i32) {
        return 1i32;
    }
    return 0i32;
}
/*
 * Redraw scroll region using data from screen (already updated). Used when
 * CSR not supported, or window is a pane that doesn't take up the full
 * width of the terminal.
 */
unsafe extern "C" fn tty_redraw_region(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    let mut c: *mut client = (*tty).client;
    let mut i: u_int = 0;
    /*
     * If region is large, schedule a redraw. In most cases this is likely
     * to be followed by some more scrolling.
     */
    if tty_large_region(tty, ctx) != 0 {
        log_debug(
            b"%s: %s large redraw\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"tty_redraw_region\x00"))
                .as_ptr(),
            (*c).name,
        );
        (*ctx).redraw_cb.expect("non-null function pointer")(ctx);
        return;
    }
    if (*ctx).ocy < (*ctx).orupper || (*ctx).ocy > (*ctx).orlower {
        i = (*ctx).ocy;
        while i < (*ctx).sy {
            tty_draw_pane(tty, ctx, i);
            i = i.wrapping_add(1)
        }
    } else {
        i = (*ctx).orupper;
        while i <= (*ctx).orlower {
            tty_draw_pane(tty, ctx, i);
            i = i.wrapping_add(1)
        }
    };
}
/* Is this position visible in the pane? */
unsafe extern "C" fn tty_is_visible(
    mut _tty: *mut tty,
    mut ctx: *const tty_ctx,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut ny: u_int,
) -> libc::c_int {
    let mut xoff: u_int = (*ctx).rxoff.wrapping_add(px);
    let mut yoff: u_int = (*ctx).ryoff.wrapping_add(py);
    if (*ctx).bigger == 0 {
        return 1i32;
    }
    if xoff.wrapping_add(nx) <= (*ctx).wox
        || xoff >= (*ctx).wox.wrapping_add((*ctx).wsx)
        || yoff.wrapping_add(ny) <= (*ctx).woy
        || yoff >= (*ctx).woy.wrapping_add((*ctx).wsy)
    {
        return 0i32;
    }
    return 1i32;
}
/* Clamp line position to visible part of pane. */
unsafe extern "C" fn tty_clamp_line(
    mut tty: *mut tty,
    mut ctx: *const tty_ctx,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut i: *mut u_int,
    mut x: *mut u_int,
    mut rx: *mut u_int,
    mut ry: *mut u_int,
) -> libc::c_int {
    let mut xoff: u_int = (*ctx).rxoff.wrapping_add(px);
    if tty_is_visible(tty, ctx, px, py, nx, 1u32) == 0 {
        return 0i32;
    }
    *ry = (*ctx).yoff.wrapping_add(py).wrapping_sub((*ctx).woy);
    if xoff >= (*ctx).wox && xoff.wrapping_add(nx) <= (*ctx).wox.wrapping_add((*ctx).wsx) {
        /* All visible. */
        *i = 0u32;
        *x = (*ctx).xoff.wrapping_add(px).wrapping_sub((*ctx).wox);
        *rx = nx
    } else if xoff < (*ctx).wox && xoff.wrapping_add(nx) > (*ctx).wox.wrapping_add((*ctx).wsx) {
        /* Both left and right not visible. */
        *i = (*ctx).wox;
        *x = 0u32;
        *rx = (*ctx).wsx
    } else if xoff < (*ctx).wox {
        /* Left not visible. */
        *i = (*ctx).wox.wrapping_sub((*ctx).xoff.wrapping_add(px));
        *x = 0u32;
        *rx = nx.wrapping_sub(*i)
    } else {
        /* Right not visible. */
        *i = 0u32;
        *x = (*ctx).xoff.wrapping_add(px).wrapping_sub((*ctx).wox);
        *rx = (*ctx).wsx.wrapping_sub(*x)
    }
    if *rx > nx {
        fatalx(
            b"%s: x too big, %u > %u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"tty_clamp_line\x00"))
                .as_ptr(),
            *rx,
            nx,
        );
    }
    return 1i32;
}
/* Clear a line. */
unsafe extern "C" fn tty_clear_line(
    mut tty: *mut tty,
    mut defaults: *const GridCell,
    mut py: u_int,
    mut px: u_int,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut c: *mut client = (*tty).client;
    log_debug(
        b"%s: %s, %u at %u,%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"tty_clear_line\x00")).as_ptr(),
        (*c).name,
        nx,
        px,
        py,
    );
    /* Nothing to clear. */
    if nx == 0u32 {
        return;
    }
    /* If genuine BCE is available, can try escape sequences. */
    if tty_fake_bce(tty, defaults, bg) == 0 {
        /* Off the end of the line, use EL if available. */
        if px.wrapping_add(nx) >= (*tty).sx && tty_term_has((*tty).term, tty_code_code::EL) != 0 {
            tty_cursor(tty, px, py);
            tty_putcode(tty, tty_code_code::EL);
            return;
        }
        /* At the start of the line. Use EL1. */
        if px == 0u32 && tty_term_has((*tty).term, tty_code_code::EL1) != 0 {
            tty_cursor(tty, px.wrapping_add(nx).wrapping_sub(1u32), py);
            tty_putcode(tty, tty_code_code::EL1);
            return;
        }
        /* Section of line. Use ECH if possible. */
        if tty_term_has((*tty).term, tty_code_code::ECH) != 0 {
            tty_cursor(tty, px, py);
            tty_putcode1(tty, tty_code_code::ECH, nx as libc::c_int);
            return;
        }
    }
    /* Couldn't use an escape sequence, use spaces. */
    tty_cursor(tty, px, py);
    tty_repeat_space(tty, nx);
}
/* Clear a line, adjusting to visible part of pane. */
unsafe extern "C" fn tty_clear_pane_line(
    mut tty: *mut tty,
    mut ctx: *const tty_ctx,
    mut py: u_int,
    mut px: u_int,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut c: *mut client = (*tty).client;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut rx: u_int = 0;
    let mut ry: u_int = 0;
    log_debug(
        b"%s: %s, %u at %u,%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"tty_clear_pane_line\x00"))
            .as_ptr(),
        (*c).name,
        nx,
        px,
        py,
    );
    if tty_clamp_line(tty, ctx, px, py, nx, &mut i, &mut x, &mut rx, &mut ry) != 0 {
        tty_clear_line(tty, &(*ctx).defaults, ry, x, rx, bg);
    };
}
/* Clamp area position to visible part of pane. */
unsafe extern "C" fn tty_clamp_area(
    mut tty: *mut tty,
    mut ctx: *const tty_ctx,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut ny: u_int,
    mut i: *mut u_int,
    mut j: *mut u_int,
    mut x: *mut u_int,
    mut y: *mut u_int,
    mut rx: *mut u_int,
    mut ry: *mut u_int,
) -> libc::c_int {
    let mut xoff: u_int = (*ctx).rxoff.wrapping_add(px);
    let mut yoff: u_int = (*ctx).ryoff.wrapping_add(py);
    if tty_is_visible(tty, ctx, px, py, nx, ny) == 0 {
        return 0i32;
    }
    if xoff >= (*ctx).wox && xoff.wrapping_add(nx) <= (*ctx).wox.wrapping_add((*ctx).wsx) {
        /* All visible. */
        *i = 0u32;
        *x = (*ctx).xoff.wrapping_add(px).wrapping_sub((*ctx).wox);
        *rx = nx
    } else if xoff < (*ctx).wox && xoff.wrapping_add(nx) > (*ctx).wox.wrapping_add((*ctx).wsx) {
        /* Both left and right not visible. */
        *i = (*ctx).wox;
        *x = 0u32;
        *rx = (*ctx).wsx
    } else if xoff < (*ctx).wox {
        /* Left not visible. */
        *i = (*ctx).wox.wrapping_sub((*ctx).xoff.wrapping_add(px));
        *x = 0u32;
        *rx = nx.wrapping_sub(*i)
    } else {
        /* Right not visible. */
        *i = 0u32;
        *x = (*ctx).xoff.wrapping_add(px).wrapping_sub((*ctx).wox);
        *rx = (*ctx).wsx.wrapping_sub(*x)
    }
    if *rx > nx {
        fatalx(
            b"%s: x too big, %u > %u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"tty_clamp_area\x00"))
                .as_ptr(),
            *rx,
            nx,
        );
    }
    if yoff >= (*ctx).woy && yoff.wrapping_add(ny) <= (*ctx).woy.wrapping_add((*ctx).wsy) {
        /* All visible. */
        *j = 0u32;
        *y = (*ctx).yoff.wrapping_add(py).wrapping_sub((*ctx).woy);
        *ry = ny
    } else if yoff < (*ctx).woy && yoff.wrapping_add(ny) > (*ctx).woy.wrapping_add((*ctx).wsy) {
        /* Both top and bottom not visible. */
        *j = (*ctx).woy;
        *y = 0u32;
        *ry = (*ctx).wsy
    } else if yoff < (*ctx).woy {
        /* Top not visible. */
        *j = (*ctx).woy.wrapping_sub((*ctx).yoff.wrapping_add(py));
        *y = 0u32;
        *ry = ny.wrapping_sub(*j)
    } else {
        /* Bottom not visible. */
        *j = 0u32;
        *y = (*ctx).yoff.wrapping_add(py).wrapping_sub((*ctx).woy);
        *ry = (*ctx).wsy.wrapping_sub(*y)
    }
    if *ry > ny {
        fatalx(
            b"%s: y too big, %u > %u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"tty_clamp_area\x00"))
                .as_ptr(),
            *ry,
            ny,
        );
    }
    return 1i32;
}
/* Clear an area, adjusting to visible part of pane. */
unsafe extern "C" fn tty_clear_area(
    mut tty: *mut tty,
    mut defaults: *const GridCell,
    mut py: u_int,
    mut ny: u_int,
    mut px: u_int,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut c: *mut client = (*tty).client;
    let mut yy: u_int = 0;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    log_debug(
        b"%s: %s, %u,%u at %u,%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"tty_clear_area\x00")).as_ptr(),
        (*c).name,
        nx,
        ny,
        px,
        py,
    );
    /* Nothing to clear. */
    if nx == 0u32 || ny == 0u32 {
        return;
    }
    /* If genuine BCE is available, can try escape sequences. */
    if tty_fake_bce(tty, defaults, bg) == 0 {
        /* Use ED if clearing off the bottom of the terminal. */
        if px == 0u32
            && px.wrapping_add(nx) >= (*tty).sx
            && py.wrapping_add(ny) >= (*tty).sy
            && tty_term_has((*tty).term, tty_code_code::ED) != 0
        {
            tty_cursor(tty, 0u32, py);
            tty_putcode(tty, tty_code_code::ED);
            return;
        }
        /*
         * On VT420 compatible terminals we can use DECFRA if the
         * background colour isn't default (because it doesn't work
         * after SGR 0).
         */
        if (*(*tty).term).flags & 0x8i32 != 0 && !(bg == 8u32 || bg == 9u32) {
            xsnprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                b"\x1b[32;%u;%u;%u;%u$x\x00" as *const u8 as *const libc::c_char,
                py.wrapping_add(1u32),
                px.wrapping_add(1u32),
                py.wrapping_add(ny),
                px.wrapping_add(nx),
            );
            tty_puts(tty, tmp.as_mut_ptr());
            return;
        }
        /* Full lines can be scrolled away to clear them. */
        if px == 0u32
            && px.wrapping_add(nx) >= (*tty).sx
            && ny > 2u32
            && tty_term_has((*tty).term, tty_code_code::CSR) != 0
            && tty_term_has((*tty).term, tty_code_code::INDN) != 0
        {
            tty_region(tty, py, py.wrapping_add(ny).wrapping_sub(1u32));
            tty_margin_off(tty);
            tty_putcode1(tty, tty_code_code::INDN, ny as libc::c_int);
            return;
        }
        /*
         * If margins are supported, can just scroll the area off to
         * clear it.
         */
        if nx > 2u32
            && ny > 2u32
            && tty_term_has((*tty).term, tty_code_code::CSR) != 0
            && (*(*tty).term).flags & 0x4i32 != 0
            && tty_term_has((*tty).term, tty_code_code::INDN) != 0
        {
            tty_region(tty, py, py.wrapping_add(ny).wrapping_sub(1u32));
            tty_margin(tty, px, px.wrapping_add(nx).wrapping_sub(1u32));
            tty_putcode1(tty, tty_code_code::INDN, ny as libc::c_int);
            return;
        }
    }
    /* Couldn't use an escape sequence, loop over the lines. */
    yy = py;
    while yy < py.wrapping_add(ny) {
        tty_clear_line(tty, defaults, yy, px, nx, bg);
        yy = yy.wrapping_add(1)
    }
}
/* Clear an area in a pane. */
unsafe extern "C" fn tty_clear_pane_area(
    mut tty: *mut tty,
    mut ctx: *const tty_ctx,
    mut py: u_int,
    mut ny: u_int,
    mut px: u_int,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut rx: u_int = 0;
    let mut ry: u_int = 0;
    if tty_clamp_area(
        tty, ctx, px, py, nx, ny, &mut i, &mut j, &mut x, &mut y, &mut rx, &mut ry,
    ) != 0
    {
        tty_clear_area(tty, &(*ctx).defaults, y, ry, x, rx, bg);
    };
}
unsafe extern "C" fn tty_draw_pane(mut tty: *mut tty, mut ctx: *const tty_ctx, mut py: u_int) {
    let mut s: *mut screen = (*ctx).s;
    let mut nx: u_int = (*ctx).sx;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut rx: u_int = 0;
    let mut ry: u_int = 0;
    log_debug(
        b"%s: %s %u %d\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"tty_draw_pane\x00")).as_ptr(),
        (*(*tty).client).name,
        py,
        (*ctx).bigger,
    );
    if (*ctx).bigger == 0 {
        tty_draw_line(
            tty,
            s,
            0u32,
            py,
            nx,
            (*ctx).xoff,
            (*ctx).yoff.wrapping_add(py),
            &(*ctx).defaults,
            (*ctx).palette,
        );
        return;
    }
    if tty_clamp_line(tty, ctx, 0u32, py, nx, &mut i, &mut x, &mut rx, &mut ry) != 0 {
        tty_draw_line(tty, s, i, py, rx, x, ry, &(*ctx).defaults, (*ctx).palette);
    };
}
unsafe extern "C" fn tty_check_codeset(
    mut tty: *mut tty,
    mut gc: *const GridCell,
) -> *const GridCell {
    static mut new: GridCell = GridCell {
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
    let mut c: libc::c_int = 0;
    /* Characters less than 0x7f are always fine, no matter what. */
    if (*gc).data.size as libc::c_int == 1i32
        && (*(*gc).data.data.as_ptr() as libc::c_int) < 0x7fi32
    {
        return gc;
    }
    /* UTF-8 terminal and a UTF-8 character - fine. */
    if (*(*tty).client).flags & 0x10000u64 != 0 {
        return gc;
    }
    memcpy(
        &mut new as *mut GridCell as *mut libc::c_void,
        gc as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    /* See if this can be mapped to an ACS character. */
    c = tty_acs_reverse_get(
        tty,
        (*gc).data.data.as_ptr() as *const libc::c_char,
        (*gc).data.size as size_t,
    );
    if c != -(1i32) {
        utf8_set(&mut new.data, c as u_char);
        new.attr = (new.attr as libc::c_int | 0x80i32) as u_short;
        return &mut new;
    }
    /* Replace by the right number of underscores. */
    new.data.size = (*gc).data.width;
    if new.data.size as libc::c_int > 21i32 {
        new.data.size = 21u8
    }
    memset(
        new.data.data.as_mut_ptr() as *mut libc::c_void,
        '_' as i32,
        new.data.size as libc::c_ulong,
    );
    return &mut new;
}
unsafe extern "C" fn tty_check_overlay(
    mut tty: *mut tty,
    mut px: u_int,
    mut py: u_int,
) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    if (*c).overlay_check.is_none() {
        return 1i32;
    }
    return (*c).overlay_check.expect("non-null function pointer")(c, px, py);
}
#[no_mangle]
pub unsafe extern "C" fn tty_draw_line(
    mut tty: *mut tty,
    mut s: *mut screen,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut atx: u_int,
    mut aty: u_int,
    mut defaults: *const GridCell,
    mut palette: *mut libc::c_int,
) {
    let mut gd: *mut Grid = (*s).grid;
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
    let mut last: GridCell = GridCell {
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
    let mut gcp: *const GridCell = 0 as *const GridCell;
    let mut gl: *mut GridLine = 0 as *mut GridLine;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut ux: u_int = 0;
    let mut sx: u_int = 0;
    let mut width: u_int = 0;
    let mut flags: libc::c_int = 0;
    let mut cleared: libc::c_int = 0i32;
    let mut wrapped: libc::c_int = 0i32;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut len: size_t = 0;
    let mut cellsize: u_int = 0;
    log_debug(
        b"%s: px=%u py=%u nx=%u atx=%u aty=%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"tty_draw_line\x00")).as_ptr(),
        px,
        py,
        nx,
        atx,
        aty,
    );
    /*
     * py is the line in the screen to draw.
     * px is the start x and nx is the width to draw.
     * atx,aty is the line on the terminal to draw it.
     */
    flags = (*tty).flags & 0x1i32;
    (*tty).flags |= 0x1i32;
    tty_update_mode(tty, (*tty).mode, s);
    tty_region_off(tty);
    tty_margin_off(tty);
    /*
     * Clamp the width to cellsize - note this is not cellused, because
     * there may be empty background cells after it (from BCE).
     */
    sx = (*(*s).grid).sx;
    if nx > sx {
        nx = sx
    }
    cellsize = (*grid_get_line(gd, (*gd).hsize.wrapping_add(py))).cellsize;
    if sx > cellsize {
        sx = cellsize
    }
    if sx > (*tty).sx {
        sx = (*tty).sx
    }
    if sx > nx {
        sx = nx
    }
    ux = 0u32;
    if py == 0u32 {
        gl = 0 as *mut GridLine
    } else {
        gl = grid_get_line(gd, (*gd).hsize.wrapping_add(py).wrapping_sub(1u32))
    }
    if gl.is_null()
        || !(*gl).flags & 0x1i32 != 0
        || atx != 0u32
        || (*tty).cx < (*tty).sx
        || nx < (*tty).sx
    {
        if nx < (*tty).sx
            && atx == 0u32
            && px.wrapping_add(sx) != nx
            && tty_term_has((*tty).term, tty_code_code::EL1) != 0
            && tty_fake_bce(tty, defaults, 8u32) == 0
        {
            tty_default_attributes(tty, defaults, palette, 8u32);
            tty_cursor(tty, nx.wrapping_sub(1u32), aty);
            tty_putcode(tty, tty_code_code::EL1);
            cleared = 1i32
        }
    } else {
        log_debug(
            b"%s: wrapped line %u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"tty_draw_line\x00"))
                .as_ptr(),
            aty,
        );
        wrapped = 1i32
    }
    memcpy(
        &mut last as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    len = 0u64;
    width = 0u32;
    i = 0u32;
    while i < sx {
        grid_view_get_cell(gd, px.wrapping_add(i), py, &mut gc);
        gcp = tty_check_codeset(tty, &mut gc);
        if len != 0u64
            && (tty_check_overlay(tty, atx.wrapping_add(ux).wrapping_add(width), aty) == 0
                || (*gcp).attr as libc::c_int & 0x80i32 != 0
                || (*gcp).flags as libc::c_int != last.flags as libc::c_int
                || (*gcp).attr as libc::c_int != last.attr as libc::c_int
                || (*gcp).fg != last.fg
                || (*gcp).bg != last.bg
                || (*gcp).us != last.us
                || ux
                    .wrapping_add(width)
                    .wrapping_add((*gcp).data.width as libc::c_uint)
                    > nx
                || (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                    .wrapping_sub(len)
                    < (*gcp).data.size as libc::c_ulong)
        {
            tty_attributes(tty, &mut last, defaults, palette);
            if last.flags as libc::c_int & 0x40i32 != 0 {
                log_debug(
                    b"%s: %zu cleared\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                        b"tty_draw_line\x00",
                    ))
                    .as_ptr(),
                    len,
                );
                tty_clear_line(
                    tty,
                    defaults,
                    aty,
                    atx.wrapping_add(ux),
                    width,
                    last.bg as u_int,
                );
            } else {
                if wrapped == 0 || atx != 0u32 || ux != 0u32 {
                    tty_cursor(tty, atx.wrapping_add(ux), aty);
                }
                tty_putn(tty, buf.as_mut_ptr() as *const libc::c_void, len, width);
            }
            ux = (ux).wrapping_add(width);
            len = 0u64;
            width = 0u32;
            wrapped = 0i32
        }
        if (*gcp).flags as libc::c_int & 0x10i32 != 0 {
            screen_select_cell(s, &mut last, gcp);
        } else {
            memcpy(
                &mut last as *mut GridCell as *mut libc::c_void,
                gcp as *const libc::c_void,
                ::std::mem::size_of::<GridCell>() as libc::c_ulong,
            );
        }
        if tty_check_overlay(tty, atx.wrapping_add(ux), aty) == 0 {
            if !((*gcp).flags as libc::c_int) & 0x4i32 != 0 {
                ux = (ux).wrapping_add((*gcp).data.width as libc::c_uint)
            }
        } else if ux.wrapping_add((*gcp).data.width as libc::c_uint) > nx {
            tty_attributes(tty, &mut last, defaults, palette);
            tty_cursor(tty, atx.wrapping_add(ux), aty);
            j = 0u32;
            while j < (*gcp).data.width as libc::c_uint {
                if ux.wrapping_add(j) > nx {
                    break;
                }
                tty_putc(tty, ' ' as u_char);
                ux = ux.wrapping_add(1);
                j = j.wrapping_add(1)
            }
        } else if (*gcp).attr as libc::c_int & 0x80i32 != 0 {
            tty_attributes(tty, &mut last, defaults, palette);
            tty_cursor(tty, atx.wrapping_add(ux), aty);
            j = 0u32;
            while j < (*gcp).data.size as libc::c_uint {
                tty_putc(tty, (*gcp).data.data[j as usize]);
                j = j.wrapping_add(1)
            }
            ux = (ux).wrapping_add((*gcp).data.width as libc::c_uint)
        } else if !((*gcp).flags as libc::c_int) & 0x4i32 != 0 {
            memcpy(
                buf.as_mut_ptr().offset(len as isize) as *mut libc::c_void,
                (*gcp).data.data.as_ptr() as *const libc::c_void,
                (*gcp).data.size as libc::c_ulong,
            );
            len = (len).wrapping_add((*gcp).data.size as libc::c_ulong);
            width = (width).wrapping_add((*gcp).data.width as libc::c_uint)
        }
        i = i.wrapping_add(1)
    }
    if len != 0u64 && (!(last.flags as libc::c_int) & 0x40i32 != 0 || last.bg != 8i32) {
        tty_attributes(tty, &mut last, defaults, palette);
        if last.flags as libc::c_int & 0x40i32 != 0 {
            log_debug(
                b"%s: %zu cleared (end)\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"tty_draw_line\x00"))
                    .as_ptr(),
                len,
            );
            tty_clear_line(
                tty,
                defaults,
                aty,
                atx.wrapping_add(ux),
                width,
                last.bg as u_int,
            );
        } else {
            if wrapped == 0 || atx != 0u32 || ux != 0u32 {
                tty_cursor(tty, atx.wrapping_add(ux), aty);
            }
            tty_putn(tty, buf.as_mut_ptr() as *const libc::c_void, len, width);
        }
        ux = (ux).wrapping_add(width)
    }
    if cleared == 0 && ux < nx {
        log_debug(
            b"%s: %u to end of line (%zu cleared)\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"tty_draw_line\x00"))
                .as_ptr(),
            nx.wrapping_sub(ux),
            len,
        );
        tty_default_attributes(tty, defaults, palette, 8u32);
        tty_clear_line(
            tty,
            defaults,
            aty,
            atx.wrapping_add(ux),
            nx.wrapping_sub(ux),
            8u32,
        );
    }
    (*tty).flags = (*tty).flags & !(0x1i32) | flags;
    tty_update_mode(tty, (*tty).mode, s);
}
#[no_mangle]
pub unsafe extern "C" fn tty_sync_start(mut tty: *mut tty) {
    if (*tty).flags & 0x80i32 != 0 {
        return;
    }
    if (*tty).flags & 0x400i32 != 0 {
        return;
    }
    (*tty).flags |= 0x400i32;
    if tty_term_has((*tty).term, tty_code_code::SYNC) != 0 {
        log_debug(
            b"%s sync start\x00" as *const u8 as *const libc::c_char,
            (*(*tty).client).name,
        );
        tty_putcode1(tty, tty_code_code::SYNC, 1i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_sync_end(mut tty: *mut tty) {
    if (*tty).flags & 0x80i32 != 0 {
        return;
    }
    if !(*tty).flags & 0x400i32 != 0 {
        return;
    }
    (*tty).flags &= !(0x400i32);
    if tty_term_has((*tty).term, tty_code_code::SYNC) != 0 {
        log_debug(
            b"%s sync end\x00" as *const u8 as *const libc::c_char,
            (*(*tty).client).name,
        );
        tty_putcode1(tty, tty_code_code::SYNC, 2i32);
    };
}
unsafe extern "C" fn tty_client_ready(mut c: *mut client) -> libc::c_int {
    if (*c).session.is_null() || (*c).tty.term.is_null() {
        return 0i32;
    }
    if (*c).flags & (0x8i32 | 0x40i32) as libc::c_ulong != 0 {
        return 0i32;
    }
    if (*c).tty.flags & 0x2i32 != 0 {
        return 0i32;
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn tty_write(
    mut cmdfn: Option<unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()>,
    mut ctx: *mut tty_ctx,
) {
    let mut c: *mut client = 0 as *mut client;
    let mut state: libc::c_int = 0;
    if (*ctx).set_client_cb.is_none() {
        return;
    }
    c = clients.tqh_first;
    while !c.is_null() {
        if !(tty_client_ready(c) == 0) {
            state = (*ctx).set_client_cb.expect("non-null function pointer")(ctx, c);
            if state == -(1i32) {
                break;
            }
            if !(state == 0i32) {
                cmdfn.expect("non-null function pointer")(&mut (*c).tty, ctx);
            }
        }
        c = (*c).entry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_insertcharacter(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    if (*ctx).bigger != 0
        || !((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx)
        || tty_fake_bce(tty, &(*ctx).defaults, (*ctx).bg) != 0
        || tty_term_has((*tty).term, tty_code_code::ICH) == 0
            && tty_term_has((*tty).term, tty_code_code::ICH1) == 0
    {
        tty_draw_pane(tty, ctx, (*ctx).ocy);
        return;
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_emulate_repeat(tty, tty_code_code::ICH, tty_code_code::ICH1, (*ctx).num);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_deletecharacter(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    if (*ctx).bigger != 0
        || !((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx)
        || tty_fake_bce(tty, &(*ctx).defaults, (*ctx).bg) != 0
        || tty_term_has((*tty).term, tty_code_code::DCH) == 0
            && tty_term_has((*tty).term, tty_code_code::DCH1) == 0
    {
        tty_draw_pane(tty, ctx, (*ctx).ocy);
        return;
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_emulate_repeat(tty, tty_code_code::DCH, tty_code_code::DCH1, (*ctx).num);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearcharacter(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    if (*ctx).bigger != 0 {
        tty_draw_pane(tty, ctx, (*ctx).ocy);
        return;
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    if tty_term_has((*tty).term, tty_code_code::ECH) != 0
        && tty_fake_bce(tty, &(*ctx).defaults, 8u32) == 0
    {
        tty_putcode1(tty, tty_code_code::ECH, (*ctx).num as libc::c_int);
    } else {
        tty_repeat_space(tty, (*ctx).num);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_insertline(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    if (*ctx).bigger != 0
        || !((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx)
        || tty_fake_bce(tty, &(*ctx).defaults, (*ctx).bg) != 0
        || tty_term_has((*tty).term, tty_code_code::CSR) == 0
        || tty_term_has((*tty).term, tty_code_code::IL1) == 0
        || (*ctx).sx == 1u32
        || (*ctx).sy == 1u32
    {
        tty_redraw_region(tty, ctx);
        return;
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_off(tty);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_emulate_repeat(tty, tty_code_code::IL, tty_code_code::IL1, (*ctx).num);
    (*tty).cy = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*tty).cx = (*tty).cy;
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_deleteline(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    if (*ctx).bigger != 0
        || !((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx)
        || tty_fake_bce(tty, &(*ctx).defaults, (*ctx).bg) != 0
        || tty_term_has((*tty).term, tty_code_code::CSR) == 0
        || tty_term_has((*tty).term, tty_code_code::DL1) == 0
        || (*ctx).sx == 1u32
        || (*ctx).sy == 1u32
    {
        tty_redraw_region(tty, ctx);
        return;
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_off(tty);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_emulate_repeat(tty, tty_code_code::DL, tty_code_code::DL1, (*ctx).num);
    (*tty).cy = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*tty).cx = (*tty).cy;
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearline(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_clear_pane_line(tty, ctx, (*ctx).ocy, 0u32, (*ctx).sx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearendofline(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    let mut nx: u_int = (*ctx).sx.wrapping_sub((*ctx).ocx);
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_clear_pane_line(tty, ctx, (*ctx).ocy, (*ctx).ocx, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearstartofline(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_clear_pane_line(
        tty,
        ctx,
        (*ctx).ocy,
        0u32,
        (*ctx).ocx.wrapping_add(1u32),
        (*ctx).bg,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_reverseindex(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    if (*ctx).ocy != (*ctx).orupper {
        return;
    }
    if (*ctx).bigger != 0
        || !((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx) && (*(*tty).term).flags & 0x4i32 == 0
        || tty_fake_bce(tty, &(*ctx).defaults, 8u32) != 0
        || tty_term_has((*tty).term, tty_code_code::CSR) == 0
        || tty_term_has((*tty).term, tty_code_code::RI) == 0
            && tty_term_has((*tty).term, tty_code_code::RIN) == 0
        || (*ctx).sx == 1u32
        || (*ctx).sy == 1u32
    {
        tty_redraw_region(tty, ctx);
        return;
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_pane(tty, ctx);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).orupper);
    if tty_term_has((*tty).term, tty_code_code::RI) != 0 {
        tty_putcode(tty, tty_code_code::RI);
    } else {
        tty_putcode1(tty, tty_code_code::RIN, 1i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_linefeed(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    if (*ctx).ocy != (*ctx).orlower {
        return;
    }
    if (*ctx).bigger != 0
        || !((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx) && (*(*tty).term).flags & 0x4i32 == 0
        || tty_fake_bce(tty, &(*ctx).defaults, 8u32) != 0
        || tty_term_has((*tty).term, tty_code_code::CSR) == 0
        || (*ctx).sx == 1u32
        || (*ctx).sy == 1u32
    {
        tty_redraw_region(tty, ctx);
        return;
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_pane(tty, ctx);
    /*
     * If we want to wrap a pane while using margins, the cursor needs to
     * be exactly on the right of the region. If the cursor is entirely off
     * the edge - move it back to the right. Some terminals are funny about
     * this and insert extra spaces, so only use the right if margins are
     * enabled.
     */
    if (*ctx).xoff.wrapping_add((*ctx).ocx) > (*tty).rright {
        if (*(*tty).term).flags & 0x4i32 == 0 {
            tty_cursor(tty, 0u32, (*ctx).yoff.wrapping_add((*ctx).ocy)); /* storage for base64 */
        } else {
            tty_cursor(tty, (*tty).rright, (*ctx).yoff.wrapping_add((*ctx).ocy));
        }
    } else {
        tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    }
    tty_putc(tty, '\n' as u_char);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_scrollup(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    let mut i: u_int = 0;
    if (*ctx).bigger != 0
        || !((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx) && (*(*tty).term).flags & 0x4i32 == 0
        || tty_fake_bce(tty, &(*ctx).defaults, 8u32) != 0
        || tty_term_has((*tty).term, tty_code_code::CSR) == 0
        || (*ctx).sx == 1u32
        || (*ctx).sy == 1u32
    {
        tty_redraw_region(tty, ctx);
        return;
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_pane(tty, ctx);
    if (*ctx).num == 1u32 || tty_term_has((*tty).term, tty_code_code::INDN) == 0 {
        if (*(*tty).term).flags & 0x4i32 == 0 {
            tty_cursor(tty, 0u32, (*tty).rlower);
        } else {
            tty_cursor(tty, (*tty).rright, (*tty).rlower);
        }
        i = 0u32;
        while i < (*ctx).num {
            tty_putc(tty, '\n' as u_char);
            i = i.wrapping_add(1)
        }
    } else {
        if (*tty).cy == (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32) {
            tty_cursor(tty, 0u32, 0u32);
        } else {
            tty_cursor(tty, 0u32, (*tty).cy);
        }
        tty_putcode1(tty, tty_code_code::INDN, (*ctx).num as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_scrolldown(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    let mut i: u_int = 0;
    if (*ctx).bigger != 0
        || !((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx) && (*(*tty).term).flags & 0x4i32 == 0
        || tty_fake_bce(tty, &(*ctx).defaults, 8u32) != 0
        || tty_term_has((*tty).term, tty_code_code::CSR) == 0
        || tty_term_has((*tty).term, tty_code_code::RI) == 0
            && tty_term_has((*tty).term, tty_code_code::RIN) == 0
        || (*ctx).sx == 1u32
        || (*ctx).sy == 1u32
    {
        tty_redraw_region(tty, ctx);
        return;
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_pane(tty, ctx);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).orupper);
    if tty_term_has((*tty).term, tty_code_code::RIN) != 0 {
        tty_putcode1(tty, tty_code_code::RIN, (*ctx).num as libc::c_int);
    } else {
        i = 0u32;
        while i < (*ctx).num {
            tty_putcode(tty, tty_code_code::RI);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearendofscreen(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut nx: u_int = 0;
    let mut ny: u_int = 0;
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, 0u32, (*ctx).sy.wrapping_sub(1u32));
    tty_margin_off(tty);
    px = 0u32;
    nx = (*ctx).sx;
    py = (*ctx).ocy.wrapping_add(1u32);
    ny = (*ctx).sy.wrapping_sub((*ctx).ocy).wrapping_sub(1u32);
    tty_clear_pane_area(tty, ctx, py, ny, px, nx, (*ctx).bg);
    px = (*ctx).ocx;
    nx = (*ctx).sx.wrapping_sub((*ctx).ocx);
    py = (*ctx).ocy;
    tty_clear_pane_line(tty, ctx, py, px, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearstartofscreen(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut nx: u_int = 0;
    let mut ny: u_int = 0;
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, 0u32, (*ctx).sy.wrapping_sub(1u32));
    tty_margin_off(tty);
    px = 0u32;
    nx = (*ctx).sx;
    py = 0u32;
    ny = (*ctx).ocy;
    tty_clear_pane_area(tty, ctx, py, ny, px, nx, (*ctx).bg);
    px = 0u32;
    nx = (*ctx).ocx.wrapping_add(1u32);
    py = (*ctx).ocy;
    tty_clear_pane_line(tty, ctx, py, px, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearscreen(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut nx: u_int = 0;
    let mut ny: u_int = 0;
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, 0u32, (*ctx).sy.wrapping_sub(1u32));
    tty_margin_off(tty);
    px = 0u32;
    nx = (*ctx).sx;
    py = 0u32;
    ny = (*ctx).sy;
    tty_clear_pane_area(tty, ctx, py, ny, px, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_alignmenttest(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    if (*ctx).bigger != 0 {
        (*ctx).redraw_cb.expect("non-null function pointer")(ctx);
        return;
    }
    tty_attributes(tty, &grid_default_cell, &(*ctx).defaults, (*ctx).palette);
    tty_region_pane(tty, ctx, 0u32, (*ctx).sy.wrapping_sub(1u32));
    tty_margin_off(tty);
    j = 0u32;
    while j < (*ctx).sy {
        tty_cursor_pane(tty, ctx, 0u32, j);
        i = 0u32;
        while i < (*ctx).sx {
            tty_putc(tty, 'E' as u_char);
            i = i.wrapping_add(1)
        }
        j = j.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_cell(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    if tty_is_visible(tty, ctx, (*ctx).ocx, (*ctx).ocy, 1u32, 1u32) == 0 {
        return;
    }
    if (*ctx)
        .xoff
        .wrapping_add((*ctx).ocx)
        .wrapping_sub((*ctx).wox)
        > (*tty).sx.wrapping_sub(1u32)
        && (*ctx).ocy == (*ctx).orlower
        && ((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx)
    {
        tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    }
    tty_margin_off(tty);
    tty_cursor_pane_unless_wrap(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_cell(tty, (*ctx).cell, &(*ctx).defaults, (*ctx).palette);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_cells(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    if tty_is_visible(tty, ctx, (*ctx).ocx, (*ctx).ocy, (*ctx).num, 1u32) == 0 {
        return;
    }
    if (*ctx).bigger != 0
        && ((*ctx).xoff.wrapping_add((*ctx).ocx) < (*ctx).wox
            || (*ctx)
                .xoff
                .wrapping_add((*ctx).ocx)
                .wrapping_add((*ctx).num)
                > (*ctx).wox.wrapping_add((*ctx).wsx))
    {
        if (*ctx).wrapped == 0
            || !((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx)
            || (*(*tty).term).flags & 0x2i32 != 0
            || (*ctx).xoff.wrapping_add((*ctx).ocx) != 0u32
            || (*ctx).yoff.wrapping_add((*ctx).ocy) != (*tty).cy.wrapping_add(1u32)
            || (*tty).cx < (*tty).sx
            || (*tty).cy == (*tty).rlower
        {
            tty_draw_pane(tty, ctx, (*ctx).ocy);
        } else {
            (*ctx).redraw_cb.expect("non-null function pointer")(ctx);
        }
        return;
    }
    tty_margin_off(tty);
    tty_cursor_pane_unless_wrap(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_attributes(tty, (*ctx).cell, &(*ctx).defaults, (*ctx).palette);
    tty_putn(tty, (*ctx).ptr, (*ctx).num as size_t, (*ctx).num);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_setselection(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    tty_set_selection(tty, (*ctx).ptr as *const libc::c_char, (*ctx).num as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn tty_set_selection(
    mut tty: *mut tty,
    mut buf: *const libc::c_char,
    mut len: size_t,
) {
    let mut encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    if !(*tty).flags & 0x10i32 != 0 {
        return;
    }
    if tty_term_has((*tty).term, tty_code_code::MS) == 0 {
        return;
    }
    size = (4u64)
        .wrapping_mul(len.wrapping_add(2u64).wrapping_div(3u64))
        .wrapping_add(1u64);
    encoded = xmalloc(size) as *mut libc::c_char;
    __b64_ntop(buf as *const libc::c_uchar, len, encoded, size);
    tty_putcode_ptr2(
        tty,
        tty_code_code::MS,
        b"\x00" as *const u8 as *const libc::c_void,
        encoded as *const libc::c_void,
    );
    free(encoded as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_rawstring(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    tty_add(tty, (*ctx).ptr as *const libc::c_char, (*ctx).num as size_t);
    tty_invalidate(tty);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_syncstart(mut tty: *mut tty, mut _ctx: *const tty_ctx) {
    tty_sync_start(tty);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cell(
    mut tty: *mut tty,
    mut gc: *const GridCell,
    mut defaults: *const GridCell,
    mut palette: *mut libc::c_int,
) {
    let mut gcp: *const GridCell = 0 as *const GridCell;
    /* Skip last character if terminal is stupid. */
    if (*(*tty).term).flags & 0x2i32 != 0
        && (*tty).cy == (*tty).sy.wrapping_sub(1u32)
        && (*tty).cx == (*tty).sx.wrapping_sub(1u32)
    {
        return;
    }
    /* If this is a padding character, do nothing. */
    if (*gc).flags as libc::c_int & 0x4i32 != 0 {
        return;
    }
    /* Check the output codeset and apply attributes. */
    gcp = tty_check_codeset(tty, gc);
    tty_attributes(tty, gcp, defaults, palette);
    /* If it is a single character, write with putc to handle ACS. */
    if (*gcp).data.size as libc::c_int == 1i32 {
        tty_attributes(tty, gcp, defaults, palette);
        if (*(*gcp).data.data.as_ptr() as libc::c_int) < 0x20i32
            || *(*gcp).data.data.as_ptr() as libc::c_int == 0x7fi32
        {
            return;
        }
        tty_putc(tty, *(*gcp).data.data.as_ptr());
        return;
    }
    /* Write the data. */
    tty_putn(
        tty,
        (*gcp).data.data.as_ptr() as *const libc::c_void,
        (*gcp).data.size as size_t,
        (*gcp).data.width as u_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_reset(mut tty: *mut tty) {
    let mut gc: *mut GridCell = &mut (*tty).cell;
    if grid_cells_equal(gc, &grid_default_cell) == 0 {
        if (*gc).attr as libc::c_int & 0x80i32 != 0 && tty_acs_needed(tty) != 0 {
            tty_putcode(tty, tty_code_code::RMACS);
        }
        tty_putcode(tty, tty_code_code::SGR0);
        memcpy(
            gc as *mut libc::c_void,
            &grid_default_cell as *const GridCell as *const libc::c_void,
            ::std::mem::size_of::<GridCell>() as libc::c_ulong,
        );
    }
    memcpy(
        &mut (*tty).last_cell as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
}
unsafe extern "C" fn tty_invalidate(mut tty: *mut tty) {
    memcpy(
        &mut (*tty).cell as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    memcpy(
        &mut (*tty).last_cell as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    (*tty).cy = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*tty).cx = (*tty).cy;
    (*tty).rleft = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*tty).rupper = (*tty).rleft;
    (*tty).rright = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*tty).rlower = (*tty).rright;
    if (*tty).flags & 0x10i32 != 0 {
        if (*(*tty).term).flags & 0x4i32 != 0 {
            tty_putcode(tty, tty_code_code::ENMG);
        }
        tty_putcode(tty, tty_code_code::SGR0);
        (*tty).mode = 0xffffffi32;
        tty_update_mode(tty, 0x1i32, 0 as *mut screen);
        tty_cursor(tty, 0u32, 0u32);
        tty_region_off(tty);
        tty_margin_off(tty);
    } else {
        (*tty).mode = 0x1i32
    };
}
/* Turn off margin. */
#[no_mangle]
pub unsafe extern "C" fn tty_region_off(mut tty: *mut tty) {
    tty_region(tty, 0u32, (*tty).sy.wrapping_sub(1u32));
}
/* Set region inside pane. */
unsafe extern "C" fn tty_region_pane(
    mut tty: *mut tty,
    mut ctx: *const tty_ctx,
    mut rupper: u_int,
    mut rlower: u_int,
) {
    tty_region(
        tty,
        (*ctx).yoff.wrapping_add(rupper).wrapping_sub((*ctx).woy),
        (*ctx).yoff.wrapping_add(rlower).wrapping_sub((*ctx).woy),
    );
}
/* Set region at absolute position. */
unsafe extern "C" fn tty_region(mut tty: *mut tty, mut rupper: u_int, mut rlower: u_int) {
    if (*tty).rlower == rlower && (*tty).rupper == rupper {
        return;
    }
    if tty_term_has((*tty).term, tty_code_code::CSR) == 0 {
        return;
    }
    (*tty).rupper = rupper;
    (*tty).rlower = rlower;
    /*
     * Some terminals (such as PuTTY) do not correctly reset the cursor to
     * 0,0 if it is beyond the last column (they do not reset their wrap
     * flag so further output causes a line feed). As a workaround, do an
     * explicit move to 0 first.
     */
    if (*tty).cx >= (*tty).sx {
        if (*tty).cy == (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32) {
            tty_cursor(tty, 0u32, 0u32);
        } else {
            tty_cursor(tty, 0u32, (*tty).cy);
        }
    }
    tty_putcode2(
        tty,
        tty_code_code::CSR,
        (*tty).rupper as libc::c_int,
        (*tty).rlower as libc::c_int,
    );
    (*tty).cy = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*tty).cx = (*tty).cy;
}
/* Turn off margin. */
#[no_mangle]
pub unsafe extern "C" fn tty_margin_off(mut tty: *mut tty) {
    tty_margin(tty, 0u32, (*tty).sx.wrapping_sub(1u32));
}
/* Set margin inside pane. */
unsafe extern "C" fn tty_margin_pane(mut tty: *mut tty, mut ctx: *const tty_ctx) {
    tty_margin(
        tty,
        (*ctx).xoff.wrapping_sub((*ctx).wox),
        (*ctx)
            .xoff
            .wrapping_add((*ctx).sx)
            .wrapping_sub(1u32)
            .wrapping_sub((*ctx).wox),
    );
}
/* Set margin at absolute position. */
unsafe extern "C" fn tty_margin(mut tty: *mut tty, mut rleft: u_int, mut rright: u_int) {
    if (*(*tty).term).flags & 0x4i32 == 0 {
        return;
    }
    if (*tty).rleft == rleft && (*tty).rright == rright {
        return;
    }
    tty_putcode2(
        tty,
        tty_code_code::CSR,
        (*tty).rupper as libc::c_int,
        (*tty).rlower as libc::c_int,
    );
    (*tty).rleft = rleft;
    (*tty).rright = rright;
    if rleft == 0u32 && rright == (*tty).sx.wrapping_sub(1u32) {
        tty_putcode(tty, tty_code_code::CLMG);
    } else {
        tty_putcode2(
            tty,
            tty_code_code::CMG,
            rleft as libc::c_int,
            rright as libc::c_int,
        );
    }
    (*tty).cy = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    (*tty).cx = (*tty).cy;
}
/*
 * Move the cursor, unless it would wrap itself when the next character is
 * printed.
 */
unsafe extern "C" fn tty_cursor_pane_unless_wrap(
    mut tty: *mut tty,
    mut ctx: *const tty_ctx,
    mut cx: u_int,
    mut cy: u_int,
) {
    if (*ctx).wrapped == 0
        || !((*ctx).xoff == 0u32 && (*ctx).sx >= (*tty).sx)
        || (*(*tty).term).flags & 0x2i32 != 0
        || (*ctx).xoff.wrapping_add(cx) != 0u32
        || (*ctx).yoff.wrapping_add(cy) != (*tty).cy.wrapping_add(1u32)
        || (*tty).cx < (*tty).sx
        || (*tty).cy == (*tty).rlower
    {
        tty_cursor_pane(tty, ctx, cx, cy);
    } else {
        log_debug(
            b"%s: will wrap at %u,%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"tty_cursor_pane_unless_wrap\x00",
            ))
            .as_ptr(),
            (*tty).cx,
            (*tty).cy,
        );
    };
}
/* Move cursor inside pane. */
unsafe extern "C" fn tty_cursor_pane(
    mut tty: *mut tty,
    mut ctx: *const tty_ctx,
    mut cx: u_int,
    mut cy: u_int,
) {
    tty_cursor(
        tty,
        (*ctx).xoff.wrapping_add(cx).wrapping_sub((*ctx).wox),
        (*ctx).yoff.wrapping_add(cy).wrapping_sub((*ctx).woy),
    );
}
/* Move cursor to absolute position. */
#[no_mangle]
pub unsafe extern "C" fn tty_cursor(mut tty: *mut tty, mut cx: u_int, mut cy: u_int) {
    let mut current_block: u64;
    let mut term: *mut tty_term = (*tty).term;
    let mut thisx: u_int = 0;
    let mut thisy: u_int = 0;
    let mut change: libc::c_int = 0;
    if (*tty).flags & 0x80i32 != 0 {
        return;
    }
    if cx > (*tty).sx.wrapping_sub(1u32) {
        cx = (*tty).sx.wrapping_sub(1u32)
    }
    thisx = (*tty).cx;
    thisy = (*tty).cy;
    /* No change. */
    if cx == thisx && cy == thisy {
        return;
    }
    /* Very end of the line, just use absolute movement. */
    if thisx > (*tty).sx.wrapping_sub(1u32) {
        current_block = 6863629183173758772;
    } else if cx == 0u32 && cy == 0u32 && tty_term_has(term, tty_code_code::HOME) != 0 {
        tty_putcode(tty, tty_code_code::HOME);
        current_block = 6993536316001430845;
    } else if cx == 0u32
        && cy == thisy.wrapping_add(1u32)
        && thisy != (*tty).rlower
        && ((*(*tty).term).flags & 0x4i32 == 0 || (*tty).rleft == 0u32)
    {
        tty_putc(tty, '\r' as u_char);
        tty_putc(tty, '\n' as u_char);
        current_block = 6993536316001430845;
    } else if cy == thisy {
        /* Move to home position (0, 0). */
        /* Zero on the next line. */
        /* Moving column or row. */
        /*
         * Moving column only, row staying the same.
         */
        /* To left edge. */
        if cx == 0u32 && ((*(*tty).term).flags & 0x4i32 == 0 || (*tty).rleft == 0u32) {
            tty_putc(tty, '\r' as u_char);
            current_block = 6993536316001430845;
        } else if cx == thisx.wrapping_sub(1u32) && tty_term_has(term, tty_code_code::CUB1) != 0 {
            tty_putcode(tty, tty_code_code::CUB1);
            current_block = 6993536316001430845;
        } else if cx == thisx.wrapping_add(1u32) && tty_term_has(term, tty_code_code::CUF1) != 0 {
            tty_putcode(tty, tty_code_code::CUF1);
            current_block = 6993536316001430845;
        } else {
            /* One to the left. */
            /* One to the right. */
            /* Calculate difference. */
            change = thisx.wrapping_sub(cx) as libc::c_int; /* +ve left, -ve right */
            /*
             * Use HPA if change is larger than absolute, otherwise move
             * the cursor with CUB/CUF.
             */
            if abs(change) as u_int > cx && tty_term_has(term, tty_code_code::HPA) != 0 {
                tty_putcode1(tty, tty_code_code::HPA, cx as libc::c_int);
                current_block = 6993536316001430845;
            } else if change > 0i32
                && tty_term_has(term, tty_code_code::CUB) != 0
                && (*(*tty).term).flags & 0x4i32 == 0
            {
                if change == 2i32 && tty_term_has(term, tty_code_code::CUB1) != 0 {
                    tty_putcode(tty, tty_code_code::CUB1);
                    tty_putcode(tty, tty_code_code::CUB1);
                } else {
                    tty_putcode1(tty, tty_code_code::CUB, change);
                }
                current_block = 6993536316001430845;
            } else if change < 0i32
                && tty_term_has(term, tty_code_code::CUF) != 0
                && (*(*tty).term).flags & 0x4i32 == 0
            {
                tty_putcode1(tty, tty_code_code::CUF, -change);
                current_block = 6993536316001430845;
            } else {
                current_block = 6863629183173758772;
            }
        }
    } else if cx == thisx {
        /*
         * Moving row only, column staying the same.
         */
        /* One above. */
        if thisy != (*tty).rupper
            && cy == thisy.wrapping_sub(1u32)
            && tty_term_has(term, tty_code_code::CUU1) != 0
        {
            tty_putcode(tty, tty_code_code::CUU1);
            current_block = 6993536316001430845;
        } else if thisy != (*tty).rlower
            && cy == thisy.wrapping_add(1u32)
            && tty_term_has(term, tty_code_code::CUD1) != 0
        {
            tty_putcode(tty, tty_code_code::CUD1);
            current_block = 6993536316001430845;
        } else {
            /* One below. */
            /* Calculate difference. */
            change = thisy.wrapping_sub(cy) as libc::c_int; /* +ve up, -ve down */
            /*
             * Try to use VPA if change is larger than absolute or if this
             * change would cross the scroll region, otherwise use CUU/CUD.
             */
            if abs(change) as u_int > cy
                || change < 0i32 && cy.wrapping_sub(change as libc::c_uint) > (*tty).rlower
                || change > 0i32 && cy.wrapping_sub(change as libc::c_uint) < (*tty).rupper
            {
                if tty_term_has(term, tty_code_code::VPA) != 0 {
                    tty_putcode1(tty, tty_code_code::VPA, cy as libc::c_int);
                    current_block = 6993536316001430845;
                } else {
                    current_block = 6863629183173758772;
                }
            } else if change > 0i32 && tty_term_has(term, tty_code_code::CUU) != 0 {
                tty_putcode1(tty, tty_code_code::CUU, change);
                current_block = 6993536316001430845;
            } else if change < 0i32 && tty_term_has(term, tty_code_code::CUD) != 0 {
                tty_putcode1(tty, tty_code_code::CUD, -change);
                current_block = 6993536316001430845;
            } else {
                current_block = 6863629183173758772;
            }
        }
    } else {
        current_block = 6863629183173758772;
    }
    match current_block {
        6863629183173758772 => {
            /* Absolute movement. */
            tty_putcode2(
                tty,
                tty_code_code::CUP,
                cy as libc::c_int,
                cx as libc::c_int,
            );
        }
        _ => {}
    }
    (*tty).cx = cx;
    (*tty).cy = cy;
}
#[no_mangle]
pub unsafe extern "C" fn tty_attributes(
    mut tty: *mut tty,
    mut gc: *const GridCell,
    mut defaults: *const GridCell,
    mut palette: *mut libc::c_int,
) {
    let mut tc: *mut GridCell = &mut (*tty).cell;
    let mut gc2: GridCell = GridCell {
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
    let mut changed: libc::c_int = 0;
    /* Copy cell and update default colours. */
    memcpy(
        &mut gc2 as *mut GridCell as *mut libc::c_void,
        gc as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    if gc2.fg == 8i32 {
        gc2.fg = (*defaults).fg
    }
    if gc2.bg == 8i32 {
        gc2.bg = (*defaults).bg
    }
    /* Ignore cell if it is the same as the last one. */
    if gc2.attr as libc::c_int == (*tty).last_cell.attr as libc::c_int
        && gc2.fg == (*tty).last_cell.fg
        && gc2.bg == (*tty).last_cell.bg
        && gc2.us == (*tty).last_cell.us
    {
        return;
    }
    /*
     * If no setab, try to use the reverse attribute as a best-effort for a
     * non-default background. This is a bit of a hack but it doesn't do
     * any serious harm and makes a couple of applications happier.
     */
    if tty_term_has((*tty).term, tty_code_code::SETAB) == 0 {
        if gc2.attr as libc::c_int & 0x10i32 != 0 {
            if gc2.fg != 7i32 && !(gc2.fg == 8i32 || gc2.fg == 9i32) {
                gc2.attr = (gc2.attr as libc::c_int & !(0x10i32)) as u_short
            }
        } else if gc2.bg != 0i32 && !(gc2.bg == 8i32 || gc2.bg == 9i32) {
            gc2.attr = (gc2.attr as libc::c_int | 0x10i32) as u_short
        }
    }
    /* Fix up the colours if necessary. */
    tty_check_fg(tty, palette, &mut gc2);
    tty_check_bg(tty, palette, &mut gc2);
    tty_check_us(tty, palette, &mut gc2);
    /*
     * If any bits are being cleared or the underline colour is now default,
     * reset everything.
     */
    if (*tc).attr as libc::c_int & !(gc2.attr as libc::c_int) != 0
        || (*tc).us != gc2.us && gc2.us == 0i32
    {
        tty_reset(tty);
    }
    /*
     * Set the colours. This may call tty_reset() (so it comes next) and
     * may add to (NOT remove) the desired attributes.
     */
    tty_colours(tty, &mut gc2);
    /* Filter out attribute bits already set. */
    changed = gc2.attr as libc::c_int & !((*tc).attr as libc::c_int);
    (*tc).attr = gc2.attr;
    /* Set the attributes. */
    if changed & 0x1i32 != 0 {
        tty_putcode(tty, tty_code_code::BOLD);
    }
    if changed & 0x2i32 != 0 {
        tty_putcode(tty, tty_code_code::DIM);
    }
    if changed & 0x40i32 != 0 {
        tty_set_italics(tty);
    }
    if changed & (0x4i32 | 0x200i32 | 0x400i32 | 0x800i32 | 0x1000i32) != 0 {
        if changed & 0x4i32 != 0 || tty_term_has((*tty).term, tty_code_code::SMULX) == 0 {
            tty_putcode(tty, tty_code_code::SMUL);
        } else if changed & 0x200i32 != 0 {
            tty_putcode1(tty, tty_code_code::SMULX, 2i32);
        } else if changed & 0x400i32 != 0 {
            tty_putcode1(tty, tty_code_code::SMULX, 3i32);
        } else if changed & 0x800i32 != 0 {
            tty_putcode1(tty, tty_code_code::SMULX, 4i32);
        } else if changed & 0x1000i32 != 0 {
            tty_putcode1(tty, tty_code_code::SMULX, 5i32);
        }
    }
    if changed & 0x8i32 != 0 {
        tty_putcode(tty, tty_code_code::BLINK);
    }
    if changed & 0x10i32 != 0 {
        if tty_term_has((*tty).term, tty_code_code::REV) != 0 {
            tty_putcode(tty, tty_code_code::REV);
        } else if tty_term_has((*tty).term, tty_code_code::SMSO) != 0 {
            tty_putcode(tty, tty_code_code::SMSO);
        }
    }
    if changed & 0x20i32 != 0 {
        tty_putcode(tty, tty_code_code::INVIS);
    }
    if changed & 0x100i32 != 0 {
        tty_putcode(tty, tty_code_code::SMXX);
    }
    if changed & 0x2000i32 != 0 {
        tty_putcode(tty, tty_code_code::SMOL);
    }
    if changed & 0x80i32 != 0 && tty_acs_needed(tty) != 0 {
        tty_putcode(tty, tty_code_code::SMACS);
    }
    memcpy(
        &mut (*tty).last_cell as *mut GridCell as *mut libc::c_void,
        &mut gc2 as *mut GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
}
unsafe extern "C" fn tty_colours(mut tty: *mut tty, mut gc: *const GridCell) {
    let mut tc: *mut GridCell = &mut (*tty).cell;
    let mut have_ax: libc::c_int = 0;
    /* No changes? Nothing is necessary. */
    if (*gc).fg == (*tc).fg && (*gc).bg == (*tc).bg && (*gc).us == (*tc).us {
        return;
    }
    /*
     * Is either the default colour? This is handled specially because the
     * best solution might be to reset both colours to default, in which
     * case if only one is default need to fall onward to set the other
     * colour.
     */
    if (*gc).fg == 8i32 || (*gc).fg == 9i32 || ((*gc).bg == 8i32 || (*gc).bg == 9i32) {
        /*
         * If don't have AX but do have op, send sgr0 (op can't
         * actually be used because it is sometimes the same as sgr0
         * and sometimes isn't). This resets both colours to default.
         *
         * Otherwise, try to set the default colour only as needed.
         */
        have_ax = tty_term_flag((*tty).term, tty_code_code::AX);
        if have_ax == 0 && tty_term_has((*tty).term, tty_code_code::OP) != 0 {
            tty_reset(tty);
        } else {
            if ((*gc).fg == 8i32 || (*gc).fg == 9i32) && !((*tc).fg == 8i32 || (*tc).fg == 9i32) {
                if have_ax != 0 {
                    tty_puts(tty, b"\x1b[39m\x00" as *const u8 as *const libc::c_char);
                } else if (*tc).fg != 7i32 {
                    tty_putcode1(tty, tty_code_code::SETAF, 7i32);
                }
                (*tc).fg = (*gc).fg
            }
            if ((*gc).bg == 8i32 || (*gc).bg == 9i32) && !((*tc).bg == 8i32 || (*tc).bg == 9i32) {
                if have_ax != 0 {
                    tty_puts(tty, b"\x1b[49m\x00" as *const u8 as *const libc::c_char);
                } else if (*tc).bg != 0i32 {
                    tty_putcode1(tty, tty_code_code::SETAB, 0i32);
                }
                (*tc).bg = (*gc).bg
            }
        }
    }
    /* Set the foreground colour. */
    if !((*gc).fg == 8i32 || (*gc).fg == 9i32) && (*gc).fg != (*tc).fg {
        tty_colours_fg(tty, gc);
    }
    /*
     * Set the background colour. This must come after the foreground as
     * tty_colour_fg() can call tty_reset().
     */
    if !((*gc).bg == 8i32 || (*gc).bg == 9i32) && (*gc).bg != (*tc).bg {
        tty_colours_bg(tty, gc);
    }
    /* Set the underscore color. */
    if (*gc).us != (*tc).us {
        tty_colours_us(tty, gc);
    };
}
unsafe extern "C" fn tty_check_fg(
    mut tty: *mut tty,
    mut palette: *mut libc::c_int,
    mut gc: *mut GridCell,
) {
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    let mut colours: u_int = 0;
    let mut c: libc::c_int = 0;
    /*
     * Perform substitution if this pane has a palette. If the bright
     * attribute is set, use the bright entry in the palette by changing to
     * the aixterm colour.
     */
    if !((*gc).flags as libc::c_int) & 0x20i32 != 0 {
        c = (*gc).fg;
        if c < 8i32 && (*gc).attr as libc::c_int & 0x1i32 != 0 {
            c += 90i32
        }
        c = tty_get_palette(palette, c);
        if c != -(1i32) {
            (*gc).fg = c
        }
    }
    /* Is this a 24-bit colour? */
    if (*gc).fg & 0x2000000i32 != 0 {
        /* Not a 24-bit terminal? Translate to 256-colour palette. */
        if (*(*tty).term).flags & 0x10i32 != 0 {
            return;
        }
        colour_split_rgb((*gc).fg, &mut r, &mut g, &mut b);
        (*gc).fg = colour_find_rgb(r, g, b)
    }
    /* How many colours does this terminal have? */
    if (*(*tty).term).flags & 0x1i32 != 0 {
        colours = 256u32
    } else {
        colours = tty_term_number((*tty).term, tty_code_code::COLORS) as u_int
    }
    /* Is this a 256-colour colour? */
    if (*gc).fg & 0x1000000i32 != 0 {
        /* And not a 256 colour mode? */
        if colours != 256u32 {
            (*gc).fg = colour_256to16((*gc).fg);
            if (*gc).fg & 8i32 != 0 {
                (*gc).fg &= 7i32;
                if colours >= 16u32 {
                    (*gc).fg += 90i32
                }
            }
        }
        return;
    }
    /* Is this an aixterm colour? */
    if (*gc).fg >= 90i32 && (*gc).fg <= 97i32 && colours < 16u32 {
        (*gc).fg -= 90i32;
        (*gc).attr = ((*gc).attr as libc::c_int | 0x1i32) as u_short
    };
}
unsafe extern "C" fn tty_check_bg(
    mut tty: *mut tty,
    mut palette: *mut libc::c_int,
    mut gc: *mut GridCell,
) {
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    let mut colours: u_int = 0;
    let mut c: libc::c_int = 0;
    /* Perform substitution if this pane has a palette. */
    if !((*gc).flags as libc::c_int) & 0x20i32 != 0 {
        c = tty_get_palette(palette, (*gc).bg);
        if c != -(1i32) {
            (*gc).bg = c
        }
    }
    /* Is this a 24-bit colour? */
    if (*gc).bg & 0x2000000i32 != 0 {
        /* Not a 24-bit terminal? Translate to 256-colour palette. */
        if (*(*tty).term).flags & 0x10i32 != 0 {
            return;
        }
        colour_split_rgb((*gc).bg, &mut r, &mut g, &mut b);
        (*gc).bg = colour_find_rgb(r, g, b)
    }
    /* How many colours does this terminal have? */
    if (*(*tty).term).flags & 0x1i32 != 0 {
        colours = 256u32
    } else {
        colours = tty_term_number((*tty).term, tty_code_code::COLORS) as u_int
    }
    /* Is this a 256-colour colour? */
    if (*gc).bg & 0x1000000i32 != 0 {
        /*
         * And not a 256 colour mode? Translate to 16-colour
         * palette. Bold background doesn't exist portably, so just
         * discard the bold bit if set.
         */
        if colours != 256u32 {
            (*gc).bg = colour_256to16((*gc).bg);
            if (*gc).bg & 8i32 != 0 {
                (*gc).bg &= 7i32;
                if colours >= 16u32 {
                    (*gc).bg += 90i32
                }
            }
        }
        return;
    }
    /* Is this an aixterm colour? */
    if (*gc).bg >= 90i32 && (*gc).bg <= 97i32 && colours < 16u32 {
        (*gc).bg -= 90i32
    };
}
unsafe extern "C" fn tty_check_us(
    mut _tty: *mut tty,
    mut palette: *mut libc::c_int,
    mut gc: *mut GridCell,
) {
    let mut c: libc::c_int = 0;
    /* Perform substitution if this pane has a palette. */
    if !((*gc).flags as libc::c_int) & 0x20i32 != 0 {
        c = tty_get_palette(palette, (*gc).us);
        if c != -(1i32) {
            (*gc).us = c
        }
    }
    /* Underscore colour is set as RGB so convert a 256 colour to RGB. */
    if (*gc).us & 0x1000000i32 != 0 {
        (*gc).us = colour_256toRGB((*gc).us)
    };
}
unsafe extern "C" fn tty_colours_fg(mut tty: *mut tty, mut gc: *const GridCell) {
    let mut tc: *mut GridCell = &mut (*tty).cell;
    let mut s: [libc::c_char; 32] = [0; 32];
    /* Is this a 24-bit or 256-colour colour? */
    if (*gc).fg & 0x2000000i32 != 0 || (*gc).fg & 0x1000000i32 != 0 {
        if !(tty_try_colour(tty, (*gc).fg, b"38\x00" as *const u8 as *const libc::c_char) == 0i32) {
            /* Should not get here, already converted in tty_check_fg. */
            return;
        }
    } else if (*gc).fg >= 90i32 && (*gc).fg <= 97i32 {
        if (*(*tty).term).flags & 0x1i32 != 0 {
            xsnprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"\x1b[%dm\x00" as *const u8 as *const libc::c_char,
                (*gc).fg,
            );
            tty_puts(tty, s.as_mut_ptr());
        } else {
            tty_putcode1(tty, tty_code_code::SETAF, (*gc).fg - 90i32 + 8i32);
        }
    } else {
        /* Is this an aixterm bright colour? */
        /* Otherwise set the foreground colour. */
        tty_putcode1(tty, tty_code_code::SETAF, (*gc).fg);
    }
    /* Save the new values in the terminal current cell. */
    (*tc).fg = (*gc).fg;
}
unsafe extern "C" fn tty_colours_bg(mut tty: *mut tty, mut gc: *const GridCell) {
    let mut tc: *mut GridCell = &mut (*tty).cell;
    let mut s: [libc::c_char; 32] = [0; 32];
    /* Is this a 24-bit or 256-colour colour? */
    if (*gc).bg & 0x2000000i32 != 0 || (*gc).bg & 0x1000000i32 != 0 {
        if !(tty_try_colour(tty, (*gc).bg, b"48\x00" as *const u8 as *const libc::c_char) == 0i32) {
            /* Should not get here, already converted in tty_check_bg. */
            return;
        }
    } else if (*gc).bg >= 90i32 && (*gc).bg <= 97i32 {
        if (*(*tty).term).flags & 0x1i32 != 0 {
            xsnprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"\x1b[%dm\x00" as *const u8 as *const libc::c_char,
                (*gc).bg + 10i32,
            );
            tty_puts(tty, s.as_mut_ptr());
        } else {
            tty_putcode1(tty, tty_code_code::SETAB, (*gc).bg - 90i32 + 8i32);
        }
    } else {
        /* Is this an aixterm bright colour? */
        /* Otherwise set the background colour. */
        tty_putcode1(tty, tty_code_code::SETAB, (*gc).bg);
    }
    /* Save the new values in the terminal current cell. */
    (*tc).bg = (*gc).bg;
}
unsafe extern "C" fn tty_colours_us(mut tty: *mut tty, mut gc: *const GridCell) {
    let mut tc: *mut GridCell = &mut (*tty).cell;
    let mut c: u_int = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    /* Clear underline colour. */
    if (*gc).us == 0i32 {
        tty_putcode(tty, tty_code_code::OL);
    } else {
        /* Must be an RGB colour - this should never happen. */
        if !(*gc).us & 0x2000000i32 != 0 {
            return;
        }
        /*
         * Setulc and setal follows the ncurses(3) one argument "direct colour"
         * capability format. Calculate the colour value.
         */
        colour_split_rgb((*gc).us, &mut r, &mut g, &mut b);
        c = (65536i32 * r as libc::c_int + 256i32 * g as libc::c_int + b as libc::c_int) as u_int;
        /*
         * Write the colour. Only use setal if the RGB flag is set because the
         * non-RGB version may be wrong.
         */
        if tty_term_has((*tty).term, tty_code_code::SETULC) != 0 {
            tty_putcode1(tty, tty_code_code::SETULC, c as libc::c_int);
        } else if tty_term_has((*tty).term, tty_code_code::SETAL) != 0
            && tty_term_has((*tty).term, tty_code_code::RGB) != 0
        {
            tty_putcode1(tty, tty_code_code::SETAL, c as libc::c_int);
        }
    }
    /* Save the new values in the terminal current cell. */
    (*tc).us = (*gc).us;
}
unsafe extern "C" fn tty_try_colour(
    mut tty: *mut tty,
    mut colour: libc::c_int,
    mut type_0: *const libc::c_char,
) -> libc::c_int {
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    if colour & 0x1000000i32 != 0 {
        if *type_0 as libc::c_int == '3' as i32
            && tty_term_has((*tty).term, tty_code_code::SETAF) != 0
        {
            tty_putcode1(tty, tty_code_code::SETAF, colour & 0xffi32);
        } else if tty_term_has((*tty).term, tty_code_code::SETAB) != 0 {
            tty_putcode1(tty, tty_code_code::SETAB, colour & 0xffi32);
        }
        return 0i32;
    }
    if colour & 0x2000000i32 != 0 {
        colour_split_rgb(colour & 0xffffffi32, &mut r, &mut g, &mut b);
        if *type_0 as libc::c_int == '3' as i32
            && tty_term_has((*tty).term, tty_code_code::SETRGBF) != 0
        {
            tty_putcode3(
                tty,
                tty_code_code::SETRGBF,
                r as libc::c_int,
                g as libc::c_int,
                b as libc::c_int,
            );
        } else if tty_term_has((*tty).term, tty_code_code::SETRGBB) != 0 {
            tty_putcode3(
                tty,
                tty_code_code::SETRGBB,
                r as libc::c_int,
                g as libc::c_int,
                b as libc::c_int,
            );
        }
        return 0i32;
    }
    return -(1i32);
}
unsafe extern "C" fn tty_window_default_style(mut gc: *mut GridCell, mut wp: *mut window_pane) {
    memcpy(
        gc as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    (*gc).fg = (*wp).fg;
    (*gc).bg = (*wp).bg;
}
#[no_mangle]
pub unsafe extern "C" fn tty_default_colours(mut gc: *mut GridCell, mut wp: *mut window_pane) {
    let mut oo: *mut crate::options::options = (*wp).options;
    memcpy(
        gc as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    if (*wp).flags & 0x1000i32 != 0 {
        (*wp).flags &= !(0x1000i32);
        tty_window_default_style(&mut (*wp).cached_active_gc, wp);
        style_add(
            &mut (*wp).cached_active_gc,
            oo,
            b"window-active-style\x00" as *const u8 as *const libc::c_char,
            0 as *mut crate::format::format_tree,
        );
        tty_window_default_style(&mut (*wp).cached_gc, wp);
        style_add(
            &mut (*wp).cached_gc,
            oo,
            b"window-style\x00" as *const u8 as *const libc::c_char,
            0 as *mut crate::format::format_tree,
        );
    }
    if (*gc).fg == 8i32 {
        if wp == (*(*wp).window).active && (*wp).cached_active_gc.fg != 8i32 {
            (*gc).fg = (*wp).cached_active_gc.fg
        } else {
            (*gc).fg = (*wp).cached_gc.fg
        }
    }
    if (*gc).bg == 8i32 {
        if wp == (*(*wp).window).active && (*wp).cached_active_gc.bg != 8i32 {
            (*gc).bg = (*wp).cached_active_gc.bg
        } else {
            (*gc).bg = (*wp).cached_gc.bg
        }
    };
}
unsafe extern "C" fn tty_default_attributes(
    mut tty: *mut tty,
    mut defaults: *const GridCell,
    mut palette: *mut libc::c_int,
    mut bg: u_int,
) {
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
    memcpy(
        &mut gc as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    gc.bg = bg as libc::c_int;
    tty_attributes(tty, &mut gc, defaults, palette);
}
