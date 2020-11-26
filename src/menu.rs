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
    pub type cmdq_state;
    pub type screen_write_collect_item;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn format_single(
        _: *mut cmdq_item,
        _: *const libc::c_char,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn format_single_from_state(
        _: *mut cmdq_item,
        _: *const libc::c_char,
        _: *mut client,
        _: *mut cmd_find_state,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn format_width(_: *const libc::c_char) -> u_int;
    #[no_mangle]
    fn tty_draw_line(
        _: *mut tty,
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: *const grid_cell,
        _: *mut libc::c_int,
    );
    #[no_mangle]
    fn cmd_find_copy_state(_: *mut cmd_find_state, _: *mut cmd_find_state);
    #[no_mangle]
    fn cmd_parse_and_append(
        _: *const libc::c_char,
        _: *mut cmd_parse_input,
        _: *mut client,
        _: *mut cmdq_state,
        _: *mut *mut libc::c_char,
    ) -> cmd_parse_status;
    #[no_mangle]
    fn cmdq_new_state(_: *mut cmd_find_state, _: *mut key_event, _: libc::c_int)
        -> *mut cmdq_state;
    #[no_mangle]
    fn cmdq_free_state(_: *mut cmdq_state);
    #[no_mangle]
    fn cmdq_get_event(_: *mut cmdq_item) -> *mut key_event;
    #[no_mangle]
    fn cmdq_get_error(_: *const libc::c_char) -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_append(_: *mut client, _: *mut cmdq_item) -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_continue(_: *mut cmdq_item);
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int) -> *const libc::c_char;
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
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut screen);
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_menu(
        _: *mut screen_write_ctx,
        _: *mut menu,
        _: libc::c_int,
        _: *const grid_cell,
    );
    #[no_mangle]
    fn screen_write_clearscreen(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_free(_: *mut screen);
    #[no_mangle]
    fn style_apply(_: *mut grid_cell, _: *mut options, _: *const libc::c_char, _: *mut format_tree);
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
pub struct screen_write_ctx {
    pub wp: *mut window_pane,
    pub s: *mut screen,
    pub flags: libc::c_int,
    pub init_ctx_cb: screen_write_init_ctx_cb,
    pub arg: *mut libc::c_void,
    pub item: *mut screen_write_collect_item,
    pub scrolled: u_int,
    pub bg: u_int,
    pub cells: u_int,
    pub written: u_int,
    pub skipped: u_int,
}
pub type screen_write_init_ctx_cb =
    Option<unsafe extern "C" fn(_: *mut screen_write_ctx, _: *mut tty_ctx) -> ()>;
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
pub type tty_ctx_set_client_cb =
    Option<unsafe extern "C" fn(_: *mut tty_ctx, _: *mut client) -> libc::c_int>;
pub type tty_ctx_redraw_cb = Option<unsafe extern "C" fn(_: *const tty_ctx) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_item {
    pub name: *const libc::c_char,
    pub key: key_code,
    pub command: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu {
    pub title: *const libc::c_char,
    pub items: *mut menu_item,
    pub count: u_int,
    pub width: u_int,
}
pub type menu_choice_cb =
    Option<unsafe extern "C" fn(_: *mut menu, _: u_int, _: key_code, _: *mut libc::c_void) -> ()>;
pub type cmd_parse_status = libc::c_uint;
pub const CMD_PARSE_SUCCESS: cmd_parse_status = 2;
pub const CMD_PARSE_ERROR: cmd_parse_status = 1;
pub const CMD_PARSE_EMPTY: cmd_parse_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_parse_input {
    pub flags: libc::c_int,
    pub file: *const libc::c_char,
    pub line: u_int,
    pub item: *mut cmdq_item,
    pub c: *mut client,
    pub fs: cmd_find_state,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_data {
    pub item: *mut cmdq_item,
    pub flags: libc::c_int,
    pub fs: cmd_find_state,
    pub s: screen,
    pub px: u_int,
    pub py: u_int,
    pub menu: *mut menu,
    pub choice: libc::c_int,
    pub cb: menu_choice_cb,
    pub data: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn menu_add_items(
    mut menu: *mut menu,
    mut items: *const menu_item,
    mut qitem: *mut cmdq_item,
    mut c: *mut client,
    mut fs: *mut cmd_find_state,
) {
    let mut loop_0: *const menu_item = 0 as *const menu_item;
    loop_0 = items;
    while !(*loop_0).name.is_null() {
        menu_add_item(menu, loop_0, qitem, c, fs);
        loop_0 = loop_0.offset(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn menu_add_item(
    mut menu: *mut menu,
    mut item: *const menu_item,
    mut qitem: *mut cmdq_item,
    mut c: *mut client,
    mut fs: *mut cmd_find_state,
) {
    let mut new_item: *mut menu_item = 0 as *mut menu_item;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut width: u_int = 0;
    let mut line: libc::c_int = 0;
    line = (item.is_null()
        || (*item).name.is_null()
        || *(*item).name as libc::c_int == '\u{0}' as i32) as libc::c_int;
    if line != 0 && (*menu).count == 0 as libc::c_int as libc::c_uint {
        return;
    }
    (*menu).items = xreallocarray(
        (*menu).items as *mut libc::c_void,
        (*menu).count.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
        ::std::mem::size_of::<menu_item>() as libc::c_ulong,
    ) as *mut menu_item;
    let fresh0 = (*menu).count;
    (*menu).count = (*menu).count.wrapping_add(1);
    new_item = &mut *(*menu).items.offset(fresh0 as isize) as *mut menu_item;
    memset(
        new_item as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<menu_item>() as libc::c_ulong,
    );
    if line != 0 {
        return;
    }
    if !fs.is_null() {
        s = format_single_from_state(qitem, (*item).name, c, fs)
    } else {
        s = format_single(
            qitem,
            (*item).name,
            c,
            0 as *mut session,
            0 as *mut winlink,
            0 as *mut window_pane,
        )
    }
    if *s as libc::c_int == '\u{0}' as i32 {
        /* no item if empty after format expanded */
        (*menu).count = (*menu).count.wrapping_sub(1);
        return;
    }
    if *s as libc::c_int != '-' as i32
        && (*item).key != 0xfe000000000 as libc::c_ulonglong
        && (*item).key != 0xff000000000 as libc::c_ulonglong
    {
        key = key_string_lookup_key((*item).key, 0 as libc::c_int);
        xasprintf(
            &mut name as *mut *mut libc::c_char,
            b"%s#[default] #[align=right](%s)\x00" as *const u8 as *const libc::c_char,
            s,
            key,
        );
    } else {
        xasprintf(
            &mut name as *mut *mut libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            s,
        );
    }
    (*new_item).name = name;
    free(s as *mut libc::c_void);
    cmd = (*item).command;
    if !cmd.is_null() {
        if !fs.is_null() {
            s = format_single_from_state(qitem, cmd, c, fs)
        } else {
            s = format_single(
                qitem,
                cmd,
                c,
                0 as *mut session,
                0 as *mut winlink,
                0 as *mut window_pane,
            )
        }
    } else {
        s = 0 as *mut libc::c_char
    }
    (*new_item).command = s;
    (*new_item).key = (*item).key;
    width = format_width((*new_item).name);
    if width > (*menu).width {
        (*menu).width = width
    };
}
#[no_mangle]
pub unsafe extern "C" fn menu_create(mut title: *const libc::c_char) -> *mut menu {
    let mut menu: *mut menu = 0 as *mut menu;
    menu = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<menu>() as libc::c_ulong,
    ) as *mut menu;
    (*menu).title = xstrdup(title);
    (*menu).width = format_width(title);
    return menu;
}
#[no_mangle]
pub unsafe extern "C" fn menu_free(mut menu: *mut menu) {
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < (*menu).count {
        free((*(*menu).items.offset(i as isize)).name as *mut libc::c_void);
        free((*(*menu).items.offset(i as isize)).command as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free((*menu).items as *mut libc::c_void);
    free((*menu).title as *mut libc::c_void);
    free(menu as *mut libc::c_void);
}
unsafe extern "C" fn menu_mode_cb(
    mut c: *mut client,
    mut _cx: *mut u_int,
    mut _cy: *mut u_int,
) -> *mut screen {
    let mut md: *mut menu_data = (*c).overlay_data as *mut menu_data;
    return &mut (*md).s;
}
unsafe extern "C" fn menu_draw_cb(mut c: *mut client, mut _ctx0: *mut screen_redraw_ctx) {
    let mut md: *mut menu_data = (*c).overlay_data as *mut menu_data;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut s: *mut screen = &mut (*md).s;
    let mut menu: *mut menu = (*md).menu;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut i: u_int = 0;
    let mut px: u_int = (*md).px;
    let mut py: u_int = (*md).py;
    let mut gc: grid_cell = grid_cell {
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
    style_apply(
        &mut gc,
        (*(*(*(*c).session).curw).window).options,
        b"mode-style\x00" as *const u8 as *const libc::c_char,
        0 as *mut format_tree,
    );
    screen_write_start(&mut ctx, s);
    screen_write_clearscreen(&mut ctx, 8 as libc::c_int as u_int);
    screen_write_menu(&mut ctx, menu, (*md).choice, &mut gc);
    screen_write_stop(&mut ctx);
    i = 0 as libc::c_int as u_int;
    while i < (*(*md).s.grid).sy {
        tty_draw_line(
            tty,
            s,
            0 as libc::c_int as u_int,
            i,
            (*menu).width.wrapping_add(4 as libc::c_int as libc::c_uint),
            px,
            py.wrapping_add(i),
            &grid_default_cell,
            0 as *mut libc::c_int,
        );
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn menu_free_cb(mut c: *mut client) {
    let mut md: *mut menu_data = (*c).overlay_data as *mut menu_data;
    if !(*md).item.is_null() {
        cmdq_continue((*md).item);
    }
    if (*md).cb.is_some() {
        (*md).cb.expect("non-null function pointer")(
            (*md).menu,
            (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint),
            0xff000000000 as libc::c_ulonglong,
            (*md).data,
        );
    }
    screen_free(&mut (*md).s);
    menu_free((*md).menu);
    free(md as *mut libc::c_void);
}
unsafe extern "C" fn menu_key_cb(mut c: *mut client, mut event: *mut key_event) -> libc::c_int {
    let mut current_block: u64;
    let mut md: *mut menu_data = (*c).overlay_data as *mut menu_data;
    let mut menu: *mut menu = (*md).menu;
    let mut m: *mut mouse_event = &mut (*event).m;
    let mut i: u_int = 0;
    let mut count: libc::c_int = (*menu).count as libc::c_int;
    let mut old: libc::c_int = (*md).choice;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut item: *const menu_item = 0 as *const menu_item;
    let mut state: *mut cmdq_state = 0 as *mut cmdq_state;
    let mut status: cmd_parse_status = CMD_PARSE_EMPTY;
    let mut error: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*event).key & 0xfffffffffff as libc::c_ulonglong
        >= KEYC_MOUSE as libc::c_ulong as libc::c_ulonglong
        && ((*event).key & 0xfffffffffff as libc::c_ulonglong)
            < KEYC_BSPACE as libc::c_ulong as libc::c_ulonglong
    {
        if (*md).flags & 0x1 as libc::c_int != 0 {
            if (*m).b & 3 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        if (*m).x < (*md).px
            || (*m).x
                > (*md)
                    .px
                    .wrapping_add(4 as libc::c_int as libc::c_uint)
                    .wrapping_add((*menu).width)
            || (*m).y < (*md).py.wrapping_add(1 as libc::c_int as libc::c_uint)
            || (*m).y
                > (*md)
                    .py
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(count as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            if !(*md).flags & 0x4 as libc::c_int != 0 {
                if (*m).b & 3 as libc::c_int as libc::c_uint == 3 as libc::c_int as libc::c_uint {
                    return 1 as libc::c_int;
                }
            } else if !((*m).b & 3 as libc::c_int as libc::c_uint
                == 3 as libc::c_int as libc::c_uint)
                && (*m).b & 64 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
                && (*m).b & 32 as libc::c_int as libc::c_uint == 0
            {
                return 1 as libc::c_int;
            }
            if (*md).choice != -(1 as libc::c_int) {
                (*md).choice = -(1 as libc::c_int);
                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong
            }
            return 0 as libc::c_int;
        }
        if !(*md).flags & 0x4 as libc::c_int != 0 {
            if (*m).b & 3 as libc::c_int as libc::c_uint == 3 as libc::c_int as libc::c_uint {
                current_block = 13409614927677246897;
            } else {
                current_block = 15925075030174552612;
            }
        } else if (*m).b & 64 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint
            && (*m).b & 32 as libc::c_int as libc::c_uint == 0
        {
            current_block = 13409614927677246897;
        } else {
            current_block = 15925075030174552612;
        }
        match current_block {
            13409614927677246897 => {}
            _ => {
                (*md).choice = (*m)
                    .y
                    .wrapping_sub((*md).py.wrapping_add(1 as libc::c_int as libc::c_uint))
                    as libc::c_int;
                if (*md).choice != old {
                    (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong
                }
                return 0 as libc::c_int;
            }
        }
    } else {
        i = 0 as libc::c_int as u_int;
        loop {
            if !(i < count as u_int) {
                current_block = 7746103178988627676;
                break;
            }
            name = (*(*menu).items.offset(i as isize)).name;
            if !(name.is_null() || *name as libc::c_int == '-' as i32) {
                if (*event).key == (*(*menu).items.offset(i as isize)).key {
                    (*md).choice = i as libc::c_int;
                    current_block = 13409614927677246897;
                    break;
                }
            }
            i = i.wrapping_add(1)
        }
        match current_block {
            13409614927677246897 => {}
            _ => {
                match (*event).key & !(0xff000000000000 as libc::c_ulonglong) {
                    68719476908 | 107 => {
                        current_block = 11976616061117098778;
                        match current_block {
                            7328775138574252198 => {
                                if (*md).choice > count - 6 as libc::c_int {
                                    (*md).choice = count - 1 as libc::c_int
                                } else {
                                    (*md).choice += 5 as libc::c_int
                                }
                                while (*md).choice != -(1 as libc::c_int)
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice -= 1
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            7272745134570625308 => {
                                /* C-b */
                                if (*md).choice > 5 as libc::c_int {
                                    (*md).choice -= 5 as libc::c_int
                                } else {
                                    (*md).choice = 0 as libc::c_int
                                }
                                while (*md).choice != count
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice += 1
                                }
                                if (*md).choice == count {
                                    (*md).choice = -(1 as libc::c_int)
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            2215219764755711832 =>
                            /* Tab */
                            {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    if (*md).choice == count - 1 as libc::c_int {
                                        return 1 as libc::c_int;
                                    }
                                    current_block = 10827572169919271282;
                                }
                            }
                            4625365797902618311 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    return 1 as libc::c_int;
                                }
                            }
                            11976616061117098778 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == 0 as libc::c_int
                                    {
                                        (*md).choice = count - 1 as libc::c_int
                                    } else {
                                        (*md).choice -= 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            8312313396388474857 =>
                            /* Escape */
                            {
                                current_block = 5768530447596251282;
                            }
                            _ => {}
                        }
                        match current_block {
                            10827572169919271282 =>
                            /* FALLTHROUGH */
                            {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == count - 1 as libc::c_int
                                    {
                                        (*md).choice = 0 as libc::c_int
                                    } else {
                                        (*md).choice += 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            18383263831861166299 =>
                            /* C-f */
                            {
                                return 0 as libc::c_int
                            }
                            5768530447596251282 =>
                                /* C-c */
                                {}
                            _ => {}
                        }
                        /* C-g */
                        return 1 as libc::c_int;
                    }
                    68719476888 => {
                        current_block = 4625365797902618311;
                        match current_block {
                            7328775138574252198 => {
                                if (*md).choice > count - 6 as libc::c_int {
                                    (*md).choice = count - 1 as libc::c_int
                                } else {
                                    (*md).choice += 5 as libc::c_int
                                }
                                while (*md).choice != -(1 as libc::c_int)
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice -= 1
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            7272745134570625308 => {
                                if (*md).choice > 5 as libc::c_int {
                                    (*md).choice -= 5 as libc::c_int
                                } else {
                                    (*md).choice = 0 as libc::c_int
                                }
                                while (*md).choice != count
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice += 1
                                }
                                if (*md).choice == count {
                                    (*md).choice = -(1 as libc::c_int)
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            2215219764755711832 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    if (*md).choice == count - 1 as libc::c_int {
                                        return 1 as libc::c_int;
                                    }
                                    current_block = 10827572169919271282;
                                }
                            }
                            4625365797902618311 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    return 1 as libc::c_int;
                                }
                            }
                            11976616061117098778 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == 0 as libc::c_int
                                    {
                                        (*md).choice = count - 1 as libc::c_int
                                    } else {
                                        (*md).choice -= 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            8312313396388474857 => {
                                current_block = 5768530447596251282;
                            }
                            _ => {}
                        }
                        match current_block {
                            10827572169919271282 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == count - 1 as libc::c_int
                                    {
                                        (*md).choice = 0 as libc::c_int
                                    } else {
                                        (*md).choice += 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            18383263831861166299 => return 0 as libc::c_int,
                            5768530447596251282 => {}
                            _ => {}
                        }
                        return 1 as libc::c_int;
                    }
                    9 => {
                        current_block = 2215219764755711832;
                        match current_block {
                            7328775138574252198 => {
                                if (*md).choice > count - 6 as libc::c_int {
                                    (*md).choice = count - 1 as libc::c_int
                                } else {
                                    (*md).choice += 5 as libc::c_int
                                }
                                while (*md).choice != -(1 as libc::c_int)
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice -= 1
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            7272745134570625308 => {
                                if (*md).choice > 5 as libc::c_int {
                                    (*md).choice -= 5 as libc::c_int
                                } else {
                                    (*md).choice = 0 as libc::c_int
                                }
                                while (*md).choice != count
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice += 1
                                }
                                if (*md).choice == count {
                                    (*md).choice = -(1 as libc::c_int)
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            2215219764755711832 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    if (*md).choice == count - 1 as libc::c_int {
                                        return 1 as libc::c_int;
                                    }
                                    current_block = 10827572169919271282;
                                }
                            }
                            4625365797902618311 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    return 1 as libc::c_int;
                                }
                            }
                            11976616061117098778 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == 0 as libc::c_int
                                    {
                                        (*md).choice = count - 1 as libc::c_int
                                    } else {
                                        (*md).choice -= 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            8312313396388474857 => {
                                current_block = 5768530447596251282;
                            }
                            _ => {}
                        }
                        match current_block {
                            10827572169919271282 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == count - 1 as libc::c_int
                                    {
                                        (*md).choice = 0 as libc::c_int
                                    } else {
                                        (*md).choice += 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            18383263831861166299 => return 0 as libc::c_int,
                            5768530447596251282 => {}
                            _ => {}
                        }
                        return 1 as libc::c_int;
                    }
                    68719476909 | 106 => {
                        current_block = 10827572169919271282;
                        match current_block {
                            7328775138574252198 => {
                                if (*md).choice > count - 6 as libc::c_int {
                                    (*md).choice = count - 1 as libc::c_int
                                } else {
                                    (*md).choice += 5 as libc::c_int
                                }
                                while (*md).choice != -(1 as libc::c_int)
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice -= 1
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            7272745134570625308 => {
                                if (*md).choice > 5 as libc::c_int {
                                    (*md).choice -= 5 as libc::c_int
                                } else {
                                    (*md).choice = 0 as libc::c_int
                                }
                                while (*md).choice != count
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice += 1
                                }
                                if (*md).choice == count {
                                    (*md).choice = -(1 as libc::c_int)
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            2215219764755711832 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    if (*md).choice == count - 1 as libc::c_int {
                                        return 1 as libc::c_int;
                                    }
                                    current_block = 10827572169919271282;
                                }
                            }
                            4625365797902618311 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    return 1 as libc::c_int;
                                }
                            }
                            11976616061117098778 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == 0 as libc::c_int
                                    {
                                        (*md).choice = count - 1 as libc::c_int
                                    } else {
                                        (*md).choice -= 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            8312313396388474857 => {
                                current_block = 5768530447596251282;
                            }
                            _ => {}
                        }
                        match current_block {
                            10827572169919271282 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == count - 1 as libc::c_int
                                    {
                                        (*md).choice = 0 as libc::c_int
                                    } else {
                                        (*md).choice += 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            18383263831861166299 => return 0 as libc::c_int,
                            5768530447596251282 => {}
                            _ => {}
                        }
                        return 1 as libc::c_int;
                    }
                    103 | 68719476906 | 2 => {
                        current_block = 7272745134570625308;
                        match current_block {
                            7328775138574252198 => {
                                if (*md).choice > count - 6 as libc::c_int {
                                    (*md).choice = count - 1 as libc::c_int
                                } else {
                                    (*md).choice += 5 as libc::c_int
                                }
                                while (*md).choice != -(1 as libc::c_int)
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice -= 1
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            7272745134570625308 => {
                                if (*md).choice > 5 as libc::c_int {
                                    (*md).choice -= 5 as libc::c_int
                                } else {
                                    (*md).choice = 0 as libc::c_int
                                }
                                while (*md).choice != count
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice += 1
                                }
                                if (*md).choice == count {
                                    (*md).choice = -(1 as libc::c_int)
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            2215219764755711832 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    if (*md).choice == count - 1 as libc::c_int {
                                        return 1 as libc::c_int;
                                    }
                                    current_block = 10827572169919271282;
                                }
                            }
                            4625365797902618311 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    return 1 as libc::c_int;
                                }
                            }
                            11976616061117098778 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == 0 as libc::c_int
                                    {
                                        (*md).choice = count - 1 as libc::c_int
                                    } else {
                                        (*md).choice -= 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            8312313396388474857 => {
                                current_block = 5768530447596251282;
                            }
                            _ => {}
                        }
                        match current_block {
                            10827572169919271282 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == count - 1 as libc::c_int
                                    {
                                        (*md).choice = 0 as libc::c_int
                                    } else {
                                        (*md).choice += 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            18383263831861166299 => return 0 as libc::c_int,
                            5768530447596251282 => {}
                            _ => {}
                        }
                        return 1 as libc::c_int;
                    }
                    71 | 68719476905 => {
                        current_block = 7328775138574252198;
                        match current_block {
                            7328775138574252198 => {
                                if (*md).choice > count - 6 as libc::c_int {
                                    (*md).choice = count - 1 as libc::c_int
                                } else {
                                    (*md).choice += 5 as libc::c_int
                                }
                                while (*md).choice != -(1 as libc::c_int)
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice -= 1
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            7272745134570625308 => {
                                if (*md).choice > 5 as libc::c_int {
                                    (*md).choice -= 5 as libc::c_int
                                } else {
                                    (*md).choice = 0 as libc::c_int
                                }
                                while (*md).choice != count
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice += 1
                                }
                                if (*md).choice == count {
                                    (*md).choice = -(1 as libc::c_int)
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            2215219764755711832 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    if (*md).choice == count - 1 as libc::c_int {
                                        return 1 as libc::c_int;
                                    }
                                    current_block = 10827572169919271282;
                                }
                            }
                            4625365797902618311 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    return 1 as libc::c_int;
                                }
                            }
                            11976616061117098778 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == 0 as libc::c_int
                                    {
                                        (*md).choice = count - 1 as libc::c_int
                                    } else {
                                        (*md).choice -= 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            8312313396388474857 => {
                                current_block = 5768530447596251282;
                            }
                            _ => {}
                        }
                        match current_block {
                            10827572169919271282 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == count - 1 as libc::c_int
                                    {
                                        (*md).choice = 0 as libc::c_int
                                    } else {
                                        (*md).choice += 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            18383263831861166299 => return 0 as libc::c_int,
                            5768530447596251282 => {}
                            _ => {}
                        }
                        return 1 as libc::c_int;
                    }
                    13 => {}
                    27 => {
                        current_block = 8312313396388474857;
                        match current_block {
                            7328775138574252198 => {
                                if (*md).choice > count - 6 as libc::c_int {
                                    (*md).choice = count - 1 as libc::c_int
                                } else {
                                    (*md).choice += 5 as libc::c_int
                                }
                                while (*md).choice != -(1 as libc::c_int)
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice -= 1
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            7272745134570625308 => {
                                if (*md).choice > 5 as libc::c_int {
                                    (*md).choice -= 5 as libc::c_int
                                } else {
                                    (*md).choice = 0 as libc::c_int
                                }
                                while (*md).choice != count
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice += 1
                                }
                                if (*md).choice == count {
                                    (*md).choice = -(1 as libc::c_int)
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            2215219764755711832 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    if (*md).choice == count - 1 as libc::c_int {
                                        return 1 as libc::c_int;
                                    }
                                    current_block = 10827572169919271282;
                                }
                            }
                            4625365797902618311 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    return 1 as libc::c_int;
                                }
                            }
                            11976616061117098778 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == 0 as libc::c_int
                                    {
                                        (*md).choice = count - 1 as libc::c_int
                                    } else {
                                        (*md).choice -= 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            8312313396388474857 => {
                                current_block = 5768530447596251282;
                            }
                            _ => {}
                        }
                        match current_block {
                            10827572169919271282 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == count - 1 as libc::c_int
                                    {
                                        (*md).choice = 0 as libc::c_int
                                    } else {
                                        (*md).choice += 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            18383263831861166299 => return 0 as libc::c_int,
                            5768530447596251282 => {}
                            _ => {}
                        }
                        return 1 as libc::c_int;
                    }
                    3 => {
                        current_block = 5768530447596251282;
                        match current_block {
                            7328775138574252198 => {
                                if (*md).choice > count - 6 as libc::c_int {
                                    (*md).choice = count - 1 as libc::c_int
                                } else {
                                    (*md).choice += 5 as libc::c_int
                                }
                                while (*md).choice != -(1 as libc::c_int)
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice -= 1
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            7272745134570625308 => {
                                if (*md).choice > 5 as libc::c_int {
                                    (*md).choice -= 5 as libc::c_int
                                } else {
                                    (*md).choice = 0 as libc::c_int
                                }
                                while (*md).choice != count
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice += 1
                                }
                                if (*md).choice == count {
                                    (*md).choice = -(1 as libc::c_int)
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            2215219764755711832 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    if (*md).choice == count - 1 as libc::c_int {
                                        return 1 as libc::c_int;
                                    }
                                    current_block = 10827572169919271282;
                                }
                            }
                            4625365797902618311 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    return 1 as libc::c_int;
                                }
                            }
                            11976616061117098778 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == 0 as libc::c_int
                                    {
                                        (*md).choice = count - 1 as libc::c_int
                                    } else {
                                        (*md).choice -= 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            8312313396388474857 => {
                                current_block = 5768530447596251282;
                            }
                            _ => {}
                        }
                        match current_block {
                            10827572169919271282 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == count - 1 as libc::c_int
                                    {
                                        (*md).choice = 0 as libc::c_int
                                    } else {
                                        (*md).choice += 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            18383263831861166299 => return 0 as libc::c_int,
                            5768530447596251282 => {}
                            _ => {}
                        }
                        return 1 as libc::c_int;
                    }
                    7 | 113 => {
                        current_block = 13300134386176516907;
                        match current_block {
                            7328775138574252198 => {
                                if (*md).choice > count - 6 as libc::c_int {
                                    (*md).choice = count - 1 as libc::c_int
                                } else {
                                    (*md).choice += 5 as libc::c_int
                                }
                                while (*md).choice != -(1 as libc::c_int)
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice -= 1
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            7272745134570625308 => {
                                if (*md).choice > 5 as libc::c_int {
                                    (*md).choice -= 5 as libc::c_int
                                } else {
                                    (*md).choice = 0 as libc::c_int
                                }
                                while (*md).choice != count
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice += 1
                                }
                                if (*md).choice == count {
                                    (*md).choice = -(1 as libc::c_int)
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            2215219764755711832 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    if (*md).choice == count - 1 as libc::c_int {
                                        return 1 as libc::c_int;
                                    }
                                    current_block = 10827572169919271282;
                                }
                            }
                            4625365797902618311 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    return 1 as libc::c_int;
                                }
                            }
                            11976616061117098778 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == 0 as libc::c_int
                                    {
                                        (*md).choice = count - 1 as libc::c_int
                                    } else {
                                        (*md).choice -= 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            8312313396388474857 => {
                                current_block = 5768530447596251282;
                            }
                            _ => {}
                        }
                        match current_block {
                            10827572169919271282 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == count - 1 as libc::c_int
                                    {
                                        (*md).choice = 0 as libc::c_int
                                    } else {
                                        (*md).choice += 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            18383263831861166299 => return 0 as libc::c_int,
                            5768530447596251282 => {}
                            _ => {}
                        }
                        return 1 as libc::c_int;
                    }
                    6 | _ => {
                        current_block = 18383263831861166299;
                        match current_block {
                            7328775138574252198 => {
                                if (*md).choice > count - 6 as libc::c_int {
                                    (*md).choice = count - 1 as libc::c_int
                                } else {
                                    (*md).choice += 5 as libc::c_int
                                }
                                while (*md).choice != -(1 as libc::c_int)
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice -= 1
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            7272745134570625308 => {
                                if (*md).choice > 5 as libc::c_int {
                                    (*md).choice -= 5 as libc::c_int
                                } else {
                                    (*md).choice = 0 as libc::c_int
                                }
                                while (*md).choice != count
                                    && (name.is_null() || *name as libc::c_int == '-' as i32)
                                {
                                    (*md).choice += 1
                                }
                                if (*md).choice == count {
                                    (*md).choice = -(1 as libc::c_int)
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                current_block = 18383263831861166299;
                            }
                            2215219764755711832 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    if (*md).choice == count - 1 as libc::c_int {
                                        return 1 as libc::c_int;
                                    }
                                    current_block = 10827572169919271282;
                                }
                            }
                            4625365797902618311 => {
                                if !(*md).flags & 0x2 as libc::c_int != 0 {
                                    current_block = 18383263831861166299;
                                } else {
                                    return 1 as libc::c_int;
                                }
                            }
                            11976616061117098778 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == 0 as libc::c_int
                                    {
                                        (*md).choice = count - 1 as libc::c_int
                                    } else {
                                        (*md).choice -= 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            8312313396388474857 => {
                                current_block = 5768530447596251282;
                            }
                            _ => {}
                        }
                        match current_block {
                            10827572169919271282 => {
                                if old == -(1 as libc::c_int) {
                                    old = 0 as libc::c_int
                                }
                                loop {
                                    if (*md).choice == -(1 as libc::c_int)
                                        || (*md).choice == count - 1 as libc::c_int
                                    {
                                        (*md).choice = 0 as libc::c_int
                                    } else {
                                        (*md).choice += 1
                                    }
                                    name = (*(*menu).items.offset((*md).choice as isize)).name;
                                    if !((name.is_null() || *name as libc::c_int == '-' as i32)
                                        && (*md).choice != old)
                                    {
                                        break;
                                    }
                                }
                                (*c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                            18383263831861166299 => return 0 as libc::c_int,
                            5768530447596251282 => {}
                            _ => {}
                        }
                        return 1 as libc::c_int;
                    }
                }
            }
        }
    }
    if (*md).choice == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    item = &mut *(*menu).items.offset((*md).choice as isize) as *mut menu_item;
    if (*item).name.is_null() || *(*item).name as libc::c_int == '-' as i32 {
        if (*md).flags & 0x4 as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    if (*md).cb.is_some() {
        (*md).cb.expect("non-null function pointer")(
            (*md).menu,
            (*md).choice as u_int,
            (*item).key,
            (*md).data,
        );
        (*md).cb = None;
        return 1 as libc::c_int;
    }
    if !(*md).item.is_null() {
        event = cmdq_get_event((*md).item)
    } else {
        event = 0 as *mut key_event
    }
    state = cmdq_new_state(&mut (*md).fs, event, 0 as libc::c_int);
    status = cmd_parse_and_append(
        (*item).command,
        0 as *mut cmd_parse_input,
        c,
        state,
        &mut error,
    );
    if status as libc::c_uint == CMD_PARSE_ERROR as libc::c_int as libc::c_uint {
        cmdq_append(c, cmdq_get_error(error));
        free(error as *mut libc::c_void);
    }
    cmdq_free_state(state);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn menu_display(
    mut menu: *mut menu,
    mut flags: libc::c_int,
    mut item: *mut cmdq_item,
    mut px: u_int,
    mut py: u_int,
    mut c: *mut client,
    mut fs: *mut cmd_find_state,
    mut cb: menu_choice_cb,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut md: *mut menu_data = 0 as *mut menu_data;
    let mut i: u_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if (*c).tty.sx < (*menu).width.wrapping_add(4 as libc::c_int as libc::c_uint)
        || (*c).tty.sy < (*menu).count.wrapping_add(2 as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    if px
        .wrapping_add((*menu).width)
        .wrapping_add(4 as libc::c_int as libc::c_uint)
        > (*c).tty.sx
    {
        px = (*c)
            .tty
            .sx
            .wrapping_sub((*menu).width)
            .wrapping_sub(4 as libc::c_int as libc::c_uint)
    }
    if py
        .wrapping_add((*menu).count)
        .wrapping_add(2 as libc::c_int as libc::c_uint)
        > (*c).tty.sy
    {
        py = (*c)
            .tty
            .sy
            .wrapping_sub((*menu).count)
            .wrapping_sub(2 as libc::c_int as libc::c_uint)
    }
    md = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<menu_data>() as libc::c_ulong,
    ) as *mut menu_data;
    (*md).item = item;
    (*md).flags = flags;
    if !fs.is_null() {
        cmd_find_copy_state(&mut (*md).fs, fs);
    }
    screen_init(
        &mut (*md).s,
        (*menu).width.wrapping_add(4 as libc::c_int as libc::c_uint),
        (*menu).count.wrapping_add(2 as libc::c_int as libc::c_uint),
        0 as libc::c_int as u_int,
    );
    if !(*md).flags & 0x1 as libc::c_int != 0 {
        (*md).s.mode |= 0x1000 as libc::c_int
    }
    (*md).s.mode &= !(0x1 as libc::c_int);
    (*md).px = px;
    (*md).py = py;
    (*md).menu = menu;
    if (*md).flags & 0x1 as libc::c_int != 0 {
        i = 0 as libc::c_int as u_int;
        while i < (*menu).count {
            name = (*(*menu).items.offset(i as isize)).name;
            if !name.is_null() && *name as libc::c_int != '-' as i32 {
                break;
            }
            i = i.wrapping_add(1)
        }
        if i != (*menu).count {
            (*md).choice = i as libc::c_int
        } else {
            (*md).choice = -(1 as libc::c_int)
        }
    } else {
        (*md).choice = -(1 as libc::c_int)
    }
    (*md).cb = cb;
    (*md).data = data;
    server_client_set_overlay(
        c,
        0 as libc::c_int as u_int,
        None,
        Some(
            menu_mode_cb
                as unsafe extern "C" fn(
                    _: *mut client,
                    _: *mut u_int,
                    _: *mut u_int,
                ) -> *mut screen,
        ),
        Some(menu_draw_cb as unsafe extern "C" fn(_: *mut client, _: *mut screen_redraw_ctx) -> ()),
        Some(menu_key_cb as unsafe extern "C" fn(_: *mut client, _: *mut key_event) -> libc::c_int),
        Some(menu_free_cb as unsafe extern "C" fn(_: *mut client) -> ()),
        md as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
