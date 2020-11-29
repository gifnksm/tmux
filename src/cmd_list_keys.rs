use crate::key_code::code as key_code_code;
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
    pub exit_msgtype: crate::msg::Msgtype,
    pub exit_session: *mut libc::c_char,
    pub exit_message: *mut libc::c_char,
    pub keytable: *mut key_table,
    pub redraw_panes: uint64_t,
    pub message_ignore_styles: libc::c_int,
    pub message_string: *mut libc::c_char,
    pub message_timer: event,
    pub prompt_string: *mut libc::c_char,
    pub prompt_buffer: *mut crate::utf8::Utf8Data,
    pub prompt_index: size_t,
    pub prompt_inputcb: prompt_input_cb,
    pub prompt_freecb: prompt_free_cb,
    pub prompt_data: *mut libc::c_void,
    pub prompt_hindex: u_int,
    pub prompt_mode: C2RustUnnamed_25,
    pub prompt_saved: *mut crate::utf8::Utf8Data,
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
    pub grid: *mut crate::grid::Grid,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub rupper: u_int,
    pub rlower: u_int,
    pub mode: libc::c_int,
    pub saved_cx: u_int,
    pub saved_cy: u_int,
    pub saved_grid: *mut crate::grid::Grid,
    pub saved_cell: crate::grid::Cell,
    pub saved_flags: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: *mut crate::screen::screen_sel,
    pub write_list: *mut crate::screen_write::screen_write_collect_line,
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
    pub cached_gc: crate::grid::Cell,
    pub cached_active_gc: crate::grid::Cell,
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
    pub border_gc: crate::grid::Cell,
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
    pub style: crate::grid::Cell,
    pub entries: [status_line_entry; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_line_entry {
    pub expanded: *mut libc::c_char,
    pub ranges: crate::style::Ranges,
}

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
    pub cell: crate::grid::Cell,
    pub last_cell: crate::grid::Cell,
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
                    lower: 0i32,
                    upper: 1i32,
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
            flags: 0x1i32 | 0x4i32,
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
                    lower: 0i32,
                    upper: 1i32,
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
            flags: 0x1i32 | 0x4i32,
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
    let mut keywidth: u_int = 0u32;
    table = key_bindings_get_table(tablename, 0i32);
    if table.is_null() {
        return 0u32;
    }
    bd = key_bindings_first(table);
    while !bd.is_null() {
        if only != 0xfe000000000u64 && (*bd).key != only
            || (*bd).key & 0xfffffffffffu64 >= key_code_code::MOUSE
                && ((*bd).key & 0xfffffffffffu64) < key_code_code::BSPACE
            || (*bd).note.is_null()
            || *(*bd).note as libc::c_int == '\u{0}' as i32
        {
            bd = key_bindings_next(table, bd)
        } else {
            width = utf8_cstrwidth(key_string_lookup_key((*bd).key, 0i32));
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
    let mut found: libc::c_int = 0i32;
    table = key_bindings_get_table(tablename, 0i32);
    if table.is_null() {
        return 0i32;
    }
    bd = key_bindings_first(table);
    while !bd.is_null() {
        if only != 0xfe000000000u64 && (*bd).key != only
            || (*bd).key & 0xfffffffffffu64 >= key_code_code::MOUSE
                && ((*bd).key & 0xfffffffffffu64) < key_code_code::BSPACE
            || ((*bd).note.is_null() || *(*bd).note as libc::c_int == '\u{0}' as i32)
                && args_has(args, 'a' as u_char) == 0
        {
            bd = key_bindings_next(table, bd)
        } else {
            found = 1i32;
            key = key_string_lookup_key((*bd).key, 0i32);
            if (*bd).note.is_null() || *(*bd).note as libc::c_int == '\u{0}' as i32 {
                note = cmd_list_print((*bd).cmdlist, 1i32)
            } else {
                note = xstrdup((*bd).note)
            }
            tmp = utf8_padcstr(key, keywidth.wrapping_add(1u32));
            if args_has(args, '1' as u_char) != 0 && !tc.is_null() {
                status_message_set(
                    tc,
                    -(1i32),
                    1i32,
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
            if args_has(args, '1' as u_char) != 0 {
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
    if args_has(args, 'P' as u_char) == 0 {
        if *prefix != 0xff000000000u64 {
            xasprintf(
                &mut s as *mut *mut libc::c_char,
                b"%s \x00" as *const u8 as *const libc::c_char,
                key_string_lookup_key(*prefix, 0i32),
            );
        } else {
            s = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
        }
    } else {
        s = xstrdup(args_get(args, 'P' as u_char))
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
    let mut only: key_code = 0xfe000000000u64;
    let mut repeat: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut tablewidth: libc::c_int = 0;
    let mut keywidth: libc::c_int = 0;
    let mut found: libc::c_int = 0i32;
    let mut tmpsize: size_t = 0;
    let mut tmpused: size_t = 0;
    let mut cplen: size_t = 0;
    if cmd_get_entry(self_0) == &cmd_list_commands_entry as *const cmd_entry {
        return cmd_list_keys_commands(self_0, item);
    }
    if (*args).argc != 0i32 {
        only = key_string_lookup_string(*(*args).argv.offset(0isize));
        if only == 0xfe000000000u64 {
            cmdq_error(
                item,
                b"invalid key: %s\x00" as *const u8 as *const libc::c_char,
                *(*args).argv.offset(0isize),
            );
            return CMD_RETURN_ERROR;
        }
        only &= 0xfffffffffffu64
    }
    tablename = args_get(args, 'T' as u_char);
    if !tablename.is_null() && key_bindings_get_table(tablename, 0i32).is_null() {
        cmdq_error(
            item,
            b"table %s doesn\'t exist\x00" as *const u8 as *const libc::c_char,
            tablename,
        );
        return CMD_RETURN_ERROR;
    }
    if args_has(args, 'N' as u_char) != 0 {
        if tablename.is_null() {
            start = cmd_list_keys_get_prefix(args, &mut prefix);
            keywidth =
                cmd_list_keys_get_width(b"root\x00" as *const u8 as *const libc::c_char, only)
                    as libc::c_int;
            if prefix != 0xff000000000u64 {
                width = cmd_list_keys_get_width(
                    b"prefix\x00" as *const u8 as *const libc::c_char,
                    only,
                ) as libc::c_int;
                if width == 0i32 {
                    prefix = 0xff000000000u64
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
            if prefix != 0xff000000000u64 {
                if cmd_list_keys_print_notes(
                    item,
                    args,
                    b"prefix\x00" as *const u8 as *const libc::c_char,
                    keywidth as u_int,
                    only,
                    start,
                ) != 0
                {
                    found = 1i32
                }
            }
            free(empty as *mut libc::c_void);
        } else {
            if args_has(args, 'P' as u_char) != 0 {
                start = xstrdup(args_get(args, 'P' as u_char))
            } else {
                start = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
            }
            keywidth = cmd_list_keys_get_width(tablename, only) as libc::c_int;
            found = cmd_list_keys_print_notes(item, args, tablename, keywidth as u_int, only, start)
        }
        free(start as *mut libc::c_void);
    } else {
        repeat = 0i32;
        keywidth = 0i32;
        tablewidth = keywidth;
        table = key_bindings_first_table();
        while !table.is_null() {
            if !tablename.is_null() && strcmp((*table).name, tablename) != 0i32 {
                table = key_bindings_next_table(table)
            } else {
                bd = key_bindings_first(table);
                while !bd.is_null() {
                    if only != 0xfe000000000u64 && (*bd).key != only {
                        bd = key_bindings_next(table, bd)
                    } else {
                        key = args_escape(key_string_lookup_key((*bd).key, 0i32));
                        if (*bd).flags & 0x1i32 != 0 {
                            repeat = 1i32
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
        tmpsize = 256u64;
        tmp = xmalloc(tmpsize) as *mut libc::c_char;
        table = key_bindings_first_table();
        while !table.is_null() {
            if !tablename.is_null() && strcmp((*table).name, tablename) != 0i32 {
                table = key_bindings_next_table(table)
            } else {
                bd = key_bindings_first(table);
                while !bd.is_null() {
                    if only != 0xfe000000000u64 && (*bd).key != only {
                        bd = key_bindings_next(table, bd)
                    } else {
                        found = 1i32;
                        key = args_escape(key_string_lookup_key((*bd).key, 0i32));
                        if repeat == 0 {
                            r = b"\x00" as *const u8 as *const libc::c_char
                        } else if (*bd).flags & 0x1i32 != 0 {
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
                        cplen = strlen(cp).wrapping_add(1u64);
                        while tmpused.wrapping_add(cplen).wrapping_add(1u64) >= tmpsize {
                            tmpsize = (tmpsize).wrapping_mul(2u64);
                            tmp = xrealloc(tmp as *mut libc::c_void, tmpsize) as *mut libc::c_char
                        }
                        strlcat(tmp, cp, tmpsize);
                        tmpused =
                            strlcat(tmp, b" \x00" as *const u8 as *const libc::c_char, tmpsize);
                        free(cp as *mut libc::c_void);
                        cp = utf8_padcstr(key, keywidth as u_int);
                        cplen = strlen(cp).wrapping_add(1u64);
                        while tmpused.wrapping_add(cplen).wrapping_add(1u64) >= tmpsize {
                            tmpsize = (tmpsize).wrapping_mul(2u64);
                            tmp = xrealloc(tmp as *mut libc::c_void, tmpsize) as *mut libc::c_char
                        }
                        strlcat(tmp, cp, tmpsize);
                        tmpused =
                            strlcat(tmp, b" \x00" as *const u8 as *const libc::c_char, tmpsize);
                        free(cp as *mut libc::c_void);
                        cp = cmd_list_print((*bd).cmdlist, 1i32);
                        cplen = strlen(cp);
                        while tmpused.wrapping_add(cplen).wrapping_add(1u64) >= tmpsize {
                            tmpsize = (tmpsize).wrapping_mul(2u64);
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
    if only != 0xfe000000000u64 && found == 0 {
        cmdq_error(
            item,
            b"unknown key: %s\x00" as *const u8 as *const libc::c_char,
            *(*args).argv.offset(0isize),
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
    if (*args).argc != 0i32 {
        command = *(*args).argv.offset(0isize)
    }
    template = args_get(args, 'F' as u_char);
    if template.is_null() {
        template =
            b"#{command_list_name}#{?command_list_alias, (#{command_list_alias}),} #{command_list_usage}\x00"
                as *const u8 as *const libc::c_char
    }
    ft = format_create(cmdq_get_client(item), item, 0i32, 0i32);
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
            && (strcmp((*entry).name, command) != 0i32
                && ((*entry).alias.is_null() || strcmp((*entry).alias, command) != 0i32)))
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
