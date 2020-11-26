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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> libc::c_ulong;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char,
                 _: ...) -> libc::c_int;
    #[no_mangle]
    fn notify_window(_: *const libc::c_char, _: *mut window);
    #[no_mangle]
    fn recalculate_sizes();
    #[no_mangle]
    fn window_count_panes(_: *mut window) -> u_int;
    #[no_mangle]
    fn layout_count_cells(_: *mut layout_cell) -> u_int;
    #[no_mangle]
    fn layout_create_cell(_: *mut layout_cell) -> *mut layout_cell;
    #[no_mangle]
    fn layout_free_cell(_: *mut layout_cell);
    #[no_mangle]
    fn layout_print_cell(_: *mut layout_cell, _: *const libc::c_char,
                         _: u_int);
    #[no_mangle]
    fn layout_destroy_cell(_: *mut window, _: *mut layout_cell,
                           _: *mut *mut layout_cell);
    #[no_mangle]
    fn layout_make_leaf(_: *mut layout_cell, _: *mut window_pane);
    #[no_mangle]
    fn layout_fix_offsets(_: *mut window);
    #[no_mangle]
    fn layout_fix_panes(_: *mut window);
    #[no_mangle]
    fn window_resize(_: *mut window, _: u_int, _: u_int, _: libc::c_int,
                     _: libc::c_int);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
