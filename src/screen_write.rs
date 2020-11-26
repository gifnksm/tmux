use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
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
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_pending(ev: *const event, events: libc::c_short, tv: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn event_initialized(ev: *const event) -> libc::c_int;
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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xvasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn format_draw(
        _: *mut screen_write_ctx,
        _: *const grid_cell,
        _: u_int,
        _: *const libc::c_char,
        _: *mut style_ranges,
    );
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    #[no_mangle]
    fn tty_window_offset(
        _: *mut tty,
        _: *mut u_int,
        _: *mut u_int,
        _: *mut u_int,
        _: *mut u_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn tty_update_window_offset(_: *mut window);
    #[no_mangle]
    fn tty_write(
        _: Option<unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()>,
        _: *mut tty_ctx,
    );
    #[no_mangle]
    fn tty_cmd_alignmenttest(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_cell(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_cells(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_clearendofline(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_clearendofscreen(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_clearline(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_clearscreen(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_clearstartofline(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_clearstartofscreen(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_deletecharacter(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_clearcharacter(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_deleteline(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_insertcharacter(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_insertline(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_scrollup(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_scrolldown(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_reverseindex(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_setselection(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_rawstring(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_cmd_syncstart(_: *mut tty, _: *const tty_ctx);
    #[no_mangle]
    fn tty_default_colours(_: *mut grid_cell, _: *mut window_pane);
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    fn grid_cells_equal(_: *const grid_cell, _: *const grid_cell) -> libc::c_int;
    #[no_mangle]
    fn grid_clear_history(_: *mut grid);
    #[no_mangle]
    fn grid_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut grid_cell);
    #[no_mangle]
    fn grid_get_line(_: *mut grid, _: u_int) -> *mut grid_line;
    #[no_mangle]
    fn grid_view_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut grid_cell);
    #[no_mangle]
    fn grid_view_set_cell(_: *mut grid, _: u_int, _: u_int, _: *const grid_cell);
    #[no_mangle]
    fn grid_view_set_padding(_: *mut grid, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_set_cells(
        _: *mut grid,
        _: u_int,
        _: u_int,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: size_t,
    );
    #[no_mangle]
    fn grid_view_clear_history(_: *mut grid, _: u_int);
    #[no_mangle]
    fn grid_view_clear(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_scroll_region_up(_: *mut grid, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_scroll_region_down(_: *mut grid, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_insert_lines(_: *mut grid, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_insert_lines_region(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_delete_lines(_: *mut grid, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_delete_lines_region(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_insert_cells(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_view_delete_cells(_: *mut grid, _: u_int, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_reset_tabs(_: *mut screen);
    #[no_mangle]
    fn screen_select_cell(_: *mut screen, _: *mut grid_cell, _: *const grid_cell);
    #[no_mangle]
    fn screen_check_selection(_: *mut screen, _: u_int, _: u_int) -> libc::c_int;
    #[no_mangle]
    fn status_line_size(_: *mut client) -> u_int;
    #[no_mangle]
    fn status_at_line(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn screen_alternate_on(_: *mut screen, _: *mut grid_cell, _: libc::c_int);
    #[no_mangle]
    fn screen_alternate_off(_: *mut screen, _: *mut grid_cell, _: libc::c_int);
    #[no_mangle]
    fn utf8_fromcstr(_: *const libc::c_char) -> *mut utf8_data;
    #[no_mangle]
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_copy(_: *mut utf8_data, _: *const utf8_data);
    #[no_mangle]
    fn utf8_set(_: *mut utf8_data, _: u_char);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn log_get_level() -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type va_list = __builtin_va_list;
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
    pub exit_type: C2RustUnnamed_31,
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
    pub prompt_mode: C2RustUnnamed_28,
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
pub struct screen_write_collect_line {
    pub bg: u_int,
    pub data: *mut libc::c_char,
    pub items: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub tqh_first: *mut screen_write_collect_item,
    pub tqh_last: *mut *mut screen_write_collect_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_write_collect_item {
    pub x: u_int,
    pub wrapped: libc::c_int,
    pub type_0: C2RustUnnamed_12,
    pub used: u_int,
    pub bg: u_int,
    pub gc: grid_cell,
    pub entry: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub tqe_next: *mut screen_write_collect_item,
    pub tqe_prev: *mut *mut screen_write_collect_item,
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
pub type C2RustUnnamed_12 = libc::c_uint;
pub const CLEAR_START: C2RustUnnamed_12 = 2;
pub const CLEAR_END: C2RustUnnamed_12 = 1;
pub const TEXT: C2RustUnnamed_12 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub offset: u_int,
    pub data: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
    pub gentry: C2RustUnnamed_16,
    pub entry: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
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
    pub entry: C2RustUnnamed_19,
    pub wentry: C2RustUnnamed_18,
    pub sentry: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
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
    pub alerts_entry: C2RustUnnamed_22,
    pub options: *mut options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_21,
    pub entry: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
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
    pub entry: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
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
    pub modes: C2RustUnnamed_26,
    pub searchstr: *mut libc::c_char,
    pub searchregex: libc::c_int,
    pub written: size_t,
    pub skipped: size_t,
    pub border_gc_set: libc::c_int,
    pub border_gc: grid_cell,
    pub entry: C2RustUnnamed_25,
    pub tree_entry: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
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
    pub entry: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
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
pub type C2RustUnnamed_28 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_28 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_28 = 0;
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
    pub entry: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
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
    pub entry: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
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
pub type C2RustUnnamed_31 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_31 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_31 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_31 = 0;
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
    pub entry: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
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
    pub entry: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
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
    pub entry: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
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
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
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
unsafe extern "C" fn screen_write_offset_timer(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut w: *mut window = data as *mut window;
    tty_update_window_offset(w);
}
/* Set cursor position. */
unsafe extern "C" fn screen_write_set_cursor(
    mut ctx: *mut screen_write_ctx,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
) {
    let mut wp: *mut window_pane = (*ctx).wp;
    let mut w: *mut window = 0 as *mut window;
    let mut s: *mut screen = (*ctx).s;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 10000 as libc::c_int as __suseconds_t,
        };
        init
    };
    if cx != -(1 as libc::c_int)
        && cx as u_int == (*s).cx
        && cy != -(1 as libc::c_int)
        && cy as u_int == (*s).cy
    {
        return;
    }
    if cx != -(1 as libc::c_int) {
        if cx as u_int > (*(*s).grid).sx {
            /* allow last column */
            cx = (*(*s).grid)
                .sx
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int
        }
        (*s).cx = cx as u_int
    }
    if cy != -(1 as libc::c_int) {
        if cy as u_int
            > (*(*s).grid)
                .sy
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            cy = (*(*s).grid)
                .sy
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int
        }
        (*s).cy = cy as u_int
    }
    if wp.is_null() {
        return;
    }
    w = (*wp).window;
    if event_initialized(&mut (*w).offset_timer) == 0 {
        event_set(
            &mut (*w).offset_timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                screen_write_offset_timer
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            w as *mut libc::c_void,
        );
    }
    if event_pending(
        &mut (*w).offset_timer,
        0x1 as libc::c_int as libc::c_short,
        0 as *mut timeval,
    ) == 0
    {
        event_add(&mut (*w).offset_timer, &mut tv);
    };
}
/* Do a full redraw. */
unsafe extern "C" fn screen_write_redraw_cb(mut ttyctx: *const tty_ctx) {
    let mut wp: *mut window_pane = (*ttyctx).arg as *mut window_pane;
    if !wp.is_null() {
        (*wp).flags |= 0x1 as libc::c_int
    };
}
/* Update context for client. */
unsafe extern "C" fn screen_write_set_client_cb(
    mut ttyctx: *mut tty_ctx,
    mut c: *mut client,
) -> libc::c_int {
    let mut wp: *mut window_pane = (*ttyctx).arg as *mut window_pane;
    if (*(*(*c).session).curw).window != (*wp).window {
        return 0 as libc::c_int;
    }
    if (*wp).layout_cell.is_null() {
        return 0 as libc::c_int;
    }
    if (*wp).flags & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
        return -(1 as libc::c_int);
    }
    if (*c).flags & 0x20000000 as libc::c_int as libc::c_ulong != 0 {
        /*
         * Redraw is already deferred to redraw another pane - redraw
         * this one also when that happens.
         */
        log_debug(
            b"adding %%%u to deferred redraw\x00" as *const u8 as *const libc::c_char,
            (*wp).id,
        );
        (*wp).flags |= 0x1 as libc::c_int;
        return -(1 as libc::c_int);
    }
    (*ttyctx).bigger = tty_window_offset(
        &mut (*c).tty,
        &mut (*ttyctx).wox,
        &mut (*ttyctx).woy,
        &mut (*ttyctx).wsx,
        &mut (*ttyctx).wsy,
    );
    (*ttyctx).rxoff = (*wp).xoff;
    (*ttyctx).xoff = (*ttyctx).rxoff;
    (*ttyctx).ryoff = (*wp).yoff;
    (*ttyctx).yoff = (*ttyctx).ryoff;
    if status_at_line(c) == 0 as libc::c_int {
        (*ttyctx).yoff =
            ((*ttyctx).yoff as libc::c_uint).wrapping_add(status_line_size(c)) as u_int as u_int
    }
    return 1 as libc::c_int;
}
/* Set up context for TTY command. */
unsafe extern "C" fn screen_write_initctx(
    mut ctx: *mut screen_write_ctx,
    mut ttyctx: *mut tty_ctx,
    mut sync: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
    memset(
        ttyctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tty_ctx>() as libc::c_ulong,
    );
    if !(*ctx).wp.is_null() {
        tty_default_colours(&mut (*ttyctx).defaults, (*ctx).wp);
        (*ttyctx).palette = (*(*ctx).wp).palette
    } else {
        memcpy(
            &mut (*ttyctx).defaults as *mut grid_cell as *mut libc::c_void,
            &grid_default_cell as *const grid_cell as *const libc::c_void,
            ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        (*ttyctx).palette = 0 as *mut libc::c_int
    }
    (*ttyctx).s = s;
    (*ttyctx).sx = (*(*s).grid).sx;
    (*ttyctx).sy = (*(*s).grid).sy;
    (*ttyctx).ocx = (*s).cx;
    (*ttyctx).ocy = (*s).cy;
    (*ttyctx).orlower = (*s).rlower;
    (*ttyctx).orupper = (*s).rupper;
    if (*ctx).init_ctx_cb.is_some() {
        (*ctx).init_ctx_cb.expect("non-null function pointer")(ctx, ttyctx);
    } else {
        (*ttyctx).redraw_cb =
            Some(screen_write_redraw_cb as unsafe extern "C" fn(_: *const tty_ctx) -> ());
        if (*ctx).wp.is_null() {
            (*ttyctx).set_client_cb = None
        } else {
            (*ttyctx).set_client_cb = Some(
                screen_write_set_client_cb
                    as unsafe extern "C" fn(_: *mut tty_ctx, _: *mut client) -> libc::c_int,
            )
        }
        (*ttyctx).arg = (*ctx).wp as *mut libc::c_void
    }
    if !(*ctx).wp.is_null()
        && !(*ctx).flags & 0x1 as libc::c_int != 0
        && (sync != 0 || (*ctx).wp != (*(*(*ctx).wp).window).active)
    {
        tty_write(
            Some(tty_cmd_syncstart as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
            ttyctx,
        );
        (*ctx).flags |= 0x1 as libc::c_int
    };
}
/* Make write list. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_make_list(mut s: *mut screen) {
    let mut y: u_int = 0;
    (*s).write_list = xcalloc(
        (*(*s).grid).sy as size_t,
        ::std::mem::size_of::<screen_write_collect_line>() as libc::c_ulong,
    ) as *mut screen_write_collect_line;
    y = 0 as libc::c_int as u_int;
    while y < (*(*s).grid).sy {
        let ref mut fresh0 = (*(*s).write_list.offset(y as isize)).items.tqh_first;
        *fresh0 = 0 as *mut screen_write_collect_item;
        let ref mut fresh1 = (*(*s).write_list.offset(y as isize)).items.tqh_last;
        *fresh1 = &mut (*(*s).write_list.offset(y as isize)).items.tqh_first;
        y = y.wrapping_add(1)
    }
}
/* Free write list. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_free_list(mut s: *mut screen) {
    let mut y: u_int = 0;
    y = 0 as libc::c_int as u_int;
    while y < (*(*s).grid).sy {
        free((*(*s).write_list.offset(y as isize)).data as *mut libc::c_void);
        y = y.wrapping_add(1)
    }
    free((*s).write_list as *mut libc::c_void);
}
/* Set up for writing. */
unsafe extern "C" fn screen_write_init(mut ctx: *mut screen_write_ctx, mut s: *mut screen) {
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<screen_write_ctx>() as libc::c_ulong,
    );
    (*ctx).s = s;
    if (*(*ctx).s).write_list.is_null() {
        screen_write_make_list((*ctx).s);
    }
    (*ctx).item = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<screen_write_collect_item>() as libc::c_ulong,
    ) as *mut screen_write_collect_item;
    (*ctx).scrolled = 0 as libc::c_int as u_int;
    (*ctx).bg = 8 as libc::c_int as u_int;
}
/* Initialize writing with a pane. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_start_pane(
    mut ctx: *mut screen_write_ctx,
    mut wp: *mut window_pane,
    mut s: *mut screen,
) {
    if s.is_null() {
        s = (*wp).screen
    }
    screen_write_init(ctx, s);
    (*ctx).wp = wp;
    if log_get_level() != 0 as libc::c_int {
        log_debug(
            b"%s: size %ux%u, pane %%%u (at %u,%u)\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"screen_write_start_pane\x00",
            ))
            .as_ptr(),
            (*(*(*ctx).s).grid).sx,
            (*(*(*ctx).s).grid).sy,
            (*wp).id,
            (*wp).xoff,
            (*wp).yoff,
        );
    };
}
/* Initialize writing with a callback. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_start_callback(
    mut ctx: *mut screen_write_ctx,
    mut s: *mut screen,
    mut cb: screen_write_init_ctx_cb,
    mut arg: *mut libc::c_void,
) {
    screen_write_init(ctx, s);
    (*ctx).init_ctx_cb = cb;
    (*ctx).arg = arg;
    if log_get_level() != 0 as libc::c_int {
        log_debug(
            b"%s: size %ux%u, with callback\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"screen_write_start_callback\x00",
            ))
            .as_ptr(),
            (*(*(*ctx).s).grid).sx,
            (*(*(*ctx).s).grid).sy,
        );
    };
}
/* Initialize writing. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_start(mut ctx: *mut screen_write_ctx, mut s: *mut screen) {
    screen_write_init(ctx, s);
    if log_get_level() != 0 as libc::c_int {
        log_debug(
            b"%s: size %ux%u, no pane\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"screen_write_start\x00"))
                .as_ptr(),
            (*(*(*ctx).s).grid).sx,
            (*(*(*ctx).s).grid).sy,
        );
    };
}
/* Finish writing. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_stop(mut ctx: *mut screen_write_ctx) {
    screen_write_collect_end(ctx);
    screen_write_collect_flush(
        ctx,
        0 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"screen_write_stop\x00"))
            .as_ptr(),
    );
    log_debug(
        b"%s: %u cells (%u written, %u skipped)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"screen_write_stop\x00"))
            .as_ptr(),
        (*ctx).cells,
        (*ctx).written,
        (*ctx).skipped,
    );
    if !(*ctx).wp.is_null() {
        (*(*ctx).wp).written = ((*(*ctx).wp).written as libc::c_ulong)
            .wrapping_add((*ctx).written as libc::c_ulong) as size_t
            as size_t;
        (*(*ctx).wp).skipped = ((*(*ctx).wp).skipped as libc::c_ulong)
            .wrapping_add((*ctx).skipped as libc::c_ulong) as size_t
            as size_t
    }
    free((*ctx).item as *mut libc::c_void);
}
/* Reset screen state. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_reset(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    screen_reset_tabs(s);
    screen_write_scrollregion(
        ctx,
        0 as libc::c_int as u_int,
        (*(*s).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    (*s).mode = 0x1 as libc::c_int | 0x10 as libc::c_int;
    screen_write_clearscreen(ctx, 8 as libc::c_int as u_int);
    screen_write_set_cursor(ctx, 0 as libc::c_int, 0 as libc::c_int);
}
/* Write character. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_putc(
    mut ctx: *mut screen_write_ctx,
    mut gcp: *const grid_cell,
    mut ch: u_char,
) {
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
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        gcp as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    utf8_set(&mut gc.data, ch);
    screen_write_cell(ctx, &mut gc);
}
/* Calculate string length. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_strlen(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut ap: ::std::ffi::VaListImpl;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ud: utf8_data = utf8_data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut left: size_t = 0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut more: utf8_state = UTF8_MORE;
    ap = args.clone();
    xvasprintf(&mut msg, fmt, ap.as_va_list());
    ptr = msg as *mut u_char;
    while *ptr as libc::c_int != '\u{0}' as i32 {
        if *ptr as libc::c_int > 0x7f as libc::c_int
            && utf8_open(&mut ud, *ptr) as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint
        {
            ptr = ptr.offset(1);
            left = strlen(ptr as *const libc::c_char);
            if left < (ud.size as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                break;
            }
            loop {
                more = utf8_append(&mut ud, *ptr);
                if !(more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint) {
                    break;
                }
                ptr = ptr.offset(1)
            }
            ptr = ptr.offset(1);
            if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                size = (size as libc::c_ulong).wrapping_add(ud.width as libc::c_ulong) as size_t
                    as size_t
            }
        } else {
            if *ptr as libc::c_int > 0x1f as libc::c_int
                && (*ptr as libc::c_int) < 0x7f as libc::c_int
            {
                size = size.wrapping_add(1)
            }
            ptr = ptr.offset(1)
        }
    }
    free(msg as *mut libc::c_void);
    return size;
}
/* Write string wrapped over lines. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_text(
    mut ctx: *mut screen_write_ctx,
    mut cx: u_int,
    mut width: u_int,
    mut lines: u_int,
    mut more: libc::c_int,
    mut gcp: *const grid_cell,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut s: *mut screen = (*ctx).s;
    let mut ap: ::std::ffi::VaListImpl;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cy: u_int = (*s).cy;
    let mut i: u_int = 0;
    let mut end: u_int = 0;
    let mut next: u_int = 0;
    let mut idx: u_int = 0 as libc::c_int as u_int;
    let mut at: u_int = 0;
    let mut left: u_int = 0;
    let mut text: *mut utf8_data = 0 as *mut utf8_data;
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
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        gcp as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    ap = args.clone();
    xvasprintf(&mut tmp, fmt, ap.as_va_list());
    text = utf8_fromcstr(tmp);
    free(tmp as *mut libc::c_void);
    left = cx.wrapping_add(width).wrapping_sub((*s).cx);
    loop {
        /* Find the end of what can fit on the line. */
        at = 0 as libc::c_int as u_int;
        end = idx;
        while (*text.offset(end as isize)).size as libc::c_int != 0 as libc::c_int {
            if (*text.offset(end as isize)).size as libc::c_int == 1 as libc::c_int
                && (*text.offset(end as isize)).data[0 as libc::c_int as usize] as libc::c_int
                    == '\n' as i32
            {
                break;
            }
            if at.wrapping_add((*text.offset(end as isize)).width as libc::c_uint) > left {
                break;
            }
            at = (at as libc::c_uint)
                .wrapping_add((*text.offset(end as isize)).width as libc::c_uint)
                as u_int as u_int;
            end = end.wrapping_add(1)
        }
        /*
         * If we're on a space, that's the end. If not, walk back to
         * try and find one.
         */
        if (*text.offset(end as isize)).size as libc::c_int == 0 as libc::c_int {
            next = end
        } else if (*text.offset(end as isize)).size as libc::c_int == 1 as libc::c_int
            && (*text.offset(end as isize)).data[0 as libc::c_int as usize] as libc::c_int
                == '\n' as i32
        {
            next = end.wrapping_add(1 as libc::c_int as libc::c_uint)
        } else if (*text.offset(end as isize)).size as libc::c_int == 1 as libc::c_int
            && (*text.offset(end as isize)).data[0 as libc::c_int as usize] as libc::c_int
                == ' ' as i32
        {
            next = end.wrapping_add(1 as libc::c_int as libc::c_uint)
        } else {
            i = end;
            while i > idx {
                if (*text.offset(i as isize)).size as libc::c_int == 1 as libc::c_int
                    && (*text.offset(i as isize)).data[0 as libc::c_int as usize] as libc::c_int
                        == ' ' as i32
                {
                    break;
                }
                i = i.wrapping_sub(1)
            }
            if i != idx {
                next = i.wrapping_add(1 as libc::c_int as libc::c_uint);
                end = i
            } else {
                next = end
            }
        }
        /* Print the line. */
        i = idx;
        while i < end {
            utf8_copy(&mut gc.data, &mut *text.offset(i as isize));
            screen_write_cell(ctx, &mut gc);
            i = i.wrapping_add(1)
        }
        /* If at the bottom, stop. */
        idx = next;
        if (*s).cy
            == cy
                .wrapping_add(lines)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
            || (*text.offset(idx as isize)).size as libc::c_int == 0 as libc::c_int
        {
            break;
        }
        screen_write_cursormove(
            ctx,
            cx as libc::c_int,
            (*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
            0 as libc::c_int,
        );
        left = width
    }
    /*
     * Fail if on the last line and there is more to come or at the end, or
     * if the text was not entirely consumed.
     */
    if (*s).cy
        == cy
            .wrapping_add(lines)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
        && (more == 0 || (*s).cx == cx.wrapping_add(width))
        || (*text.offset(idx as isize)).size as libc::c_int != 0 as libc::c_int
    {
        free(text as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    free(text as *mut libc::c_void);
    /*
     * If no more to come, move to the next line. Otherwise, leave on
     * the same line (except if at the end).
     */
    if more == 0 || (*s).cx == cx.wrapping_add(width) {
        screen_write_cursormove(
            ctx,
            cx as libc::c_int,
            (*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
            0 as libc::c_int,
        );
    }
    return 1 as libc::c_int;
}
/* Write simple string (no maximum length). */
#[no_mangle]
pub unsafe extern "C" fn screen_write_puts(
    mut ctx: *mut screen_write_ctx,
    mut gcp: *const grid_cell,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    screen_write_vnputs(
        ctx,
        -(1 as libc::c_int) as ssize_t,
        gcp,
        fmt,
        ap.as_va_list(),
    );
}
/* Write string with length limit (-1 for unlimited). */
#[no_mangle]
pub unsafe extern "C" fn screen_write_nputs(
    mut ctx: *mut screen_write_ctx,
    mut maxlen: ssize_t,
    mut gcp: *const grid_cell,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    screen_write_vnputs(ctx, maxlen, gcp, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn screen_write_vnputs(
    mut ctx: *mut screen_write_ctx,
    mut maxlen: ssize_t,
    mut gcp: *const grid_cell,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
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
    let mut ud: *mut utf8_data = &mut gc.data;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut u_char = 0 as *mut u_char;
    let mut left: size_t = 0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut more: utf8_state = UTF8_MORE;
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        gcp as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    xvasprintf(&mut msg, fmt, ap.as_va_list());
    ptr = msg as *mut u_char;
    while *ptr as libc::c_int != '\u{0}' as i32 {
        if *ptr as libc::c_int > 0x7f as libc::c_int
            && utf8_open(ud, *ptr) as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint
        {
            ptr = ptr.offset(1);
            left = strlen(ptr as *const libc::c_char);
            if left < ((*ud).size as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
                break;
            }
            loop {
                more = utf8_append(ud, *ptr);
                if !(more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint) {
                    break;
                }
                ptr = ptr.offset(1)
            }
            ptr = ptr.offset(1);
            if more as libc::c_uint != UTF8_DONE as libc::c_int as libc::c_uint {
                continue;
            }
            if maxlen > 0 as libc::c_int as libc::c_long
                && size.wrapping_add((*ud).width as libc::c_ulong) > maxlen as size_t
            {
                while size < maxlen as size_t {
                    screen_write_putc(ctx, &mut gc, ' ' as i32 as u_char);
                    size = size.wrapping_add(1)
                }
                break;
            } else {
                size = (size as libc::c_ulong).wrapping_add((*ud).width as libc::c_ulong) as size_t
                    as size_t;
                screen_write_cell(ctx, &mut gc);
            }
        } else {
            if maxlen > 0 as libc::c_int as libc::c_long
                && size.wrapping_add(1 as libc::c_int as libc::c_ulong) > maxlen as size_t
            {
                break;
            }
            if *ptr as libc::c_int == '\u{1}' as i32 {
                gc.attr = (gc.attr as libc::c_int ^ 0x80 as libc::c_int) as u_short
            } else if *ptr as libc::c_int == '\n' as i32 {
                screen_write_linefeed(ctx, 0 as libc::c_int, 8 as libc::c_int as u_int);
                screen_write_carriagereturn(ctx);
            } else if *ptr as libc::c_int > 0x1f as libc::c_int
                && (*ptr as libc::c_int) < 0x7f as libc::c_int
            {
                size = size.wrapping_add(1);
                screen_write_putc(ctx, &mut gc, *ptr);
            }
            ptr = ptr.offset(1)
        }
    }
    free(msg as *mut libc::c_void);
}
/*
 * Copy from another screen but without the selection stuff. Assumes the target
 * region is already big enough.
 */
#[no_mangle]
pub unsafe extern "C" fn screen_write_fast_copy(
    mut ctx: *mut screen_write_ctx,
    mut src: *mut screen,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut ny: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*src).grid;
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
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    if nx == 0 as libc::c_int as libc::c_uint || ny == 0 as libc::c_int as libc::c_uint {
        return;
    }
    cy = (*s).cy;
    yy = py;
    while yy < py.wrapping_add(ny) {
        if yy >= (*gd).hsize.wrapping_add((*gd).sy) {
            break;
        }
        cx = (*s).cx;
        xx = px;
        while xx < px.wrapping_add(nx) {
            if xx >= (*grid_get_line(gd, yy)).cellsize {
                break;
            }
            grid_get_cell(gd, xx, yy, &mut gc);
            if xx.wrapping_add(gc.data.width as libc::c_uint) > px.wrapping_add(nx) {
                break;
            }
            grid_view_set_cell((*(*ctx).s).grid, cx, cy, &mut gc);
            cx = cx.wrapping_add(1);
            xx = xx.wrapping_add(1)
        }
        cy = cy.wrapping_add(1);
        yy = yy.wrapping_add(1)
    }
}
/* Draw a horizontal line on screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_hline(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
    mut left: libc::c_int,
    mut right: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
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
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut i: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    gc.attr = (gc.attr as libc::c_int | 0x80 as libc::c_int) as u_short;
    screen_write_putc(
        ctx,
        &mut gc,
        if left != 0 { 't' as i32 } else { 'q' as i32 } as u_char,
    );
    i = 1 as libc::c_int as u_int;
    while i < nx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        screen_write_putc(ctx, &mut gc, 'q' as i32 as u_char);
        i = i.wrapping_add(1)
    }
    screen_write_putc(
        ctx,
        &mut gc,
        if right != 0 { 'u' as i32 } else { 'q' as i32 } as u_char,
    );
    screen_write_set_cursor(ctx, cx as libc::c_int, cy as libc::c_int);
}
/* Draw a horizontal line on screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_vline(
    mut ctx: *mut screen_write_ctx,
    mut ny: u_int,
    mut top: libc::c_int,
    mut bottom: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
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
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut i: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    gc.attr = (gc.attr as libc::c_int | 0x80 as libc::c_int) as u_short;
    screen_write_putc(
        ctx,
        &mut gc,
        if top != 0 { 'w' as i32 } else { 'x' as i32 } as u_char,
    );
    i = 1 as libc::c_int as u_int;
    while i < ny.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        screen_write_set_cursor(ctx, cx as libc::c_int, cy.wrapping_add(i) as libc::c_int);
        screen_write_putc(ctx, &mut gc, 'x' as i32 as u_char);
        i = i.wrapping_add(1)
    }
    screen_write_set_cursor(
        ctx,
        cx as libc::c_int,
        cy.wrapping_add(ny)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
    );
    screen_write_putc(
        ctx,
        &mut gc,
        if bottom != 0 { 'v' as i32 } else { 'x' as i32 } as u_char,
    );
    screen_write_set_cursor(ctx, cx as libc::c_int, cy as libc::c_int);
}
/* Draw a menu on screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_menu(
    mut ctx: *mut screen_write_ctx,
    mut menu: *mut menu,
    mut choice: libc::c_int,
    mut choice_gc: *const grid_cell,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut default_gc: grid_cell = grid_cell {
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
    let mut gc: *const grid_cell = &mut default_gc;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    cx = (*s).cx;
    cy = (*s).cy;
    memcpy(
        &mut default_gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    screen_write_box(
        ctx,
        (*menu).width.wrapping_add(4 as libc::c_int as libc::c_uint),
        (*menu).count.wrapping_add(2 as libc::c_int as libc::c_uint),
    );
    screen_write_cursormove(
        ctx,
        cx.wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_int,
        cy as libc::c_int,
        0 as libc::c_int,
    );
    format_draw(
        ctx,
        &mut default_gc,
        (*menu).width,
        (*menu).title,
        0 as *mut style_ranges,
    );
    i = 0 as libc::c_int as u_int;
    while i < (*menu).count {
        name = (*(*menu).items.offset(i as isize)).name;
        if name.is_null() {
            screen_write_cursormove(
                ctx,
                cx as libc::c_int,
                cy.wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(i) as libc::c_int,
                0 as libc::c_int,
            );
            screen_write_hline(
                ctx,
                (*menu).width.wrapping_add(4 as libc::c_int as libc::c_uint),
                1 as libc::c_int,
                1 as libc::c_int,
            );
        } else {
            if choice >= 0 as libc::c_int
                && i == choice as u_int
                && *name as libc::c_int != '-' as i32
            {
                gc = choice_gc
            }
            screen_write_cursormove(
                ctx,
                cx.wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_int,
                cy.wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(i) as libc::c_int,
                0 as libc::c_int,
            );
            j = 0 as libc::c_int as u_int;
            while j < (*menu).width {
                screen_write_putc(ctx, gc, ' ' as i32 as u_char);
                j = j.wrapping_add(1)
            }
            screen_write_cursormove(
                ctx,
                cx.wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_int,
                cy.wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(i) as libc::c_int,
                0 as libc::c_int,
            );
            if *name as libc::c_int == '-' as i32 {
                name = name.offset(1);
                default_gc.attr = (default_gc.attr as libc::c_int | 0x2 as libc::c_int) as u_short;
                format_draw(ctx, gc, (*menu).width, name, 0 as *mut style_ranges);
                default_gc.attr =
                    (default_gc.attr as libc::c_int & !(0x2 as libc::c_int)) as u_short
            } else {
                format_draw(ctx, gc, (*menu).width, name, 0 as *mut style_ranges);
            }
            gc = &mut default_gc
        }
        i = i.wrapping_add(1)
    }
    screen_write_set_cursor(ctx, cx as libc::c_int, cy as libc::c_int);
}
/* Draw a box on screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_box(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
    mut ny: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
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
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut i: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    gc.attr = (gc.attr as libc::c_int | 0x80 as libc::c_int) as u_short;
    screen_write_putc(ctx, &mut gc, 'l' as i32 as u_char);
    i = 1 as libc::c_int as u_int;
    while i < nx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        screen_write_putc(ctx, &mut gc, 'q' as i32 as u_char);
        i = i.wrapping_add(1)
    }
    screen_write_putc(ctx, &mut gc, 'k' as i32 as u_char);
    screen_write_set_cursor(
        ctx,
        cx as libc::c_int,
        cy.wrapping_add(ny)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
    );
    screen_write_putc(ctx, &mut gc, 'm' as i32 as u_char);
    i = 1 as libc::c_int as u_int;
    while i < nx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        screen_write_putc(ctx, &mut gc, 'q' as i32 as u_char);
        i = i.wrapping_add(1)
    }
    screen_write_putc(ctx, &mut gc, 'j' as i32 as u_char);
    i = 1 as libc::c_int as u_int;
    while i < ny.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        screen_write_set_cursor(ctx, cx as libc::c_int, cy.wrapping_add(i) as libc::c_int);
        screen_write_putc(ctx, &mut gc, 'x' as i32 as u_char);
        i = i.wrapping_add(1)
    }
    i = 1 as libc::c_int as u_int;
    while i < ny.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        screen_write_set_cursor(
            ctx,
            cx.wrapping_add(nx)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
            cy.wrapping_add(i) as libc::c_int,
        );
        screen_write_putc(ctx, &mut gc, 'x' as i32 as u_char);
        i = i.wrapping_add(1)
    }
    screen_write_set_cursor(ctx, cx as libc::c_int, cy as libc::c_int);
}
/*
 * Write a preview version of a window. Assumes target area is big enough and
 * already cleared.
 */
#[no_mangle]
pub unsafe extern "C" fn screen_write_preview(
    mut ctx: *mut screen_write_ctx,
    mut src: *mut screen,
    mut nx: u_int,
    mut ny: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
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
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    cx = (*s).cx;
    cy = (*s).cy;
    /*
     * If the cursor is on, pick the area around the cursor, otherwise use
     * the top left.
     */
    if (*src).mode & 0x1 as libc::c_int != 0 {
        px = (*src).cx;
        if px < nx.wrapping_div(3 as libc::c_int as libc::c_uint) {
            px = 0 as libc::c_int as u_int
        } else {
            px = px.wrapping_sub(nx.wrapping_div(3 as libc::c_int as libc::c_uint))
        }
        if px.wrapping_add(nx) > (*(*src).grid).sx {
            if nx > (*(*src).grid).sx {
                px = 0 as libc::c_int as u_int
            } else {
                px = (*(*src).grid).sx.wrapping_sub(nx)
            }
        }
        py = (*src).cy;
        if py < ny.wrapping_div(3 as libc::c_int as libc::c_uint) {
            py = 0 as libc::c_int as u_int
        } else {
            py = py.wrapping_sub(ny.wrapping_div(3 as libc::c_int as libc::c_uint))
        }
        if py.wrapping_add(ny) > (*(*src).grid).sy {
            if ny > (*(*src).grid).sy {
                py = 0 as libc::c_int as u_int
            } else {
                py = (*(*src).grid).sy.wrapping_sub(ny)
            }
        }
    } else {
        px = 0 as libc::c_int as u_int;
        py = 0 as libc::c_int as u_int
    }
    screen_write_fast_copy(ctx, src, px, (*(*src).grid).hsize.wrapping_add(py), nx, ny);
    if (*src).mode & 0x1 as libc::c_int != 0 {
        grid_view_get_cell((*src).grid, (*src).cx, (*src).cy, &mut gc);
        gc.attr = (gc.attr as libc::c_int | 0x10 as libc::c_int) as u_short;
        screen_write_set_cursor(
            ctx,
            cx.wrapping_add((*src).cx.wrapping_sub(px)) as libc::c_int,
            cy.wrapping_add((*src).cy.wrapping_sub(py)) as libc::c_int,
        );
        screen_write_cell(ctx, &mut gc);
    };
}
/* Set a mode. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_mode_set(
    mut ctx: *mut screen_write_ctx,
    mut mode: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
    (*s).mode |= mode;
}
/* Clear a mode. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_mode_clear(
    mut ctx: *mut screen_write_ctx,
    mut mode: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
    (*s).mode &= !mode;
}
/* Cursor up by ny. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursorup(mut ctx: *mut screen_write_ctx, mut ny: u_int) {
    let mut s: *mut screen = (*ctx).s;
    let mut cx: u_int = (*s).cx;
    let mut cy: u_int = (*s).cy;
    if ny == 0 as libc::c_int as libc::c_uint {
        ny = 1 as libc::c_int as u_int
    }
    if cy < (*s).rupper {
        /* Above region. */
        if ny > cy {
            ny = cy
        }
    } else if ny > cy.wrapping_sub((*s).rupper) {
        ny = cy.wrapping_sub((*s).rupper)
    }
    if cx == (*(*s).grid).sx {
        cx = cx.wrapping_sub(1)
    }
    cy = (cy as libc::c_uint).wrapping_sub(ny) as u_int as u_int;
    screen_write_set_cursor(ctx, cx as libc::c_int, cy as libc::c_int);
}
/* Below region. */
/* Cursor down by ny. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursordown(mut ctx: *mut screen_write_ctx, mut ny: u_int) {
    let mut s: *mut screen = (*ctx).s;
    let mut cx: u_int = (*s).cx;
    let mut cy: u_int = (*s).cy;
    if ny == 0 as libc::c_int as libc::c_uint {
        ny = 1 as libc::c_int as u_int
    }
    if cy > (*s).rlower {
        /* Below region. */
        if ny
            > (*(*s).grid)
                .sy
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(cy)
        {
            ny = (*(*s).grid)
                .sy
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub(cy)
        }
    } else if ny > (*s).rlower.wrapping_sub(cy) {
        ny = (*s).rlower.wrapping_sub(cy)
    }
    if cx == (*(*s).grid).sx {
        cx = cx.wrapping_sub(1)
    } else if ny == 0 as libc::c_int as libc::c_uint {
        return;
    }
    cy = (cy as libc::c_uint).wrapping_add(ny) as u_int as u_int;
    screen_write_set_cursor(ctx, cx as libc::c_int, cy as libc::c_int);
}
/* Above region. */
/* Cursor right by nx. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursorright(mut ctx: *mut screen_write_ctx, mut nx: u_int) {
    let mut s: *mut screen = (*ctx).s;
    let mut cx: u_int = (*s).cx;
    let mut cy: u_int = (*s).cy;
    if nx == 0 as libc::c_int as libc::c_uint {
        nx = 1 as libc::c_int as u_int
    }
    if nx
        > (*(*s).grid)
            .sx
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(cx)
    {
        nx = (*(*s).grid)
            .sx
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(cx)
    }
    if nx == 0 as libc::c_int as libc::c_uint {
        return;
    }
    cx = (cx as libc::c_uint).wrapping_add(nx) as u_int as u_int;
    screen_write_set_cursor(ctx, cx as libc::c_int, cy as libc::c_int);
}
/* Cursor left by nx. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursorleft(mut ctx: *mut screen_write_ctx, mut nx: u_int) {
    let mut s: *mut screen = (*ctx).s;
    let mut cx: u_int = (*s).cx;
    let mut cy: u_int = (*s).cy;
    if nx == 0 as libc::c_int as libc::c_uint {
        nx = 1 as libc::c_int as u_int
    }
    if nx > cx {
        nx = cx
    }
    if nx == 0 as libc::c_int as libc::c_uint {
        return;
    }
    cx = (cx as libc::c_uint).wrapping_sub(nx) as u_int as u_int;
    screen_write_set_cursor(ctx, cx as libc::c_int, cy as libc::c_int);
}
/* Backspace; cursor left unless at start of wrapped line when can move up. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_backspace(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut cx: u_int = (*s).cx;
    let mut cy: u_int = (*s).cy;
    if cx == 0 as libc::c_int as libc::c_uint {
        if cy == 0 as libc::c_int as libc::c_uint {
            return;
        }
        gl = grid_get_line(
            (*s).grid,
            (*(*s).grid)
                .hsize
                .wrapping_add(cy)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if (*gl).flags & 0x1 as libc::c_int != 0 {
            cy = cy.wrapping_sub(1);
            cx = (*(*s).grid)
                .sx
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        }
    } else {
        cx = cx.wrapping_sub(1)
    }
    screen_write_set_cursor(ctx, cx as libc::c_int, cy as libc::c_int);
}
/* VT100 alignment test. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_alignmenttest(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
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
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    memcpy(
        &mut gc as *mut grid_cell as *mut libc::c_void,
        &grid_default_cell as *const grid_cell as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    utf8_set(&mut gc.data, 'E' as i32 as u_char);
    yy = 0 as libc::c_int as u_int;
    while yy < (*(*s).grid).sy {
        xx = 0 as libc::c_int as u_int;
        while xx < (*(*s).grid).sx {
            grid_view_set_cell((*s).grid, xx, yy, &mut gc);
            xx = xx.wrapping_add(1)
        }
        yy = yy.wrapping_add(1)
    }
    screen_write_set_cursor(ctx, 0 as libc::c_int, 0 as libc::c_int);
    (*s).rupper = 0 as libc::c_int as u_int;
    (*s).rlower = (*(*s).grid)
        .sy
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
    screen_write_collect_clear(
        ctx,
        0 as libc::c_int as u_int,
        (*(*s).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    tty_write(
        Some(tty_cmd_alignmenttest as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
/* Insert nx characters. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_insertcharacter(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    if nx == 0 as libc::c_int as libc::c_uint {
        nx = 1 as libc::c_int as u_int
    }
    if nx > (*(*s).grid).sx.wrapping_sub((*s).cx) {
        nx = (*(*s).grid).sx.wrapping_sub((*s).cx)
    }
    if nx == 0 as libc::c_int as libc::c_uint {
        return;
    }
    if (*s).cx
        > (*(*s).grid)
            .sx
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx, 0 as libc::c_int);
    ttyctx.bg = bg;
    grid_view_insert_cells((*s).grid, (*s).cx, (*s).cy, nx, bg);
    screen_write_collect_flush(
        ctx,
        0 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
            b"screen_write_insertcharacter\x00",
        ))
        .as_ptr(),
    );
    ttyctx.num = nx;
    tty_write(
        Some(tty_cmd_insertcharacter as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
/* Delete nx characters. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_deletecharacter(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    if nx == 0 as libc::c_int as libc::c_uint {
        nx = 1 as libc::c_int as u_int
    }
    if nx > (*(*s).grid).sx.wrapping_sub((*s).cx) {
        nx = (*(*s).grid).sx.wrapping_sub((*s).cx)
    }
    if nx == 0 as libc::c_int as libc::c_uint {
        return;
    }
    if (*s).cx
        > (*(*s).grid)
            .sx
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx, 0 as libc::c_int);
    ttyctx.bg = bg;
    grid_view_delete_cells((*s).grid, (*s).cx, (*s).cy, nx, bg);
    screen_write_collect_flush(
        ctx,
        0 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
            b"screen_write_deletecharacter\x00",
        ))
        .as_ptr(),
    );
    ttyctx.num = nx;
    tty_write(
        Some(tty_cmd_deletecharacter as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
/* Clear nx characters. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearcharacter(
    mut ctx: *mut screen_write_ctx,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    if nx == 0 as libc::c_int as libc::c_uint {
        nx = 1 as libc::c_int as u_int
    }
    if nx > (*(*s).grid).sx.wrapping_sub((*s).cx) {
        nx = (*(*s).grid).sx.wrapping_sub((*s).cx)
    }
    if nx == 0 as libc::c_int as libc::c_uint {
        return;
    }
    if (*s).cx
        > (*(*s).grid)
            .sx
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx, 0 as libc::c_int);
    ttyctx.bg = bg;
    grid_view_clear(
        (*s).grid,
        (*s).cx,
        (*s).cy,
        nx,
        1 as libc::c_int as u_int,
        bg,
    );
    screen_write_collect_flush(
        ctx,
        0 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
            b"screen_write_clearcharacter\x00",
        ))
        .as_ptr(),
    );
    ttyctx.num = nx;
    tty_write(
        Some(tty_cmd_clearcharacter as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
/* Insert ny lines. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_insertline(
    mut ctx: *mut screen_write_ctx,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    if ny == 0 as libc::c_int as libc::c_uint {
        ny = 1 as libc::c_int as u_int
    }
    if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
        if ny > (*(*s).grid).sy.wrapping_sub((*s).cy) {
            ny = (*(*s).grid).sy.wrapping_sub((*s).cy)
        }
        if ny == 0 as libc::c_int as libc::c_uint {
            return;
        }
        screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
        ttyctx.bg = bg;
        grid_view_insert_lines(gd, (*s).cy, ny, bg);
        screen_write_collect_flush(
            ctx,
            0 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"screen_write_insertline\x00",
            ))
            .as_ptr(),
        );
        ttyctx.num = ny;
        tty_write(
            Some(tty_cmd_insertline as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
            &mut ttyctx,
        );
        return;
    }
    if ny
        > (*s)
            .rlower
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub((*s).cy)
    {
        ny = (*s)
            .rlower
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub((*s).cy)
    }
    if ny == 0 as libc::c_int as libc::c_uint {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
    ttyctx.bg = bg;
    if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
        grid_view_insert_lines(gd, (*s).cy, ny, bg);
    } else {
        grid_view_insert_lines_region(gd, (*s).rlower, (*s).cy, ny, bg);
    }
    screen_write_collect_flush(
        ctx,
        0 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"screen_write_insertline\x00"))
            .as_ptr(),
    );
    ttyctx.num = ny;
    tty_write(
        Some(tty_cmd_insertline as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
/* Delete ny lines. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_deleteline(
    mut ctx: *mut screen_write_ctx,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    if ny == 0 as libc::c_int as libc::c_uint {
        ny = 1 as libc::c_int as u_int
    }
    if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
        if ny > (*(*s).grid).sy.wrapping_sub((*s).cy) {
            ny = (*(*s).grid).sy.wrapping_sub((*s).cy)
        }
        if ny == 0 as libc::c_int as libc::c_uint {
            return;
        }
        screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
        ttyctx.bg = bg;
        grid_view_delete_lines(gd, (*s).cy, ny, bg);
        screen_write_collect_flush(
            ctx,
            0 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"screen_write_deleteline\x00",
            ))
            .as_ptr(),
        );
        ttyctx.num = ny;
        tty_write(
            Some(tty_cmd_deleteline as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
            &mut ttyctx,
        );
        return;
    }
    if ny
        > (*s)
            .rlower
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub((*s).cy)
    {
        ny = (*s)
            .rlower
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_sub((*s).cy)
    }
    if ny == 0 as libc::c_int as libc::c_uint {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
    ttyctx.bg = bg;
    if (*s).cy < (*s).rupper || (*s).cy > (*s).rlower {
        grid_view_delete_lines(gd, (*s).cy, ny, bg);
    } else {
        grid_view_delete_lines_region(gd, (*s).rlower, (*s).cy, ny, bg);
    }
    screen_write_collect_flush(
        ctx,
        0 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"screen_write_deleteline\x00"))
            .as_ptr(),
    );
    ttyctx.num = ny;
    tty_write(
        Some(tty_cmd_deleteline as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
/* Clear line at cursor. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearline(mut ctx: *mut screen_write_ctx, mut bg: u_int) {
    let mut s: *mut screen = (*ctx).s;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut sx: u_int = (*(*s).grid).sx;
    gl = grid_get_line((*s).grid, (*(*s).grid).hsize.wrapping_add((*s).cy));
    if (*gl).cellsize == 0 as libc::c_int as libc::c_uint
        && (bg == 8 as libc::c_int as libc::c_uint || bg == 9 as libc::c_int as libc::c_uint)
    {
        return;
    }
    grid_view_clear(
        (*s).grid,
        0 as libc::c_int as u_int,
        (*s).cy,
        sx,
        1 as libc::c_int as u_int,
        bg,
    );
    screen_write_collect_clear(ctx, (*s).cy, 1 as libc::c_int as u_int);
    (*(*(*ctx).s).write_list.offset((*s).cy as isize)).bg =
        (1 as libc::c_int as libc::c_uint).wrapping_add(bg);
    (*(*ctx).item).used = 0 as libc::c_int as u_int;
}
/* Clear to end of line from cursor. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearendofline(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut sx: u_int = (*(*s).grid).sx;
    let mut ci: *mut screen_write_collect_item = (*ctx).item;
    if (*s).cx == 0 as libc::c_int as libc::c_uint {
        screen_write_clearline(ctx, bg);
        return;
    }
    gl = grid_get_line((*s).grid, (*(*s).grid).hsize.wrapping_add((*s).cy));
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as libc::c_uint)
        || (*s).cx >= (*gl).cellsize
            && (bg == 8 as libc::c_int as libc::c_uint || bg == 9 as libc::c_int as libc::c_uint)
    {
        return;
    }
    grid_view_clear(
        (*s).grid,
        (*s).cx,
        (*s).cy,
        sx.wrapping_sub((*s).cx),
        1 as libc::c_int as u_int,
        bg,
    );
    screen_write_collect_clear_end(ctx, (*s).cy, (*s).cx);
    (*ci).x = (*s).cx;
    (*ci).type_0 = CLEAR_END;
    (*ci).bg = bg;
    (*ci).entry.tqe_next = 0 as *mut screen_write_collect_item;
    (*ci).entry.tqe_prev = (*(*(*ctx).s).write_list.offset((*s).cy as isize))
        .items
        .tqh_last;
    let ref mut fresh2 = *(*(*(*ctx).s).write_list.offset((*s).cy as isize))
        .items
        .tqh_last;
    *fresh2 = ci;
    let ref mut fresh3 = (*(*(*ctx).s).write_list.offset((*s).cy as isize))
        .items
        .tqh_last;
    *fresh3 = &mut (*ci).entry.tqe_next;
    (*ctx).item = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<screen_write_collect_item>() as libc::c_ulong,
    ) as *mut screen_write_collect_item;
}
/* Clear to start of line from cursor. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearstartofline(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut sx: u_int = (*(*s).grid).sx;
    let mut ci: *mut screen_write_collect_item = (*ctx).item;
    if (*s).cx >= sx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        screen_write_clearline(ctx, bg);
        return;
    }
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            (*s).cy,
            sx,
            1 as libc::c_int as u_int,
            bg,
        );
    } else {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            (*s).cy,
            (*s).cx.wrapping_add(1 as libc::c_int as libc::c_uint),
            1 as libc::c_int as u_int,
            bg,
        );
    }
    screen_write_collect_clear_start(ctx, (*s).cy, (*s).cx);
    (*ci).x = (*s).cx;
    (*ci).type_0 = CLEAR_START;
    (*ci).bg = bg;
    (*ci).entry.tqe_next = 0 as *mut screen_write_collect_item;
    (*ci).entry.tqe_prev = (*(*(*ctx).s).write_list.offset((*s).cy as isize))
        .items
        .tqh_last;
    let ref mut fresh4 = *(*(*(*ctx).s).write_list.offset((*s).cy as isize))
        .items
        .tqh_last;
    *fresh4 = ci;
    let ref mut fresh5 = (*(*(*ctx).s).write_list.offset((*s).cy as isize))
        .items
        .tqh_last;
    *fresh5 = &mut (*ci).entry.tqe_next;
    (*ctx).item = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<screen_write_collect_item>() as libc::c_ulong,
    ) as *mut screen_write_collect_item;
}
/* Move cursor to px,py. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_cursormove(
    mut ctx: *mut screen_write_ctx,
    mut px: libc::c_int,
    mut py: libc::c_int,
    mut origin: libc::c_int,
) {
    let mut s: *mut screen = (*ctx).s;
    if origin != 0 && py != -(1 as libc::c_int) && (*s).mode & 0x2000 as libc::c_int != 0 {
        if py as u_int > (*s).rlower.wrapping_sub((*s).rupper) {
            py = (*s).rlower as libc::c_int
        } else {
            py = (py as libc::c_uint).wrapping_add((*s).rupper) as libc::c_int as libc::c_int
        }
    }
    if px != -(1 as libc::c_int)
        && px as u_int
            > (*(*s).grid)
                .sx
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        px = (*(*s).grid)
            .sx
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int
    }
    if py != -(1 as libc::c_int)
        && py as u_int
            > (*(*s).grid)
                .sy
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        py = (*(*s).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int
    }
    screen_write_set_cursor(ctx, px, py);
}
/* Reverse index (up with scroll). */
#[no_mangle]
pub unsafe extern "C" fn screen_write_reverseindex(mut ctx: *mut screen_write_ctx, mut bg: u_int) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    if (*s).cy == (*s).rupper {
        grid_view_scroll_region_down((*s).grid, (*s).rupper, (*s).rlower, bg);
        screen_write_collect_flush(
            ctx,
            0 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"screen_write_reverseindex\x00",
            ))
            .as_ptr(),
        );
        screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
        ttyctx.bg = bg;
        tty_write(
            Some(
                tty_cmd_reverseindex as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> (),
            ),
            &mut ttyctx,
        );
    } else if (*s).cy > 0 as libc::c_int as libc::c_uint {
        screen_write_set_cursor(
            ctx,
            -(1 as libc::c_int),
            (*s).cy.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
        );
    };
}
/* Set scroll region. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_scrollregion(
    mut ctx: *mut screen_write_ctx,
    mut rupper: u_int,
    mut rlower: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    if rupper
        > (*(*s).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        rupper = (*(*s).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    }
    if rlower
        > (*(*s).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        rlower = (*(*s).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    }
    if rupper >= rlower {
        /* cannot be one line */
        return;
    }
    screen_write_collect_flush(
        ctx,
        0 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
            b"screen_write_scrollregion\x00",
        ))
        .as_ptr(),
    );
    /* Cursor moves to top-left. */
    screen_write_set_cursor(ctx, 0 as libc::c_int, 0 as libc::c_int);
    (*s).rupper = rupper;
    (*s).rlower = rlower;
}
/* Line feed. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_linefeed(
    mut ctx: *mut screen_write_ctx,
    mut wrapped: libc::c_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    gl = grid_get_line(gd, (*gd).hsize.wrapping_add((*s).cy));
    if wrapped != 0 {
        (*gl).flags |= 0x1 as libc::c_int
    } else {
        (*gl).flags &= !(0x1 as libc::c_int)
    }
    log_debug(
        b"%s: at %u,%u (region %u-%u)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"screen_write_linefeed\x00"))
            .as_ptr(),
        (*s).cx,
        (*s).cy,
        (*s).rupper,
        (*s).rlower,
    );
    if bg != (*ctx).bg {
        screen_write_collect_flush(
            ctx,
            1 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"screen_write_linefeed\x00",
            ))
            .as_ptr(),
        );
        (*ctx).bg = bg
    }
    if (*s).cy == (*s).rlower {
        grid_view_scroll_region_up(gd, (*s).rupper, (*s).rlower, bg);
        screen_write_collect_scroll(ctx);
        (*ctx).scrolled = (*ctx).scrolled.wrapping_add(1)
    } else if (*s).cy
        < (*(*s).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        screen_write_set_cursor(
            ctx,
            -(1 as libc::c_int),
            (*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
        );
    };
}
/* Scroll up. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_scrollup(
    mut ctx: *mut screen_write_ctx,
    mut lines: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut i: u_int = 0;
    if lines == 0 as libc::c_int as libc::c_uint {
        lines = 1 as libc::c_int as u_int
    } else if lines
        > (*s)
            .rlower
            .wrapping_sub((*s).rupper)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
    {
        lines = (*s)
            .rlower
            .wrapping_sub((*s).rupper)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
    }
    if bg != (*ctx).bg {
        screen_write_collect_flush(
            ctx,
            1 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"screen_write_scrollup\x00",
            ))
            .as_ptr(),
        );
        (*ctx).bg = bg
    }
    i = 0 as libc::c_int as u_int;
    while i < lines {
        grid_view_scroll_region_up(gd, (*s).rupper, (*s).rlower, bg);
        screen_write_collect_scroll(ctx);
        i = i.wrapping_add(1)
    }
    (*ctx).scrolled = ((*ctx).scrolled as libc::c_uint).wrapping_add(lines) as u_int as u_int;
}
/* Scroll down. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_scrolldown(
    mut ctx: *mut screen_write_ctx,
    mut lines: u_int,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    let mut i: u_int = 0;
    screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
    ttyctx.bg = bg;
    if lines == 0 as libc::c_int as libc::c_uint {
        lines = 1 as libc::c_int as u_int
    } else if lines
        > (*s)
            .rlower
            .wrapping_sub((*s).rupper)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
    {
        lines = (*s)
            .rlower
            .wrapping_sub((*s).rupper)
            .wrapping_add(1 as libc::c_int as libc::c_uint)
    }
    i = 0 as libc::c_int as u_int;
    while i < lines {
        grid_view_scroll_region_down(gd, (*s).rupper, (*s).rlower, bg);
        i = i.wrapping_add(1)
    }
    screen_write_collect_flush(
        ctx,
        0 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"screen_write_scrolldown\x00"))
            .as_ptr(),
    );
    ttyctx.num = lines;
    tty_write(
        Some(tty_cmd_scrolldown as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
/* Carriage return (cursor to start of line). */
#[no_mangle]
pub unsafe extern "C" fn screen_write_carriagereturn(mut ctx: *mut screen_write_ctx) {
    screen_write_set_cursor(ctx, 0 as libc::c_int, -(1 as libc::c_int));
}
/* Clear to end of screen from cursor. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearendofscreen(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    let mut sy: u_int = (*(*s).grid).sy;
    screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
    ttyctx.bg = bg;
    /* Scroll into history if it is enabled and clearing entire screen. */
    if (*s).cx == 0 as libc::c_int as libc::c_uint
        && (*s).cy == 0 as libc::c_int as libc::c_uint
        && (*gd).flags & 0x1 as libc::c_int != 0
    {
        grid_view_clear_history(gd, bg);
    } else {
        if (*s).cx <= sx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
            grid_view_clear(
                gd,
                (*s).cx,
                (*s).cy,
                sx.wrapping_sub((*s).cx),
                1 as libc::c_int as u_int,
                bg,
            );
        }
        grid_view_clear(
            gd,
            0 as libc::c_int as u_int,
            (*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint),
            sx,
            sy.wrapping_sub((*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint)),
            bg,
        );
    }
    screen_write_collect_clear(
        ctx,
        (*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint),
        sy.wrapping_sub((*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint)),
    );
    screen_write_collect_flush(
        ctx,
        0 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
            b"screen_write_clearendofscreen\x00",
        ))
        .as_ptr(),
    );
    tty_write(
        Some(
            tty_cmd_clearendofscreen as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
/* Clear to start of screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearstartofscreen(
    mut ctx: *mut screen_write_ctx,
    mut bg: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
    ttyctx.bg = bg;
    if (*s).cy > 0 as libc::c_int as libc::c_uint {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
            sx,
            (*s).cy,
            bg,
        );
    }
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            (*s).cy,
            sx,
            1 as libc::c_int as u_int,
            bg,
        );
    } else {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            (*s).cy,
            (*s).cx.wrapping_add(1 as libc::c_int as libc::c_uint),
            1 as libc::c_int as u_int,
            bg,
        );
    }
    screen_write_collect_clear(ctx, 0 as libc::c_int as u_int, (*s).cy);
    screen_write_collect_flush(
        ctx,
        0 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
            b"screen_write_clearstartofscreen\x00",
        ))
        .as_ptr(),
    );
    tty_write(
        Some(
            tty_cmd_clearstartofscreen
                as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> (),
        ),
        &mut ttyctx,
    );
}
/* Clear entire screen. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearscreen(mut ctx: *mut screen_write_ctx, mut bg: u_int) {
    let mut s: *mut screen = (*ctx).s;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    let mut sy: u_int = (*(*s).grid).sy;
    screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
    ttyctx.bg = bg;
    /* Scroll into history if it is enabled. */
    if (*(*s).grid).flags & 0x1 as libc::c_int != 0 {
        grid_view_clear_history((*s).grid, bg);
    } else {
        grid_view_clear(
            (*s).grid,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
            sx,
            sy,
            bg,
        );
    }
    screen_write_collect_clear(ctx, 0 as libc::c_int as u_int, sy);
    tty_write(
        Some(tty_cmd_clearscreen as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
/* Clear entire history. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_clearhistory(mut ctx: *mut screen_write_ctx) {
    grid_clear_history((*(*ctx).s).grid);
}
/* Clear to start of a collected line. */
unsafe extern "C" fn screen_write_collect_clear_start(
    mut ctx: *mut screen_write_ctx,
    mut y: u_int,
    mut x: u_int,
) {
    let mut ci: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut tmp: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut items: u_int = 0 as libc::c_int as u_int;
    if (*(*(*ctx).s).write_list.offset(y as isize))
        .items
        .tqh_first
        .is_null()
    {
        return;
    }
    let mut current_block_15: u64;
    ci = (*(*(*ctx).s).write_list.offset(y as isize)).items.tqh_first;
    while !ci.is_null() && {
        tmp = (*ci).entry.tqe_next;
        (1 as libc::c_int) != 0
    } {
        match (*ci).type_0 as libc::c_uint {
            1 => {
                if (*ci).x <= x {
                    (*ci).x = x
                }
                current_block_15 = 17179679302217393232;
            }
            0 => {
                if (*ci).x > x {
                    current_block_15 = 17179679302217393232;
                } else {
                    current_block_15 = 12209867499936983673;
                }
            }
            2 | _ => {
                current_block_15 = 12209867499936983673;
            }
        }
        match current_block_15 {
            12209867499936983673 => {
                items = items.wrapping_add(1);
                size = (size as libc::c_ulong).wrapping_add((*ci).used as libc::c_ulong) as size_t
                    as size_t;
                if !(*ci).entry.tqe_next.is_null() {
                    (*(*ci).entry.tqe_next).entry.tqe_prev = (*ci).entry.tqe_prev
                } else {
                    let ref mut fresh6 =
                        (*(*(*ctx).s).write_list.offset(y as isize)).items.tqh_last;
                    *fresh6 = (*ci).entry.tqe_prev
                }
                *(*ci).entry.tqe_prev = (*ci).entry.tqe_next;
                free(ci as *mut libc::c_void);
            }
            _ => {}
        }
        ci = tmp
    }
    (*ctx).skipped = ((*ctx).skipped as libc::c_ulong).wrapping_add(size) as u_int as u_int;
    log_debug(
        b"%s: dropped %u items (%zu bytes) (line %u)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
            b"screen_write_collect_clear_start\x00",
        ))
        .as_ptr(),
        items,
        size,
        y,
    );
}
/* Clear to end of a collected line. */
unsafe extern "C" fn screen_write_collect_clear_end(
    mut ctx: *mut screen_write_ctx,
    mut y: u_int,
    mut x: u_int,
) {
    let mut ci: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut tmp: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut items: u_int = 0 as libc::c_int as u_int;
    if (*(*(*ctx).s).write_list.offset(y as isize))
        .items
        .tqh_first
        .is_null()
    {
        return;
    }
    let mut current_block_15: u64;
    ci = (*(*(*ctx).s).write_list.offset(y as isize)).items.tqh_first;
    while !ci.is_null() && {
        tmp = (*ci).entry.tqe_next;
        (1 as libc::c_int) != 0
    } {
        match (*ci).type_0 as libc::c_uint {
            2 => {
                if (*ci).x >= x {
                    (*ci).x = x
                }
                current_block_15 = 17179679302217393232;
            }
            0 => {
                if (*ci).x < x {
                    current_block_15 = 17179679302217393232;
                } else {
                    current_block_15 = 12209867499936983673;
                }
            }
            1 | _ => {
                current_block_15 = 12209867499936983673;
            }
        }
        match current_block_15 {
            12209867499936983673 => {
                items = items.wrapping_add(1);
                size = (size as libc::c_ulong).wrapping_add((*ci).used as libc::c_ulong) as size_t
                    as size_t;
                if !(*ci).entry.tqe_next.is_null() {
                    (*(*ci).entry.tqe_next).entry.tqe_prev = (*ci).entry.tqe_prev
                } else {
                    let ref mut fresh7 =
                        (*(*(*ctx).s).write_list.offset(y as isize)).items.tqh_last;
                    *fresh7 = (*ci).entry.tqe_prev
                }
                *(*ci).entry.tqe_prev = (*ci).entry.tqe_next;
                free(ci as *mut libc::c_void);
            }
            _ => {}
        }
        ci = tmp
    }
    (*ctx).skipped = ((*ctx).skipped as libc::c_ulong).wrapping_add(size) as u_int as u_int;
    log_debug(
        b"%s: dropped %u items (%zu bytes) (line %u)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
            b"screen_write_collect_clear_end\x00",
        ))
        .as_ptr(),
        items,
        size,
        y,
    );
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
/* Clear collected lines. */
unsafe extern "C" fn screen_write_collect_clear(
    mut ctx: *mut screen_write_ctx,
    mut y: u_int,
    mut n: u_int,
) {
    let mut ci: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut tmp: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut cl: *mut screen_write_collect_line = 0 as *mut screen_write_collect_line;
    let mut i: u_int = 0;
    let mut items: u_int = 0;
    let mut size: size_t = 0;
    i = y;
    while i < y.wrapping_add(n) {
        if !(*(*(*ctx).s).write_list.offset(i as isize))
            .items
            .tqh_first
            .is_null()
        {
            items = 0 as libc::c_int as u_int;
            size = 0 as libc::c_int as size_t;
            cl = &mut *(*(*ctx).s).write_list.offset(i as isize) as *mut screen_write_collect_line;
            ci = (*cl).items.tqh_first;
            while !ci.is_null() && {
                tmp = (*ci).entry.tqe_next;
                (1 as libc::c_int) != 0
            } {
                items = items.wrapping_add(1);
                size = (size as libc::c_ulong).wrapping_add((*ci).used as libc::c_ulong) as size_t
                    as size_t;
                if !(*ci).entry.tqe_next.is_null() {
                    (*(*ci).entry.tqe_next).entry.tqe_prev = (*ci).entry.tqe_prev
                } else {
                    (*cl).items.tqh_last = (*ci).entry.tqe_prev
                }
                *(*ci).entry.tqe_prev = (*ci).entry.tqe_next;
                free(ci as *mut libc::c_void);
                ci = tmp
            }
            (*ctx).skipped = ((*ctx).skipped as libc::c_ulong).wrapping_add(size) as u_int as u_int;
            log_debug(
                b"%s: dropped %u items (%zu bytes) (line %u)\x00" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                    b"screen_write_collect_clear\x00",
                ))
                .as_ptr(),
                items,
                size,
                y,
            );
        }
        i = i.wrapping_add(1)
    }
}
/* Scroll collected lines up. */
unsafe extern "C" fn screen_write_collect_scroll(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    let mut cl: *mut screen_write_collect_line = 0 as *mut screen_write_collect_line;
    let mut y: u_int = 0;
    let mut saved: *mut libc::c_char = 0 as *mut libc::c_char;
    log_debug(
        b"%s: at %u,%u (region %u-%u)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
            b"screen_write_collect_scroll\x00",
        ))
        .as_ptr(),
        (*s).cx,
        (*s).cy,
        (*s).rupper,
        (*s).rlower,
    );
    screen_write_collect_clear(ctx, (*s).rupper, 1 as libc::c_int as u_int);
    saved = (*(*(*ctx).s).write_list.offset((*s).rupper as isize)).data;
    y = (*s).rupper;
    while y < (*s).rlower {
        cl = &mut *(*(*ctx).s)
            .write_list
            .offset(y.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
            as *mut screen_write_collect_line;
        if !(*cl).items.tqh_first.is_null() {
            let ref mut fresh8 = *(*(*(*ctx).s).write_list.offset(y as isize)).items.tqh_last;
            *fresh8 = (*cl).items.tqh_first;
            (*(*cl).items.tqh_first).entry.tqe_prev =
                (*(*(*ctx).s).write_list.offset(y as isize)).items.tqh_last;
            let ref mut fresh9 = (*(*(*ctx).s).write_list.offset(y as isize)).items.tqh_last;
            *fresh9 = (*cl).items.tqh_last;
            (*cl).items.tqh_first = 0 as *mut screen_write_collect_item;
            (*cl).items.tqh_last = &mut (*cl).items.tqh_first
        }
        (*(*(*ctx).s).write_list.offset(y as isize)).bg = (*cl).bg;
        let ref mut fresh10 = (*(*(*ctx).s).write_list.offset(y as isize)).data;
        *fresh10 = (*cl).data;
        y = y.wrapping_add(1)
    }
    (*(*(*ctx).s).write_list.offset((*s).rlower as isize)).bg =
        (1 as libc::c_int + 8 as libc::c_int) as u_int;
    let ref mut fresh11 = (*(*(*ctx).s).write_list.offset((*s).rlower as isize)).data;
    *fresh11 = saved;
}
/* Flush collected lines. */
unsafe extern "C" fn screen_write_collect_flush(
    mut ctx: *mut screen_write_ctx,
    mut scroll_only: libc::c_int,
    mut from: *const libc::c_char,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ci: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut tmp: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut cl: *mut screen_write_collect_line = 0 as *mut screen_write_collect_line;
    let mut y: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut items: u_int = 0 as libc::c_int as u_int;
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    let mut written: size_t = 0 as libc::c_int as size_t;
    if (*ctx).scrolled != 0 as libc::c_int as libc::c_uint {
        log_debug(
            b"%s: scrolled %u (region %u-%u)\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"screen_write_collect_flush\x00",
            ))
            .as_ptr(),
            (*ctx).scrolled,
            (*s).rupper,
            (*s).rlower,
        );
        if (*ctx).scrolled
            > (*s)
                .rlower
                .wrapping_sub((*s).rupper)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        {
            (*ctx).scrolled = (*s)
                .rlower
                .wrapping_sub((*s).rupper)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }
        screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
        ttyctx.num = (*ctx).scrolled;
        ttyctx.bg = (*ctx).bg;
        tty_write(
            Some(tty_cmd_scrollup as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
            &mut ttyctx,
        );
    }
    (*ctx).scrolled = 0 as libc::c_int as u_int;
    (*ctx).bg = 8 as libc::c_int as u_int;
    if scroll_only != 0 {
        return;
    }
    cx = (*s).cx;
    cy = (*s).cy;
    y = 0 as libc::c_int as u_int;
    while y < (*(*s).grid).sy {
        cl = &mut *(*(*ctx).s).write_list.offset(y as isize) as *mut screen_write_collect_line;
        if (*cl).bg != 0 as libc::c_int as libc::c_uint {
            screen_write_set_cursor(ctx, 0 as libc::c_int, y as libc::c_int);
            screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
            ttyctx.bg = (*cl).bg.wrapping_sub(1 as libc::c_int as libc::c_uint);
            tty_write(
                Some(
                    tty_cmd_clearline as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> (),
                ),
                &mut ttyctx,
            );
        }
        ci = (*cl).items.tqh_first;
        while !ci.is_null() && {
            tmp = (*ci).entry.tqe_next;
            (1 as libc::c_int) != 0
        } {
            screen_write_set_cursor(ctx, (*ci).x as libc::c_int, y as libc::c_int);
            if (*ci).type_0 as libc::c_uint == CLEAR_END as libc::c_int as libc::c_uint {
                log_debug(
                    b"XXX %u %u\x00" as *const u8 as *const libc::c_char,
                    (*ci).x,
                    (*ci).bg,
                );
                screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
                ttyctx.bg = (*ci).bg;
                tty_write(
                    Some(
                        tty_cmd_clearendofline
                            as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> (),
                    ),
                    &mut ttyctx,
                );
            } else if (*ci).type_0 as libc::c_uint == CLEAR_START as libc::c_int as libc::c_uint {
                screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
                ttyctx.bg = (*ci).bg;
                tty_write(
                    Some(
                        tty_cmd_clearstartofline
                            as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> (),
                    ),
                    &mut ttyctx,
                );
            } else {
                screen_write_initctx(ctx, &mut ttyctx, 0 as libc::c_int);
                ttyctx.cell = &mut (*ci).gc;
                ttyctx.wrapped = (*ci).wrapped;
                ttyctx.ptr = (*cl).data.offset((*ci).x as isize) as *mut libc::c_void;
                ttyctx.num = (*ci).used;
                tty_write(
                    Some(
                        tty_cmd_cells as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> (),
                    ),
                    &mut ttyctx,
                );
            }
            items = items.wrapping_add(1);
            written = (written as libc::c_ulong).wrapping_add((*ci).used as libc::c_ulong) as size_t
                as size_t;
            if !(*ci).entry.tqe_next.is_null() {
                (*(*ci).entry.tqe_next).entry.tqe_prev = (*ci).entry.tqe_prev
            } else {
                (*cl).items.tqh_last = (*ci).entry.tqe_prev
            }
            *(*ci).entry.tqe_prev = (*ci).entry.tqe_next;
            free(ci as *mut libc::c_void);
            ci = tmp
        }
        (*cl).bg = 0 as libc::c_int as u_int;
        y = y.wrapping_add(1)
    }
    (*s).cx = cx;
    (*s).cy = cy;
    log_debug(
        b"%s: flushed %u items (%zu bytes) (%s)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
            b"screen_write_collect_flush\x00",
        ))
        .as_ptr(),
        items,
        written,
        from,
    );
    (*ctx).written = ((*ctx).written as libc::c_ulong).wrapping_add(written) as u_int as u_int;
}
/* Finish and store collected cells. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_collect_end(mut ctx: *mut screen_write_ctx) {
    let mut s: *mut screen = (*ctx).s;
    let mut ci: *mut screen_write_collect_item = (*ctx).item;
    let mut cl: *mut screen_write_collect_line =
        &mut *(*s).write_list.offset((*s).cy as isize) as *mut screen_write_collect_line;
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
    let mut xx: u_int = 0;
    if (*ci).used == 0 as libc::c_int as libc::c_uint {
        return;
    }
    (*ci).x = (*s).cx;
    (*ci).entry.tqe_next = 0 as *mut screen_write_collect_item;
    (*ci).entry.tqe_prev = (*cl).items.tqh_last;
    *(*cl).items.tqh_last = ci;
    (*cl).items.tqh_last = &mut (*ci).entry.tqe_next;
    (*ctx).item = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<screen_write_collect_item>() as libc::c_ulong,
    ) as *mut screen_write_collect_item;
    log_debug(
        b"%s: %u %.*s (at %u,%u)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"screen_write_collect_end\x00"))
            .as_ptr(),
        (*ci).used,
        (*ci).used as libc::c_int,
        (*cl).data.offset((*ci).x as isize),
        (*s).cx,
        (*s).cy,
    );
    if (*s).cx != 0 as libc::c_int as libc::c_uint {
        xx = (*s).cx;
        while xx > 0 as libc::c_int as libc::c_uint {
            grid_view_get_cell((*s).grid, xx, (*s).cy, &mut gc);
            if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
                break;
            }
            grid_view_set_cell((*s).grid, xx, (*s).cy, &grid_default_cell);
            xx = xx.wrapping_sub(1)
        }
        if gc.data.width as libc::c_int > 1 as libc::c_int {
            grid_view_set_cell((*s).grid, xx, (*s).cy, &grid_default_cell);
        }
    }
    grid_view_set_cells(
        (*s).grid,
        (*s).cx,
        (*s).cy,
        &mut (*ci).gc,
        (*cl).data.offset((*ci).x as isize),
        (*ci).used as size_t,
    );
    screen_write_set_cursor(
        ctx,
        (*s).cx.wrapping_add((*ci).used) as libc::c_int,
        -(1 as libc::c_int),
    );
    xx = (*s).cx;
    while xx < (*(*s).grid).sx {
        grid_view_get_cell((*s).grid, xx, (*s).cy, &mut gc);
        if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
            break;
        }
        grid_view_set_cell((*s).grid, xx, (*s).cy, &grid_default_cell);
        xx = xx.wrapping_add(1)
    }
}
/* Write cell data, collecting if necessary. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_collect_add(
    mut ctx: *mut screen_write_ctx,
    mut gc: *const grid_cell,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut ci: *mut screen_write_collect_item = 0 as *mut screen_write_collect_item;
    let mut sx: u_int = (*(*s).grid).sx;
    let mut collect: libc::c_int = 0;
    /*
     * Don't need to check that the attributes and whatnot are still the
     * same - input_parse will end the collection when anything that isn't
     * a plain character is encountered.
     */
    collect = 1 as libc::c_int; /* may have changed */
    if (*gc).data.width as libc::c_int != 1 as libc::c_int
        || (*gc).data.size as libc::c_int != 1 as libc::c_int
        || *(*gc).data.data.as_ptr() as libc::c_int >= 0x7f as libc::c_int
    {
        collect = 0 as libc::c_int
    } else if (*gc).attr as libc::c_int & 0x80 as libc::c_int != 0 {
        collect = 0 as libc::c_int
    } else if !(*s).mode & 0x10 as libc::c_int != 0 {
        collect = 0 as libc::c_int
    } else if (*s).mode & 0x2 as libc::c_int != 0 {
        collect = 0 as libc::c_int
    } else if !(*s).sel.is_null() {
        collect = 0 as libc::c_int
    }
    if collect == 0 {
        screen_write_collect_end(ctx);
        screen_write_collect_flush(
            ctx,
            0 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"screen_write_collect_add\x00",
            ))
            .as_ptr(),
        );
        screen_write_cell(ctx, gc);
        return;
    }
    (*ctx).cells = (*ctx).cells.wrapping_add(1);
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as libc::c_uint)
        || (*(*ctx).item).used
            > sx.wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub((*s).cx)
    {
        screen_write_collect_end(ctx);
    }
    ci = (*ctx).item;
    if (*s).cx > sx.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        log_debug(
            b"%s: wrapped at %u,%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"screen_write_collect_add\x00",
            ))
            .as_ptr(),
            (*s).cx,
            (*s).cy,
        );
        (*ci).wrapped = 1 as libc::c_int;
        screen_write_linefeed(ctx, 1 as libc::c_int, 8 as libc::c_int as u_int);
        screen_write_set_cursor(ctx, 0 as libc::c_int, -(1 as libc::c_int));
    }
    if (*ci).used == 0 as libc::c_int as libc::c_uint {
        memcpy(
            &mut (*ci).gc as *mut grid_cell as *mut libc::c_void,
            gc as *const libc::c_void,
            ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
    }
    if (*(*(*ctx).s).write_list.offset((*s).cy as isize))
        .data
        .is_null()
    {
        let ref mut fresh12 = (*(*(*ctx).s).write_list.offset((*s).cy as isize)).data;
        *fresh12 = xmalloc((*(*(*ctx).s).grid).sx as size_t) as *mut libc::c_char
    }
    let fresh13 = (*ci).used;
    (*ci).used = (*ci).used.wrapping_add(1);
    *(*(*(*ctx).s).write_list.offset((*s).cy as isize))
        .data
        .offset((*s).cx.wrapping_add(fresh13) as isize) =
        (*gc).data.data[0 as libc::c_int as usize] as libc::c_char;
}
/* Write cell data. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_cell(
    mut ctx: *mut screen_write_ctx,
    mut gc: *const grid_cell,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut gce: *mut grid_cell_entry = 0 as *mut grid_cell_entry;
    let mut tmp_gc: grid_cell = grid_cell {
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
    let mut now_gc: grid_cell = grid_cell {
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
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    let mut sx: u_int = (*(*s).grid).sx;
    let mut sy: u_int = (*(*s).grid).sy;
    let mut width: u_int = (*gc).data.width as u_int;
    let mut xx: u_int = 0;
    let mut last: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut selected: libc::c_int = 0;
    let mut skip: libc::c_int = 1 as libc::c_int;
    /* Ignore padding cells. */
    if (*gc).flags as libc::c_int & 0x4 as libc::c_int != 0 {
        return;
    }
    (*ctx).cells = (*ctx).cells.wrapping_add(1);
    /* If the width is zero, combine onto the previous character. */
    if width == 0 as libc::c_int as libc::c_uint {
        screen_write_collect_flush(
            ctx,
            0 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"screen_write_cell\x00"))
                .as_ptr(),
        );
        gc = screen_write_combine(ctx, &(*gc).data, &mut xx);
        if !gc.is_null() {
            cx = (*s).cx;
            cy = (*s).cy;
            screen_write_set_cursor(ctx, xx as libc::c_int, (*s).cy as libc::c_int);
            screen_write_initctx(ctx, &mut ttyctx, 0 as libc::c_int);
            ttyctx.cell = gc;
            tty_write(
                Some(tty_cmd_cell as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
                &mut ttyctx,
            );
            (*s).cx = cx;
            (*s).cy = cy
        }
        return;
    }
    /* Flush any existing scrolling. */
    screen_write_collect_flush(
        ctx,
        1 as libc::c_int,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"screen_write_cell\x00"))
            .as_ptr(),
    );
    /* If this character doesn't fit, ignore it. */
    if !(*s).mode & 0x10 as libc::c_int != 0
        && width > 1 as libc::c_int as libc::c_uint
        && (width > sx || (*s).cx != sx && (*s).cx > sx.wrapping_sub(width))
    {
        return;
    }
    /* If in insert mode, make space for the cells. */
    if (*s).mode & 0x2 as libc::c_int != 0 {
        grid_view_insert_cells(
            (*s).grid,
            (*s).cx,
            (*s).cy,
            width,
            8 as libc::c_int as u_int,
        );
        skip = 0 as libc::c_int
    }
    /* Check this will fit on the current line and wrap if not. */
    if (*s).mode & 0x10 as libc::c_int != 0 && (*s).cx > sx.wrapping_sub(width) {
        log_debug(
            b"%s: wrapped at %u,%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"screen_write_cell\x00"))
                .as_ptr(),
            (*s).cx,
            (*s).cy,
        );
        screen_write_linefeed(ctx, 1 as libc::c_int, 8 as libc::c_int as u_int);
        screen_write_set_cursor(ctx, 0 as libc::c_int, -(1 as libc::c_int));
        screen_write_collect_flush(
            ctx,
            1 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"screen_write_cell\x00"))
                .as_ptr(),
        );
    }
    /* Sanity check cursor position. */
    if (*s).cx > sx.wrapping_sub(width)
        || (*s).cy > sy.wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return;
    }
    screen_write_initctx(ctx, &mut ttyctx, 0 as libc::c_int);
    /* Handle overwriting of UTF-8 characters. */
    gl = grid_get_line((*s).grid, (*(*s).grid).hsize.wrapping_add((*s).cy));
    if (*gl).flags & 0x2 as libc::c_int != 0 {
        grid_view_get_cell(gd, (*s).cx, (*s).cy, &mut now_gc);
        if screen_write_overwrite(ctx, &mut now_gc, width) != 0 {
            skip = 0 as libc::c_int
        }
    }
    /*
     * If the new character is UTF-8 wide, fill in padding cells. Have
     * already ensured there is enough room.
     */
    xx = (*s).cx.wrapping_add(1 as libc::c_int as libc::c_uint);
    while xx < (*s).cx.wrapping_add(width) {
        log_debug(
            b"%s: new padding at %u,%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"screen_write_cell\x00"))
                .as_ptr(),
            xx,
            (*s).cy,
        );
        grid_view_set_padding(gd, xx, (*s).cy);
        skip = 0 as libc::c_int;
        xx = xx.wrapping_add(1)
    }
    /* If no change, do not draw. */
    if skip != 0 {
        if (*s).cx >= (*gl).cellsize {
            skip = grid_cells_equal(gc, &grid_default_cell)
        } else {
            gce = &mut *(*gl).celldata.offset((*s).cx as isize) as *mut grid_cell_entry;
            if (*gce).flags as libc::c_int & 0x8 as libc::c_int != 0 {
                skip = 0 as libc::c_int
            } else if (*gc).flags as libc::c_int != (*gce).flags as libc::c_int {
                skip = 0 as libc::c_int
            } else if (*gc).attr as libc::c_int != (*gce).c2rust_unnamed.data.attr as libc::c_int {
                skip = 0 as libc::c_int
            } else if (*gc).fg != (*gce).c2rust_unnamed.data.fg as libc::c_int {
                skip = 0 as libc::c_int
            } else if (*gc).bg != (*gce).c2rust_unnamed.data.bg as libc::c_int {
                skip = 0 as libc::c_int
            } else if (*gc).data.width as libc::c_int != 1 as libc::c_int {
                skip = 0 as libc::c_int
            } else if (*gc).data.size as libc::c_int != 1 as libc::c_int {
                skip = 0 as libc::c_int
            } else if (*gce).c2rust_unnamed.data.data as libc::c_int
                != (*gc).data.data[0 as libc::c_int as usize] as libc::c_int
            {
                skip = 0 as libc::c_int
            }
        }
    }
    /* Update the selected flag and set the cell. */
    selected = screen_check_selection(s, (*s).cx, (*s).cy);
    if selected != 0 && !((*gc).flags as libc::c_int) & 0x10 as libc::c_int != 0 {
        memcpy(
            &mut tmp_gc as *mut grid_cell as *mut libc::c_void,
            gc as *const libc::c_void,
            ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        tmp_gc.flags = (tmp_gc.flags as libc::c_int | 0x10 as libc::c_int) as u_char;
        grid_view_set_cell(gd, (*s).cx, (*s).cy, &mut tmp_gc);
    } else if selected == 0 && (*gc).flags as libc::c_int & 0x10 as libc::c_int != 0 {
        memcpy(
            &mut tmp_gc as *mut grid_cell as *mut libc::c_void,
            gc as *const libc::c_void,
            ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        tmp_gc.flags = (tmp_gc.flags as libc::c_int & !(0x10 as libc::c_int)) as u_char;
        grid_view_set_cell(gd, (*s).cx, (*s).cy, &mut tmp_gc);
    } else if skip == 0 {
        grid_view_set_cell(gd, (*s).cx, (*s).cy, gc);
    }
    if selected != 0 {
        skip = 0 as libc::c_int
    }
    /*
     * Move the cursor. If not wrapping, stick at the last character and
     * replace it.
     */
    last = ((*s).mode & 0x10 as libc::c_int == 0) as libc::c_int as u_int;
    if (*s).cx <= sx.wrapping_sub(last).wrapping_sub(width) {
        screen_write_set_cursor(
            ctx,
            (*s).cx.wrapping_add(width) as libc::c_int,
            -(1 as libc::c_int),
        );
    } else {
        screen_write_set_cursor(
            ctx,
            sx.wrapping_sub(last) as libc::c_int,
            -(1 as libc::c_int),
        );
    }
    /* Create space for character in insert mode. */
    if (*s).mode & 0x2 as libc::c_int != 0 {
        screen_write_collect_flush(
            ctx,
            0 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"screen_write_cell\x00"))
                .as_ptr(),
        );
        ttyctx.num = width;
        tty_write(
            Some(
                tty_cmd_insertcharacter
                    as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> (),
            ),
            &mut ttyctx,
        );
    }
    /* Write to the screen. */
    if skip == 0 {
        if selected != 0 {
            screen_select_cell(s, &mut tmp_gc, gc);
            ttyctx.cell = &mut tmp_gc
        } else {
            ttyctx.cell = gc
        }
        tty_write(
            Some(tty_cmd_cell as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
            &mut ttyctx,
        );
        (*ctx).written = (*ctx).written.wrapping_add(1)
    } else {
        (*ctx).skipped = (*ctx).skipped.wrapping_add(1)
    };
}
/* Combine a UTF-8 zero-width character onto the previous. */
unsafe extern "C" fn screen_write_combine(
    mut ctx: *mut screen_write_ctx,
    mut ud: *const utf8_data,
    mut xx: *mut u_int,
) -> *const grid_cell {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    static mut gc: grid_cell = grid_cell {
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
    let mut n: u_int = 0;
    /* Can't combine if at 0. */
    if (*s).cx == 0 as libc::c_int as libc::c_uint {
        return 0 as *const grid_cell;
    }
    /* Empty data is out. */
    if (*ud).size as libc::c_int == 0 as libc::c_int {
        fatalx(b"UTF-8 data empty\x00" as *const u8 as *const libc::c_char);
    }
    /* Retrieve the previous cell. */
    n = 1 as libc::c_int as u_int;
    while n <= (*s).cx {
        grid_view_get_cell(gd, (*s).cx.wrapping_sub(n), (*s).cy, &mut gc);
        if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
            break;
        }
        n = n.wrapping_add(1)
    }
    if n > (*s).cx {
        return 0 as *const grid_cell;
    }
    *xx = (*s).cx.wrapping_sub(n);
    /* Check there is enough space. */
    if (gc.data.size as libc::c_int + (*ud).size as libc::c_int) as libc::c_ulong
        > ::std::mem::size_of::<[u_char; 21]>() as libc::c_ulong
    {
        return 0 as *const grid_cell;
    }
    log_debug(
        b"%s: %.*s onto %.*s at %u,%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"screen_write_combine\x00"))
            .as_ptr(),
        (*ud).size as libc::c_int,
        (*ud).data.as_ptr(),
        gc.data.size as libc::c_int,
        gc.data.data.as_mut_ptr(),
        *xx,
        (*s).cy,
    );
    /* Append the data. */
    memcpy(
        gc.data
            .data
            .as_mut_ptr()
            .offset(gc.data.size as libc::c_int as isize) as *mut libc::c_void,
        (*ud).data.as_ptr() as *const libc::c_void,
        (*ud).size as libc::c_ulong,
    );
    gc.data.size = (gc.data.size as libc::c_int + (*ud).size as libc::c_int) as u_char;
    /* Set the new cell. */
    grid_view_set_cell(gd, *xx, (*s).cy, &mut gc);
    return &mut gc;
}
/*
 * UTF-8 wide characters are a bit of an annoyance. They take up more than one
 * cell on the screen, so following cells must not be drawn by marking them as
 * padding.
 *
 * So far, so good. The problem is, when overwriting a padding cell, or a UTF-8
 * character, it is necessary to also overwrite any other cells which covered
 * by the same character.
 */
