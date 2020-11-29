use crate::{grid::Cell as GridCell, key_code::code as key_code_code, utf8::Utf8Data};
use ::c2rust_bitfields;
use ::libc;

extern "C" {
    pub type re_dfa_t;
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn regfree(__preg: *mut regex_t);
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_free(bufev: *mut bufferevent);
    #[no_mangle]
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_enable(bufev: *mut bufferevent, event: libc::c_short) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_disable(bufev: *mut bufferevent, event: libc::c_short) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_initialized(ev: *const event) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t) -> *mut libc::c_uchar;
    #[no_mangle]
    fn bufferevent_new(
        fd: libc::c_int,
        readcb: bufferevent_data_cb,
        writecb: bufferevent_data_cb,
        errorcb: bufferevent_event_cb,
        cbarg: *mut libc::c_void,
    ) -> *mut bufferevent;
    #[no_mangle]
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_w_options: *mut crate::options::options;
    #[no_mangle]
    fn setblocking(_: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn options_free(_: *mut crate::options::options);
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn tty_update_window_offset(_: *mut window);
    #[no_mangle]
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char);
    #[no_mangle]
    fn cmdq_get_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
    #[no_mangle]
    fn cmdq_continue(_: *mut crate::cmd_queue::cmdq_item);
    #[no_mangle]
    fn alerts_queue(_: *mut window, _: libc::c_int);
    #[no_mangle]
    fn file_read(_: *mut client, _: *const libc::c_char, _: client_file_cb, _: *mut libc::c_void);
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_clear_marked();
    #[no_mangle]
    fn server_check_marked() -> libc::c_int;
    #[no_mangle]
    fn server_client_unref(_: *mut client);
    #[no_mangle]
    fn server_status_session(_: *mut session);
    #[no_mangle]
    fn server_redraw_window_borders(_: *mut window);
    #[no_mangle]
    fn server_status_window(_: *mut window);
    #[no_mangle]
    fn server_destroy_pane(_: *mut window_pane, _: libc::c_int);
    #[no_mangle]
    fn input_init(_: *mut window_pane, _: *mut bufferevent) -> *mut crate::input::input_ctx;
    #[no_mangle]
    fn input_free(_: *mut crate::input::input_ctx);
    #[no_mangle]
    fn input_parse_pane(_: *mut window_pane);
    #[no_mangle]
    fn input_parse_buffer(_: *mut window_pane, _: *mut u_char, _: size_t);
    #[no_mangle]
    fn input_key_pane(_: *mut window_pane, _: key_code, _: *mut mouse_event) -> libc::c_int;
    #[no_mangle]
    fn grid_cells_look_equal(
        _: *const crate::grid::Cell,
        _: *const crate::grid::Cell,
    ) -> libc::c_int;
    #[no_mangle]
    fn grid_view_string_cells(_: *mut grid, _: u_int, _: u_int, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    fn options_create(_: *mut crate::options::options) -> *mut crate::options::options;
    #[no_mangle]
    fn notify_pane(_: *const libc::c_char, _: *mut window_pane);
    #[no_mangle]
    fn notify_window(_: *const libc::c_char, _: *mut window);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn layout_free_cell(_: *mut layout_cell);
    #[no_mangle]
    fn control_write_output(_: *mut client, _: *mut window_pane);
    #[no_mangle]
    static window_buffer_mode: window_mode;
    #[no_mangle]
    static window_client_mode: window_mode;
    #[no_mangle]
    static window_clock_mode: window_mode;
    #[no_mangle]
    static window_copy_mode: window_mode;
    #[no_mangle]
    static window_tree_mode: window_mode;
    #[no_mangle]
    static window_view_mode: window_mode;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn layout_init(_: *mut window, _: *mut window_pane);
    #[no_mangle]
    fn layout_fix_panes(_: *mut window);
    #[no_mangle]
    fn layout_free(_: *mut window);
    #[no_mangle]
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int);
    #[no_mangle]
    fn screen_set_title(_: *mut screen, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn screen_free(_: *mut screen);
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn utf8_stravis(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_5,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_0,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub ev_io: C2RustUnnamed_3,
    pub ev_signal: C2RustUnnamed_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub ev_signal_next: C2RustUnnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub ev_io_next: C2RustUnnamed_4,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_5 {
    pub ev_next_with_common_timeout: C2RustUnnamed_6,
    pub min_heap_idx: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_8,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_7,
    pub evcb_arg: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
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
    pub exit_type: C2RustUnnamed_29,
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
    pub prompt_mode: C2RustUnnamed_26,
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
    pub entry: C2RustUnnamed_9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
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
    pub entry: C2RustUnnamed_10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
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
    pub gentry: C2RustUnnamed_14,
    pub entry: C2RustUnnamed_13,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_14 {
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
    pub entry: C2RustUnnamed_17,
    pub wentry: C2RustUnnamed_16,
    pub sentry: C2RustUnnamed_15,
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
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_17 {
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
    pub alerts_entry: C2RustUnnamed_20,
    pub options: *mut crate::options::options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_19,
    pub entry: C2RustUnnamed_18,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_18 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_19 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_20 {
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
    pub entry: C2RustUnnamed_21,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_21 {
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
    pub modes: C2RustUnnamed_24,
    pub searchstr: *mut libc::c_char,
    pub searchregex: libc::c_int,
    pub written: size_t,
    pub skipped: size_t,
    pub border_gc_set: libc::c_int,
    pub border_gc: crate::grid::Cell,
    pub entry: C2RustUnnamed_23,
    pub tree_entry: C2RustUnnamed_22,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_22 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_23 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_24 {
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
    pub entry: C2RustUnnamed_25,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_25 {
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
pub type C2RustUnnamed_26 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_26 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_26 = 0;
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
    pub entry: C2RustUnnamed_27,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_27 {
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
    pub entry: C2RustUnnamed_28,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_28 {
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
pub type C2RustUnnamed_29 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_29 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_29 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_29 = 0;

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
    pub entry: C2RustUnnamed_30,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_30 {
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
    pub entry: C2RustUnnamed_31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_31 {
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
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
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
pub struct windows {
    pub rbh_root: *mut window,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_pane_input_data {
    pub item: *mut crate::cmd_queue::cmdq_item,
    pub wp: u_int,
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
 * Each window is attached to a number of panes, each of which is a pty. This
 * file contains code to handle them.
 *
 * A pane has two buffers attached, these are filled and emptied by the main
 * server poll loop. Output data is received from pty's in screen format,
 * translated and returned as a series of escape sequences and strings via
 * input_parse (in input.c). Input data is received as key codes and written
 * directly via input_key.
 *
 * Each pane also has a "virtual" screen (screen.c) which contains the current
 * state and is redisplayed when the window is reattached to a client.
 *
 * Windows are stored directly on a global array and wrapped in any number of
 * winlink structs to be linked onto local session RB trees. A reference count
 * is maintained and a window removed from the global list and destroyed when
 * it reaches zero.
 */
/* Global window list. */
#[no_mangle]
pub static mut windows: windows = windows {
    rbh_root: 0 as *mut window,
};
/* Global panes tree. */
#[no_mangle]
pub static mut all_window_panes: window_pane_tree = window_pane_tree {
    rbh_root: 0 as *mut window_pane,
};
static mut next_window_pane_id: u_int = 0;
static mut next_window_id: u_int = 0;
static mut next_active_point: u_int = 0;
/* List of window modes. */
#[no_mangle]
pub static mut all_window_modes: [*const window_mode; 7] = unsafe {
    [
        &window_buffer_mode as *const window_mode,
        &window_client_mode as *const window_mode,
        &window_clock_mode as *const window_mode,
        &window_copy_mode as *const window_mode,
        &window_tree_mode as *const window_mode,
        &window_view_mode as *const window_mode,
        0 as *const window_mode,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn windows_RB_PREV(mut elm: *mut window) -> *mut window {
    if !(*elm).entry.rbe_left.is_null() {
        elm = (*elm).entry.rbe_left;
        while !(*elm).entry.rbe_right.is_null() {
            elm = (*elm).entry.rbe_right
        }
    } else if !(*elm).entry.rbe_parent.is_null()
        && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
    {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
        {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_INSERT(
    mut head: *mut windows,
    mut elm: *mut window,
) -> *mut window {
    let mut tmp: *mut window = 0 as *mut window;
    let mut parent: *mut window = 0 as *mut window;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = window_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut window;
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
    windows_RB_INSERT_COLOR(head, elm);
    return 0 as *mut window;
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_FIND(
    mut head: *mut windows,
    mut elm: *mut window,
) -> *mut window {
    let mut tmp: *mut window = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = window_cmp(elm, tmp);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut window;
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_NFIND(
    mut head: *mut windows,
    mut elm: *mut window,
) -> *mut window {
    let mut tmp: *mut window = (*head).rbh_root;
    let mut res: *mut window = 0 as *mut window;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = window_cmp(elm, tmp);
        if comp < 0i32 {
            res = tmp;
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_NEXT(mut elm: *mut window) -> *mut window {
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
pub unsafe extern "C" fn windows_RB_REMOVE_COLOR(
    mut head: *mut windows,
    mut parent: *mut window,
    mut elm: *mut window,
) {
    let mut tmp: *mut window = 0 as *mut window;
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
                    let mut oleft: *mut window = 0 as *mut window;
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
                    let mut oright: *mut window = 0 as *mut window;
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
#[no_mangle]
pub unsafe extern "C" fn windows_RB_REMOVE(
    mut head: *mut windows,
    mut elm: *mut window,
) -> *mut window {
    let mut current_block: u64;
    let mut child: *mut window = 0 as *mut window;
    let mut parent: *mut window = 0 as *mut window;
    let mut old: *mut window = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut window = 0 as *mut window;
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
        current_block = 4254310618125552165;
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
        windows_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn windows_RB_MINMAX(
    mut head: *mut windows,
    mut val: libc::c_int,
) -> *mut window {
    let mut tmp: *mut window = (*head).rbh_root;
    let mut parent: *mut window = 0 as *mut window;
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
#[no_mangle]
pub unsafe extern "C" fn windows_RB_INSERT_COLOR(mut head: *mut windows, mut elm: *mut window) {
    let mut parent: *mut window = 0 as *mut window;
    let mut gparent: *mut window = 0 as *mut window;
    let mut tmp: *mut window = 0 as *mut window;
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
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_PREV(mut elm: *mut winlink) -> *mut winlink {
    if !(*elm).entry.rbe_left.is_null() {
        elm = (*elm).entry.rbe_left;
        while !(*elm).entry.rbe_right.is_null() {
            elm = (*elm).entry.rbe_right
        }
    } else if !(*elm).entry.rbe_parent.is_null()
        && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
    {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
        {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_MINMAX(
    mut head: *mut winlinks,
    mut val: libc::c_int,
) -> *mut winlink {
    let mut tmp: *mut winlink = (*head).rbh_root;
    let mut parent: *mut winlink = 0 as *mut winlink;
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
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_NEXT(mut elm: *mut winlink) -> *mut winlink {
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
pub unsafe extern "C" fn winlinks_RB_NFIND(
    mut head: *mut winlinks,
    mut elm: *mut winlink,
) -> *mut winlink {
    let mut tmp: *mut winlink = (*head).rbh_root;
    let mut res: *mut winlink = 0 as *mut winlink;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = winlink_cmp(elm, tmp);
        if comp < 0i32 {
            res = tmp;
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_FIND(
    mut head: *mut winlinks,
    mut elm: *mut winlink,
) -> *mut winlink {
    let mut tmp: *mut winlink = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = winlink_cmp(elm, tmp);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut winlink;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_INSERT(
    mut head: *mut winlinks,
    mut elm: *mut winlink,
) -> *mut winlink {
    let mut tmp: *mut winlink = 0 as *mut winlink;
    let mut parent: *mut winlink = 0 as *mut winlink;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = winlink_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut winlink;
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
    winlinks_RB_INSERT_COLOR(head, elm);
    return 0 as *mut winlink;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_REMOVE(
    mut head: *mut winlinks,
    mut elm: *mut winlink,
) -> *mut winlink {
    let mut current_block: u64;
    let mut child: *mut winlink = 0 as *mut winlink;
    let mut parent: *mut winlink = 0 as *mut winlink;
    let mut old: *mut winlink = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut winlink = 0 as *mut winlink;
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
        current_block = 14005405705760971010;
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
        winlinks_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_INSERT_COLOR(mut head: *mut winlinks, mut elm: *mut winlink) {
    let mut parent: *mut winlink = 0 as *mut winlink;
    let mut gparent: *mut winlink = 0 as *mut winlink;
    let mut tmp: *mut winlink = 0 as *mut winlink;
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
#[no_mangle]
pub unsafe extern "C" fn winlinks_RB_REMOVE_COLOR(
    mut head: *mut winlinks,
    mut parent: *mut winlink,
    mut elm: *mut winlink,
) {
    let mut tmp: *mut winlink = 0 as *mut winlink;
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
                    let mut oleft: *mut winlink = 0 as *mut winlink;
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
                    let mut oright: *mut winlink = 0 as *mut winlink;
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
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_INSERT_COLOR(
    mut head: *mut window_pane_tree,
    mut elm: *mut window_pane,
) {
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    let mut gparent: *mut window_pane = 0 as *mut window_pane;
    let mut tmp: *mut window_pane = 0 as *mut window_pane;
    loop {
        parent = (*elm).tree_entry.rbe_parent;
        if !(!parent.is_null() && (*parent).tree_entry.rbe_color == 1i32) {
            break;
        }
        gparent = (*parent).tree_entry.rbe_parent;
        if parent == (*gparent).tree_entry.rbe_left {
            tmp = (*gparent).tree_entry.rbe_right;
            if !tmp.is_null() && (*tmp).tree_entry.rbe_color == 1i32 {
                (*tmp).tree_entry.rbe_color = 0i32;
                (*parent).tree_entry.rbe_color = 0i32;
                (*gparent).tree_entry.rbe_color = 1i32;
                elm = gparent
            } else {
                if (*parent).tree_entry.rbe_right == elm {
                    tmp = (*parent).tree_entry.rbe_right;
                    (*parent).tree_entry.rbe_right = (*tmp).tree_entry.rbe_left;
                    if !(*parent).tree_entry.rbe_right.is_null() {
                        (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent = parent
                    }
                    (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                    if !(*tmp).tree_entry.rbe_parent.is_null() {
                        if parent == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left {
                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp
                        } else {
                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).tree_entry.rbe_left = parent;
                    (*parent).tree_entry.rbe_parent = tmp;
                    !(*tmp).tree_entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).tree_entry.rbe_color = 0i32;
                (*gparent).tree_entry.rbe_color = 1i32;
                tmp = (*gparent).tree_entry.rbe_left;
                (*gparent).tree_entry.rbe_left = (*tmp).tree_entry.rbe_right;
                if !(*gparent).tree_entry.rbe_left.is_null() {
                    (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent = gparent
                }
                (*tmp).tree_entry.rbe_parent = (*gparent).tree_entry.rbe_parent;
                if !(*tmp).tree_entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left {
                        (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp
                    } else {
                        (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).tree_entry.rbe_right = gparent;
                (*gparent).tree_entry.rbe_parent = tmp;
                !(*tmp).tree_entry.rbe_parent.is_null();
            }
        } else {
            tmp = (*gparent).tree_entry.rbe_left;
            if !tmp.is_null() && (*tmp).tree_entry.rbe_color == 1i32 {
                (*tmp).tree_entry.rbe_color = 0i32;
                (*parent).tree_entry.rbe_color = 0i32;
                (*gparent).tree_entry.rbe_color = 1i32;
                elm = gparent
            } else {
                if (*parent).tree_entry.rbe_left == elm {
                    tmp = (*parent).tree_entry.rbe_left;
                    (*parent).tree_entry.rbe_left = (*tmp).tree_entry.rbe_right;
                    if !(*parent).tree_entry.rbe_left.is_null() {
                        (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent = parent
                    }
                    (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                    if !(*tmp).tree_entry.rbe_parent.is_null() {
                        if parent == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left {
                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp
                        } else {
                            (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).tree_entry.rbe_right = parent;
                    (*parent).tree_entry.rbe_parent = tmp;
                    !(*tmp).tree_entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).tree_entry.rbe_color = 0i32;
                (*gparent).tree_entry.rbe_color = 1i32;
                tmp = (*gparent).tree_entry.rbe_right;
                (*gparent).tree_entry.rbe_right = (*tmp).tree_entry.rbe_left;
                if !(*gparent).tree_entry.rbe_right.is_null() {
                    (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent = gparent
                }
                (*tmp).tree_entry.rbe_parent = (*gparent).tree_entry.rbe_parent;
                if !(*tmp).tree_entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left {
                        (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp
                    } else {
                        (*(*gparent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).tree_entry.rbe_left = gparent;
                (*gparent).tree_entry.rbe_parent = tmp;
                !(*tmp).tree_entry.rbe_parent.is_null();
            }
        }
    }
    (*(*head).rbh_root).tree_entry.rbe_color = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_REMOVE_COLOR(
    mut head: *mut window_pane_tree,
    mut parent: *mut window_pane,
    mut elm: *mut window_pane,
) {
    let mut tmp: *mut window_pane = 0 as *mut window_pane;
    while (elm.is_null() || (*elm).tree_entry.rbe_color == 0i32) && elm != (*head).rbh_root {
        if (*parent).tree_entry.rbe_left == elm {
            tmp = (*parent).tree_entry.rbe_right;
            if (*tmp).tree_entry.rbe_color == 1i32 {
                (*tmp).tree_entry.rbe_color = 0i32;
                (*parent).tree_entry.rbe_color = 1i32;
                tmp = (*parent).tree_entry.rbe_right;
                (*parent).tree_entry.rbe_right = (*tmp).tree_entry.rbe_left;
                if !(*parent).tree_entry.rbe_right.is_null() {
                    (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent = parent
                }
                (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                if !(*tmp).tree_entry.rbe_parent.is_null() {
                    if parent == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp
                    } else {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).tree_entry.rbe_left = parent;
                (*parent).tree_entry.rbe_parent = tmp;
                !(*tmp).tree_entry.rbe_parent.is_null();
                tmp = (*parent).tree_entry.rbe_right
            }
            if ((*tmp).tree_entry.rbe_left.is_null()
                || (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color == 0i32)
                && ((*tmp).tree_entry.rbe_right.is_null()
                    || (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color == 0i32)
            {
                (*tmp).tree_entry.rbe_color = 1i32;
                elm = parent;
                parent = (*elm).tree_entry.rbe_parent
            } else {
                if (*tmp).tree_entry.rbe_right.is_null()
                    || (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color == 0i32
                {
                    let mut oleft: *mut window_pane = 0 as *mut window_pane;
                    oleft = (*tmp).tree_entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).tree_entry.rbe_color = 0i32
                    }
                    (*tmp).tree_entry.rbe_color = 1i32;
                    oleft = (*tmp).tree_entry.rbe_left;
                    (*tmp).tree_entry.rbe_left = (*oleft).tree_entry.rbe_right;
                    if !(*tmp).tree_entry.rbe_left.is_null() {
                        (*(*oleft).tree_entry.rbe_right).tree_entry.rbe_parent = tmp
                    }
                    (*oleft).tree_entry.rbe_parent = (*tmp).tree_entry.rbe_parent;
                    if !(*oleft).tree_entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left = oleft
                        } else {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_right = oleft
                        }
                    } else {
                        (*head).rbh_root = oleft
                    }
                    (*oleft).tree_entry.rbe_right = tmp;
                    (*tmp).tree_entry.rbe_parent = oleft;
                    !(*oleft).tree_entry.rbe_parent.is_null();
                    tmp = (*parent).tree_entry.rbe_right
                }
                (*tmp).tree_entry.rbe_color = (*parent).tree_entry.rbe_color;
                (*parent).tree_entry.rbe_color = 0i32;
                if !(*tmp).tree_entry.rbe_right.is_null() {
                    (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color = 0i32
                }
                tmp = (*parent).tree_entry.rbe_right;
                (*parent).tree_entry.rbe_right = (*tmp).tree_entry.rbe_left;
                if !(*parent).tree_entry.rbe_right.is_null() {
                    (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_parent = parent
                }
                (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                if !(*tmp).tree_entry.rbe_parent.is_null() {
                    if parent == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp
                    } else {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).tree_entry.rbe_left = parent;
                (*parent).tree_entry.rbe_parent = tmp;
                !(*tmp).tree_entry.rbe_parent.is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).tree_entry.rbe_left;
            if (*tmp).tree_entry.rbe_color == 1i32 {
                (*tmp).tree_entry.rbe_color = 0i32;
                (*parent).tree_entry.rbe_color = 1i32;
                tmp = (*parent).tree_entry.rbe_left;
                (*parent).tree_entry.rbe_left = (*tmp).tree_entry.rbe_right;
                if !(*parent).tree_entry.rbe_left.is_null() {
                    (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent = parent
                }
                (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                if !(*tmp).tree_entry.rbe_parent.is_null() {
                    if parent == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp
                    } else {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).tree_entry.rbe_right = parent;
                (*parent).tree_entry.rbe_parent = tmp;
                !(*tmp).tree_entry.rbe_parent.is_null();
                tmp = (*parent).tree_entry.rbe_left
            }
            if ((*tmp).tree_entry.rbe_left.is_null()
                || (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color == 0i32)
                && ((*tmp).tree_entry.rbe_right.is_null()
                    || (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_color == 0i32)
            {
                (*tmp).tree_entry.rbe_color = 1i32;
                elm = parent;
                parent = (*elm).tree_entry.rbe_parent
            } else {
                if (*tmp).tree_entry.rbe_left.is_null()
                    || (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color == 0i32
                {
                    let mut oright: *mut window_pane = 0 as *mut window_pane;
                    oright = (*tmp).tree_entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).tree_entry.rbe_color = 0i32
                    }
                    (*tmp).tree_entry.rbe_color = 1i32;
                    oright = (*tmp).tree_entry.rbe_right;
                    (*tmp).tree_entry.rbe_right = (*oright).tree_entry.rbe_left;
                    if !(*tmp).tree_entry.rbe_right.is_null() {
                        (*(*oright).tree_entry.rbe_left).tree_entry.rbe_parent = tmp
                    }
                    (*oright).tree_entry.rbe_parent = (*tmp).tree_entry.rbe_parent;
                    if !(*oright).tree_entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_left = oright
                        } else {
                            (*(*tmp).tree_entry.rbe_parent).tree_entry.rbe_right = oright
                        }
                    } else {
                        (*head).rbh_root = oright
                    }
                    (*oright).tree_entry.rbe_left = tmp;
                    (*tmp).tree_entry.rbe_parent = oright;
                    !(*oright).tree_entry.rbe_parent.is_null();
                    tmp = (*parent).tree_entry.rbe_left
                }
                (*tmp).tree_entry.rbe_color = (*parent).tree_entry.rbe_color;
                (*parent).tree_entry.rbe_color = 0i32;
                if !(*tmp).tree_entry.rbe_left.is_null() {
                    (*(*tmp).tree_entry.rbe_left).tree_entry.rbe_color = 0i32
                }
                tmp = (*parent).tree_entry.rbe_left;
                (*parent).tree_entry.rbe_left = (*tmp).tree_entry.rbe_right;
                if !(*parent).tree_entry.rbe_left.is_null() {
                    (*(*tmp).tree_entry.rbe_right).tree_entry.rbe_parent = parent
                }
                (*tmp).tree_entry.rbe_parent = (*parent).tree_entry.rbe_parent;
                if !(*tmp).tree_entry.rbe_parent.is_null() {
                    if parent == (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_left = tmp
                    } else {
                        (*(*parent).tree_entry.rbe_parent).tree_entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).tree_entry.rbe_right = parent;
                (*parent).tree_entry.rbe_parent = tmp;
                !(*tmp).tree_entry.rbe_parent.is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).tree_entry.rbe_color = 0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_NEXT(mut elm: *mut window_pane) -> *mut window_pane {
    if !(*elm).tree_entry.rbe_right.is_null() {
        elm = (*elm).tree_entry.rbe_right;
        while !(*elm).tree_entry.rbe_left.is_null() {
            elm = (*elm).tree_entry.rbe_left
        }
    } else if !(*elm).tree_entry.rbe_parent.is_null()
        && elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_left
    {
        elm = (*elm).tree_entry.rbe_parent
    } else {
        while !(*elm).tree_entry.rbe_parent.is_null()
            && elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_right
        {
            elm = (*elm).tree_entry.rbe_parent
        }
        elm = (*elm).tree_entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_PREV(mut elm: *mut window_pane) -> *mut window_pane {
    if !(*elm).tree_entry.rbe_left.is_null() {
        elm = (*elm).tree_entry.rbe_left;
        while !(*elm).tree_entry.rbe_right.is_null() {
            elm = (*elm).tree_entry.rbe_right
        }
    } else if !(*elm).tree_entry.rbe_parent.is_null()
        && elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_right
    {
        elm = (*elm).tree_entry.rbe_parent
    } else {
        while !(*elm).tree_entry.rbe_parent.is_null()
            && elm == (*(*elm).tree_entry.rbe_parent).tree_entry.rbe_left
        {
            elm = (*elm).tree_entry.rbe_parent
        }
        elm = (*elm).tree_entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_NFIND(
    mut head: *mut window_pane_tree,
    mut elm: *mut window_pane,
) -> *mut window_pane {
    let mut tmp: *mut window_pane = (*head).rbh_root;
    let mut res: *mut window_pane = 0 as *mut window_pane;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = window_pane_cmp(elm, tmp);
        if comp < 0i32 {
            res = tmp;
            tmp = (*tmp).tree_entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).tree_entry.rbe_right
        } else {
            return tmp;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_MINMAX(
    mut head: *mut window_pane_tree,
    mut val: libc::c_int,
) -> *mut window_pane {
    let mut tmp: *mut window_pane = (*head).rbh_root;
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).tree_entry.rbe_left
        } else {
            tmp = (*tmp).tree_entry.rbe_right
        }
    }
    return parent;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_INSERT(
    mut head: *mut window_pane_tree,
    mut elm: *mut window_pane,
) -> *mut window_pane {
    let mut tmp: *mut window_pane = 0 as *mut window_pane;
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = window_pane_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).tree_entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).tree_entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).tree_entry.rbe_parent = parent;
    (*elm).tree_entry.rbe_right = 0 as *mut window_pane;
    (*elm).tree_entry.rbe_left = (*elm).tree_entry.rbe_right;
    (*elm).tree_entry.rbe_color = 1i32;
    if !parent.is_null() {
        if comp < 0i32 {
            (*parent).tree_entry.rbe_left = elm
        } else {
            (*parent).tree_entry.rbe_right = elm
        }
    } else {
        (*head).rbh_root = elm
    }
    window_pane_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut window_pane;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_FIND(
    mut head: *mut window_pane_tree,
    mut elm: *mut window_pane,
) -> *mut window_pane {
    let mut tmp: *mut window_pane = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = window_pane_cmp(elm, tmp);
        if comp < 0i32 {
            tmp = (*tmp).tree_entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).tree_entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut window_pane;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_tree_RB_REMOVE(
    mut head: *mut window_pane_tree,
    mut elm: *mut window_pane,
) -> *mut window_pane {
    let mut current_block: u64;
    let mut child: *mut window_pane = 0 as *mut window_pane;
    let mut parent: *mut window_pane = 0 as *mut window_pane;
    let mut old: *mut window_pane = elm;
    let mut color: libc::c_int = 0;
    if (*elm).tree_entry.rbe_left.is_null() {
        child = (*elm).tree_entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).tree_entry.rbe_right.is_null() {
        child = (*elm).tree_entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut window_pane = 0 as *mut window_pane;
        elm = (*elm).tree_entry.rbe_right;
        loop {
            left = (*elm).tree_entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left
        }
        child = (*elm).tree_entry.rbe_right;
        parent = (*elm).tree_entry.rbe_parent;
        color = (*elm).tree_entry.rbe_color;
        if !child.is_null() {
            (*child).tree_entry.rbe_parent = parent
        }
        if !parent.is_null() {
            if (*parent).tree_entry.rbe_left == elm {
                (*parent).tree_entry.rbe_left = child
            } else {
                (*parent).tree_entry.rbe_right = child
            }
        } else {
            (*head).rbh_root = child
        }
        if (*elm).tree_entry.rbe_parent == old {
            parent = elm
        }
        (*elm).tree_entry = (*old).tree_entry;
        if !(*old).tree_entry.rbe_parent.is_null() {
            if (*(*old).tree_entry.rbe_parent).tree_entry.rbe_left == old {
                (*(*old).tree_entry.rbe_parent).tree_entry.rbe_left = elm
            } else {
                (*(*old).tree_entry.rbe_parent).tree_entry.rbe_right = elm
            }
        } else {
            (*head).rbh_root = elm
        }
        (*(*old).tree_entry.rbe_left).tree_entry.rbe_parent = elm;
        if !(*old).tree_entry.rbe_right.is_null() {
            (*(*old).tree_entry.rbe_right).tree_entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).tree_entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 17407180261566896428;
    }
    match current_block {
        7226443171521532240 => {
            parent = (*elm).tree_entry.rbe_parent;
            color = (*elm).tree_entry.rbe_color;
            if !child.is_null() {
                (*child).tree_entry.rbe_parent = parent
            }
            if !parent.is_null() {
                if (*parent).tree_entry.rbe_left == elm {
                    (*parent).tree_entry.rbe_left = child
                } else {
                    (*parent).tree_entry.rbe_right = child
                }
            } else {
                (*head).rbh_root = child
            }
        }
        _ => {}
    }
    if color == 0i32 {
        window_pane_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn window_cmp(mut w1: *mut window, mut w2: *mut window) -> libc::c_int {
    return (*w1).id.wrapping_sub((*w2).id) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_cmp(mut wl1: *mut winlink, mut wl2: *mut winlink) -> libc::c_int {
    return (*wl1).idx - (*wl2).idx;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_cmp(
    mut wp1: *mut window_pane,
    mut wp2: *mut window_pane,
) -> libc::c_int {
    return (*wp1).id.wrapping_sub((*wp2).id) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_find_by_window(
    mut wwl: *mut winlinks,
    mut w: *mut window,
) -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(wwl, -(1i32));
    while !wl.is_null() {
        if (*wl).window == w {
            return wl;
        }
        wl = winlinks_RB_NEXT(wl)
    }
    return 0 as *mut winlink;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_find_by_index(
    mut wwl: *mut winlinks,
    mut idx: libc::c_int,
) -> *mut winlink {
    let mut wl: winlink = winlink {
        idx: 0,
        session: 0 as *mut session,
        window: 0 as *mut window,
        flags: 0,
        entry: C2RustUnnamed_17 {
            rbe_left: 0 as *mut winlink,
            rbe_right: 0 as *mut winlink,
            rbe_parent: 0 as *mut winlink,
            rbe_color: 0,
        },
        wentry: C2RustUnnamed_16 {
            tqe_next: 0 as *mut winlink,
            tqe_prev: 0 as *mut *mut winlink,
        },
        sentry: C2RustUnnamed_15 {
            tqe_next: 0 as *mut winlink,
            tqe_prev: 0 as *mut *mut winlink,
        },
    };
    if idx < 0i32 {
        fatalx(b"bad index\x00" as *const u8 as *const libc::c_char);
    }
    wl.idx = idx;
    return winlinks_RB_FIND(wwl, &mut wl);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_find_by_window_id(
    mut wwl: *mut winlinks,
    mut id: u_int,
) -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(wwl, -(1i32));
    while !wl.is_null() {
        if (*(*wl).window).id == id {
            return wl;
        }
        wl = winlinks_RB_NEXT(wl)
    }
    return 0 as *mut winlink;
}
unsafe extern "C" fn winlink_next_index(
    mut wwl: *mut winlinks,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = idx;
    loop {
        if winlink_find_by_index(wwl, i).is_null() {
            return i;
        }
        if i == 2147483647i32 {
            i = 0i32
        } else {
            i += 1
        }
        if !(i != idx) {
            break;
        }
    }
    return -(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_count(mut wwl: *mut winlinks) -> u_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut n: u_int = 0;
    n = 0u32;
    wl = winlinks_RB_MINMAX(wwl, -(1i32));
    while !wl.is_null() {
        n = n.wrapping_add(1);
        wl = winlinks_RB_NEXT(wl)
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_add(mut wwl: *mut winlinks, mut idx: libc::c_int) -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if idx < 0i32 {
        idx = winlink_next_index(wwl, -idx - 1i32);
        if idx == -(1i32) {
            return 0 as *mut winlink;
        }
    } else if !winlink_find_by_index(wwl, idx).is_null() {
        return 0 as *mut winlink;
    }
    wl = xcalloc(1u64, ::std::mem::size_of::<winlink>() as libc::c_ulong) as *mut winlink;
    (*wl).idx = idx;
    winlinks_RB_INSERT(wwl, wl);
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_set_window(mut wl: *mut winlink, mut w: *mut window) {
    if !(*wl).window.is_null() {
        if !(*wl).wentry.tqe_next.is_null() {
            (*(*wl).wentry.tqe_next).wentry.tqe_prev = (*wl).wentry.tqe_prev
        } else {
            (*(*wl).window).winlinks.tqh_last = (*wl).wentry.tqe_prev
        }
        *(*wl).wentry.tqe_prev = (*wl).wentry.tqe_next;
        window_remove_ref(
            (*wl).window,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"winlink_set_window\x00"))
                .as_ptr(),
        );
    }
    (*wl).wentry.tqe_next = 0 as *mut winlink;
    (*wl).wentry.tqe_prev = (*w).winlinks.tqh_last;
    *(*w).winlinks.tqh_last = wl;
    (*w).winlinks.tqh_last = &mut (*wl).wentry.tqe_next;
    (*wl).window = w;
    window_add_ref(
        w,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"winlink_set_window\x00"))
            .as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn winlink_remove(mut wwl: *mut winlinks, mut wl: *mut winlink) {
    let mut w: *mut window = (*wl).window;
    if !w.is_null() {
        if !(*wl).wentry.tqe_next.is_null() {
            (*(*wl).wentry.tqe_next).wentry.tqe_prev = (*wl).wentry.tqe_prev
        } else {
            (*w).winlinks.tqh_last = (*wl).wentry.tqe_prev
        }
        *(*wl).wentry.tqe_prev = (*wl).wentry.tqe_next;
        window_remove_ref(
            w,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"winlink_remove\x00"))
                .as_ptr(),
        );
    }
    winlinks_RB_REMOVE(wwl, wl);
    free(wl as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_next(mut wl: *mut winlink) -> *mut winlink {
    return winlinks_RB_NEXT(wl);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_previous(mut wl: *mut winlink) -> *mut winlink {
    return winlinks_RB_PREV(wl);
}
#[no_mangle]
pub unsafe extern "C" fn winlink_next_by_number(
    mut wl: *mut winlink,
    mut s: *mut session,
    mut n: libc::c_int,
) -> *mut winlink {
    while n > 0i32 {
        wl = winlinks_RB_NEXT(wl);
        if wl.is_null() {
            wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1i32))
        }
        n -= 1
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_previous_by_number(
    mut wl: *mut winlink,
    mut s: *mut session,
    mut n: libc::c_int,
) -> *mut winlink {
    while n > 0i32 {
        wl = winlinks_RB_PREV(wl);
        if wl.is_null() {
            wl = winlinks_RB_MINMAX(&mut (*s).windows, 1i32)
        }
        n -= 1
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_stack_push(mut stack: *mut winlink_stack, mut wl: *mut winlink) {
    if wl.is_null() {
        return;
    }
    winlink_stack_remove(stack, wl);
    (*wl).sentry.tqe_next = (*stack).tqh_first;
    if !(*wl).sentry.tqe_next.is_null() {
        (*(*stack).tqh_first).sentry.tqe_prev = &mut (*wl).sentry.tqe_next
    } else {
        (*stack).tqh_last = &mut (*wl).sentry.tqe_next
    }
    (*stack).tqh_first = wl;
    (*wl).sentry.tqe_prev = &mut (*stack).tqh_first;
}
#[no_mangle]
pub unsafe extern "C" fn winlink_stack_remove(mut stack: *mut winlink_stack, mut wl: *mut winlink) {
    let mut wl2: *mut winlink = 0 as *mut winlink;
    if wl.is_null() {
        return;
    }
    wl2 = (*stack).tqh_first;
    while !wl2.is_null() {
        if wl2 == wl {
            if !(*wl).sentry.tqe_next.is_null() {
                (*(*wl).sentry.tqe_next).sentry.tqe_prev = (*wl).sentry.tqe_prev
            } else {
                (*stack).tqh_last = (*wl).sentry.tqe_prev
            }
            *(*wl).sentry.tqe_prev = (*wl).sentry.tqe_next;
            return;
        }
        wl2 = (*wl2).sentry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_find_by_id_str(mut s: *const libc::c_char) -> *mut window {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut id: u_int = 0;
    if *s as libc::c_int != '@' as i32 {
        return 0 as *mut window;
    }
    id = strtonum(
        s.offset(1isize),
        0i64,
        (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32) as libc::c_longlong,
        &mut errstr,
    ) as u_int;
    if !errstr.is_null() {
        return 0 as *mut window;
    }
    return window_find_by_id(id);
}
#[no_mangle]
pub unsafe extern "C" fn window_find_by_id(mut id: u_int) -> *mut window {
    let mut w: window = window {
        id: 0,
        latest: 0 as *mut libc::c_void,
        name: 0 as *mut libc::c_char,
        name_event: event {
            ev_evcallback: event_callback {
                evcb_active_next: C2RustUnnamed_8 {
                    tqe_next: 0 as *mut event_callback,
                    tqe_prev: 0 as *mut *mut event_callback,
                },
                evcb_flags: 0,
                evcb_pri: 0,
                evcb_closure: 0,
                evcb_cb_union: C2RustUnnamed_7 {
                    evcb_callback: None,
                },
                evcb_arg: 0 as *mut libc::c_void,
            },
            ev_timeout_pos: C2RustUnnamed_5 {
                ev_next_with_common_timeout: C2RustUnnamed_6 {
                    tqe_next: 0 as *mut event,
                    tqe_prev: 0 as *mut *mut event,
                },
            },
            ev_fd: 0,
            ev_base: 0 as *mut event_base,
            ev_: C2RustUnnamed_0 {
                ev_io: C2RustUnnamed_3 {
                    ev_io_next: C2RustUnnamed_4 {
                        le_next: 0 as *mut event,
                        le_prev: 0 as *mut *mut event,
                    },
                    ev_timeout: timeval {
                        tv_sec: 0,
                        tv_usec: 0,
                    },
                },
            },
            ev_events: 0,
            ev_res: 0,
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
        name_time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        alerts_timer: event {
            ev_evcallback: event_callback {
                evcb_active_next: C2RustUnnamed_8 {
                    tqe_next: 0 as *mut event_callback,
                    tqe_prev: 0 as *mut *mut event_callback,
                },
                evcb_flags: 0,
                evcb_pri: 0,
                evcb_closure: 0,
                evcb_cb_union: C2RustUnnamed_7 {
                    evcb_callback: None,
                },
                evcb_arg: 0 as *mut libc::c_void,
            },
            ev_timeout_pos: C2RustUnnamed_5 {
                ev_next_with_common_timeout: C2RustUnnamed_6 {
                    tqe_next: 0 as *mut event,
                    tqe_prev: 0 as *mut *mut event,
                },
            },
            ev_fd: 0,
            ev_base: 0 as *mut event_base,
            ev_: C2RustUnnamed_0 {
                ev_io: C2RustUnnamed_3 {
                    ev_io_next: C2RustUnnamed_4 {
                        le_next: 0 as *mut event,
                        le_prev: 0 as *mut *mut event,
                    },
                    ev_timeout: timeval {
                        tv_sec: 0,
                        tv_usec: 0,
                    },
                },
            },
            ev_events: 0,
            ev_res: 0,
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
        offset_timer: event {
            ev_evcallback: event_callback {
                evcb_active_next: C2RustUnnamed_8 {
                    tqe_next: 0 as *mut event_callback,
                    tqe_prev: 0 as *mut *mut event_callback,
                },
                evcb_flags: 0,
                evcb_pri: 0,
                evcb_closure: 0,
                evcb_cb_union: C2RustUnnamed_7 {
                    evcb_callback: None,
                },
                evcb_arg: 0 as *mut libc::c_void,
            },
            ev_timeout_pos: C2RustUnnamed_5 {
                ev_next_with_common_timeout: C2RustUnnamed_6 {
                    tqe_next: 0 as *mut event,
                    tqe_prev: 0 as *mut *mut event,
                },
            },
            ev_fd: 0,
            ev_base: 0 as *mut event_base,
            ev_: C2RustUnnamed_0 {
                ev_io: C2RustUnnamed_3 {
                    ev_io_next: C2RustUnnamed_4 {
                        le_next: 0 as *mut event,
                        le_prev: 0 as *mut *mut event,
                    },
                    ev_timeout: timeval {
                        tv_sec: 0,
                        tv_usec: 0,
                    },
                },
            },
            ev_events: 0,
            ev_res: 0,
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
        activity_time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        active: 0 as *mut window_pane,
        last: 0 as *mut window_pane,
        panes: window_panes {
            tqh_first: 0 as *mut window_pane,
            tqh_last: 0 as *mut *mut window_pane,
        },
        lastlayout: 0,
        layout_root: 0 as *mut layout_cell,
        saved_layout_root: 0 as *mut layout_cell,
        old_layout: 0 as *mut libc::c_char,
        sx: 0,
        sy: 0,
        xpixel: 0,
        ypixel: 0,
        new_sx: 0,
        new_sy: 0,
        new_xpixel: 0,
        new_ypixel: 0,
        flags: 0,
        alerts_queued: 0,
        alerts_entry: C2RustUnnamed_20 {
            tqe_next: 0 as *mut window,
            tqe_prev: 0 as *mut *mut window,
        },
        options: 0 as *mut crate::options::options,
        references: 0,
        winlinks: C2RustUnnamed_19 {
            tqh_first: 0 as *mut winlink,
            tqh_last: 0 as *mut *mut winlink,
        },
        entry: C2RustUnnamed_18 {
            rbe_left: 0 as *mut window,
            rbe_right: 0 as *mut window,
            rbe_parent: 0 as *mut window,
            rbe_color: 0,
        },
    };
    w.id = id;
    return windows_RB_FIND(&mut windows, &mut w);
}
#[no_mangle]
pub unsafe extern "C" fn window_update_activity(mut w: *mut window) {
    gettimeofday(&mut (*w).activity_time, 0 as *mut libc::c_void);
    alerts_queue(w, 0x2i32);
}
#[no_mangle]
pub unsafe extern "C" fn window_create(
    mut sx: u_int,
    mut sy: u_int,
    mut xpixel: u_int,
    mut ypixel: u_int,
) -> *mut window {
    let mut w: *mut window = 0 as *mut window;
    if xpixel == 0u32 {
        xpixel = 16u32
    }
    if ypixel == 0u32 {
        ypixel = 32u32
    }
    w = xcalloc(1u64, ::std::mem::size_of::<window>() as libc::c_ulong) as *mut window;
    (*w).name = xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    (*w).flags = 0i32;
    (*w).panes.tqh_first = 0 as *mut window_pane;
    (*w).panes.tqh_last = &mut (*w).panes.tqh_first;
    (*w).active = 0 as *mut window_pane;
    (*w).lastlayout = -(1i32);
    (*w).layout_root = 0 as *mut layout_cell;
    (*w).sx = sx;
    (*w).sy = sy;
    (*w).xpixel = xpixel;
    (*w).ypixel = ypixel;
    (*w).options = options_create(global_w_options);
    (*w).references = 0u32;
    (*w).winlinks.tqh_first = 0 as *mut winlink;
    (*w).winlinks.tqh_last = &mut (*w).winlinks.tqh_first;
    let fresh0 = next_window_id;
    next_window_id = next_window_id.wrapping_add(1);
    (*w).id = fresh0;
    windows_RB_INSERT(&mut windows, w);
    window_update_activity(w);
    return w;
}
unsafe extern "C" fn window_destroy(mut w: *mut window) {
    log_debug(
        b"window @%u destroyed (%d references)\x00" as *const u8 as *const libc::c_char,
        (*w).id,
        (*w).references,
    );
    windows_RB_REMOVE(&mut windows, w);
    if !(*w).layout_root.is_null() {
        layout_free_cell((*w).layout_root);
    }
    if !(*w).saved_layout_root.is_null() {
        layout_free_cell((*w).saved_layout_root);
    }
    free((*w).old_layout as *mut libc::c_void);
    window_destroy_panes(w);
    if event_initialized(&mut (*w).name_event) != 0 {
        event_del(&mut (*w).name_event);
    }
    if event_initialized(&mut (*w).alerts_timer) != 0 {
        event_del(&mut (*w).alerts_timer);
    }
    if event_initialized(&mut (*w).offset_timer) != 0 {
        event_del(&mut (*w).offset_timer);
    }
    options_free((*w).options);
    free((*w).name as *mut libc::c_void);
    free(w as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_destroy_ready(mut wp: *mut window_pane) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if (*wp).pipe_fd != -(1i32) {
        if evbuffer_get_length((*(*wp).pipe_event).output) != 0u64 {
            return 0i32;
        }
        if ioctl((*wp).fd, 0x541bu64, &mut n as *mut libc::c_int) != -(1i32) && n > 0i32 {
            return 0i32;
        }
    }
    if !(*wp).flags & 0x100i32 != 0 {
        return 0i32;
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_add_ref(mut w: *mut window, mut from: *const libc::c_char) {
    (*w).references = (*w).references.wrapping_add(1);
    log_debug(
        b"%s: @%u %s, now %d\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"window_add_ref\x00")).as_ptr(),
        (*w).id,
        from,
        (*w).references,
    );
}
#[no_mangle]
pub unsafe extern "C" fn window_remove_ref(mut w: *mut window, mut from: *const libc::c_char) {
    (*w).references = (*w).references.wrapping_sub(1);
    log_debug(
        b"%s: @%u %s, now %d\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"window_remove_ref\x00"))
            .as_ptr(),
        (*w).id,
        from,
        (*w).references,
    );
    if (*w).references == 0u32 {
        window_destroy(w);
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_set_name(mut w: *mut window, mut new_name: *const libc::c_char) {
    free((*w).name as *mut libc::c_void);
    utf8_stravis(&mut (*w).name, new_name, 0x1i32 | 0x2i32 | 0x8i32 | 0x10i32);
    notify_window(b"window-renamed\x00" as *const u8 as *const libc::c_char, w);
}
#[no_mangle]
pub unsafe extern "C" fn window_resize(
    mut w: *mut window,
    mut sx: u_int,
    mut sy: u_int,
    mut xpixel: libc::c_int,
    mut ypixel: libc::c_int,
) {
    if xpixel == 0i32 {
        xpixel = 16i32
    }
    if ypixel == 0i32 {
        ypixel = 32i32
    }
    log_debug(
        b"%s: @%u resize %ux%u (%ux%u)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"window_resize\x00")).as_ptr(),
        (*w).id,
        sx,
        sy,
        if xpixel == -(1i32) {
            (*w).xpixel
        } else {
            xpixel as u_int
        },
        if ypixel == -(1i32) {
            (*w).ypixel
        } else {
            ypixel as u_int
        },
    );
    (*w).sx = sx;
    (*w).sy = sy;
    if xpixel != -(1i32) {
        (*w).xpixel = xpixel as u_int
    }
    if ypixel != -(1i32) {
        (*w).ypixel = ypixel as u_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_send_resize(mut wp: *mut window_pane, mut force: libc::c_int) {
    let mut w: *mut window = (*wp).window;
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut sy: u_int = 0;
    if (*wp).fd == -(1i32) {
        return;
    }
    if force == 0 {
        sy = (*wp).sy
    } else if (*wp).sy <= 1u32 {
        sy = (*wp).sy.wrapping_add(1u32)
    } else {
        sy = (*wp).sy.wrapping_sub(1u32)
    }
    log_debug(
        b"%s: %%%u resize to %u,%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"window_pane_send_resize\x00"))
            .as_ptr(),
        (*wp).id,
        (*wp).sx,
        sy,
    );
    memset(
        &mut ws as *mut winsize as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<winsize>() as libc::c_ulong,
    );
    ws.ws_col = (*wp).sx as libc::c_ushort;
    ws.ws_row = sy as libc::c_ushort;
    ws.ws_xpixel = (*w).xpixel.wrapping_mul(ws.ws_col as libc::c_uint) as libc::c_ushort;
    ws.ws_ypixel = (*w).ypixel.wrapping_mul(ws.ws_row as libc::c_uint) as libc::c_ushort;
    if ioctl((*wp).fd, 0x5414u64, &mut ws as *mut winsize) == -(1i32) {
        fatal(b"ioctl failed\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_has_pane(
    mut w: *mut window,
    mut wp: *mut window_pane,
) -> libc::c_int {
    let mut wp1: *mut window_pane = 0 as *mut window_pane;
    wp1 = (*w).panes.tqh_first;
    while !wp1.is_null() {
        if wp1 == wp {
            return 1i32;
        }
        wp1 = (*wp1).entry.tqe_next
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_set_active_pane(
    mut w: *mut window,
    mut wp: *mut window_pane,
    mut notify: libc::c_int,
) -> libc::c_int {
    log_debug(
        b"%s: pane %%%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"window_set_active_pane\x00"))
            .as_ptr(),
        (*wp).id,
    );
    if wp == (*w).active {
        return 0i32;
    }
    (*w).last = (*w).active;
    (*w).active = wp;
    let fresh1 = next_active_point;
    next_active_point = next_active_point.wrapping_add(1);
    (*(*w).active).active_point = fresh1;
    (*(*w).active).flags |= 0x80i32;
    tty_update_window_offset(w);
    if notify != 0 {
        notify_window(
            b"window-pane-changed\x00" as *const u8 as *const libc::c_char,
            w,
        );
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_redraw_active_switch(mut w: *mut window, mut wp: *mut window_pane) {
    let mut gc1: *mut GridCell = 0 as *mut GridCell;
    let mut gc2: *mut GridCell = 0 as *mut GridCell;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    if wp == (*w).active {
        return;
    }
    loop {
        /*
         * If the active and inactive styles or palettes are different,
         * need to redraw the panes.
         */
        gc1 = &mut (*wp).cached_gc;
        gc2 = &mut (*wp).cached_active_gc;
        if grid_cells_look_equal(gc1, gc2) == 0 {
            (*wp).flags |= 0x1i32
        } else {
            c1 = window_pane_get_palette(wp, (*gc1).fg);
            c2 = window_pane_get_palette(wp, (*gc2).fg);
            if c1 != c2 {
                (*wp).flags |= 0x1i32
            } else {
                c1 = window_pane_get_palette(wp, (*gc1).bg);
                c2 = window_pane_get_palette(wp, (*gc2).bg);
                if c1 != c2 {
                    (*wp).flags |= 0x1i32
                }
            }
        }
        if wp == (*w).active {
            break;
        }
        wp = (*w).active
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_get_active_at(
    mut w: *mut window,
    mut x: u_int,
    mut y: u_int,
) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if !(window_pane_visible(wp) == 0) {
            if !(x < (*wp).xoff || x > (*wp).xoff.wrapping_add((*wp).sx)) {
                if !(y < (*wp).yoff || y > (*wp).yoff.wrapping_add((*wp).sy)) {
                    return wp;
                }
            }
        }
        wp = (*wp).entry.tqe_next
    }
    return 0 as *mut window_pane;
}
#[no_mangle]
pub unsafe extern "C" fn window_find_string(
    mut w: *mut window,
    mut s: *const libc::c_char,
) -> *mut window_pane {
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut top: u_int = 0u32;
    let mut bottom: u_int = (*w).sy.wrapping_sub(1u32);
    let mut status: libc::c_int = 0;
    x = (*w).sx.wrapping_div(2u32);
    y = (*w).sy.wrapping_div(2u32);
    status = options_get_number(
        (*w).options,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if status == 1i32 {
        top = top.wrapping_add(1)
    } else if status == 2i32 {
        bottom = bottom.wrapping_sub(1)
    }
    if strcasecmp(s, b"top\x00" as *const u8 as *const libc::c_char) == 0i32 {
        y = top
    } else if strcasecmp(s, b"bottom\x00" as *const u8 as *const libc::c_char) == 0i32 {
        y = bottom
    } else if strcasecmp(s, b"left\x00" as *const u8 as *const libc::c_char) == 0i32 {
        x = 0u32
    } else if strcasecmp(s, b"right\x00" as *const u8 as *const libc::c_char) == 0i32 {
        x = (*w).sx.wrapping_sub(1u32)
    } else if strcasecmp(s, b"top-left\x00" as *const u8 as *const libc::c_char) == 0i32 {
        x = 0u32;
        y = top
    } else if strcasecmp(s, b"top-right\x00" as *const u8 as *const libc::c_char) == 0i32 {
        x = (*w).sx.wrapping_sub(1u32);
        y = top
    } else if strcasecmp(s, b"bottom-left\x00" as *const u8 as *const libc::c_char) == 0i32 {
        x = 0u32;
        y = bottom
    } else if strcasecmp(s, b"bottom-right\x00" as *const u8 as *const libc::c_char) == 0i32 {
        x = (*w).sx.wrapping_sub(1u32);
        y = bottom
    } else {
        return 0 as *mut window_pane;
    }
    return window_get_active_at(w, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn window_zoom(mut wp: *mut window_pane) -> libc::c_int {
    let mut w: *mut window = (*wp).window;
    let mut wp1: *mut window_pane = 0 as *mut window_pane;
    if (*w).flags & 0x8i32 != 0 {
        return -(1i32);
    }
    if window_count_panes(w) == 1u32 {
        return -(1i32);
    }
    if (*w).active != wp {
        window_set_active_pane(w, wp, 1i32);
    }
    wp1 = (*w).panes.tqh_first;
    while !wp1.is_null() {
        (*wp1).saved_layout_cell = (*wp1).layout_cell;
        (*wp1).layout_cell = 0 as *mut layout_cell;
        wp1 = (*wp1).entry.tqe_next
    }
    (*w).saved_layout_root = (*w).layout_root;
    layout_init(w, wp);
    (*w).flags |= 0x8i32;
    notify_window(
        b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
        w,
    );
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_unzoom(mut w: *mut window) -> libc::c_int {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if (*w).flags & 0x8i32 == 0 {
        return -(1i32);
    }
    (*w).flags &= !(0x8i32);
    layout_free(w);
    (*w).layout_root = (*w).saved_layout_root;
    (*w).saved_layout_root = 0 as *mut layout_cell;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        (*wp).layout_cell = (*wp).saved_layout_cell;
        (*wp).saved_layout_cell = 0 as *mut layout_cell;
        wp = (*wp).entry.tqe_next
    }
    layout_fix_panes(w);
    notify_window(
        b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
        w,
    );
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_push_zoom(
    mut w: *mut window,
    mut flag: libc::c_int,
) -> libc::c_int {
    log_debug(
        b"%s: @%u %d\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"window_push_zoom\x00"))
            .as_ptr(),
        (*w).id,
        (flag != 0 && (*w).flags & 0x8i32 != 0) as libc::c_int,
    );
    if flag != 0 && (*w).flags & 0x8i32 != 0 {
        (*w).flags |= 0x10i32
    } else {
        (*w).flags &= !(0x10i32)
    }
    return (window_unzoom(w) == 0i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pop_zoom(mut w: *mut window) -> libc::c_int {
    log_debug(
        b"%s: @%u %d\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"window_pop_zoom\x00")).as_ptr(),
        (*w).id,
        ((*w).flags & 0x10i32 != 0) as libc::c_int,
    );
    if (*w).flags & 0x10i32 != 0 {
        return (window_zoom((*w).active) == 0i32) as libc::c_int;
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_add_pane(
    mut w: *mut window,
    mut other: *mut window_pane,
    mut hlimit: u_int,
    mut flags: libc::c_int,
) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if other.is_null() {
        other = (*w).active
    }
    wp = window_pane_create(w, (*w).sx, (*w).sy, hlimit);
    if (*w).panes.tqh_first.is_null() {
        log_debug(
            b"%s: @%u at start\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"window_add_pane\x00"))
                .as_ptr(),
            (*w).id,
        );
        (*wp).entry.tqe_next = (*w).panes.tqh_first;
        if !(*wp).entry.tqe_next.is_null() {
            (*(*w).panes.tqh_first).entry.tqe_prev = &mut (*wp).entry.tqe_next
        } else {
            (*w).panes.tqh_last = &mut (*wp).entry.tqe_next
        }
        (*w).panes.tqh_first = wp;
        (*wp).entry.tqe_prev = &mut (*w).panes.tqh_first
    } else if flags & 0x8i32 != 0 {
        log_debug(
            b"%s: @%u before %%%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"window_add_pane\x00"))
                .as_ptr(),
            (*w).id,
            (*wp).id,
        );
        if flags & 0x20i32 != 0 {
            (*wp).entry.tqe_next = (*w).panes.tqh_first;
            if !(*wp).entry.tqe_next.is_null() {
                (*(*w).panes.tqh_first).entry.tqe_prev = &mut (*wp).entry.tqe_next
            } else {
                (*w).panes.tqh_last = &mut (*wp).entry.tqe_next
            }
            (*w).panes.tqh_first = wp;
            (*wp).entry.tqe_prev = &mut (*w).panes.tqh_first
        } else {
            (*wp).entry.tqe_prev = (*other).entry.tqe_prev;
            (*wp).entry.tqe_next = other;
            *(*other).entry.tqe_prev = wp;
            (*other).entry.tqe_prev = &mut (*wp).entry.tqe_next
        }
    } else {
        log_debug(
            b"%s: @%u after %%%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"window_add_pane\x00"))
                .as_ptr(),
            (*w).id,
            (*wp).id,
        );
        if flags & 0x20i32 != 0 {
            (*wp).entry.tqe_next = 0 as *mut window_pane;
            (*wp).entry.tqe_prev = (*w).panes.tqh_last;
            *(*w).panes.tqh_last = wp;
            (*w).panes.tqh_last = &mut (*wp).entry.tqe_next
        } else {
            (*wp).entry.tqe_next = (*other).entry.tqe_next;
            if !(*wp).entry.tqe_next.is_null() {
                (*(*wp).entry.tqe_next).entry.tqe_prev = &mut (*wp).entry.tqe_next
            } else {
                (*w).panes.tqh_last = &mut (*wp).entry.tqe_next
            }
            (*other).entry.tqe_next = wp;
            (*wp).entry.tqe_prev = &mut (*other).entry.tqe_next
        }
    }
    return wp;
}
#[no_mangle]
pub unsafe extern "C" fn window_lost_pane(mut w: *mut window, mut wp: *mut window_pane) {
    log_debug(
        b"%s: @%u pane %%%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"window_lost_pane\x00"))
            .as_ptr(),
        (*w).id,
        (*wp).id,
    );
    if wp == marked_pane.wp {
        server_clear_marked();
    }
    if wp == (*w).active {
        (*w).active = (*w).last;
        (*w).last = 0 as *mut window_pane;
        if (*w).active.is_null() {
            (*w).active = *(*((*wp).entry.tqe_prev as *mut window_panes)).tqh_last;
            if (*w).active.is_null() {
                (*w).active = (*wp).entry.tqe_next
            }
        }
        if !(*w).active.is_null() {
            (*(*w).active).flags |= 0x80i32;
            notify_window(
                b"window-pane-changed\x00" as *const u8 as *const libc::c_char,
                w,
            );
        }
    } else if wp == (*w).last {
        (*w).last = 0 as *mut window_pane
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_remove_pane(mut w: *mut window, mut wp: *mut window_pane) {
    window_lost_pane(w, wp);
    if !(*wp).entry.tqe_next.is_null() {
        (*(*wp).entry.tqe_next).entry.tqe_prev = (*wp).entry.tqe_prev
    } else {
        (*w).panes.tqh_last = (*wp).entry.tqe_prev
    }
    *(*wp).entry.tqe_prev = (*wp).entry.tqe_next;
    window_pane_destroy(wp);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_at_index(
    mut w: *mut window,
    mut idx: u_int,
) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut n: u_int = 0;
    n = options_get_number(
        (*w).options,
        b"pane-base-index\x00" as *const u8 as *const libc::c_char,
    ) as u_int;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if n == idx {
            return wp;
        }
        n = n.wrapping_add(1);
        wp = (*wp).entry.tqe_next
    }
    return 0 as *mut window_pane;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_next_by_number(
    mut w: *mut window,
    mut wp: *mut window_pane,
    mut n: u_int,
) -> *mut window_pane {
    while n > 0u32 {
        wp = (*wp).entry.tqe_next;
        if wp.is_null() {
            wp = (*w).panes.tqh_first
        }
        n = n.wrapping_sub(1)
    }
    return wp;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_previous_by_number(
    mut w: *mut window,
    mut wp: *mut window_pane,
    mut n: u_int,
) -> *mut window_pane {
    while n > 0u32 {
        wp = *(*((*wp).entry.tqe_prev as *mut window_panes)).tqh_last;
        if wp.is_null() {
            wp = *(*((*w).panes.tqh_last as *mut window_panes)).tqh_last
        }
        n = n.wrapping_sub(1)
    }
    return wp;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_index(
    mut wp: *mut window_pane,
    mut i: *mut u_int,
) -> libc::c_int {
    let mut wq: *mut window_pane = 0 as *mut window_pane;
    let mut w: *mut window = (*wp).window;
    *i = options_get_number(
        (*w).options,
        b"pane-base-index\x00" as *const u8 as *const libc::c_char,
    ) as u_int;
    wq = (*w).panes.tqh_first;
    while !wq.is_null() {
        if wp == wq {
            return 0i32;
        }
        *i = (*i).wrapping_add(1);
        wq = (*wq).entry.tqe_next
    }
    return -(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn window_count_panes(mut w: *mut window) -> u_int {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut n: u_int = 0;
    n = 0u32;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        n = n.wrapping_add(1);
        wp = (*wp).entry.tqe_next
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn window_destroy_panes(mut w: *mut window) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    while !(*w).panes.tqh_first.is_null() {
        wp = (*w).panes.tqh_first;
        if !(*wp).entry.tqe_next.is_null() {
            (*(*wp).entry.tqe_next).entry.tqe_prev = (*wp).entry.tqe_prev
        } else {
            (*w).panes.tqh_last = (*wp).entry.tqe_prev
        }
        *(*wp).entry.tqe_prev = (*wp).entry.tqe_next;
        window_pane_destroy(wp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_printable_flags(mut wl: *mut winlink) -> *const libc::c_char {
    let mut s: *mut session = (*wl).session;
    static mut flags: [libc::c_char; 32] = [0; 32];
    let mut pos: libc::c_int = 0;
    pos = 0i32;
    if (*wl).flags & 0x2i32 != 0 {
        let fresh2 = pos;
        pos = pos + 1;
        flags[fresh2 as usize] = '#' as libc::c_char
    }
    if (*wl).flags & 0x1i32 != 0 {
        let fresh3 = pos;
        pos = pos + 1;
        flags[fresh3 as usize] = '!' as libc::c_char
    }
    if (*wl).flags & 0x4i32 != 0 {
        let fresh4 = pos;
        pos = pos + 1;
        flags[fresh4 as usize] = '~' as libc::c_char
    }
    if wl == (*s).curw {
        let fresh5 = pos;
        pos = pos + 1;
        flags[fresh5 as usize] = '*' as libc::c_char
    }
    if wl == (*s).lastw.tqh_first {
        let fresh6 = pos;
        pos = pos + 1;
        flags[fresh6 as usize] = '-' as libc::c_char
    }
    if server_check_marked() != 0 && wl == marked_pane.wl {
        let fresh7 = pos;
        pos = pos + 1;
        flags[fresh7 as usize] = 'M' as libc::c_char
    }
    if (*(*wl).window).flags & 0x8i32 != 0 {
        let fresh8 = pos;
        pos = pos + 1;
        flags[fresh8 as usize] = 'Z' as libc::c_char
    }
    flags[pos as usize] = '\u{0}' as libc::c_char;
    return flags.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_by_id_str(
    mut s: *const libc::c_char,
) -> *mut window_pane {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut id: u_int = 0;
    if *s as libc::c_int != '%' as i32 {
        return 0 as *mut window_pane;
    }
    id = strtonum(
        s.offset(1isize),
        0i64,
        (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32) as libc::c_longlong,
        &mut errstr,
    ) as u_int;
    if !errstr.is_null() {
        return 0 as *mut window_pane;
    }
    return window_pane_find_by_id(id);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_by_id(mut id: u_int) -> *mut window_pane {
    let mut wp: window_pane = window_pane {
        id: 0,
        active_point: 0,
        window: 0 as *mut window,
        options: 0 as *mut crate::options::options,
        layout_cell: 0 as *mut layout_cell,
        saved_layout_cell: 0 as *mut layout_cell,
        sx: 0,
        sy: 0,
        xoff: 0,
        yoff: 0,
        fg: 0,
        bg: 0,
        flags: 0,
        argc: 0,
        argv: 0 as *mut *mut libc::c_char,
        shell: 0 as *mut libc::c_char,
        cwd: 0 as *mut libc::c_char,
        pid: 0,
        tty: [0; 32],
        status: 0,
        fd: 0,
        event: 0 as *mut bufferevent,
        offset: window_pane_offset { used: 0 },
        base_offset: 0,
        resize_timer: event {
            ev_evcallback: event_callback {
                evcb_active_next: C2RustUnnamed_8 {
                    tqe_next: 0 as *mut event_callback,
                    tqe_prev: 0 as *mut *mut event_callback,
                },
                evcb_flags: 0,
                evcb_pri: 0,
                evcb_closure: 0,
                evcb_cb_union: C2RustUnnamed_7 {
                    evcb_callback: None,
                },
                evcb_arg: 0 as *mut libc::c_void,
            },
            ev_timeout_pos: C2RustUnnamed_5 {
                ev_next_with_common_timeout: C2RustUnnamed_6 {
                    tqe_next: 0 as *mut event,
                    tqe_prev: 0 as *mut *mut event,
                },
            },
            ev_fd: 0,
            ev_base: 0 as *mut event_base,
            ev_: C2RustUnnamed_0 {
                ev_io: C2RustUnnamed_3 {
                    ev_io_next: C2RustUnnamed_4 {
                        le_next: 0 as *mut event,
                        le_prev: 0 as *mut *mut event,
                    },
                    ev_timeout: timeval {
                        tv_sec: 0,
                        tv_usec: 0,
                    },
                },
            },
            ev_events: 0,
            ev_res: 0,
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
        force_timer: event {
            ev_evcallback: event_callback {
                evcb_active_next: C2RustUnnamed_8 {
                    tqe_next: 0 as *mut event_callback,
                    tqe_prev: 0 as *mut *mut event_callback,
                },
                evcb_flags: 0,
                evcb_pri: 0,
                evcb_closure: 0,
                evcb_cb_union: C2RustUnnamed_7 {
                    evcb_callback: None,
                },
                evcb_arg: 0 as *mut libc::c_void,
            },
            ev_timeout_pos: C2RustUnnamed_5 {
                ev_next_with_common_timeout: C2RustUnnamed_6 {
                    tqe_next: 0 as *mut event,
                    tqe_prev: 0 as *mut *mut event,
                },
            },
            ev_fd: 0,
            ev_base: 0 as *mut event_base,
            ev_: C2RustUnnamed_0 {
                ev_io: C2RustUnnamed_3 {
                    ev_io_next: C2RustUnnamed_4 {
                        le_next: 0 as *mut event,
                        le_prev: 0 as *mut *mut event,
                    },
                    ev_timeout: timeval {
                        tv_sec: 0,
                        tv_usec: 0,
                    },
                },
            },
            ev_events: 0,
            ev_res: 0,
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
        ictx: 0 as *mut crate::input::input_ctx,
        cached_gc: GridCell {
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
        },
        cached_active_gc: GridCell {
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
        },
        palette: 0 as *mut libc::c_int,
        pipe_fd: 0,
        pipe_event: 0 as *mut bufferevent,
        pipe_offset: window_pane_offset { used: 0 },
        screen: 0 as *mut screen,
        base: screen {
            title: 0 as *mut libc::c_char,
            path: 0 as *mut libc::c_char,
            titles: 0 as *mut crate::screen::screen_titles,
            grid: 0 as *mut grid,
            cx: 0,
            cy: 0,
            cstyle: 0,
            ccolour: 0 as *mut libc::c_char,
            rupper: 0,
            rlower: 0,
            mode: 0,
            saved_cx: 0,
            saved_cy: 0,
            saved_grid: 0 as *mut grid,
            saved_cell: GridCell {
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
            },
            saved_flags: 0,
            tabs: 0 as *mut bitstr_t,
            sel: 0 as *mut crate::screen::screen_sel,
            write_list: 0 as *mut crate::screen_write::screen_write_collect_line,
        },
        status_screen: screen {
            title: 0 as *mut libc::c_char,
            path: 0 as *mut libc::c_char,
            titles: 0 as *mut crate::screen::screen_titles,
            grid: 0 as *mut grid,
            cx: 0,
            cy: 0,
            cstyle: 0,
            ccolour: 0 as *mut libc::c_char,
            rupper: 0,
            rlower: 0,
            mode: 0,
            saved_cx: 0,
            saved_cy: 0,
            saved_grid: 0 as *mut grid,
            saved_cell: GridCell {
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
            },
            saved_flags: 0,
            tabs: 0 as *mut bitstr_t,
            sel: 0 as *mut crate::screen::screen_sel,
            write_list: 0 as *mut crate::screen_write::screen_write_collect_line,
        },
        status_size: 0,
        modes: C2RustUnnamed_24 {
            tqh_first: 0 as *mut window_mode_entry,
            tqh_last: 0 as *mut *mut window_mode_entry,
        },
        searchstr: 0 as *mut libc::c_char,
        searchregex: 0,
        written: 0,
        skipped: 0,
        border_gc_set: 0,
        border_gc: GridCell {
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
        },
        entry: C2RustUnnamed_23 {
            tqe_next: 0 as *mut window_pane,
            tqe_prev: 0 as *mut *mut window_pane,
        },
        tree_entry: C2RustUnnamed_22 {
            rbe_left: 0 as *mut window_pane,
            rbe_right: 0 as *mut window_pane,
            rbe_parent: 0 as *mut window_pane,
            rbe_color: 0,
        },
    };
    wp.id = id;
    return window_pane_tree_RB_FIND(&mut all_window_panes, &mut wp);
}
unsafe extern "C" fn window_pane_create(
    mut w: *mut window,
    mut sx: u_int,
    mut sy: u_int,
    mut hlimit: u_int,
) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut host: [libc::c_char; 65] = [0; 65];
    wp = xcalloc(1u64, ::std::mem::size_of::<window_pane>() as libc::c_ulong) as *mut window_pane;
    (*wp).window = w;
    (*wp).options = options_create((*w).options);
    (*wp).flags = 0x1000i32;
    let fresh9 = next_window_pane_id;
    next_window_pane_id = next_window_pane_id.wrapping_add(1);
    (*wp).id = fresh9;
    window_pane_tree_RB_INSERT(&mut all_window_panes, wp);
    (*wp).argc = 0i32;
    (*wp).argv = 0 as *mut *mut libc::c_char;
    (*wp).shell = 0 as *mut libc::c_char;
    (*wp).cwd = 0 as *mut libc::c_char;
    (*wp).fd = -(1i32);
    (*wp).event = 0 as *mut bufferevent;
    (*wp).fg = 8i32;
    (*wp).bg = 8i32;
    (*wp).modes.tqh_first = 0 as *mut window_mode_entry;
    (*wp).modes.tqh_last = &mut (*wp).modes.tqh_first;
    (*wp).layout_cell = 0 as *mut layout_cell;
    (*wp).xoff = 0u32;
    (*wp).yoff = 0u32;
    (*wp).sx = sx;
    (*wp).sy = sy;
    (*wp).pipe_fd = -(1i32);
    (*wp).pipe_event = 0 as *mut bufferevent;
    screen_init(&mut (*wp).base, sx, sy, hlimit);
    (*wp).screen = &mut (*wp).base;
    screen_init(&mut (*wp).status_screen, 1u32, 1u32, 0u32);
    if gethostname(
        host.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong,
    ) == 0i32
    {
        screen_set_title(&mut (*wp).base, host.as_mut_ptr());
    }
    return wp;
}
unsafe extern "C" fn window_pane_destroy(mut wp: *mut window_pane) {
    window_pane_reset_mode_all(wp);
    free((*wp).searchstr as *mut libc::c_void);
    if (*wp).fd != -(1i32) {
        bufferevent_free((*wp).event);
        close((*wp).fd);
    }
    if !(*wp).ictx.is_null() {
        input_free((*wp).ictx);
    }
    screen_free(&mut (*wp).status_screen);
    screen_free(&mut (*wp).base);
    if (*wp).pipe_fd != -(1i32) {
        bufferevent_free((*wp).pipe_event);
        close((*wp).pipe_fd);
    }
    if event_initialized(&mut (*wp).resize_timer) != 0 {
        event_del(&mut (*wp).resize_timer);
    }
    if event_initialized(&mut (*wp).force_timer) != 0 {
        event_del(&mut (*wp).force_timer);
    }
    window_pane_tree_RB_REMOVE(&mut all_window_panes, wp);
    options_free((*wp).options);
    free((*wp).cwd as *mut libc::c_void);
    free((*wp).shell as *mut libc::c_void);
    cmd_free_argv((*wp).argc, (*wp).argv);
    free((*wp).palette as *mut libc::c_void);
    free(wp as *mut libc::c_void);
}
unsafe extern "C" fn window_pane_read_callback(
    mut _bufev: *mut bufferevent,
    mut data: *mut libc::c_void,
) {
    let mut wp: *mut window_pane = data as *mut window_pane;
    let mut evb: *mut evbuffer = (*(*wp).event).input;
    let mut wpo: *mut window_pane_offset = &mut (*wp).pipe_offset;
    let mut size: size_t = evbuffer_get_length(evb);
    let mut new_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_size: size_t = 0;
    let mut c: *mut client = 0 as *mut client;
    if (*wp).pipe_fd != -(1i32) {
        new_data = window_pane_get_new_data(wp, wpo, &mut new_size) as *mut libc::c_char;
        if new_size > 0u64 {
            bufferevent_write((*wp).pipe_event, new_data as *const libc::c_void, new_size);
            window_pane_update_used_data(wp, wpo, new_size);
        }
    }
    log_debug(
        b"%%%u has %zu bytes\x00" as *const u8 as *const libc::c_char,
        (*wp).id,
        size,
    );
    c = clients.tqh_first;
    while !c.is_null() {
        if !(*c).session.is_null() && (*c).flags & 0x2000u64 != 0 {
            control_write_output(c, wp);
        }
        c = (*c).entry.tqe_next
    }
    input_parse_pane(wp);
    bufferevent_disable((*wp).event, 0x2i16);
}
unsafe extern "C" fn window_pane_error_callback(
    mut _bufev: *mut bufferevent,
    mut _what: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut wp: *mut window_pane = data as *mut window_pane;
    log_debug(
        b"%%%u error\x00" as *const u8 as *const libc::c_char,
        (*wp).id,
    );
    (*wp).flags |= 0x100i32;
    if window_pane_destroy_ready(wp) != 0 {
        server_destroy_pane(wp, 1i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_set_event(mut wp: *mut window_pane) {
    setblocking((*wp).fd, 0i32);
    (*wp).event = bufferevent_new(
        (*wp).fd,
        Some(
            window_pane_read_callback
                as unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void) -> (),
        ),
        None,
        Some(
            window_pane_error_callback
                as unsafe extern "C" fn(
                    _: *mut bufferevent,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        wp as *mut libc::c_void,
    );
    (*wp).ictx = input_init(wp, (*wp).event);
    bufferevent_enable((*wp).event, (0x2i32 | 0x4i32) as libc::c_short);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_resize(
    mut wp: *mut window_pane,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    if sx == (*wp).sx && sy == (*wp).sy {
        return;
    }
    (*wp).sx = sx;
    (*wp).sy = sy;
    log_debug(
        b"%s: %%%u resize %ux%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"window_pane_resize\x00"))
            .as_ptr(),
        (*wp).id,
        sx,
        sy,
    );
    screen_resize(
        &mut (*wp).base,
        sx,
        sy,
        ((*wp).base.saved_grid == 0 as *mut grid) as libc::c_int,
    );
    wme = (*wp).modes.tqh_first;
    if !wme.is_null() && (*(*wme).mode).resize.is_some() {
        (*(*wme).mode).resize.expect("non-null function pointer")(wme, sx, sy);
    }
    /*
     * If the pane has already been resized, set the force flag and make
     * the application resize twice to force it to redraw.
     */
    if (*wp).flags & 0x8i32 != 0 {
        (*wp).flags |= 0x10i32
    }
    (*wp).flags |= 0x8i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_set_palette(
    mut wp: *mut window_pane,
    mut n: u_int,
    mut colour: libc::c_int,
) {
    if n > 0xffu32 {
        return;
    }
    if (*wp).palette.is_null() {
        (*wp).palette = xcalloc(
            0x100u64,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int
    }
    *(*wp).palette.offset(n as isize) = colour;
    (*wp).flags |= 0x1i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_unset_palette(mut wp: *mut window_pane, mut n: u_int) {
    if n > 0xffu32 || (*wp).palette.is_null() {
        return;
    }
    *(*wp).palette.offset(n as isize) = 0i32;
    (*wp).flags |= 0x1i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_reset_palette(mut wp: *mut window_pane) {
    if (*wp).palette.is_null() {
        return;
    }
    free((*wp).palette as *mut libc::c_void);
    (*wp).palette = 0 as *mut libc::c_int;
    (*wp).flags |= 0x1i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_get_palette(
    mut wp: *mut window_pane,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut new: libc::c_int = 0;
    if wp.is_null() || (*wp).palette.is_null() {
        return -(1i32);
    }
    new = -(1i32);
    if c < 8i32 {
        new = *(*wp).palette.offset(c as isize)
    } else if c >= 90i32 && c <= 97i32 {
        new = *(*wp).palette.offset((8i32 + c - 90i32) as isize)
    } else if c & 0x1000000i32 != 0 {
        new = *(*wp).palette.offset((c & !(0x1000000i32)) as isize)
    }
    if new == 0i32 {
        return -(1i32);
    }
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_set_mode(
    mut wp: *mut window_pane,
    mut swp: *mut window_pane,
    mut mode: *const window_mode,
    mut fs: *mut cmd_find_state,
    mut args: *mut args,
) -> libc::c_int {
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    if !(*wp).modes.tqh_first.is_null() && (*(*wp).modes.tqh_first).mode == mode {
        return 1i32;
    }
    wme = (*wp).modes.tqh_first;
    while !wme.is_null() {
        if (*wme).mode == mode {
            break;
        }
        wme = (*wme).entry.tqe_next
    }
    if !wme.is_null() {
        if !(*wme).entry.tqe_next.is_null() {
            (*(*wme).entry.tqe_next).entry.tqe_prev = (*wme).entry.tqe_prev
        } else {
            (*wp).modes.tqh_last = (*wme).entry.tqe_prev
        }
        *(*wme).entry.tqe_prev = (*wme).entry.tqe_next;
        (*wme).entry.tqe_next = (*wp).modes.tqh_first;
        if !(*wme).entry.tqe_next.is_null() {
            (*(*wp).modes.tqh_first).entry.tqe_prev = &mut (*wme).entry.tqe_next
        } else {
            (*wp).modes.tqh_last = &mut (*wme).entry.tqe_next
        }
        (*wp).modes.tqh_first = wme;
        (*wme).entry.tqe_prev = &mut (*wp).modes.tqh_first
    } else {
        wme = xcalloc(
            1u64,
            ::std::mem::size_of::<window_mode_entry>() as libc::c_ulong,
        ) as *mut window_mode_entry;
        (*wme).wp = wp;
        (*wme).swp = swp;
        (*wme).mode = mode;
        (*wme).prefix = 1u32;
        (*wme).entry.tqe_next = (*wp).modes.tqh_first;
        if !(*wme).entry.tqe_next.is_null() {
            (*(*wp).modes.tqh_first).entry.tqe_prev = &mut (*wme).entry.tqe_next
        } else {
            (*wp).modes.tqh_last = &mut (*wme).entry.tqe_next
        }
        (*wp).modes.tqh_first = wme;
        (*wme).entry.tqe_prev = &mut (*wp).modes.tqh_first;
        (*wme).screen = (*(*wme).mode).init.expect("non-null function pointer")(wme, fs, args)
    }
    (*wp).screen = (*wme).screen;
    (*wp).flags |= 0x1i32 | 0x80i32;
    server_redraw_window_borders((*wp).window);
    server_status_window((*wp).window);
    notify_pane(
        b"pane-mode-changed\x00" as *const u8 as *const libc::c_char,
        wp,
    );
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_reset_mode(mut wp: *mut window_pane) {
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    let mut next: *mut window_mode_entry = 0 as *mut window_mode_entry;
    if (*wp).modes.tqh_first.is_null() {
        return;
    }
    wme = (*wp).modes.tqh_first;
    if !(*wme).entry.tqe_next.is_null() {
        (*(*wme).entry.tqe_next).entry.tqe_prev = (*wme).entry.tqe_prev
    } else {
        (*wp).modes.tqh_last = (*wme).entry.tqe_prev
    }
    *(*wme).entry.tqe_prev = (*wme).entry.tqe_next;
    (*(*wme).mode).free.expect("non-null function pointer")(wme);
    free(wme as *mut libc::c_void);
    next = (*wp).modes.tqh_first;
    if next.is_null() {
        log_debug(
            b"%s: no next mode\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"window_pane_reset_mode\x00",
            ))
            .as_ptr(),
        );
        (*wp).screen = &mut (*wp).base
    } else {
        log_debug(
            b"%s: next mode is %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"window_pane_reset_mode\x00",
            ))
            .as_ptr(),
            (*(*next).mode).name,
        );
        (*wp).screen = (*next).screen;
        if (*(*next).mode).resize.is_some() {
            (*(*next).mode).resize.expect("non-null function pointer")(next, (*wp).sx, (*wp).sy);
        }
    }
    (*wp).flags |= 0x1i32 | 0x80i32;
    server_redraw_window_borders((*wp).window);
    server_status_window((*wp).window);
    notify_pane(
        b"pane-mode-changed\x00" as *const u8 as *const libc::c_char,
        wp,
    );
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_reset_mode_all(mut wp: *mut window_pane) {
    while !(*wp).modes.tqh_first.is_null() {
        window_pane_reset_mode(wp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_key(
    mut wp: *mut window_pane,
    mut c: *mut client,
    mut s: *mut session,
    mut wl: *mut winlink,
    mut key: key_code,
    mut m: *mut mouse_event,
) -> libc::c_int {
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    let mut wp2: *mut window_pane = 0 as *mut window_pane;
    if key & 0xfffffffffffu64 >= key_code_code::MOUSE
        && (key & 0xfffffffffffu64) < key_code_code::BSPACE
        && m.is_null()
    {
        return -(1i32);
    }
    wme = (*wp).modes.tqh_first;
    if !wme.is_null() {
        if (*(*wme).mode).key.is_some() && !c.is_null() {
            key &= !(0xff000000000000u64);
            (*(*wme).mode).key.expect("non-null function pointer")(wme, c, s, wl, key, m);
        }
        return 0i32;
    }
    if (*wp).fd == -(1i32) || (*wp).flags & 0x40i32 != 0 {
        return 0i32;
    }
    if input_key_pane(wp, key, m) != 0i32 {
        return -(1i32);
    }
    if key & 0xfffffffffffu64 >= key_code_code::MOUSE
        && (key & 0xfffffffffffu64) < key_code_code::BSPACE
    {
        return 0i32;
    }
    if options_get_number(
        (*(*wp).window).options,
        b"synchronize-panes\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        wp2 = (*(*wp).window).panes.tqh_first;
        while !wp2.is_null() {
            if wp2 != wp
                && (*wp2).modes.tqh_first.is_null()
                && (*wp2).fd != -(1i32)
                && !(*wp2).flags & 0x40i32 != 0
                && window_pane_visible(wp2) != 0
            {
                input_key_pane(wp2, key, 0 as *mut mouse_event);
            }
            wp2 = (*wp2).entry.tqe_next
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_visible(mut wp: *mut window_pane) -> libc::c_int {
    if !(*(*wp).window).flags & 0x8i32 != 0 {
        return 1i32;
    }
    return (wp == (*(*wp).window).active) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_search(
    mut wp: *mut window_pane,
    mut term: *const libc::c_char,
    mut regex: libc::c_int,
    mut ignore: libc::c_int,
) -> u_int {
    let mut s: *mut screen = &mut (*wp).base;
    let mut r: regex_t = regex_t {
        buffer: 0 as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *mut libc::c_char,
        translate: 0 as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    let mut flags: libc::c_int = 0i32;
    let mut found: libc::c_int = 0;
    let mut n: size_t = 0;
    if regex == 0 {
        if ignore != 0 {
            flags |= (1i32) << 4i32
        }
        xasprintf(
            &mut new as *mut *mut libc::c_char,
            b"*%s*\x00" as *const u8 as *const libc::c_char,
            term,
        );
    } else {
        if ignore != 0 {
            flags |= (1i32) << 1i32
        }
        if regcomp(&mut r, term, flags | 1i32) != 0i32 {
            return 0u32;
        }
    }
    i = 0u32;
    while i < (*(*s).grid).sy {
        line = grid_view_string_cells((*s).grid, 0u32, i, (*(*s).grid).sx);
        n = strlen(line);
        while n > 0u64 {
            if *(*__ctype_b_loc()).offset(
                *line.offset(n.wrapping_sub(1u64) as isize) as u_char as libc::c_int as isize
            ) as libc::c_int
                & _ISspace as libc::c_ushort as libc::c_int
                == 0
            {
                break;
            }
            *line.offset(n.wrapping_sub(1u64) as isize) = '\u{0}' as libc::c_char;
            n = n.wrapping_sub(1)
        }
        log_debug(
            b"%s: %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"window_pane_search\x00"))
                .as_ptr(),
            line,
        );
        if regex == 0 {
            found = (fnmatch(new, line, flags) == 0i32) as libc::c_int
        } else {
            found = (regexec(&mut r, line, 0u64, 0 as *mut regmatch_t, 0i32) == 0i32) as libc::c_int
        }
        free(line as *mut libc::c_void);
        if found != 0 {
            break;
        }
        i = i.wrapping_add(1)
    }
    if regex == 0 {
        free(new as *mut libc::c_void);
    } else {
        regfree(&mut r);
    }
    if i == (*(*s).grid).sy {
        return 0u32;
    }
    return i.wrapping_add(1u32);
}
/* Get MRU pane from a list. */
unsafe extern "C" fn window_pane_choose_best(
    mut list: *mut *mut window_pane,
    mut size: u_int,
) -> *mut window_pane {
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut i: u_int = 0;
    if size == 0u32 {
        return 0 as *mut window_pane;
    }
    best = *list.offset(0isize);
    i = 1u32;
    while i < size {
        next = *list.offset(i as isize);
        if (*next).active_point > (*best).active_point {
            best = next
        }
        i = i.wrapping_add(1)
    }
    return best;
}
/*
 * Find the pane directly above another. We build a list of those adjacent to
 * top edge and then choose the best.
 */
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_up(mut wp: *mut window_pane) -> *mut window_pane {
    let mut w: *mut window = 0 as *mut window;
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut left: u_int = 0;
    let mut right: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut status: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if wp.is_null() {
        return 0 as *mut window_pane;
    }
    w = (*wp).window;
    status = options_get_number(
        (*w).options,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    list = 0 as *mut *mut window_pane;
    size = 0u32;
    edge = (*wp).yoff;
    if status == 1i32 {
        if edge == 1u32 {
            edge = (*w).sy.wrapping_add(1u32)
        }
    } else if status == 2i32 {
        if edge == 0u32 {
            edge = (*w).sy
        }
    } else if edge == 0u32 {
        edge = (*w).sy.wrapping_add(1u32)
    }
    left = (*wp).xoff;
    right = (*wp).xoff.wrapping_add((*wp).sx);
    next = (*w).panes.tqh_first;
    while !next.is_null() {
        if !(next == wp) {
            if !((*next).yoff.wrapping_add((*next).sy).wrapping_add(1u32) != edge) {
                end = (*next).xoff.wrapping_add((*next).sx).wrapping_sub(1u32);
                found = 0i32;
                if (*next).xoff < left && end > right {
                    found = 1i32
                } else if (*next).xoff >= left && (*next).xoff <= right {
                    found = 1i32
                } else if end >= left && end <= right {
                    found = 1i32
                }
                if !(found == 0) {
                    list = xreallocarray(
                        list as *mut libc::c_void,
                        size.wrapping_add(1u32) as size_t,
                        ::std::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    ) as *mut *mut window_pane;
                    let fresh10 = size;
                    size = size.wrapping_add(1);
                    let ref mut fresh11 = *list.offset(fresh10 as isize);
                    *fresh11 = next
                }
            }
        }
        next = (*next).entry.tqe_next
    }
    best = window_pane_choose_best(list, size);
    free(list as *mut libc::c_void);
    return best;
}
/* Find the pane directly below another. */
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_down(mut wp: *mut window_pane) -> *mut window_pane {
    let mut w: *mut window = 0 as *mut window;
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut left: u_int = 0;
    let mut right: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut status: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if wp.is_null() {
        return 0 as *mut window_pane;
    }
    w = (*wp).window;
    status = options_get_number(
        (*w).options,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    list = 0 as *mut *mut window_pane;
    size = 0u32;
    edge = (*wp).yoff.wrapping_add((*wp).sy).wrapping_add(1u32);
    if status == 1i32 {
        if edge >= (*w).sy {
            edge = 1u32
        }
    } else if status == 2i32 {
        if edge >= (*w).sy.wrapping_sub(1u32) {
            edge = 0u32
        }
    } else if edge >= (*w).sy {
        edge = 0u32
    }
    left = (*wp).xoff;
    right = (*wp).xoff.wrapping_add((*wp).sx);
    next = (*w).panes.tqh_first;
    while !next.is_null() {
        if !(next == wp) {
            if !((*next).yoff != edge) {
                end = (*next).xoff.wrapping_add((*next).sx).wrapping_sub(1u32);
                found = 0i32;
                if (*next).xoff < left && end > right {
                    found = 1i32
                } else if (*next).xoff >= left && (*next).xoff <= right {
                    found = 1i32
                } else if end >= left && end <= right {
                    found = 1i32
                }
                if !(found == 0) {
                    list = xreallocarray(
                        list as *mut libc::c_void,
                        size.wrapping_add(1u32) as size_t,
                        ::std::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    ) as *mut *mut window_pane;
                    let fresh12 = size;
                    size = size.wrapping_add(1);
                    let ref mut fresh13 = *list.offset(fresh12 as isize);
                    *fresh13 = next
                }
            }
        }
        next = (*next).entry.tqe_next
    }
    best = window_pane_choose_best(list, size);
    free(list as *mut libc::c_void);
    return best;
}
/* Find the pane directly to the left of another. */
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_left(mut wp: *mut window_pane) -> *mut window_pane {
    let mut w: *mut window = 0 as *mut window;
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut top: u_int = 0;
    let mut bottom: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut found: libc::c_int = 0;
    if wp.is_null() {
        return 0 as *mut window_pane;
    }
    w = (*wp).window;
    list = 0 as *mut *mut window_pane;
    size = 0u32;
    edge = (*wp).xoff;
    if edge == 0u32 {
        edge = (*w).sx.wrapping_add(1u32)
    }
    top = (*wp).yoff;
    bottom = (*wp).yoff.wrapping_add((*wp).sy);
    next = (*w).panes.tqh_first;
    while !next.is_null() {
        if !(next == wp) {
            if !((*next).xoff.wrapping_add((*next).sx).wrapping_add(1u32) != edge) {
                end = (*next).yoff.wrapping_add((*next).sy).wrapping_sub(1u32);
                found = 0i32;
                if (*next).yoff < top && end > bottom {
                    found = 1i32
                } else if (*next).yoff >= top && (*next).yoff <= bottom {
                    found = 1i32
                } else if end >= top && end <= bottom {
                    found = 1i32
                }
                if !(found == 0) {
                    list = xreallocarray(
                        list as *mut libc::c_void,
                        size.wrapping_add(1u32) as size_t,
                        ::std::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    ) as *mut *mut window_pane;
                    let fresh14 = size;
                    size = size.wrapping_add(1);
                    let ref mut fresh15 = *list.offset(fresh14 as isize);
                    *fresh15 = next
                }
            }
        }
        next = (*next).entry.tqe_next
    }
    best = window_pane_choose_best(list, size);
    free(list as *mut libc::c_void);
    return best;
}
/* Find the pane directly to the right of another. */
#[no_mangle]
pub unsafe extern "C" fn window_pane_find_right(mut wp: *mut window_pane) -> *mut window_pane {
    let mut w: *mut window = 0 as *mut window;
    let mut next: *mut window_pane = 0 as *mut window_pane;
    let mut best: *mut window_pane = 0 as *mut window_pane;
    let mut list: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut edge: u_int = 0;
    let mut top: u_int = 0;
    let mut bottom: u_int = 0;
    let mut end: u_int = 0;
    let mut size: u_int = 0;
    let mut found: libc::c_int = 0;
    if wp.is_null() {
        return 0 as *mut window_pane;
    }
    w = (*wp).window;
    list = 0 as *mut *mut window_pane;
    size = 0u32;
    edge = (*wp).xoff.wrapping_add((*wp).sx).wrapping_add(1u32);
    if edge >= (*w).sx {
        edge = 0u32
    }
    top = (*wp).yoff;
    bottom = (*wp).yoff.wrapping_add((*wp).sy);
    next = (*w).panes.tqh_first;
    while !next.is_null() {
        if !(next == wp) {
            if !((*next).xoff != edge) {
                end = (*next).yoff.wrapping_add((*next).sy).wrapping_sub(1u32);
                found = 0i32;
                if (*next).yoff < top && end > bottom {
                    found = 1i32
                } else if (*next).yoff >= top && (*next).yoff <= bottom {
                    found = 1i32
                } else if end >= top && end <= bottom {
                    found = 1i32
                }
                if !(found == 0) {
                    list = xreallocarray(
                        list as *mut libc::c_void,
                        size.wrapping_add(1u32) as size_t,
                        ::std::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    ) as *mut *mut window_pane;
                    let fresh16 = size;
                    size = size.wrapping_add(1);
                    let ref mut fresh17 = *list.offset(fresh16 as isize);
                    *fresh17 = next
                }
            }
        }
        next = (*next).entry.tqe_next
    }
    best = window_pane_choose_best(list, size);
    free(list as *mut libc::c_void);
    return best;
}
/* Clear alert flags for a winlink */
#[no_mangle]
pub unsafe extern "C" fn winlink_clear_flags(mut wl: *mut winlink) {
    let mut loop_0: *mut winlink = 0 as *mut winlink;
    (*(*wl).window).flags &= !(0x1i32 | 0x2i32 | 0x4i32);
    loop_0 = (*(*wl).window).winlinks.tqh_first;
    while !loop_0.is_null() {
        if (*loop_0).flags & (0x1i32 | 0x2i32 | 0x4i32) != 0i32 {
            (*loop_0).flags &= !(0x1i32 | 0x2i32 | 0x4i32);
            server_status_session((*loop_0).session);
        }
        loop_0 = (*loop_0).wentry.tqe_next
    }
}
/* Shuffle window indexes up. */
#[no_mangle]
pub unsafe extern "C" fn winlink_shuffle_up(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut before: libc::c_int,
) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    if wl.is_null() {
        return -(1i32);
    }
    if before != 0 {
        idx = (*wl).idx
    } else {
        idx = (*wl).idx + 1i32
    }
    /* Find the next free index. */
    last = idx;
    while last < 2147483647i32 {
        if winlink_find_by_index(&mut (*s).windows, last).is_null() {
            break;
        }
        last += 1
    }
    if last == 2147483647i32 {
        return -(1i32);
    }
    /* Move everything from last - 1 to idx up a bit. */
    while last > idx {
        wl = winlink_find_by_index(&mut (*s).windows, last - 1i32);
        winlinks_RB_REMOVE(&mut (*s).windows, wl);
        (*wl).idx += 1;
        winlinks_RB_INSERT(&mut (*s).windows, wl);
        last -= 1
    }
    return idx;
}
unsafe extern "C" fn window_pane_input_callback(
    mut c: *mut client,
    mut _path: *const libc::c_char,
    mut error: libc::c_int,
    mut closed: libc::c_int,
    mut buffer: *mut evbuffer,
    mut data: *mut libc::c_void,
) {
    let mut cdata: *mut window_pane_input_data = data as *mut window_pane_input_data;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut buf: *mut u_char = evbuffer_pullup(buffer, -1i64);
    let mut len: size_t = evbuffer_get_length(buffer);
    wp = window_pane_find_by_id((*cdata).wp);
    if wp.is_null() || closed != 0 || error != 0i32 || (*c).flags & 0x200u64 != 0 {
        if wp.is_null() {
            (*c).flags |= 0x4u64
        }
        evbuffer_drain(buffer, len);
        cmdq_continue((*cdata).item);
        server_client_unref(c);
        free(cdata as *mut libc::c_void);
        return;
    }
    input_parse_buffer(wp, buf, len);
    evbuffer_drain(buffer, len);
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_start_input(
    mut wp: *mut window_pane,
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: *mut client = cmdq_get_client(item);
    let mut cdata: *mut window_pane_input_data = 0 as *mut window_pane_input_data;
    if !(*wp).flags & 0x800i32 != 0 {
        *cause = xstrdup(b"pane is not empty\x00" as *const u8 as *const libc::c_char);
        return -(1i32);
    }
    cdata = xmalloc(::std::mem::size_of::<window_pane_input_data>() as libc::c_ulong)
        as *mut window_pane_input_data;
    (*cdata).item = item;
    (*cdata).wp = (*wp).id;
    (*c).references += 1;
    file_read(
        c,
        b"-\x00" as *const u8 as *const libc::c_char,
        Some(
            window_pane_input_callback
                as unsafe extern "C" fn(
                    _: *mut client,
                    _: *const libc::c_char,
                    _: libc::c_int,
                    _: libc::c_int,
                    _: *mut evbuffer,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        cdata as *mut libc::c_void,
    );
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_get_new_data(
    mut wp: *mut window_pane,
    mut wpo: *mut window_pane_offset,
    mut size: *mut size_t,
) -> *mut libc::c_void {
    let mut used: size_t = (*wpo).used.wrapping_sub((*wp).base_offset);
    *size = evbuffer_get_length((*(*wp).event).input).wrapping_sub(used);
    return evbuffer_pullup((*(*wp).event).input, -1i64).offset(used as isize) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn window_pane_update_used_data(
    mut wp: *mut window_pane,
    mut wpo: *mut window_pane_offset,
    mut size: size_t,
) {
    let mut used: size_t = (*wpo).used.wrapping_sub((*wp).base_offset);
    if size > evbuffer_get_length((*(*wp).event).input).wrapping_sub(used) {
        size = evbuffer_get_length((*(*wp).event).input).wrapping_sub(used)
    }
    (*wpo).used = ((*wpo).used).wrapping_add(size);
}
