use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type args_value;
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
    pub type cmd;
    pub type tmuxpeer;
    pub type options_array_item;
    pub type options_entry;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xvasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_s_options: *mut crate::options::options;
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    fn cfg_add_cause(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn format_create(
        _: *mut client,
        _: *mut cmdq_item,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut crate::format::format_tree;
    #[no_mangle]
    fn format_free(_: *mut crate::format::format_tree);
    #[no_mangle]
    fn format_merge(_: *mut crate::format::format_tree, _: *mut crate::format::format_tree);
    #[no_mangle]
    fn format_add(
        _: *mut crate::format::format_tree,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn options_get(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> *mut crate::options::options_entry;
    #[no_mangle]
    fn options_array_first(
        _: *mut crate::options::options_entry,
    ) -> *mut crate::options::options_array_item;
    #[no_mangle]
    fn options_array_next(
        _: *mut crate::options::options_array_item,
    ) -> *mut crate::options::options_array_item;
    #[no_mangle]
    fn options_array_item_value(_: *mut crate::options::options_array_item) -> *mut options_value;
    #[no_mangle]
    fn args_print(_: *mut args) -> *mut libc::c_char;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn args_first(_: *mut args, _: *mut *mut crate::arguments::args_entry) -> u_char;
    #[no_mangle]
    fn args_next(_: *mut *mut crate::arguments::args_entry) -> u_char;
    #[no_mangle]
    fn args_first_value(
        _: *mut args,
        _: u_char,
        _: *mut *mut crate::arguments::args_value,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn args_next_value(_: *mut *mut crate::arguments::args_value) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_find_target(
        _: *mut cmd_find_state,
        _: *mut cmdq_item,
        _: *const libc::c_char,
        _: cmd_find_type,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_client(_: *mut cmdq_item, _: *const libc::c_char, _: libc::c_int) -> *mut client;
    #[no_mangle]
    fn cmd_find_clear_state(_: *mut cmd_find_state, _: libc::c_int);
    #[no_mangle]
    fn cmd_find_valid_state(_: *mut cmd_find_state) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_copy_state(_: *mut cmd_find_state, _: *mut cmd_find_state);
    #[no_mangle]
    fn cmd_find_from_client(_: *mut cmd_find_state, _: *mut client, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_get_entry(_: *mut crate::cmd::cmd) -> *const cmd_entry;
    #[no_mangle]
    fn cmd_get_args(_: *mut crate::cmd::cmd) -> *mut args;
    #[no_mangle]
    fn cmd_get_group(_: *mut crate::cmd::cmd) -> u_int;
    #[no_mangle]
    fn cmd_get_source(_: *mut crate::cmd::cmd, _: *mut *const libc::c_char, _: *mut u_int);
    #[no_mangle]
    fn cmd_print(_: *mut crate::cmd::cmd) -> *mut libc::c_char;
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list);
    #[no_mangle]
    fn cmd_list_first(_: *mut cmd_list) -> *mut crate::cmd::cmd;
    #[no_mangle]
    fn cmd_list_next(_: *mut crate::cmd::cmd) -> *mut crate::cmd::cmd;
    #[no_mangle]
    fn file_error(_: *mut client, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn server_add_message(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn server_client_unref(_: *mut client);
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn server_client_get_pane(_: *mut client) -> *mut window_pane;
    #[no_mangle]
    fn file_print(_: *mut client, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn status_message_set(
        _: *mut client,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn window_copy_add(_: *mut window_pane, _: *const libc::c_char, _: ...);
    #[no_mangle]
    static window_view_mode: window_mode;
    #[no_mangle]
    fn window_pane_set_mode(
        _: *mut window_pane,
        _: *mut window_pane,
        _: *const window_mode,
        _: *mut cmd_find_state,
        _: *mut args,
    ) -> libc::c_int;
    #[no_mangle]
    fn control_write(_: *mut client, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn utf8_sanitize(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn log_get_level() -> libc::c_int;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
}
pub type __builtin_va_list = [__va_list_tag; 1];

#[repr(C)]
#[derive(Copy, Clone)]
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
pub type __int32_t = libc::c_int;
pub type __uint64_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type va_list = __builtin_va_list;

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
    pub rbh_root: *mut crate::arguments::args_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct client {
    pub name: *const libc::c_char,
    pub peer: *mut crate::proc::tmuxpeer,
    pub queue: *mut cmdq_list,
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
    pub saved_cell: grid_cell,
    pub saved_flags: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: *mut crate::screen::screen_sel,
    pub write_list: *mut crate::screen_write::screen_write_collect_line,
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
    pub options: *mut crate::options::options,
    pub flags: libc::c_int,
    pub attached: u_int,
    pub tio: *mut termios,
    pub environ: *mut crate::environ::environ,
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
    pub options: *mut crate::options::options,
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
    pub list: *mut crate::cmd::cmds,
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
    pub codes: *mut crate::tty_term::tty_code,
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
/* Command queue. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_list {
    pub item: *mut cmdq_item,
    pub list: cmdq_item_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_item_list {
    pub tqh_first: *mut cmdq_item,
    pub tqh_last: *mut *mut cmdq_item,
}
/* Command queue item. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_item {
    pub name: *mut libc::c_char,
    pub queue: *mut cmdq_list,
    pub next: *mut cmdq_item,
    pub client: *mut client,
    pub target_client: *mut client,
    pub type_0: cmdq_type,
    pub group: u_int,
    pub number: u_int,
    pub time: time_t,
    pub flags: libc::c_int,
    pub state: *mut cmdq_state,
    pub source: cmd_find_state,
    pub target: cmd_find_state,
    pub cmdlist: *mut cmd_list,
    pub cmd: *mut crate::cmd::cmd,
    pub cb: cmdq_cb,
    pub data: *mut libc::c_void,
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub tqe_next: *mut cmdq_item,
    pub tqe_prev: *mut *mut cmdq_item,
}
pub type cmdq_cb =
    Option<unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void) -> cmd_retval>;
pub type cmd_retval = libc::c_int;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
/*
 * Command queue state. This is the context for commands on the command queue.
 * It holds information about how the commands were fired (the key and flags),
 * any additional formats for the commands, and the current default target.
 * Multiple commands can share the same state and a command may update the
 * default target.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_state {
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub formats: *mut crate::format::format_tree,
    pub event: key_event,
    pub current: cmd_find_state,
}
/* Command queue item type. */
pub type cmdq_type = libc::c_uint;
pub const CMDQ_CALLBACK: cmdq_type = 1;
pub const CMDQ_COMMAND: cmdq_type = 0;
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

#[repr(C)]
#[derive(Copy, Clone)]
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
pub type cmd_find_type = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;

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
    pub args: C2RustUnnamed_33,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec:
        Option<unsafe extern "C" fn(_: *mut crate::cmd::cmd, _: *mut cmdq_item) -> cmd_retval>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct options_array {
    pub rbh_root: *mut crate::options::options_array_item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union options_value {
    pub string: *mut libc::c_char,
    pub number: libc::c_longlong,
    pub style: style,
    pub array: options_array,
    pub cmdlist: *mut cmd_list,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
/* Get command queue name. */
unsafe extern "C" fn cmdq_name(mut c: *mut client) -> *const libc::c_char {
    static mut s: [libc::c_char; 256] = [0; 256];
    if c.is_null() {
        return b"<global>\x00" as *const u8 as *const libc::c_char;
    }
    if !(*c).name.is_null() {
        xsnprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"<%s>\x00" as *const u8 as *const libc::c_char,
            (*c).name,
        );
    } else {
        xsnprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"<%p>\x00" as *const u8 as *const libc::c_char,
            c,
        );
    }
    return s.as_mut_ptr();
}
/* Get command queue from client. */
unsafe extern "C" fn cmdq_get(mut c: *mut client) -> *mut cmdq_list {
    static mut global_queue: *mut cmdq_list = 0 as *const cmdq_list as *mut cmdq_list;
    if c.is_null() {
        if global_queue.is_null() {
            global_queue = cmdq_new()
        }
        return global_queue;
    }
    return (*c).queue;
}
/* Create a queue. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_new() -> *mut cmdq_list {
    let mut queue: *mut cmdq_list = 0 as *mut cmdq_list;
    queue = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cmdq_list>() as libc::c_ulong,
    ) as *mut cmdq_list;
    (*queue).list.tqh_first = 0 as *mut cmdq_item;
    (*queue).list.tqh_last = &mut (*queue).list.tqh_first;
    return queue;
}
/* Free a queue. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_free(mut queue: *mut cmdq_list) {
    if !(*queue).list.tqh_first.is_null() {
        fatalx(b"queue not empty\x00" as *const u8 as *const libc::c_char);
    }
    free(queue as *mut libc::c_void);
}
/* Get item name. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_name(mut item: *mut cmdq_item) -> *const libc::c_char {
    return (*item).name;
}
/* Get item client. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_client(mut item: *mut cmdq_item) -> *mut client {
    return (*item).client;
}
/* Get item target client. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_target_client(mut item: *mut cmdq_item) -> *mut client {
    return (*item).target_client;
}
/* Get item state. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_state(mut item: *mut cmdq_item) -> *mut cmdq_state {
    return (*item).state;
}
/* Get item target. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_target(mut item: *mut cmdq_item) -> *mut cmd_find_state {
    return &mut (*item).target;
}
/* Get item source. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_source(mut item: *mut cmdq_item) -> *mut cmd_find_state {
    return &mut (*item).source;
}
/* Get state event. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_event(mut item: *mut cmdq_item) -> *mut key_event {
    return &mut (*(*item).state).event;
}
/* Get state current target. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_current(mut item: *mut cmdq_item) -> *mut cmd_find_state {
    return &mut (*(*item).state).current;
}
/* Get state flags. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_flags(mut item: *mut cmdq_item) -> libc::c_int {
    return (*(*item).state).flags;
}
/* Create a new state. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_new_state(
    mut current: *mut cmd_find_state,
    mut event: *mut key_event,
    mut flags: libc::c_int,
) -> *mut cmdq_state {
    let mut state: *mut cmdq_state = 0 as *mut cmdq_state;
    state = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cmdq_state>() as libc::c_ulong,
    ) as *mut cmdq_state;
    (*state).references = 1 as libc::c_int;
    (*state).flags = flags;
    if !event.is_null() {
        memcpy(
            &mut (*state).event as *mut key_event as *mut libc::c_void,
            event as *const libc::c_void,
            ::std::mem::size_of::<key_event>() as libc::c_ulong,
        );
    } else {
        (*state).event.key = 0xff000000000 as libc::c_ulonglong
    }
    if !current.is_null() && cmd_find_valid_state(current) != 0 {
        cmd_find_copy_state(&mut (*state).current, current);
    } else {
        cmd_find_clear_state(&mut (*state).current, 0 as libc::c_int);
    }
    return state;
}
/* Add a reference to a state. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_link_state(mut state: *mut cmdq_state) -> *mut cmdq_state {
    (*state).references += 1;
    return state;
}
/* Make a copy of a state. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_copy_state(mut state: *mut cmdq_state) -> *mut cmdq_state {
    return cmdq_new_state(&mut (*state).current, &mut (*state).event, (*state).flags);
}
/* Free a state. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_free_state(mut state: *mut cmdq_state) {
    (*state).references -= 1;
    if (*state).references != 0 as libc::c_int {
        return;
    }
    if !(*state).formats.is_null() {
        format_free((*state).formats);
    }
    free(state as *mut libc::c_void);
}
/* Add a format to command queue. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_add_format(
    mut state: *mut cmdq_state,
    mut key: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    xvasprintf(&mut value, fmt, ap.as_va_list());
    if (*state).formats.is_null() {
        (*state).formats = format_create(
            0 as *mut client,
            0 as *mut cmdq_item,
            0 as libc::c_int,
            0 as libc::c_int,
        )
    }
    format_add(
        (*state).formats,
        key,
        b"%s\x00" as *const u8 as *const libc::c_char,
        value,
    );
    free(value as *mut libc::c_void);
}
/* Merge formats from item. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_merge_formats(
    mut item: *mut cmdq_item,
    mut ft: *mut crate::format::format_tree,
) {
    let mut entry: *const cmd_entry = 0 as *const cmd_entry;
    if !(*item).cmd.is_null() {
        entry = cmd_get_entry((*item).cmd);
        format_add(
            ft,
            b"command\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*entry).name,
        );
    }
    if !(*(*item).state).formats.is_null() {
        format_merge(ft, (*(*item).state).formats);
    };
}
/* Append an item. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_append(
    mut c: *mut client,
    mut item: *mut cmdq_item,
) -> *mut cmdq_item {
    let mut queue: *mut cmdq_list = cmdq_get(c);
    let mut next: *mut cmdq_item = 0 as *mut cmdq_item;
    loop {
        next = (*item).next;
        (*item).next = 0 as *mut cmdq_item;
        if !c.is_null() {
            (*c).references += 1
        }
        (*item).client = c;
        (*item).queue = queue;
        (*item).entry.tqe_next = 0 as *mut cmdq_item;
        (*item).entry.tqe_prev = (*queue).list.tqh_last;
        *(*queue).list.tqh_last = item;
        (*queue).list.tqh_last = &mut (*item).entry.tqe_next;
        log_debug(
            b"%s %s: %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"cmdq_append\x00")).as_ptr(),
            cmdq_name(c),
            (*item).name,
        );
        item = next;
        if item.is_null() {
            break;
        }
    }
    return *(*((*queue).list.tqh_last as *mut cmdq_item_list)).tqh_last;
}
/* Insert an item. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_insert_after(
    mut after: *mut cmdq_item,
    mut item: *mut cmdq_item,
) -> *mut cmdq_item {
    let mut c: *mut client = (*after).client;
    let mut queue: *mut cmdq_list = (*after).queue;
    let mut next: *mut cmdq_item = 0 as *mut cmdq_item;
    loop {
        next = (*item).next;
        (*item).next = (*after).next;
        (*after).next = item;
        if !c.is_null() {
            (*c).references += 1
        }
        (*item).client = c;
        (*item).queue = queue;
        (*item).entry.tqe_next = (*after).entry.tqe_next;
        if !(*item).entry.tqe_next.is_null() {
            (*(*item).entry.tqe_next).entry.tqe_prev = &mut (*item).entry.tqe_next
        } else {
            (*queue).list.tqh_last = &mut (*item).entry.tqe_next
        }
        (*after).entry.tqe_next = item;
        (*item).entry.tqe_prev = &mut (*after).entry.tqe_next;
        log_debug(
            b"%s %s: %s after %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"cmdq_insert_after\x00"))
                .as_ptr(),
            cmdq_name(c),
            (*item).name,
            (*after).name,
        );
        after = item;
        item = next;
        if item.is_null() {
            break;
        }
    }
    return after;
}
/* Insert a hook. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_insert_hook(
    mut s: *mut session,
    mut item: *mut cmdq_item,
    mut current: *mut cmd_find_state,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut state: *mut cmdq_state = (*item).state;
    let mut cmd: *mut crate::cmd::cmd = (*item).cmd;
    let mut args_0: *mut args = cmd_get_args(cmd);
    let mut entryp: *mut crate::arguments::args_entry = 0 as *mut crate::arguments::args_entry;
    let mut valuep: *mut crate::arguments::args_value = 0 as *mut crate::arguments::args_value;
    let mut oo: *mut crate::options::options = 0 as *mut crate::options::options;
    let mut ap: ::std::ffi::VaListImpl;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 32] = [0; 32];
    let mut flag: libc::c_char = 0;
    let mut arguments: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    let mut new_item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut new_state: *mut cmdq_state = 0 as *mut cmdq_state;
    let mut o: *mut crate::options::options_entry = 0 as *mut crate::options::options_entry;
    let mut a: *mut crate::options::options_array_item =
        0 as *mut crate::options::options_array_item;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    if (*(*item).state).flags & 0x4 as libc::c_int != 0 {
        return;
    }
    if s.is_null() {
        oo = global_s_options
    } else {
        oo = (*s).options
    }
    ap = args.clone();
    xvasprintf(&mut name, fmt, ap.as_va_list());
    o = options_get(oo, name);
    if o.is_null() {
        free(name as *mut libc::c_void);
        return;
    }
    log_debug(
        b"running hook %s (parent %p)\x00" as *const u8 as *const libc::c_char,
        name,
        item,
    );
    /*
     * The hooks get a new state because they should not update the current
     * target or formats for any subsequent commands.
     */
    new_state = cmdq_new_state(current, &mut (*state).event, 0x4 as libc::c_int);
    cmdq_add_format(
        new_state,
        b"hook\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        name,
    );
    arguments = args_print(args_0);
    cmdq_add_format(
        new_state,
        b"hook_arguments\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        arguments,
    );
    free(arguments as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < (*args_0).argc {
        xsnprintf(
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"hook_argument_%d\x00" as *const u8 as *const libc::c_char,
            i,
        );
        cmdq_add_format(
            new_state,
            tmp.as_mut_ptr(),
            b"%s\x00" as *const u8 as *const libc::c_char,
            *(*args_0).argv.offset(i as isize),
        );
        i += 1
    }
    flag = args_first(args_0, &mut entryp) as libc::c_char;
    while flag as libc::c_int != 0 as libc::c_int {
        value = args_get(args_0, flag as u_char);
        if value.is_null() {
            xsnprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"hook_flag_%c\x00" as *const u8 as *const libc::c_char,
                flag as libc::c_int,
            );
            cmdq_add_format(
                new_state,
                tmp.as_mut_ptr(),
                b"1\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            xsnprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"hook_flag_%c\x00" as *const u8 as *const libc::c_char,
                flag as libc::c_int,
            );
            cmdq_add_format(
                new_state,
                tmp.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char,
                value,
            );
        }
        i = 0 as libc::c_int;
        value = args_first_value(args_0, flag as u_char, &mut valuep);
        while !value.is_null() {
            xsnprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"hook_flag_%c_%d\x00" as *const u8 as *const libc::c_char,
                flag as libc::c_int,
                i,
            );
            cmdq_add_format(
                new_state,
                tmp.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char,
                value,
            );
            i += 1;
            value = args_next_value(&mut valuep)
        }
        flag = args_next(&mut entryp) as libc::c_char
    }
    a = options_array_first(o);
    while !a.is_null() {
        cmdlist = (*options_array_item_value(a)).cmdlist;
        if !cmdlist.is_null() {
            new_item = cmdq_get_command(cmdlist, new_state);
            if !item.is_null() {
                item = cmdq_insert_after(item, new_item)
            } else {
                item = cmdq_append(0 as *mut client, new_item)
            }
        }
        a = options_array_next(a)
    }
    cmdq_free_state(new_state);
    free(name as *mut libc::c_void);
}
/* Continue processing command queue. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_continue(mut item: *mut cmdq_item) {
    (*item).flags &= !(0x2 as libc::c_int);
}
/* Remove an item. */
unsafe extern "C" fn cmdq_remove(mut item: *mut cmdq_item) {
    if !(*item).client.is_null() {
        server_client_unref((*item).client);
    }
    if !(*item).cmdlist.is_null() {
        cmd_list_free((*item).cmdlist);
    }
    cmdq_free_state((*item).state);
    if !(*item).entry.tqe_next.is_null() {
        (*(*item).entry.tqe_next).entry.tqe_prev = (*item).entry.tqe_prev
    } else {
        (*(*item).queue).list.tqh_last = (*item).entry.tqe_prev
    }
    *(*item).entry.tqe_prev = (*item).entry.tqe_next;
    free((*item).name as *mut libc::c_void);
    free(item as *mut libc::c_void);
}
/* Remove all subsequent items that match this item's group. */
unsafe extern "C" fn cmdq_remove_group(mut item: *mut cmdq_item) {
    let mut this: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut next: *mut cmdq_item = 0 as *mut cmdq_item;
    if (*item).group == 0 as libc::c_int as libc::c_uint {
        return;
    }
    this = (*item).entry.tqe_next;
    while !this.is_null() {
        next = (*this).entry.tqe_next;
        if (*this).group == (*item).group {
            cmdq_remove(this);
        }
        this = next
    }
}
/* Get a command for the command queue. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_command(
    mut cmdlist: *mut cmd_list,
    mut state: *mut cmdq_state,
) -> *mut cmdq_item {
    let mut item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut first: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut last: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut cmd: *mut crate::cmd::cmd = 0 as *mut crate::cmd::cmd;
    let mut entry: *const cmd_entry = 0 as *const cmd_entry;
    let mut created: libc::c_int = 0 as libc::c_int;
    if state.is_null() {
        state = cmdq_new_state(
            0 as *mut cmd_find_state,
            0 as *mut key_event,
            0 as libc::c_int,
        );
        created = 1 as libc::c_int
    }
    cmd = cmd_list_first(cmdlist);
    while !cmd.is_null() {
        entry = cmd_get_entry(cmd);
        item = xcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<cmdq_item>() as libc::c_ulong,
        ) as *mut cmdq_item;
        xasprintf(
            &mut (*item).name as *mut *mut libc::c_char,
            b"[%s/%p]\x00" as *const u8 as *const libc::c_char,
            (*entry).name,
            item,
        );
        (*item).type_0 = CMDQ_COMMAND;
        (*item).group = cmd_get_group(cmd);
        (*item).state = cmdq_link_state(state);
        (*item).cmdlist = cmdlist;
        (*item).cmd = cmd;
        (*cmdlist).references += 1;
        log_debug(
            b"%s: %s group %u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"cmdq_get_command\x00"))
                .as_ptr(),
            (*item).name,
            (*item).group,
        );
        if first.is_null() {
            first = item
        }
        if !last.is_null() {
            (*last).next = item
        }
        last = item;
        cmd = cmd_list_next(cmd)
    }
    if created != 0 {
        cmdq_free_state(state);
    }
    return first;
}
/* Fill in flag for a command. */
unsafe extern "C" fn cmdq_find_flag(
    mut item: *mut cmdq_item,
    mut fs: *mut cmd_find_state,
    mut flag: *const cmd_entry_flag,
) -> cmd_retval {
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    if (*flag).flag as libc::c_int == 0 as libc::c_int {
        cmd_find_from_client(fs, (*item).target_client, 0 as libc::c_int);
        return CMD_RETURN_NORMAL;
    }
    value = args_get(cmd_get_args((*item).cmd), (*flag).flag as u_char);
    if cmd_find_target(fs, item, value, (*flag).type_0, (*flag).flags) != 0 as libc::c_int {
        cmd_find_clear_state(fs, 0 as libc::c_int);
        return CMD_RETURN_ERROR;
    }
    return CMD_RETURN_NORMAL;
}
/* Add message with command. */
unsafe extern "C" fn cmdq_add_message(mut item: *mut cmdq_item) {
    let mut c: *mut client = (*item).client;
    let mut state: *mut cmdq_state = (*item).state;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = cmd_print((*item).cmd);
    if !c.is_null() {
        name = (*c).name;
        if !(*c).session.is_null() && (*state).event.key != 0xff000000000 as libc::c_ulonglong {
            key = key_string_lookup_key((*state).event.key, 0 as libc::c_int);
            server_add_message(
                b"%s key %s: %s\x00" as *const u8 as *const libc::c_char,
                name,
                key,
                tmp,
            );
        } else {
            server_add_message(
                b"%s command: %s\x00" as *const u8 as *const libc::c_char,
                name,
                tmp,
            );
        }
    } else {
        server_add_message(b"command: %s\x00" as *const u8 as *const libc::c_char, tmp);
    }
    free(tmp as *mut libc::c_void);
}
/* Fire command on command queue. */
unsafe extern "C" fn cmdq_fire_command(mut item: *mut cmdq_item) -> cmd_retval {
    let mut current_block: u64;
    let mut name: *const libc::c_char = cmdq_name((*item).client);
    let mut state: *mut cmdq_state = (*item).state;
    let mut cmd: *mut crate::cmd::cmd = (*item).cmd;
    let mut args: *mut args = cmd_get_args(cmd);
    let mut entry: *const cmd_entry = cmd_get_entry(cmd);
    let mut tc: *mut client = 0 as *mut client;
    let mut saved: *mut client = (*item).client;
    let mut retval: cmd_retval = CMD_RETURN_NORMAL;
    let mut fsp: *mut cmd_find_state = 0 as *mut cmd_find_state;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    let mut flags: libc::c_int = 0;
    let mut quiet: libc::c_int = 0 as libc::c_int;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if cfg_finished != 0 {
        cmdq_add_message(item);
    }
    if log_get_level() > 1 as libc::c_int {
        tmp = cmd_print(cmd);
        log_debug(
            b"%s %s: (%u) %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"cmdq_fire_command\x00"))
                .as_ptr(),
            name,
            (*item).group,
            tmp,
        );
        free(tmp as *mut libc::c_void);
    }
    flags = ((*state).flags & 0x2 as libc::c_int != 0) as libc::c_int;
    cmdq_guard(
        item,
        b"begin\x00" as *const u8 as *const libc::c_char,
        flags,
    );
    if (*item).client.is_null() {
        (*item).client = cmd_find_client(item, 0 as *const libc::c_char, 1 as libc::c_int)
    }
    if (*entry).flags & 0x20 as libc::c_int != 0 {
        quiet = 1 as libc::c_int
    }
    if (*entry).flags & 0x8 as libc::c_int != 0 {
        tc = cmd_find_client(item, args_get(args, 'c' as i32 as u_char), quiet);
        if tc.is_null() && quiet == 0 {
            retval = CMD_RETURN_ERROR;
            current_block = 17209596729593091530;
        } else {
            current_block = 15345278821338558188;
        }
    } else if (*entry).flags & 0x10 as libc::c_int != 0 {
        tc = cmd_find_client(item, args_get(args, 't' as i32 as u_char), quiet);
        if tc.is_null() && quiet == 0 {
            retval = CMD_RETURN_ERROR;
            current_block = 17209596729593091530;
        } else {
            current_block = 15345278821338558188;
        }
    } else {
        tc = cmd_find_client(item, 0 as *const libc::c_char, 1 as libc::c_int);
        current_block = 15345278821338558188;
    }
    match current_block {
        15345278821338558188 => {
            (*item).target_client = tc;
            retval = cmdq_find_flag(item, &mut (*item).source, &(*entry).source);
            if !(retval as libc::c_int == CMD_RETURN_ERROR as libc::c_int) {
                retval = cmdq_find_flag(item, &mut (*item).target, &(*entry).target);
                if !(retval as libc::c_int == CMD_RETURN_ERROR as libc::c_int) {
                    retval = (*entry).exec.expect("non-null function pointer")(cmd, item);
                    if !(retval as libc::c_int == CMD_RETURN_ERROR as libc::c_int) {
                        if (*entry).flags & 0x4 as libc::c_int != 0 {
                            if cmd_find_valid_state(&mut (*item).target) != 0 {
                                fsp = &mut (*item).target;
                                current_block = 7427571413727699167;
                            } else if cmd_find_valid_state(&mut (*(*item).state).current) != 0 {
                                fsp = &mut (*(*item).state).current;
                                current_block = 7427571413727699167;
                            } else if cmd_find_from_client(
                                &mut fs,
                                (*item).client,
                                0 as libc::c_int,
                            ) == 0 as libc::c_int
                            {
                                fsp = &mut fs;
                                current_block = 7427571413727699167;
                            } else {
                                current_block = 17209596729593091530;
                            }
                            match current_block {
                                17209596729593091530 => {}
                                _ => {
                                    cmdq_insert_hook(
                                        (*fsp).s,
                                        item,
                                        fsp,
                                        b"after-%s\x00" as *const u8 as *const libc::c_char,
                                        (*entry).name,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    (*item).client = saved;
    if retval as libc::c_int == CMD_RETURN_ERROR as libc::c_int {
        cmdq_guard(
            item,
            b"error\x00" as *const u8 as *const libc::c_char,
            flags,
        );
    } else {
        cmdq_guard(item, b"end\x00" as *const u8 as *const libc::c_char, flags);
    }
    return retval;
}
/* Get a callback for the command queue. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_callback1(
    mut name: *const libc::c_char,
    mut cb: cmdq_cb,
    mut data: *mut libc::c_void,
) -> *mut cmdq_item {
    let mut item: *mut cmdq_item = 0 as *mut cmdq_item;
    item = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cmdq_item>() as libc::c_ulong,
    ) as *mut cmdq_item;
    xasprintf(
        &mut (*item).name as *mut *mut libc::c_char,
        b"[%s/%p]\x00" as *const u8 as *const libc::c_char,
        name,
        item,
    );
    (*item).type_0 = CMDQ_CALLBACK;
    (*item).group = 0 as libc::c_int as u_int;
    (*item).state = cmdq_new_state(
        0 as *mut cmd_find_state,
        0 as *mut key_event,
        0 as libc::c_int,
    );
    (*item).cb = cb;
    (*item).data = data;
    return item;
}
/* Generic error callback. */
unsafe extern "C" fn cmdq_error_callback(
    mut item: *mut cmdq_item,
    mut data: *mut libc::c_void,
) -> cmd_retval {
    let mut error: *mut libc::c_char = data as *mut libc::c_char;
    cmdq_error(item, b"%s\x00" as *const u8 as *const libc::c_char, error);
    free(error as *mut libc::c_void);
    return CMD_RETURN_NORMAL;
}
/* Get an error callback for the command queue. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_get_error(mut error: *const libc::c_char) -> *mut cmdq_item {
    return cmdq_get_callback1(
        b"cmdq_error_callback\x00" as *const u8 as *const libc::c_char,
        Some(
            cmdq_error_callback
                as unsafe extern "C" fn(_: *mut cmdq_item, _: *mut libc::c_void) -> cmd_retval,
        ),
        xstrdup(error) as *mut libc::c_void,
    );
}
/* Fire callback on callback queue. */
unsafe extern "C" fn cmdq_fire_callback(mut item: *mut cmdq_item) -> cmd_retval {
    return (*item).cb.expect("non-null function pointer")(item, (*item).data);
}
/* Process next item on command queue. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_next(mut c: *mut client) -> u_int {
    let mut current_block: u64;
    let mut queue: *mut cmdq_list = cmdq_get(c);
    let mut name: *const libc::c_char = cmdq_name(c);
    let mut item: *mut cmdq_item = 0 as *mut cmdq_item;
    let mut retval: cmd_retval = CMD_RETURN_NORMAL;
    let mut items: u_int = 0 as libc::c_int as u_int;
    static mut number: u_int = 0;
    if (*queue).list.tqh_first.is_null() {
        log_debug(
            b"%s %s: empty\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"cmdq_next\x00")).as_ptr(),
            name,
        );
        return 0 as libc::c_int as u_int;
    }
    if (*(*queue).list.tqh_first).flags & 0x2 as libc::c_int != 0 {
        log_debug(
            b"%s %s: waiting\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"cmdq_next\x00")).as_ptr(),
            name,
        );
        return 0 as libc::c_int as u_int;
    }
    log_debug(
        b"%s %s: enter\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"cmdq_next\x00")).as_ptr(),
        name,
    );
    loop {
        (*queue).item = (*queue).list.tqh_first;
        item = (*queue).item;
        if item.is_null() {
            current_block = 17500079516916021833;
            break;
        }
        log_debug(
            b"%s %s: %s (%d), flags %x\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"cmdq_next\x00")).as_ptr(),
            name,
            (*item).name,
            (*item).type_0 as libc::c_uint,
            (*item).flags,
        );
        /*
         * Any item with the waiting flag set waits until an external
         * event clears the flag (for example, a job - look at
         * run-shell).
         */
        if (*item).flags & 0x2 as libc::c_int != 0 {
            current_block = 4630972204675230691;
            break;
        }
        /*
         * Items are only fired once, once the fired flag is set, a
         * waiting flag can only be cleared by an external event.
         */
        if !(*item).flags & 0x1 as libc::c_int != 0 {
            (*item).time = time(0 as *mut time_t);
            number = number.wrapping_add(1);
            (*item).number = number;
            match (*item).type_0 as libc::c_uint {
                0 => {
                    retval = cmdq_fire_command(item);
                    /*
                     * If a command returns an error, remove any
                     * subsequent commands in the same group.
                     */
                    if retval as libc::c_int == CMD_RETURN_ERROR as libc::c_int {
                        cmdq_remove_group(item);
                    }
                }
                1 => retval = cmdq_fire_callback(item),
                _ => retval = CMD_RETURN_ERROR,
            }
            (*item).flags |= 0x1 as libc::c_int;
            if retval as libc::c_int == CMD_RETURN_WAIT as libc::c_int {
                (*item).flags |= 0x2 as libc::c_int;
                current_block = 4630972204675230691;
                break;
            } else {
                items = items.wrapping_add(1)
            }
        }
        cmdq_remove(item);
    }
    match current_block {
        4630972204675230691 => {
            log_debug(
                b"%s %s: exit (wait)\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"cmdq_next\x00"))
                    .as_ptr(),
                name,
            );
            return items;
        }
        _ => {
            (*queue).item = 0 as *mut cmdq_item;
            log_debug(
                b"%s %s: exit (empty)\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"cmdq_next\x00"))
                    .as_ptr(),
                name,
            );
            return items;
        }
    };
}
/* Get running item if any. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_running(mut c: *mut client) -> *mut cmdq_item {
    let mut queue: *mut cmdq_list = cmdq_get(c);
    return (*queue).item;
}
/* Print a guard line. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_guard(
    mut item: *mut cmdq_item,
    mut guard: *const libc::c_char,
    mut flags: libc::c_int,
) {
    let mut c: *mut client = (*item).client;
    let mut t: libc::c_long = (*item).time;
    let mut number: u_int = (*item).number;
    if !c.is_null() && (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
        control_write(
            c,
            b"%%%s %ld %u %d\x00" as *const u8 as *const libc::c_char,
            guard,
            t,
            number,
            flags,
        );
    };
}
/* Show message from command. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_print(
    mut item: *mut cmdq_item,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut c: *mut client = (*item).client;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    let mut ap: ::std::ffi::VaListImpl;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    xvasprintf(&mut msg, fmt, ap.as_va_list());
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"cmdq_print\x00")).as_ptr(),
        msg,
    );
    if !c.is_null() {
        if (*c).session.is_null() || (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
            if !(*c).flags & 0x10000 as libc::c_int as libc::c_ulong != 0 {
                tmp = msg;
                msg = utf8_sanitize(tmp);
                free(tmp as *mut libc::c_void);
            }
            if (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
                control_write(c, b"%s\x00" as *const u8 as *const libc::c_char, msg);
            } else {
                file_print(c, b"%s\n\x00" as *const u8 as *const libc::c_char, msg);
            }
        } else {
            wp = server_client_get_pane(c);
            wme = (*wp).modes.tqh_first;
            if wme.is_null() || (*wme).mode != &window_view_mode as *const window_mode {
                window_pane_set_mode(
                    wp,
                    0 as *mut window_pane,
                    &window_view_mode,
                    0 as *mut cmd_find_state,
                    0 as *mut args,
                );
            }
            window_copy_add(wp, b"%s\x00" as *const u8 as *const libc::c_char, msg);
        }
    }
    free(msg as *mut libc::c_void);
}
/* Show error from command. */
#[no_mangle]
pub unsafe extern "C" fn cmdq_error(
    mut item: *mut cmdq_item,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut c: *mut client = (*item).client;
    let mut cmd: *mut crate::cmd::cmd = (*item).cmd;
    let mut ap: ::std::ffi::VaListImpl;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file: *const libc::c_char = 0 as *const libc::c_char;
    let mut line: u_int = 0;
    ap = args.clone();
    xvasprintf(&mut msg, fmt, ap.as_va_list());
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"cmdq_error\x00")).as_ptr(),
        msg,
    );
    if c.is_null() {
        cmd_get_source(cmd, &mut file, &mut line);
        cfg_add_cause(
            b"%s:%u: %s\x00" as *const u8 as *const libc::c_char,
            file,
            line,
            msg,
        );
    } else if (*c).session.is_null() || (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
        server_add_message(
            b"%s message: %s\x00" as *const u8 as *const libc::c_char,
            (*c).name,
            msg,
        );
        if !(*c).flags & 0x10000 as libc::c_int as libc::c_ulong != 0 {
            tmp = msg;
            msg = utf8_sanitize(tmp);
            free(tmp as *mut libc::c_void);
        }
        if (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
            control_write(c, b"%s\x00" as *const u8 as *const libc::c_char, msg);
        } else {
            file_error(c, b"%s\n\x00" as *const u8 as *const libc::c_char, msg);
        }
        (*c).retval = 1 as libc::c_int
    } else {
        *msg = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<u_char>() as libc::c_ulong > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *msg as u_char as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    }
                } else {
                    __res = toupper(*msg as u_char as libc::c_int)
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*msg as u_char as libc::c_int as isize)
            }
            __res
        }) as libc::c_char;
        status_message_set(
            c,
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            msg,
        );
    }
    free(msg as *mut libc::c_void);
}