unsafe extern "C" fn screen_write_overwrite(
    mut ctx: *mut screen_write_ctx,
    mut gc: *mut grid_cell,
    mut width: u_int,
) -> libc::c_int {
    let mut s: *mut screen = (*ctx).s;
    let mut gd: *mut grid = (*s).grid;
    let mut tmp_gc: grid_cell = grid_cell {
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
    let mut xx: u_int = 0;
    let mut done: libc::c_int = 0 as libc::c_int;
    if (*gc).flags as libc::c_int & 0x4 as libc::c_int != 0 {
        /*
         * A padding cell, so clear any following and leading padding
         * cells back to the character. Don't overwrite the current
         * cell as that happens later anyway.
         */
        xx = (*s).cx.wrapping_add(1 as libc::c_int as libc::c_uint);
        loop {
            xx = xx.wrapping_sub(1);
            if !(xx > 0 as libc::c_int as libc::c_uint) {
                break;
            }
            grid_view_get_cell(gd, xx, (*s).cy, &mut tmp_gc);
            if !(tmp_gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
                break;
            }
            log_debug(
                b"%s: padding at %u,%u\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"screen_write_overwrite\x00",
                ))
                .as_ptr(),
                xx,
                (*s).cy,
            );
            grid_view_set_cell(gd, xx, (*s).cy, &grid_default_cell);
        }
        /* Overwrite the character at the start of this padding. */
        log_debug(
            b"%s: character at %u,%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"screen_write_overwrite\x00",
            ))
            .as_ptr(),
            xx,
            (*s).cy,
        );
        grid_view_set_cell(gd, xx, (*s).cy, &grid_default_cell);
        done = 1 as libc::c_int
    }
    /*
     * Overwrite any padding cells that belong to any UTF-8 characters
     * we'll be overwriting with the current character.
     */
    if width != 1 as libc::c_int as libc::c_uint
        || (*gc).data.width as libc::c_int != 1 as libc::c_int
        || (*gc).flags as libc::c_int & 0x4 as libc::c_int != 0
    {
        xx = (*s)
            .cx
            .wrapping_add(width)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
        loop {
            xx = xx.wrapping_add(1);
            if !(xx < (*(*s).grid).sx) {
                break;
            }
            grid_view_get_cell(gd, xx, (*s).cy, &mut tmp_gc);
            if !(tmp_gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
                break;
            }
            log_debug(
                b"%s: overwrite at %u,%u\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"screen_write_overwrite\x00",
                ))
                .as_ptr(),
                xx,
                (*s).cy,
            );
            grid_view_set_cell(gd, xx, (*s).cy, &grid_default_cell);
            done = 1 as libc::c_int
        }
    }
    return done;
}
/* Set external clipboard. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_setselection(
    mut ctx: *mut screen_write_ctx,
    mut str: *mut u_char,
    mut len: u_int,
) {
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    screen_write_initctx(ctx, &mut ttyctx, 0 as libc::c_int);
    ttyctx.ptr = str as *mut libc::c_void;
    ttyctx.num = len;
    tty_write(
        Some(tty_cmd_setselection as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
/* Write unmodified string. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_rawstring(
    mut ctx: *mut screen_write_ctx,
    mut str: *mut u_char,
    mut len: u_int,
) {
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    screen_write_initctx(ctx, &mut ttyctx, 0 as libc::c_int);
    ttyctx.ptr = str as *mut libc::c_void;
    ttyctx.num = len;
    tty_write(
        Some(tty_cmd_rawstring as unsafe extern "C" fn(_: *mut tty, _: *const tty_ctx) -> ()),
        &mut ttyctx,
    );
}
/* Turn alternate screen on. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_alternateon(
    mut ctx: *mut screen_write_ctx,
    mut gc: *mut grid_cell,
    mut cursor: libc::c_int,
) {
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    let mut wp: *mut window_pane = (*ctx).wp;
    if !wp.is_null()
        && options_get_number(
            (*wp).options,
            b"alternate-screen\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        return;
    }
    screen_alternate_on((*ctx).s, gc, cursor);
    screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
    ttyctx.redraw_cb.expect("non-null function pointer")(&mut ttyctx);
}
/* Turn alternate screen off. */
#[no_mangle]
pub unsafe extern "C" fn screen_write_alternateoff(
    mut ctx: *mut screen_write_ctx,
    mut gc: *mut grid_cell,
    mut cursor: libc::c_int,
) {
    let mut ttyctx: tty_ctx = tty_ctx {
        s: 0 as *mut screen,
        redraw_cb: None,
        set_client_cb: None,
        arg: 0 as *mut libc::c_void,
        cell: 0 as *const grid_cell,
        wrapped: 0,
        num: 0,
        ptr: 0 as *mut libc::c_void,
        ocx: 0,
        ocy: 0,
        orupper: 0,
        orlower: 0,
        xoff: 0,
        yoff: 0,
        rxoff: 0,
        ryoff: 0,
        sx: 0,
        sy: 0,
        bg: 0,
        defaults: grid_cell {
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
        },
        palette: 0 as *mut libc::c_int,
        bigger: 0,
        wox: 0,
        woy: 0,
        wsx: 0,
        wsy: 0,
    };
    let mut wp: *mut window_pane = (*ctx).wp;
    if !wp.is_null()
        && options_get_number(
            (*wp).options,
            b"alternate-screen\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        return;
    }
    screen_alternate_off((*ctx).s, gc, cursor);
    screen_write_initctx(ctx, &mut ttyctx, 1 as libc::c_int);
    ttyctx.redraw_cb.expect("non-null function pointer")(&mut ttyctx);
}