/* $OpenBSD$ */
/*
 * Copyright (c) 2010 Nicholas Marriott <nicholas.marriott@gmail.com>
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
/* Find the bottom-right cell. */
unsafe extern "C" fn layout_find_bottomright(mut lc: *mut layout_cell)
 -> *mut layout_cell {
    if (*lc).type_0 as libc::c_uint ==
           LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint {
        return lc
    }
    lc = *(*((*lc).cells.tqh_last as *mut layout_cells)).tqh_last;
    return layout_find_bottomright(lc);
}
/* Calculate layout checksum. */
unsafe extern "C" fn layout_checksum(mut layout: *const libc::c_char)
 -> u_short {
    let mut csum: u_short = 0;
    csum = 0 as libc::c_int as u_short;
    while *layout as libc::c_int != '\u{0}' as i32 {
        csum =
            ((csum as libc::c_int >> 1 as libc::c_int) +
                 ((csum as libc::c_int & 1 as libc::c_int) <<
                      15 as libc::c_int)) as u_short;
        csum = (csum as libc::c_int + *layout as libc::c_int) as u_short;
        layout = layout.offset(1)
    }
    return csum;
}
/* Dump layout as a string. */
#[no_mangle]
pub unsafe extern "C" fn layout_dump(mut root: *mut layout_cell)
 -> *mut libc::c_char {
    let mut layout: [libc::c_char; 8192] = [0; 8192];
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    *layout.as_mut_ptr() = '\u{0}' as i32 as libc::c_char;
    if layout_append(root, layout.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 8192]>() as
                         libc::c_ulong) != 0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    xasprintf(&mut out as *mut *mut libc::c_char,
              b"%04hx,%s\x00" as *const u8 as *const libc::c_char,
              layout_checksum(layout.as_mut_ptr()) as libc::c_int,
              layout.as_mut_ptr());
    return out;
}
/* Append information for a single cell. */
unsafe extern "C" fn layout_append(mut lc: *mut layout_cell,
                                   mut buf: *mut libc::c_char,
                                   mut len: size_t) -> libc::c_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut tmplen: size_t = 0;
    let mut brackets: *const libc::c_char =
        b"][\x00" as *const u8 as *const libc::c_char;
    if len == 0 as libc::c_int as libc::c_ulong { return -(1 as libc::c_int) }
    if !(*lc).wp.is_null() {
        tmplen =
            xsnprintf(tmp.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 64]>() as
                          libc::c_ulong,
                      b"%ux%u,%u,%u,%u\x00" as *const u8 as
                          *const libc::c_char, (*lc).sx, (*lc).sy, (*lc).xoff,
                      (*lc).yoff, (*(*lc).wp).id) as size_t
    } else {
        tmplen =
            xsnprintf(tmp.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 64]>() as
                          libc::c_ulong,
                      b"%ux%u,%u,%u\x00" as *const u8 as *const libc::c_char,
                      (*lc).sx, (*lc).sy, (*lc).xoff, (*lc).yoff) as size_t
    }
    if tmplen >
           (::std::mem::size_of::<[libc::c_char; 64]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
       {
        return -(1 as libc::c_int)
    }
    if strlcat(buf, tmp.as_mut_ptr(), len) >= len {
        return -(1 as libc::c_int)
    }
    let mut current_block_21: u64;
    match (*lc).type_0 as libc::c_uint {
        0 => {
            brackets = b"}{\x00" as *const u8 as *const libc::c_char;
            current_block_21 = 10584518304474786328;
        }
        1 => { current_block_21 = 10584518304474786328; }
        2 | _ => { current_block_21 = 10043043949733653460; }
    }
    match current_block_21 {
        10584518304474786328 =>
        /* FALLTHROUGH */
        {
            if strlcat(buf, &*brackets.offset(1 as libc::c_int as isize), len)
                   >= len {
                return -(1 as libc::c_int)
            }
            lcchild = (*lc).cells.tqh_first;
            while !lcchild.is_null() {
                if layout_append(lcchild, buf, len) != 0 as libc::c_int {
                    return -(1 as libc::c_int)
                }
                if strlcat(buf, b",\x00" as *const u8 as *const libc::c_char,
                           len) >= len {
                    return -(1 as libc::c_int)
                }
                lcchild = (*lcchild).entry.tqe_next
            }
            *buf.offset(strlen(buf).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong) as isize)
                = *brackets.offset(0 as libc::c_int as isize)
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
/* Check layout sizes fit. */
unsafe extern "C" fn layout_check(mut lc: *mut layout_cell) -> libc::c_int {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0 as libc::c_int as u_int;
    match (*lc).type_0 as libc::c_uint {
        0 => {
            lcchild = (*lc).cells.tqh_first;
            while !lcchild.is_null() {
                if (*lcchild).sy != (*lc).sy { return 0 as libc::c_int }
                if layout_check(lcchild) == 0 { return 0 as libc::c_int }
                n =
                    (n as
                         libc::c_uint).wrapping_add((*lcchild).sx.wrapping_add(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint))
                        as u_int as u_int;
                lcchild = (*lcchild).entry.tqe_next
            }
            if n.wrapping_sub(1 as libc::c_int as libc::c_uint) != (*lc).sx {
                return 0 as libc::c_int
            }
        }
        1 => {
            lcchild = (*lc).cells.tqh_first;
            while !lcchild.is_null() {
                if (*lcchild).sx != (*lc).sx { return 0 as libc::c_int }
                if layout_check(lcchild) == 0 { return 0 as libc::c_int }
                n =
                    (n as
                         libc::c_uint).wrapping_add((*lcchild).sy.wrapping_add(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint))
                        as u_int as u_int;
                lcchild = (*lcchild).entry.tqe_next
            }
            if n.wrapping_sub(1 as libc::c_int as libc::c_uint) != (*lc).sy {
                return 0 as libc::c_int
            }
        }
        2 | _ => { }
    }
    return 1 as libc::c_int;
}
/* Parse a layout string and arrange window as layout. */
#[no_mangle]
pub unsafe extern "C" fn layout_parse(mut w: *mut window,
                                      mut layout: *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut npanes: u_int = 0;
    let mut ncells: u_int = 0;
    let mut sx: u_int = 0 as libc::c_int as u_int;
    let mut sy: u_int = 0 as libc::c_int as u_int;
    let mut csum: u_short = 0;
    /* Check validity. */
    if sscanf(layout, b"%hx,\x00" as *const u8 as *const libc::c_char,
              &mut csum as *mut u_short) != 1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    layout = layout.offset(5 as libc::c_int as isize);
    if csum as libc::c_int != layout_checksum(layout) as libc::c_int {
        return -(1 as libc::c_int)
    }
    /* Build the layout. */
    lc = layout_construct(0 as *mut layout_cell, &mut layout);
    if lc.is_null() { return -(1 as libc::c_int) }
    if *layout as libc::c_int != '\u{0}' as i32 {
        current_block = 9981732256371745227;
    } else { current_block = 17965632435239708295; }
    loop  {
        match current_block {
            9981732256371745227 => {
                layout_free_cell(lc);
                return -(1 as libc::c_int)
            }
            _ =>
            /* Check this window will fit into the layout. */
            {
                npanes = window_count_panes(w);
                ncells = layout_count_cells(lc);
                if npanes > ncells {
                    current_block = 9981732256371745227;
                    continue ;
                }
                if npanes == ncells {
                    /*
	 * It appears older versions of tmux were able to generate layouts with
	 * an incorrect top cell size - if it is larger than the top child then
	 * correct that (if this is still wrong the check code will catch it).
	 */
                    match (*lc).type_0 as libc::c_uint {
                        0 => {
                            lcchild = (*lc).cells.tqh_first;
                            while !lcchild.is_null() {
                                sy =
                                    (*lcchild).sy.wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint);
                                sx =
                                    (sx as
                                         libc::c_uint).wrapping_add((*lcchild).sx.wrapping_add(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint))
                                        as u_int as u_int;
                                lcchild = (*lcchild).entry.tqe_next
                            }
                        }
                        1 => {
                            lcchild = (*lc).cells.tqh_first;
                            while !lcchild.is_null() {
                                sx =
                                    (*lcchild).sx.wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint);
                                sy =
                                    (sy as
                                         libc::c_uint).wrapping_add((*lcchild).sy.wrapping_add(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint))
                                        as u_int as u_int;
                                lcchild = (*lcchild).entry.tqe_next
                            }
                        }
                        2 | _ => { }
                    }
                    if (*lc).type_0 as libc::c_uint !=
                           LAYOUT_WINDOWPANE as libc::c_int as libc::c_uint &&
                           ((*lc).sx != sx || (*lc).sy != sy) {
                        log_debug(b"fix layout %u,%u to %u,%u\x00" as
                                      *const u8 as *const libc::c_char,
                                  (*lc).sx, (*lc).sy, sx, sy);
                        layout_print_cell(lc,
                                          (*::std::mem::transmute::<&[u8; 13],
                                                                    &[libc::c_char; 13]>(b"layout_parse\x00")).as_ptr(),
                                          0 as libc::c_int as u_int);
                        (*lc).sx =
                            sx.wrapping_sub(1 as libc::c_int as libc::c_uint);
                        (*lc).sy =
                            sy.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    }
                    /* Check the new layout. */
                    if layout_check(lc) == 0 { return -(1 as libc::c_int) }
                    /* Resize to the layout size. */
                    window_resize(w, (*lc).sx, (*lc).sy, -(1 as libc::c_int),
                                  -(1 as libc::c_int));
                    /* Destroy the old layout and swap to the new. */
                    layout_free_cell((*w).layout_root);
                    (*w).layout_root = lc;
                    /* Assign the panes into the cells. */
                    wp = (*w).panes.tqh_first;
                    layout_assign(&mut wp, lc);
                    /* Update pane offsets and sizes. */
                    layout_fix_offsets(w);
                    layout_fix_panes(w);
                    recalculate_sizes();
                    layout_print_cell(lc,
                                      (*::std::mem::transmute::<&[u8; 13],
                                                                &[libc::c_char; 13]>(b"layout_parse\x00")).as_ptr(),
                                      0 as libc::c_int as u_int);
                    notify_window(b"window-layout-changed\x00" as *const u8 as
                                      *const libc::c_char, w);
                    return 0 as libc::c_int
                } else {
                    /* Fewer panes than cells - close the bottom right. */
                    lcchild = layout_find_bottomright(lc);
                    layout_destroy_cell(w, lcchild, &mut lc);
                    current_block = 17965632435239708295;
                }
            }
        }
    };
}
/* Assign panes into cells. */
unsafe extern "C" fn layout_assign(mut wp: *mut *mut window_pane,
                                   mut lc: *mut layout_cell) {
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    match (*lc).type_0 as libc::c_uint {
        2 => {
            layout_make_leaf(lc, *wp);
            *wp = (**wp).entry.tqe_next;
            return
        }
        0 | 1 => {
            lcchild = (*lc).cells.tqh_first;
            while !lcchild.is_null() {
                layout_assign(wp, lcchild);
                lcchild = (*lcchild).entry.tqe_next
            }
            return
        }
        _ => { }
    };
}
/* Construct a cell from all or part of a layout tree. */
unsafe extern "C" fn layout_construct(mut lcparent: *mut layout_cell,
                                      mut layout: *mut *const libc::c_char)
 -> *mut layout_cell {
    let mut current_block: u64;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    let mut saved: *const libc::c_char = 0 as *const libc::c_char;
    if *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as isize)
           as libc::c_int &
           _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0 {
        return 0 as *mut layout_cell
    }
    if sscanf(*layout, b"%ux%u,%u,%u\x00" as *const u8 as *const libc::c_char,
              &mut sx as *mut u_int, &mut sy as *mut u_int,
              &mut xoff as *mut u_int, &mut yoff as *mut u_int) !=
           4 as libc::c_int {
        return 0 as *mut layout_cell
    }
    while *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as
                                         isize) as libc::c_int &
              _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        *layout = (*layout).offset(1)
    }
    if **layout as libc::c_int != 'x' as i32 { return 0 as *mut layout_cell }
    *layout = (*layout).offset(1);
    while *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as
                                         isize) as libc::c_int &
              _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        *layout = (*layout).offset(1)
    }
    if **layout as libc::c_int != ',' as i32 { return 0 as *mut layout_cell }
    *layout = (*layout).offset(1);
    while *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as
                                         isize) as libc::c_int &
              _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        *layout = (*layout).offset(1)
    }
    if **layout as libc::c_int != ',' as i32 { return 0 as *mut layout_cell }
    *layout = (*layout).offset(1);
    while *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as
                                         isize) as libc::c_int &
              _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        *layout = (*layout).offset(1)
    }
    if **layout as libc::c_int == ',' as i32 {
        saved = *layout;
        *layout = (*layout).offset(1);
        while *(*__ctype_b_loc()).offset(**layout as u_char as libc::c_int as
                                             isize) as libc::c_int &
                  _ISdigit as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
            *layout = (*layout).offset(1)
        }
        if **layout as libc::c_int == 'x' as i32 { *layout = saved }
    }
    lc = layout_create_cell(lcparent);
    (*lc).sx = sx;
    (*lc).sy = sy;
    (*lc).xoff = xoff;
    (*lc).yoff = yoff;
    match **layout as libc::c_int {
        44 | 125 | 93 | 0 => { return lc }
        123 => {
            (*lc).type_0 = LAYOUT_LEFTRIGHT;
            current_block = 14072441030219150333;
        }
        91 => {
            (*lc).type_0 = LAYOUT_TOPBOTTOM;
            current_block = 14072441030219150333;
        }
        _ => { current_block = 6881314151732327134; }
    }
    loop  {
        match current_block {
            6881314151732327134 => {
                layout_free_cell(lc);
                return 0 as *mut layout_cell
            }
            _ => {
                *layout = (*layout).offset(1);
                lcchild = layout_construct(lc, layout);
                if lcchild.is_null() {
                    current_block = 6881314151732327134;
                    continue ;
                }
                (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                (*lcchild).entry.tqe_prev = (*lc).cells.tqh_last;
                *(*lc).cells.tqh_last = lcchild;
                (*lc).cells.tqh_last = &mut (*lcchild).entry.tqe_next;
                if **layout as libc::c_int == ',' as i32 {
                    current_block = 14072441030219150333;
                    continue ;
                }
                match (*lc).type_0 as libc::c_uint {
                    0 => {
                        if **layout as libc::c_int != '}' as i32 {
                            current_block = 6881314151732327134;
                        } else { break ; }
                    }
                    1 => {
                        if **layout as libc::c_int != ']' as i32 {
                            current_block = 6881314151732327134;
                        } else { break ; }
                    }
                    _ => { current_block = 6881314151732327134; }
                }
            }
        }
    }
    *layout = (*layout).offset(1);
    return lc;
}
