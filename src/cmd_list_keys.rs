use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_s_options: *mut crate::options::options;
    #[no_mangle]
    fn format_create(
        _: *mut client,
        _: *mut crate::cmd_queue::cmdq_item,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut crate::format::format_tree;
    #[no_mangle]
    fn format_free(_: *mut crate::format::format_tree);
    #[no_mangle]
    fn format_add(
        _: *mut crate::format::format_tree,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn format_expand(
        _: *mut crate::format::format_tree,
        _: *const libc::c_char,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn format_defaults(
        _: *mut crate::format::format_tree,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    );
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn args_escape(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn cmd_get_entry(_: *mut crate::cmd::cmd) -> *const cmd_entry;
    #[no_mangle]
    fn cmd_get_args(_: *mut crate::cmd::cmd) -> *mut args;
    #[no_mangle]
    fn cmd_list_print(_: *mut cmd_list, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn cmdq_get_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
    #[no_mangle]
    fn cmdq_get_target_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
    #[no_mangle]
    fn cmdq_print(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn cmdq_error(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn key_bindings_get_table(_: *const libc::c_char, _: libc::c_int) -> *mut key_table;
    #[no_mangle]
    fn key_bindings_first_table() -> *mut key_table;
    #[no_mangle]
    fn key_bindings_next_table(_: *mut key_table) -> *mut key_table;
    #[no_mangle]
    fn key_bindings_first(_: *mut key_table) -> *mut key_binding;
    #[no_mangle]
    fn key_bindings_next(_: *mut key_table, _: *mut key_binding) -> *mut key_binding;
    #[no_mangle]
    fn key_string_lookup_string(_: *const libc::c_char) -> key_code;
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn status_message_set(
        _: *mut client,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn utf8_cstrwidth(_: *const libc::c_char) -> u_int;
    #[no_mangle]
    fn utf8_padcstr(_: *const libc::c_char, _: u_int) -> *mut libc::c_char;
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
    pub rbh_root: *mut crate::arguments::args_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct client {
    pub name: *const libc::c_char,
    pub peer: *mut crate::proc::tmuxpeer,
    pub queue: *mut crate::cmd_queue::cmdq_list,
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
pub type cmd_find_type = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type cmd_retval = libc::c_int;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;

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
    pub exec: Option<
        unsafe extern "C" fn(
            _: *mut crate::cmd::cmd,
            _: *mut crate::cmd_queue::cmdq_item,
        ) -> cmd_retval,
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[no_mangle]
pub static mut cmd_list_keys_entry: cmd_entry = {
    {
        let mut init = cmd_entry {
            name: b"list-keys\x00" as *const u8 as *const libc::c_char,
            alias: b"lsk\x00" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed_33 {
                    template: b"1aNP:T:\x00" as *const u8 as *const libc::c_char,
                    lower: 0 as libc::c_int,
                    upper: 1 as libc::c_int,
                };
                init
            },
            usage: b"[-1aN] [-P prefix-string] [-T key-table] [key]\x00" as *const u8
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
            flags: 0x1 as libc::c_int | 0x4 as libc::c_int,
            exec: Some(
                cmd_list_keys_exec
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd::cmd,
                        _: *mut crate::cmd_queue::cmdq_item,
                    ) -> cmd_retval,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut cmd_list_commands_entry: cmd_entry = {
    {
        let mut init = cmd_entry {
            name: b"list-commands\x00" as *const u8 as *const libc::c_char,
            alias: b"lscm\x00" as *const u8 as *const libc::c_char,
            args: {
                let mut init = C2RustUnnamed_33 {
                    template: b"F:\x00" as *const u8 as *const libc::c_char,
                    lower: 0 as libc::c_int,
                    upper: 1 as libc::c_int,
                };
                init
            },
            usage: b"[-F format] [command]\x00" as *const u8 as *const libc::c_char,
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
            flags: 0x1 as libc::c_int | 0x4 as libc::c_int,
            exec: Some(
                cmd_list_keys_exec
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd::cmd,
                        _: *mut crate::cmd_queue::cmdq_item,
                    ) -> cmd_retval,
            ),
        };
        init
    }
};
unsafe extern "C" fn cmd_list_keys_get_width(
    mut tablename: *const libc::c_char,
    mut only: key_code,
) -> u_int {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut width: u_int = 0;
    let mut keywidth: u_int = 0 as libc::c_int as u_int;
    table = key_bindings_get_table(tablename, 0 as libc::c_int);
    if table.is_null() {
        return 0 as libc::c_int as u_int;
    }
    bd = key_bindings_first(table);
    while !bd.is_null() {
        if only != 0xfe000000000 as libc::c_ulonglong && (*bd).key != only
            || (*bd).key & 0xfffffffffff as libc::c_ulonglong
                >= KEYC_MOUSE as libc::c_ulong as libc::c_ulonglong
                && ((*bd).key & 0xfffffffffff as libc::c_ulonglong)
                    < KEYC_BSPACE as libc::c_ulong as libc::c_ulonglong
            || (*bd).note.is_null()
            || *(*bd).note as libc::c_int == '\u{0}' as i32
        {
            bd = key_bindings_next(table, bd)
        } else {
            width = utf8_cstrwidth(key_string_lookup_key((*bd).key, 0 as libc::c_int));
            if width > keywidth {
                keywidth = width
            }
            bd = key_bindings_next(table, bd)
        }
    }
    return keywidth;
}
unsafe extern "C" fn cmd_list_keys_print_notes(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut args: *mut args,
    mut tablename: *const libc::c_char,
    mut keywidth: u_int,
    mut only: key_code,
    mut prefix: *const libc::c_char,
) -> libc::c_int {
    let mut tc: *mut client = cmdq_get_target_client(item);
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut note: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found: libc::c_int = 0 as libc::c_int;
    table = key_bindings_get_table(tablename, 0 as libc::c_int);
    if table.is_null() {
        return 0 as libc::c_int;
    }
    bd = key_bindings_first(table);
    while !bd.is_null() {
        if only != 0xfe000000000 as libc::c_ulonglong && (*bd).key != only
            || (*bd).key & 0xfffffffffff as libc::c_ulonglong
                >= KEYC_MOUSE as libc::c_ulong as libc::c_ulonglong
                && ((*bd).key & 0xfffffffffff as libc::c_ulonglong)
                    < KEYC_BSPACE as libc::c_ulong as libc::c_ulonglong
            || ((*bd).note.is_null() || *(*bd).note as libc::c_int == '\u{0}' as i32)
                && args_has(args, 'a' as i32 as u_char) == 0
        {
            bd = key_bindings_next(table, bd)
        } else {
            found = 1 as libc::c_int;
            key = key_string_lookup_key((*bd).key, 0 as libc::c_int);
            if (*bd).note.is_null() || *(*bd).note as libc::c_int == '\u{0}' as i32 {
                note = cmd_list_print((*bd).cmdlist, 1 as libc::c_int)
            } else {
                note = xstrdup((*bd).note)
            }
            tmp = utf8_padcstr(key, keywidth.wrapping_add(1 as libc::c_int as libc::c_uint));
            if args_has(args, '1' as i32 as u_char) != 0 && !tc.is_null() {
                status_message_set(
                    tc,
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                    prefix,
                    tmp,
                    note,
                );
            } else {
                cmdq_print(
                    item,
                    b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                    prefix,
                    tmp,
                    note,
                );
            }
            free(tmp as *mut libc::c_void);
            free(note as *mut libc::c_void);
            if args_has(args, '1' as i32 as u_char) != 0 {
                break;
            }
            bd = key_bindings_next(table, bd)
        }
    }
    return found;
}
unsafe extern "C" fn cmd_list_keys_get_prefix(
    mut args: *mut args,
    mut prefix: *mut key_code,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    *prefix = options_get_number(
        global_s_options,
        b"prefix\x00" as *const u8 as *const libc::c_char,
    ) as key_code;
    if args_has(args, 'P' as i32 as u_char) == 0 {
        if *prefix != 0xff000000000 as libc::c_ulonglong {
            xasprintf(
                &mut s as *mut *mut libc::c_char,
                b"%s \x00" as *const u8 as *const libc::c_char,
                key_string_lookup_key(*prefix, 0 as libc::c_int),
            );
        } else {
            s = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
        }
    } else {
        s = xstrdup(args_get(args, 'P' as i32 as u_char))
    }
    return s;
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
 * List key bindings.
 */
unsafe extern "C" fn cmd_list_keys_exec(
    mut self_0: *mut crate::cmd::cmd,
    mut item: *mut crate::cmd_queue::cmdq_item,
) -> cmd_retval {
    let mut args: *mut args = cmd_get_args(self_0);
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut tablename: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut empty: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: key_code = 0;
    let mut only: key_code = 0xfe000000000 as libc::c_ulonglong;
    let mut repeat: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut tablewidth: libc::c_int = 0;
    let mut keywidth: libc::c_int = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut tmpsize: size_t = 0;
    let mut tmpused: size_t = 0;
    let mut cplen: size_t = 0;
    if cmd_get_entry(self_0) == &cmd_list_commands_entry as *const cmd_entry {
        return cmd_list_keys_commands(self_0, item);
    }
    if (*args).argc != 0 as libc::c_int {
        only = key_string_lookup_string(*(*args).argv.offset(0 as libc::c_int as isize));
        if only == 0xfe000000000 as libc::c_ulonglong {
            cmdq_error(
                item,
                b"invalid key: %s\x00" as *const u8 as *const libc::c_char,
                *(*args).argv.offset(0 as libc::c_int as isize),
            );
            return CMD_RETURN_ERROR;
        }
        only &= 0xfffffffffff as libc::c_ulonglong
    }
    tablename = args_get(args, 'T' as i32 as u_char);
    if !tablename.is_null() && key_bindings_get_table(tablename, 0 as libc::c_int).is_null() {
        cmdq_error(
            item,
            b"table %s doesn\'t exist\x00" as *const u8 as *const libc::c_char,
            tablename,
        );
        return CMD_RETURN_ERROR;
    }
    if args_has(args, 'N' as i32 as u_char) != 0 {
        if tablename.is_null() {
            start = cmd_list_keys_get_prefix(args, &mut prefix);
            keywidth =
                cmd_list_keys_get_width(b"root\x00" as *const u8 as *const libc::c_char, only)
                    as libc::c_int;
            if prefix != 0xff000000000 as libc::c_ulonglong {
                width = cmd_list_keys_get_width(
                    b"prefix\x00" as *const u8 as *const libc::c_char,
                    only,
                ) as libc::c_int;
                if width == 0 as libc::c_int {
                    prefix = 0xff000000000 as libc::c_ulonglong
                } else if width > keywidth {
                    keywidth = width
                }
            }
            empty = utf8_padcstr(
                b"\x00" as *const u8 as *const libc::c_char,
                utf8_cstrwidth(start),
            );
            found = cmd_list_keys_print_notes(
                item,
                args,
                b"root\x00" as *const u8 as *const libc::c_char,
                keywidth as u_int,
                only,
                empty,
            );
            if prefix != 0xff000000000 as libc::c_ulonglong {
                if cmd_list_keys_print_notes(
                    item,
                    args,
                    b"prefix\x00" as *const u8 as *const libc::c_char,
                    keywidth as u_int,
                    only,
                    start,
                ) != 0
                {
                    found = 1 as libc::c_int
                }
            }
            free(empty as *mut libc::c_void);
        } else {
            if args_has(args, 'P' as i32 as u_char) != 0 {
                start = xstrdup(args_get(args, 'P' as i32 as u_char))
            } else {
                start = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
            }
            keywidth = cmd_list_keys_get_width(tablename, only) as libc::c_int;
            found = cmd_list_keys_print_notes(item, args, tablename, keywidth as u_int, only, start)
        }
        free(start as *mut libc::c_void);
    } else {
        repeat = 0 as libc::c_int;
        keywidth = 0 as libc::c_int;
        tablewidth = keywidth;
        table = key_bindings_first_table();
        while !table.is_null() {
            if !tablename.is_null() && strcmp((*table).name, tablename) != 0 as libc::c_int {
                table = key_bindings_next_table(table)
            } else {
                bd = key_bindings_first(table);
                while !bd.is_null() {
                    if only != 0xfe000000000 as libc::c_ulonglong && (*bd).key != only {
                        bd = key_bindings_next(table, bd)
                    } else {
                        key = args_escape(key_string_lookup_key((*bd).key, 0 as libc::c_int));
                        if (*bd).flags & 0x1 as libc::c_int != 0 {
                            repeat = 1 as libc::c_int
                        }
                        width = utf8_cstrwidth((*table).name) as libc::c_int;
                        if width > tablewidth {
                            tablewidth = width
                        }
                        width = utf8_cstrwidth(key) as libc::c_int;
                        if width > keywidth {
                            keywidth = width
                        }
                        free(key as *mut libc::c_void);
                        bd = key_bindings_next(table, bd)
                    }
                }
                table = key_bindings_next_table(table)
            }
        }
        tmpsize = 256 as libc::c_int as size_t;
        tmp = xmalloc(tmpsize) as *mut libc::c_char;
        table = key_bindings_first_table();
        while !table.is_null() {
            if !tablename.is_null() && strcmp((*table).name, tablename) != 0 as libc::c_int {
                table = key_bindings_next_table(table)
            } else {
                bd = key_bindings_first(table);
                while !bd.is_null() {
                    if only != 0xfe000000000 as libc::c_ulonglong && (*bd).key != only {
                        bd = key_bindings_next(table, bd)
                    } else {
                        found = 1 as libc::c_int;
                        key = args_escape(key_string_lookup_key((*bd).key, 0 as libc::c_int));
                        if repeat == 0 {
                            r = b"\x00" as *const u8 as *const libc::c_char
                        } else if (*bd).flags & 0x1 as libc::c_int != 0 {
                            r = b"-r \x00" as *const u8 as *const libc::c_char
                        } else {
                            r = b"   \x00" as *const u8 as *const libc::c_char
                        }
                        tmpused = xsnprintf(
                            tmp,
                            tmpsize,
                            b"%s-T \x00" as *const u8 as *const libc::c_char,
                            r,
                        ) as size_t;
                        cp = utf8_padcstr((*table).name, tablewidth as u_int);
                        cplen = strlen(cp).wrapping_add(1 as libc::c_int as libc::c_ulong);
                        while tmpused
                            .wrapping_add(cplen)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            >= tmpsize
                        {
                            tmpsize = (tmpsize as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                as size_t as size_t;
                            tmp = xrealloc(tmp as *mut libc::c_void, tmpsize) as *mut libc::c_char
                        }
                        strlcat(tmp, cp, tmpsize);
                        tmpused =
                            strlcat(tmp, b" \x00" as *const u8 as *const libc::c_char, tmpsize);
                        free(cp as *mut libc::c_void);
                        cp = utf8_padcstr(key, keywidth as u_int);
                        cplen = strlen(cp).wrapping_add(1 as libc::c_int as libc::c_ulong);
                        while tmpused
                            .wrapping_add(cplen)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            >= tmpsize
                        {
                            tmpsize = (tmpsize as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                as size_t as size_t;
                            tmp = xrealloc(tmp as *mut libc::c_void, tmpsize) as *mut libc::c_char
                        }
                        strlcat(tmp, cp, tmpsize);
                        tmpused =
                            strlcat(tmp, b" \x00" as *const u8 as *const libc::c_char, tmpsize);
                        free(cp as *mut libc::c_void);
                        cp = cmd_list_print((*bd).cmdlist, 1 as libc::c_int);
                        cplen = strlen(cp);
                        while tmpused
                            .wrapping_add(cplen)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            >= tmpsize
                        {
                            tmpsize = (tmpsize as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                as size_t as size_t;
                            tmp = xrealloc(tmp as *mut libc::c_void, tmpsize) as *mut libc::c_char
                        }
                        strlcat(tmp, cp, tmpsize);
                        free(cp as *mut libc::c_void);
                        cmdq_print(
                            item,
                            b"bind-key %s\x00" as *const u8 as *const libc::c_char,
                            tmp,
                        );
                        free(key as *mut libc::c_void);
                        bd = key_bindings_next(table, bd)
                    }
                }
                table = key_bindings_next_table(table)
            }
        }
        free(tmp as *mut libc::c_void);
    }
    if only != 0xfe000000000 as libc::c_ulonglong && found == 0 {
        cmdq_error(
            item,
            b"unknown key: %s\x00" as *const u8 as *const libc::c_char,
            *(*args).argv.offset(0 as libc::c_int as isize),
        );
        return CMD_RETURN_ERROR;
    }
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn cmd_list_keys_commands(
    mut self_0: *mut crate::cmd::cmd,
    mut item: *mut crate::cmd_queue::cmdq_item,
) -> cmd_retval {
    let mut args: *mut args = cmd_get_args(self_0);
    let mut entryp: *mut *const cmd_entry = 0 as *mut *const cmd_entry;
    let mut entry: *const cmd_entry = 0 as *const cmd_entry;
    let mut ft: *mut crate::format::format_tree = 0 as *mut crate::format::format_tree;
    let mut template: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut command: *const libc::c_char = 0 as *const libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*args).argc != 0 as libc::c_int {
        command = *(*args).argv.offset(0 as libc::c_int as isize)
    }
    template = args_get(args, 'F' as i32 as u_char);
    if template.is_null() {
        template =
            b"#{command_list_name}#{?command_list_alias, (#{command_list_alias}),} #{command_list_usage}\x00"
                as *const u8 as *const libc::c_char
    }
    ft = format_create(
        cmdq_get_client(item),
        item,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    format_defaults(
        ft,
        0 as *mut client,
        0 as *mut session,
        0 as *mut winlink,
        0 as *mut window_pane,
    );
    entryp = cmd_table.as_mut_ptr();
    while !(*entryp).is_null() {
        entry = *entryp;
        if !(!command.is_null()
            && (strcmp((*entry).name, command) != 0 as libc::c_int
                && ((*entry).alias.is_null()
                    || strcmp((*entry).alias, command) != 0 as libc::c_int)))
        {
            format_add(
                ft,
                b"command_list_name\x00" as *const u8 as *const libc::c_char,
                b"%s\x00" as *const u8 as *const libc::c_char,
                (*entry).name,
            );
            if !(*entry).alias.is_null() {
                s = (*entry).alias
            } else {
                s = b"\x00" as *const u8 as *const libc::c_char
            }
            format_add(
                ft,
                b"command_list_alias\x00" as *const u8 as *const libc::c_char,
                b"%s\x00" as *const u8 as *const libc::c_char,
                s,
            );
            if !(*entry).usage.is_null() {
                s = (*entry).usage
            } else {
                s = b"\x00" as *const u8 as *const libc::c_char
            }
            format_add(
                ft,
                b"command_list_usage\x00" as *const u8 as *const libc::c_char,
                b"%s\x00" as *const u8 as *const libc::c_char,
                s,
            );
            line = format_expand(ft, template);
            if *line as libc::c_int != '\u{0}' as i32 {
                cmdq_print(item, b"%s\x00" as *const u8 as *const libc::c_char, line);
            }
            free(line as *mut libc::c_void);
        }
        entryp = entryp.offset(1)
    }
    format_free(ft);
    return CMD_RETURN_NORMAL;
}
