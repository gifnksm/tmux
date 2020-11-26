use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type screen_write_collect_line;
    pub type screen_sel;
    pub type screen_titles;
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
    pub type options_array_item;
    pub type options_entry;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn __b64_pton(_: *const libc::c_char, _: *mut libc::c_uchar, _: size_t)
     -> libc::c_int;
    #[no_mangle]
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char,
               _: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_pending(ev: *const event, events: libc::c_short,
                     tv: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn event_initialized(ev: *const event) -> libc::c_int;
    #[no_mangle]
    fn event_set(_: *mut event, _: libc::c_int, _: libc::c_short,
                 _:
                     Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: libc::c_short,
                                                 _: *mut libc::c_void) -> ()>,
                 _: *mut libc::c_void);
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t)
     -> *mut libc::c_uchar;
    #[no_mangle]
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> libc::c_ulong;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    static mut global_options: *mut options;
    #[no_mangle]
    fn paste_add(_: *const libc::c_char, _: *mut libc::c_char, _: size_t);
    #[no_mangle]
    fn tty_update_features(_: *mut tty);
    #[no_mangle]
    fn tty_term_string(_: *mut tty_term, _: tty_code_code)
     -> *const libc::c_char;
    #[no_mangle]
    fn tty_add_features(_: *mut libc::c_int, _: *const libc::c_char,
                        _: *const libc::c_char);
    #[no_mangle]
    fn tty_default_features(_: *mut libc::c_int, _: *const libc::c_char,
                            _: u_int);
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn options_array_item_value(_: *mut options_array_item)
     -> *mut options_value;
    #[no_mangle]
    fn server_client_handle_key(_: *mut client, _: *mut key_event)
     -> libc::c_int;
    #[no_mangle]
    fn options_array_item_index(_: *mut options_array_item) -> u_int;
    #[no_mangle]
    fn options_array_next(_: *mut options_array_item)
     -> *mut options_array_item;
    #[no_mangle]
    fn options_array_first(_: *mut options_entry) -> *mut options_array_item;
    #[no_mangle]
    fn options_get(_: *mut options, _: *const libc::c_char)
     -> *mut options_entry;
    #[no_mangle]
    fn utf8_from_data(_: *const utf8_data, _: *mut utf8_char) -> utf8_state;
    #[no_mangle]
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn log_get_level() -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
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
    pub ev_timeout_pos: C2RustUnnamed_5,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_0,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ev_io: C2RustUnnamed_3,
    pub ev_signal: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ev_signal_next: C2RustUnnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ev_io_next: C2RustUnnamed_4,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ev_next_with_common_timeout: C2RustUnnamed_6,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_8,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_7,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
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
pub type bitstr_t = libc::c_uchar;
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
    pub exit_type: C2RustUnnamed_29,
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
    pub prompt_mode: C2RustUnnamed_26,
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
    pub entry: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
    pub entry: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
    pub c2rust_unnamed: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub offset: u_int,
    pub data: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
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
    pub gentry: C2RustUnnamed_14,
    pub entry: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
    pub entry: C2RustUnnamed_17,
    pub wentry: C2RustUnnamed_16,
    pub sentry: C2RustUnnamed_15,
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
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
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
    pub alerts_entry: C2RustUnnamed_20,
    pub options: *mut options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_19,
    pub entry: C2RustUnnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
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
    pub entry: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
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
    pub modes: C2RustUnnamed_24,
    pub searchstr: *mut libc::c_char,
    pub searchregex: libc::c_int,
    pub written: size_t,
    pub skipped: size_t,
    pub border_gc_set: libc::c_int,
    pub border_gc: grid_cell,
    pub entry: C2RustUnnamed_23,
    pub tree_entry: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
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
    pub entry: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
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
pub type C2RustUnnamed_26 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_26 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_26 = 0;
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
    pub entry: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
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
    pub entry: C2RustUnnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
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
pub type C2RustUnnamed_29 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_29 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_29 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_29 = 0;
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
    pub entry: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
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
    pub entry: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
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
    pub entry: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub rbe_left: *mut client_window,
    pub rbe_right: *mut client_window,
    pub rbe_parent: *mut client_window,
    pub rbe_color: libc::c_int,
}
pub type C2RustUnnamed_33 = libc::c_ulong;
pub const KEYC_KP_PERIOD: C2RustUnnamed_33 = 68719476927;
pub const KEYC_KP_ZERO: C2RustUnnamed_33 = 68719476926;
pub const KEYC_KP_ENTER: C2RustUnnamed_33 = 68719476925;
pub const KEYC_KP_THREE: C2RustUnnamed_33 = 68719476924;
pub const KEYC_KP_TWO: C2RustUnnamed_33 = 68719476923;
pub const KEYC_KP_ONE: C2RustUnnamed_33 = 68719476922;
pub const KEYC_KP_SIX: C2RustUnnamed_33 = 68719476921;
pub const KEYC_KP_FIVE: C2RustUnnamed_33 = 68719476920;
pub const KEYC_KP_FOUR: C2RustUnnamed_33 = 68719476919;
pub const KEYC_KP_PLUS: C2RustUnnamed_33 = 68719476918;
pub const KEYC_KP_NINE: C2RustUnnamed_33 = 68719476917;
pub const KEYC_KP_EIGHT: C2RustUnnamed_33 = 68719476916;
pub const KEYC_KP_SEVEN: C2RustUnnamed_33 = 68719476915;
pub const KEYC_KP_MINUS: C2RustUnnamed_33 = 68719476914;
pub const KEYC_KP_STAR: C2RustUnnamed_33 = 68719476913;
pub const KEYC_KP_SLASH: C2RustUnnamed_33 = 68719476912;
pub const KEYC_RIGHT: C2RustUnnamed_33 = 68719476911;
pub const KEYC_LEFT: C2RustUnnamed_33 = 68719476910;
pub const KEYC_DOWN: C2RustUnnamed_33 = 68719476909;
pub const KEYC_UP: C2RustUnnamed_33 = 68719476908;
pub const KEYC_BTAB: C2RustUnnamed_33 = 68719476907;
pub const KEYC_PPAGE: C2RustUnnamed_33 = 68719476906;
pub const KEYC_NPAGE: C2RustUnnamed_33 = 68719476905;
pub const KEYC_END: C2RustUnnamed_33 = 68719476904;
pub const KEYC_HOME: C2RustUnnamed_33 = 68719476903;
pub const KEYC_DC: C2RustUnnamed_33 = 68719476902;
pub const KEYC_IC: C2RustUnnamed_33 = 68719476901;
pub const KEYC_F12: C2RustUnnamed_33 = 68719476900;
pub const KEYC_F11: C2RustUnnamed_33 = 68719476899;
pub const KEYC_F10: C2RustUnnamed_33 = 68719476898;
pub const KEYC_F9: C2RustUnnamed_33 = 68719476897;
pub const KEYC_F8: C2RustUnnamed_33 = 68719476896;
pub const KEYC_F7: C2RustUnnamed_33 = 68719476895;
pub const KEYC_F6: C2RustUnnamed_33 = 68719476894;
pub const KEYC_F5: C2RustUnnamed_33 = 68719476893;
pub const KEYC_F4: C2RustUnnamed_33 = 68719476892;
pub const KEYC_F3: C2RustUnnamed_33 = 68719476891;
pub const KEYC_F2: C2RustUnnamed_33 = 68719476890;
pub const KEYC_F1: C2RustUnnamed_33 = 68719476889;
pub const KEYC_BSPACE: C2RustUnnamed_33 = 68719476888;
pub const KEYC_TRIPLECLICK3_BORDER: C2RustUnnamed_33 = 68719476887;
pub const KEYC_TRIPLECLICK3_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476886;
pub const KEYC_TRIPLECLICK3_STATUS_RIGHT: C2RustUnnamed_33 = 68719476885;
pub const KEYC_TRIPLECLICK3_STATUS_LEFT: C2RustUnnamed_33 = 68719476884;
pub const KEYC_TRIPLECLICK3_STATUS: C2RustUnnamed_33 = 68719476883;
pub const KEYC_TRIPLECLICK3_PANE: C2RustUnnamed_33 = 68719476882;
pub const KEYC_TRIPLECLICK2_BORDER: C2RustUnnamed_33 = 68719476881;
pub const KEYC_TRIPLECLICK2_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476880;
pub const KEYC_TRIPLECLICK2_STATUS_RIGHT: C2RustUnnamed_33 = 68719476879;
pub const KEYC_TRIPLECLICK2_STATUS_LEFT: C2RustUnnamed_33 = 68719476878;
pub const KEYC_TRIPLECLICK2_STATUS: C2RustUnnamed_33 = 68719476877;
pub const KEYC_TRIPLECLICK2_PANE: C2RustUnnamed_33 = 68719476876;
pub const KEYC_TRIPLECLICK1_BORDER: C2RustUnnamed_33 = 68719476875;
pub const KEYC_TRIPLECLICK1_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476874;
pub const KEYC_TRIPLECLICK1_STATUS_RIGHT: C2RustUnnamed_33 = 68719476873;
pub const KEYC_TRIPLECLICK1_STATUS_LEFT: C2RustUnnamed_33 = 68719476872;
pub const KEYC_TRIPLECLICK1_STATUS: C2RustUnnamed_33 = 68719476871;
pub const KEYC_TRIPLECLICK1_PANE: C2RustUnnamed_33 = 68719476870;
pub const KEYC_DOUBLECLICK3_BORDER: C2RustUnnamed_33 = 68719476869;
pub const KEYC_DOUBLECLICK3_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476868;
pub const KEYC_DOUBLECLICK3_STATUS_RIGHT: C2RustUnnamed_33 = 68719476867;
pub const KEYC_DOUBLECLICK3_STATUS_LEFT: C2RustUnnamed_33 = 68719476866;
pub const KEYC_DOUBLECLICK3_STATUS: C2RustUnnamed_33 = 68719476865;
pub const KEYC_DOUBLECLICK3_PANE: C2RustUnnamed_33 = 68719476864;
pub const KEYC_DOUBLECLICK2_BORDER: C2RustUnnamed_33 = 68719476863;
pub const KEYC_DOUBLECLICK2_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476862;
pub const KEYC_DOUBLECLICK2_STATUS_RIGHT: C2RustUnnamed_33 = 68719476861;
pub const KEYC_DOUBLECLICK2_STATUS_LEFT: C2RustUnnamed_33 = 68719476860;
pub const KEYC_DOUBLECLICK2_STATUS: C2RustUnnamed_33 = 68719476859;
pub const KEYC_DOUBLECLICK2_PANE: C2RustUnnamed_33 = 68719476858;
pub const KEYC_DOUBLECLICK1_BORDER: C2RustUnnamed_33 = 68719476857;
pub const KEYC_DOUBLECLICK1_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476856;
pub const KEYC_DOUBLECLICK1_STATUS_RIGHT: C2RustUnnamed_33 = 68719476855;
pub const KEYC_DOUBLECLICK1_STATUS_LEFT: C2RustUnnamed_33 = 68719476854;
pub const KEYC_DOUBLECLICK1_STATUS: C2RustUnnamed_33 = 68719476853;
pub const KEYC_DOUBLECLICK1_PANE: C2RustUnnamed_33 = 68719476852;
pub const KEYC_SECONDCLICK3_BORDER: C2RustUnnamed_33 = 68719476851;
pub const KEYC_SECONDCLICK3_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476850;
pub const KEYC_SECONDCLICK3_STATUS_RIGHT: C2RustUnnamed_33 = 68719476849;
pub const KEYC_SECONDCLICK3_STATUS_LEFT: C2RustUnnamed_33 = 68719476848;
pub const KEYC_SECONDCLICK3_STATUS: C2RustUnnamed_33 = 68719476847;
pub const KEYC_SECONDCLICK3_PANE: C2RustUnnamed_33 = 68719476846;
pub const KEYC_SECONDCLICK2_BORDER: C2RustUnnamed_33 = 68719476845;
pub const KEYC_SECONDCLICK2_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476844;
pub const KEYC_SECONDCLICK2_STATUS_RIGHT: C2RustUnnamed_33 = 68719476843;
pub const KEYC_SECONDCLICK2_STATUS_LEFT: C2RustUnnamed_33 = 68719476842;
pub const KEYC_SECONDCLICK2_STATUS: C2RustUnnamed_33 = 68719476841;
pub const KEYC_SECONDCLICK2_PANE: C2RustUnnamed_33 = 68719476840;
pub const KEYC_SECONDCLICK1_BORDER: C2RustUnnamed_33 = 68719476839;
pub const KEYC_SECONDCLICK1_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476838;
pub const KEYC_SECONDCLICK1_STATUS_RIGHT: C2RustUnnamed_33 = 68719476837;
pub const KEYC_SECONDCLICK1_STATUS_LEFT: C2RustUnnamed_33 = 68719476836;
pub const KEYC_SECONDCLICK1_STATUS: C2RustUnnamed_33 = 68719476835;
pub const KEYC_SECONDCLICK1_PANE: C2RustUnnamed_33 = 68719476834;
pub const KEYC_WHEELDOWN_BORDER: C2RustUnnamed_33 = 68719476833;
pub const KEYC_WHEELDOWN_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476832;
pub const KEYC_WHEELDOWN_STATUS_RIGHT: C2RustUnnamed_33 = 68719476831;
pub const KEYC_WHEELDOWN_STATUS_LEFT: C2RustUnnamed_33 = 68719476830;
pub const KEYC_WHEELDOWN_STATUS: C2RustUnnamed_33 = 68719476829;
pub const KEYC_WHEELDOWN_PANE: C2RustUnnamed_33 = 68719476828;
pub const KEYC_WHEELUP_BORDER: C2RustUnnamed_33 = 68719476827;
pub const KEYC_WHEELUP_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476826;
pub const KEYC_WHEELUP_STATUS_RIGHT: C2RustUnnamed_33 = 68719476825;
pub const KEYC_WHEELUP_STATUS_LEFT: C2RustUnnamed_33 = 68719476824;
pub const KEYC_WHEELUP_STATUS: C2RustUnnamed_33 = 68719476823;
pub const KEYC_WHEELUP_PANE: C2RustUnnamed_33 = 68719476822;
pub const KEYC_MOUSEDRAGEND3_BORDER: C2RustUnnamed_33 = 68719476821;
pub const KEYC_MOUSEDRAGEND3_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476820;
pub const KEYC_MOUSEDRAGEND3_STATUS_RIGHT: C2RustUnnamed_33 = 68719476819;
pub const KEYC_MOUSEDRAGEND3_STATUS_LEFT: C2RustUnnamed_33 = 68719476818;
pub const KEYC_MOUSEDRAGEND3_STATUS: C2RustUnnamed_33 = 68719476817;
pub const KEYC_MOUSEDRAGEND3_PANE: C2RustUnnamed_33 = 68719476816;
pub const KEYC_MOUSEDRAGEND2_BORDER: C2RustUnnamed_33 = 68719476815;
pub const KEYC_MOUSEDRAGEND2_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476814;
pub const KEYC_MOUSEDRAGEND2_STATUS_RIGHT: C2RustUnnamed_33 = 68719476813;
pub const KEYC_MOUSEDRAGEND2_STATUS_LEFT: C2RustUnnamed_33 = 68719476812;
pub const KEYC_MOUSEDRAGEND2_STATUS: C2RustUnnamed_33 = 68719476811;
pub const KEYC_MOUSEDRAGEND2_PANE: C2RustUnnamed_33 = 68719476810;
pub const KEYC_MOUSEDRAGEND1_BORDER: C2RustUnnamed_33 = 68719476809;
pub const KEYC_MOUSEDRAGEND1_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476808;
pub const KEYC_MOUSEDRAGEND1_STATUS_RIGHT: C2RustUnnamed_33 = 68719476807;
pub const KEYC_MOUSEDRAGEND1_STATUS_LEFT: C2RustUnnamed_33 = 68719476806;
pub const KEYC_MOUSEDRAGEND1_STATUS: C2RustUnnamed_33 = 68719476805;
pub const KEYC_MOUSEDRAGEND1_PANE: C2RustUnnamed_33 = 68719476804;
pub const KEYC_MOUSEDRAG3_BORDER: C2RustUnnamed_33 = 68719476803;
pub const KEYC_MOUSEDRAG3_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476802;
pub const KEYC_MOUSEDRAG3_STATUS_RIGHT: C2RustUnnamed_33 = 68719476801;
pub const KEYC_MOUSEDRAG3_STATUS_LEFT: C2RustUnnamed_33 = 68719476800;
pub const KEYC_MOUSEDRAG3_STATUS: C2RustUnnamed_33 = 68719476799;
pub const KEYC_MOUSEDRAG3_PANE: C2RustUnnamed_33 = 68719476798;
pub const KEYC_MOUSEDRAG2_BORDER: C2RustUnnamed_33 = 68719476797;
pub const KEYC_MOUSEDRAG2_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476796;
pub const KEYC_MOUSEDRAG2_STATUS_RIGHT: C2RustUnnamed_33 = 68719476795;
pub const KEYC_MOUSEDRAG2_STATUS_LEFT: C2RustUnnamed_33 = 68719476794;
pub const KEYC_MOUSEDRAG2_STATUS: C2RustUnnamed_33 = 68719476793;
pub const KEYC_MOUSEDRAG2_PANE: C2RustUnnamed_33 = 68719476792;
pub const KEYC_MOUSEDRAG1_BORDER: C2RustUnnamed_33 = 68719476791;
pub const KEYC_MOUSEDRAG1_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476790;
pub const KEYC_MOUSEDRAG1_STATUS_RIGHT: C2RustUnnamed_33 = 68719476789;
pub const KEYC_MOUSEDRAG1_STATUS_LEFT: C2RustUnnamed_33 = 68719476788;
pub const KEYC_MOUSEDRAG1_STATUS: C2RustUnnamed_33 = 68719476787;
pub const KEYC_MOUSEDRAG1_PANE: C2RustUnnamed_33 = 68719476786;
pub const KEYC_MOUSEUP3_BORDER: C2RustUnnamed_33 = 68719476785;
pub const KEYC_MOUSEUP3_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476784;
pub const KEYC_MOUSEUP3_STATUS_RIGHT: C2RustUnnamed_33 = 68719476783;
pub const KEYC_MOUSEUP3_STATUS_LEFT: C2RustUnnamed_33 = 68719476782;
pub const KEYC_MOUSEUP3_STATUS: C2RustUnnamed_33 = 68719476781;
pub const KEYC_MOUSEUP3_PANE: C2RustUnnamed_33 = 68719476780;
pub const KEYC_MOUSEUP2_BORDER: C2RustUnnamed_33 = 68719476779;
pub const KEYC_MOUSEUP2_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476778;
pub const KEYC_MOUSEUP2_STATUS_RIGHT: C2RustUnnamed_33 = 68719476777;
pub const KEYC_MOUSEUP2_STATUS_LEFT: C2RustUnnamed_33 = 68719476776;
pub const KEYC_MOUSEUP2_STATUS: C2RustUnnamed_33 = 68719476775;
pub const KEYC_MOUSEUP2_PANE: C2RustUnnamed_33 = 68719476774;
pub const KEYC_MOUSEUP1_BORDER: C2RustUnnamed_33 = 68719476773;
pub const KEYC_MOUSEUP1_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476772;
pub const KEYC_MOUSEUP1_STATUS_RIGHT: C2RustUnnamed_33 = 68719476771;
pub const KEYC_MOUSEUP1_STATUS_LEFT: C2RustUnnamed_33 = 68719476770;
pub const KEYC_MOUSEUP1_STATUS: C2RustUnnamed_33 = 68719476769;
pub const KEYC_MOUSEUP1_PANE: C2RustUnnamed_33 = 68719476768;
pub const KEYC_MOUSEDOWN3_BORDER: C2RustUnnamed_33 = 68719476767;
pub const KEYC_MOUSEDOWN3_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476766;
pub const KEYC_MOUSEDOWN3_STATUS_RIGHT: C2RustUnnamed_33 = 68719476765;
pub const KEYC_MOUSEDOWN3_STATUS_LEFT: C2RustUnnamed_33 = 68719476764;
pub const KEYC_MOUSEDOWN3_STATUS: C2RustUnnamed_33 = 68719476763;
pub const KEYC_MOUSEDOWN3_PANE: C2RustUnnamed_33 = 68719476762;
pub const KEYC_MOUSEDOWN2_BORDER: C2RustUnnamed_33 = 68719476761;
pub const KEYC_MOUSEDOWN2_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476760;
pub const KEYC_MOUSEDOWN2_STATUS_RIGHT: C2RustUnnamed_33 = 68719476759;
pub const KEYC_MOUSEDOWN2_STATUS_LEFT: C2RustUnnamed_33 = 68719476758;
pub const KEYC_MOUSEDOWN2_STATUS: C2RustUnnamed_33 = 68719476757;
pub const KEYC_MOUSEDOWN2_PANE: C2RustUnnamed_33 = 68719476756;
pub const KEYC_MOUSEDOWN1_BORDER: C2RustUnnamed_33 = 68719476755;
pub const KEYC_MOUSEDOWN1_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476754;
pub const KEYC_MOUSEDOWN1_STATUS_RIGHT: C2RustUnnamed_33 = 68719476753;
pub const KEYC_MOUSEDOWN1_STATUS_LEFT: C2RustUnnamed_33 = 68719476752;
pub const KEYC_MOUSEDOWN1_STATUS: C2RustUnnamed_33 = 68719476751;
pub const KEYC_MOUSEDOWN1_PANE: C2RustUnnamed_33 = 68719476750;
pub const KEYC_MOUSEMOVE_BORDER: C2RustUnnamed_33 = 68719476749;
pub const KEYC_MOUSEMOVE_STATUS_DEFAULT: C2RustUnnamed_33 = 68719476748;
pub const KEYC_MOUSEMOVE_STATUS_RIGHT: C2RustUnnamed_33 = 68719476747;
pub const KEYC_MOUSEMOVE_STATUS_LEFT: C2RustUnnamed_33 = 68719476746;
pub const KEYC_MOUSEMOVE_STATUS: C2RustUnnamed_33 = 68719476745;
pub const KEYC_MOUSEMOVE_PANE: C2RustUnnamed_33 = 68719476744;
pub const KEYC_DOUBLECLICK: C2RustUnnamed_33 = 68719476743;
pub const KEYC_DRAGGING: C2RustUnnamed_33 = 68719476742;
pub const KEYC_MOUSE: C2RustUnnamed_33 = 68719476741;
pub const KEYC_PASTE_END: C2RustUnnamed_33 = 68719476740;
pub const KEYC_PASTE_START: C2RustUnnamed_33 = 68719476739;
pub const KEYC_ANY: C2RustUnnamed_33 = 68719476738;
pub const KEYC_FOCUS_OUT: C2RustUnnamed_33 = 68719476737;
pub const KEYC_FOCUS_IN: C2RustUnnamed_33 = 68719476736;
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
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
pub type style_align = libc::c_uint;
pub const STYLE_ALIGN_RIGHT: style_align = 3;
pub const STYLE_ALIGN_CENTRE: style_align = 2;
pub const STYLE_ALIGN_LEFT: style_align = 1;
pub const STYLE_ALIGN_DEFAULT: style_align = 0;
pub type style_list = libc::c_uint;
pub const STYLE_LIST_RIGHT_MARKER: style_list = 4;
pub const STYLE_LIST_LEFT_MARKER: style_list = 3;
pub const STYLE_LIST_FOCUS: style_list = 2;
pub const STYLE_LIST_ON: style_list = 1;
pub const STYLE_LIST_OFF: style_list = 0;
pub type style_default_type = libc::c_uint;
pub const STYLE_DEFAULT_POP: style_default_type = 2;
pub const STYLE_DEFAULT_PUSH: style_default_type = 1;
pub const STYLE_DEFAULT_BASE: style_default_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct style {
    pub gc: grid_cell,
    pub ignore: libc::c_int,
    pub fill: libc::c_int,
    pub align: style_align,
    pub list: style_list,
    pub range_type: style_range_type,
    pub range_argument: u_int,
    pub default_type: style_default_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_array {
    pub rbh_root: *mut options_array_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union options_value {
    pub string: *mut libc::c_char,
    pub number: libc::c_longlong,
    pub style: style,
    pub array: options_array,
    pub cmdlist: *mut cmd_list,
}
/*
 * Default terminfo(5) keys. Any keys that have builtin modifiers (that is,
 * where the key itself contains the modifiers) has the KEYC_XTERM flag set so
 * a leading escape is not treated as meta (and probably removed).
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_default_key_code {
    pub code: tty_code_code,
    pub key: key_code,
}
/* Default raw keys. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_default_key_raw {
    pub string: *const libc::c_char,
    pub key: key_code,
}
/* Default xterm keys. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_default_key_xterm {
    pub template: *const libc::c_char,
    pub key: key_code,
}
static mut tty_default_raw_keys: [tty_default_key_raw; 107] =
    [{
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bO[\x00" as *const u8 as
                                         *const libc::c_char,
                                 key: '\u{1b}' as i32 as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOo\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_SLASH as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOj\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_STAR as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOm\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_MINUS as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOw\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_SEVEN as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOx\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_EIGHT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOy\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_NINE as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOk\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_PLUS as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOt\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_FOUR as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOu\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_FIVE as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOv\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_SIX as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOq\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_ONE as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOr\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_TWO as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOs\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_THREE as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOM\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_ENTER as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOp\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_ZERO as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOn\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_KP_PERIOD as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x2000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOA\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_UP as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOB\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_DOWN as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOC\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_RIGHT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOD\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_LEFT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[A\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_UP as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[B\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_DOWN as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[C\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_RIGHT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[D\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_LEFT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1bOA\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_UP as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as libc::c_ulonglong
                                         |
                                         0x100000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1bOB\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_DOWN as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as libc::c_ulonglong
                                         |
                                         0x100000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1bOC\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_RIGHT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as libc::c_ulonglong
                                         |
                                         0x100000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1bOD\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_LEFT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as libc::c_ulonglong
                                         |
                                         0x100000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1b[A\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_UP as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as libc::c_ulonglong
                                         |
                                         0x100000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1b[B\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_DOWN as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as libc::c_ulonglong
                                         |
                                         0x100000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1b[C\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_RIGHT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as libc::c_ulonglong
                                         |
                                         0x100000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1b[D\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_LEFT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x4000000000000 as libc::c_ulonglong
                                         |
                                         0x100000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOH\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_HOME as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOF\x00" as *const u8 as
                                         *const libc::c_char,
                                 key: KEYC_END as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1bOH\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_HOME as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x100000000000 as libc::c_ulonglong |
                                         0x8000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1bOF\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_END as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x100000000000 as libc::c_ulonglong |
                                         0x8000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[H\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_HOME as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[F\x00" as *const u8 as
                                         *const libc::c_char,
                                 key: KEYC_END as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1b[H\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_HOME as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x100000000000 as libc::c_ulonglong |
                                         0x8000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b\x1b[F\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_END as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x100000000000 as libc::c_ulonglong |
                                         0x8000000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOa\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_UP as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOb\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_DOWN as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOc\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_RIGHT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1bOd\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_LEFT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[a\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_UP as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[b\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_DOWN as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[c\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_RIGHT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[d\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_LEFT as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[11^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F1 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[12^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F2 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[13^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F3 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[14^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F4 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[15^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F5 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[17^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F6 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[18^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F7 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[19^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F8 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[20^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F9 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[21^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F10 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[23^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F11 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[24^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F12 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[2^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_IC as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[3^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_DC as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[7^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_HOME as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[8^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_END as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[6^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_NPAGE as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[5^\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_PPAGE as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[11$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F1 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[12$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F2 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[13$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F3 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[14$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F4 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[15$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F5 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[17$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F6 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[18$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F7 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[19$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F8 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[20$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F9 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[21$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F10 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[23$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F11 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[24$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F12 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[2$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_IC as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[3$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_DC as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[7$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_HOME as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[8$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_END as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[6$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_NPAGE as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[5$\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_PPAGE as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[11@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F1 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[12@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F2 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[13@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F3 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[14@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F4 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[15@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F5 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[17@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F6 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[18@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F7 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[19@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F8 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[20@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F9 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[21@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F10 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[23@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F11 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[24@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_F12 as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[2@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_IC as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[3@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_DC as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[7@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_HOME as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[8@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_END as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[6@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_NPAGE as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[5@\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_PPAGE as libc::c_ulong as
                                         libc::c_ulonglong |
                                         0x200000000000 as libc::c_ulonglong |
                                         0x400000000000 as
                                             libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[I\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_FOCUS_IN as libc::c_ulong as
                                         key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[O\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_FOCUS_OUT as libc::c_ulong as
                                         key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[200~\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_PASTE_START as libc::c_ulong as
                                         key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_raw{string:
                                     b"\x1b[201~\x00" as *const u8 as
                                         *const libc::c_char,
                                 key:
                                     KEYC_PASTE_END as libc::c_ulong as
                                         key_code,};
         init
     }];
static mut tty_default_xterm_keys: [tty_default_key_xterm; 30] =
    [{
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[1;_P\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F1 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1bO1;_P\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F1 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1bO_P\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F1 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[1;_Q\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F2 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1bO1;_Q\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F2 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1bO_Q\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F2 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[1;_R\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F3 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1bO1;_R\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F3 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1bO_R\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F3 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[1;_S\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F4 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1bO1;_S\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F4 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1bO_S\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F4 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[15;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F5 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[17;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F6 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[18;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F7 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[19;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F8 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[20;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F9 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[21;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F10 as libc::c_ulong as
                                           key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[23;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F11 as libc::c_ulong as
                                           key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[24;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_F12 as libc::c_ulong as
                                           key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[1;_A\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_UP as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[1;_B\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_DOWN as libc::c_ulong as
                                           key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[1;_C\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_RIGHT as libc::c_ulong as
                                           key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[1;_D\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_LEFT as libc::c_ulong as
                                           key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[1;_H\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_HOME as libc::c_ulong as
                                           key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[1;_F\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_END as libc::c_ulong as
                                           key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[5;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_PPAGE as libc::c_ulong as
                                           key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[6;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_NPAGE as libc::c_ulong as
                                           key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[2;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_IC as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_xterm{template:
                                       b"\x1b[3;_~\x00" as *const u8 as
                                           *const libc::c_char,
                                   key:
                                       KEYC_DC as libc::c_ulong as key_code,};
         init
     }];
static mut tty_default_xterm_modifiers: [key_code; 9] =
    [0 as libc::c_int as key_code, 0 as libc::c_int as key_code,
     0x400000000000 as libc::c_ulonglong,
     0x100000000000 as libc::c_ulonglong |
         0x8000000000000 as libc::c_ulonglong,
     0x400000000000 as libc::c_ulonglong | 0x100000000000 as libc::c_ulonglong
         | 0x8000000000000 as libc::c_ulonglong,
     0x200000000000 as libc::c_ulonglong,
     0x400000000000 as libc::c_ulonglong |
         0x200000000000 as libc::c_ulonglong,
     0x100000000000 as libc::c_ulonglong |
         0x8000000000000 as libc::c_ulonglong |
         0x200000000000 as libc::c_ulonglong,
     0x400000000000 as libc::c_ulonglong | 0x100000000000 as libc::c_ulonglong
         | 0x8000000000000 as libc::c_ulonglong |
         0x200000000000 as libc::c_ulonglong];
static mut tty_default_code_keys: [tty_default_key_code; 136] =
    [{
         let mut init =
             tty_default_key_code{code: TTYC_KF1,
                                  key: KEYC_F1 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF2,
                                  key: KEYC_F2 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF3,
                                  key: KEYC_F3 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF4,
                                  key: KEYC_F4 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF5,
                                  key: KEYC_F5 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF6,
                                  key: KEYC_F6 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF7,
                                  key: KEYC_F7 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF8,
                                  key: KEYC_F8 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF9,
                                  key: KEYC_F9 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF10,
                                  key:
                                      KEYC_F10 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF11,
                                  key:
                                      KEYC_F11 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF12,
                                  key:
                                      KEYC_F12 as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF13,
                                  key:
                                      KEYC_F1 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF14,
                                  key:
                                      KEYC_F2 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF15,
                                  key:
                                      KEYC_F3 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF16,
                                  key:
                                      KEYC_F4 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF17,
                                  key:
                                      KEYC_F5 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF18,
                                  key:
                                      KEYC_F6 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF19,
                                  key:
                                      KEYC_F7 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF20,
                                  key:
                                      KEYC_F8 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF21,
                                  key:
                                      KEYC_F9 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF22,
                                  key:
                                      KEYC_F10 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF23,
                                  key:
                                      KEYC_F11 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF24,
                                  key:
                                      KEYC_F12 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF25,
                                  key:
                                      KEYC_F1 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF26,
                                  key:
                                      KEYC_F2 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF27,
                                  key:
                                      KEYC_F3 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF28,
                                  key:
                                      KEYC_F4 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF29,
                                  key:
                                      KEYC_F5 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF30,
                                  key:
                                      KEYC_F6 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF31,
                                  key:
                                      KEYC_F7 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF32,
                                  key:
                                      KEYC_F8 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF33,
                                  key:
                                      KEYC_F9 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF34,
                                  key:
                                      KEYC_F10 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF35,
                                  key:
                                      KEYC_F11 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF36,
                                  key:
                                      KEYC_F12 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF37,
                                  key:
                                      KEYC_F1 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF38,
                                  key:
                                      KEYC_F2 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF39,
                                  key:
                                      KEYC_F3 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF40,
                                  key:
                                      KEYC_F4 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF41,
                                  key:
                                      KEYC_F5 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF42,
                                  key:
                                      KEYC_F6 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF43,
                                  key:
                                      KEYC_F7 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF44,
                                  key:
                                      KEYC_F8 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF45,
                                  key:
                                      KEYC_F9 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF46,
                                  key:
                                      KEYC_F10 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF47,
                                  key:
                                      KEYC_F11 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF48,
                                  key:
                                      KEYC_F12 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF49,
                                  key:
                                      KEYC_F1 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF50,
                                  key:
                                      KEYC_F2 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF51,
                                  key:
                                      KEYC_F3 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF52,
                                  key:
                                      KEYC_F4 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF53,
                                  key:
                                      KEYC_F5 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF54,
                                  key:
                                      KEYC_F6 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF55,
                                  key:
                                      KEYC_F7 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF56,
                                  key:
                                      KEYC_F8 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF57,
                                  key:
                                      KEYC_F9 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF58,
                                  key:
                                      KEYC_F10 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF59,
                                  key:
                                      KEYC_F11 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF60,
                                  key:
                                      KEYC_F12 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF61,
                                  key:
                                      KEYC_F1 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF62,
                                  key:
                                      KEYC_F2 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KF63,
                                  key:
                                      KEYC_F3 as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KICH1,
                                  key: KEYC_IC as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDCH1,
                                  key: KEYC_DC as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KHOME,
                                  key:
                                      KEYC_HOME as libc::c_ulong as
                                          key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KEND,
                                  key:
                                      KEYC_END as libc::c_ulong as key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KNP,
                                  key:
                                      KEYC_NPAGE as libc::c_ulong as
                                          key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KPP,
                                  key:
                                      KEYC_PPAGE as libc::c_ulong as
                                          key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KCBT,
                                  key:
                                      KEYC_BTAB as libc::c_ulong as
                                          key_code,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KCUU1,
                                  key:
                                      KEYC_UP as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x4000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KCUD1,
                                  key:
                                      KEYC_DOWN as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x4000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KCUB1,
                                  key:
                                      KEYC_LEFT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x4000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KCUF1,
                                  key:
                                      KEYC_RIGHT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x4000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDC2,
                                  key:
                                      KEYC_DC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDC3,
                                  key:
                                      KEYC_DC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDC4,
                                  key:
                                      KEYC_DC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDC5,
                                  key:
                                      KEYC_DC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDC6,
                                  key:
                                      KEYC_DC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDC7,
                                  key:
                                      KEYC_DC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KIND,
                                  key:
                                      KEYC_DOWN as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDN2,
                                  key:
                                      KEYC_DOWN as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDN3,
                                  key:
                                      KEYC_DOWN as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDN4,
                                  key:
                                      KEYC_DOWN as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDN5,
                                  key:
                                      KEYC_DOWN as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDN6,
                                  key:
                                      KEYC_DOWN as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KDN7,
                                  key:
                                      KEYC_DOWN as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KEND2,
                                  key:
                                      KEYC_END as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KEND3,
                                  key:
                                      KEYC_END as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KEND4,
                                  key:
                                      KEYC_END as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KEND5,
                                  key:
                                      KEYC_END as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KEND6,
                                  key:
                                      KEYC_END as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KEND7,
                                  key:
                                      KEYC_END as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KHOM2,
                                  key:
                                      KEYC_HOME as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KHOM3,
                                  key:
                                      KEYC_HOME as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KHOM4,
                                  key:
                                      KEYC_HOME as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KHOM5,
                                  key:
                                      KEYC_HOME as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KHOM6,
                                  key:
                                      KEYC_HOME as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KHOM7,
                                  key:
                                      KEYC_HOME as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KIC2,
                                  key:
                                      KEYC_IC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KIC3,
                                  key:
                                      KEYC_IC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KIC4,
                                  key:
                                      KEYC_IC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KIC5,
                                  key:
                                      KEYC_IC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KIC6,
                                  key:
                                      KEYC_IC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KIC7,
                                  key:
                                      KEYC_IC as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KLFT2,
                                  key:
                                      KEYC_LEFT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KLFT3,
                                  key:
                                      KEYC_LEFT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KLFT4,
                                  key:
                                      KEYC_LEFT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KLFT5,
                                  key:
                                      KEYC_LEFT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KLFT6,
                                  key:
                                      KEYC_LEFT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KLFT7,
                                  key:
                                      KEYC_LEFT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KNXT2,
                                  key:
                                      KEYC_NPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KNXT3,
                                  key:
                                      KEYC_NPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KNXT4,
                                  key:
                                      KEYC_NPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KNXT5,
                                  key:
                                      KEYC_NPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KNXT6,
                                  key:
                                      KEYC_NPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KNXT7,
                                  key:
                                      KEYC_NPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KPRV2,
                                  key:
                                      KEYC_PPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KPRV3,
                                  key:
                                      KEYC_PPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KPRV4,
                                  key:
                                      KEYC_PPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KPRV5,
                                  key:
                                      KEYC_PPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KPRV6,
                                  key:
                                      KEYC_PPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KPRV7,
                                  key:
                                      KEYC_PPAGE as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KRIT2,
                                  key:
                                      KEYC_RIGHT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KRIT3,
                                  key:
                                      KEYC_RIGHT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KRIT4,
                                  key:
                                      KEYC_RIGHT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KRIT5,
                                  key:
                                      KEYC_RIGHT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KRIT6,
                                  key:
                                      KEYC_RIGHT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KRIT7,
                                  key:
                                      KEYC_RIGHT as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KRI,
                                  key:
                                      KEYC_UP as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KUP2,
                                  key:
                                      KEYC_UP as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KUP3,
                                  key:
                                      KEYC_UP as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KUP4,
                                  key:
                                      KEYC_UP as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KUP5,
                                  key:
                                      KEYC_UP as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KUP6,
                                  key:
                                      KEYC_UP as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x400000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     },
     {
         let mut init =
             tty_default_key_code{code: TTYC_KUP7,
                                  key:
                                      KEYC_UP as libc::c_ulong as
                                          libc::c_ulonglong |
                                          0x100000000000 as libc::c_ulonglong
                                          |
                                          0x8000000000000 as libc::c_ulonglong
                                          |
                                          0x200000000000 as
                                              libc::c_ulonglong,};
         init
     }];
/* Add key to tree. */
unsafe extern "C" fn tty_keys_add(mut tty: *mut tty,
                                  mut s: *const libc::c_char,
                                  mut key: key_code) {
    let mut tk: *mut tty_key = 0 as *mut tty_key;
    let mut size: size_t = 0;
    let mut keystr: *const libc::c_char = 0 as *const libc::c_char;
    keystr = key_string_lookup_key(key, 1 as libc::c_int);
    tk = tty_keys_find(tty, s, strlen(s), &mut size);
    if tk.is_null() {
        log_debug(b"new key %s: 0x%llx (%s)\x00" as *const u8 as
                      *const libc::c_char, s, key, keystr);
        tty_keys_add1(&mut (*tty).key_tree, s, key);
    } else {
        log_debug(b"replacing key %s: 0x%llx (%s)\x00" as *const u8 as
                      *const libc::c_char, s, key, keystr);
        (*tk).key = key
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
/*
 * Handle keys input from the outside terminal. tty_default_*_keys[] are a base
 * table of supported keys which are looked up in terminfo(5) and translated
 * into a ternary tree.
 */
/* Add next node to the tree. */
unsafe extern "C" fn tty_keys_add1(mut tkp: *mut *mut tty_key,
                                   mut s: *const libc::c_char,
                                   mut key: key_code) {
    let mut tk: *mut tty_key = 0 as *mut tty_key;
    /* Allocate a tree entry if there isn't one already. */
    tk = *tkp;
    if tk.is_null() {
        *tkp =
            xcalloc(1 as libc::c_int as size_t,
                    ::std::mem::size_of::<tty_key>() as libc::c_ulong) as
                *mut tty_key;
        tk = *tkp;
        (*tk).ch = *s;
        (*tk).key = 0xfe000000000 as libc::c_ulonglong
    }
    /* Find the next entry. */
    if *s as libc::c_int == (*tk).ch as libc::c_int {
        /* Move forward in string. */
        s = s.offset(1);
        /* If this is the end of the string, no more is necessary. */
        if *s as libc::c_int == '\u{0}' as i32 { (*tk).key = key; return }
        /* Use the child tree for the next character. */
        tkp = &mut (*tk).next
    } else if (*s as libc::c_int) < (*tk).ch as libc::c_int {
        tkp = &mut (*tk).left
    } else if *s as libc::c_int > (*tk).ch as libc::c_int {
        tkp = &mut (*tk).right
    }
    /* And recurse to add it. */
    tty_keys_add1(tkp, s, key);
}
/* Initialise a key tree from the table. */
#[no_mangle]
pub unsafe extern "C" fn tty_keys_build(mut tty: *mut tty) {
    let mut tdkr: *const tty_default_key_raw =
        0 as *const tty_default_key_raw;
    let mut tdkx: *const tty_default_key_xterm =
        0 as *const tty_default_key_xterm;
    let mut tdkc: *const tty_default_key_code =
        0 as *const tty_default_key_code;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    let mut ov: *mut options_value = 0 as *mut options_value;
    let mut copy: [libc::c_char; 16] = [0; 16];
    let mut key: key_code = 0;
    if !(*tty).key_tree.is_null() { tty_keys_free(tty); }
    (*tty).key_tree = 0 as *mut tty_key;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[tty_default_key_xterm; 30]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<tty_default_key_xterm>()
                                                   as libc::c_ulong) {
        tdkx =
            &*tty_default_xterm_keys.as_ptr().offset(i as isize) as
                *const tty_default_key_xterm;
        j = 2 as libc::c_int as u_int;
        while (j as libc::c_ulong) <
                  (::std::mem::size_of::<[key_code; 9]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<key_code>()
                                                       as libc::c_ulong) {
            strlcpy(copy.as_mut_ptr(), (*tdkx).template,
                    ::std::mem::size_of::<[libc::c_char; 16]>() as
                        libc::c_ulong);
            copy[strcspn(copy.as_mut_ptr(),
                         b"_\x00" as *const u8 as *const libc::c_char) as
                     usize] =
                ('0' as i32 as libc::c_uint).wrapping_add(j) as libc::c_char;
            key = (*tdkx).key | tty_default_xterm_modifiers[j as usize];
            tty_keys_add(tty, copy.as_mut_ptr(), key);
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[tty_default_key_raw; 107]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<tty_default_key_raw>()
                                                   as libc::c_ulong) {
        tdkr =
            &*tty_default_raw_keys.as_ptr().offset(i as isize) as
                *const tty_default_key_raw;
        s = (*tdkr).string;
        if *s as libc::c_int != '\u{0}' as i32 {
            tty_keys_add(tty, s, (*tdkr).key);
        }
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[tty_default_key_code; 136]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<tty_default_key_code>()
                                                   as libc::c_ulong) {
        tdkc =
            &*tty_default_code_keys.as_ptr().offset(i as isize) as
                *const tty_default_key_code;
        s = tty_term_string((*tty).term, (*tdkc).code);
        if *s as libc::c_int != '\u{0}' as i32 {
            tty_keys_add(tty, s, (*tdkc).key);
        }
        i = i.wrapping_add(1)
    }
    o =
        options_get(global_options,
                    b"user-keys\x00" as *const u8 as *const libc::c_char);
    if !o.is_null() {
        a = options_array_first(o);
        while !a.is_null() {
            i = options_array_item_index(a);
            ov = options_array_item_value(a);
            tty_keys_add(tty, (*ov).string,
                         (0x2000000000 as
                              libc::c_ulonglong).wrapping_add(i as
                                                                  libc::c_ulonglong));
            a = options_array_next(a)
        }
    };
}
/* Free the entire key tree. */
#[no_mangle]
pub unsafe extern "C" fn tty_keys_free(mut tty: *mut tty) {
    tty_keys_free1((*tty).key_tree);
}
/* Free a single key. */
unsafe extern "C" fn tty_keys_free1(mut tk: *mut tty_key) {
    if !(*tk).next.is_null() { tty_keys_free1((*tk).next); }
    if !(*tk).left.is_null() { tty_keys_free1((*tk).left); }
    if !(*tk).right.is_null() { tty_keys_free1((*tk).right); }
    free(tk as *mut libc::c_void);
}
/* Lookup a key in the tree. */
unsafe extern "C" fn tty_keys_find(mut tty: *mut tty,
                                   mut buf: *const libc::c_char,
                                   mut len: size_t, mut size: *mut size_t)
 -> *mut tty_key {
    *size = 0 as libc::c_int as size_t;
    return tty_keys_find1((*tty).key_tree, buf, len, size);
}
/* Find the next node. */
unsafe extern "C" fn tty_keys_find1(mut tk: *mut tty_key,
                                    mut buf: *const libc::c_char,
                                    mut len: size_t, mut size: *mut size_t)
 -> *mut tty_key {
    /* If no data, no match. */
    if len == 0 as libc::c_int as libc::c_ulong { return 0 as *mut tty_key }
    /* If the node is NULL, this is the end of the tree. No match. */
    if tk.is_null() { return 0 as *mut tty_key }
    /* Pick the next in the sequence. */
    if (*tk).ch as libc::c_int == *buf as libc::c_int {
        /* Move forward in the string. */
        buf = buf.offset(1);
        len = len.wrapping_sub(1);
        *size = (*size).wrapping_add(1);
        /* At the end of the string, return the current node. */
        if len == 0 as libc::c_int as libc::c_ulong ||
               (*tk).next.is_null() &&
                   (*tk).key != 0xfe000000000 as libc::c_ulonglong {
            return tk
        }
        /* Move into the next tree for the following character. */
        tk = (*tk).next
    } else if (*buf as libc::c_int) < (*tk).ch as libc::c_int {
        tk = (*tk).left
    } else if *buf as libc::c_int > (*tk).ch as libc::c_int {
        tk = (*tk).right
    }
    /* Move to the next in the tree. */
    return tty_keys_find1(tk, buf, len, size);
}
/* Look up part of the next key. */
unsafe extern "C" fn tty_keys_next1(mut tty: *mut tty,
                                    mut buf: *const libc::c_char,
                                    mut len: size_t, mut key: *mut key_code,
                                    mut size: *mut size_t,
                                    mut expired: libc::c_int) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut tk: *mut tty_key = 0 as *mut tty_key;
    let mut tk1: *mut tty_key = 0 as *mut tty_key;
    let mut ud: utf8_data =
        utf8_data{data: [0; 21], have: 0, size: 0, width: 0,};
    let mut more: utf8_state = UTF8_MORE;
    let mut uc: utf8_char = 0;
    let mut i: u_int = 0;
    log_debug(b"%s: next key is %zu (%.*s) (expired=%d)\x00" as *const u8 as
                  *const libc::c_char, (*c).name, len, len as libc::c_int,
              buf, expired);
    /* Is this a known key? */
    tk = tty_keys_find(tty, buf, len, size);
    if !tk.is_null() && (*tk).key != 0xfe000000000 as libc::c_ulonglong {
        tk1 = tk;
        loop  {
            log_debug(b"%s: keys in list: %#llx\x00" as *const u8 as
                          *const libc::c_char, (*c).name, (*tk1).key);
            tk1 = (*tk1).next;
            if tk1.is_null() { break ; }
        }
        if !(*tk).next.is_null() && expired == 0 { return 1 as libc::c_int }
        *key = (*tk).key;
        return 0 as libc::c_int
    }
    /* Is this valid UTF-8? */
    more = utf8_open(&mut ud, *buf as u_char);
    if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
        *size = ud.size as size_t;
        if len < ud.size as libc::c_ulong {
            if expired == 0 { return 1 as libc::c_int }
            return -(1 as libc::c_int)
        }
        i = 1 as libc::c_int as u_int;
        while i < ud.size as libc::c_uint {
            more = utf8_append(&mut ud, *buf.offset(i as isize) as u_char);
            i = i.wrapping_add(1)
        }
        if more as libc::c_uint != UTF8_DONE as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int)
        }
        if utf8_from_data(&mut ud, &mut uc) as libc::c_uint !=
               UTF8_DONE as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int)
        }
        *key = uc as key_code;
        log_debug(b"%s: UTF-8 key %.*s %#llx\x00" as *const u8 as
                      *const libc::c_char, (*c).name, ud.size as libc::c_int,
                  ud.data.as_mut_ptr(), *key);
        return 0 as libc::c_int
    }
    return -(1 as libc::c_int);
}
/* Process at least one key in the buffer. Return 0 if no keys present. */
#[no_mangle]
pub unsafe extern "C" fn tty_keys_next(mut tty: *mut tty) -> libc::c_int {
    let mut current_block: u64;
    let mut c: *mut client = (*tty).client;
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut size: size_t = 0;
    let mut bspace: cc_t = 0;
    let mut delay: libc::c_int = 0;
    let mut expired: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut key: key_code = 0;
    let mut m: mouse_event =
        {
            let mut init =
                mouse_event{valid: 0 as libc::c_int,
                            ignore: 0,
                            key: 0,
                            statusat: 0,
                            statuslines: 0,
                            x: 0,
                            y: 0,
                            b: 0,
                            lx: 0,
                            ly: 0,
                            lb: 0,
                            ox: 0,
                            oy: 0,
                            s: 0,
                            w: 0,
                            wp: 0,
                            sgr_type: 0,
                            sgr_b: 0,};
            init
        };
    let mut event: *mut key_event = 0 as *mut key_event;
    /* Get key buffer. */
    buf =
        evbuffer_pullup((*tty).in_0, -(1 as libc::c_int) as ssize_t) as
            *const libc::c_char;
    len = evbuffer_get_length((*tty).in_0);
    if len == 0 as libc::c_int as libc::c_ulong { return 0 as libc::c_int }
    log_debug(b"%s: keys are %zu (%.*s)\x00" as *const u8 as
                  *const libc::c_char, (*c).name, len, len as libc::c_int,
              buf);
    /* Is this a clipboard response? */
    match tty_keys_clipboard(tty, buf, len, &mut size) {
        0 => {
            /* yes */
            key = 0xfe000000000 as libc::c_ulonglong;
            current_block = 18117557595413158975;
        }
        -1 => {
            /* no, or not valid */
            current_block = 12800627514080957624;
        }
        1 => {
            /* partial */
            current_block = 3055274896829046098;
        }
        _ => { current_block = 12800627514080957624; }
    }
    match current_block {
        12800627514080957624 =>
        /* Is this a device attributes response? */
        {
            match tty_keys_device_attributes(tty, buf, len, &mut size) {
                0 => {
                    /* yes */
                    key = 0xfe000000000 as libc::c_ulonglong;
                    current_block = 18117557595413158975;
                }
                -1 => {
                    /* no, or not valid */
                    current_block = 17833034027772472439;
                }
                1 => {
                    /* partial */
                    current_block = 3055274896829046098;
                }
                _ => { current_block = 17833034027772472439; }
            }
            match current_block {
                18117557595413158975 => { }
                3055274896829046098 => { }
                _ =>
                /* Is this an extended device attributes response? */
                {
                    match tty_keys_extended_device_attributes(tty, buf, len,
                                                              &mut size) {
                        0 => {
                            /* yes */
                            key = 0xfe000000000 as libc::c_ulonglong;
                            current_block = 18117557595413158975;
                        }
                        -1 => {
                            /* no, or not valid */
                            current_block = 5783071609795492627;
                        }
                        1 => {
                            /* partial */
                            current_block = 3055274896829046098;
                        }
                        _ => { current_block = 5783071609795492627; }
                    }
                    match current_block {
                        18117557595413158975 => { }
                        3055274896829046098 => { }
                        _ =>
                        /* Is this a mouse key press? */
                        {
                            match tty_keys_mouse(tty, buf, len, &mut size,
                                                 &mut m) {
                                0 => {
                                    /* yes */
                                    key =
                                        KEYC_MOUSE as libc::c_ulong as
                                            key_code;
                                    current_block = 18117557595413158975;
                                }
                                -1 => {
                                    /* no, or not valid */
                                    current_block = 15925075030174552612;
                                }
                                -2 => {
                                    /* yes, but we don't care. */
                                    key =
                                        KEYC_MOUSE as libc::c_ulong as
                                            key_code;
                                    log_debug(b"%s: discard key %.*s %#llx\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              (*c).name, size as libc::c_int,
                                              buf, key);
                                    /* Remove data from buffer. */
                                    evbuffer_drain((*tty).in_0, size);
                                    return 1 as libc::c_int
                                }
                                1 => {
                                    /* partial */
                                    current_block = 3055274896829046098;
                                }
                                _ => { current_block = 15925075030174552612; }
                            }
                            match current_block {
                                18117557595413158975 => { }
                                3055274896829046098 => { }
                                _ =>
                                /* Is this an extended key press? */
                                {
                                    match tty_keys_extended_key(tty, buf, len,
                                                                &mut size,
                                                                &mut key) {
                                        0 => {
                                            current_block =
                                                13861916007813738255;
                                            match current_block {
                                                8502317632408751573 =>
                                                /* partial */
                                                {
                                                    current_block =
                                                        3055274896829046098;
                                                }
                                                _ =>
                                                /* yes */
                                                {
                                                    current_block =
                                                        18117557595413158975;
                                                }
                                            }
                                        }
                                        1 => {
                                            current_block =
                                                8502317632408751573;
                                            match current_block {
                                                8502317632408751573 => {
                                                    current_block =
                                                        3055274896829046098;
                                                }
                                                _ => {
                                                    current_block =
                                                        18117557595413158975;
                                                }
                                            }
                                        }
                                        -1 | _ => {
                                            current_block =
                                                3752191268344402962;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    loop  {
        match current_block {
            3752191268344402962 =>
            /* no, or not valid */
            /* Try to lookup complete key. */
            {
                n =
                    tty_keys_next1(tty, buf, len, &mut key, &mut size,
                                   expired);
                if n == 0 as libc::c_int {
                    current_block = 18117557595413158975;
                    continue ;
                }
                if n == 1 as libc::c_int {
                    current_block = 3055274896829046098;
                    continue ;
                }
                /*
	 * If not a complete key, look for key with an escape prefix (meta
	 * modifier).
	 */
                if *buf as libc::c_int == '\u{1b}' as i32 &&
                       len > 1 as libc::c_int as libc::c_ulong {
                    /* Look for a key without the escape. */
                    n =
                        tty_keys_next1(tty,
                                       buf.offset(1 as libc::c_int as isize),
                                       len.wrapping_sub(1 as libc::c_int as
                                                            libc::c_ulong),
                                       &mut key, &mut size, expired);
                    if n == 0 as libc::c_int {
                        /* found */
                        if key & 0x8000000000000 as libc::c_ulonglong != 0 {
                            /*
				 * We want the escape key as well as the xterm
				 * key, because the xterm sequence implicitly
				 * includes the escape (so if we see
				 * \033\033[1;3D we know it is an Escape
				 * followed by M-Left, not just M-Left).
				 */
                            key = '\u{1b}' as i32 as key_code;
                            size = 1 as libc::c_int as size_t;
                            current_block = 18117557595413158975;
                            continue ;
                        } else {
                            key |= 0x100000000000 as libc::c_ulonglong;
                            size = size.wrapping_add(1);
                            current_block = 18117557595413158975;
                            continue ;
                        }
                    } else if n == 1 as libc::c_int {
                        current_block = 3055274896829046098;
                        continue ;
                    }
                }
                /*
	 * At this point, we know the key is not partial (with or without
	 * escape). So pass it through even if the timer has not expired.
	 */
                if *buf as libc::c_int == '\u{1b}' as i32 &&
                       len >= 2 as libc::c_int as libc::c_ulong {
                    key =
                        *buf.offset(1 as libc::c_int as isize) as u_char as
                            libc::c_ulonglong |
                            0x100000000000 as libc::c_ulonglong;
                    size = 2 as libc::c_int as size_t
                } else {
                    key =
                        *buf.offset(0 as libc::c_int as isize) as u_char as
                            key_code;
                    size = 1 as libc::c_int as size_t
                }
                current_block = 18117557595413158975;
            }
            18117557595413158975 =>
            /* found */
            {
                log_debug(b"%s: complete key %.*s %#llx\x00" as *const u8 as
                              *const libc::c_char, (*c).name,
                          size as libc::c_int, buf, key);
                /*
	 * Check for backspace key using termios VERASE - the terminfo
	 * kbs entry is extremely unreliable, so cannot be safely
	 * used. termios should have a better idea.
	 */
                bspace = (*tty).tio.c_cc[2 as libc::c_int as usize];
                if bspace as libc::c_int != '\u{0}' as i32 &&
                       key & 0xfffffffffff as libc::c_ulonglong ==
                           bspace as libc::c_ulonglong {
                    key =
                        key & 0xf00000000000 as libc::c_ulonglong |
                            KEYC_BSPACE as libc::c_ulong as libc::c_ulonglong
                }
                /* Remove data from buffer. */
                evbuffer_drain((*tty).in_0, size);
                /* Remove key timer. */
                if event_initialized(&mut (*tty).key_timer) != 0 {
                    event_del(&mut (*tty).key_timer);
                }
                (*tty).flags &= !(0x4 as libc::c_int);
                /* Check for focus events. */
                if key == KEYC_FOCUS_OUT as libc::c_ulong as libc::c_ulonglong
                   {
                    (*(*tty).client).flags &=
                        !(0x8000 as libc::c_int) as libc::c_ulong
                } else if key ==
                              KEYC_FOCUS_IN as libc::c_ulong as
                                  libc::c_ulonglong {
                    (*(*tty).client).flags |=
                        0x8000 as libc::c_int as libc::c_ulong
                }
                /* Fire the key. */
                if key != 0xfe000000000 as libc::c_ulonglong {
                    event =
                        xmalloc(::std::mem::size_of::<key_event>() as
                                    libc::c_ulong) as *mut key_event;
                    (*event).key = key;
                    memcpy(&mut (*event).m as *mut mouse_event as
                               *mut libc::c_void,
                           &mut m as *mut mouse_event as *const libc::c_void,
                           ::std::mem::size_of::<mouse_event>() as
                               libc::c_ulong);
                    if server_client_handle_key(c, event) == 0 {
                        free(event as *mut libc::c_void);
                    }
                }
                return 1 as libc::c_int
            }
            _ =>
            /* partial */
            {
                log_debug(b"%s: partial key %.*s\x00" as *const u8 as
                              *const libc::c_char, (*c).name,
                          len as libc::c_int, buf);
                /* If timer is going, check for expiration. */
                if (*tty).flags & 0x4 as libc::c_int != 0 {
                    if event_initialized(&mut (*tty).key_timer) != 0 &&
                           event_pending(&mut (*tty).key_timer,
                                         0x1 as libc::c_int as libc::c_short,
                                         0 as *mut timeval) == 0 {
                        expired = 1 as libc::c_int;
                        current_block = 3752191268344402962;
                    } else { return 0 as libc::c_int }
                } else {
                    /* Get the time period. */
                    delay =
                        options_get_number(global_options,
                                           b"escape-time\x00" as *const u8 as
                                               *const libc::c_char) as
                            libc::c_int;
                    tv.tv_sec = (delay / 1000 as libc::c_int) as __time_t;
                    tv.tv_usec =
                        (delay % 1000 as libc::c_int) as libc::c_long *
                            1000 as libc::c_long;
                    /* Start the timer. */
                    if event_initialized(&mut (*tty).key_timer) != 0 {
                        event_del(&mut (*tty).key_timer);
                    }
                    event_set(&mut (*tty).key_timer, -(1 as libc::c_int),
                              0 as libc::c_int as libc::c_short,
                              Some(tty_keys_callback as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_short,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()), tty as *mut libc::c_void);
                    event_add(&mut (*tty).key_timer, &mut tv);
                    (*tty).flags |= 0x4 as libc::c_int;
                    return 0 as libc::c_int
                }
            }
        }
    };
}
/* Key timer callback. */
unsafe extern "C" fn tty_keys_callback(mut fd: libc::c_int,
                                       mut events: libc::c_short,
                                       mut data: *mut libc::c_void) {
    let mut tty: *mut tty = data as *mut tty;
    if (*tty).flags & 0x4 as libc::c_int != 0 {
        while tty_keys_next(tty) != 0 { }
    };
}
/*
 * Handle extended key input. This has two forms: \033[27;m;k~ and \033[k;mu,
 * where k is key as a number and m is a modifier. Returns 0 for success, -1
 * for failure, 1 for partial;
 */
unsafe extern "C" fn tty_keys_extended_key(mut tty: *mut tty,
                                           mut buf: *const libc::c_char,
                                           mut len: size_t,
                                           mut size: *mut size_t,
                                           mut key: *mut key_code)
 -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut end: size_t = 0;
    let mut number: u_int = 0;
    let mut modifiers: u_int = 0;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    *size = 0 as libc::c_int as size_t;
    /* First two bytes are always \033[. */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int !=
           '\u{1b}' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 1 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(1 as libc::c_int as isize) as libc::c_int != '[' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 2 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    /*
	 * Look for a terminator. Stop at either '~' or anything that isn't a
	 * number or ';'.
	 */
    end = 2 as libc::c_int as size_t;
    while end < len &&
              end !=
                  ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
          {
        if *buf.offset(end as isize) as libc::c_int == '~' as i32 { break ; }
        if *(*__ctype_b_loc()).offset(*buf.offset(end as isize) as u_char as
                                          libc::c_int as isize) as libc::c_int
               & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
               && *buf.offset(end as isize) as libc::c_int != ';' as i32 {
            break ;
        }
        end = end.wrapping_add(1)
    }
    if end == len { return 1 as libc::c_int }
    if end == ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong ||
           *buf.offset(end as isize) as libc::c_int != '~' as i32 &&
               *buf.offset(end as isize) as libc::c_int != 'u' as i32 {
        return -(1 as libc::c_int)
    }
    /* Copy to the buffer. */
    memcpy(tmp.as_mut_ptr() as *mut libc::c_void,
           buf.offset(2 as libc::c_int as isize) as *const libc::c_void, end);
    tmp[end as usize] = '\u{0}' as i32 as libc::c_char;
    /* Try to parse either form of key. */
    if *buf.offset(end as isize) as libc::c_int == '~' as i32 {
        if sscanf(tmp.as_mut_ptr(),
                  b"27;%u;%u\x00" as *const u8 as *const libc::c_char,
                  &mut modifiers as *mut u_int, &mut number as *mut u_int) !=
               2 as libc::c_int {
            return -(1 as libc::c_int)
        }
    } else if sscanf(tmp.as_mut_ptr(),
                     b"%u;%u\x00" as *const u8 as *const libc::c_char,
                     &mut number as *mut u_int, &mut modifiers as *mut u_int)
                  != 2 as libc::c_int {
        return -(1 as libc::c_int)
    }
    *size = end.wrapping_add(1 as libc::c_int as libc::c_ulong);
    /* Store the key and modifiers. */
    *key = number as key_code;
    match modifiers {
        2 => { *key |= 0x400000000000 as libc::c_ulonglong }
        3 => {
            *key |=
                0x100000000000 as libc::c_ulonglong |
                    0x8000000000000 as libc::c_ulonglong
        }
        4 => {
            *key |=
                0x400000000000 as libc::c_ulonglong |
                    0x100000000000 as libc::c_ulonglong |
                    0x8000000000000 as libc::c_ulonglong
        }
        5 => { *key |= 0x200000000000 as libc::c_ulonglong }
        6 => {
            *key |=
                0x400000000000 as libc::c_ulonglong |
                    0x200000000000 as libc::c_ulonglong
        }
        7 => {
            *key |=
                0x100000000000 as libc::c_ulonglong |
                    0x200000000000 as libc::c_ulonglong
        }
        8 => {
            *key |=
                0x400000000000 as libc::c_ulonglong |
                    0x100000000000 as libc::c_ulonglong |
                    0x8000000000000 as libc::c_ulonglong |
                    0x200000000000 as libc::c_ulonglong
        }
        _ => { *key = 0xff000000000 as libc::c_ulonglong }
    }
    if log_get_level() != 0 as libc::c_int {
        log_debug(b"%s: extended key %.*s is %llx (%s)\x00" as *const u8 as
                      *const libc::c_char, (*c).name, *size as libc::c_int,
                  buf, *key, key_string_lookup_key(*key, 1 as libc::c_int));
    }
    return 0 as libc::c_int;
}
/*
 * Handle mouse key input. Returns 0 for success, -1 for failure, 1 for partial
 * (probably a mouse sequence but need more data).
 */
unsafe extern "C" fn tty_keys_mouse(mut tty: *mut tty,
                                    mut buf: *const libc::c_char,
                                    mut len: size_t, mut size: *mut size_t,
                                    mut m: *mut mouse_event) -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut b: u_int = 0;
    let mut sgr_b: u_int = 0;
    let mut sgr_type: u_char = 0;
    let mut ch: u_char = 0;
    /*
	 * Standard mouse sequences are \033[M followed by three characters
	 * indicating button, X and Y, all based at 32 with 1,1 top-left.
	 *
	 * UTF-8 mouse sequences are similar but the three are expressed as
	 * UTF-8 characters.
	 *
	 * SGR extended mouse sequences are \033[< followed by three numbers in
	 * decimal and separated by semicolons indicating button, X and Y. A
	 * trailing 'M' is click or scroll and trailing 'm' release. All are
	 * based at 0 with 1,1 top-left.
	 */
    *size = 0 as libc::c_int as size_t;
    sgr_b = 0 as libc::c_int as u_int;
    b = sgr_b;
    y = b;
    x = y;
    sgr_type = ' ' as i32 as u_char;
    /* First two bytes are always \033[. */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int !=
           '\u{1b}' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 1 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(1 as libc::c_int as isize) as libc::c_int != '[' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 2 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    /*
	 * Third byte is M in old standard (and UTF-8 extension which we do not
	 * support), < in SGR extension.
	 */
    if *buf.offset(2 as libc::c_int as isize) as libc::c_int == 'M' as i32 {
        /* Read the three inputs. */
        *size = 3 as libc::c_int as size_t;
        i = 0 as libc::c_int as u_int;
        while i < 3 as libc::c_int as libc::c_uint {
            if len <= *size { return 1 as libc::c_int }
            let fresh0 = *size;
            *size = (*size).wrapping_add(1);
            ch = *buf.offset(fresh0 as isize) as u_char;
            if i == 0 as libc::c_int as libc::c_uint {
                b = ch as u_int
            } else if i == 1 as libc::c_int as libc::c_uint {
                x = ch as u_int
            } else { y = ch as u_int }
            i = i.wrapping_add(1)
        }
        log_debug(b"%s: mouse input: %.*s\x00" as *const u8 as
                      *const libc::c_char, (*c).name, *size as libc::c_int,
                  buf);
        /* Check and return the mouse input. */
        if b < 32 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int)
        }
        b =
            (b as
                 libc::c_uint).wrapping_sub(32 as libc::c_int as libc::c_uint)
                as u_int as u_int;
        if x >= 33 as libc::c_int as libc::c_uint {
            x =
                (x as
                     libc::c_uint).wrapping_sub(33 as libc::c_int as
                                                    libc::c_uint) as u_int as
                    u_int
        } else { x = (256 as libc::c_int as libc::c_uint).wrapping_sub(x) }
        if y >= 33 as libc::c_int as libc::c_uint {
            y =
                (y as
                     libc::c_uint).wrapping_sub(33 as libc::c_int as
                                                    libc::c_uint) as u_int as
                    u_int
        } else { y = (256 as libc::c_int as libc::c_uint).wrapping_sub(y) }
    } else if *buf.offset(2 as libc::c_int as isize) as libc::c_int ==
                  '<' as i32 {
        /* Read the three inputs. */
        *size = 3 as libc::c_int as size_t;
        loop  {
            if len <= *size { return 1 as libc::c_int }
            let fresh1 = *size;
            *size = (*size).wrapping_add(1);
            ch = *buf.offset(fresh1 as isize) as u_char;
            if ch as libc::c_int == ';' as i32 { break ; }
            if (ch as libc::c_int) < '0' as i32 ||
                   ch as libc::c_int > '9' as i32 {
                return -(1 as libc::c_int)
            }
            sgr_b =
                (10 as libc::c_int as
                     libc::c_uint).wrapping_mul(sgr_b).wrapping_add((ch as
                                                                         libc::c_int
                                                                         -
                                                                         '0'
                                                                             as
                                                                             i32)
                                                                        as
                                                                        libc::c_uint)
        }
        loop  {
            if len <= *size { return 1 as libc::c_int }
            let fresh2 = *size;
            *size = (*size).wrapping_add(1);
            ch = *buf.offset(fresh2 as isize) as u_char;
            if ch as libc::c_int == ';' as i32 { break ; }
            if (ch as libc::c_int) < '0' as i32 ||
                   ch as libc::c_int > '9' as i32 {
                return -(1 as libc::c_int)
            }
            x =
                (10 as libc::c_int as
                     libc::c_uint).wrapping_mul(x).wrapping_add((ch as
                                                                     libc::c_int
                                                                     -
                                                                     '0' as
                                                                         i32)
                                                                    as
                                                                    libc::c_uint)
        }
        loop  {
            if len <= *size { return 1 as libc::c_int }
            let fresh3 = *size;
            *size = (*size).wrapping_add(1);
            ch = *buf.offset(fresh3 as isize) as u_char;
            if ch as libc::c_int == 'M' as i32 ||
                   ch as libc::c_int == 'm' as i32 {
                break ;
            }
            if (ch as libc::c_int) < '0' as i32 ||
                   ch as libc::c_int > '9' as i32 {
                return -(1 as libc::c_int)
            }
            y =
                (10 as libc::c_int as
                     libc::c_uint).wrapping_mul(y).wrapping_add((ch as
                                                                     libc::c_int
                                                                     -
                                                                     '0' as
                                                                         i32)
                                                                    as
                                                                    libc::c_uint)
        }
        log_debug(b"%s: mouse input (SGR): %.*s\x00" as *const u8 as
                      *const libc::c_char, (*c).name, *size as libc::c_int,
                  buf);
        /* Check and return the mouse input. */
        if x < 1 as libc::c_int as libc::c_uint ||
               y < 1 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int)
        }
        x = x.wrapping_sub(1);
        y = y.wrapping_sub(1);
        b = sgr_b;
        /* Type is M for press, m for release. */
        sgr_type = ch;
        if sgr_type as libc::c_int == 'm' as i32 {
            b |= 3 as libc::c_int as libc::c_uint
        }
        /*
		 * Some terminals (like PuTTY 0.63) mistakenly send
		 * button-release events for scroll-wheel button-press event.
		 * Discard it before it reaches any program running inside
		 * tmux.
		 */
        if sgr_type as libc::c_int == 'm' as i32 &&
               sgr_b & 64 as libc::c_int as libc::c_uint != 0 {
            return -(2 as libc::c_int)
        }
    } else { return -(1 as libc::c_int) }
    /* Fill mouse event. */
    (*m).lx = (*tty).mouse_last_x;
    (*m).x = x;
    (*m).ly = (*tty).mouse_last_y;
    (*m).y = y;
    (*m).lb = (*tty).mouse_last_b;
    (*m).b = b;
    (*m).sgr_type = sgr_type as u_int;
    (*m).sgr_b = sgr_b;
    /* Update last mouse state. */
    (*tty).mouse_last_x = x;
    (*tty).mouse_last_y = y;
    (*tty).mouse_last_b = b;
    return 0 as libc::c_int;
}
/*
 * Handle OSC 52 clipboard input. Returns 0 for success, -1 for failure, 1 for
 * partial.
 */
unsafe extern "C" fn tty_keys_clipboard(mut tty: *mut tty,
                                        mut buf: *const libc::c_char,
                                        mut len: size_t,
                                        mut size: *mut size_t)
 -> libc::c_int {
    let mut end: size_t = 0;
    let mut terminator: size_t = 0;
    let mut needed: size_t = 0;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outlen: libc::c_int = 0;
    *size = 0 as libc::c_int as size_t;
    /* First five bytes are always \033]52;. */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int !=
           '\u{1b}' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 1 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(1 as libc::c_int as isize) as libc::c_int != ']' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 2 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(2 as libc::c_int as isize) as libc::c_int != '5' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 3 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(3 as libc::c_int as isize) as libc::c_int != '2' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 4 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(4 as libc::c_int as isize) as libc::c_int != ';' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 5 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    /* Find the terminator if any. */
    end = 5 as libc::c_int as size_t;
    while end < len {
        if *buf.offset(end as isize) as libc::c_int == '\u{7}' as i32 {
            terminator = 1 as libc::c_int as size_t;
            break ;
        } else if end > 5 as libc::c_int as libc::c_ulong &&
                      *buf.offset(end.wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) as
                                      isize) as libc::c_int == '\u{1b}' as i32
                      &&
                      *buf.offset(end as isize) as libc::c_int == '\\' as i32
         {
            terminator = 2 as libc::c_int as size_t;
            break ;
        } else { end = end.wrapping_add(1) }
    }
    if end == len { return 1 as libc::c_int }
    *size = end.wrapping_add(terminator);
    /* Skip the initial part. */
    buf = buf.offset(5 as libc::c_int as isize);
    end =
        (end as libc::c_ulong).wrapping_sub(5 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    /* Get the second argument. */
    while end != 0 as libc::c_int as libc::c_ulong &&
              *buf as libc::c_int != ';' as i32 {
        buf = buf.offset(1);
        end = end.wrapping_sub(1)
    }
    if end == 0 as libc::c_int as libc::c_ulong ||
           end == 1 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    buf = buf.offset(1);
    end = end.wrapping_sub(1);
    /* It has to be a string so copy it. */
    copy =
        xmalloc(end.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
            *mut libc::c_char;
    memcpy(copy as *mut libc::c_void, buf as *const libc::c_void, end);
    *copy.offset(end as isize) = '\u{0}' as i32 as libc::c_char;
    /* Convert from base64. */
    needed =
        end.wrapping_div(4 as libc::c_int as
                             libc::c_ulong).wrapping_mul(3 as libc::c_int as
                                                             libc::c_ulong);
    out = xmalloc(needed) as *mut libc::c_char;
    outlen = __b64_pton(copy, out as *mut libc::c_uchar, len);
    if outlen == -(1 as libc::c_int) {
        free(out as *mut libc::c_void);
        free(copy as *mut libc::c_void);
        return 0 as libc::c_int
    }
    free(copy as *mut libc::c_void);
    /* Create a new paste buffer. */
    log_debug(b"%s: %.*s\x00" as *const u8 as *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"tty_keys_clipboard\x00")).as_ptr(),
              outlen, out);
    paste_add(0 as *const libc::c_char, out, outlen as size_t);
    return 0 as libc::c_int;
}
/*
 * Handle secondary device attributes input. Returns 0 for success, -1 for
 * failure, 1 for partial.
 */
unsafe extern "C" fn tty_keys_device_attributes(mut tty: *mut tty,
                                                mut buf: *const libc::c_char,
                                                mut len: size_t,
                                                mut size: *mut size_t)
 -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut i: u_int = 0;
    let mut n: u_int = 0 as libc::c_int as u_int;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: [libc::c_char; 32] =
        [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    *size = 0 as libc::c_int as size_t;
    if (*tty).flags & 0x100 as libc::c_int != 0 { return -(1 as libc::c_int) }
    /*
	 * First three bytes are always \033[>. Some older Terminal.app
	 * versions respond as for DA (\033[?) so accept and ignore that.
	 */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int !=
           '\u{1b}' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 1 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(1 as libc::c_int as isize) as libc::c_int != '[' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 2 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(2 as libc::c_int as isize) as libc::c_int != '>' as i32 &&
           *buf.offset(2 as libc::c_int as isize) as libc::c_int != '?' as i32
       {
        return -(1 as libc::c_int)
    }
    if len == 3 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    /* Copy the rest up to a 'c'. */
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[libc::c_char; 64]>() as
                   libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong) {
        if (3 as libc::c_int as libc::c_uint).wrapping_add(i) as libc::c_ulong
               == len {
            return 1 as libc::c_int
        }
        if *buf.offset((3 as libc::c_int as libc::c_uint).wrapping_add(i) as
                           isize) as libc::c_int == 'c' as i32 {
            break ;
        }
        tmp[i as usize] =
            *buf.offset((3 as libc::c_int as libc::c_uint).wrapping_add(i) as
                            isize);
        i = i.wrapping_add(1)
    }
    if i as libc::c_ulong ==
           (::std::mem::size_of::<[libc::c_char; 64]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
       {
        return -(1 as libc::c_int)
    }
    tmp[i as usize] = '\u{0}' as i32 as libc::c_char;
    *size = (4 as libc::c_int as libc::c_uint).wrapping_add(i) as size_t;
    /* Ignore DA response. */
    if *buf.offset(2 as libc::c_int as isize) as libc::c_int == '?' as i32 {
        return 0 as libc::c_int
    }
    /* Convert all arguments to numbers. */
    cp = tmp.as_mut_ptr();
    loop  {
        next = strsep(&mut cp, b";\x00" as *const u8 as *const libc::c_char);
        if next.is_null() { break ; }
        p[n as usize] =
            strtoul(next, &mut endptr, 10 as libc::c_int) as libc::c_char;
        if *endptr as libc::c_int != '\u{0}' as i32 {
            p[n as usize] = 0 as libc::c_int as libc::c_char
        }
        n = n.wrapping_add(1)
    }
    /* Add terminal features. */
    match p[0 as libc::c_int as usize] as libc::c_int {
        41 => {
            /* VT420 */
            tty_add_features(&mut (*c).term_features,
                             b"margins,rectfill\x00" as *const u8 as
                                 *const libc::c_char,
                             b",\x00" as *const u8 as *const libc::c_char);
        }
        77 => {
            /* mintty */
            tty_default_features(&mut (*c).term_features,
                                 b"mintty\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as libc::c_int as u_int);
        }
        84 => {
            /* tmux */
            tty_default_features(&mut (*c).term_features,
                                 b"tmux\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as libc::c_int as u_int);
        }
        85 => {
            /* rxvt-unicode */
            tty_default_features(&mut (*c).term_features,
                                 b"rxvt-unicode\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as libc::c_int as u_int);
        }
        _ => { }
    }
    log_debug(b"%s: received secondary DA %.*s\x00" as *const u8 as
                  *const libc::c_char, (*c).name, *size as libc::c_int, buf);
    tty_update_features(tty);
    (*tty).flags |= 0x100 as libc::c_int;
    return 0 as libc::c_int;
}
/*
 * Handle extended device attributes input. Returns 0 for success, -1 for
 * failure, 1 for partial.
 */
unsafe extern "C" fn tty_keys_extended_device_attributes(mut tty: *mut tty,
                                                         mut buf:
                                                             *const libc::c_char,
                                                         mut len: size_t,
                                                         mut size:
                                                             *mut size_t)
 -> libc::c_int {
    let mut c: *mut client = (*tty).client;
    let mut i: u_int = 0;
    let mut tmp: [libc::c_char; 128] = [0; 128];
    *size = 0 as libc::c_int as size_t;
    if (*tty).flags & 0x200 as libc::c_int != 0 { return -(1 as libc::c_int) }
    /* First four bytes are always \033P>|. */
    if *buf.offset(0 as libc::c_int as isize) as libc::c_int !=
           '\u{1b}' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 1 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(1 as libc::c_int as isize) as libc::c_int != 'P' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 2 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(2 as libc::c_int as isize) as libc::c_int != '>' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 3 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    if *buf.offset(3 as libc::c_int as isize) as libc::c_int != '|' as i32 {
        return -(1 as libc::c_int)
    }
    if len == 4 as libc::c_int as libc::c_ulong { return 1 as libc::c_int }
    /* Copy the rest up to a '\033\\'. */
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[libc::c_char; 128]>() as
                   libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong) {
        if (4 as libc::c_int as libc::c_uint).wrapping_add(i) as libc::c_ulong
               == len {
            return 1 as libc::c_int
        }
        if *buf.offset((4 as libc::c_int as
                            libc::c_uint).wrapping_add(i).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                           as isize) as libc::c_int == '\u{1b}' as i32 &&
               *buf.offset((4 as libc::c_int as libc::c_uint).wrapping_add(i)
                               as isize) as libc::c_int == '\\' as i32 {
            break ;
        }
        tmp[i as usize] =
            *buf.offset((4 as libc::c_int as libc::c_uint).wrapping_add(i) as
                            isize);
        i = i.wrapping_add(1)
    }
    if i as libc::c_ulong ==
           (::std::mem::size_of::<[libc::c_char; 128]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
       {
        return -(1 as libc::c_int)
    }
    tmp[i.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize] =
        '\u{0}' as i32 as libc::c_char;
    *size = (5 as libc::c_int as libc::c_uint).wrapping_add(i) as size_t;
    /* Add terminal features. */
    if strncmp(tmp.as_mut_ptr(),
               b"iTerm2 \x00" as *const u8 as *const libc::c_char,
               7 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        tty_default_features(&mut (*c).term_features,
                             b"iTerm2\x00" as *const u8 as
                                 *const libc::c_char,
                             0 as libc::c_int as u_int);
    } else if strncmp(tmp.as_mut_ptr(),
                      b"tmux \x00" as *const u8 as *const libc::c_char,
                      5 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        tty_default_features(&mut (*c).term_features,
                             b"tmux\x00" as *const u8 as *const libc::c_char,
                             0 as libc::c_int as u_int);
    } else if strncmp(tmp.as_mut_ptr(),
                      b"XTerm(\x00" as *const u8 as *const libc::c_char,
                      6 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        tty_default_features(&mut (*c).term_features,
                             b"XTerm\x00" as *const u8 as *const libc::c_char,
                             0 as libc::c_int as u_int);
    } else if strncmp(tmp.as_mut_ptr(),
                      b"mintty \x00" as *const u8 as *const libc::c_char,
                      7 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        tty_default_features(&mut (*c).term_features,
                             b"mintty\x00" as *const u8 as
                                 *const libc::c_char,
                             0 as libc::c_int as u_int);
    }
    log_debug(b"%s: received extended DA %.*s\x00" as *const u8 as
                  *const libc::c_char, (*c).name, *size as libc::c_int, buf);
    free((*c).term_type as *mut libc::c_void);
    (*c).term_type = xstrdup(tmp.as_mut_ptr());
    tty_update_features(tty);
    (*tty).flags |= 0x200 as libc::c_int;
    return 0 as libc::c_int;
}
