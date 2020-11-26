use ::libc;
extern "C" {
    pub type screen_write_collect_line;
    pub type screen_sel;
    pub type screen_titles;
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
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
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn __b64_ntop(_: *const libc::c_uchar, _: size_t, _: *mut libc::c_char,
                  _: size_t) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn tcsetattr(__fd: libc::c_int, __optional_actions: libc::c_int,
                 __termios_p: *const termios) -> libc::c_int;
    #[no_mangle]
    fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t)
     -> ssize_t;
    #[no_mangle]
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut options;
    #[no_mangle]
    fn setblocking(_: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn options_get_string(_: *mut options, _: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn tty_term_apply_overrides(_: *mut tty_term);
    #[no_mangle]
    fn tty_term_create(_: *mut tty, _: *mut libc::c_char, _: *mut libc::c_int,
                       _: libc::c_int, _: *mut *mut libc::c_char)
     -> *mut tty_term;
    #[no_mangle]
    fn tty_term_free(_: *mut tty_term);
    #[no_mangle]
    fn tty_term_has(_: *mut tty_term, _: tty_code_code) -> libc::c_int;
    #[no_mangle]
    fn tty_term_string(_: *mut tty_term, _: tty_code_code)
     -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_string1(_: *mut tty_term, _: tty_code_code, _: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_string2(_: *mut tty_term, _: tty_code_code, _: libc::c_int,
                        _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_string3(_: *mut tty_term, _: tty_code_code, _: libc::c_int,
                        _: libc::c_int, _: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_ptr1(_: *mut tty_term, _: tty_code_code,
                     _: *const libc::c_void) -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_ptr2(_: *mut tty_term, _: tty_code_code,
                     _: *const libc::c_void, _: *const libc::c_void)
     -> *const libc::c_char;
    #[no_mangle]
    fn tty_term_number(_: *mut tty_term, _: tty_code_code) -> libc::c_int;
    #[no_mangle]
    fn tty_term_flag(_: *mut tty_term, _: tty_code_code) -> libc::c_int;
    #[no_mangle]
    fn tty_apply_features(_: *mut tty_term, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tty_acs_needed(_: *mut tty) -> libc::c_int;
    #[no_mangle]
    fn tty_acs_get(_: *mut tty, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn tty_acs_reverse_get(_: *mut tty, _: *const libc::c_char, _: size_t)
     -> libc::c_int;
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
    fn colour_split_rgb(_: libc::c_int, _: *mut u_char, _: *mut u_char,
                        _: *mut u_char);
    #[no_mangle]
    fn colour_256toRGB(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn colour_256to16(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    fn grid_cells_equal(_: *const grid_cell, _: *const grid_cell)
     -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn utf8_set(_: *mut utf8_data, _: u_char);
    #[no_mangle]
    fn screen_select_cell(_: *mut screen, _: *mut grid_cell,
                          _: *const grid_cell);
    #[no_mangle]
    fn grid_view_get_cell(_: *mut grid, _: u_int, _: u_int,
                          _: *mut grid_cell);
    #[no_mangle]
    fn grid_get_line(_: *mut grid, _: u_int) -> *mut grid_line;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn style_add(_: *mut grid_cell, _: *mut options, _: *const libc::c_char,
                 _: *mut format_tree);
    #[no_mangle]
    fn evbuffer_read(buffer: *mut evbuffer, fd: libc::c_int,
                     howmuch: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_write(buffer: *mut evbuffer, fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_add(buf: *mut evbuffer, data: *const libc::c_void,
                    datlen: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_free(buf: *mut evbuffer);
    #[no_mangle]
    fn evbuffer_new() -> *mut evbuffer;
    #[no_mangle]
    fn event_set(_: *mut event, _: libc::c_int, _: libc::c_short,
                 _:
                     Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_short,
                                                 _: *mut libc::c_void) -> ()>,
                 _: *mut libc::c_void);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen {
    pub title: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub titles: *mut screen_titles,
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
    pub sel: *mut screen_sel,
    pub write_list: *mut screen_write_collect_line,
}
pub type bitstr_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid_cell {
    pub data: utf8_data,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utf8_data {
    pub data: [u_char; 21],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid {
    pub flags: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub hscrolled: u_int,
    pub hsize: u_int,
    pub hlimit: u_int,
    pub linedata: *mut grid_line,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct grid_line {
    pub cellused: u_int,
    pub cellsize: u_int,
    pub celldata: *mut grid_cell_entry,
    pub extdsize: u_int,
    pub extddata: *mut grid_extd_entry,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct grid_extd_entry {
    pub data: utf8_char,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}
pub type utf8_char = u_int;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub offset: u_int,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ev_io: C2RustUnnamed_4,
    pub ev_signal: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ev_signal_next: C2RustUnnamed_3,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub ev_io_next: C2RustUnnamed_5,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ev_next_with_common_timeout: C2RustUnnamed_7,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_9,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_8,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub evcb_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_short,
                                                   _: *mut libc::c_void)
                                  -> ()>,
    pub evcb_selfcb: Option<unsafe extern "C" fn(_: *mut event_callback,
                                                 _: *mut libc::c_void) -> ()>,
    pub evcb_evfinalize: Option<unsafe extern "C" fn(_: *mut event,
                                                     _: *mut libc::c_void)
                                    -> ()>,
    pub evcb_cbfinalize: Option<unsafe extern "C" fn(_: *mut event_callback,
                                                     _: *mut libc::c_void)
                                    -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type bufferevent_event_cb
    =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type bufferevent_data_cb
    =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client {
    pub name: *const libc::c_char,
    pub peer: *mut tmuxpeer,
    pub queue: *mut cmdq_list,
    pub windows: client_windows,
    pub control_state: *mut control_state,
    pub pause_age: u_int,
    pub pid: pid_t,
    pub fd: libc::c_int,
    pub out_fd: libc::c_int,
    pub event: event,
    pub retval: libc::c_int,
    pub creation_time: timeval,
    pub activity_time: timeval,
    pub environ: *mut environ,
    pub jobs: *mut format_job_tree,
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
    pub entry: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_files {
    pub rbh_root: *mut client_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub rbe_left: *mut client_file,
    pub rbe_right: *mut client_file,
    pub rbe_parent: *mut client_file,
    pub rbe_color: libc::c_int,
}
pub type client_file_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: *const libc::c_char,
                                _: libc::c_int, _: libc::c_int,
                                _: *mut evbuffer, _: *mut libc::c_void)
               -> ()>;
pub type overlay_free_cb = Option<unsafe extern "C" fn(_: *mut client) -> ()>;
pub type overlay_key_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut key_event)
               -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_event {
    pub key: key_code,
    pub m: mouse_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type overlay_draw_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut screen_redraw_ctx)
               -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type overlay_mode_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut u_int, _: *mut u_int)
               -> *mut screen>;
pub type overlay_check_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: u_int, _: u_int)
               -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub options: *mut options,
    pub flags: libc::c_int,
    pub attached: u_int,
    pub tio: *mut termios,
    pub environ: *mut environ,
    pub references: libc::c_int,
    pub gentry: C2RustUnnamed_13,
    pub entry: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winlink {
    pub idx: libc::c_int,
    pub session: *mut session,
    pub window: *mut window,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_16,
    pub wentry: C2RustUnnamed_15,
    pub sentry: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub options: *mut options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_18,
    pub entry: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_pane {
    pub id: u_int,
    pub active_point: u_int,
    pub window: *mut window,
    pub options: *mut options,
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
    pub ictx: *mut input_ctx,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub tqh_first: *mut window_mode_entry,
    pub tqh_last: *mut *mut window_mode_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_mode_entry {
    pub wp: *mut window_pane,
    pub swp: *mut window_pane,
    pub mode: *const window_mode,
    pub data: *mut libc::c_void,
    pub screen: *mut screen,
    pub prefix: u_int,
    pub entry: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub tqe_next: *mut window_mode_entry,
    pub tqe_prev: *mut *mut window_mode_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_mode {
    pub name: *const libc::c_char,
    pub default_format: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(_: *mut window_mode_entry,
                                          _: *mut cmd_find_state,
                                          _: *mut args) -> *mut screen>,
    pub free: Option<unsafe extern "C" fn(_: *mut window_mode_entry) -> ()>,
    pub resize: Option<unsafe extern "C" fn(_: *mut window_mode_entry,
                                            _: u_int, _: u_int) -> ()>,
    pub key: Option<unsafe extern "C" fn(_: *mut window_mode_entry,
                                         _: *mut client, _: *mut session,
                                         _: *mut winlink, _: key_code,
                                         _: *mut mouse_event) -> ()>,
    pub key_table: Option<unsafe extern "C" fn(_: *mut window_mode_entry)
                              -> *const libc::c_char>,
    pub command: Option<unsafe extern "C" fn(_: *mut window_mode_entry,
                                             _: *mut client, _: *mut session,
                                             _: *mut winlink, _: *mut args,
                                             _: *mut mouse_event) -> ()>,
    pub formats: Option<unsafe extern "C" fn(_: *mut window_mode_entry,
                                             _: *mut format_tree) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_find_state {
    pub flags: libc::c_int,
    pub current: *mut cmd_find_state,
    pub s: *mut session,
    pub wl: *mut winlink,
    pub w: *mut window,
    pub wp: *mut window_pane,
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_pane_offset {
    pub used: size_t,
}
pub type layout_type = libc::c_uint;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winlink_stack {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
pub type C2RustUnnamed_25 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_25 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_25 = 0;
pub type prompt_free_cb
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type prompt_input_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub default_key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub note: *const libc::c_char,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_list {
    pub references: libc::c_int,
    pub group: u_int,
    pub list: *mut cmds,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_line {
    pub timer: event,
    pub screen: screen,
    pub active: *mut screen,
    pub references: libc::c_int,
    pub style: grid_cell,
    pub entries: [status_line_entry; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_line_entry {
    pub expanded: *mut libc::c_char,
    pub ranges: style_ranges,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct style_ranges {
    pub tqh_first: *mut style_range,
    pub tqh_last: *mut *mut style_range,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct style_range {
    pub type_0: style_range_type,
    pub argument: u_int,
    pub start: u_int,
    pub end: u_int,
    pub entry: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub tqe_next: *mut style_range,
    pub tqe_prev: *mut *mut style_range,
}
pub type style_range_type = libc::c_uint;
pub const STYLE_RANGE_WINDOW: style_range_type = 3;
pub const STYLE_RANGE_RIGHT: style_range_type = 2;
pub const STYLE_RANGE_LEFT: style_range_type = 1;
pub const STYLE_RANGE_NONE: style_range_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub mouse_drag_update: Option<unsafe extern "C" fn(_: *mut client,
                                                       _: *mut mouse_event)
                                      -> ()>,
    pub mouse_drag_release: Option<unsafe extern "C" fn(_: *mut client,
                                                        _: *mut mouse_event)
                                       -> ()>,
    pub key_timer: event,
    pub key_tree: *mut tty_key,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub tty: *mut tty,
    pub features: libc::c_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_windows {
    pub rbh_root: *mut client_window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_window {
    pub window: u_int,
    pub pane: *mut window_pane,
    pub entry: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub rbe_left: *mut client_window,
    pub rbe_right: *mut client_window,
    pub rbe_parent: *mut client_window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_ctx {
    pub s: *mut screen,
    pub redraw_cb: tty_ctx_redraw_cb,
    pub set_client_cb: tty_ctx_set_client_cb,
    pub arg: *mut libc::c_void,
    pub cell: *const grid_cell,
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
    pub defaults: grid_cell,
    pub palette: *mut libc::c_int,
    pub bigger: libc::c_int,
    pub wox: u_int,
    pub woy: u_int,
    pub wsx: u_int,
    pub wsy: u_int,
}
pub type tty_ctx_set_client_cb
    =
    Option<unsafe extern "C" fn(_: *mut tty_ctx, _: *mut client)
               -> libc::c_int>;
pub type tty_ctx_redraw_cb
    =
    Option<unsafe extern "C" fn(_: *const tty_ctx) -> ()>;
pub type tty_code_code = libc::c_uint;
pub const TTYC_XT: tty_code_code = 224;
pub const TTYC_VPA: tty_code_code = 223;
pub const TTYC_U8: tty_code_code = 222;
pub const TTYC_TSL: tty_code_code = 221;
pub const TTYC_TC: tty_code_code = 220;
pub const TTYC_SYNC: tty_code_code = 219;
pub const TTYC_SS: tty_code_code = 218;
pub const TTYC_SMXX: tty_code_code = 217;
pub const TTYC_SMULX: tty_code_code = 216;
pub const TTYC_SMUL: tty_code_code = 215;
pub const TTYC_SMSO: tty_code_code = 214;
pub const TTYC_SMOL: tty_code_code = 213;
pub const TTYC_SMKX: tty_code_code = 212;
pub const TTYC_SMCUP: tty_code_code = 211;
pub const TTYC_SMACS: tty_code_code = 210;
pub const TTYC_SITM: tty_code_code = 209;
pub const TTYC_SGR0: tty_code_code = 208;
pub const TTYC_SETULC: tty_code_code = 207;
pub const TTYC_SETRGBF: tty_code_code = 206;
pub const TTYC_SETRGBB: tty_code_code = 205;
pub const TTYC_SETAL: tty_code_code = 204;
pub const TTYC_SETAF: tty_code_code = 203;
pub const TTYC_SETAB: tty_code_code = 202;
pub const TTYC_SE: tty_code_code = 201;
pub const TTYC_RMKX: tty_code_code = 200;
pub const TTYC_RMCUP: tty_code_code = 199;
pub const TTYC_RMACS: tty_code_code = 198;
pub const TTYC_RIN: tty_code_code = 197;
pub const TTYC_RI: tty_code_code = 196;
pub const TTYC_RGB: tty_code_code = 195;
pub const TTYC_REV: tty_code_code = 194;
pub const TTYC_OP: tty_code_code = 193;
pub const TTYC_OL: tty_code_code = 192;
pub const TTYC_MS: tty_code_code = 191;
pub const TTYC_KUP7: tty_code_code = 190;
pub const TTYC_KUP6: tty_code_code = 189;
pub const TTYC_KUP5: tty_code_code = 188;
pub const TTYC_KUP4: tty_code_code = 187;
pub const TTYC_KUP3: tty_code_code = 186;
pub const TTYC_KUP2: tty_code_code = 185;
pub const TTYC_KRIT7: tty_code_code = 184;
pub const TTYC_KRIT6: tty_code_code = 183;
pub const TTYC_KRIT5: tty_code_code = 182;
pub const TTYC_KRIT4: tty_code_code = 181;
pub const TTYC_KRIT3: tty_code_code = 180;
pub const TTYC_KRIT2: tty_code_code = 179;
pub const TTYC_KRI: tty_code_code = 178;
pub const TTYC_KPRV7: tty_code_code = 177;
pub const TTYC_KPRV6: tty_code_code = 176;
pub const TTYC_KPRV5: tty_code_code = 175;
pub const TTYC_KPRV4: tty_code_code = 174;
pub const TTYC_KPRV3: tty_code_code = 173;
pub const TTYC_KPRV2: tty_code_code = 172;
pub const TTYC_KPP: tty_code_code = 171;
pub const TTYC_KNXT7: tty_code_code = 170;
pub const TTYC_KNXT6: tty_code_code = 169;
pub const TTYC_KNXT5: tty_code_code = 168;
pub const TTYC_KNXT4: tty_code_code = 167;
pub const TTYC_KNXT3: tty_code_code = 166;
pub const TTYC_KNXT2: tty_code_code = 165;
pub const TTYC_KNP: tty_code_code = 164;
pub const TTYC_KMOUS: tty_code_code = 163;
pub const TTYC_KLFT7: tty_code_code = 162;
pub const TTYC_KLFT6: tty_code_code = 161;
pub const TTYC_KLFT5: tty_code_code = 160;
pub const TTYC_KLFT4: tty_code_code = 159;
pub const TTYC_KLFT3: tty_code_code = 158;
pub const TTYC_KLFT2: tty_code_code = 157;
pub const TTYC_KIND: tty_code_code = 156;
pub const TTYC_KICH1: tty_code_code = 155;
pub const TTYC_KIC7: tty_code_code = 154;
pub const TTYC_KIC6: tty_code_code = 153;
pub const TTYC_KIC5: tty_code_code = 152;
pub const TTYC_KIC4: tty_code_code = 151;
pub const TTYC_KIC3: tty_code_code = 150;
pub const TTYC_KIC2: tty_code_code = 149;
pub const TTYC_KHOME: tty_code_code = 148;
pub const TTYC_KHOM7: tty_code_code = 147;
pub const TTYC_KHOM6: tty_code_code = 146;
pub const TTYC_KHOM5: tty_code_code = 145;
pub const TTYC_KHOM4: tty_code_code = 144;
pub const TTYC_KHOM3: tty_code_code = 143;
pub const TTYC_KHOM2: tty_code_code = 142;
pub const TTYC_KF9: tty_code_code = 141;
pub const TTYC_KF8: tty_code_code = 140;
pub const TTYC_KF7: tty_code_code = 139;
pub const TTYC_KF63: tty_code_code = 138;
pub const TTYC_KF62: tty_code_code = 137;
pub const TTYC_KF61: tty_code_code = 136;
pub const TTYC_KF60: tty_code_code = 135;
pub const TTYC_KF6: tty_code_code = 134;
pub const TTYC_KF59: tty_code_code = 133;
pub const TTYC_KF58: tty_code_code = 132;
pub const TTYC_KF57: tty_code_code = 131;
pub const TTYC_KF56: tty_code_code = 130;
pub const TTYC_KF55: tty_code_code = 129;
pub const TTYC_KF54: tty_code_code = 128;
pub const TTYC_KF53: tty_code_code = 127;
pub const TTYC_KF52: tty_code_code = 126;
pub const TTYC_KF51: tty_code_code = 125;
pub const TTYC_KF50: tty_code_code = 124;
pub const TTYC_KF5: tty_code_code = 123;
pub const TTYC_KF49: tty_code_code = 122;
pub const TTYC_KF48: tty_code_code = 121;
pub const TTYC_KF47: tty_code_code = 120;
pub const TTYC_KF46: tty_code_code = 119;
pub const TTYC_KF45: tty_code_code = 118;
pub const TTYC_KF44: tty_code_code = 117;
pub const TTYC_KF43: tty_code_code = 116;
pub const TTYC_KF42: tty_code_code = 115;
pub const TTYC_KF41: tty_code_code = 114;
pub const TTYC_KF40: tty_code_code = 113;
pub const TTYC_KF4: tty_code_code = 112;
pub const TTYC_KF39: tty_code_code = 111;
pub const TTYC_KF38: tty_code_code = 110;
pub const TTYC_KF37: tty_code_code = 109;
pub const TTYC_KF36: tty_code_code = 108;
pub const TTYC_KF35: tty_code_code = 107;
pub const TTYC_KF34: tty_code_code = 106;
pub const TTYC_KF33: tty_code_code = 105;
pub const TTYC_KF32: tty_code_code = 104;
pub const TTYC_KF31: tty_code_code = 103;
pub const TTYC_KF30: tty_code_code = 102;
pub const TTYC_KF3: tty_code_code = 101;
pub const TTYC_KF29: tty_code_code = 100;
pub const TTYC_KF28: tty_code_code = 99;
pub const TTYC_KF27: tty_code_code = 98;
pub const TTYC_KF26: tty_code_code = 97;
pub const TTYC_KF25: tty_code_code = 96;
pub const TTYC_KF24: tty_code_code = 95;
pub const TTYC_KF23: tty_code_code = 94;
pub const TTYC_KF22: tty_code_code = 93;
pub const TTYC_KF21: tty_code_code = 92;
pub const TTYC_KF20: tty_code_code = 91;
pub const TTYC_KF2: tty_code_code = 90;
pub const TTYC_KF19: tty_code_code = 89;
pub const TTYC_KF18: tty_code_code = 88;
pub const TTYC_KF17: tty_code_code = 87;
pub const TTYC_KF16: tty_code_code = 86;
pub const TTYC_KF15: tty_code_code = 85;
pub const TTYC_KF14: tty_code_code = 84;
pub const TTYC_KF13: tty_code_code = 83;
pub const TTYC_KF12: tty_code_code = 82;
pub const TTYC_KF11: tty_code_code = 81;
pub const TTYC_KF10: tty_code_code = 80;
pub const TTYC_KF1: tty_code_code = 79;
pub const TTYC_KEND7: tty_code_code = 78;
pub const TTYC_KEND6: tty_code_code = 77;
pub const TTYC_KEND5: tty_code_code = 76;
pub const TTYC_KEND4: tty_code_code = 75;
pub const TTYC_KEND3: tty_code_code = 74;
pub const TTYC_KEND2: tty_code_code = 73;
pub const TTYC_KEND: tty_code_code = 72;
pub const TTYC_KDN7: tty_code_code = 71;
pub const TTYC_KDN6: tty_code_code = 70;
pub const TTYC_KDN5: tty_code_code = 69;
pub const TTYC_KDN4: tty_code_code = 68;
pub const TTYC_KDN3: tty_code_code = 67;
pub const TTYC_KDN2: tty_code_code = 66;
pub const TTYC_KDCH1: tty_code_code = 65;
pub const TTYC_KDC7: tty_code_code = 64;
pub const TTYC_KDC6: tty_code_code = 63;
pub const TTYC_KDC5: tty_code_code = 62;
pub const TTYC_KDC4: tty_code_code = 61;
pub const TTYC_KDC3: tty_code_code = 60;
pub const TTYC_KDC2: tty_code_code = 59;
pub const TTYC_KCUU1: tty_code_code = 58;
pub const TTYC_KCUF1: tty_code_code = 57;
pub const TTYC_KCUD1: tty_code_code = 56;
pub const TTYC_KCUB1: tty_code_code = 55;
pub const TTYC_KCBT: tty_code_code = 54;
pub const TTYC_INVIS: tty_code_code = 53;
pub const TTYC_INDN: tty_code_code = 52;
pub const TTYC_IL1: tty_code_code = 51;
pub const TTYC_IL: tty_code_code = 50;
pub const TTYC_ICH1: tty_code_code = 49;
pub const TTYC_ICH: tty_code_code = 48;
pub const TTYC_HPA: tty_code_code = 47;
pub const TTYC_HOME: tty_code_code = 46;
pub const TTYC_FSL: tty_code_code = 45;
pub const TTYC_ENMG: tty_code_code = 44;
pub const TTYC_ENFCS: tty_code_code = 43;
pub const TTYC_ENEKS: tty_code_code = 42;
pub const TTYC_ENBP: tty_code_code = 41;
pub const TTYC_ENACS: tty_code_code = 40;
pub const TTYC_EL1: tty_code_code = 39;
pub const TTYC_EL: tty_code_code = 38;
pub const TTYC_ED: tty_code_code = 37;
pub const TTYC_ECH: tty_code_code = 36;
pub const TTYC_E3: tty_code_code = 35;
pub const TTYC_DSMG: tty_code_code = 34;
pub const TTYC_DSFCS: tty_code_code = 33;
pub const TTYC_DSEKS: tty_code_code = 32;
pub const TTYC_DSBP: tty_code_code = 31;
pub const TTYC_DL1: tty_code_code = 30;
pub const TTYC_DL: tty_code_code = 29;
pub const TTYC_DIM: tty_code_code = 28;
pub const TTYC_DCH1: tty_code_code = 27;
pub const TTYC_DCH: tty_code_code = 26;
pub const TTYC_CVVIS: tty_code_code = 25;
pub const TTYC_CUU1: tty_code_code = 24;
pub const TTYC_CUU: tty_code_code = 23;
pub const TTYC_CUP: tty_code_code = 22;
pub const TTYC_CUF1: tty_code_code = 21;
pub const TTYC_CUF: tty_code_code = 20;
pub const TTYC_CUD1: tty_code_code = 19;
pub const TTYC_CUD: tty_code_code = 18;
pub const TTYC_CUB1: tty_code_code = 17;
pub const TTYC_CUB: tty_code_code = 16;
pub const TTYC_CSR: tty_code_code = 15;
pub const TTYC_CS: tty_code_code = 14;
pub const TTYC_CR: tty_code_code = 13;
pub const TTYC_COLORS: tty_code_code = 12;
pub const TTYC_CNORM: tty_code_code = 11;
pub const TTYC_CMG: tty_code_code = 10;
pub const TTYC_CLMG: tty_code_code = 9;
pub const TTYC_CLEAR: tty_code_code = 8;
pub const TTYC_CIVIS: tty_code_code = 7;
pub const TTYC_BOLD: tty_code_code = 6;
pub const TTYC_BLINK: tty_code_code = 5;
pub const TTYC_BEL: tty_code_code = 4;
pub const TTYC_BCE: tty_code_code = 3;
pub const TTYC_AX: tty_code_code = 2;
pub const TTYC_AM: tty_code_code = 1;
pub const TTYC_ACSC: tty_code_code = 0;
#[derive(Copy, Clone)]
#[repr(C)]
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
static mut tty_log_fd: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub unsafe extern "C" fn tty_create_log() {
    let mut name: [libc::c_char; 64] = [0; 64];
    xsnprintf(name.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
              b"tmux-out-%ld.log\x00" as *const u8 as *const libc::c_char,
              getpid() as libc::c_long);
    tty_log_fd =
        open(name.as_mut_ptr(),
             0o1 as libc::c_int | 0o100 as libc::c_int |
                 0o1000 as libc::c_int, 0o644 as libc::c_int);
    if tty_log_fd != -(1 as libc::c_int) &&
           fcntl(tty_log_fd, 2 as libc::c_int, 1 as libc::c_int) ==
               -(1 as libc::c_int) {
        fatal(b"fcntl failed\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_init(mut tty: *mut tty, mut c: *mut client)
 -> libc::c_int {
    if isatty((*c).fd) == 0 { return -(1 as libc::c_int) }
    memset(tty as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<tty>() as libc::c_ulong);
    (*tty).client = c;
    (*tty).cstyle = 0 as libc::c_int as u_int;
    (*tty).ccolour = xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    if tcgetattr((*c).fd, &mut (*tty).tio) != 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tty_resize(mut tty: *mut tty) {
    let mut c: *mut client = (*tty).client;
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut xpixel: u_int = 0;
    let mut ypixel: u_int = 0;
    if ioctl((*c).fd, 0x5413 as libc::c_int as libc::c_ulong,
             &mut ws as *mut winsize) != -(1 as libc::c_int) {
        sx = ws.ws_col as u_int;
        if sx == 0 as libc::c_int as libc::c_uint {
            sx = 80 as libc::c_int as u_int;
            xpixel = 0 as libc::c_int as u_int
        } else { xpixel = (ws.ws_xpixel as libc::c_uint).wrapping_div(sx) }
        sy = ws.ws_row as u_int;
        if sy == 0 as libc::c_int as libc::c_uint {
            sy = 24 as libc::c_int as u_int;
            ypixel = 0 as libc::c_int as u_int
        } else { ypixel = (ws.ws_ypixel as libc::c_uint).wrapping_div(sy) }
    } else {
        sx = 80 as libc::c_int as u_int;
        sy = 24 as libc::c_int as u_int;
        xpixel = 0 as libc::c_int as u_int;
        ypixel = 0 as libc::c_int as u_int
    }
    log_debug(b"%s: %s now %ux%u (%ux%u)\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"tty_resize\x00")).as_ptr(),
              (*c).name, sx, sy, xpixel, ypixel);
    tty_set_size(tty, sx, sy, xpixel, ypixel);
    tty_invalidate(tty);
}
#[no_mangle]
pub unsafe extern "C" fn tty_set_size(mut tty: *mut tty, mut sx: u_int,
                                      mut sy: u_int, mut xpixel: u_int,
                                      mut ypixel: u_int) {
    (*tty).sx = sx;
    (*tty).sy = sy;
    (*tty).xpixel = xpixel;
    (*tty).ypixel = ypixel;
}
unsafe extern "C" fn tty_read_callback(mut fd: libc::c_int,
                                       mut events: libc::c_short,
                                       mut data: *mut libc::c_void) {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    let mut name: *const libc::c_char = (*c).name;
    let mut size: size_t = evbuffer_get_length((*tty).in_0);
    let mut nread: libc::c_int = 0;
    nread = evbuffer_read((*tty).in_0, (*c).fd, -(1 as libc::c_int));
    if nread == 0 as libc::c_int || nread == -(1 as libc::c_int) {
        if nread == 0 as libc::c_int {
            log_debug(b"%s: read closed\x00" as *const u8 as
                          *const libc::c_char, name);
        } else {
            log_debug(b"%s: read error: %s\x00" as *const u8 as
                          *const libc::c_char, name,
                      strerror(*__errno_location()));
        }
        event_del(&mut (*tty).event_in);
        server_client_lost((*tty).client);
        return
    }
    log_debug(b"%s: read %d bytes (already %zu)\x00" as *const u8 as
                  *const libc::c_char, name, nread, size);
    while tty_keys_next(tty) != 0 { };
}
unsafe extern "C" fn tty_timer_callback(mut fd: libc::c_int,
                                        mut events: libc::c_short,
                                        mut data: *mut libc::c_void) {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    let mut tv: timeval =
        {
            let mut init =
                timeval{tv_sec: 0,
                        tv_usec: 100000 as libc::c_int as __suseconds_t,};
            init
        };
    log_debug(b"%s: %zu discarded\x00" as *const u8 as *const libc::c_char,
              (*c).name, (*tty).discarded);
    (*c).flags |=
        (0x8 as libc::c_int | 0x10 as libc::c_int | 0x1000000 as libc::c_int |
             0x400 as libc::c_int | 0x2000000 as libc::c_int |
             0x20000000 as libc::c_int) as libc::c_ulong;
    (*c).discarded =
        ((*c).discarded as libc::c_ulong).wrapping_add((*tty).discarded) as
            size_t as size_t;
    if (*tty).discarded <
           (1 as libc::c_int as
                libc::c_uint).wrapping_add((*tty).sx.wrapping_mul((*tty).sy).wrapping_div(8
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
               as libc::c_ulong {
        (*tty).flags &= !(0x80 as libc::c_int);
        tty_invalidate(tty);
        return
    }
    (*tty).discarded = 0 as libc::c_int as size_t;
    event_add(&mut (*tty).timer, &mut tv);
}
unsafe extern "C" fn tty_block_maybe(mut tty: *mut tty) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut size: size_t = evbuffer_get_length((*tty).out);
    let mut tv: timeval =
        {
            let mut init =
                timeval{tv_sec: 0,
                        tv_usec: 100000 as libc::c_int as __suseconds_t,};
            init
        };
    if size <
           (1 as libc::c_int as
                libc::c_uint).wrapping_add((*tty).sx.wrapping_mul((*tty).sy).wrapping_mul(8
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
               as libc::c_ulong {
        return 0 as libc::c_int
    }
    if (*tty).flags & 0x80 as libc::c_int != 0 { return 1 as libc::c_int }
    (*tty).flags |= 0x80 as libc::c_int;
    log_debug(b"%s: can\'t keep up, %zu discarded\x00" as *const u8 as
                  *const libc::c_char, (*c).name, size);
    evbuffer_drain((*tty).out, size);
    (*c).discarded =
        ((*c).discarded as libc::c_ulong).wrapping_add(size) as size_t as
            size_t;
    (*tty).discarded = 0 as libc::c_int as size_t;
    event_add(&mut (*tty).timer, &mut tv);
    return 1 as libc::c_int;
}
unsafe extern "C" fn tty_write_callback(mut fd: libc::c_int,
                                        mut events: libc::c_short,
                                        mut data: *mut libc::c_void) {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    let mut size: size_t = evbuffer_get_length((*tty).out);
    let mut nwrite: libc::c_int = 0;
    nwrite = evbuffer_write((*tty).out, (*c).fd);
    if nwrite == -(1 as libc::c_int) { return }
    log_debug(b"%s: wrote %d bytes (of %zu)\x00" as *const u8 as
                  *const libc::c_char, (*c).name, nwrite, size);
    if (*c).redraw > 0 as libc::c_int as libc::c_ulong {
        if nwrite as size_t >= (*c).redraw {
            (*c).redraw = 0 as libc::c_int as size_t
        } else {
            (*c).redraw =
                ((*c).redraw as
                     libc::c_ulong).wrapping_sub(nwrite as libc::c_ulong) as
                    size_t as size_t
        }
        log_debug(b"%s: waiting for redraw, %zu bytes left\x00" as *const u8
                      as *const libc::c_char, (*c).name, (*c).redraw);
    } else if tty_block_maybe(tty) != 0 { return }
    if evbuffer_get_length((*tty).out) != 0 as libc::c_int as libc::c_ulong {
        event_add(&mut (*tty).event_out, 0 as *const timeval);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_open(mut tty: *mut tty,
                                  mut cause: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    (*tty).term =
        tty_term_create(tty, (*c).term_name, &mut (*c).term_features, (*c).fd,
                        cause);
    if (*tty).term.is_null() { tty_close(tty); return -(1 as libc::c_int) }
    (*tty).flags |= 0x20 as libc::c_int;
    (*tty).flags &=
        !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x80 as libc::c_int |
              0x4 as libc::c_int);
    event_set(&mut (*tty).event_in, (*c).fd,
              (0x10 as libc::c_int | 0x2 as libc::c_int) as libc::c_short,
              Some(tty_read_callback as
                       unsafe extern "C" fn(_: libc::c_int, _: libc::c_short,
                                            _: *mut libc::c_void) -> ()),
              tty as *mut libc::c_void);
    (*tty).in_0 = evbuffer_new();
    if (*tty).in_0.is_null() {
        fatal(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    event_set(&mut (*tty).event_out, (*c).fd,
              0x4 as libc::c_int as libc::c_short,
              Some(tty_write_callback as
                       unsafe extern "C" fn(_: libc::c_int, _: libc::c_short,
                                            _: *mut libc::c_void) -> ()),
              tty as *mut libc::c_void);
    (*tty).out = evbuffer_new();
    if (*tty).out.is_null() {
        fatal(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    event_set(&mut (*tty).timer, -(1 as libc::c_int),
              0 as libc::c_int as libc::c_short,
              Some(tty_timer_callback as
                       unsafe extern "C" fn(_: libc::c_int, _: libc::c_short,
                                            _: *mut libc::c_void) -> ()),
              tty as *mut libc::c_void);
    tty_start_tty(tty);
    tty_keys_build(tty);
    return 0 as libc::c_int;
}
unsafe extern "C" fn tty_start_timer_callback(mut fd: libc::c_int,
                                              mut events: libc::c_short,
                                              mut data: *mut libc::c_void) {
    let mut tty: *mut tty = data as *mut tty;
    let mut c: *mut client = (*tty).client;
    log_debug(b"%s: start timer fired\x00" as *const u8 as
                  *const libc::c_char, (*c).name);
    if (*tty).flags & (0x100 as libc::c_int | 0x200 as libc::c_int) ==
           0 as libc::c_int {
        tty_update_features(tty);
    }
    (*tty).flags |= 0x100 as libc::c_int | 0x200 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tty_start_tty(mut tty: *mut tty) {
    let mut c: *mut client = (*tty).client;
    let mut tio: termios =
        termios{c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,};
    let mut tv: timeval =
        {
            let mut init =
                timeval{tv_sec: 1 as libc::c_int as __time_t, tv_usec: 0,};
            init
        };
    setblocking((*c).fd, 0 as libc::c_int);
    event_add(&mut (*tty).event_in, 0 as *const timeval);
    memcpy(&mut tio as *mut termios as *mut libc::c_void,
           &mut (*tty).tio as *mut termios as *const libc::c_void,
           ::std::mem::size_of::<termios>() as libc::c_ulong);
    tio.c_iflag &=
        !(0o2000 as libc::c_int | 0o10000 as libc::c_int |
              0o400 as libc::c_int | 0o100 as libc::c_int |
              0o200 as libc::c_int | 0o20000 as libc::c_int |
              0o40 as libc::c_int) as libc::c_uint;
    tio.c_iflag |= 0o1 as libc::c_int as libc::c_uint;
    tio.c_oflag &=
        !(0o1 as libc::c_int | 0o4 as libc::c_int | 0o10 as libc::c_int |
              0o40 as libc::c_int) as libc::c_uint;
    tio.c_lflag &=
        !(0o100000 as libc::c_int | 0o2 as libc::c_int | 0o10 as libc::c_int |
              0o20 as libc::c_int | 0o100 as libc::c_int |
              0o1000 as libc::c_int | 0o2000 as libc::c_int |
              0o4000 as libc::c_int | 0o1 as libc::c_int) as libc::c_uint;
    tio.c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
    tio.c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
    if tcsetattr((*c).fd, 0 as libc::c_int, &mut tio) == 0 as libc::c_int {
        tcflush((*c).fd, 2 as libc::c_int);
    }
    tty_putcode(tty, TTYC_SMCUP);
    tty_putcode(tty, TTYC_SMKX);
    tty_putcode(tty, TTYC_CLEAR);
    if tty_acs_needed(tty) != 0 {
        log_debug(b"%s: using capabilities for ACS\x00" as *const u8 as
                      *const libc::c_char, (*c).name);
        tty_putcode(tty, TTYC_ENACS);
    } else {
        log_debug(b"%s: using UTF-8 for ACS\x00" as *const u8 as
                      *const libc::c_char, (*c).name);
    }
    tty_putcode(tty, TTYC_CNORM);
    if tty_term_has((*tty).term, TTYC_KMOUS) != 0 {
        tty_puts(tty,
                 b"\x1b[?1000l\x1b[?1002l\x1b[?1003l\x00" as *const u8 as
                     *const libc::c_char);
        tty_puts(tty,
                 b"\x1b[?1006l\x1b[?1005l\x00" as *const u8 as
                     *const libc::c_char);
    }
    event_set(&mut (*tty).start_timer, -(1 as libc::c_int),
              0 as libc::c_int as libc::c_short,
              Some(tty_start_timer_callback as
                       unsafe extern "C" fn(_: libc::c_int, _: libc::c_short,
                                            _: *mut libc::c_void) -> ()),
              tty as *mut libc::c_void);
    event_add(&mut (*tty).start_timer, &mut tv);
    (*tty).flags |= 0x10 as libc::c_int;
    tty_invalidate(tty);
    if *(*tty).ccolour as libc::c_int != '\u{0}' as i32 {
        tty_force_cursor_colour(tty,
                                b"\x00" as *const u8 as *const libc::c_char);
    }
    (*tty).mouse_drag_flag = 0 as libc::c_int;
    (*tty).mouse_drag_update = None;
    (*tty).mouse_drag_release = None;
}
#[no_mangle]
pub unsafe extern "C" fn tty_send_requests(mut tty: *mut tty) {
    if !(*tty).flags & 0x10 as libc::c_int != 0 { return }
    if (*(*tty).term).flags & 0x20 as libc::c_int != 0 {
        if !(*tty).flags & 0x100 as libc::c_int != 0 {
            tty_puts(tty, b"\x1b[>c\x00" as *const u8 as *const libc::c_char);
        }
        if !(*tty).flags & 0x200 as libc::c_int != 0 {
            tty_puts(tty, b"\x1b[>q\x00" as *const u8 as *const libc::c_char);
        }
    } else { (*tty).flags |= 0x100 as libc::c_int | 0x200 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn tty_stop_tty(mut tty: *mut tty) {
    let mut c: *mut client = (*tty).client;
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    if (*tty).flags & 0x10 as libc::c_int == 0 { return }
    (*tty).flags &= !(0x10 as libc::c_int);
    event_del(&mut (*tty).start_timer);
    event_del(&mut (*tty).timer);
    (*tty).flags &= !(0x80 as libc::c_int);
    event_del(&mut (*tty).event_in);
    event_del(&mut (*tty).event_out);
    /*
	 * Be flexible about error handling and try not kill the server just
	 * because the fd is invalid. Things like ssh -t can easily leave us
	 * with a dead tty.
	 */
    if ioctl((*c).fd, 0x5413 as libc::c_int as libc::c_ulong,
             &mut ws as *mut winsize) == -(1 as libc::c_int) {
        return
    }
    if tcsetattr((*c).fd, 0 as libc::c_int, &mut (*tty).tio) ==
           -(1 as libc::c_int) {
        return
    }
    tty_raw(tty,
            tty_term_string2((*tty).term, TTYC_CSR, 0 as libc::c_int,
                             ws.ws_row as libc::c_int - 1 as libc::c_int));
    if tty_acs_needed(tty) != 0 {
        tty_raw(tty, tty_term_string((*tty).term, TTYC_RMACS));
    }
    tty_raw(tty, tty_term_string((*tty).term, TTYC_SGR0));
    tty_raw(tty, tty_term_string((*tty).term, TTYC_RMKX));
    tty_raw(tty, tty_term_string((*tty).term, TTYC_CLEAR));
    if tty_term_has((*tty).term, TTYC_SS) != 0 &&
           (*tty).cstyle != 0 as libc::c_int as libc::c_uint {
        if tty_term_has((*tty).term, TTYC_SE) != 0 {
            tty_raw(tty, tty_term_string((*tty).term, TTYC_SE));
        } else {
            tty_raw(tty,
                    tty_term_string1((*tty).term, TTYC_SS, 0 as libc::c_int));
        }
    }
    if (*tty).mode & 0x400 as libc::c_int != 0 {
        tty_raw(tty, tty_term_string((*tty).term, TTYC_DSBP));
    }
    if *(*tty).ccolour as libc::c_int != '\u{0}' as i32 {
        tty_raw(tty, tty_term_string((*tty).term, TTYC_CR));
    }
    tty_raw(tty, tty_term_string((*tty).term, TTYC_CNORM));
    if tty_term_has((*tty).term, TTYC_KMOUS) != 0 {
        tty_raw(tty,
                b"\x1b[?1000l\x1b[?1002l\x1b[?1003l\x00" as *const u8 as
                    *const libc::c_char);
        tty_raw(tty,
                b"\x1b[?1006l\x1b[?1005l\x00" as *const u8 as
                    *const libc::c_char);
    }
    if (*(*tty).term).flags & 0x20 as libc::c_int != 0 {
        tty_raw(tty, b"\x1b[?7727l\x00" as *const u8 as *const libc::c_char);
    }
    tty_raw(tty, tty_term_string((*tty).term, TTYC_DSFCS));
    tty_raw(tty, tty_term_string((*tty).term, TTYC_DSEKS));
    if (*(*tty).term).flags & 0x4 as libc::c_int != 0 {
        tty_raw(tty, tty_term_string((*tty).term, TTYC_DSMG));
    }
    tty_raw(tty, tty_term_string((*tty).term, TTYC_RMCUP));
    setblocking((*c).fd, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tty_close(mut tty: *mut tty) {
    if event_initialized(&mut (*tty).key_timer) != 0 {
        event_del(&mut (*tty).key_timer);
    }
    tty_stop_tty(tty);
    if (*tty).flags & 0x20 as libc::c_int != 0 {
        evbuffer_free((*tty).in_0);
        event_del(&mut (*tty).event_in);
        evbuffer_free((*tty).out);
        event_del(&mut (*tty).event_out);
        tty_term_free((*tty).term);
        tty_keys_free(tty);
        (*tty).flags &= !(0x20 as libc::c_int)
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
    if (*(*tty).term).flags & 0x4 as libc::c_int != 0 {
        tty_putcode(tty, TTYC_ENMG);
    }
    if options_get_number(global_options,
                          b"extended-keys\x00" as *const u8 as
                              *const libc::c_char) != 0 {
        tty_puts(tty, tty_term_string((*tty).term, TTYC_ENEKS));
    }
    if options_get_number(global_options,
                          b"focus-events\x00" as *const u8 as
                              *const libc::c_char) != 0 {
        tty_puts(tty, tty_term_string((*tty).term, TTYC_ENFCS));
    }
    if (*(*tty).term).flags & 0x20 as libc::c_int != 0 {
        tty_puts(tty, b"\x1b[?7727h\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_raw(mut tty: *mut tty,
                                 mut s: *const libc::c_char) {
    let mut c: *mut client = (*tty).client;
    let mut n: ssize_t = 0;
    let mut slen: ssize_t = 0;
    let mut i: u_int = 0;
    slen = strlen(s) as ssize_t;
    i = 0 as libc::c_int as u_int;
    while i < 5 as libc::c_int as libc::c_uint {
        n = write((*c).fd, s as *const libc::c_void, slen as size_t);
        if n >= 0 as libc::c_int as libc::c_long {
            s = s.offset(n as isize);
            slen -= n;
            if slen == 0 as libc::c_int as libc::c_long { break ; }
        } else if n == -(1 as libc::c_int) as libc::c_long &&
                      *__errno_location() != 11 as libc::c_int {
            break ;
        }
        usleep(100 as libc::c_int as __useconds_t);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode(mut tty: *mut tty,
                                     mut code: tty_code_code) {
    tty_puts(tty, tty_term_string((*tty).term, code));
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode1(mut tty: *mut tty,
                                      mut code: tty_code_code,
                                      mut a: libc::c_int) {
    if a < 0 as libc::c_int { return }
    tty_puts(tty, tty_term_string1((*tty).term, code, a));
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode2(mut tty: *mut tty,
                                      mut code: tty_code_code,
                                      mut a: libc::c_int,
                                      mut b: libc::c_int) {
    if a < 0 as libc::c_int || b < 0 as libc::c_int { return }
    tty_puts(tty, tty_term_string2((*tty).term, code, a, b));
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode3(mut tty: *mut tty,
                                      mut code: tty_code_code,
                                      mut a: libc::c_int, mut b: libc::c_int,
                                      mut c: libc::c_int) {
    if a < 0 as libc::c_int || b < 0 as libc::c_int || c < 0 as libc::c_int {
        return
    }
    tty_puts(tty, tty_term_string3((*tty).term, code, a, b, c));
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode_ptr1(mut tty: *mut tty,
                                          mut code: tty_code_code,
                                          mut a: *const libc::c_void) {
    if !a.is_null() { tty_puts(tty, tty_term_ptr1((*tty).term, code, a)); };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putcode_ptr2(mut tty: *mut tty,
                                          mut code: tty_code_code,
                                          mut a: *const libc::c_void,
                                          mut b: *const libc::c_void) {
    if !a.is_null() && !b.is_null() {
        tty_puts(tty, tty_term_ptr2((*tty).term, code, a, b));
    };
}
unsafe extern "C" fn tty_add(mut tty: *mut tty, mut buf: *const libc::c_char,
                             mut len: size_t) {
    let mut c: *mut client = (*tty).client;
    if (*tty).flags & 0x80 as libc::c_int != 0 {
        (*tty).discarded =
            ((*tty).discarded as libc::c_ulong).wrapping_add(len) as size_t as
                size_t;
        return
    }
    evbuffer_add((*tty).out, buf as *const libc::c_void, len);
    log_debug(b"%s: %.*s\x00" as *const u8 as *const libc::c_char, (*c).name,
              len as libc::c_int, buf);
    (*c).written =
        ((*c).written as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    if tty_log_fd != -(1 as libc::c_int) {
        write(tty_log_fd, buf as *const libc::c_void, len);
    }
    if (*tty).flags & 0x10 as libc::c_int != 0 {
        event_add(&mut (*tty).event_out, 0 as *const timeval);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_puts(mut tty: *mut tty,
                                  mut s: *const libc::c_char) {
    if *s as libc::c_int != '\u{0}' as i32 { tty_add(tty, s, strlen(s)); };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putc(mut tty: *mut tty, mut ch: u_char) {
    let mut acs: *const libc::c_char = 0 as *const libc::c_char;
    if (*(*tty).term).flags & 0x2 as libc::c_int != 0 &&
           ch as libc::c_int >= 0x20 as libc::c_int &&
           ch as libc::c_int != 0x7f as libc::c_int &&
           (*tty).cy ==
               (*tty).sy.wrapping_sub(1 as libc::c_int as libc::c_uint) &&
           (*tty).cx.wrapping_add(1 as libc::c_int as libc::c_uint) >=
               (*tty).sx {
        return
    }
    if (*tty).cell.attr as libc::c_int & 0x80 as libc::c_int != 0 {
        acs = tty_acs_get(tty, ch);
        if !acs.is_null() {
            tty_add(tty, acs, strlen(acs));
        } else {
            tty_add(tty, &mut ch as *mut u_char as *const libc::c_char,
                    1 as libc::c_int as size_t);
        }
    } else {
        tty_add(tty, &mut ch as *mut u_char as *const libc::c_char,
                1 as libc::c_int as size_t);
    }
    if ch as libc::c_int >= 0x20 as libc::c_int &&
           ch as libc::c_int != 0x7f as libc::c_int {
        if (*tty).cx >= (*tty).sx {
            (*tty).cx = 1 as libc::c_int as u_int;
            if (*tty).cy != (*tty).rlower {
                (*tty).cy = (*tty).cy.wrapping_add(1)
            }
            /*
			 * On !am terminals, force the cursor position to where
			 * we think it should be after a line wrap - this means
			 * it works on sensible terminals as well.
			 */
            if (*(*tty).term).flags & 0x2 as libc::c_int != 0 {
                tty_putcode2(tty, TTYC_CUP, (*tty).cy as libc::c_int,
                             (*tty).cx as libc::c_int);
            }
        } else { (*tty).cx = (*tty).cx.wrapping_add(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_putn(mut tty: *mut tty,
                                  mut buf: *const libc::c_void,
                                  mut len: size_t, mut width: u_int) {
    if (*(*tty).term).flags & 0x2 as libc::c_int != 0 &&
           (*tty).cy ==
               (*tty).sy.wrapping_sub(1 as libc::c_int as libc::c_uint) &&
           ((*tty).cx as libc::c_ulong).wrapping_add(len) >=
               (*tty).sx as libc::c_ulong {
        len =
            (*tty).sx.wrapping_sub((*tty).cx).wrapping_sub(1 as libc::c_int as
                                                               libc::c_uint)
                as size_t
    }
    tty_add(tty, buf as *const libc::c_char, len);
    if (*tty).cx.wrapping_add(width) > (*tty).sx {
        (*tty).cx = (*tty).cx.wrapping_add(width).wrapping_sub((*tty).sx);
        if (*tty).cx <= (*tty).sx {
            (*tty).cy = (*tty).cy.wrapping_add(1)
        } else {
            (*tty).cy =
                (2147483647 as libc::c_int as
                     libc::c_uint).wrapping_mul(2 as
                                                    libc::c_uint).wrapping_add(1
                                                                                   as
                                                                                   libc::c_uint);
            (*tty).cx = (*tty).cy
        }
    } else {
        (*tty).cx =
            ((*tty).cx as libc::c_uint).wrapping_add(width) as u_int as u_int
    };
}
unsafe extern "C" fn tty_set_italics(mut tty: *mut tty) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if tty_term_has((*tty).term, TTYC_SITM) != 0 {
        s =
            options_get_string(global_options,
                               b"default-terminal\x00" as *const u8 as
                                   *const libc::c_char);
        if strcmp(s, b"screen\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int &&
               strncmp(s, b"screen-\x00" as *const u8 as *const libc::c_char,
                       7 as libc::c_int as libc::c_ulong) != 0 as libc::c_int
           {
            tty_putcode(tty, TTYC_SITM);
            return
        }
    }
    tty_putcode(tty, TTYC_SMSO);
}
#[no_mangle]
pub unsafe extern "C" fn tty_set_title(mut tty: *mut tty,
                                       mut title: *const libc::c_char) {
    if tty_term_has((*tty).term, TTYC_TSL) == 0 ||
           tty_term_has((*tty).term, TTYC_FSL) == 0 {
        return
    }
    tty_putcode(tty, TTYC_TSL);
    tty_puts(tty, title);
    tty_putcode(tty, TTYC_FSL);
}
unsafe extern "C" fn tty_force_cursor_colour(mut tty: *mut tty,
                                             mut ccolour:
                                                 *const libc::c_char) {
    if *ccolour as libc::c_int == '\u{0}' as i32 {
        tty_putcode(tty, TTYC_CR);
    } else { tty_putcode_ptr1(tty, TTYC_CS, ccolour as *const libc::c_void); }
    free((*tty).ccolour as *mut libc::c_void);
    (*tty).ccolour = xstrdup(ccolour);
}
#[no_mangle]
pub unsafe extern "C" fn tty_update_mode(mut tty: *mut tty,
                                         mut mode: libc::c_int,
                                         mut s: *mut screen) {
    let mut c: *mut client = (*tty).client;
    let mut changed: libc::c_int = 0;
    if !s.is_null() &&
           strcmp((*s).ccolour, (*tty).ccolour) != 0 as libc::c_int {
        tty_force_cursor_colour(tty, (*s).ccolour);
    }
    if (*tty).flags & 0x1 as libc::c_int != 0 {
        mode &= !(0x1 as libc::c_int)
    }
    changed = mode ^ (*tty).mode;
    if changed != 0 as libc::c_int {
        log_debug(b"%s: update mode %x to %x\x00" as *const u8 as
                      *const libc::c_char, (*c).name, (*tty).mode, mode);
    }
    if changed & 0x80 as libc::c_int != 0 {
        if tty_term_has((*tty).term, TTYC_CVVIS) != 0 {
            tty_putcode(tty, TTYC_CVVIS);
        } else { tty_putcode(tty, TTYC_CNORM); }
        changed |= 0x1 as libc::c_int
    }
    if changed & 0x1 as libc::c_int != 0 {
        if mode & 0x1 as libc::c_int != 0 {
            tty_putcode(tty, TTYC_CNORM);
        } else { tty_putcode(tty, TTYC_CIVIS); }
    }
    if !s.is_null() && (*tty).cstyle != (*s).cstyle {
        if tty_term_has((*tty).term, TTYC_SS) != 0 {
            if (*s).cstyle == 0 as libc::c_int as libc::c_uint &&
                   tty_term_has((*tty).term, TTYC_SE) != 0 {
                tty_putcode(tty, TTYC_SE);
            } else { tty_putcode1(tty, TTYC_SS, (*s).cstyle as libc::c_int); }
        }
        (*tty).cstyle = (*s).cstyle
    }
    if changed &
           (0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int)
           != 0 && tty_term_has((*tty).term, TTYC_KMOUS) != 0 {
        if mode &
               (0x20 as libc::c_int | 0x40 as libc::c_int |
                    0x1000 as libc::c_int) == 0 as libc::c_int {
            tty_puts(tty,
                     b"\x1b[?1006l\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x20 as libc::c_int != 0 &&
               !mode & 0x20 as libc::c_int != 0 {
            tty_puts(tty,
                     b"\x1b[?1000l\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x40 as libc::c_int != 0 &&
               !mode & 0x40 as libc::c_int != 0 {
            tty_puts(tty,
                     b"\x1b[?1002l\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x1000 as libc::c_int != 0 &&
               !mode & 0x1000 as libc::c_int != 0 {
            tty_puts(tty,
                     b"\x1b[?1003l\x00" as *const u8 as *const libc::c_char);
        }
        if mode &
               (0x20 as libc::c_int | 0x40 as libc::c_int |
                    0x1000 as libc::c_int) != 0 {
            tty_puts(tty,
                     b"\x1b[?1006h\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x20 as libc::c_int != 0 &&
               mode & 0x20 as libc::c_int != 0 {
            tty_puts(tty,
                     b"\x1b[?1000h\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x40 as libc::c_int != 0 &&
               mode & 0x40 as libc::c_int != 0 {
            tty_puts(tty,
                     b"\x1b[?1002h\x00" as *const u8 as *const libc::c_char);
        }
        if changed & 0x1000 as libc::c_int != 0 &&
               mode & 0x1000 as libc::c_int != 0 {
            tty_puts(tty,
                     b"\x1b[?1003h\x00" as *const u8 as *const libc::c_char);
        }
    }
    if changed & 0x400 as libc::c_int != 0 {
        if mode & 0x400 as libc::c_int != 0 {
            tty_putcode(tty, TTYC_ENBP);
        } else { tty_putcode(tty, TTYC_DSBP); }
    }
    (*tty).mode = mode;
}
unsafe extern "C" fn tty_emulate_repeat(mut tty: *mut tty,
                                        mut code: tty_code_code,
                                        mut code1: tty_code_code,
                                        mut n: u_int) {
    if tty_term_has((*tty).term, code) != 0 {
        tty_putcode1(tty, code, n as libc::c_int);
    } else {
        loop  {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if !(fresh0 > 0 as libc::c_int as libc::c_uint) { break ; }
            tty_putcode(tty, code1);
        }
    };
}
unsafe extern "C" fn tty_repeat_space(mut tty: *mut tty, mut n: u_int) {
    static mut s: [libc::c_char; 500] = [0; 500];
    if *s.as_mut_ptr() as libc::c_int != ' ' as i32 {
        memset(s.as_mut_ptr() as *mut libc::c_void, ' ' as i32,
               ::std::mem::size_of::<[libc::c_char; 500]>() as libc::c_ulong);
    }
    while n as libc::c_ulong >
              ::std::mem::size_of::<[libc::c_char; 500]>() as libc::c_ulong {
        tty_putn(tty, s.as_mut_ptr() as *const libc::c_void,
                 ::std::mem::size_of::<[libc::c_char; 500]>() as
                     libc::c_ulong,
                 ::std::mem::size_of::<[libc::c_char; 500]>() as libc::c_ulong
                     as u_int);
        n =
            (n as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_char; 500]>()
                                                 as libc::c_ulong) as u_int as
                u_int
    }
    if n != 0 as libc::c_int as libc::c_uint {
        tty_putn(tty, s.as_mut_ptr() as *const libc::c_void, n as size_t, n);
    };
}
/* Is this window bigger than the terminal? */
#[no_mangle]
pub unsafe extern "C" fn tty_window_bigger(mut tty: *mut tty) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    return ((*tty).sx < (*w).sx ||
                (*tty).sy.wrapping_sub(status_line_size(c)) < (*w).sy) as
               libc::c_int;
}
/* What offset should this window be drawn at? */
#[no_mangle]
pub unsafe extern "C" fn tty_window_offset(mut tty: *mut tty,
                                           mut ox: *mut u_int,
                                           mut oy: *mut u_int,
                                           mut sx: *mut u_int,
                                           mut sy: *mut u_int)
 -> libc::c_int {
    *ox = (*tty).oox;
    *oy = (*tty).ooy;
    *sx = (*tty).osx;
    *sy = (*tty).osy;
    return (*tty).oflag;
}
/* What offset should this window be drawn at? */
unsafe extern "C" fn tty_window_offset1(mut tty: *mut tty, mut ox: *mut u_int,
                                        mut oy: *mut u_int,
                                        mut sx: *mut u_int,
                                        mut sy: *mut u_int) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = server_client_get_pane(c);
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut lines: u_int = 0;
    lines = status_line_size(c);
    if (*tty).sx >= (*w).sx && (*tty).sy.wrapping_sub(lines) >= (*w).sy {
        *ox = 0 as libc::c_int as u_int;
        *oy = 0 as libc::c_int as u_int;
        *sx = (*w).sx;
        *sy = (*w).sy;
        (*c).pan_window = 0 as *mut libc::c_void;
        return 0 as libc::c_int
    }
    *sx = (*tty).sx;
    *sy = (*tty).sy.wrapping_sub(lines);
    if (*c).pan_window == w as *mut libc::c_void {
        if *sx >= (*w).sx {
            (*c).pan_ox = 0 as libc::c_int as u_int
        } else if (*c).pan_ox.wrapping_add(*sx) > (*w).sx {
            (*c).pan_ox = (*w).sx.wrapping_sub(*sx)
        }
        *ox = (*c).pan_ox;
        if *sy >= (*w).sy {
            (*c).pan_oy = 0 as libc::c_int as u_int
        } else if (*c).pan_oy.wrapping_add(*sy) > (*w).sy {
            (*c).pan_oy = (*w).sy.wrapping_sub(*sy)
        }
        *oy = (*c).pan_oy;
        return 1 as libc::c_int
    }
    if !(*(*wp).screen).mode & 0x1 as libc::c_int != 0 {
        *ox = 0 as libc::c_int as u_int;
        *oy = 0 as libc::c_int as u_int
    } else {
        cx = (*wp).xoff.wrapping_add((*(*wp).screen).cx);
        cy = (*wp).yoff.wrapping_add((*(*wp).screen).cy);
        if cx < *sx {
            *ox = 0 as libc::c_int as u_int
        } else if cx > (*w).sx.wrapping_sub(*sx) {
            *ox = (*w).sx.wrapping_sub(*sx)
        } else {
            *ox =
                cx.wrapping_sub((*sx).wrapping_div(2 as libc::c_int as
                                                       libc::c_uint))
        }
        if cy < *sy {
            *oy = 0 as libc::c_int as u_int
        } else if cy > (*w).sy.wrapping_sub(*sy) {
            *oy = (*w).sy.wrapping_sub(*sy)
        } else {
            *oy =
                cy.wrapping_sub((*sy).wrapping_div(2 as libc::c_int as
                                                       libc::c_uint))
        }
    }
    (*c).pan_window = 0 as *mut libc::c_void;
    return 1 as libc::c_int;
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
    };
}
/* Update stored offsets for a client and redraw if necessary. */
#[no_mangle]
pub unsafe extern "C" fn tty_update_client_offset(mut c: *mut client) {
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    if !(*c).flags & 0x1 as libc::c_int as libc::c_ulong != 0 { return }
    (*c).tty.oflag =
        tty_window_offset1(&mut (*c).tty, &mut ox, &mut oy, &mut sx, &mut sy);
    if ox == (*c).tty.oox && oy == (*c).tty.ooy && sx == (*c).tty.osx &&
           sy == (*c).tty.osy {
        return
    }
    log_debug(b"%s: %s offset has changed (%u,%u %ux%u -> %u,%u %ux%u)\x00" as
                  *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"tty_update_client_offset\x00")).as_ptr(),
              (*c).name, (*c).tty.oox, (*c).tty.ooy, (*c).tty.osx,
              (*c).tty.osy, ox, oy, sx, sy);
    (*c).tty.oox = ox;
    (*c).tty.ooy = oy;
    (*c).tty.osx = sx;
    (*c).tty.osy = sy;
    (*c).flags |= (0x8 as libc::c_int | 0x10 as libc::c_int) as libc::c_ulong;
}
/* Get a palette entry. */
unsafe extern "C" fn tty_get_palette(mut palette: *mut libc::c_int,
                                     mut c: libc::c_int) -> libc::c_int {
    let mut new: libc::c_int = 0;
    if palette.is_null() { return -(1 as libc::c_int) }
    new = -(1 as libc::c_int);
    if c < 8 as libc::c_int {
        new = *palette.offset(c as isize)
    } else if c >= 90 as libc::c_int && c <= 97 as libc::c_int {
        new =
            *palette.offset((8 as libc::c_int + c - 90 as libc::c_int) as
                                isize)
    } else if c & 0x1000000 as libc::c_int != 0 {
        new = *palette.offset((c & !(0x1000000 as libc::c_int)) as isize)
    }
    if new == 0 as libc::c_int { return -(1 as libc::c_int) }
    return new;
}
/*
 * Is the region large enough to be worth redrawing once later rather than
 * probably several times now? Currently yes if it is more than 50% of the
 * pane.
 */
unsafe extern "C" fn tty_large_region(mut tty: *mut tty,
                                      mut ctx: *const tty_ctx)
 -> libc::c_int {
    return ((*ctx).orlower.wrapping_sub((*ctx).orupper) >=
                (*ctx).sy.wrapping_div(2 as libc::c_int as libc::c_uint)) as
               libc::c_int;
}
/*
 * Return if BCE is needed but the terminal doesn't have it - it'll need to be
 * emulated.
 */
unsafe extern "C" fn tty_fake_bce(mut tty: *const tty,
                                  mut gc: *const grid_cell, mut bg: u_int)
 -> libc::c_int {
    if tty_term_flag((*tty).term, TTYC_BCE) != 0 { return 0 as libc::c_int }
    if !(bg == 8 as libc::c_int as libc::c_uint ||
             bg == 9 as libc::c_int as libc::c_uint) ||
           !((*gc).bg == 8 as libc::c_int || (*gc).bg == 9 as libc::c_int) {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * Redraw scroll region using data from screen (already updated). Used when
 * CSR not supported, or window is a pane that doesn't take up the full
 * width of the terminal.
 */
unsafe extern "C" fn tty_redraw_region(mut tty: *mut tty,
                                       mut ctx: *const tty_ctx) {
    let mut c: *mut client = (*tty).client;
    let mut i: u_int = 0;
    /*
	 * If region is large, schedule a redraw. In most cases this is likely
	 * to be followed by some more scrolling.
	 */
    if tty_large_region(tty, ctx) != 0 {
        log_debug(b"%s: %s large redraw\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"tty_redraw_region\x00")).as_ptr(),
                  (*c).name);
        (*ctx).redraw_cb.expect("non-null function pointer")(ctx);
        return
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
unsafe extern "C" fn tty_is_visible(mut tty: *mut tty,
                                    mut ctx: *const tty_ctx, mut px: u_int,
                                    mut py: u_int, mut nx: u_int,
                                    mut ny: u_int) -> libc::c_int {
    let mut xoff: u_int = (*ctx).rxoff.wrapping_add(px);
    let mut yoff: u_int = (*ctx).ryoff.wrapping_add(py);
    if (*ctx).bigger == 0 { return 1 as libc::c_int }
    if xoff.wrapping_add(nx) <= (*ctx).wox ||
           xoff >= (*ctx).wox.wrapping_add((*ctx).wsx) ||
           yoff.wrapping_add(ny) <= (*ctx).woy ||
           yoff >= (*ctx).woy.wrapping_add((*ctx).wsy) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Clamp line position to visible part of pane. */
unsafe extern "C" fn tty_clamp_line(mut tty: *mut tty,
                                    mut ctx: *const tty_ctx, mut px: u_int,
                                    mut py: u_int, mut nx: u_int,
                                    mut i: *mut u_int, mut x: *mut u_int,
                                    mut rx: *mut u_int, mut ry: *mut u_int)
 -> libc::c_int {
    let mut xoff: u_int = (*ctx).rxoff.wrapping_add(px);
    if tty_is_visible(tty, ctx, px, py, nx, 1 as libc::c_int as u_int) == 0 {
        return 0 as libc::c_int
    }
    *ry = (*ctx).yoff.wrapping_add(py).wrapping_sub((*ctx).woy);
    if xoff >= (*ctx).wox &&
           xoff.wrapping_add(nx) <= (*ctx).wox.wrapping_add((*ctx).wsx) {
        /* All visible. */
        *i = 0 as libc::c_int as u_int;
        *x = (*ctx).xoff.wrapping_add(px).wrapping_sub((*ctx).wox);
        *rx = nx
    } else if xoff < (*ctx).wox &&
                  xoff.wrapping_add(nx) > (*ctx).wox.wrapping_add((*ctx).wsx)
     {
        /* Both left and right not visible. */
        *i = (*ctx).wox;
        *x = 0 as libc::c_int as u_int;
        *rx = (*ctx).wsx
    } else if xoff < (*ctx).wox {
        /* Left not visible. */
        *i = (*ctx).wox.wrapping_sub((*ctx).xoff.wrapping_add(px));
        *x = 0 as libc::c_int as u_int;
        *rx = nx.wrapping_sub(*i)
    } else {
        /* Right not visible. */
        *i = 0 as libc::c_int as u_int;
        *x = (*ctx).xoff.wrapping_add(px).wrapping_sub((*ctx).wox);
        *rx = (*ctx).wsx.wrapping_sub(*x)
    }
    if *rx > nx {
        fatalx(b"%s: x too big, %u > %u\x00" as *const u8 as
                   *const libc::c_char,
               (*::std::mem::transmute::<&[u8; 15],
                                         &[libc::c_char; 15]>(b"tty_clamp_line\x00")).as_ptr(),
               *rx, nx);
    }
    return 1 as libc::c_int;
}
/* Clear a line. */
unsafe extern "C" fn tty_clear_line(mut tty: *mut tty,
                                    mut defaults: *const grid_cell,
                                    mut py: u_int, mut px: u_int,
                                    mut nx: u_int, mut bg: u_int) {
    let mut c: *mut client = (*tty).client;
    log_debug(b"%s: %s, %u at %u,%u\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"tty_clear_line\x00")).as_ptr(),
              (*c).name, nx, px, py);
    /* Nothing to clear. */
    if nx == 0 as libc::c_int as libc::c_uint { return }
    /* If genuine BCE is available, can try escape sequences. */
    if tty_fake_bce(tty, defaults, bg) == 0 {
        /* Off the end of the line, use EL if available. */
        if px.wrapping_add(nx) >= (*tty).sx &&
               tty_term_has((*tty).term, TTYC_EL) != 0 {
            tty_cursor(tty, px, py);
            tty_putcode(tty, TTYC_EL);
            return
        }
        /* At the start of the line. Use EL1. */
        if px == 0 as libc::c_int as libc::c_uint &&
               tty_term_has((*tty).term, TTYC_EL1) != 0 {
            tty_cursor(tty,
                       px.wrapping_add(nx).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint),
                       py);
            tty_putcode(tty, TTYC_EL1);
            return
        }
        /* Section of line. Use ECH if possible. */
        if tty_term_has((*tty).term, TTYC_ECH) != 0 {
            tty_cursor(tty, px, py);
            tty_putcode1(tty, TTYC_ECH, nx as libc::c_int);
            return
        }
    }
    /* Couldn't use an escape sequence, use spaces. */
    tty_cursor(tty, px, py);
    tty_repeat_space(tty, nx);
}
/* Clear a line, adjusting to visible part of pane. */
unsafe extern "C" fn tty_clear_pane_line(mut tty: *mut tty,
                                         mut ctx: *const tty_ctx,
                                         mut py: u_int, mut px: u_int,
                                         mut nx: u_int, mut bg: u_int) {
    let mut c: *mut client = (*tty).client;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut rx: u_int = 0;
    let mut ry: u_int = 0;
    log_debug(b"%s: %s, %u at %u,%u\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"tty_clear_pane_line\x00")).as_ptr(),
              (*c).name, nx, px, py);
    if tty_clamp_line(tty, ctx, px, py, nx, &mut i, &mut x, &mut rx, &mut ry)
           != 0 {
        tty_clear_line(tty, &(*ctx).defaults, ry, x, rx, bg);
    };
}
/* Clamp area position to visible part of pane. */
unsafe extern "C" fn tty_clamp_area(mut tty: *mut tty,
                                    mut ctx: *const tty_ctx, mut px: u_int,
                                    mut py: u_int, mut nx: u_int,
                                    mut ny: u_int, mut i: *mut u_int,
                                    mut j: *mut u_int, mut x: *mut u_int,
                                    mut y: *mut u_int, mut rx: *mut u_int,
                                    mut ry: *mut u_int) -> libc::c_int {
    let mut xoff: u_int = (*ctx).rxoff.wrapping_add(px);
    let mut yoff: u_int = (*ctx).ryoff.wrapping_add(py);
    if tty_is_visible(tty, ctx, px, py, nx, ny) == 0 {
        return 0 as libc::c_int
    }
    if xoff >= (*ctx).wox &&
           xoff.wrapping_add(nx) <= (*ctx).wox.wrapping_add((*ctx).wsx) {
        /* All visible. */
        *i = 0 as libc::c_int as u_int;
        *x = (*ctx).xoff.wrapping_add(px).wrapping_sub((*ctx).wox);
        *rx = nx
    } else if xoff < (*ctx).wox &&
                  xoff.wrapping_add(nx) > (*ctx).wox.wrapping_add((*ctx).wsx)
     {
        /* Both left and right not visible. */
        *i = (*ctx).wox;
        *x = 0 as libc::c_int as u_int;
        *rx = (*ctx).wsx
    } else if xoff < (*ctx).wox {
        /* Left not visible. */
        *i = (*ctx).wox.wrapping_sub((*ctx).xoff.wrapping_add(px));
        *x = 0 as libc::c_int as u_int;
        *rx = nx.wrapping_sub(*i)
    } else {
        /* Right not visible. */
        *i = 0 as libc::c_int as u_int;
        *x = (*ctx).xoff.wrapping_add(px).wrapping_sub((*ctx).wox);
        *rx = (*ctx).wsx.wrapping_sub(*x)
    }
    if *rx > nx {
        fatalx(b"%s: x too big, %u > %u\x00" as *const u8 as
                   *const libc::c_char,
               (*::std::mem::transmute::<&[u8; 15],
                                         &[libc::c_char; 15]>(b"tty_clamp_area\x00")).as_ptr(),
               *rx, nx);
    }
    if yoff >= (*ctx).woy &&
           yoff.wrapping_add(ny) <= (*ctx).woy.wrapping_add((*ctx).wsy) {
        /* All visible. */
        *j = 0 as libc::c_int as u_int;
        *y = (*ctx).yoff.wrapping_add(py).wrapping_sub((*ctx).woy);
        *ry = ny
    } else if yoff < (*ctx).woy &&
                  yoff.wrapping_add(ny) > (*ctx).woy.wrapping_add((*ctx).wsy)
     {
        /* Both top and bottom not visible. */
        *j = (*ctx).woy;
        *y = 0 as libc::c_int as u_int;
        *ry = (*ctx).wsy
    } else if yoff < (*ctx).woy {
        /* Top not visible. */
        *j = (*ctx).woy.wrapping_sub((*ctx).yoff.wrapping_add(py));
        *y = 0 as libc::c_int as u_int;
        *ry = ny.wrapping_sub(*j)
    } else {
        /* Bottom not visible. */
        *j = 0 as libc::c_int as u_int;
        *y = (*ctx).yoff.wrapping_add(py).wrapping_sub((*ctx).woy);
        *ry = (*ctx).wsy.wrapping_sub(*y)
    }
    if *ry > ny {
        fatalx(b"%s: y too big, %u > %u\x00" as *const u8 as
                   *const libc::c_char,
               (*::std::mem::transmute::<&[u8; 15],
                                         &[libc::c_char; 15]>(b"tty_clamp_area\x00")).as_ptr(),
               *ry, ny);
    }
    return 1 as libc::c_int;
}
/* Clear an area, adjusting to visible part of pane. */
unsafe extern "C" fn tty_clear_area(mut tty: *mut tty,
                                    mut defaults: *const grid_cell,
                                    mut py: u_int, mut ny: u_int,
                                    mut px: u_int, mut nx: u_int,
                                    mut bg: u_int) {
    let mut c: *mut client = (*tty).client;
    let mut yy: u_int = 0;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    log_debug(b"%s: %s, %u,%u at %u,%u\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"tty_clear_area\x00")).as_ptr(),
              (*c).name, nx, ny, px, py);
    /* Nothing to clear. */
    if nx == 0 as libc::c_int as libc::c_uint ||
           ny == 0 as libc::c_int as libc::c_uint {
        return
    }
    /* If genuine BCE is available, can try escape sequences. */
    if tty_fake_bce(tty, defaults, bg) == 0 {
        /* Use ED if clearing off the bottom of the terminal. */
        if px == 0 as libc::c_int as libc::c_uint &&
               px.wrapping_add(nx) >= (*tty).sx &&
               py.wrapping_add(ny) >= (*tty).sy &&
               tty_term_has((*tty).term, TTYC_ED) != 0 {
            tty_cursor(tty, 0 as libc::c_int as u_int, py);
            tty_putcode(tty, TTYC_ED);
            return
        }
        /*
		 * On VT420 compatible terminals we can use DECFRA if the
		 * background colour isn't default (because it doesn't work
		 * after SGR 0).
		 */
        if (*(*tty).term).flags & 0x8 as libc::c_int != 0 &&
               !(bg == 8 as libc::c_int as libc::c_uint ||
                     bg == 9 as libc::c_int as libc::c_uint) {
            xsnprintf(tmp.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 64]>() as
                          libc::c_ulong,
                      b"\x1b[32;%u;%u;%u;%u$x\x00" as *const u8 as
                          *const libc::c_char,
                      py.wrapping_add(1 as libc::c_int as libc::c_uint),
                      px.wrapping_add(1 as libc::c_int as libc::c_uint),
                      py.wrapping_add(ny), px.wrapping_add(nx));
            tty_puts(tty, tmp.as_mut_ptr());
            return
        }
        /* Full lines can be scrolled away to clear them. */
        if px == 0 as libc::c_int as libc::c_uint &&
               px.wrapping_add(nx) >= (*tty).sx &&
               ny > 2 as libc::c_int as libc::c_uint &&
               tty_term_has((*tty).term, TTYC_CSR) != 0 &&
               tty_term_has((*tty).term, TTYC_INDN) != 0 {
            tty_region(tty, py,
                       py.wrapping_add(ny).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint));
            tty_margin_off(tty);
            tty_putcode1(tty, TTYC_INDN, ny as libc::c_int);
            return
        }
        /*
		 * If margins are supported, can just scroll the area off to
		 * clear it.
		 */
        if nx > 2 as libc::c_int as libc::c_uint &&
               ny > 2 as libc::c_int as libc::c_uint &&
               tty_term_has((*tty).term, TTYC_CSR) != 0 &&
               (*(*tty).term).flags & 0x4 as libc::c_int != 0 &&
               tty_term_has((*tty).term, TTYC_INDN) != 0 {
            tty_region(tty, py,
                       py.wrapping_add(ny).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint));
            tty_margin(tty, px,
                       px.wrapping_add(nx).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint));
            tty_putcode1(tty, TTYC_INDN, ny as libc::c_int);
            return
        }
    }
    /* Couldn't use an escape sequence, loop over the lines. */
    yy = py;
    while yy < py.wrapping_add(ny) {
        tty_clear_line(tty, defaults, yy, px, nx, bg);
        yy = yy.wrapping_add(1)
    };
}
/* Clear an area in a pane. */
unsafe extern "C" fn tty_clear_pane_area(mut tty: *mut tty,
                                         mut ctx: *const tty_ctx,
                                         mut py: u_int, mut ny: u_int,
                                         mut px: u_int, mut nx: u_int,
                                         mut bg: u_int) {
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut rx: u_int = 0;
    let mut ry: u_int = 0;
    if tty_clamp_area(tty, ctx, px, py, nx, ny, &mut i, &mut j, &mut x,
                      &mut y, &mut rx, &mut ry) != 0 {
        tty_clear_area(tty, &(*ctx).defaults, y, ry, x, rx, bg);
    };
}
unsafe extern "C" fn tty_draw_pane(mut tty: *mut tty, mut ctx: *const tty_ctx,
                                   mut py: u_int) {
    let mut s: *mut screen = (*ctx).s;
    let mut nx: u_int = (*ctx).sx;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut rx: u_int = 0;
    let mut ry: u_int = 0;
    log_debug(b"%s: %s %u %d\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"tty_draw_pane\x00")).as_ptr(),
              (*(*tty).client).name, py, (*ctx).bigger);
    if (*ctx).bigger == 0 {
        tty_draw_line(tty, s, 0 as libc::c_int as u_int, py, nx, (*ctx).xoff,
                      (*ctx).yoff.wrapping_add(py), &(*ctx).defaults,
                      (*ctx).palette);
        return
    }
    if tty_clamp_line(tty, ctx, 0 as libc::c_int as u_int, py, nx, &mut i,
                      &mut x, &mut rx, &mut ry) != 0 {
        tty_draw_line(tty, s, i, py, rx, x, ry, &(*ctx).defaults,
                      (*ctx).palette);
    };
}
unsafe extern "C" fn tty_check_codeset(mut tty: *mut tty,
                                       mut gc: *const grid_cell)
 -> *const grid_cell {
    static mut new: grid_cell =
        grid_cell{data: utf8_data{data: [0; 21], have: 0, size: 0, width: 0,},
                  attr: 0,
                  flags: 0,
                  fg: 0,
                  bg: 0,
                  us: 0,};
    let mut c: libc::c_int = 0;
    /* Characters less than 0x7f are always fine, no matter what. */
    if (*gc).data.size as libc::c_int == 1 as libc::c_int &&
           (*(*gc).data.data.as_ptr() as libc::c_int) < 0x7f as libc::c_int {
        return gc
    }
    /* UTF-8 terminal and a UTF-8 character - fine. */
    if (*(*tty).client).flags & 0x10000 as libc::c_int as libc::c_ulong != 0 {
        return gc
    }
    memcpy(&mut new as *mut grid_cell as *mut libc::c_void,
           gc as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    /* See if this can be mapped to an ACS character. */
    c =
        tty_acs_reverse_get(tty,
                            (*gc).data.data.as_ptr() as *const libc::c_char,
                            (*gc).data.size as size_t);
    if c != -(1 as libc::c_int) {
        utf8_set(&mut new.data, c as u_char);
        new.attr = (new.attr as libc::c_int | 0x80 as libc::c_int) as u_short;
        return &mut new
    }
    /* Replace by the right number of underscores. */
    new.data.size = (*gc).data.width;
    if new.data.size as libc::c_int > 21 as libc::c_int {
        new.data.size = 21 as libc::c_int as u_char
    }
    memset(new.data.data.as_mut_ptr() as *mut libc::c_void, '_' as i32,
           new.data.size as libc::c_ulong);
    return &mut new;
}
unsafe extern "C" fn tty_check_overlay(mut tty: *mut tty, mut px: u_int,
                                       mut py: u_int) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    if (*c).overlay_check.is_none() { return 1 as libc::c_int }
    return (*c).overlay_check.expect("non-null function pointer")(c, px, py);
}
#[no_mangle]
pub unsafe extern "C" fn tty_draw_line(mut tty: *mut tty, mut s: *mut screen,
                                       mut px: u_int, mut py: u_int,
                                       mut nx: u_int, mut atx: u_int,
                                       mut aty: u_int,
                                       mut defaults: *const grid_cell,
                                       mut palette: *mut libc::c_int) {
    let mut gd: *mut grid = (*s).grid;
    let mut gc: grid_cell =
        grid_cell{data: utf8_data{data: [0; 21], have: 0, size: 0, width: 0,},
                  attr: 0,
                  flags: 0,
                  fg: 0,
                  bg: 0,
                  us: 0,};
    let mut last: grid_cell =
        grid_cell{data: utf8_data{data: [0; 21], have: 0, size: 0, width: 0,},
                  attr: 0,
                  flags: 0,
                  fg: 0,
                  bg: 0,
                  us: 0,};
    let mut gcp: *const grid_cell = 0 as *const grid_cell;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut ux: u_int = 0;
    let mut sx: u_int = 0;
    let mut width: u_int = 0;
    let mut flags: libc::c_int = 0;
    let mut cleared: libc::c_int = 0 as libc::c_int;
    let mut wrapped: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut len: size_t = 0;
    let mut cellsize: u_int = 0;
    log_debug(b"%s: px=%u py=%u nx=%u atx=%u aty=%u\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"tty_draw_line\x00")).as_ptr(),
              px, py, nx, atx, aty);
    /*
	 * py is the line in the screen to draw.
	 * px is the start x and nx is the width to draw.
	 * atx,aty is the line on the terminal to draw it.
	 */
    flags = (*tty).flags & 0x1 as libc::c_int;
    (*tty).flags |= 0x1 as libc::c_int;
    tty_update_mode(tty, (*tty).mode, s);
    tty_region_off(tty);
    tty_margin_off(tty);
    /*
	 * Clamp the width to cellsize - note this is not cellused, because
	 * there may be empty background cells after it (from BCE).
	 */
    sx = (*(*s).grid).sx;
    if nx > sx { nx = sx }
    cellsize = (*grid_get_line(gd, (*gd).hsize.wrapping_add(py))).cellsize;
    if sx > cellsize { sx = cellsize }
    if sx > (*tty).sx { sx = (*tty).sx }
    if sx > nx { sx = nx }
    ux = 0 as libc::c_int as u_int;
    if py == 0 as libc::c_int as libc::c_uint {
        gl = 0 as *mut grid_line
    } else {
        gl =
            grid_get_line(gd,
                          (*gd).hsize.wrapping_add(py).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint))
    }
    if gl.is_null() || !(*gl).flags & 0x1 as libc::c_int != 0 ||
           atx != 0 as libc::c_int as libc::c_uint || (*tty).cx < (*tty).sx ||
           nx < (*tty).sx {
        if nx < (*tty).sx && atx == 0 as libc::c_int as libc::c_uint &&
               px.wrapping_add(sx) != nx &&
               tty_term_has((*tty).term, TTYC_EL1) != 0 &&
               tty_fake_bce(tty, defaults, 8 as libc::c_int as u_int) == 0 {
            tty_default_attributes(tty, defaults, palette,
                                   8 as libc::c_int as u_int);
            tty_cursor(tty, nx.wrapping_sub(1 as libc::c_int as libc::c_uint),
                       aty);
            tty_putcode(tty, TTYC_EL1);
            cleared = 1 as libc::c_int
        }
    } else {
        log_debug(b"%s: wrapped line %u\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"tty_draw_line\x00")).as_ptr(),
                  aty);
        wrapped = 1 as libc::c_int
    }
    memcpy(&mut last as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    len = 0 as libc::c_int as size_t;
    width = 0 as libc::c_int as u_int;
    i = 0 as libc::c_int as u_int;
    while i < sx {
        grid_view_get_cell(gd, px.wrapping_add(i), py, &mut gc);
        gcp = tty_check_codeset(tty, &mut gc);
        if len != 0 as libc::c_int as libc::c_ulong &&
               (tty_check_overlay(tty,
                                  atx.wrapping_add(ux).wrapping_add(width),
                                  aty) == 0 ||
                    (*gcp).attr as libc::c_int & 0x80 as libc::c_int != 0 ||
                    (*gcp).flags as libc::c_int != last.flags as libc::c_int
                    || (*gcp).attr as libc::c_int != last.attr as libc::c_int
                    || (*gcp).fg != last.fg || (*gcp).bg != last.bg ||
                    (*gcp).us != last.us ||
                    ux.wrapping_add(width).wrapping_add((*gcp).data.width as
                                                            libc::c_uint) > nx
                    ||
                    (::std::mem::size_of::<[libc::c_char; 512]>() as
                         libc::c_ulong).wrapping_sub(len) <
                        (*gcp).data.size as libc::c_ulong) {
            tty_attributes(tty, &mut last, defaults, palette);
            if last.flags as libc::c_int & 0x40 as libc::c_int != 0 {
                log_debug(b"%s: %zu cleared\x00" as *const u8 as
                              *const libc::c_char,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"tty_draw_line\x00")).as_ptr(),
                          len);
                tty_clear_line(tty, defaults, aty, atx.wrapping_add(ux),
                               width, last.bg as u_int);
            } else {
                if wrapped == 0 || atx != 0 as libc::c_int as libc::c_uint ||
                       ux != 0 as libc::c_int as libc::c_uint {
                    tty_cursor(tty, atx.wrapping_add(ux), aty);
                }
                tty_putn(tty, buf.as_mut_ptr() as *const libc::c_void, len,
                         width);
            }
            ux = (ux as libc::c_uint).wrapping_add(width) as u_int as u_int;
            len = 0 as libc::c_int as size_t;
            width = 0 as libc::c_int as u_int;
            wrapped = 0 as libc::c_int
        }
        if (*gcp).flags as libc::c_int & 0x10 as libc::c_int != 0 {
            screen_select_cell(s, &mut last, gcp);
        } else {
            memcpy(&mut last as *mut grid_cell as *mut libc::c_void,
                   gcp as *const libc::c_void,
                   ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
        }
        if tty_check_overlay(tty, atx.wrapping_add(ux), aty) == 0 {
            if !((*gcp).flags as libc::c_int) & 0x4 as libc::c_int != 0 {
                ux =
                    (ux as
                         libc::c_uint).wrapping_add((*gcp).data.width as
                                                        libc::c_uint) as u_int
                        as u_int
            }
        } else if ux.wrapping_add((*gcp).data.width as libc::c_uint) > nx {
            tty_attributes(tty, &mut last, defaults, palette);
            tty_cursor(tty, atx.wrapping_add(ux), aty);
            j = 0 as libc::c_int as u_int;
            while j < (*gcp).data.width as libc::c_uint {
                if ux.wrapping_add(j) > nx { break ; }
                tty_putc(tty, ' ' as i32 as u_char);
                ux = ux.wrapping_add(1);
                j = j.wrapping_add(1)
            }
        } else if (*gcp).attr as libc::c_int & 0x80 as libc::c_int != 0 {
            tty_attributes(tty, &mut last, defaults, palette);
            tty_cursor(tty, atx.wrapping_add(ux), aty);
            j = 0 as libc::c_int as u_int;
            while j < (*gcp).data.size as libc::c_uint {
                tty_putc(tty, (*gcp).data.data[j as usize]);
                j = j.wrapping_add(1)
            }
            ux =
                (ux as
                     libc::c_uint).wrapping_add((*gcp).data.width as
                                                    libc::c_uint) as u_int as
                    u_int
        } else if !((*gcp).flags as libc::c_int) & 0x4 as libc::c_int != 0 {
            memcpy(buf.as_mut_ptr().offset(len as isize) as *mut libc::c_void,
                   (*gcp).data.data.as_ptr() as *const libc::c_void,
                   (*gcp).data.size as libc::c_ulong);
            len =
                (len as
                     libc::c_ulong).wrapping_add((*gcp).data.size as
                                                     libc::c_ulong) as size_t
                    as size_t;
            width =
                (width as
                     libc::c_uint).wrapping_add((*gcp).data.width as
                                                    libc::c_uint) as u_int as
                    u_int
        }
        i = i.wrapping_add(1)
    }
    if len != 0 as libc::c_int as libc::c_ulong &&
           (!(last.flags as libc::c_int) & 0x40 as libc::c_int != 0 ||
                last.bg != 8 as libc::c_int) {
        tty_attributes(tty, &mut last, defaults, palette);
        if last.flags as libc::c_int & 0x40 as libc::c_int != 0 {
            log_debug(b"%s: %zu cleared (end)\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"tty_draw_line\x00")).as_ptr(),
                      len);
            tty_clear_line(tty, defaults, aty, atx.wrapping_add(ux), width,
                           last.bg as u_int);
        } else {
            if wrapped == 0 || atx != 0 as libc::c_int as libc::c_uint ||
                   ux != 0 as libc::c_int as libc::c_uint {
                tty_cursor(tty, atx.wrapping_add(ux), aty);
            }
            tty_putn(tty, buf.as_mut_ptr() as *const libc::c_void, len,
                     width);
        }
        ux = (ux as libc::c_uint).wrapping_add(width) as u_int as u_int
    }
    if cleared == 0 && ux < nx {
        log_debug(b"%s: %u to end of line (%zu cleared)\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"tty_draw_line\x00")).as_ptr(),
                  nx.wrapping_sub(ux), len);
        tty_default_attributes(tty, defaults, palette,
                               8 as libc::c_int as u_int);
        tty_clear_line(tty, defaults, aty, atx.wrapping_add(ux),
                       nx.wrapping_sub(ux), 8 as libc::c_int as u_int);
    }
    (*tty).flags = (*tty).flags & !(0x1 as libc::c_int) | flags;
    tty_update_mode(tty, (*tty).mode, s);
}
#[no_mangle]
pub unsafe extern "C" fn tty_sync_start(mut tty: *mut tty) {
    if (*tty).flags & 0x80 as libc::c_int != 0 { return }
    if (*tty).flags & 0x400 as libc::c_int != 0 { return }
    (*tty).flags |= 0x400 as libc::c_int;
    if tty_term_has((*tty).term, TTYC_SYNC) != 0 {
        log_debug(b"%s sync start\x00" as *const u8 as *const libc::c_char,
                  (*(*tty).client).name);
        tty_putcode1(tty, TTYC_SYNC, 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_sync_end(mut tty: *mut tty) {
    if (*tty).flags & 0x80 as libc::c_int != 0 { return }
    if !(*tty).flags & 0x400 as libc::c_int != 0 { return }
    (*tty).flags &= !(0x400 as libc::c_int);
    if tty_term_has((*tty).term, TTYC_SYNC) != 0 {
        log_debug(b"%s sync end\x00" as *const u8 as *const libc::c_char,
                  (*(*tty).client).name);
        tty_putcode1(tty, TTYC_SYNC, 2 as libc::c_int);
    };
}
unsafe extern "C" fn tty_client_ready(mut c: *mut client) -> libc::c_int {
    if (*c).session.is_null() || (*c).tty.term.is_null() {
        return 0 as libc::c_int
    }
    if (*c).flags &
           (0x8 as libc::c_int | 0x40 as libc::c_int) as libc::c_ulong != 0 {
        return 0 as libc::c_int
    }
    if (*c).tty.flags & 0x2 as libc::c_int != 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tty_write(mut cmdfn:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut tty,
                                                                   _:
                                                                       *const tty_ctx)
                                                  -> ()>,
                                   mut ctx: *mut tty_ctx) {
    let mut c: *mut client = 0 as *mut client;
    let mut state: libc::c_int = 0;
    if (*ctx).set_client_cb.is_none() { return }
    c = clients.tqh_first;
    while !c.is_null() {
        if !(tty_client_ready(c) == 0) {
            state =
                (*ctx).set_client_cb.expect("non-null function pointer")(ctx,
                                                                         c);
            if state == -(1 as libc::c_int) { break ; }
            if !(state == 0 as libc::c_int) {
                cmdfn.expect("non-null function pointer")(&mut (*c).tty, ctx);
            }
        }
        c = (*c).entry.tqe_next
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_insertcharacter(mut tty: *mut tty,
                                                 mut ctx: *const tty_ctx) {
    if (*ctx).bigger != 0 ||
           !((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                 (*ctx).sx >= (*tty).sx) ||
           tty_fake_bce(tty, &(*ctx).defaults, (*ctx).bg) != 0 ||
           tty_term_has((*tty).term, TTYC_ICH) == 0 &&
               tty_term_has((*tty).term, TTYC_ICH1) == 0 {
        tty_draw_pane(tty, ctx, (*ctx).ocy);
        return
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_emulate_repeat(tty, TTYC_ICH, TTYC_ICH1, (*ctx).num);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_deletecharacter(mut tty: *mut tty,
                                                 mut ctx: *const tty_ctx) {
    if (*ctx).bigger != 0 ||
           !((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                 (*ctx).sx >= (*tty).sx) ||
           tty_fake_bce(tty, &(*ctx).defaults, (*ctx).bg) != 0 ||
           tty_term_has((*tty).term, TTYC_DCH) == 0 &&
               tty_term_has((*tty).term, TTYC_DCH1) == 0 {
        tty_draw_pane(tty, ctx, (*ctx).ocy);
        return
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_emulate_repeat(tty, TTYC_DCH, TTYC_DCH1, (*ctx).num);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearcharacter(mut tty: *mut tty,
                                                mut ctx: *const tty_ctx) {
    if (*ctx).bigger != 0 { tty_draw_pane(tty, ctx, (*ctx).ocy); return }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    if tty_term_has((*tty).term, TTYC_ECH) != 0 &&
           tty_fake_bce(tty, &(*ctx).defaults, 8 as libc::c_int as u_int) == 0
       {
        tty_putcode1(tty, TTYC_ECH, (*ctx).num as libc::c_int);
    } else { tty_repeat_space(tty, (*ctx).num); };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_insertline(mut tty: *mut tty,
                                            mut ctx: *const tty_ctx) {
    if (*ctx).bigger != 0 ||
           !((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                 (*ctx).sx >= (*tty).sx) ||
           tty_fake_bce(tty, &(*ctx).defaults, (*ctx).bg) != 0 ||
           tty_term_has((*tty).term, TTYC_CSR) == 0 ||
           tty_term_has((*tty).term, TTYC_IL1) == 0 ||
           (*ctx).sx == 1 as libc::c_int as libc::c_uint ||
           (*ctx).sy == 1 as libc::c_int as libc::c_uint {
        tty_redraw_region(tty, ctx);
        return
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_off(tty);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_emulate_repeat(tty, TTYC_IL, TTYC_IL1, (*ctx).num);
    (*tty).cy =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    (*tty).cx = (*tty).cy;
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_deleteline(mut tty: *mut tty,
                                            mut ctx: *const tty_ctx) {
    if (*ctx).bigger != 0 ||
           !((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                 (*ctx).sx >= (*tty).sx) ||
           tty_fake_bce(tty, &(*ctx).defaults, (*ctx).bg) != 0 ||
           tty_term_has((*tty).term, TTYC_CSR) == 0 ||
           tty_term_has((*tty).term, TTYC_DL1) == 0 ||
           (*ctx).sx == 1 as libc::c_int as libc::c_uint ||
           (*ctx).sy == 1 as libc::c_int as libc::c_uint {
        tty_redraw_region(tty, ctx);
        return
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_off(tty);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_emulate_repeat(tty, TTYC_DL, TTYC_DL1, (*ctx).num);
    (*tty).cy =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    (*tty).cx = (*tty).cy;
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearline(mut tty: *mut tty,
                                           mut ctx: *const tty_ctx) {
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_clear_pane_line(tty, ctx, (*ctx).ocy, 0 as libc::c_int as u_int,
                        (*ctx).sx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearendofline(mut tty: *mut tty,
                                                mut ctx: *const tty_ctx) {
    let mut nx: u_int = (*ctx).sx.wrapping_sub((*ctx).ocx);
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_clear_pane_line(tty, ctx, (*ctx).ocy, (*ctx).ocx, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearstartofline(mut tty: *mut tty,
                                                  mut ctx: *const tty_ctx) {
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_clear_pane_line(tty, ctx, (*ctx).ocy, 0 as libc::c_int as u_int,
                        (*ctx).ocx.wrapping_add(1 as libc::c_int as
                                                    libc::c_uint), (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_reverseindex(mut tty: *mut tty,
                                              mut ctx: *const tty_ctx) {
    if (*ctx).ocy != (*ctx).orupper { return }
    if (*ctx).bigger != 0 ||
           !((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                 (*ctx).sx >= (*tty).sx) &&
               (*(*tty).term).flags & 0x4 as libc::c_int == 0 ||
           tty_fake_bce(tty, &(*ctx).defaults, 8 as libc::c_int as u_int) != 0
           || tty_term_has((*tty).term, TTYC_CSR) == 0 ||
           tty_term_has((*tty).term, TTYC_RI) == 0 &&
               tty_term_has((*tty).term, TTYC_RIN) == 0 ||
           (*ctx).sx == 1 as libc::c_int as libc::c_uint ||
           (*ctx).sy == 1 as libc::c_int as libc::c_uint {
        tty_redraw_region(tty, ctx);
        return
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_pane(tty, ctx);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).orupper);
    if tty_term_has((*tty).term, TTYC_RI) != 0 {
        tty_putcode(tty, TTYC_RI);
    } else { tty_putcode1(tty, TTYC_RIN, 1 as libc::c_int); };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_linefeed(mut tty: *mut tty,
                                          mut ctx: *const tty_ctx) {
    if (*ctx).ocy != (*ctx).orlower { return }
    if (*ctx).bigger != 0 ||
           !((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                 (*ctx).sx >= (*tty).sx) &&
               (*(*tty).term).flags & 0x4 as libc::c_int == 0 ||
           tty_fake_bce(tty, &(*ctx).defaults, 8 as libc::c_int as u_int) != 0
           || tty_term_has((*tty).term, TTYC_CSR) == 0 ||
           (*ctx).sx == 1 as libc::c_int as libc::c_uint ||
           (*ctx).sy == 1 as libc::c_int as libc::c_uint {
        tty_redraw_region(tty, ctx);
        return
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
        if (*(*tty).term).flags & 0x4 as libc::c_int == 0 {
            tty_cursor(tty, 0 as libc::c_int as u_int,
                       (*ctx).yoff.wrapping_add((*ctx).ocy)); /* storage for base64 */
        } else {
            tty_cursor(tty, (*tty).rright,
                       (*ctx).yoff.wrapping_add((*ctx).ocy));
        }
    } else { tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).ocy); }
    tty_putc(tty, '\n' as i32 as u_char);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_scrollup(mut tty: *mut tty,
                                          mut ctx: *const tty_ctx) {
    let mut i: u_int = 0;
    if (*ctx).bigger != 0 ||
           !((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                 (*ctx).sx >= (*tty).sx) &&
               (*(*tty).term).flags & 0x4 as libc::c_int == 0 ||
           tty_fake_bce(tty, &(*ctx).defaults, 8 as libc::c_int as u_int) != 0
           || tty_term_has((*tty).term, TTYC_CSR) == 0 ||
           (*ctx).sx == 1 as libc::c_int as libc::c_uint ||
           (*ctx).sy == 1 as libc::c_int as libc::c_uint {
        tty_redraw_region(tty, ctx);
        return
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_pane(tty, ctx);
    if (*ctx).num == 1 as libc::c_int as libc::c_uint ||
           tty_term_has((*tty).term, TTYC_INDN) == 0 {
        if (*(*tty).term).flags & 0x4 as libc::c_int == 0 {
            tty_cursor(tty, 0 as libc::c_int as u_int, (*tty).rlower);
        } else { tty_cursor(tty, (*tty).rright, (*tty).rlower); }
        i = 0 as libc::c_int as u_int;
        while i < (*ctx).num {
            tty_putc(tty, '\n' as i32 as u_char);
            i = i.wrapping_add(1)
        }
    } else {
        if (*tty).cy ==
               (2147483647 as libc::c_int as
                    libc::c_uint).wrapping_mul(2 as
                                                   libc::c_uint).wrapping_add(1
                                                                                  as
                                                                                  libc::c_uint)
           {
            tty_cursor(tty, 0 as libc::c_int as u_int,
                       0 as libc::c_int as u_int);
        } else { tty_cursor(tty, 0 as libc::c_int as u_int, (*tty).cy); }
        tty_putcode1(tty, TTYC_INDN, (*ctx).num as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_scrolldown(mut tty: *mut tty,
                                            mut ctx: *const tty_ctx) {
    let mut i: u_int = 0;
    if (*ctx).bigger != 0 ||
           !((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                 (*ctx).sx >= (*tty).sx) &&
               (*(*tty).term).flags & 0x4 as libc::c_int == 0 ||
           tty_fake_bce(tty, &(*ctx).defaults, 8 as libc::c_int as u_int) != 0
           || tty_term_has((*tty).term, TTYC_CSR) == 0 ||
           tty_term_has((*tty).term, TTYC_RI) == 0 &&
               tty_term_has((*tty).term, TTYC_RIN) == 0 ||
           (*ctx).sx == 1 as libc::c_int as libc::c_uint ||
           (*ctx).sy == 1 as libc::c_int as libc::c_uint {
        tty_redraw_region(tty, ctx);
        return
    }
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    tty_margin_pane(tty, ctx);
    tty_cursor_pane(tty, ctx, (*ctx).ocx, (*ctx).orupper);
    if tty_term_has((*tty).term, TTYC_RIN) != 0 {
        tty_putcode1(tty, TTYC_RIN, (*ctx).num as libc::c_int);
    } else {
        i = 0 as libc::c_int as u_int;
        while i < (*ctx).num {
            tty_putcode(tty, TTYC_RI);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearendofscreen(mut tty: *mut tty,
                                                  mut ctx: *const tty_ctx) {
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut nx: u_int = 0;
    let mut ny: u_int = 0;
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, 0 as libc::c_int as u_int,
                    (*ctx).sy.wrapping_sub(1 as libc::c_int as libc::c_uint));
    tty_margin_off(tty);
    px = 0 as libc::c_int as u_int;
    nx = (*ctx).sx;
    py = (*ctx).ocy.wrapping_add(1 as libc::c_int as libc::c_uint);
    ny =
        (*ctx).sy.wrapping_sub((*ctx).ocy).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint);
    tty_clear_pane_area(tty, ctx, py, ny, px, nx, (*ctx).bg);
    px = (*ctx).ocx;
    nx = (*ctx).sx.wrapping_sub((*ctx).ocx);
    py = (*ctx).ocy;
    tty_clear_pane_line(tty, ctx, py, px, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearstartofscreen(mut tty: *mut tty,
                                                    mut ctx: *const tty_ctx) {
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut nx: u_int = 0;
    let mut ny: u_int = 0;
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, 0 as libc::c_int as u_int,
                    (*ctx).sy.wrapping_sub(1 as libc::c_int as libc::c_uint));
    tty_margin_off(tty);
    px = 0 as libc::c_int as u_int;
    nx = (*ctx).sx;
    py = 0 as libc::c_int as u_int;
    ny = (*ctx).ocy;
    tty_clear_pane_area(tty, ctx, py, ny, px, nx, (*ctx).bg);
    px = 0 as libc::c_int as u_int;
    nx = (*ctx).ocx.wrapping_add(1 as libc::c_int as libc::c_uint);
    py = (*ctx).ocy;
    tty_clear_pane_line(tty, ctx, py, px, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_clearscreen(mut tty: *mut tty,
                                             mut ctx: *const tty_ctx) {
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut nx: u_int = 0;
    let mut ny: u_int = 0;
    tty_default_attributes(tty, &(*ctx).defaults, (*ctx).palette, (*ctx).bg);
    tty_region_pane(tty, ctx, 0 as libc::c_int as u_int,
                    (*ctx).sy.wrapping_sub(1 as libc::c_int as libc::c_uint));
    tty_margin_off(tty);
    px = 0 as libc::c_int as u_int;
    nx = (*ctx).sx;
    py = 0 as libc::c_int as u_int;
    ny = (*ctx).sy;
    tty_clear_pane_area(tty, ctx, py, ny, px, nx, (*ctx).bg);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_alignmenttest(mut tty: *mut tty,
                                               mut ctx: *const tty_ctx) {
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    if (*ctx).bigger != 0 {
        (*ctx).redraw_cb.expect("non-null function pointer")(ctx);
        return
    }
    tty_attributes(tty, &grid_default_cell, &(*ctx).defaults, (*ctx).palette);
    tty_region_pane(tty, ctx, 0 as libc::c_int as u_int,
                    (*ctx).sy.wrapping_sub(1 as libc::c_int as libc::c_uint));
    tty_margin_off(tty);
    j = 0 as libc::c_int as u_int;
    while j < (*ctx).sy {
        tty_cursor_pane(tty, ctx, 0 as libc::c_int as u_int, j);
        i = 0 as libc::c_int as u_int;
        while i < (*ctx).sx {
            tty_putc(tty, 'E' as i32 as u_char);
            i = i.wrapping_add(1)
        }
        j = j.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_cell(mut tty: *mut tty,
                                      mut ctx: *const tty_ctx) {
    if tty_is_visible(tty, ctx, (*ctx).ocx, (*ctx).ocy,
                      1 as libc::c_int as u_int, 1 as libc::c_int as u_int) ==
           0 {
        return
    }
    if (*ctx).xoff.wrapping_add((*ctx).ocx).wrapping_sub((*ctx).wox) >
           (*tty).sx.wrapping_sub(1 as libc::c_int as libc::c_uint) &&
           (*ctx).ocy == (*ctx).orlower &&
           ((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                (*ctx).sx >= (*tty).sx) {
        tty_region_pane(tty, ctx, (*ctx).orupper, (*ctx).orlower);
    }
    tty_margin_off(tty);
    tty_cursor_pane_unless_wrap(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_cell(tty, (*ctx).cell, &(*ctx).defaults, (*ctx).palette);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_cells(mut tty: *mut tty,
                                       mut ctx: *const tty_ctx) {
    if tty_is_visible(tty, ctx, (*ctx).ocx, (*ctx).ocy, (*ctx).num,
                      1 as libc::c_int as u_int) == 0 {
        return
    }
    if (*ctx).bigger != 0 &&
           ((*ctx).xoff.wrapping_add((*ctx).ocx) < (*ctx).wox ||
                (*ctx).xoff.wrapping_add((*ctx).ocx).wrapping_add((*ctx).num)
                    > (*ctx).wox.wrapping_add((*ctx).wsx)) {
        if (*ctx).wrapped == 0 ||
               !((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                     (*ctx).sx >= (*tty).sx) ||
               (*(*tty).term).flags & 0x2 as libc::c_int != 0 ||
               (*ctx).xoff.wrapping_add((*ctx).ocx) !=
                   0 as libc::c_int as libc::c_uint ||
               (*ctx).yoff.wrapping_add((*ctx).ocy) !=
                   (*tty).cy.wrapping_add(1 as libc::c_int as libc::c_uint) ||
               (*tty).cx < (*tty).sx || (*tty).cy == (*tty).rlower {
            tty_draw_pane(tty, ctx, (*ctx).ocy);
        } else { (*ctx).redraw_cb.expect("non-null function pointer")(ctx); }
        return
    }
    tty_margin_off(tty);
    tty_cursor_pane_unless_wrap(tty, ctx, (*ctx).ocx, (*ctx).ocy);
    tty_attributes(tty, (*ctx).cell, &(*ctx).defaults, (*ctx).palette);
    tty_putn(tty, (*ctx).ptr, (*ctx).num as size_t, (*ctx).num);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_setselection(mut tty: *mut tty,
                                              mut ctx: *const tty_ctx) {
    tty_set_selection(tty, (*ctx).ptr as *const libc::c_char,
                      (*ctx).num as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn tty_set_selection(mut tty: *mut tty,
                                           mut buf: *const libc::c_char,
                                           mut len: size_t) {
    let mut encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    if !(*tty).flags & 0x10 as libc::c_int != 0 { return }
    if tty_term_has((*tty).term, TTYC_MS) == 0 { return }
    size =
        (4 as libc::c_int as
             libc::c_ulong).wrapping_mul(len.wrapping_add(2 as libc::c_int as
                                                              libc::c_ulong).wrapping_div(3
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong)).wrapping_add(1
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong);
    encoded = xmalloc(size) as *mut libc::c_char;
    __b64_ntop(buf as *const libc::c_uchar, len, encoded, size);
    tty_putcode_ptr2(tty, TTYC_MS,
                     b"\x00" as *const u8 as *const libc::c_char as
                         *const libc::c_void, encoded as *const libc::c_void);
    free(encoded as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_rawstring(mut tty: *mut tty,
                                           mut ctx: *const tty_ctx) {
    tty_add(tty, (*ctx).ptr as *const libc::c_char, (*ctx).num as size_t);
    tty_invalidate(tty);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cmd_syncstart(mut tty: *mut tty,
                                           mut ctx: *const tty_ctx) {
    tty_sync_start(tty);
}
#[no_mangle]
pub unsafe extern "C" fn tty_cell(mut tty: *mut tty, mut gc: *const grid_cell,
                                  mut defaults: *const grid_cell,
                                  mut palette: *mut libc::c_int) {
    let mut gcp: *const grid_cell = 0 as *const grid_cell;
    /* Skip last character if terminal is stupid. */
    if (*(*tty).term).flags & 0x2 as libc::c_int != 0 &&
           (*tty).cy ==
               (*tty).sy.wrapping_sub(1 as libc::c_int as libc::c_uint) &&
           (*tty).cx ==
               (*tty).sx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        return
    }
    /* If this is a padding character, do nothing. */
    if (*gc).flags as libc::c_int & 0x4 as libc::c_int != 0 { return }
    /* Check the output codeset and apply attributes. */
    gcp = tty_check_codeset(tty, gc);
    tty_attributes(tty, gcp, defaults, palette);
    /* If it is a single character, write with putc to handle ACS. */
    if (*gcp).data.size as libc::c_int == 1 as libc::c_int {
        tty_attributes(tty, gcp, defaults, palette);
        if (*(*gcp).data.data.as_ptr() as libc::c_int) < 0x20 as libc::c_int
               ||
               *(*gcp).data.data.as_ptr() as libc::c_int ==
                   0x7f as libc::c_int {
            return
        }
        tty_putc(tty, *(*gcp).data.data.as_ptr());
        return
    }
    /* Write the data. */
    tty_putn(tty, (*gcp).data.data.as_ptr() as *const libc::c_void,
             (*gcp).data.size as size_t, (*gcp).data.width as u_int);
}
#[no_mangle]
pub unsafe extern "C" fn tty_reset(mut tty: *mut tty) {
    let mut gc: *mut grid_cell = &mut (*tty).cell;
    if grid_cells_equal(gc, &grid_default_cell) == 0 {
        if (*gc).attr as libc::c_int & 0x80 as libc::c_int != 0 &&
               tty_acs_needed(tty) != 0 {
            tty_putcode(tty, TTYC_RMACS);
        }
        tty_putcode(tty, TTYC_SGR0);
        memcpy(gc as *mut libc::c_void,
               &grid_default_cell as *const grid_cell as *const libc::c_void,
               ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    }
    memcpy(&mut (*tty).last_cell as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
}
unsafe extern "C" fn tty_invalidate(mut tty: *mut tty) {
    memcpy(&mut (*tty).cell as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    memcpy(&mut (*tty).last_cell as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    (*tty).cy =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    (*tty).cx = (*tty).cy;
    (*tty).rleft =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    (*tty).rupper = (*tty).rleft;
    (*tty).rright =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    (*tty).rlower = (*tty).rright;
    if (*tty).flags & 0x10 as libc::c_int != 0 {
        if (*(*tty).term).flags & 0x4 as libc::c_int != 0 {
            tty_putcode(tty, TTYC_ENMG);
        }
        tty_putcode(tty, TTYC_SGR0);
        (*tty).mode = 0xffffff as libc::c_int;
        tty_update_mode(tty, 0x1 as libc::c_int, 0 as *mut screen);
        tty_cursor(tty, 0 as libc::c_int as u_int, 0 as libc::c_int as u_int);
        tty_region_off(tty);
        tty_margin_off(tty);
    } else { (*tty).mode = 0x1 as libc::c_int };
}
/* Turn off margin. */
#[no_mangle]
pub unsafe extern "C" fn tty_region_off(mut tty: *mut tty) {
    tty_region(tty, 0 as libc::c_int as u_int,
               (*tty).sy.wrapping_sub(1 as libc::c_int as libc::c_uint));
}
/* Set region inside pane. */
unsafe extern "C" fn tty_region_pane(mut tty: *mut tty,
                                     mut ctx: *const tty_ctx,
                                     mut rupper: u_int, mut rlower: u_int) {
    tty_region(tty, (*ctx).yoff.wrapping_add(rupper).wrapping_sub((*ctx).woy),
               (*ctx).yoff.wrapping_add(rlower).wrapping_sub((*ctx).woy));
}
/* Set region at absolute position. */
unsafe extern "C" fn tty_region(mut tty: *mut tty, mut rupper: u_int,
                                mut rlower: u_int) {
    if (*tty).rlower == rlower && (*tty).rupper == rupper { return }
    if tty_term_has((*tty).term, TTYC_CSR) == 0 { return }
    (*tty).rupper = rupper;
    (*tty).rlower = rlower;
    /*
	 * Some terminals (such as PuTTY) do not correctly reset the cursor to
	 * 0,0 if it is beyond the last column (they do not reset their wrap
	 * flag so further output causes a line feed). As a workaround, do an
	 * explicit move to 0 first.
	 */
    if (*tty).cx >= (*tty).sx {
        if (*tty).cy ==
               (2147483647 as libc::c_int as
                    libc::c_uint).wrapping_mul(2 as
                                                   libc::c_uint).wrapping_add(1
                                                                                  as
                                                                                  libc::c_uint)
           {
            tty_cursor(tty, 0 as libc::c_int as u_int,
                       0 as libc::c_int as u_int);
        } else { tty_cursor(tty, 0 as libc::c_int as u_int, (*tty).cy); }
    }
    tty_putcode2(tty, TTYC_CSR, (*tty).rupper as libc::c_int,
                 (*tty).rlower as libc::c_int);
    (*tty).cy =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    (*tty).cx = (*tty).cy;
}
/* Turn off margin. */
#[no_mangle]
pub unsafe extern "C" fn tty_margin_off(mut tty: *mut tty) {
    tty_margin(tty, 0 as libc::c_int as u_int,
               (*tty).sx.wrapping_sub(1 as libc::c_int as libc::c_uint));
}
/* Set margin inside pane. */
unsafe extern "C" fn tty_margin_pane(mut tty: *mut tty,
                                     mut ctx: *const tty_ctx) {
    tty_margin(tty, (*ctx).xoff.wrapping_sub((*ctx).wox),
               (*ctx).xoff.wrapping_add((*ctx).sx).wrapping_sub(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint).wrapping_sub((*ctx).wox));
}
/* Set margin at absolute position. */
unsafe extern "C" fn tty_margin(mut tty: *mut tty, mut rleft: u_int,
                                mut rright: u_int) {
    if (*(*tty).term).flags & 0x4 as libc::c_int == 0 { return }
    if (*tty).rleft == rleft && (*tty).rright == rright { return }
    tty_putcode2(tty, TTYC_CSR, (*tty).rupper as libc::c_int,
                 (*tty).rlower as libc::c_int);
    (*tty).rleft = rleft;
    (*tty).rright = rright;
    if rleft == 0 as libc::c_int as libc::c_uint &&
           rright == (*tty).sx.wrapping_sub(1 as libc::c_int as libc::c_uint)
       {
        tty_putcode(tty, TTYC_CLMG);
    } else {
        tty_putcode2(tty, TTYC_CMG, rleft as libc::c_int,
                     rright as libc::c_int);
    }
    (*tty).cy =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    (*tty).cx = (*tty).cy;
}
/*
 * Move the cursor, unless it would wrap itself when the next character is
 * printed.
 */
unsafe extern "C" fn tty_cursor_pane_unless_wrap(mut tty: *mut tty,
                                                 mut ctx: *const tty_ctx,
                                                 mut cx: u_int,
                                                 mut cy: u_int) {
    if (*ctx).wrapped == 0 ||
           !((*ctx).xoff == 0 as libc::c_int as libc::c_uint &&
                 (*ctx).sx >= (*tty).sx) ||
           (*(*tty).term).flags & 0x2 as libc::c_int != 0 ||
           (*ctx).xoff.wrapping_add(cx) != 0 as libc::c_int as libc::c_uint ||
           (*ctx).yoff.wrapping_add(cy) !=
               (*tty).cy.wrapping_add(1 as libc::c_int as libc::c_uint) ||
           (*tty).cx < (*tty).sx || (*tty).cy == (*tty).rlower {
        tty_cursor_pane(tty, ctx, cx, cy);
    } else {
        log_debug(b"%s: will wrap at %u,%u\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 28],
                                            &[libc::c_char; 28]>(b"tty_cursor_pane_unless_wrap\x00")).as_ptr(),
                  (*tty).cx, (*tty).cy);
    };
}
/* Move cursor inside pane. */
unsafe extern "C" fn tty_cursor_pane(mut tty: *mut tty,
                                     mut ctx: *const tty_ctx, mut cx: u_int,
                                     mut cy: u_int) {
    tty_cursor(tty, (*ctx).xoff.wrapping_add(cx).wrapping_sub((*ctx).wox),
               (*ctx).yoff.wrapping_add(cy).wrapping_sub((*ctx).woy));
}
/* Move cursor to absolute position. */
#[no_mangle]
pub unsafe extern "C" fn tty_cursor(mut tty: *mut tty, mut cx: u_int,
                                    mut cy: u_int) {
    let mut current_block: u64;
    let mut term: *mut tty_term = (*tty).term;
    let mut thisx: u_int = 0;
    let mut thisy: u_int = 0;
    let mut change: libc::c_int = 0;
    if (*tty).flags & 0x80 as libc::c_int != 0 { return }
    if cx > (*tty).sx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        cx = (*tty).sx.wrapping_sub(1 as libc::c_int as libc::c_uint)
    }
    thisx = (*tty).cx;
    thisy = (*tty).cy;
    /* No change. */
    if cx == thisx && cy == thisy { return }
    /* Very end of the line, just use absolute movement. */
    if thisx > (*tty).sx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        current_block = 6863629183173758772;
    } else if cx == 0 as libc::c_int as libc::c_uint &&
                  cy == 0 as libc::c_int as libc::c_uint &&
                  tty_term_has(term, TTYC_HOME) != 0 {
        tty_putcode(tty, TTYC_HOME);
        current_block = 6993536316001430845;
    } else if cx == 0 as libc::c_int as libc::c_uint &&
                  cy == thisy.wrapping_add(1 as libc::c_int as libc::c_uint)
                  && thisy != (*tty).rlower &&
                  ((*(*tty).term).flags & 0x4 as libc::c_int == 0 ||
                       (*tty).rleft == 0 as libc::c_int as libc::c_uint) {
        tty_putc(tty, '\r' as i32 as u_char);
        tty_putc(tty, '\n' as i32 as u_char);
        current_block = 6993536316001430845;
    } else if cy == thisy {
        /* Move to home position (0, 0). */
        /* Zero on the next line. */
        /* Moving column or row. */
        /*
		 * Moving column only, row staying the same.
		 */
        /* To left edge. */
        if cx == 0 as libc::c_int as libc::c_uint &&
               ((*(*tty).term).flags & 0x4 as libc::c_int == 0 ||
                    (*tty).rleft == 0 as libc::c_int as libc::c_uint) {
            tty_putc(tty, '\r' as i32 as u_char);
            current_block = 6993536316001430845;
        } else if cx == thisx.wrapping_sub(1 as libc::c_int as libc::c_uint)
                      && tty_term_has(term, TTYC_CUB1) != 0 {
            tty_putcode(tty, TTYC_CUB1);
            current_block = 6993536316001430845;
        } else if cx == thisx.wrapping_add(1 as libc::c_int as libc::c_uint)
                      && tty_term_has(term, TTYC_CUF1) != 0 {
            tty_putcode(tty, TTYC_CUF1);
            current_block = 6993536316001430845;
        } else {
            /* One to the left. */
            /* One to the right. */
            /* Calculate difference. */
            change =
                thisx.wrapping_sub(cx) as
                    libc::c_int; /* +ve left, -ve right */
            /*
		 * Use HPA if change is larger than absolute, otherwise move
		 * the cursor with CUB/CUF.
		 */
            if abs(change) as u_int > cx && tty_term_has(term, TTYC_HPA) != 0
               {
                tty_putcode1(tty, TTYC_HPA, cx as libc::c_int);
                current_block = 6993536316001430845;
            } else if change > 0 as libc::c_int &&
                          tty_term_has(term, TTYC_CUB) != 0 &&
                          (*(*tty).term).flags & 0x4 as libc::c_int == 0 {
                if change == 2 as libc::c_int &&
                       tty_term_has(term, TTYC_CUB1) != 0 {
                    tty_putcode(tty, TTYC_CUB1);
                    tty_putcode(tty, TTYC_CUB1);
                } else { tty_putcode1(tty, TTYC_CUB, change); }
                current_block = 6993536316001430845;
            } else if change < 0 as libc::c_int &&
                          tty_term_has(term, TTYC_CUF) != 0 &&
                          (*(*tty).term).flags & 0x4 as libc::c_int == 0 {
                tty_putcode1(tty, TTYC_CUF, -change);
                current_block = 6993536316001430845;
            } else { current_block = 6863629183173758772; }
        }
    } else if cx == thisx {
        /*
		 * Moving row only, column staying the same.
		 */
        /* One above. */
        if thisy != (*tty).rupper &&
               cy == thisy.wrapping_sub(1 as libc::c_int as libc::c_uint) &&
               tty_term_has(term, TTYC_CUU1) != 0 {
            tty_putcode(tty, TTYC_CUU1);
            current_block = 6993536316001430845;
        } else if thisy != (*tty).rlower &&
                      cy ==
                          thisy.wrapping_add(1 as libc::c_int as libc::c_uint)
                      && tty_term_has(term, TTYC_CUD1) != 0 {
            tty_putcode(tty, TTYC_CUD1);
            current_block = 6993536316001430845;
        } else {
            /* One below. */
            /* Calculate difference. */
            change =
                thisy.wrapping_sub(cy) as libc::c_int; /* +ve up, -ve down */
            /*
		 * Try to use VPA if change is larger than absolute or if this
		 * change would cross the scroll region, otherwise use CUU/CUD.
		 */
            if abs(change) as u_int > cy ||
                   change < 0 as libc::c_int &&
                       cy.wrapping_sub(change as libc::c_uint) > (*tty).rlower
                   ||
                   change > 0 as libc::c_int &&
                       cy.wrapping_sub(change as libc::c_uint) < (*tty).rupper
               {
                if tty_term_has(term, TTYC_VPA) != 0 {
                    tty_putcode1(tty, TTYC_VPA, cy as libc::c_int);
                    current_block = 6993536316001430845;
                } else { current_block = 6863629183173758772; }
            } else if change > 0 as libc::c_int &&
                          tty_term_has(term, TTYC_CUU) != 0 {
                tty_putcode1(tty, TTYC_CUU, change);
                current_block = 6993536316001430845;
            } else if change < 0 as libc::c_int &&
                          tty_term_has(term, TTYC_CUD) != 0 {
                tty_putcode1(tty, TTYC_CUD, -change);
                current_block = 6993536316001430845;
            } else { current_block = 6863629183173758772; }
        }
    } else { current_block = 6863629183173758772; }
    match current_block {
        6863629183173758772 => {
            /* Absolute movement. */
            tty_putcode2(tty, TTYC_CUP, cy as libc::c_int, cx as libc::c_int);
        }
        _ => { }
    }
    (*tty).cx = cx;
    (*tty).cy = cy;
}
#[no_mangle]
pub unsafe extern "C" fn tty_attributes(mut tty: *mut tty,
                                        mut gc: *const grid_cell,
                                        mut defaults: *const grid_cell,
                                        mut palette: *mut libc::c_int) {
    let mut tc: *mut grid_cell = &mut (*tty).cell;
    let mut gc2: grid_cell =
        grid_cell{data: utf8_data{data: [0; 21], have: 0, size: 0, width: 0,},
                  attr: 0,
                  flags: 0,
                  fg: 0,
                  bg: 0,
                  us: 0,};
    let mut changed: libc::c_int = 0;
    /* Copy cell and update default colours. */
    memcpy(&mut gc2 as *mut grid_cell as *mut libc::c_void,
           gc as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    if gc2.fg == 8 as libc::c_int { gc2.fg = (*defaults).fg }
    if gc2.bg == 8 as libc::c_int { gc2.bg = (*defaults).bg }
    /* Ignore cell if it is the same as the last one. */
    if gc2.attr as libc::c_int == (*tty).last_cell.attr as libc::c_int &&
           gc2.fg == (*tty).last_cell.fg && gc2.bg == (*tty).last_cell.bg &&
           gc2.us == (*tty).last_cell.us {
        return
    }
    /*
	 * If no setab, try to use the reverse attribute as a best-effort for a
	 * non-default background. This is a bit of a hack but it doesn't do
	 * any serious harm and makes a couple of applications happier.
	 */
    if tty_term_has((*tty).term, TTYC_SETAB) == 0 {
        if gc2.attr as libc::c_int & 0x10 as libc::c_int != 0 {
            if gc2.fg != 7 as libc::c_int &&
                   !(gc2.fg == 8 as libc::c_int || gc2.fg == 9 as libc::c_int)
               {
                gc2.attr =
                    (gc2.attr as libc::c_int & !(0x10 as libc::c_int)) as
                        u_short
            }
        } else if gc2.bg != 0 as libc::c_int &&
                      !(gc2.bg == 8 as libc::c_int ||
                            gc2.bg == 9 as libc::c_int) {
            gc2.attr =
                (gc2.attr as libc::c_int | 0x10 as libc::c_int) as u_short
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
    if (*tc).attr as libc::c_int & !(gc2.attr as libc::c_int) != 0 ||
           (*tc).us != gc2.us && gc2.us == 0 as libc::c_int {
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
    if changed & 0x1 as libc::c_int != 0 { tty_putcode(tty, TTYC_BOLD); }
    if changed & 0x2 as libc::c_int != 0 { tty_putcode(tty, TTYC_DIM); }
    if changed & 0x40 as libc::c_int != 0 { tty_set_italics(tty); }
    if changed &
           (0x4 as libc::c_int | 0x200 as libc::c_int | 0x400 as libc::c_int |
                0x800 as libc::c_int | 0x1000 as libc::c_int) != 0 {
        if changed & 0x4 as libc::c_int != 0 ||
               tty_term_has((*tty).term, TTYC_SMULX) == 0 {
            tty_putcode(tty, TTYC_SMUL);
        } else if changed & 0x200 as libc::c_int != 0 {
            tty_putcode1(tty, TTYC_SMULX, 2 as libc::c_int);
        } else if changed & 0x400 as libc::c_int != 0 {
            tty_putcode1(tty, TTYC_SMULX, 3 as libc::c_int);
        } else if changed & 0x800 as libc::c_int != 0 {
            tty_putcode1(tty, TTYC_SMULX, 4 as libc::c_int);
        } else if changed & 0x1000 as libc::c_int != 0 {
            tty_putcode1(tty, TTYC_SMULX, 5 as libc::c_int);
        }
    }
    if changed & 0x8 as libc::c_int != 0 { tty_putcode(tty, TTYC_BLINK); }
    if changed & 0x10 as libc::c_int != 0 {
        if tty_term_has((*tty).term, TTYC_REV) != 0 {
            tty_putcode(tty, TTYC_REV);
        } else if tty_term_has((*tty).term, TTYC_SMSO) != 0 {
            tty_putcode(tty, TTYC_SMSO);
        }
    }
    if changed & 0x20 as libc::c_int != 0 { tty_putcode(tty, TTYC_INVIS); }
    if changed & 0x100 as libc::c_int != 0 { tty_putcode(tty, TTYC_SMXX); }
    if changed & 0x2000 as libc::c_int != 0 { tty_putcode(tty, TTYC_SMOL); }
    if changed & 0x80 as libc::c_int != 0 && tty_acs_needed(tty) != 0 {
        tty_putcode(tty, TTYC_SMACS);
    }
    memcpy(&mut (*tty).last_cell as *mut grid_cell as *mut libc::c_void,
           &mut gc2 as *mut grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
}
unsafe extern "C" fn tty_colours(mut tty: *mut tty,
                                 mut gc: *const grid_cell) {
    let mut tc: *mut grid_cell = &mut (*tty).cell;
    let mut have_ax: libc::c_int = 0;
    /* No changes? Nothing is necessary. */
    if (*gc).fg == (*tc).fg && (*gc).bg == (*tc).bg && (*gc).us == (*tc).us {
        return
    }
    /*
	 * Is either the default colour? This is handled specially because the
	 * best solution might be to reset both colours to default, in which
	 * case if only one is default need to fall onward to set the other
	 * colour.
	 */
    if (*gc).fg == 8 as libc::c_int || (*gc).fg == 9 as libc::c_int ||
           ((*gc).bg == 8 as libc::c_int || (*gc).bg == 9 as libc::c_int) {
        /*
		 * If don't have AX but do have op, send sgr0 (op can't
		 * actually be used because it is sometimes the same as sgr0
		 * and sometimes isn't). This resets both colours to default.
		 *
		 * Otherwise, try to set the default colour only as needed.
		 */
        have_ax = tty_term_flag((*tty).term, TTYC_AX);
        if have_ax == 0 && tty_term_has((*tty).term, TTYC_OP) != 0 {
            tty_reset(tty);
        } else {
            if ((*gc).fg == 8 as libc::c_int || (*gc).fg == 9 as libc::c_int)
                   &&
                   !((*tc).fg == 8 as libc::c_int ||
                         (*tc).fg == 9 as libc::c_int) {
                if have_ax != 0 {
                    tty_puts(tty,
                             b"\x1b[39m\x00" as *const u8 as
                                 *const libc::c_char);
                } else if (*tc).fg != 7 as libc::c_int {
                    tty_putcode1(tty, TTYC_SETAF, 7 as libc::c_int);
                }
                (*tc).fg = (*gc).fg
            }
            if ((*gc).bg == 8 as libc::c_int || (*gc).bg == 9 as libc::c_int)
                   &&
                   !((*tc).bg == 8 as libc::c_int ||
                         (*tc).bg == 9 as libc::c_int) {
                if have_ax != 0 {
                    tty_puts(tty,
                             b"\x1b[49m\x00" as *const u8 as
                                 *const libc::c_char);
                } else if (*tc).bg != 0 as libc::c_int {
                    tty_putcode1(tty, TTYC_SETAB, 0 as libc::c_int);
                }
                (*tc).bg = (*gc).bg
            }
        }
    }
    /* Set the foreground colour. */
    if !((*gc).fg == 8 as libc::c_int || (*gc).fg == 9 as libc::c_int) &&
           (*gc).fg != (*tc).fg {
        tty_colours_fg(tty, gc);
    }
    /*
	 * Set the background colour. This must come after the foreground as
	 * tty_colour_fg() can call tty_reset().
	 */
    if !((*gc).bg == 8 as libc::c_int || (*gc).bg == 9 as libc::c_int) &&
           (*gc).bg != (*tc).bg {
        tty_colours_bg(tty, gc);
    }
    /* Set the underscore color. */
    if (*gc).us != (*tc).us { tty_colours_us(tty, gc); };
}
unsafe extern "C" fn tty_check_fg(mut tty: *mut tty,
                                  mut palette: *mut libc::c_int,
                                  mut gc: *mut grid_cell) {
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
    if !((*gc).flags as libc::c_int) & 0x20 as libc::c_int != 0 {
        c = (*gc).fg;
        if c < 8 as libc::c_int &&
               (*gc).attr as libc::c_int & 0x1 as libc::c_int != 0 {
            c += 90 as libc::c_int
        }
        c = tty_get_palette(palette, c);
        if c != -(1 as libc::c_int) { (*gc).fg = c }
    }
    /* Is this a 24-bit colour? */
    if (*gc).fg & 0x2000000 as libc::c_int != 0 {
        /* Not a 24-bit terminal? Translate to 256-colour palette. */
        if (*(*tty).term).flags & 0x10 as libc::c_int != 0 { return }
        colour_split_rgb((*gc).fg, &mut r, &mut g, &mut b);
        (*gc).fg = colour_find_rgb(r, g, b)
    }
    /* How many colours does this terminal have? */
    if (*(*tty).term).flags & 0x1 as libc::c_int != 0 {
        colours = 256 as libc::c_int as u_int
    } else { colours = tty_term_number((*tty).term, TTYC_COLORS) as u_int }
    /* Is this a 256-colour colour? */
    if (*gc).fg & 0x1000000 as libc::c_int != 0 {
        /* And not a 256 colour mode? */
        if colours != 256 as libc::c_int as libc::c_uint {
            (*gc).fg = colour_256to16((*gc).fg);
            if (*gc).fg & 8 as libc::c_int != 0 {
                (*gc).fg &= 7 as libc::c_int;
                if colours >= 16 as libc::c_int as libc::c_uint {
                    (*gc).fg += 90 as libc::c_int
                }
            }
        }
        return
    }
    /* Is this an aixterm colour? */
    if (*gc).fg >= 90 as libc::c_int && (*gc).fg <= 97 as libc::c_int &&
           colours < 16 as libc::c_int as libc::c_uint {
        (*gc).fg -= 90 as libc::c_int;
        (*gc).attr =
            ((*gc).attr as libc::c_int | 0x1 as libc::c_int) as u_short
    };
}
unsafe extern "C" fn tty_check_bg(mut tty: *mut tty,
                                  mut palette: *mut libc::c_int,
                                  mut gc: *mut grid_cell) {
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    let mut colours: u_int = 0;
    let mut c: libc::c_int = 0;
    /* Perform substitution if this pane has a palette. */
    if !((*gc).flags as libc::c_int) & 0x20 as libc::c_int != 0 {
        c = tty_get_palette(palette, (*gc).bg);
        if c != -(1 as libc::c_int) { (*gc).bg = c }
    }
    /* Is this a 24-bit colour? */
    if (*gc).bg & 0x2000000 as libc::c_int != 0 {
        /* Not a 24-bit terminal? Translate to 256-colour palette. */
        if (*(*tty).term).flags & 0x10 as libc::c_int != 0 { return }
        colour_split_rgb((*gc).bg, &mut r, &mut g, &mut b);
        (*gc).bg = colour_find_rgb(r, g, b)
    }
    /* How many colours does this terminal have? */
    if (*(*tty).term).flags & 0x1 as libc::c_int != 0 {
        colours = 256 as libc::c_int as u_int
    } else { colours = tty_term_number((*tty).term, TTYC_COLORS) as u_int }
    /* Is this a 256-colour colour? */
    if (*gc).bg & 0x1000000 as libc::c_int != 0 {
        /*
		 * And not a 256 colour mode? Translate to 16-colour
		 * palette. Bold background doesn't exist portably, so just
		 * discard the bold bit if set.
		 */
        if colours != 256 as libc::c_int as libc::c_uint {
            (*gc).bg = colour_256to16((*gc).bg);
            if (*gc).bg & 8 as libc::c_int != 0 {
                (*gc).bg &= 7 as libc::c_int;
                if colours >= 16 as libc::c_int as libc::c_uint {
                    (*gc).bg += 90 as libc::c_int
                }
            }
        }
        return
    }
    /* Is this an aixterm colour? */
    if (*gc).bg >= 90 as libc::c_int && (*gc).bg <= 97 as libc::c_int &&
           colours < 16 as libc::c_int as libc::c_uint {
        (*gc).bg -= 90 as libc::c_int
    };
}
unsafe extern "C" fn tty_check_us(mut tty: *mut tty,
                                  mut palette: *mut libc::c_int,
                                  mut gc: *mut grid_cell) {
    let mut c: libc::c_int = 0;
    /* Perform substitution if this pane has a palette. */
    if !((*gc).flags as libc::c_int) & 0x20 as libc::c_int != 0 {
        c = tty_get_palette(palette, (*gc).us);
        if c != -(1 as libc::c_int) { (*gc).us = c }
    }
    /* Underscore colour is set as RGB so convert a 256 colour to RGB. */
    if (*gc).us & 0x1000000 as libc::c_int != 0 {
        (*gc).us = colour_256toRGB((*gc).us)
    };
}
unsafe extern "C" fn tty_colours_fg(mut tty: *mut tty,
                                    mut gc: *const grid_cell) {
    let mut tc: *mut grid_cell = &mut (*tty).cell;
    let mut s: [libc::c_char; 32] = [0; 32];
    /* Is this a 24-bit or 256-colour colour? */
    if (*gc).fg & 0x2000000 as libc::c_int != 0 ||
           (*gc).fg & 0x1000000 as libc::c_int != 0 {
        if !(tty_try_colour(tty, (*gc).fg,
                            b"38\x00" as *const u8 as *const libc::c_char) ==
                 0 as libc::c_int) {
            /* Should not get here, already converted in tty_check_fg. */
            return
        }
    } else if (*gc).fg >= 90 as libc::c_int && (*gc).fg <= 97 as libc::c_int {
        if (*(*tty).term).flags & 0x1 as libc::c_int != 0 {
            xsnprintf(s.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 32]>() as
                          libc::c_ulong,
                      b"\x1b[%dm\x00" as *const u8 as *const libc::c_char,
                      (*gc).fg);
            tty_puts(tty, s.as_mut_ptr());
        } else {
            tty_putcode1(tty, TTYC_SETAF,
                         (*gc).fg - 90 as libc::c_int + 8 as libc::c_int);
        }
    } else {
        /* Is this an aixterm bright colour? */
        /* Otherwise set the foreground colour. */
        tty_putcode1(tty, TTYC_SETAF, (*gc).fg);
    }
    /* Save the new values in the terminal current cell. */
    (*tc).fg = (*gc).fg;
}
unsafe extern "C" fn tty_colours_bg(mut tty: *mut tty,
                                    mut gc: *const grid_cell) {
    let mut tc: *mut grid_cell = &mut (*tty).cell;
    let mut s: [libc::c_char; 32] = [0; 32];
    /* Is this a 24-bit or 256-colour colour? */
    if (*gc).bg & 0x2000000 as libc::c_int != 0 ||
           (*gc).bg & 0x1000000 as libc::c_int != 0 {
        if !(tty_try_colour(tty, (*gc).bg,
                            b"48\x00" as *const u8 as *const libc::c_char) ==
                 0 as libc::c_int) {
            /* Should not get here, already converted in tty_check_bg. */
            return
        }
    } else if (*gc).bg >= 90 as libc::c_int && (*gc).bg <= 97 as libc::c_int {
        if (*(*tty).term).flags & 0x1 as libc::c_int != 0 {
            xsnprintf(s.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 32]>() as
                          libc::c_ulong,
                      b"\x1b[%dm\x00" as *const u8 as *const libc::c_char,
                      (*gc).bg + 10 as libc::c_int);
            tty_puts(tty, s.as_mut_ptr());
        } else {
            tty_putcode1(tty, TTYC_SETAB,
                         (*gc).bg - 90 as libc::c_int + 8 as libc::c_int);
        }
    } else {
        /* Is this an aixterm bright colour? */
        /* Otherwise set the background colour. */
        tty_putcode1(tty, TTYC_SETAB, (*gc).bg);
    }
    /* Save the new values in the terminal current cell. */
    (*tc).bg = (*gc).bg;
}
unsafe extern "C" fn tty_colours_us(mut tty: *mut tty,
                                    mut gc: *const grid_cell) {
    let mut tc: *mut grid_cell = &mut (*tty).cell;
    let mut c: u_int = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    /* Clear underline colour. */
    if (*gc).us == 0 as libc::c_int {
        tty_putcode(tty, TTYC_OL);
    } else {
        /* Must be an RGB colour - this should never happen. */
        if !(*gc).us & 0x2000000 as libc::c_int != 0 { return }
        /*
	 * Setulc and setal follows the ncurses(3) one argument "direct colour"
	 * capability format. Calculate the colour value.
	 */
        colour_split_rgb((*gc).us, &mut r, &mut g, &mut b);
        c =
            (65536 as libc::c_int * r as libc::c_int +
                 256 as libc::c_int * g as libc::c_int + b as libc::c_int) as
                u_int;
        /*
	 * Write the colour. Only use setal if the RGB flag is set because the
	 * non-RGB version may be wrong.
	 */
        if tty_term_has((*tty).term, TTYC_SETULC) != 0 {
            tty_putcode1(tty, TTYC_SETULC, c as libc::c_int);
        } else if tty_term_has((*tty).term, TTYC_SETAL) != 0 &&
                      tty_term_has((*tty).term, TTYC_RGB) != 0 {
            tty_putcode1(tty, TTYC_SETAL, c as libc::c_int);
        }
    }
    /* Save the new values in the terminal current cell. */
    (*tc).us = (*gc).us;
}
unsafe extern "C" fn tty_try_colour(mut tty: *mut tty,
                                    mut colour: libc::c_int,
                                    mut type_0: *const libc::c_char)
 -> libc::c_int {
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    if colour & 0x1000000 as libc::c_int != 0 {
        if *type_0 as libc::c_int == '3' as i32 &&
               tty_term_has((*tty).term, TTYC_SETAF) != 0 {
            tty_putcode1(tty, TTYC_SETAF, colour & 0xff as libc::c_int);
        } else if tty_term_has((*tty).term, TTYC_SETAB) != 0 {
            tty_putcode1(tty, TTYC_SETAB, colour & 0xff as libc::c_int);
        }
        return 0 as libc::c_int
    }
    if colour & 0x2000000 as libc::c_int != 0 {
        colour_split_rgb(colour & 0xffffff as libc::c_int, &mut r, &mut g,
                         &mut b);
        if *type_0 as libc::c_int == '3' as i32 &&
               tty_term_has((*tty).term, TTYC_SETRGBF) != 0 {
            tty_putcode3(tty, TTYC_SETRGBF, r as libc::c_int,
                         g as libc::c_int, b as libc::c_int);
        } else if tty_term_has((*tty).term, TTYC_SETRGBB) != 0 {
            tty_putcode3(tty, TTYC_SETRGBB, r as libc::c_int,
                         g as libc::c_int, b as libc::c_int);
        }
        return 0 as libc::c_int
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn tty_window_default_style(mut gc: *mut grid_cell,
                                              mut wp: *mut window_pane) {
    memcpy(gc as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    (*gc).fg = (*wp).fg;
    (*gc).bg = (*wp).bg;
}
#[no_mangle]
pub unsafe extern "C" fn tty_default_colours(mut gc: *mut grid_cell,
                                             mut wp: *mut window_pane) {
    let mut oo: *mut options = (*wp).options;
    memcpy(gc as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    if (*wp).flags & 0x1000 as libc::c_int != 0 {
        (*wp).flags &= !(0x1000 as libc::c_int);
        tty_window_default_style(&mut (*wp).cached_active_gc, wp);
        style_add(&mut (*wp).cached_active_gc, oo,
                  b"window-active-style\x00" as *const u8 as
                      *const libc::c_char, 0 as *mut format_tree);
        tty_window_default_style(&mut (*wp).cached_gc, wp);
        style_add(&mut (*wp).cached_gc, oo,
                  b"window-style\x00" as *const u8 as *const libc::c_char,
                  0 as *mut format_tree);
    }
    if (*gc).fg == 8 as libc::c_int {
        if wp == (*(*wp).window).active &&
               (*wp).cached_active_gc.fg != 8 as libc::c_int {
            (*gc).fg = (*wp).cached_active_gc.fg
        } else { (*gc).fg = (*wp).cached_gc.fg }
    }
    if (*gc).bg == 8 as libc::c_int {
        if wp == (*(*wp).window).active &&
               (*wp).cached_active_gc.bg != 8 as libc::c_int {
            (*gc).bg = (*wp).cached_active_gc.bg
        } else { (*gc).bg = (*wp).cached_gc.bg }
    };
}
unsafe extern "C" fn tty_default_attributes(mut tty: *mut tty,
                                            mut defaults: *const grid_cell,
                                            mut palette: *mut libc::c_int,
                                            mut bg: u_int) {
    let mut gc: grid_cell =
        grid_cell{data: utf8_data{data: [0; 21], have: 0, size: 0, width: 0,},
                  attr: 0,
                  flags: 0,
                  fg: 0,
                  bg: 0,
                  us: 0,};
    memcpy(&mut gc as *mut grid_cell as *mut libc::c_void,
           &grid_default_cell as *const grid_cell as *const libc::c_void,
           ::std::mem::size_of::<grid_cell>() as libc::c_ulong);
    gc.bg = bg as libc::c_int;
    tty_attributes(tty, &mut gc, defaults, palette);
}
