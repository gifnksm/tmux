use ::libc;
extern "C" {
    pub type screen_write_collect_line;
    pub type screen_sel;
    pub type screen_titles;
    pub type term;
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type environ;
    pub type options;
    pub type format_tree;
    pub type input_ctx;
    pub type cmds;
    pub type format_job_tree;
    pub type control_state;
    pub type cmdq_list;
    pub type tmuxpeer;
    pub type options_array_item;
    pub type options_entry;
    #[no_mangle]
    fn tigetflag(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tigetnum(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tigetstr(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn tparm(_: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn setupterm(_: *const libc::c_char, _: libc::c_int, _: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn del_curterm(_: *mut TERMINAL) -> libc::c_int;
    #[no_mangle]
    static mut cur_term: *mut TERMINAL;
    #[no_mangle]
    fn strnvis(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: size_t,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn strunvis(_: *mut libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut options;
    #[no_mangle]
    fn options_get_only(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    #[no_mangle]
    fn options_array_first(_: *mut options_entry) -> *mut options_array_item;
    #[no_mangle]
    fn options_array_next(_: *mut options_array_item) -> *mut options_array_item;
    #[no_mangle]
    fn options_array_item_value(_: *mut options_array_item) -> *mut options_value;
    #[no_mangle]
    fn tty_apply_features(_: *mut tty_term, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tty_add_features(_: *mut libc::c_int, _: *const libc::c_char, _: *const libc::c_char);
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type TERMINAL = term;
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
    pub evcb_callback:
        Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_short, _: *mut libc::c_void) -> ()>,
    pub evcb_selfcb:
        Option<unsafe extern "C" fn(_: *mut event_callback, _: *mut libc::c_void) -> ()>,
    pub evcb_evfinalize: Option<unsafe extern "C" fn(_: *mut event, _: *mut libc::c_void) -> ()>,
    pub evcb_cbfinalize:
        Option<unsafe extern "C" fn(_: *mut event_callback, _: *mut libc::c_void) -> ()>,
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
pub type bufferevent_event_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short, _: *mut libc::c_void) -> ()>;
pub type bufferevent_data_cb =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void) -> ()>;
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
pub type overlay_draw_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut screen_redraw_ctx) -> ()>;
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
pub type overlay_mode_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut u_int, _: *mut u_int) -> *mut screen>;
pub type overlay_check_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: u_int, _: u_int) -> libc::c_int>;
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
    pub formats: Option<unsafe extern "C" fn(_: *mut window_mode_entry, _: *mut format_tree) -> ()>,
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
pub type prompt_free_cb = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type prompt_input_cb = Option<
    unsafe extern "C" fn(
        _: *mut client,
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int,
>;
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
    pub mouse_drag_update: Option<unsafe extern "C" fn(_: *mut client, _: *mut mouse_event) -> ()>,
    pub mouse_drag_release: Option<unsafe extern "C" fn(_: *mut client, _: *mut mouse_event) -> ()>,
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
pub struct tty_code {
    pub type_0: tty_code_type,
    pub value: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_31 {
    pub string: *mut libc::c_char,
    pub number: libc::c_int,
    pub flag: libc::c_int,
}
pub type tty_code_type = libc::c_uint;
pub const TTYCODE_FLAG: tty_code_type = 3;
pub const TTYCODE_NUMBER: tty_code_type = 2;
pub const TTYCODE_STRING: tty_code_type = 1;
pub const TTYCODE_NONE: tty_code_type = 0;
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
pub struct tty_terms {
    pub lh_first: *mut tty_term,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term_code_entry {
    pub type_0: tty_code_type,
    pub name: *const libc::c_char,
}
#[no_mangle]
pub static mut tty_terms: tty_terms = {
    let mut init = tty_terms {
        lh_first: 0 as *const tty_term as *mut tty_term,
    };
    init
};
static mut tty_term_codes: [tty_term_code_entry; 225] = [
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"acsc\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"am\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"AX\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"bce\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"bel\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"blink\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"bold\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"civis\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"clear\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Clmg\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Cmg\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cnorm\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_NUMBER,
            name: b"colors\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Cr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Cs\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"csr\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cub\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cub1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cud\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cud1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cuf\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cuf1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cup\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cuu\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cuu1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"cvvis\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"dch\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"dch1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"dim\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"dl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"dl1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Dsbp\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Dseks\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Dsfcs\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Dsmg\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"E3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ech\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ed\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"el\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"el1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"enacs\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Enbp\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Eneks\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Enfcs\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Enmg\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"fsl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"home\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"hpa\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ich\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ich1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"il\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"il1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"indn\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"invis\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kcbt\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kcub1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kcud1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kcuf1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kcuu1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDC7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kdch1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kDN7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kend\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kEND7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf10\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf11\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf12\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf13\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf14\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf15\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf16\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf17\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf18\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf19\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf2\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf20\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf21\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf22\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf23\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf24\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf25\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf26\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf27\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf28\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf29\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf30\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf31\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf32\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf33\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf34\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf35\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf36\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf37\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf38\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf39\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf40\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf41\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf42\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf43\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf44\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf45\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf46\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf47\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf48\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf49\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf50\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf51\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf52\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf53\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf54\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf55\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf56\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf57\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf58\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf59\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf60\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf61\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf62\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf63\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf8\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kf9\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kHOM7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"khome\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kIC7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kich1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kind\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kLFT7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kmous\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"knp\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kNXT7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kpp\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kPRV7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kri\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kRIT7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP5\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP6\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"kUP7\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Ms\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ol\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"op\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"rev\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"RGB\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"ri\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"rin\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"rmacs\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"rmcup\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"rmkx\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Se\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"setab\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"setaf\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"setal\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"setrgbb\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"setrgbf\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Setulc\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"sgr0\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"sitm\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smacs\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smcup\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smkx\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Smol\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smso\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smul\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Smulx\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"smxx\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Ss\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"Sync\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"Tc\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"tsl\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_NUMBER,
            name: b"U8\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_STRING,
            name: b"vpa\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = tty_term_code_entry {
            type_0: TTYCODE_FLAG,
            name: b"XT\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn tty_term_ncodes() -> u_int {
    return (::std::mem::size_of::<[tty_term_code_entry; 225]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<tty_term_code_entry>() as libc::c_ulong)
        as u_int;
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2008 Nicholas Marriott <nicholas.marriott@gmail.com>
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
unsafe extern "C" fn tty_term_strip(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    static mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut len: size_t = 0;
    /* Ignore strings with no padding. */
    if strchr(s, '$' as i32).is_null() {
        return xstrdup(s);
    }
    len = 0 as libc::c_int as size_t;
    ptr = s;
    while *ptr as libc::c_int != '\u{0}' as i32 {
        if *ptr as libc::c_int == '$' as i32
            && *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '<' as i32
        {
            while *ptr as libc::c_int != '\u{0}' as i32 && *ptr as libc::c_int != '>' as i32 {
                ptr = ptr.offset(1)
            }
            if *ptr as libc::c_int == '>' as i32 {
                ptr = ptr.offset(1)
            }
            if *ptr as libc::c_int == '\u{0}' as i32 {
                break;
            }
        }
        let fresh0 = len;
        len = len.wrapping_add(1);
        buf[fresh0 as usize] = *ptr;
        if len
            == (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            break;
        }
        ptr = ptr.offset(1)
    }
    buf[len as usize] = '\u{0}' as i32 as libc::c_char;
    return xstrdup(buf.as_mut_ptr());
}
unsafe extern "C" fn tty_term_override_next(
    mut s: *const libc::c_char,
    mut offset: *mut size_t,
) -> *mut libc::c_char {
    static mut value: [libc::c_char; 8192] = [0; 8192];
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut at: size_t = *offset;
    if *s.offset(at as isize) as libc::c_int == '\u{0}' as i32 {
        return 0 as *mut libc::c_char;
    }
    while *s.offset(at as isize) as libc::c_int != '\u{0}' as i32 {
        if *s.offset(at as isize) as libc::c_int == ':' as i32 {
            if !(*s.offset(at.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == ':' as i32)
            {
                break;
            }
            let fresh1 = n;
            n = n.wrapping_add(1);
            value[fresh1 as usize] = ':' as i32 as libc::c_char;
            at = (at as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        } else {
            let fresh2 = n;
            n = n.wrapping_add(1);
            value[fresh2 as usize] = *s.offset(at as isize);
            at = at.wrapping_add(1)
        }
        if n == (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            return 0 as *mut libc::c_char;
        }
    }
    if *s.offset(at as isize) as libc::c_int != '\u{0}' as i32 {
        *offset = at.wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        *offset = at
    }
    value[n as usize] = '\u{0}' as i32 as libc::c_char;
    return value.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_apply(
    mut term: *mut tty_term,
    mut capabilities: *const libc::c_char,
    mut quiet: libc::c_int,
) {
    let mut ent: *const tty_term_code_entry = 0 as *const tty_term_code_entry;
    let mut code: *mut tty_code = 0 as *mut tty_code;
    let mut offset: size_t = 0 as libc::c_int as size_t;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = (*term).name;
    let mut i: u_int = 0;
    let mut n: libc::c_int = 0;
    let mut remove: libc::c_int = 0;
    loop {
        s = tty_term_override_next(capabilities, &mut offset);
        if s.is_null() {
            break;
        }
        if *s as libc::c_int == '\u{0}' as i32 {
            continue;
        }
        value = 0 as *mut libc::c_char;
        remove = 0 as libc::c_int;
        cp = strchr(s, '=' as i32);
        if !cp.is_null() {
            let fresh3 = cp;
            cp = cp.offset(1);
            *fresh3 = '\u{0}' as i32 as libc::c_char;
            value = xstrdup(cp);
            if strunvis(value, cp) == -(1 as libc::c_int) {
                free(value as *mut libc::c_void);
                value = xstrdup(cp)
            }
        } else if *s.offset(strlen(s).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            == '@' as i32
        {
            *s.offset(strlen(s).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
                '\u{0}' as i32 as libc::c_char;
            remove = 1 as libc::c_int
        } else {
            value = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
        }
        if quiet == 0 {
            if remove != 0 {
                log_debug(
                    b"%s override: %s@\x00" as *const u8 as *const libc::c_char,
                    name,
                    s,
                );
            } else if *value as libc::c_int == '\u{0}' as i32 {
                log_debug(
                    b"%s override: %s\x00" as *const u8 as *const libc::c_char,
                    name,
                    s,
                );
            } else {
                log_debug(
                    b"%s override: %s=%s\x00" as *const u8 as *const libc::c_char,
                    name,
                    s,
                    value,
                );
            }
        }
        i = 0 as libc::c_int as u_int;
        while i < tty_term_ncodes() {
            ent = &*tty_term_codes.as_ptr().offset(i as isize) as *const tty_term_code_entry;
            if !(strcmp(s, (*ent).name) != 0 as libc::c_int) {
                code = &mut *(*term).codes.offset(i as isize) as *mut tty_code;
                if remove != 0 {
                    (*code).type_0 = TTYCODE_NONE
                } else {
                    match (*ent).type_0 as libc::c_uint {
                        1 => {
                            if (*code).type_0 as libc::c_uint
                                == TTYCODE_STRING as libc::c_int as libc::c_uint
                            {
                                free((*code).value.string as *mut libc::c_void);
                            }
                            (*code).value.string = xstrdup(value);
                            (*code).type_0 = (*ent).type_0
                        }
                        2 => {
                            n = strtonum(
                                value,
                                0 as libc::c_int as libc::c_longlong,
                                2147483647 as libc::c_int as libc::c_longlong,
                                &mut errstr,
                            ) as libc::c_int;
                            if errstr.is_null() {
                                (*code).value.number = n;
                                (*code).type_0 = (*ent).type_0
                            }
                        }
                        3 => {
                            (*code).value.flag = 1 as libc::c_int;
                            (*code).type_0 = (*ent).type_0
                        }
                        0 | _ => {}
                    }
                }
            }
            i = i.wrapping_add(1)
        }
        free(value as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_apply_overrides(mut term: *mut tty_term) {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    let mut ov: *mut options_value = 0 as *mut options_value;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut offset: size_t = 0;
    let mut first: *mut libc::c_char = 0 as *mut libc::c_char;
    o = options_get_only(
        global_options,
        b"terminal-overrides\x00" as *const u8 as *const libc::c_char,
    );
    a = options_array_first(o);
    while !a.is_null() {
        ov = options_array_item_value(a);
        s = (*ov).string;
        offset = 0 as libc::c_int as size_t;
        first = tty_term_override_next(s, &mut offset);
        if !first.is_null() && fnmatch(first, (*term).name, 0 as libc::c_int) == 0 as libc::c_int {
            tty_term_apply(term, s.offset(offset as isize), 0 as libc::c_int);
        }
        a = options_array_next(a)
    }
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_create(
    mut tty: *mut tty,
    mut name: *mut libc::c_char,
    mut feat: *mut libc::c_int,
    mut fd: libc::c_int,
    mut cause: *mut *mut libc::c_char,
) -> *mut tty_term {
    let mut term: *mut tty_term = 0 as *mut tty_term;
    let mut ent: *const tty_term_code_entry = 0 as *const tty_term_code_entry;
    let mut code: *mut tty_code = 0 as *mut tty_code;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    let mut ov: *mut options_value = 0 as *mut options_value;
    let mut i: u_int = 0;
    let mut n: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut acs: *const libc::c_char = 0 as *const libc::c_char;
    let mut offset: size_t = 0;
    let mut first: *mut libc::c_char = 0 as *mut libc::c_char;
    log_debug(
        b"adding term %s\x00" as *const u8 as *const libc::c_char,
        name,
    );
    term = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<tty_term>() as libc::c_ulong,
    ) as *mut tty_term;
    (*term).tty = tty;
    (*term).name = xstrdup(name);
    (*term).codes = xcalloc(
        tty_term_ncodes() as size_t,
        ::std::mem::size_of::<tty_code>() as libc::c_ulong,
    ) as *mut tty_code;
    (*term).entry.le_next = tty_terms.lh_first;
    if !(*term).entry.le_next.is_null() {
        (*tty_terms.lh_first).entry.le_prev = &mut (*term).entry.le_next
    }
    tty_terms.lh_first = term;
    (*term).entry.le_prev = &mut tty_terms.lh_first;
    /* Set up curses terminal. */
    if setupterm(name, fd, &mut error) != 0 as libc::c_int {
        match error {
            1 => {
                xasprintf(
                    cause,
                    b"can\'t use hardcopy terminal: %s\x00" as *const u8 as *const libc::c_char,
                    name,
                );
            }
            0 => {
                xasprintf(
                    cause,
                    b"missing or unsuitable terminal: %s\x00" as *const u8 as *const libc::c_char,
                    name,
                );
            }
            -1 => {
                xasprintf(
                    cause,
                    b"can\'t find terminfo database\x00" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                xasprintf(
                    cause,
                    b"unknown error\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
    } else {
        /* Fill in codes. */
        i = 0 as libc::c_int as u_int;
        while i < tty_term_ncodes() {
            ent = &*tty_term_codes.as_ptr().offset(i as isize) as *const tty_term_code_entry;
            code = &mut *(*term).codes.offset(i as isize) as *mut tty_code;
            (*code).type_0 = TTYCODE_NONE;
            match (*ent).type_0 as libc::c_uint {
                1 => {
                    s = tigetstr((*ent).name as *mut libc::c_char);
                    if !(s.is_null()
                        || s == -(1 as libc::c_int) as *mut libc::c_char as *const libc::c_char)
                    {
                        (*code).type_0 = TTYCODE_STRING;
                        (*code).value.string = tty_term_strip(s)
                    }
                }
                2 => {
                    n = tigetnum((*ent).name as *mut libc::c_char);
                    if !(n == -(1 as libc::c_int) || n == -(2 as libc::c_int)) {
                        (*code).type_0 = TTYCODE_NUMBER;
                        (*code).value.number = n
                    }
                }
                3 => {
                    n = tigetflag((*ent).name as *mut libc::c_char);
                    if !(n == -(1 as libc::c_int)) {
                        (*code).type_0 = TTYCODE_FLAG;
                        (*code).value.flag = n
                    }
                }
                0 | _ => {}
            }
            i = i.wrapping_add(1)
        }
        /* Apply terminal features. */
        o = options_get_only(
            global_options,
            b"terminal-features\x00" as *const u8 as *const libc::c_char,
        );
        a = options_array_first(o);
        while !a.is_null() {
            ov = options_array_item_value(a);
            s = (*ov).string;
            offset = 0 as libc::c_int as size_t;
            first = tty_term_override_next(s, &mut offset);
            if !first.is_null()
                && fnmatch(first, (*term).name, 0 as libc::c_int) == 0 as libc::c_int
            {
                tty_add_features(
                    feat,
                    s.offset(offset as isize),
                    b":\x00" as *const u8 as *const libc::c_char,
                );
            }
            a = options_array_next(a)
        }
        /* Delete curses data. */
        del_curterm(cur_term);
        /* Apply overrides so any capabilities used for features are changed. */
        tty_term_apply_overrides(term);
        /* These are always required. */
        if tty_term_has(term, TTYC_CLEAR) == 0 {
            xasprintf(
                cause,
                b"terminal does not support clear\x00" as *const u8 as *const libc::c_char,
            );
        } else if tty_term_has(term, TTYC_CUP) == 0 {
            xasprintf(
                cause,
                b"terminal does not support cup\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            /*
             * If TERM has XT or clear starts with CSI then it is safe to assume
             * the terminal is derived from the VT100. This controls whether device
             * attributes requests are sent to get more information.
             *
             * This is a bit of a hack but there aren't that many alternatives.
             * Worst case tmux will just fall back to using whatever terminfo(5)
             * says without trying to correct anything that is missing.
             *
             * Also add few features that VT100-like terminals should either
             * support or safely ignore.
             */
            s = tty_term_string(term, TTYC_CLEAR);
            if tty_term_flag(term, TTYC_XT) != 0
                || strncmp(
                    s,
                    b"\x1b[\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                (*term).flags |= 0x20 as libc::c_int;
                tty_add_features(
                    feat,
                    b"bpaste,focus,title\x00" as *const u8 as *const libc::c_char,
                    b",\x00" as *const u8 as *const libc::c_char,
                );
            }
            /* Add RGB feature if terminal has RGB colours. */
            if (tty_term_flag(term, TTYC_TC) != 0 || tty_term_has(term, TTYC_RGB) != 0)
                && (tty_term_has(term, TTYC_SETRGBF) == 0 || tty_term_has(term, TTYC_SETRGBB) == 0)
            {
                tty_add_features(
                    feat,
                    b"RGB\x00" as *const u8 as *const libc::c_char,
                    b",\x00" as *const u8 as *const libc::c_char,
                );
            }
            if tty_term_has(term, TTYC_SETRGBF) != 0 && tty_term_has(term, TTYC_SETRGBB) != 0 {
                (*term).flags |= 0x10 as libc::c_int
            }
            /* Apply the features and overrides again. */
            tty_apply_features(term, *feat);
            tty_term_apply_overrides(term);
            /*
             * Terminals without am (auto right margin) wrap at at $COLUMNS - 1
             * rather than $COLUMNS (the cursor can never be beyond $COLUMNS - 1).
             *
             * Terminals without xenl (eat newline glitch) ignore a newline beyond
             * the right edge of the terminal, but tmux doesn't care about this -
             * it always uses absolute only moves the cursor with a newline when
             * also sending a linefeed.
             *
             * This is irritating, most notably because it is painful to write to
             * the very bottom-right of the screen without scrolling.
             *
             * Flag the terminal here and apply some workarounds in other places to
             * do the best possible.
             */
            if tty_term_flag(term, TTYC_AM) == 0 {
                (*term).flags |= 0x2 as libc::c_int
            }
            /* Generate ACS table. If none is present, use nearest ASCII. */
            memset(
                (*term).acs.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[[libc::c_char; 2]; 256]>() as libc::c_ulong,
            );
            if tty_term_has(term, TTYC_ACSC) != 0 {
                acs = tty_term_string(term, TTYC_ACSC)
            } else {
                acs = b"a#j+k+l+m+n+o-p-q-r-s-t+u+v+w+x|y<z>~.\x00" as *const u8
                    as *const libc::c_char
            }
            while *acs.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
                && *acs.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
            {
                (*term).acs[*acs.offset(0 as libc::c_int as isize) as u_char as usize]
                    [0 as libc::c_int as usize] = *acs.offset(1 as libc::c_int as isize);
                acs = acs.offset(2 as libc::c_int as isize)
            }
            /* Log the capabilities. */
            i = 0 as libc::c_int as u_int;
            while i < tty_term_ncodes() {
                log_debug(
                    b"%s%s\x00" as *const u8 as *const libc::c_char,
                    name,
                    tty_term_describe(term, i as tty_code_code),
                );
                i = i.wrapping_add(1)
            }
            return term;
        }
    }
    tty_term_free(term);
    return 0 as *mut tty_term;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_free(mut term: *mut tty_term) {
    let mut i: u_int = 0;
    log_debug(
        b"removing term %s\x00" as *const u8 as *const libc::c_char,
        (*term).name,
    );
    i = 0 as libc::c_int as u_int;
    while i < tty_term_ncodes() {
        if (*(*term).codes.offset(i as isize)).type_0 as libc::c_uint
            == TTYCODE_STRING as libc::c_int as libc::c_uint
        {
            free((*(*term).codes.offset(i as isize)).value.string as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    free((*term).codes as *mut libc::c_void);
    if !(*term).entry.le_next.is_null() {
        (*(*term).entry.le_next).entry.le_prev = (*term).entry.le_prev
    }
    *(*term).entry.le_prev = (*term).entry.le_next;
    free((*term).name as *mut libc::c_void);
    free(term as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_has(
    mut term: *mut tty_term,
    mut code: tty_code_code,
) -> libc::c_int {
    return ((*(*term).codes.offset(code as isize)).type_0 as libc::c_uint
        != TTYCODE_NONE as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string(
    mut term: *mut tty_term,
    mut code: tty_code_code,
) -> *const libc::c_char {
    if tty_term_has(term, code) == 0 {
        return b"\x00" as *const u8 as *const libc::c_char;
    }
    if (*(*term).codes.offset(code as isize)).type_0 as libc::c_uint
        != TTYCODE_STRING as libc::c_int as libc::c_uint
    {
        fatalx(
            b"not a string: %d\x00" as *const u8 as *const libc::c_char,
            code as libc::c_uint,
        );
    }
    return (*(*term).codes.offset(code as isize)).value.string;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string1(
    mut term: *mut tty_term,
    mut code: tty_code_code,
    mut a: libc::c_int,
) -> *const libc::c_char {
    return tparm(
        tty_term_string(term, code) as *mut libc::c_char,
        a,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string2(
    mut term: *mut tty_term,
    mut code: tty_code_code,
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> *const libc::c_char {
    return tparm(
        tty_term_string(term, code) as *mut libc::c_char,
        a,
        b,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_string3(
    mut term: *mut tty_term,
    mut code: tty_code_code,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) -> *const libc::c_char {
    return tparm(
        tty_term_string(term, code) as *mut libc::c_char,
        a,
        b,
        c,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_ptr1(
    mut term: *mut tty_term,
    mut code: tty_code_code,
    mut a: *const libc::c_void,
) -> *const libc::c_char {
    return tparm(
        tty_term_string(term, code) as *mut libc::c_char,
        a as libc::c_long,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_ptr2(
    mut term: *mut tty_term,
    mut code: tty_code_code,
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> *const libc::c_char {
    return tparm(
        tty_term_string(term, code) as *mut libc::c_char,
        a as libc::c_long,
        b as libc::c_long,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_number(
    mut term: *mut tty_term,
    mut code: tty_code_code,
) -> libc::c_int {
    if tty_term_has(term, code) == 0 {
        return 0 as libc::c_int;
    }
    if (*(*term).codes.offset(code as isize)).type_0 as libc::c_uint
        != TTYCODE_NUMBER as libc::c_int as libc::c_uint
    {
        fatalx(
            b"not a number: %d\x00" as *const u8 as *const libc::c_char,
            code as libc::c_uint,
        );
    }
    return (*(*term).codes.offset(code as isize)).value.number;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_flag(
    mut term: *mut tty_term,
    mut code: tty_code_code,
) -> libc::c_int {
    if tty_term_has(term, code) == 0 {
        return 0 as libc::c_int;
    }
    if (*(*term).codes.offset(code as isize)).type_0 as libc::c_uint
        != TTYCODE_FLAG as libc::c_int as libc::c_uint
    {
        fatalx(
            b"not a flag: %d\x00" as *const u8 as *const libc::c_char,
            code as libc::c_uint,
        );
    }
    return (*(*term).codes.offset(code as isize)).value.flag;
}
#[no_mangle]
pub unsafe extern "C" fn tty_term_describe(
    mut term: *mut tty_term,
    mut code: tty_code_code,
) -> *const libc::c_char {
    static mut s: [libc::c_char; 256] = [0; 256];
    let mut out: [libc::c_char; 128] = [0; 128];
    match (*(*term).codes.offset(code as isize)).type_0 as libc::c_uint {
        0 => {
            xsnprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%4u: %s: [missing]\x00" as *const u8 as *const libc::c_char,
                code as libc::c_uint,
                tty_term_codes[code as usize].name,
            );
        }
        1 => {
            strnvis(
                out.as_mut_ptr(),
                (*(*term).codes.offset(code as isize)).value.string,
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int,
            );
            xsnprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%4u: %s: (string) %s\x00" as *const u8 as *const libc::c_char,
                code as libc::c_uint,
                tty_term_codes[code as usize].name,
                out.as_mut_ptr(),
            );
        }
        2 => {
            xsnprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%4u: %s: (number) %d\x00" as *const u8 as *const libc::c_char,
                code as libc::c_uint,
                tty_term_codes[code as usize].name,
                (*(*term).codes.offset(code as isize)).value.number,
            );
        }
        3 => {
            xsnprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"%4u: %s: (flag) %s\x00" as *const u8 as *const libc::c_char,
                code as libc::c_uint,
                tty_term_codes[code as usize].name,
                if (*(*term).codes.offset(code as isize)).value.flag != 0 {
                    b"true\x00" as *const u8 as *const libc::c_char
                } else {
                    b"false\x00" as *const u8 as *const libc::c_char
                },
            );
        }
        _ => {}
    }
    return s.as_mut_ptr();
}
