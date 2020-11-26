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
    pub type cmdq_item;
    pub type tmuxproc;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn socketpair(__domain: libc::c_int, __type: libc::c_int,
                  __protocol: libc::c_int, __fds: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn killpg(__pgrp: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigprocmask(__how: libc::c_int, __set: *const sigset_t,
                   __oset: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    fn bufferevent_free(bufev: *mut bufferevent);
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn bufferevent_get_output(bufev: *mut bufferevent) -> *mut evbuffer;
    #[no_mangle]
    fn bufferevent_enable(bufev: *mut bufferevent, event: libc::c_short)
     -> libc::c_int;
    #[no_mangle]
    fn bufferevent_disable(bufev: *mut bufferevent, event: libc::c_short)
     -> libc::c_int;
    #[no_mangle]
    fn bufferevent_new(fd: libc::c_int, readcb: bufferevent_data_cb,
                       writecb: bufferevent_data_cb,
                       errorcb: bufferevent_event_cb,
                       cbarg: *mut libc::c_void) -> *mut bufferevent;
    #[no_mangle]
    fn closefrom(_: libc::c_int);
    #[no_mangle]
    fn fdforkpty(_: libc::c_int, _: *mut libc::c_int, _: *mut libc::c_char,
                 _: *mut termios, _: *mut winsize) -> pid_t;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    static mut ptm_fd: libc::c_int;
    #[no_mangle]
    fn setblocking(_: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn find_home() -> *const libc::c_char;
    #[no_mangle]
    fn proc_clear_signals(_: *mut tmuxproc, _: libc::c_int);
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn environ_free(_: *mut environ);
    #[no_mangle]
    fn environ_push(_: *mut environ);
    #[no_mangle]
    static mut server_proc: *mut tmuxproc;
    #[no_mangle]
    fn environ_for_session(_: *mut session, _: libc::c_int) -> *mut environ;
    #[no_mangle]
    fn cmdq_print(_: *mut cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, _: ...) -> !;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed = 2;
pub const SHUT_WR: C2RustUnnamed = 1;
pub const SHUT_RD: C2RustUnnamed = 0;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
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
/* A single job. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct job {
    pub state: C2RustUnnamed_34,
    pub flags: libc::c_int,
    pub cmd: *mut libc::c_char,
    pub pid: pid_t,
    pub status: libc::c_int,
    pub fd: libc::c_int,
    pub event: *mut bufferevent,
    pub updatecb: job_update_cb,
    pub completecb: job_complete_cb,
    pub freecb: job_free_cb,
    pub data: *mut libc::c_void,
    pub entry: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub le_next: *mut job,
    pub le_prev: *mut *mut job,
}
pub type job_free_cb
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type C2RustUnnamed_34 = libc::c_uint;
pub const JOB_CLOSED: C2RustUnnamed_34 = 2;
pub const JOB_DEAD: C2RustUnnamed_34 = 1;
pub const JOB_RUNNING: C2RustUnnamed_34 = 0;
/* All jobs list. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct joblist {
    pub lh_first: *mut job,
}
static mut all_jobs: joblist =
    { let mut init = joblist{lh_first: 0 as *const job as *mut job,}; init };
/* Start a job running, if it isn't already. */
#[no_mangle]
pub unsafe extern "C" fn job_run(mut cmd: *const libc::c_char,
                                 mut s: *mut session,
                                 mut cwd: *const libc::c_char,
                                 mut updatecb: job_update_cb,
                                 mut completecb: job_complete_cb,
                                 mut freecb: job_free_cb,
                                 mut data: *mut libc::c_void,
                                 mut flags: libc::c_int, mut sx: libc::c_int,
                                 mut sy: libc::c_int) -> *mut job {
    let mut current_block: u64;
    let mut job: *mut job = 0 as *mut job;
    let mut env: *mut environ = 0 as *mut environ;
    let mut pid: pid_t = 0;
    let mut nullfd: libc::c_int = 0;
    let mut out: [libc::c_int; 2] = [0; 2];
    let mut master: libc::c_int = 0;
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut set: sigset_t = sigset_t{__val: [0; 16],};
    let mut oldset: sigset_t = sigset_t{__val: [0; 16],};
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    /*
	 * Do not set TERM during .tmux.conf, it is nice to be able to use
	 * if-shell to decide on default-terminal based on outside TERM.
	 */
    env = environ_for_session(s, (cfg_finished == 0) as libc::c_int);
    sigfillset(&mut set);
    sigprocmask(0 as libc::c_int, &mut set, &mut oldset);
    if flags & 0x4 as libc::c_int != 0 {
        memset(&mut ws as *mut winsize as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<winsize>() as libc::c_ulong);
        ws.ws_col = sx as libc::c_ushort;
        ws.ws_row = sy as libc::c_ushort;
        pid =
            fdforkpty(ptm_fd, &mut master, 0 as *mut libc::c_char,
                      0 as *mut termios, &mut ws);
        current_block = 12599329904712511516;
    } else if socketpair(1 as libc::c_int, SOCK_STREAM as libc::c_int,
                         0 as libc::c_int, out.as_mut_ptr()) !=
                  0 as libc::c_int {
        current_block = 972739748014652499;
    } else { pid = fork(); current_block = 12599329904712511516; }
    match current_block {
        12599329904712511516 => {
            log_debug(b"%s: cmd=%s, cwd=%s\x00" as *const u8 as
                          *const libc::c_char,
                      (*::std::mem::transmute::<&[u8; 8],
                                                &[libc::c_char; 8]>(b"job_run\x00")).as_ptr(),
                      cmd,
                      if cwd.is_null() {
                          b"\x00" as *const u8 as *const libc::c_char
                      } else { cwd });
            match pid {
                -1 => {
                    if !flags & 0x4 as libc::c_int != 0 {
                        close(out[0 as libc::c_int as usize]);
                        close(out[1 as libc::c_int as usize]);
                    }
                }
                0 => {
                    proc_clear_signals(server_proc, 1 as libc::c_int);
                    sigprocmask(2 as libc::c_int, &mut oldset,
                                0 as *mut sigset_t);
                    if cwd.is_null() || chdir(cwd) != 0 as libc::c_int {
                        home = find_home();
                        if home.is_null() || chdir(home) != 0 as libc::c_int {
                            chdir(b"/\x00" as *const u8 as
                                      *const libc::c_char);
                        }
                    }
                    environ_push(env);
                    environ_free(env);
                    if !flags & 0x4 as libc::c_int != 0 {
                        if dup2(out[1 as libc::c_int as usize],
                                0 as libc::c_int) == -(1 as libc::c_int) {
                            fatal(b"dup2 failed\x00" as *const u8 as
                                      *const libc::c_char);
                        }
                        if dup2(out[1 as libc::c_int as usize],
                                1 as libc::c_int) == -(1 as libc::c_int) {
                            fatal(b"dup2 failed\x00" as *const u8 as
                                      *const libc::c_char);
                        }
                        if out[1 as libc::c_int as usize] != 0 as libc::c_int
                               &&
                               out[1 as libc::c_int as usize] !=
                                   1 as libc::c_int {
                            close(out[1 as libc::c_int as usize]);
                        }
                        close(out[0 as libc::c_int as usize]);
                        nullfd =
                            open(b"/dev/null\x00" as *const u8 as
                                     *const libc::c_char, 0o2 as libc::c_int,
                                 0 as libc::c_int);
                        if nullfd == -(1 as libc::c_int) {
                            fatal(b"open failed\x00" as *const u8 as
                                      *const libc::c_char);
                        }
                        if dup2(nullfd, 2 as libc::c_int) ==
                               -(1 as libc::c_int) {
                            fatal(b"dup2 failed\x00" as *const u8 as
                                      *const libc::c_char);
                        }
                        if nullfd != 2 as libc::c_int { close(nullfd); }
                    }
                    closefrom(2 as libc::c_int + 1 as libc::c_int);
                    execl(b"/bin/sh\x00" as *const u8 as *const libc::c_char,
                          b"sh\x00" as *const u8 as *const libc::c_char,
                          b"-c\x00" as *const u8 as *const libc::c_char, cmd,
                          0 as *mut libc::c_void as *mut libc::c_char);
                    fatal(b"execl failed\x00" as *const u8 as
                              *const libc::c_char);
                }
                _ => {
                    sigprocmask(2 as libc::c_int, &mut oldset,
                                0 as *mut sigset_t);
                    environ_free(env);
                    job =
                        xmalloc(::std::mem::size_of::<job>() as libc::c_ulong)
                            as *mut job;
                    (*job).state = JOB_RUNNING;
                    (*job).flags = flags;
                    (*job).cmd = xstrdup(cmd);
                    (*job).pid = pid;
                    (*job).status = 0 as libc::c_int;
                    (*job).entry.le_next = all_jobs.lh_first;
                    if !(*job).entry.le_next.is_null() {
                        (*all_jobs.lh_first).entry.le_prev =
                            &mut (*job).entry.le_next
                    }
                    all_jobs.lh_first = job;
                    (*job).entry.le_prev = &mut all_jobs.lh_first;
                    (*job).updatecb = updatecb;
                    (*job).completecb = completecb;
                    (*job).freecb = freecb;
                    (*job).data = data;
                    if !flags & 0x4 as libc::c_int != 0 {
                        close(out[1 as libc::c_int as usize]);
                        (*job).fd = out[0 as libc::c_int as usize]
                    } else { (*job).fd = master }
                    setblocking((*job).fd, 0 as libc::c_int);
                    (*job).event =
                        bufferevent_new((*job).fd,
                                        Some(job_read_callback as
                                                 unsafe extern "C" fn(_:
                                                                          *mut bufferevent,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> ()),
                                        Some(job_write_callback as
                                                 unsafe extern "C" fn(_:
                                                                          *mut bufferevent,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> ()),
                                        Some(job_error_callback as
                                                 unsafe extern "C" fn(_:
                                                                          *mut bufferevent,
                                                                      _:
                                                                          libc::c_short,
                                                                      _:
                                                                          *mut libc::c_void)
                                                     -> ()),
                                        job as *mut libc::c_void);
                    if (*job).event.is_null() {
                        fatalx(b"out of memory\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    bufferevent_enable((*job).event,
                                       (0x2 as libc::c_int |
                                            0x4 as libc::c_int) as
                                           libc::c_short);
                    log_debug(b"run job %p: %s, pid %ld\x00" as *const u8 as
                                  *const libc::c_char, job, (*job).cmd,
                              (*job).pid as libc::c_long);
                    return job
                }
            }
        }
        _ => { }
    }
    sigprocmask(2 as libc::c_int, &mut oldset, 0 as *mut sigset_t);
    environ_free(env);
    return 0 as *mut job;
}
/* Kill and free an individual job. */
#[no_mangle]
pub unsafe extern "C" fn job_free(mut job: *mut job) {
    log_debug(b"free job %p: %s\x00" as *const u8 as *const libc::c_char, job,
              (*job).cmd);
    if !(*job).entry.le_next.is_null() {
        (*(*job).entry.le_next).entry.le_prev = (*job).entry.le_prev
    }
    *(*job).entry.le_prev = (*job).entry.le_next;
    free((*job).cmd as *mut libc::c_void);
    if (*job).freecb.is_some() && !(*job).data.is_null() {
        (*job).freecb.expect("non-null function pointer")((*job).data);
    }
    if (*job).pid != -(1 as libc::c_int) {
        kill((*job).pid, 15 as libc::c_int);
    }
    if !(*job).event.is_null() { bufferevent_free((*job).event); }
    if (*job).fd != -(1 as libc::c_int) { close((*job).fd); }
    free(job as *mut libc::c_void);
}
/* Resize job. */
#[no_mangle]
pub unsafe extern "C" fn job_resize(mut job: *mut job, mut sx: u_int,
                                    mut sy: u_int) {
    let mut ws: winsize =
        winsize{ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0,};
    if (*job).fd == -(1 as libc::c_int) ||
           !(*job).flags & 0x4 as libc::c_int != 0 {
        return
    }
    log_debug(b"resize job %p: %ux%u\x00" as *const u8 as *const libc::c_char,
              job, sx, sy);
    memset(&mut ws as *mut winsize as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<winsize>() as libc::c_ulong);
    ws.ws_col = sx as libc::c_ushort;
    ws.ws_row = sy as libc::c_ushort;
    if ioctl((*job).fd, 0x5414 as libc::c_int as libc::c_ulong,
             &mut ws as *mut winsize) == -(1 as libc::c_int) {
        fatal(b"ioctl failed\x00" as *const u8 as *const libc::c_char);
    };
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2009 Nicholas Marriott <nicholas.marriott@gmail.com>
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
 * Job scheduling. Run queued commands in the background and record their
 * output.
 */
/* Job buffer read callback. */
unsafe extern "C" fn job_read_callback(mut bufev: *mut bufferevent,
                                       mut data: *mut libc::c_void) {
    let mut job: *mut job = data as *mut job;
    if (*job).updatecb.is_some() {
        (*job).updatecb.expect("non-null function pointer")(job);
    };
}
/*
 * Job buffer write callback. Fired when the buffer falls below watermark
 * (default is empty). If all the data has been written, disable the write
 * event.
 */
unsafe extern "C" fn job_write_callback(mut bufev: *mut bufferevent,
                                        mut data: *mut libc::c_void) {
    let mut job: *mut job = data as *mut job;
    let mut len: size_t =
        evbuffer_get_length(bufferevent_get_output((*job).event));
    log_debug(b"job write %p: %s, pid %ld, output left %zu\x00" as *const u8
                  as *const libc::c_char, job, (*job).cmd,
              (*job).pid as libc::c_long, len);
    if len == 0 as libc::c_int as libc::c_ulong &&
           !(*job).flags & 0x2 as libc::c_int != 0 {
        shutdown((*job).fd, SHUT_WR as libc::c_int);
        bufferevent_disable((*job).event,
                            0x4 as libc::c_int as libc::c_short);
    };
}
/* Job buffer error callback. */
unsafe extern "C" fn job_error_callback(mut bufev: *mut bufferevent,
                                        mut events: libc::c_short,
                                        mut data: *mut libc::c_void) {
    let mut job: *mut job = data as *mut job;
    log_debug(b"job error %p: %s, pid %ld\x00" as *const u8 as
                  *const libc::c_char, job, (*job).cmd,
              (*job).pid as libc::c_long);
    if (*job).state as libc::c_uint == JOB_DEAD as libc::c_int as libc::c_uint
       {
        if (*job).completecb.is_some() {
            (*job).completecb.expect("non-null function pointer")(job);
        }
        job_free(job);
    } else {
        bufferevent_disable((*job).event,
                            0x2 as libc::c_int as libc::c_short);
        (*job).state = JOB_CLOSED
    };
}
/* Job died (waitpid() returned its pid). */
#[no_mangle]
pub unsafe extern "C" fn job_check_died(mut pid: pid_t,
                                        mut status: libc::c_int) {
    let mut job: *mut job = 0 as *mut job;
    job = all_jobs.lh_first;
    while !job.is_null() {
        if pid == (*job).pid { break ; }
        job = (*job).entry.le_next
    }
    if job.is_null() { return }
    if status & 0xff as libc::c_int == 0x7f as libc::c_int {
        if (status & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
               21 as libc::c_int ||
               (status & 0xff00 as libc::c_int) >> 8 as libc::c_int ==
                   22 as libc::c_int {
            return
        }
        killpg((*job).pid, 18 as libc::c_int);
        return
    }
    log_debug(b"job died %p: %s, pid %ld\x00" as *const u8 as
                  *const libc::c_char, job, (*job).cmd,
              (*job).pid as libc::c_long);
    (*job).status = status;
    if (*job).state as libc::c_uint ==
           JOB_CLOSED as libc::c_int as libc::c_uint {
        if (*job).completecb.is_some() {
            (*job).completecb.expect("non-null function pointer")(job);
        }
        job_free(job);
    } else { (*job).pid = -(1 as libc::c_int); (*job).state = JOB_DEAD };
}
/* Get job status. */
#[no_mangle]
pub unsafe extern "C" fn job_get_status(mut job: *mut job) -> libc::c_int {
    return (*job).status;
}
/* Get job data. */
#[no_mangle]
pub unsafe extern "C" fn job_get_data(mut job: *mut job)
 -> *mut libc::c_void {
    return (*job).data;
}
/* Get job event. */
#[no_mangle]
pub unsafe extern "C" fn job_get_event(mut job: *mut job)
 -> *mut bufferevent {
    return (*job).event;
}
/* Kill all jobs. */
#[no_mangle]
pub unsafe extern "C" fn job_kill_all() {
    let mut job: *mut job = 0 as *mut job;
    job = all_jobs.lh_first;
    while !job.is_null() {
        if (*job).pid != -(1 as libc::c_int) {
            kill((*job).pid, 15 as libc::c_int);
        }
        job = (*job).entry.le_next
    };
}
/* Are any jobs still running? */
#[no_mangle]
pub unsafe extern "C" fn job_still_running() -> libc::c_int {
    let mut job: *mut job = 0 as *mut job;
    job = all_jobs.lh_first;
    while !job.is_null() {
        if !(*job).flags & 0x1 as libc::c_int != 0 &&
               (*job).state as libc::c_uint ==
                   JOB_RUNNING as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
        job = (*job).entry.le_next
    }
    return 0 as libc::c_int;
}
/* Print job summary. */
#[no_mangle]
pub unsafe extern "C" fn job_print_summary(mut item: *mut cmdq_item,
                                           mut blank: libc::c_int) {
    let mut job: *mut job = 0 as *mut job;
    let mut n: u_int = 0 as libc::c_int as u_int;
    job = all_jobs.lh_first;
    while !job.is_null() {
        if blank != 0 {
            cmdq_print(item, b"%s\x00" as *const u8 as *const libc::c_char,
                       b"\x00" as *const u8 as *const libc::c_char);
            blank = 0 as libc::c_int
        }
        cmdq_print(item,
                   b"Job %u: %s [fd=%d, pid=%ld, status=%d]\x00" as *const u8
                       as *const libc::c_char, n, (*job).cmd, (*job).fd,
                   (*job).pid as libc::c_long, (*job).status);
        n = n.wrapping_add(1);
        job = (*job).entry.le_next
    };
}
