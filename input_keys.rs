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
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn bufferevent_write(bufev: *mut bufferevent, data: *const libc::c_void,
                         size: size_t) -> libc::c_int;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut options;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char)
     -> libc::c_longlong;
    #[no_mangle]
    fn cmd_mouse_at(_: *mut window_pane, _: *mut mouse_event, _: *mut u_int,
                    _: *mut u_int, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn utf8_to_data(_: utf8_char, _: *mut utf8_data);
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ev_io: C2RustUnnamed_2,
    pub ev_signal: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ev_signal_next: C2RustUnnamed_1,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ev_io_next: C2RustUnnamed_3,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ev_next_with_common_timeout: C2RustUnnamed_5,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_7,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_6,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
pub struct C2RustUnnamed_7 {
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
    pub entry: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
    pub entry: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
    pub c2rust_unnamed: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub offset: u_int,
    pub data: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
pub type C2RustUnnamed_32 = libc::c_ulong;
pub const KEYC_KP_PERIOD: C2RustUnnamed_32 = 68719476927;
pub const KEYC_KP_ZERO: C2RustUnnamed_32 = 68719476926;
pub const KEYC_KP_ENTER: C2RustUnnamed_32 = 68719476925;
pub const KEYC_KP_THREE: C2RustUnnamed_32 = 68719476924;
pub const KEYC_KP_TWO: C2RustUnnamed_32 = 68719476923;
pub const KEYC_KP_ONE: C2RustUnnamed_32 = 68719476922;
pub const KEYC_KP_SIX: C2RustUnnamed_32 = 68719476921;
pub const KEYC_KP_FIVE: C2RustUnnamed_32 = 68719476920;
pub const KEYC_KP_FOUR: C2RustUnnamed_32 = 68719476919;
pub const KEYC_KP_PLUS: C2RustUnnamed_32 = 68719476918;
pub const KEYC_KP_NINE: C2RustUnnamed_32 = 68719476917;
pub const KEYC_KP_EIGHT: C2RustUnnamed_32 = 68719476916;
pub const KEYC_KP_SEVEN: C2RustUnnamed_32 = 68719476915;
pub const KEYC_KP_MINUS: C2RustUnnamed_32 = 68719476914;
pub const KEYC_KP_STAR: C2RustUnnamed_32 = 68719476913;
pub const KEYC_KP_SLASH: C2RustUnnamed_32 = 68719476912;
pub const KEYC_RIGHT: C2RustUnnamed_32 = 68719476911;
pub const KEYC_LEFT: C2RustUnnamed_32 = 68719476910;
pub const KEYC_DOWN: C2RustUnnamed_32 = 68719476909;
pub const KEYC_UP: C2RustUnnamed_32 = 68719476908;
pub const KEYC_BTAB: C2RustUnnamed_32 = 68719476907;
pub const KEYC_PPAGE: C2RustUnnamed_32 = 68719476906;
pub const KEYC_NPAGE: C2RustUnnamed_32 = 68719476905;
pub const KEYC_END: C2RustUnnamed_32 = 68719476904;
pub const KEYC_HOME: C2RustUnnamed_32 = 68719476903;
pub const KEYC_DC: C2RustUnnamed_32 = 68719476902;
pub const KEYC_IC: C2RustUnnamed_32 = 68719476901;
pub const KEYC_F12: C2RustUnnamed_32 = 68719476900;
pub const KEYC_F11: C2RustUnnamed_32 = 68719476899;
pub const KEYC_F10: C2RustUnnamed_32 = 68719476898;
pub const KEYC_F9: C2RustUnnamed_32 = 68719476897;
pub const KEYC_F8: C2RustUnnamed_32 = 68719476896;
pub const KEYC_F7: C2RustUnnamed_32 = 68719476895;
pub const KEYC_F6: C2RustUnnamed_32 = 68719476894;
pub const KEYC_F5: C2RustUnnamed_32 = 68719476893;
pub const KEYC_F4: C2RustUnnamed_32 = 68719476892;
pub const KEYC_F3: C2RustUnnamed_32 = 68719476891;
pub const KEYC_F2: C2RustUnnamed_32 = 68719476890;
pub const KEYC_F1: C2RustUnnamed_32 = 68719476889;
pub const KEYC_BSPACE: C2RustUnnamed_32 = 68719476888;
pub const KEYC_TRIPLECLICK3_BORDER: C2RustUnnamed_32 = 68719476887;
pub const KEYC_TRIPLECLICK3_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476886;
pub const KEYC_TRIPLECLICK3_STATUS_RIGHT: C2RustUnnamed_32 = 68719476885;
pub const KEYC_TRIPLECLICK3_STATUS_LEFT: C2RustUnnamed_32 = 68719476884;
pub const KEYC_TRIPLECLICK3_STATUS: C2RustUnnamed_32 = 68719476883;
pub const KEYC_TRIPLECLICK3_PANE: C2RustUnnamed_32 = 68719476882;
pub const KEYC_TRIPLECLICK2_BORDER: C2RustUnnamed_32 = 68719476881;
pub const KEYC_TRIPLECLICK2_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476880;
pub const KEYC_TRIPLECLICK2_STATUS_RIGHT: C2RustUnnamed_32 = 68719476879;
pub const KEYC_TRIPLECLICK2_STATUS_LEFT: C2RustUnnamed_32 = 68719476878;
pub const KEYC_TRIPLECLICK2_STATUS: C2RustUnnamed_32 = 68719476877;
pub const KEYC_TRIPLECLICK2_PANE: C2RustUnnamed_32 = 68719476876;
pub const KEYC_TRIPLECLICK1_BORDER: C2RustUnnamed_32 = 68719476875;
pub const KEYC_TRIPLECLICK1_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476874;
pub const KEYC_TRIPLECLICK1_STATUS_RIGHT: C2RustUnnamed_32 = 68719476873;
pub const KEYC_TRIPLECLICK1_STATUS_LEFT: C2RustUnnamed_32 = 68719476872;
pub const KEYC_TRIPLECLICK1_STATUS: C2RustUnnamed_32 = 68719476871;
pub const KEYC_TRIPLECLICK1_PANE: C2RustUnnamed_32 = 68719476870;
pub const KEYC_DOUBLECLICK3_BORDER: C2RustUnnamed_32 = 68719476869;
pub const KEYC_DOUBLECLICK3_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476868;
pub const KEYC_DOUBLECLICK3_STATUS_RIGHT: C2RustUnnamed_32 = 68719476867;
pub const KEYC_DOUBLECLICK3_STATUS_LEFT: C2RustUnnamed_32 = 68719476866;
pub const KEYC_DOUBLECLICK3_STATUS: C2RustUnnamed_32 = 68719476865;
pub const KEYC_DOUBLECLICK3_PANE: C2RustUnnamed_32 = 68719476864;
pub const KEYC_DOUBLECLICK2_BORDER: C2RustUnnamed_32 = 68719476863;
pub const KEYC_DOUBLECLICK2_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476862;
pub const KEYC_DOUBLECLICK2_STATUS_RIGHT: C2RustUnnamed_32 = 68719476861;
pub const KEYC_DOUBLECLICK2_STATUS_LEFT: C2RustUnnamed_32 = 68719476860;
pub const KEYC_DOUBLECLICK2_STATUS: C2RustUnnamed_32 = 68719476859;
pub const KEYC_DOUBLECLICK2_PANE: C2RustUnnamed_32 = 68719476858;
pub const KEYC_DOUBLECLICK1_BORDER: C2RustUnnamed_32 = 68719476857;
pub const KEYC_DOUBLECLICK1_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476856;
pub const KEYC_DOUBLECLICK1_STATUS_RIGHT: C2RustUnnamed_32 = 68719476855;
pub const KEYC_DOUBLECLICK1_STATUS_LEFT: C2RustUnnamed_32 = 68719476854;
pub const KEYC_DOUBLECLICK1_STATUS: C2RustUnnamed_32 = 68719476853;
pub const KEYC_DOUBLECLICK1_PANE: C2RustUnnamed_32 = 68719476852;
pub const KEYC_SECONDCLICK3_BORDER: C2RustUnnamed_32 = 68719476851;
pub const KEYC_SECONDCLICK3_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476850;
pub const KEYC_SECONDCLICK3_STATUS_RIGHT: C2RustUnnamed_32 = 68719476849;
pub const KEYC_SECONDCLICK3_STATUS_LEFT: C2RustUnnamed_32 = 68719476848;
pub const KEYC_SECONDCLICK3_STATUS: C2RustUnnamed_32 = 68719476847;
pub const KEYC_SECONDCLICK3_PANE: C2RustUnnamed_32 = 68719476846;
pub const KEYC_SECONDCLICK2_BORDER: C2RustUnnamed_32 = 68719476845;
pub const KEYC_SECONDCLICK2_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476844;
pub const KEYC_SECONDCLICK2_STATUS_RIGHT: C2RustUnnamed_32 = 68719476843;
pub const KEYC_SECONDCLICK2_STATUS_LEFT: C2RustUnnamed_32 = 68719476842;
pub const KEYC_SECONDCLICK2_STATUS: C2RustUnnamed_32 = 68719476841;
pub const KEYC_SECONDCLICK2_PANE: C2RustUnnamed_32 = 68719476840;
pub const KEYC_SECONDCLICK1_BORDER: C2RustUnnamed_32 = 68719476839;
pub const KEYC_SECONDCLICK1_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476838;
pub const KEYC_SECONDCLICK1_STATUS_RIGHT: C2RustUnnamed_32 = 68719476837;
pub const KEYC_SECONDCLICK1_STATUS_LEFT: C2RustUnnamed_32 = 68719476836;
pub const KEYC_SECONDCLICK1_STATUS: C2RustUnnamed_32 = 68719476835;
pub const KEYC_SECONDCLICK1_PANE: C2RustUnnamed_32 = 68719476834;
pub const KEYC_WHEELDOWN_BORDER: C2RustUnnamed_32 = 68719476833;
pub const KEYC_WHEELDOWN_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476832;
pub const KEYC_WHEELDOWN_STATUS_RIGHT: C2RustUnnamed_32 = 68719476831;
pub const KEYC_WHEELDOWN_STATUS_LEFT: C2RustUnnamed_32 = 68719476830;
pub const KEYC_WHEELDOWN_STATUS: C2RustUnnamed_32 = 68719476829;
pub const KEYC_WHEELDOWN_PANE: C2RustUnnamed_32 = 68719476828;
pub const KEYC_WHEELUP_BORDER: C2RustUnnamed_32 = 68719476827;
pub const KEYC_WHEELUP_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476826;
pub const KEYC_WHEELUP_STATUS_RIGHT: C2RustUnnamed_32 = 68719476825;
pub const KEYC_WHEELUP_STATUS_LEFT: C2RustUnnamed_32 = 68719476824;
pub const KEYC_WHEELUP_STATUS: C2RustUnnamed_32 = 68719476823;
pub const KEYC_WHEELUP_PANE: C2RustUnnamed_32 = 68719476822;
pub const KEYC_MOUSEDRAGEND3_BORDER: C2RustUnnamed_32 = 68719476821;
pub const KEYC_MOUSEDRAGEND3_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476820;
pub const KEYC_MOUSEDRAGEND3_STATUS_RIGHT: C2RustUnnamed_32 = 68719476819;
pub const KEYC_MOUSEDRAGEND3_STATUS_LEFT: C2RustUnnamed_32 = 68719476818;
pub const KEYC_MOUSEDRAGEND3_STATUS: C2RustUnnamed_32 = 68719476817;
pub const KEYC_MOUSEDRAGEND3_PANE: C2RustUnnamed_32 = 68719476816;
pub const KEYC_MOUSEDRAGEND2_BORDER: C2RustUnnamed_32 = 68719476815;
pub const KEYC_MOUSEDRAGEND2_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476814;
pub const KEYC_MOUSEDRAGEND2_STATUS_RIGHT: C2RustUnnamed_32 = 68719476813;
pub const KEYC_MOUSEDRAGEND2_STATUS_LEFT: C2RustUnnamed_32 = 68719476812;
pub const KEYC_MOUSEDRAGEND2_STATUS: C2RustUnnamed_32 = 68719476811;
pub const KEYC_MOUSEDRAGEND2_PANE: C2RustUnnamed_32 = 68719476810;
pub const KEYC_MOUSEDRAGEND1_BORDER: C2RustUnnamed_32 = 68719476809;
pub const KEYC_MOUSEDRAGEND1_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476808;
pub const KEYC_MOUSEDRAGEND1_STATUS_RIGHT: C2RustUnnamed_32 = 68719476807;
pub const KEYC_MOUSEDRAGEND1_STATUS_LEFT: C2RustUnnamed_32 = 68719476806;
pub const KEYC_MOUSEDRAGEND1_STATUS: C2RustUnnamed_32 = 68719476805;
pub const KEYC_MOUSEDRAGEND1_PANE: C2RustUnnamed_32 = 68719476804;
pub const KEYC_MOUSEDRAG3_BORDER: C2RustUnnamed_32 = 68719476803;
pub const KEYC_MOUSEDRAG3_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476802;
pub const KEYC_MOUSEDRAG3_STATUS_RIGHT: C2RustUnnamed_32 = 68719476801;
pub const KEYC_MOUSEDRAG3_STATUS_LEFT: C2RustUnnamed_32 = 68719476800;
pub const KEYC_MOUSEDRAG3_STATUS: C2RustUnnamed_32 = 68719476799;
pub const KEYC_MOUSEDRAG3_PANE: C2RustUnnamed_32 = 68719476798;
pub const KEYC_MOUSEDRAG2_BORDER: C2RustUnnamed_32 = 68719476797;
pub const KEYC_MOUSEDRAG2_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476796;
pub const KEYC_MOUSEDRAG2_STATUS_RIGHT: C2RustUnnamed_32 = 68719476795;
pub const KEYC_MOUSEDRAG2_STATUS_LEFT: C2RustUnnamed_32 = 68719476794;
pub const KEYC_MOUSEDRAG2_STATUS: C2RustUnnamed_32 = 68719476793;
pub const KEYC_MOUSEDRAG2_PANE: C2RustUnnamed_32 = 68719476792;
pub const KEYC_MOUSEDRAG1_BORDER: C2RustUnnamed_32 = 68719476791;
pub const KEYC_MOUSEDRAG1_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476790;
pub const KEYC_MOUSEDRAG1_STATUS_RIGHT: C2RustUnnamed_32 = 68719476789;
pub const KEYC_MOUSEDRAG1_STATUS_LEFT: C2RustUnnamed_32 = 68719476788;
pub const KEYC_MOUSEDRAG1_STATUS: C2RustUnnamed_32 = 68719476787;
pub const KEYC_MOUSEDRAG1_PANE: C2RustUnnamed_32 = 68719476786;
pub const KEYC_MOUSEUP3_BORDER: C2RustUnnamed_32 = 68719476785;
pub const KEYC_MOUSEUP3_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476784;
pub const KEYC_MOUSEUP3_STATUS_RIGHT: C2RustUnnamed_32 = 68719476783;
pub const KEYC_MOUSEUP3_STATUS_LEFT: C2RustUnnamed_32 = 68719476782;
pub const KEYC_MOUSEUP3_STATUS: C2RustUnnamed_32 = 68719476781;
pub const KEYC_MOUSEUP3_PANE: C2RustUnnamed_32 = 68719476780;
pub const KEYC_MOUSEUP2_BORDER: C2RustUnnamed_32 = 68719476779;
pub const KEYC_MOUSEUP2_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476778;
pub const KEYC_MOUSEUP2_STATUS_RIGHT: C2RustUnnamed_32 = 68719476777;
pub const KEYC_MOUSEUP2_STATUS_LEFT: C2RustUnnamed_32 = 68719476776;
pub const KEYC_MOUSEUP2_STATUS: C2RustUnnamed_32 = 68719476775;
pub const KEYC_MOUSEUP2_PANE: C2RustUnnamed_32 = 68719476774;
pub const KEYC_MOUSEUP1_BORDER: C2RustUnnamed_32 = 68719476773;
pub const KEYC_MOUSEUP1_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476772;
pub const KEYC_MOUSEUP1_STATUS_RIGHT: C2RustUnnamed_32 = 68719476771;
pub const KEYC_MOUSEUP1_STATUS_LEFT: C2RustUnnamed_32 = 68719476770;
pub const KEYC_MOUSEUP1_STATUS: C2RustUnnamed_32 = 68719476769;
pub const KEYC_MOUSEUP1_PANE: C2RustUnnamed_32 = 68719476768;
pub const KEYC_MOUSEDOWN3_BORDER: C2RustUnnamed_32 = 68719476767;
pub const KEYC_MOUSEDOWN3_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476766;
pub const KEYC_MOUSEDOWN3_STATUS_RIGHT: C2RustUnnamed_32 = 68719476765;
pub const KEYC_MOUSEDOWN3_STATUS_LEFT: C2RustUnnamed_32 = 68719476764;
pub const KEYC_MOUSEDOWN3_STATUS: C2RustUnnamed_32 = 68719476763;
pub const KEYC_MOUSEDOWN3_PANE: C2RustUnnamed_32 = 68719476762;
pub const KEYC_MOUSEDOWN2_BORDER: C2RustUnnamed_32 = 68719476761;
pub const KEYC_MOUSEDOWN2_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476760;
pub const KEYC_MOUSEDOWN2_STATUS_RIGHT: C2RustUnnamed_32 = 68719476759;
pub const KEYC_MOUSEDOWN2_STATUS_LEFT: C2RustUnnamed_32 = 68719476758;
pub const KEYC_MOUSEDOWN2_STATUS: C2RustUnnamed_32 = 68719476757;
pub const KEYC_MOUSEDOWN2_PANE: C2RustUnnamed_32 = 68719476756;
pub const KEYC_MOUSEDOWN1_BORDER: C2RustUnnamed_32 = 68719476755;
pub const KEYC_MOUSEDOWN1_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476754;
pub const KEYC_MOUSEDOWN1_STATUS_RIGHT: C2RustUnnamed_32 = 68719476753;
pub const KEYC_MOUSEDOWN1_STATUS_LEFT: C2RustUnnamed_32 = 68719476752;
pub const KEYC_MOUSEDOWN1_STATUS: C2RustUnnamed_32 = 68719476751;
pub const KEYC_MOUSEDOWN1_PANE: C2RustUnnamed_32 = 68719476750;
pub const KEYC_MOUSEMOVE_BORDER: C2RustUnnamed_32 = 68719476749;
pub const KEYC_MOUSEMOVE_STATUS_DEFAULT: C2RustUnnamed_32 = 68719476748;
pub const KEYC_MOUSEMOVE_STATUS_RIGHT: C2RustUnnamed_32 = 68719476747;
pub const KEYC_MOUSEMOVE_STATUS_LEFT: C2RustUnnamed_32 = 68719476746;
pub const KEYC_MOUSEMOVE_STATUS: C2RustUnnamed_32 = 68719476745;
pub const KEYC_MOUSEMOVE_PANE: C2RustUnnamed_32 = 68719476744;
pub const KEYC_DOUBLECLICK: C2RustUnnamed_32 = 68719476743;
pub const KEYC_DRAGGING: C2RustUnnamed_32 = 68719476742;
pub const KEYC_MOUSE: C2RustUnnamed_32 = 68719476741;
pub const KEYC_PASTE_END: C2RustUnnamed_32 = 68719476740;
pub const KEYC_PASTE_START: C2RustUnnamed_32 = 68719476739;
pub const KEYC_ANY: C2RustUnnamed_32 = 68719476738;
pub const KEYC_FOCUS_OUT: C2RustUnnamed_32 = 68719476737;
pub const KEYC_FOCUS_IN: C2RustUnnamed_32 = 68719476736;
/* Entry in the key tree. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_key_entry {
    pub key: key_code,
    pub data: *const libc::c_char,
    pub entry: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub rbe_left: *mut input_key_entry,
    pub rbe_right: *mut input_key_entry,
    pub rbe_parent: *mut input_key_entry,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_key_tree {
    pub rbh_root: *mut input_key_entry,
}
unsafe extern "C" fn input_key_tree_RB_INSERT_COLOR(mut head:
                                                        *mut input_key_tree,
                                                    mut elm:
                                                        *mut input_key_entry) {
    let mut parent: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut gparent: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut tmp: *mut input_key_entry = 0 as *mut input_key_entry;
    loop  {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() &&
                 (*parent).entry.rbe_color == 1 as libc::c_int) {
            break ;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
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
                        if parent ==
                               (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right =
                                tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_left;
                (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*gparent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = gparent
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if gparent ==
                           (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else { (*head).rbh_root = tmp }
                (*tmp).entry.rbe_right = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
            }
        } else {
            tmp = (*gparent).entry.rbe_left;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
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
                        if parent ==
                               (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right =
                                tmp
                        }
                    } else { (*head).rbh_root = tmp }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_right;
                (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*gparent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = gparent
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if gparent ==
                           (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else { (*head).rbh_root = tmp }
                (*tmp).entry.rbe_left = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn input_key_tree_RB_FIND(mut head: *mut input_key_tree,
                                            mut elm: *mut input_key_entry)
 -> *mut input_key_entry {
    let mut tmp: *mut input_key_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = input_key_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    return 0 as *mut input_key_entry;
}
unsafe extern "C" fn input_key_tree_RB_INSERT(mut head: *mut input_key_tree,
                                              mut elm: *mut input_key_entry)
 -> *mut input_key_entry {
    let mut tmp: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut parent: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = input_key_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut input_key_entry;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
    } else { (*head).rbh_root = elm }
    input_key_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut input_key_entry;
}
unsafe extern "C" fn input_key_tree_RB_MINMAX(mut head: *mut input_key_tree,
                                              mut val: libc::c_int)
 -> *mut input_key_entry {
    let mut tmp: *mut input_key_entry = (*head).rbh_root;
    let mut parent: *mut input_key_entry = 0 as *mut input_key_entry;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
unsafe extern "C" fn input_key_tree_RB_NEXT(mut elm: *mut input_key_entry)
 -> *mut input_key_entry {
    if !(*elm).entry.rbe_right.is_null() {
        elm = (*elm).entry.rbe_right;
        while !(*elm).entry.rbe_left.is_null() { elm = (*elm).entry.rbe_left }
    } else if !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_left {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_right {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub static mut input_key_tree: input_key_tree =
    {
        let mut init =
            input_key_tree{rbh_root:
                               0 as *const input_key_entry as
                                   *mut input_key_entry,};
        init
    };
/* List of default keys, the tree is built from this. */
static mut input_key_defaults: [input_key_entry; 91] =
    [{
         let mut init =
             input_key_entry{key:
                                 KEYC_PASTE_START as libc::c_ulong as
                                     key_code,
                             data:
                                 b"\x1b[200~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_PASTE_END as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[201~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F1 as libc::c_ulong as key_code,
                             data:
                                 b"\x1bOP\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F2 as libc::c_ulong as key_code,
                             data:
                                 b"\x1bOQ\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F3 as libc::c_ulong as key_code,
                             data:
                                 b"\x1bOR\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F4 as libc::c_ulong as key_code,
                             data:
                                 b"\x1bOS\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F5 as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[15~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F6 as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[17~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F7 as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[18~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F8 as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[19~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F9 as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[20~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F10 as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[21~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F11 as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[23~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_F12 as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[24~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F1 as libc::c_ulong as libc::c_ulonglong
                                     | 0x400000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[25~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F2 as libc::c_ulong as libc::c_ulonglong
                                     | 0x400000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[26~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F3 as libc::c_ulong as libc::c_ulonglong
                                     | 0x400000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[28~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F4 as libc::c_ulong as libc::c_ulonglong
                                     | 0x400000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[29~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F5 as libc::c_ulong as libc::c_ulonglong
                                     | 0x400000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[31~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F6 as libc::c_ulong as libc::c_ulonglong
                                     | 0x400000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[32~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F7 as libc::c_ulong as libc::c_ulonglong
                                     | 0x400000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[33~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F8 as libc::c_ulong as libc::c_ulonglong
                                     | 0x400000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[34~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_IC as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[2~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_DC as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[3~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_HOME as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[1~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_END as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[4~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_NPAGE as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[6~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_PPAGE as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[5~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_BTAB as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[Z\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_UP as libc::c_ulong as libc::c_ulonglong
                                     | 0x4000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOA\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_DOWN as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x4000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOB\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_RIGHT as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x4000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOC\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_LEFT as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x4000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOD\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_UP as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[A\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_DOWN as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[B\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_RIGHT as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[C\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_LEFT as libc::c_ulong as key_code,
                             data:
                                 b"\x1b[D\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_SLASH as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOo\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_STAR as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOj\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_MINUS as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOm\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_SEVEN as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOw\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_EIGHT as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOx\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_NINE as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOy\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_PLUS as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOk\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_FOUR as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOt\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_FIVE as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOu\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_SIX as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOv\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_ONE as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOq\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_TWO as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOr\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_THREE as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOs\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_ENTER as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOM\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_ZERO as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOp\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_KP_PERIOD as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x2000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1bOn\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_SLASH as libc::c_ulong as key_code,
                             data:
                                 b"/\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_STAR as libc::c_ulong as key_code,
                             data:
                                 b"*\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_MINUS as libc::c_ulong as key_code,
                             data:
                                 b"-\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_SEVEN as libc::c_ulong as key_code,
                             data:
                                 b"7\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_EIGHT as libc::c_ulong as key_code,
                             data:
                                 b"8\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_NINE as libc::c_ulong as key_code,
                             data:
                                 b"9\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_PLUS as libc::c_ulong as key_code,
                             data:
                                 b"+\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_FOUR as libc::c_ulong as key_code,
                             data:
                                 b"4\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_FIVE as libc::c_ulong as key_code,
                             data:
                                 b"5\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_SIX as libc::c_ulong as key_code,
                             data:
                                 b"6\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_ONE as libc::c_ulong as key_code,
                             data:
                                 b"1\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_TWO as libc::c_ulong as key_code,
                             data:
                                 b"2\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_THREE as libc::c_ulong as key_code,
                             data:
                                 b"3\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_ENTER as libc::c_ulong as key_code,
                             data:
                                 b"\n\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_ZERO as libc::c_ulong as key_code,
                             data:
                                 b"0\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key: KEYC_KP_PERIOD as libc::c_ulong as key_code,
                             data:
                                 b".\x00" as *const u8 as *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F1 as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[1;_P\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F2 as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[1;_Q\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F3 as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[1;_R\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F4 as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[1;_S\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F5 as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[15;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F6 as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[17;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F7 as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[18;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F8 as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[19;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F9 as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[20;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F10 as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[21;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F11 as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[23;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_F12 as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[24;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_UP as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[1;_A\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_DOWN as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[1;_B\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_RIGHT as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[1;_C\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_LEFT as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[1;_D\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_HOME as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[1;_H\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_END as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[1;_F\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_PPAGE as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[5;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_NPAGE as libc::c_ulong as
                                     libc::c_ulonglong |
                                     0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[6;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_IC as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[2;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     },
     {
         let mut init =
             input_key_entry{key:
                                 KEYC_DC as libc::c_ulong as libc::c_ulonglong
                                     | 0x10000000000000 as libc::c_ulonglong,
                             data:
                                 b"\x1b[3;_~\x00" as *const u8 as
                                     *const libc::c_char,
                             entry:
                                 C2RustUnnamed_33{rbe_left:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_right:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_parent:
                                                      0 as
                                                          *const input_key_entry
                                                          as
                                                          *mut input_key_entry,
                                                  rbe_color: 0,},};
         init
     }];
static mut input_key_modifiers: [key_code; 9] =
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
/* Tree of input keys. */
/* Input key comparison function. */
unsafe extern "C" fn input_key_cmp(mut ike1: *mut input_key_entry,
                                   mut ike2: *mut input_key_entry)
 -> libc::c_int {
    if (*ike1).key < (*ike2).key { return -(1 as libc::c_int) }
    if (*ike1).key > (*ike2).key { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/* Look for key in tree. */
unsafe extern "C" fn input_key_get(mut key: key_code)
 -> *mut input_key_entry {
    let mut entry: input_key_entry =
        {
            let mut init =
                input_key_entry{key: key,
                                data: 0 as *const libc::c_char,
                                entry:
                                    C2RustUnnamed_33{rbe_left:
                                                         0 as
                                                             *const input_key_entry
                                                             as
                                                             *mut input_key_entry,
                                                     rbe_right:
                                                         0 as
                                                             *const input_key_entry
                                                             as
                                                             *mut input_key_entry,
                                                     rbe_parent:
                                                         0 as
                                                             *const input_key_entry
                                                             as
                                                             *mut input_key_entry,
                                                     rbe_color: 0,},};
            init
        };
    return input_key_tree_RB_FIND(&mut input_key_tree, &mut entry);
}
/* Split a character into two UTF-8 bytes. */
unsafe extern "C" fn input_key_split2(mut c: u_int, mut dst: *mut u_char)
 -> size_t {
    if c > 0x7f as libc::c_int as libc::c_uint {
        *dst.offset(0 as libc::c_int as isize) =
            (c >> 6 as libc::c_int | 0xc0 as libc::c_int as libc::c_uint) as
                u_char;
        *dst.offset(1 as libc::c_int as isize) =
            (c & 0x3f as libc::c_int as libc::c_uint |
                 0x80 as libc::c_int as libc::c_uint) as u_char;
        return 2 as libc::c_int as size_t
    }
    *dst.offset(0 as libc::c_int as isize) = c as u_char;
    return 1 as libc::c_int as size_t;
}
/* Build input key tree. */
#[no_mangle]
pub unsafe extern "C" fn input_key_build() {
    let mut ike: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut new: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: key_code = 0;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[input_key_entry; 91]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<input_key_entry>()
                                                   as libc::c_ulong) {
        ike =
            &mut *input_key_defaults.as_mut_ptr().offset(i as isize) as
                *mut input_key_entry;
        if !(*ike).key & 0x10000000000000 as libc::c_ulonglong != 0 {
            input_key_tree_RB_INSERT(&mut input_key_tree, ike);
        } else {
            j = 2 as libc::c_int as u_int;
            while (j as libc::c_ulong) <
                      (::std::mem::size_of::<[key_code; 9]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<key_code>()
                                                           as libc::c_ulong) {
                key = (*ike).key & !(0x10000000000000 as libc::c_ulonglong);
                data = xstrdup((*ike).data);
                *data.offset(strcspn(data,
                                     b"_\x00" as *const u8 as
                                         *const libc::c_char) as isize) =
                    ('0' as i32 as libc::c_uint).wrapping_add(j) as
                        libc::c_char;
                new =
                    xcalloc(1 as libc::c_int as size_t,
                            ::std::mem::size_of::<input_key_entry>() as
                                libc::c_ulong) as *mut input_key_entry;
                (*new).key = key | input_key_modifiers[j as usize];
                (*new).data = data;
                input_key_tree_RB_INSERT(&mut input_key_tree, new);
                j = j.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    ike = input_key_tree_RB_MINMAX(&mut input_key_tree, -(1 as libc::c_int));
    while !ike.is_null() {
        log_debug(b"%s: 0x%llx (%s) is %s\x00" as *const u8 as
                      *const libc::c_char,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"input_key_build\x00")).as_ptr(),
                  (*ike).key,
                  key_string_lookup_key((*ike).key, 1 as libc::c_int),
                  (*ike).data);
        ike = input_key_tree_RB_NEXT(ike)
    };
}
/* Translate a key code into an output key sequence for a pane. */
#[no_mangle]
pub unsafe extern "C" fn input_key_pane(mut wp: *mut window_pane,
                                        mut key: key_code,
                                        mut m: *mut mouse_event)
 -> libc::c_int {
    if log_get_level() != 0 as libc::c_int {
        log_debug(b"writing key 0x%llx (%s) to %%%u\x00" as *const u8 as
                      *const libc::c_char, key,
                  key_string_lookup_key(key, 1 as libc::c_int), (*wp).id);
    }
    if key & 0xfffffffffff as libc::c_ulonglong >=
           KEYC_MOUSE as libc::c_ulong as libc::c_ulonglong &&
           (key & 0xfffffffffff as libc::c_ulonglong) <
               KEYC_BSPACE as libc::c_ulong as libc::c_ulonglong {
        if !m.is_null() && (*m).wp != -(1 as libc::c_int) &&
               (*m).wp as u_int == (*wp).id {
            input_key_mouse(wp, m);
        }
        return 0 as libc::c_int
    }
    return input_key((*wp).screen, (*wp).event, key);
}
/* Translate a key code into an output key sequence. */
#[no_mangle]
pub unsafe extern "C" fn input_key(mut s: *mut screen,
                                   mut bev: *mut bufferevent,
                                   mut key: key_code) -> libc::c_int {
    let mut ike: *mut input_key_entry = 0 as *mut input_key_entry;
    let mut justkey: key_code = 0;
    let mut newkey: key_code = 0;
    let mut outkey: key_code = 0;
    let mut ud: utf8_data =
        utf8_data{data: [0; 21], have: 0, size: 0, width: 0,};
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut modifier: libc::c_char = 0;
    /* Mouse keys need a pane. */
    if key & 0xfffffffffff as libc::c_ulonglong >=
           KEYC_MOUSE as libc::c_ulong as libc::c_ulonglong &&
           (key & 0xfffffffffff as libc::c_ulonglong) <
               KEYC_BSPACE as libc::c_ulong as libc::c_ulonglong {
        return 0 as libc::c_int
    }
    /* Literal keys go as themselves (can't be more than eight bits). */
    if key & 0x1000000000000 as libc::c_ulonglong != 0 {
        ud.data[0 as libc::c_int as usize] = key as u_char;
        bufferevent_write(bev,
                          &mut *ud.data.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                              *mut u_char as *const libc::c_void,
                          1 as libc::c_int as size_t);
        return 0 as libc::c_int
    }
    /* Is this backspace? */
    if key & 0xfffffffffff as libc::c_ulonglong ==
           KEYC_BSPACE as libc::c_ulong as libc::c_ulonglong {
        newkey =
            options_get_number(global_options,
                               b"backspace\x00" as *const u8 as
                                   *const libc::c_char) as key_code;
        if newkey >= 0x7f as libc::c_int as libc::c_ulonglong {
            newkey = '\u{7f}' as i32 as key_code
        }
        key =
            newkey |
                key &
                    (0xf00000000000 as libc::c_ulonglong |
                         0xff000000000000 as libc::c_ulonglong)
    }
    /*
	 * If this is a normal 7-bit key, just send it, with a leading escape
	 * if necessary. If it is a UTF-8 key, split it and send it.
	 */
    justkey =
        key &
            !(0x100000000000 as libc::c_ulonglong |
                  0x8000000000000 as libc::c_ulonglong);
    if justkey <= 0x7f as libc::c_int as libc::c_ulonglong {
        if key & 0x100000000000 as libc::c_ulonglong != 0 {
            bufferevent_write(bev,
                              b"\x1b\x00" as *const u8 as *const libc::c_char
                                  as *const libc::c_void,
                              1 as libc::c_int as size_t);
        }
        ud.data[0 as libc::c_int as usize] = justkey as u_char;
        bufferevent_write(bev,
                          &mut *ud.data.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize) as
                              *mut u_char as *const libc::c_void,
                          1 as libc::c_int as size_t);
        return 0 as libc::c_int
    }
    if justkey > 0x7f as libc::c_int as libc::c_ulonglong &&
           justkey < 0x1000000000 as libc::c_ulonglong {
        if key & 0x100000000000 as libc::c_ulonglong != 0 {
            bufferevent_write(bev,
                              b"\x1b\x00" as *const u8 as *const libc::c_char
                                  as *const libc::c_void,
                              1 as libc::c_int as size_t);
        }
        utf8_to_data(justkey as utf8_char, &mut ud);
        bufferevent_write(bev, ud.data.as_mut_ptr() as *const libc::c_void,
                          ud.size as size_t);
        return 0 as libc::c_int
    }
    /*
	 * Look up in the tree. If not in application keypad or cursor mode,
	 * remove the flags from the key.
	 */
    if !(*s).mode & 0x8 as libc::c_int != 0 {
        key &= !(0x2000000000000 as libc::c_ulonglong)
    }
    if !(*s).mode & 0x4 as libc::c_int != 0 {
        key &= !(0x4000000000000 as libc::c_ulonglong)
    }
    ike = input_key_get(key);
    if ike.is_null() && key & 0x100000000000 as libc::c_ulonglong != 0 &&
           !key & 0x8000000000000 as libc::c_ulonglong != 0 {
        ike = input_key_get(key & !(0x100000000000 as libc::c_ulonglong))
    }
    if ike.is_null() && key & 0x4000000000000 as libc::c_ulonglong != 0 {
        ike = input_key_get(key & !(0x4000000000000 as libc::c_ulonglong))
    }
    if ike.is_null() && key & 0x2000000000000 as libc::c_ulonglong != 0 {
        ike = input_key_get(key & !(0x2000000000000 as libc::c_ulonglong))
    }
    if !ike.is_null() {
        log_debug(b"found key 0x%llx: \"%s\"\x00" as *const u8 as
                      *const libc::c_char, key, (*ike).data);
        if key & 0x100000000000 as libc::c_ulonglong != 0 &&
               !key & 0x8000000000000 as libc::c_ulonglong != 0 {
            bufferevent_write(bev,
                              b"\x1b\x00" as *const u8 as *const libc::c_char
                                  as *const libc::c_void,
                              1 as libc::c_int as size_t);
        }
        bufferevent_write(bev, (*ike).data as *const libc::c_void,
                          strlen((*ike).data));
        return 0 as libc::c_int
    }
    /* No builtin key sequence; construct an extended key sequence. */
    if !(*s).mode & 0x8000 as libc::c_int != 0 {
        if key & 0xf00000000000 as libc::c_ulonglong !=
               0x200000000000 as libc::c_ulonglong {
            log_debug(b"key 0x%llx missing\x00" as *const u8 as
                          *const libc::c_char, key);
            return -(1 as libc::c_int)
        } else {
            justkey = key & 0xfffffffffff as libc::c_ulonglong;
            match justkey {
                32 | 50 => {
                    key =
                        0 as libc::c_int as libc::c_ulonglong |
                            key & !(0xfffffffffff as libc::c_ulonglong)
                }
                124 => {
                    key =
                        28 as libc::c_int as libc::c_ulonglong |
                            key & !(0xfffffffffff as libc::c_ulonglong)
                }
                54 => {
                    key =
                        30 as libc::c_int as libc::c_ulonglong |
                            key & !(0xfffffffffff as libc::c_ulonglong)
                }
                45 | 47 => {
                    key =
                        31 as libc::c_int as libc::c_ulonglong |
                            key & !(0xfffffffffff as libc::c_ulonglong)
                }
                63 => {
                    key =
                        127 as libc::c_int as libc::c_ulonglong |
                            key & !(0xfffffffffff as libc::c_ulonglong)
                }
                _ => {
                    if justkey >= 'A' as i32 as libc::c_ulonglong &&
                           justkey <= '_' as i32 as libc::c_ulonglong {
                        key =
                            justkey.wrapping_sub('A' as i32 as
                                                     libc::c_ulonglong) |
                                key & !(0xfffffffffff as libc::c_ulonglong)
                    } else if justkey >= 'a' as i32 as libc::c_ulonglong &&
                                  justkey <= '~' as i32 as libc::c_ulonglong {
                        key =
                            justkey.wrapping_sub(96 as libc::c_int as
                                                     libc::c_ulonglong) |
                                key & !(0xfffffffffff as libc::c_ulonglong)
                    } else { return 0 as libc::c_int }
                }
            }
            return input_key(s, bev,
                             key & !(0x200000000000 as libc::c_ulonglong))
        }
    } else {
        outkey = key & 0xfffffffffff as libc::c_ulonglong;
        match key & 0xf00000000000 as libc::c_ulonglong {
            70368744177664 => { modifier = '2' as i32 as libc::c_char }
            17592186044416 => { modifier = '3' as i32 as libc::c_char }
            87960930222080 => { modifier = '4' as i32 as libc::c_char }
            35184372088832 => { modifier = '5' as i32 as libc::c_char }
            105553116266496 => { modifier = '6' as i32 as libc::c_char }
            52776558133248 => { modifier = '7' as i32 as libc::c_char }
            123145302310912 => { modifier = '8' as i32 as libc::c_char }
            _ => {
                fatalx(b"invalid key modifiers: %llx\x00" as *const u8 as
                           *const libc::c_char, key);
            }
        }
        xsnprintf(tmp.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 64]>() as
                      libc::c_ulong,
                  b"\x1b[%llu;%cu\x00" as *const u8 as *const libc::c_char,
                  outkey, modifier as libc::c_int);
        bufferevent_write(bev, tmp.as_mut_ptr() as *const libc::c_void,
                          strlen(tmp.as_mut_ptr()));
        return 0 as libc::c_int
    };
}
/* Get mouse event string. */
#[no_mangle]
pub unsafe extern "C" fn input_key_get_mouse(mut s: *mut screen,
                                             mut m: *mut mouse_event,
                                             mut x: u_int, mut y: u_int,
                                             mut rbuf:
                                                 *mut *const libc::c_char,
                                             mut rlen: *mut size_t)
 -> libc::c_int {
    static mut buf: [libc::c_char; 40] = [0; 40];
    let mut len: size_t = 0;
    *rbuf = 0 as *const libc::c_char;
    *rlen = 0 as libc::c_int as size_t;
    /* If this pane is not in button or all mode, discard motion events. */
    if (*m).b & 32 as libc::c_int as libc::c_uint != 0 &&
           (*s).mode & (0x40 as libc::c_int | 0x1000 as libc::c_int) ==
               0 as libc::c_int {
        return 0 as libc::c_int
    }
    if (*s).mode &
           (0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int)
           == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    /*
	 * If this event is a release event and not in all mode, discard it.
	 * In SGR mode we can tell absolutely because a release is normally
	 * shown by the last character. Without SGR, we check if the last
	 * buttons was also a release.
	 */
    if (*m).sgr_type != ' ' as i32 as libc::c_uint {
        if (*m).sgr_b & 32 as libc::c_int as libc::c_uint != 0 &&
               (*m).sgr_b & 3 as libc::c_int as libc::c_uint ==
                   3 as libc::c_int as libc::c_uint &&
               !(*s).mode & 0x1000 as libc::c_int != 0 {
            return 0 as libc::c_int
        }
    } else if (*m).b & 32 as libc::c_int as libc::c_uint != 0 &&
                  (*m).b & 3 as libc::c_int as libc::c_uint ==
                      3 as libc::c_int as libc::c_uint &&
                  (*m).lb & 3 as libc::c_int as libc::c_uint ==
                      3 as libc::c_int as libc::c_uint &&
                  !(*s).mode & 0x1000 as libc::c_int != 0 {
        return 0 as libc::c_int
    }
    /*
	 * Use the SGR (1006) extension only if the application requested it
	 * and the underlying terminal also sent the event in this format (this
	 * is because an old style mouse release event cannot be converted into
	 * the new SGR format, since the released button is unknown). Otherwise
	 * pretend that tmux doesn't speak this extension, and fall back to the
	 * UTF-8 (1005) extension if the application requested, or to the
	 * legacy format.
	 */
    if (*m).sgr_type != ' ' as i32 as libc::c_uint &&
           (*s).mode & 0x200 as libc::c_int != 0 {
        len =
            xsnprintf(buf.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 40]>() as
                          libc::c_ulong,
                      b"\x1b[<%u;%u;%u%c\x00" as *const u8 as
                          *const libc::c_char, (*m).sgr_b,
                      x.wrapping_add(1 as libc::c_int as libc::c_uint),
                      y.wrapping_add(1 as libc::c_int as libc::c_uint),
                      (*m).sgr_type) as size_t
    } else if (*s).mode & 0x100 as libc::c_int != 0 {
        if (*m).b > (0x7ff as libc::c_int - 32 as libc::c_int) as libc::c_uint
               ||
               x > (0x7ff as libc::c_int - 33 as libc::c_int) as libc::c_uint
               ||
               y > (0x7ff as libc::c_int - 33 as libc::c_int) as libc::c_uint
           {
            return 0 as libc::c_int
        }
        len =
            xsnprintf(buf.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 40]>() as
                          libc::c_ulong,
                      b"\x1b[M\x00" as *const u8 as *const libc::c_char) as
                size_t;
        len =
            (len as
                 libc::c_ulong).wrapping_add(input_key_split2((*m).b.wrapping_add(32
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint),
                                                              &mut *buf.as_mut_ptr().offset(len
                                                                                                as
                                                                                                isize)
                                                                  as
                                                                  *mut libc::c_char
                                                                  as
                                                                  *mut u_char))
                as size_t as size_t;
        len =
            (len as
                 libc::c_ulong).wrapping_add(input_key_split2(x.wrapping_add(33
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint),
                                                              &mut *buf.as_mut_ptr().offset(len
                                                                                                as
                                                                                                isize)
                                                                  as
                                                                  *mut libc::c_char
                                                                  as
                                                                  *mut u_char))
                as size_t as size_t;
        len =
            (len as
                 libc::c_ulong).wrapping_add(input_key_split2(y.wrapping_add(33
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint),
                                                              &mut *buf.as_mut_ptr().offset(len
                                                                                                as
                                                                                                isize)
                                                                  as
                                                                  *mut libc::c_char
                                                                  as
                                                                  *mut u_char))
                as size_t as size_t
    } else {
        if (*m).b > 223 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
        len =
            xsnprintf(buf.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 40]>() as
                          libc::c_ulong,
                      b"\x1b[M\x00" as *const u8 as *const libc::c_char) as
                size_t;
        let fresh0 = len;
        len = len.wrapping_add(1);
        buf[fresh0 as usize] =
            (*m).b.wrapping_add(32 as libc::c_int as libc::c_uint) as
                libc::c_char;
        let fresh1 = len;
        len = len.wrapping_add(1);
        buf[fresh1 as usize] =
            x.wrapping_add(33 as libc::c_int as libc::c_uint) as libc::c_char;
        let fresh2 = len;
        len = len.wrapping_add(1);
        buf[fresh2 as usize] =
            y.wrapping_add(33 as libc::c_int as libc::c_uint) as libc::c_char
    }
    *rbuf = buf.as_mut_ptr();
    *rlen = len;
    return 1 as libc::c_int;
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
 * This file is rather misleadingly named, it contains the code which takes a
 * key code and translates it into something suitable to be sent to the
 * application running in a pane (similar to input.c does in the other
 * direction with output).
 */
/* Translate mouse and output. */
unsafe extern "C" fn input_key_mouse(mut wp: *mut window_pane,
                                     mut m: *mut mouse_event) {
    let mut s: *mut screen = (*wp).screen;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    /* Ignore events if no mouse mode or the pane is not visible. */
    if (*m).ignore != 0 ||
           (*s).mode &
               (0x20 as libc::c_int | 0x40 as libc::c_int |
                    0x1000 as libc::c_int) == 0 as libc::c_int {
        return
    }
    if cmd_mouse_at(wp, m, &mut x, &mut y, 0 as libc::c_int) !=
           0 as libc::c_int {
        return
    }
    if window_pane_visible(wp) == 0 { return }
    if input_key_get_mouse(s, m, x, y, &mut buf, &mut len) == 0 { return }
    log_debug(b"writing mouse %.*s to %%%u\x00" as *const u8 as
                  *const libc::c_char, len as libc::c_int, buf, (*wp).id);
    bufferevent_write((*wp).event, buf as *const libc::c_void, len);
}
