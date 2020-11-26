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
    pub type cmd;
    pub type cmdq_item;
    pub type cmdq_state;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    #[no_mangle]
    fn tty_attributes(_: *mut tty, _: *const grid_cell, _: *const grid_cell, _: *mut libc::c_int);
    #[no_mangle]
    fn tty_cursor(_: *mut tty, _: u_int, _: u_int);
    #[no_mangle]
    fn tty_putc(_: *mut tty, _: u_char);
    #[no_mangle]
    fn tty_putn(_: *mut tty, _: *const libc::c_void, _: size_t, _: u_int);
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_strtonum(
        _: *mut args,
        _: u_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *mut libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn cmd_get_args(_: *mut cmd) -> *mut args;
    #[no_mangle]
    fn cmd_template_replace(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn cmd_parse_and_append(
        _: *const libc::c_char,
        _: *mut cmd_parse_input,
        _: *mut client,
        _: *mut cmdq_state,
        _: *mut *mut libc::c_char,
    ) -> cmd_parse_status;
    #[no_mangle]
    fn cmdq_get_target_client(_: *mut cmdq_item) -> *mut client;
    #[no_mangle]
    fn cmdq_get_error(_: *const libc::c_char) -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_append(_: *mut client, _: *mut cmdq_item) -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_continue(_: *mut cmdq_item);
    #[no_mangle]
    fn cmdq_error(_: *mut cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn server_client_set_overlay(
        _: *mut client,
        _: u_int,
        _: overlay_check_cb,
        _: overlay_mode_cb,
        _: overlay_draw_cb,
        _: overlay_key_cb,
        _: overlay_free_cb,
        _: *mut libc::c_void,
    );
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    #[no_mangle]
    fn window_pane_visible(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    static window_clock_table: [[[libc::c_char; 5]; 5]; 14];
    #[no_mangle]
    fn window_unzoom(_: *mut window) -> libc::c_int;
    #[no_mangle]
    fn window_pane_at_index(_: *mut window, _: u_int) -> *mut window_pane;
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
    pub rbh_root: *mut args_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grid_cell {
    pub data: utf8_data,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utf8_data {
    pub data: [u_char; 21],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
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
    pub data: utf8_char,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}
pub type utf8_char = u_int;

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
    pub options: *mut options,
    pub flags: libc::c_int,
    pub attached: u_int,
    pub tio: *mut termios,
    pub environ: *mut environ,
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
    pub options: *mut options,
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
    pub formats: Option<unsafe extern "C" fn(_: *mut window_mode_entry, _: *mut format_tree) -> ()>,
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
    pub codes: *mut tty_code,
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
pub type cmd_find_type = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type cmd_retval = libc::c_int;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type cmd_parse_status = libc::c_uint;
pub const CMD_PARSE_SUCCESS: cmd_parse_status = 2;
pub const CMD_PARSE_ERROR: cmd_parse_status = 1;
pub const CMD_PARSE_EMPTY: cmd_parse_status = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_parse_input {
    pub flags: libc::c_int,
    pub file: *const libc::c_char,
    pub line: u_int,
    pub item: *mut cmdq_item,
    pub c: *mut client,
    pub fs: cmd_find_state,
}

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
    pub args: C2RustUnnamed_32,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item) -> cmd_retval>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_display_panes_data {
    pub item: *mut cmdq_item,
    pub command: *mut libc::c_char,
}
#[no_mangle]
pub static mut cmd_display_panes_entry: cmd_entry = {
    {
        let mut init = cmd_entry {
            name: b"display-panes\x00" as *const u8 as *const libc::c_char,
            alias: b"displayp\x00" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed_32 {
                    template: b"bd:t:\x00" as *const u8 as *const libc::c_char,
                    lower: 0 as libc::c_int,
                    upper: 1 as libc::c_int,
                };
                init
            },
            usage: b"[-b] [-d duration] [-t target-client] [template]\x00" as *const u8
                as *const libc::c_char,
            source: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            target: cmd_entry_flag {
                flag: 0,
                type_0: CMD_FIND_PANE,
                flags: 0,
            },
            flags: 0x4 as libc::c_int | 0x10 as libc::c_int,
            exec: Some(
                cmd_display_panes_exec
                    as unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item) -> cmd_retval,
            ),
        };
        init
    }
};
unsafe extern "C" fn cmd_display_panes_draw_pane(
    mut ctx: *mut screen_redraw_ctx,
    mut wp: *mut window_pane,
) {
    let mut c: *mut client = (*ctx).c;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut s: *mut session = (*c).session;
    let mut oo: *mut options = (*s).options;
    let mut w: *mut window = (*wp).window;
    let mut fgc: grid_cell = grid_cell {
        data: utf8_data {
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
    let mut bgc: grid_cell = grid_cell {
        data: utf8_data {
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
    let mut pane: u_int = 0;
    let mut idx: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut xoff: u_int = 0;
    let mut yoff: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut colour: libc::c_int = 0;
    let mut active_colour: libc::c_int = 0;
    let mut buf: [libc::c_char; 16] = [0; 16];
    let mut lbuf: [libc::c_char; 16] = [0; 16];
    let mut rbuf: [libc::c_char; 16] = [0; 16];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut llen: size_t = 0;
    let mut rlen: size_t = 0;
    if (*wp).xoff.wrapping_add((*wp).sx) <= (*ctx).ox
        || (*wp).xoff >= (*ctx).ox.wrapping_add((*ctx).sx)
        || (*wp).yoff.wrapping_add((*wp).sy) <= (*ctx).oy
        || (*wp).yoff >= (*ctx).oy.wrapping_add((*ctx).sy)
    {
        return;
    }
    if (*wp).xoff >= (*ctx).ox
        && (*wp).xoff.wrapping_add((*wp).sx) <= (*ctx).ox.wrapping_add((*ctx).sx)
    {
        /* All visible. */
        xoff = (*wp).xoff.wrapping_sub((*ctx).ox);
        sx = (*wp).sx
    } else if (*wp).xoff < (*ctx).ox
        && (*wp).xoff.wrapping_add((*wp).sx) > (*ctx).ox.wrapping_add((*ctx).sx)
    {
        /* Both left and right not visible. */
        xoff = 0 as libc::c_int as u_int;
        sx = (*ctx).sx
    } else if (*wp).xoff < (*ctx).ox {
        /* Left not visible. */
        xoff = 0 as libc::c_int as u_int;
        sx = (*wp).sx.wrapping_sub((*ctx).ox.wrapping_sub((*wp).xoff))
    } else {
        /* Right not visible. */
        xoff = (*wp).xoff.wrapping_sub((*ctx).ox);
        sx = (*wp).sx.wrapping_sub(xoff)
    }
    if (*wp).yoff >= (*ctx).oy
        && (*wp).yoff.wrapping_add((*wp).sy) <= (*ctx).oy.wrapping_add((*ctx).sy)
    {
        /* All visible. */
        yoff = (*wp).yoff.wrapping_sub((*ctx).oy);
        sy = (*wp).sy
    } else if (*wp).yoff < (*ctx).oy
        && (*wp).yoff.wrapping_add((*wp).sy) > (*ctx).oy.wrapping_add((*ctx).sy)
    {
        /* Both top and bottom not visible. */
        yoff = 0 as libc::c_int as u_int;
        sy = (*ctx).sy
    } else if (*wp).yoff < (*ctx).oy {
        /* Top not visible. */
        yoff = 0 as libc::c_int as u_int;
        sy = (*wp).sy.wrapping_sub((*ctx).oy.wrapping_sub((*wp).yoff))
    } else {
        /* Bottom not visible. */
        yoff = (*wp).yoff.wrapping_sub((*ctx).oy);
        sy = (*wp).sy.wrapping_sub(yoff)
    }
    if (*ctx).statustop != 0 {
        yoff = (yoff as libc::c_uint).wrapping_add((*ctx).statuslines) as u_int as u_int
    }
    px = sx.wrapping_div(2 as libc::c_int as libc::c_uint);
    py = sy.wrapping_div(2 as libc::c_int as libc::c_uint);
    if window_pane_index(wp, &mut pane) != 0 as libc::c_int {
        fatalx(b"index not found\x00" as *const u8 as *const libc::c_char);
    }
    len = xsnprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%u\x00" as *const u8 as *const libc::c_char,
        pane,
    ) as size_t;
    if (sx as libc::c_ulong) < len {
        return;
    }
    colour = options_get_number(
        oo,
        b"display-panes-colour\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    active_colour = options_get_number(
        oo,
        b"display-panes-active-colour\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    memcpy(
        &mut fgc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    memcpy(
        &mut bgc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    if (*w).active == wp {
        fgc.fg = active_colour;
        bgc.bg = active_colour
    } else {
        fgc.fg = colour;
        bgc.bg = colour
    }
    rlen = xsnprintf(
        rbuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"%ux%u\x00" as *const u8 as *const libc::c_char,
        (*wp).sx,
        (*wp).sy,
    ) as size_t;
    if pane > 9 as libc::c_int as libc::c_uint && pane < 35 as libc::c_int as libc::c_uint {
        llen = xsnprintf(
            lbuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"%c\x00" as *const u8 as *const libc::c_char,
            ('a' as i32 as libc::c_uint)
                .wrapping_add(pane.wrapping_sub(10 as libc::c_int as libc::c_uint)),
        ) as size_t
    } else {
        llen = 0 as libc::c_int as size_t
    }
    if (sx as libc::c_ulong) < len.wrapping_mul(6 as libc::c_int as libc::c_ulong)
        || sy < 5 as libc::c_int as libc::c_uint
    {
        tty_attributes(tty, &mut fgc, &grid_default_cell, 0 as *mut libc::c_int);
        if sx as libc::c_ulong
            >= len
                .wrapping_add(llen)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            len = (len as libc::c_ulong)
                .wrapping_add(llen.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            tty_cursor(
                tty,
                (xoff.wrapping_add(px) as libc::c_ulong)
                    .wrapping_sub(len.wrapping_div(2 as libc::c_int as libc::c_ulong))
                    as u_int,
                yoff.wrapping_add(py),
            );
            tty_putn(
                tty,
                buf.as_mut_ptr() as *const libc::c_void,
                len,
                len as u_int,
            );
            tty_putn(
                tty,
                b" \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
                1 as libc::c_int as u_int,
            );
            tty_putn(
                tty,
                lbuf.as_mut_ptr() as *const libc::c_void,
                llen,
                llen as u_int,
            );
        } else {
            tty_cursor(
                tty,
                (xoff.wrapping_add(px) as libc::c_ulong)
                    .wrapping_sub(len.wrapping_div(2 as libc::c_int as libc::c_ulong))
                    as u_int,
                yoff.wrapping_add(py),
            );
            tty_putn(
                tty,
                buf.as_mut_ptr() as *const libc::c_void,
                len,
                len as u_int,
            );
        }
    } else {
        px = (px as libc::c_ulong).wrapping_sub(len.wrapping_mul(3 as libc::c_int as libc::c_ulong))
            as u_int as u_int;
        py = (py as libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint) as u_int as u_int;
        tty_attributes(tty, &mut bgc, &grid_default_cell, 0 as *mut libc::c_int);
        ptr = buf.as_mut_ptr();
        while *ptr as libc::c_int != '\u{0}' as i32 {
            if !((*ptr as libc::c_int) < '0' as i32 || *ptr as libc::c_int > '9' as i32) {
                idx = (*ptr as libc::c_int - '0' as i32) as u_int;
                j = 0 as libc::c_int as u_int;
                while j < 5 as libc::c_int as libc::c_uint {
                    i = px;
                    while i < px.wrapping_add(5 as libc::c_int as libc::c_uint) {
                        tty_cursor(
                            tty,
                            xoff.wrapping_add(i),
                            yoff.wrapping_add(py).wrapping_add(j),
                        );
                        if window_clock_table[idx as usize][j as usize][i.wrapping_sub(px) as usize]
                            != 0
                        {
                            tty_putc(tty, ' ' as i32 as u_char);
                        }
                        i = i.wrapping_add(1)
                    }
                    j = j.wrapping_add(1)
                }
                px = (px as libc::c_uint).wrapping_add(6 as libc::c_int as libc::c_uint) as u_int
                    as u_int
            }
            ptr = ptr.offset(1)
        }
        if !(sy <= 6 as libc::c_int as libc::c_uint) {
            tty_attributes(tty, &mut fgc, &grid_default_cell, 0 as *mut libc::c_int);
            if rlen != 0 as libc::c_int as libc::c_ulong && sx as libc::c_ulong >= rlen {
                tty_cursor(
                    tty,
                    (xoff.wrapping_add(sx) as libc::c_ulong).wrapping_sub(rlen) as u_int,
                    yoff,
                );
                tty_putn(
                    tty,
                    rbuf.as_mut_ptr() as *const libc::c_void,
                    rlen,
                    rlen as u_int,
                );
            }
            if llen != 0 as libc::c_int as libc::c_ulong {
                tty_cursor(
                    tty,
                    (xoff.wrapping_add(sx.wrapping_div(2 as libc::c_int as libc::c_uint))
                        as libc::c_ulong)
                        .wrapping_add(len.wrapping_mul(3 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(llen)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as u_int,
                    yoff.wrapping_add(py)
                        .wrapping_add(5 as libc::c_int as libc::c_uint),
                );
                tty_putn(
                    tty,
                    lbuf.as_mut_ptr() as *const libc::c_void,
                    llen,
                    llen as u_int,
                );
            }
        }
    }
    tty_cursor(tty, 0 as libc::c_int as u_int, 0 as libc::c_int as u_int);
}
unsafe extern "C" fn cmd_display_panes_draw(mut c: *mut client, mut ctx: *mut screen_redraw_ctx) {
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    log_debug(
        b"%s: %s @%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"cmd_display_panes_draw\x00"))
            .as_ptr(),
        (*c).name,
        (*w).id,
    );
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if window_pane_visible(wp) != 0 {
            cmd_display_panes_draw_pane(ctx, wp);
        }
        wp = (*wp).entry.tqe_next
    }
}
unsafe extern "C" fn cmd_display_panes_free(mut c: *mut client) {
    let mut cdata: *mut cmd_display_panes_data = (*c).overlay_data as *mut cmd_display_panes_data;
    if !(*cdata).item.is_null() {
        cmdq_continue((*cdata).item);
    }
    free((*cdata).command as *mut libc::c_void);
    free(cdata as *mut libc::c_void);
}
unsafe extern "C" fn cmd_display_panes_key(
    mut c: *mut client,
    mut event: *mut key_event,
) -> libc::c_int {
    let mut cdata: *mut cmd_display_panes_data = (*c).overlay_data as *mut cmd_display_panes_data;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut status: cmd_parse_status = CMD_PARSE_EMPTY;
    let mut index: u_int = 0;
    let mut key: key_code = 0;
    if (*event).key >= '0' as i32 as libc::c_ulonglong
        && (*event).key <= '9' as i32 as libc::c_ulonglong
    {
        index = (*event).key.wrapping_sub('0' as i32 as libc::c_ulonglong) as u_int
    } else if (*event).key & 0xf00000000000 as libc::c_ulonglong
        == 0 as libc::c_int as libc::c_ulonglong
    {
        key = (*event).key & 0xfffffffffff as libc::c_ulonglong;
        if key >= 'a' as i32 as libc::c_ulonglong && key <= 'z' as i32 as libc::c_ulonglong {
            index = (10 as libc::c_int as libc::c_ulonglong)
                .wrapping_add(key.wrapping_sub('a' as i32 as libc::c_ulonglong))
                as u_int
        } else {
            return -(1 as libc::c_int);
        }
    } else {
        return -(1 as libc::c_int);
    }
    wp = window_pane_at_index(w, index);
    if wp.is_null() {
        return 1 as libc::c_int;
    }
    window_unzoom(w);
    xasprintf(
        &mut expanded as *mut *mut libc::c_char,
        b"%%%u\x00" as *const u8 as *const libc::c_char,
        (*wp).id,
    );
    cmd = cmd_template_replace((*cdata).command, expanded, 1 as libc::c_int);
    status = cmd_parse_and_append(
        cmd,
        0 as *mut cmd_parse_input,
        c,
        0 as *mut cmdq_state,
        &mut error,
    );
    if status as libc::c_uint == CMD_PARSE_ERROR as libc::c_int as libc::c_uint {
        cmdq_append(c, cmdq_get_error(error));
        free(error as *mut libc::c_void);
    }
    free(cmd as *mut libc::c_void);
    free(expanded as *mut libc::c_void);
    return 1 as libc::c_int;
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
 * Display panes on a client.
 */
unsafe extern "C" fn cmd_display_panes_exec(
    mut self_0: *mut cmd,
    mut item: *mut cmdq_item,
) -> cmd_retval {
    let mut args: *mut args = cmd_get_args(self_0);
    let mut tc: *mut client = cmdq_get_target_client(item);
    let mut s: *mut session = (*tc).session;
    let mut delay: u_int = 0;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cdata: *mut cmd_display_panes_data = 0 as *mut cmd_display_panes_data;
    if (*tc).overlay_draw.is_some() {
        return CMD_RETURN_NORMAL;
    }
    if args_has(args, 'd' as i32 as u_char) != 0 {
        delay = args_strtonum(
            args,
            'd' as i32 as u_char,
            0 as libc::c_int as libc::c_longlong,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as libc::c_longlong,
            &mut cause,
        ) as u_int;
        if !cause.is_null() {
            cmdq_error(
                item,
                b"delay %s\x00" as *const u8 as *const libc::c_char,
                cause,
            );
            free(cause as *mut libc::c_void);
            return CMD_RETURN_ERROR;
        }
    } else {
        delay = options_get_number(
            (*s).options,
            b"display-panes-time\x00" as *const u8 as *const libc::c_char,
        ) as u_int
    }
    cdata = xmalloc(::std::mem::size_of::<cmd_display_panes_data>() as libc::c_ulong)
        as *mut cmd_display_panes_data;
    if (*args).argc != 0 as libc::c_int {
        (*cdata).command = xstrdup(*(*args).argv.offset(0 as libc::c_int as isize))
    } else {
        (*cdata).command = xstrdup(b"select-pane -t \'%%\'\x00" as *const u8 as *const libc::c_char)
    }
    if args_has(args, 'b' as i32 as u_char) != 0 {
        (*cdata).item = 0 as *mut cmdq_item
    } else {
        (*cdata).item = item
    }
    server_client_set_overlay(
        tc,
        delay,
        None,
        None,
        Some(
            cmd_display_panes_draw
                as unsafe extern "C" fn(_: *mut client, _: *mut screen_redraw_ctx) -> (),
        ),
        Some(
            cmd_display_panes_key
                as unsafe extern "C" fn(_: *mut client, _: *mut key_event) -> libc::c_int,
        ),
        Some(cmd_display_panes_free as unsafe extern "C" fn(_: *mut client) -> ()),
        cdata as *mut libc::c_void,
    );
    if args_has(args, 'b' as i32 as u_char) != 0 {
        return CMD_RETURN_NORMAL;
    }
    return CMD_RETURN_WAIT;
}
