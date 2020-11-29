use crate::{grid::Cell as GridCell, key_code::code as key_code_code, utf8::Utf8Data};
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn format_draw(
        _: *mut screen_write_ctx,
        _: *const crate::grid::Cell,
        _: u_int,
        _: *const libc::c_char,
        _: *mut style_ranges,
    );
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_mouse_at(
        _: *mut window_pane,
        _: *mut mouse_event,
        _: *mut u_int,
        _: *mut u_int,
        _: libc::c_int,
    ) -> libc::c_int;
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
        _: *mut crate::cmd_queue::cmdq_state,
        _: *mut *mut libc::c_char,
    ) -> cmd_parse_status;
    #[no_mangle]
    fn cmdq_new_state(
        _: *mut cmd_find_state,
        _: *mut key_event,
        _: libc::c_int,
    ) -> *mut crate::cmd_queue::cmdq_state;
    #[no_mangle]
    fn cmdq_free_state(_: *mut crate::cmd_queue::cmdq_state);
    #[no_mangle]
    fn server_redraw_window(_: *mut window);
    #[no_mangle]
    fn server_unzoom_window(_: *mut window);
    #[no_mangle]
    fn status_message_set(
        _: *mut client,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn status_prompt_set(
        _: *mut client,
        _: *mut cmd_find_state,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: prompt_input_cb,
        _: prompt_free_cb,
        _: *mut libc::c_void,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut screen);
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_puts(
        _: *mut screen_write_ctx,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn screen_write_nputs(
        _: *mut screen_write_ctx,
        _: ssize_t,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn screen_write_box(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_clearendofline(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_cursormove(
        _: *mut screen_write_ctx,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_write_clearscreen(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_free(_: *mut screen);
    #[no_mangle]
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int);
    #[no_mangle]
    fn window_zoom(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn menu_create(_: *const libc::c_char) -> *mut menu;
    #[no_mangle]
    fn menu_add_items(
        _: *mut menu,
        _: *const menu_item,
        _: *mut crate::cmd_queue::cmdq_item,
        _: *mut client,
        _: *mut cmd_find_state,
    );
    #[no_mangle]
    fn menu_display(
        _: *mut menu,
        _: libc::c_int,
        _: *mut crate::cmd_queue::cmdq_item,
        _: u_int,
        _: u_int,
        _: *mut client,
        _: *mut cmd_find_state,
        _: menu_choice_cb,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    fn menu_free(_: *mut menu);
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn utf8_cstrwidth(_: *const libc::c_char) -> u_int;
    #[no_mangle]
    fn style_apply(
        _: *mut crate::grid::Cell,
        _: *mut crate::options::options,
        _: *const libc::c_char,
        _: *mut crate::format::format_tree,
    );
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
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
    pub saved_cell: crate::grid::Cell,
    pub saved_flags: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: *mut crate::screen::screen_sel,
    pub write_list: *mut crate::screen_write::screen_write_collect_line,
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
    pub celldata: *mut crate::grid::CellEntry,
    pub extdsize: u_int,
    pub extddata: *mut crate::grid::ExtdEntry,
    pub flags: libc::c_int,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_tree_data {
    pub dead: libc::c_int,
    pub references: u_int,
    pub zoomed: libc::c_int,
    pub wp: *mut window_pane,
    pub modedata: *mut libc::c_void,
    pub menu: *const menu_item,
    pub sort_list: *mut *const libc::c_char,
    pub sort_size: u_int,
    pub sort_crit: mode_tree_sort_criteria,
    pub buildcb: mode_tree_build_cb,
    pub drawcb: mode_tree_draw_cb,
    pub searchcb: mode_tree_search_cb,
    pub menucb: mode_tree_menu_cb,
    pub heightcb: mode_tree_height_cb,
    pub children: mode_tree_list,
    pub saved: mode_tree_list,
    pub line_list: *mut mode_tree_line,
    pub line_size: u_int,
    pub depth: u_int,
    pub width: u_int,
    pub height: u_int,
    pub offset: u_int,
    pub current: u_int,
    pub screen: screen,
    pub preview: libc::c_int,
    pub search: *mut libc::c_char,
    pub filter: *mut libc::c_char,
    pub no_matches: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_tree_line {
    pub item: *mut mode_tree_item,
    pub depth: u_int,
    pub last: libc::c_int,
    pub flat: libc::c_int,
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2017 Nicholas Marriott <nicholas.marriott@gmail.com>
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_tree_item {
    pub parent: *mut mode_tree_item,
    pub itemdata: *mut libc::c_void,
    pub line: u_int,
    pub tag: uint64_t,
    pub name: *const libc::c_char,
    pub text: *const libc::c_char,
    pub expanded: libc::c_int,
    pub tagged: libc::c_int,
    pub draw_as_parent: libc::c_int,
    pub no_tag: libc::c_int,
    pub children: mode_tree_list,
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub tqe_next: *mut mode_tree_item,
    pub tqe_prev: *mut *mut mode_tree_item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_tree_list {
    pub tqh_first: *mut mode_tree_item,
    pub tqh_last: *mut *mut mode_tree_item,
}
pub type mode_tree_height_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u_int) -> u_int>;
pub type mode_tree_menu_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut client, _: key_code) -> ()>;
pub type mode_tree_search_cb = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *const libc::c_char,
    ) -> libc::c_int,
>;
pub type mode_tree_draw_cb = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut screen_write_ctx,
        _: u_int,
        _: u_int,
    ) -> (),
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct screen_write_ctx {
    pub wp: *mut window_pane,
    pub s: *mut screen,
    pub flags: libc::c_int,
    pub init_ctx_cb: screen_write_init_ctx_cb,
    pub arg: *mut libc::c_void,
    pub item: *mut crate::screen_write::screen_write_collect_item,
    pub scrolled: u_int,
    pub bg: u_int,
    pub cells: u_int,
    pub written: u_int,
    pub skipped: u_int,
}
pub type screen_write_init_ctx_cb =
    Option<unsafe extern "C" fn(_: *mut screen_write_ctx, _: *mut tty_ctx) -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_ctx {
    pub s: *mut screen,
    pub redraw_cb: tty_ctx_redraw_cb,
    pub set_client_cb: tty_ctx_set_client_cb,
    pub arg: *mut libc::c_void,
    pub cell: *const crate::grid::Cell,
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
    pub defaults: crate::grid::Cell,
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
pub type mode_tree_build_cb = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut mode_tree_sort_criteria,
        _: *mut uint64_t,
        _: *const libc::c_char,
    ) -> (),
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_tree_sort_criteria {
    pub field: u_int,
    pub reversed: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct menu_item {
    pub name: *const libc::c_char,
    pub key: key_code,
    pub command: *const libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_parse_input {
    pub flags: libc::c_int,
    pub file: *const libc::c_char,
    pub line: u_int,
    pub item: *mut crate::cmd_queue::cmdq_item,
    pub c: *mut client,
    pub fs: cmd_find_state,
}
pub type mode_tree_each_cb = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut client,
        _: key_code,
    ) -> (),
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_tree_menu {
    pub data: *mut mode_tree_data,
    pub c: *mut client,
    pub line: u_int,
    pub itemdata: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128i32) && __c < 256i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut mode_tree_menu_items: [menu_item; 5] = [
    {
        let mut init = menu_item {
            name: b"Scroll Left\x00" as *const u8 as *const libc::c_char,
            key: '<' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: b"Scroll Right\x00" as *const u8 as *const libc::c_char,
            key: '>' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: b"\x00" as *const u8 as *const libc::c_char,
            key: 0xff000000000u64,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: b"Cancel\x00" as *const u8 as *const libc::c_char,
            key: 'q' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: 0 as *const libc::c_char,
            key: 0xff000000000u64,
            command: 0 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn mode_tree_find_item(
    mut mtl: *mut mode_tree_list,
    mut tag: uint64_t,
) -> *mut mode_tree_item {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut child: *mut mode_tree_item = 0 as *mut mode_tree_item;
    mti = (*mtl).tqh_first;
    while !mti.is_null() {
        if (*mti).tag == tag {
            return mti;
        }
        child = mode_tree_find_item(&mut (*mti).children, tag);
        if !child.is_null() {
            return child;
        }
        mti = (*mti).entry.tqe_next
    }
    return 0 as *mut mode_tree_item;
}
unsafe extern "C" fn mode_tree_free_item(mut mti: *mut mode_tree_item) {
    mode_tree_free_items(&mut (*mti).children);
    free((*mti).name as *mut libc::c_void);
    free((*mti).text as *mut libc::c_void);
    free(mti as *mut libc::c_void);
}
unsafe extern "C" fn mode_tree_free_items(mut mtl: *mut mode_tree_list) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut mti1: *mut mode_tree_item = 0 as *mut mode_tree_item;
    mti = (*mtl).tqh_first;
    while !mti.is_null() && {
        mti1 = (*mti).entry.tqe_next;
        (1i32) != 0
    } {
        if !(*mti).entry.tqe_next.is_null() {
            (*(*mti).entry.tqe_next).entry.tqe_prev = (*mti).entry.tqe_prev
        } else {
            (*mtl).tqh_last = (*mti).entry.tqe_prev
        }
        *(*mti).entry.tqe_prev = (*mti).entry.tqe_next;
        mode_tree_free_item(mti);
        mti = mti1
    }
}
unsafe extern "C" fn mode_tree_check_selected(mut mtd: *mut mode_tree_data) {
    /*
     * If the current line would now be off screen reset the offset to the
     * last visible line.
     */
    if (*mtd).current > (*mtd).height.wrapping_sub(1u32) {
        (*mtd).offset = (*mtd)
            .current
            .wrapping_sub((*mtd).height)
            .wrapping_add(1u32)
    };
}
unsafe extern "C" fn mode_tree_clear_lines(mut mtd: *mut mode_tree_data) {
    free((*mtd).line_list as *mut libc::c_void);
    (*mtd).line_list = 0 as *mut mode_tree_line;
    (*mtd).line_size = 0u32;
}
unsafe extern "C" fn mode_tree_build_lines(
    mut mtd: *mut mode_tree_data,
    mut mtl: *mut mode_tree_list,
    mut depth: u_int,
) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut line: *mut mode_tree_line = 0 as *mut mode_tree_line;
    let mut i: u_int = 0;
    let mut flat: libc::c_int = 1i32;
    (*mtd).depth = depth;
    mti = (*mtl).tqh_first;
    while !mti.is_null() {
        (*mtd).line_list = xreallocarray(
            (*mtd).line_list as *mut libc::c_void,
            (*mtd).line_size.wrapping_add(1u32) as size_t,
            ::std::mem::size_of::<mode_tree_line>() as libc::c_ulong,
        ) as *mut mode_tree_line;
        let fresh0 = (*mtd).line_size;
        (*mtd).line_size = (*mtd).line_size.wrapping_add(1);
        line = &mut *(*mtd).line_list.offset(fresh0 as isize) as *mut mode_tree_line;
        (*line).item = mti;
        (*line).depth = depth;
        (*line).last =
            (mti == *(*((*mtl).tqh_last as *mut mode_tree_list)).tqh_last) as libc::c_int;
        (*mti).line = (*mtd).line_size.wrapping_sub(1u32);
        if !(*mti).children.tqh_first.is_null() {
            flat = 0i32
        }
        if (*mti).expanded != 0 {
            mode_tree_build_lines(mtd, &mut (*mti).children, depth.wrapping_add(1u32));
        }
        mti = (*mti).entry.tqe_next
    }
    mti = (*mtl).tqh_first;
    while !mti.is_null() {
        i = 0u32;
        while i < (*mtd).line_size {
            line = &mut *(*mtd).line_list.offset(i as isize) as *mut mode_tree_line;
            if (*line).item == mti {
                (*line).flat = flat
            }
            i = i.wrapping_add(1)
        }
        mti = (*mti).entry.tqe_next
    }
}
unsafe extern "C" fn mode_tree_clear_tagged(mut mtl: *mut mode_tree_list) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    mti = (*mtl).tqh_first;
    while !mti.is_null() {
        (*mti).tagged = 0i32;
        mode_tree_clear_tagged(&mut (*mti).children);
        mti = (*mti).entry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_up(mut mtd: *mut mode_tree_data, mut wrap: libc::c_int) {
    if (*mtd).current == 0u32 {
        if wrap != 0 {
            (*mtd).current = (*mtd).line_size.wrapping_sub(1u32);
            if (*mtd).line_size >= (*mtd).height {
                (*mtd).offset = (*mtd).line_size.wrapping_sub((*mtd).height)
            }
        }
    } else {
        (*mtd).current = (*mtd).current.wrapping_sub(1);
        if (*mtd).current < (*mtd).offset {
            (*mtd).offset = (*mtd).offset.wrapping_sub(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_down(mut mtd: *mut mode_tree_data, mut wrap: libc::c_int) {
    if (*mtd).current == (*mtd).line_size.wrapping_sub(1u32) {
        if wrap != 0 {
            (*mtd).current = 0u32;
            (*mtd).offset = 0u32
        }
    } else {
        (*mtd).current = (*mtd).current.wrapping_add(1);
        if (*mtd).current > (*mtd).offset.wrapping_add((*mtd).height).wrapping_sub(1u32) {
            (*mtd).offset = (*mtd).offset.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_get_current(mut mtd: *mut mode_tree_data) -> *mut libc::c_void {
    return (*(*(*mtd).line_list.offset((*mtd).current as isize)).item).itemdata;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_get_current_name(
    mut mtd: *mut mode_tree_data,
) -> *const libc::c_char {
    return (*(*(*mtd).line_list.offset((*mtd).current as isize)).item).name;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_expand_current(mut mtd: *mut mode_tree_data) {
    if (*(*(*mtd).line_list.offset((*mtd).current as isize)).item).expanded == 0 {
        (*(*(*mtd).line_list.offset((*mtd).current as isize)).item).expanded = 1i32;
        mode_tree_build(mtd);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_collapse_current(mut mtd: *mut mode_tree_data) {
    if (*(*(*mtd).line_list.offset((*mtd).current as isize)).item).expanded != 0 {
        (*(*(*mtd).line_list.offset((*mtd).current as isize)).item).expanded = 0i32;
        mode_tree_build(mtd);
    };
}
unsafe extern "C" fn mode_tree_get_tag(
    mut mtd: *mut mode_tree_data,
    mut tag: uint64_t,
    mut found: *mut u_int,
) -> libc::c_int {
    let mut i: u_int = 0;
    i = 0u32;
    while i < (*mtd).line_size {
        if (*(*(*mtd).line_list.offset(i as isize)).item).tag == tag {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i != (*mtd).line_size {
        *found = i;
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_expand(mut mtd: *mut mode_tree_data, mut tag: uint64_t) {
    let mut found: u_int = 0;
    if mode_tree_get_tag(mtd, tag, &mut found) == 0 {
        return;
    }
    if (*(*(*mtd).line_list.offset(found as isize)).item).expanded == 0 {
        (*(*(*mtd).line_list.offset(found as isize)).item).expanded = 1i32;
        mode_tree_build(mtd);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_set_current(
    mut mtd: *mut mode_tree_data,
    mut tag: uint64_t,
) -> libc::c_int {
    let mut found: u_int = 0;
    if mode_tree_get_tag(mtd, tag, &mut found) != 0 {
        (*mtd).current = found;
        if (*mtd).current > (*mtd).height.wrapping_sub(1u32) {
            (*mtd).offset = (*mtd)
                .current
                .wrapping_sub((*mtd).height)
                .wrapping_add(1u32)
        } else {
            (*mtd).offset = 0u32
        }
        return 1i32;
    }
    (*mtd).current = 0u32;
    (*mtd).offset = 0u32;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_count_tagged(mut mtd: *mut mode_tree_data) -> u_int {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut i: u_int = 0;
    let mut tagged: u_int = 0;
    tagged = 0u32;
    i = 0u32;
    while i < (*mtd).line_size {
        mti = (*(*mtd).line_list.offset(i as isize)).item;
        if (*mti).tagged != 0 {
            tagged = tagged.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    return tagged;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_each_tagged(
    mut mtd: *mut mode_tree_data,
    mut cb: mode_tree_each_cb,
    mut c: *mut client,
    mut key: key_code,
    mut current: libc::c_int,
) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut i: u_int = 0;
    let mut fired: libc::c_int = 0;
    fired = 0i32;
    i = 0u32;
    while i < (*mtd).line_size {
        mti = (*(*mtd).line_list.offset(i as isize)).item;
        if (*mti).tagged != 0 {
            fired = 1i32;
            cb.expect("non-null function pointer")((*mtd).modedata, (*mti).itemdata, c, key);
        }
        i = i.wrapping_add(1)
    }
    if fired == 0 && current != 0 {
        mti = (*(*mtd).line_list.offset((*mtd).current as isize)).item;
        cb.expect("non-null function pointer")((*mtd).modedata, (*mti).itemdata, c, key);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_start(
    mut wp: *mut window_pane,
    mut args: *mut args,
    mut buildcb: mode_tree_build_cb,
    mut drawcb: mode_tree_draw_cb,
    mut searchcb: mode_tree_search_cb,
    mut menucb: mode_tree_menu_cb,
    mut heightcb: mode_tree_height_cb,
    mut modedata: *mut libc::c_void,
    mut menu: *const menu_item,
    mut sort_list: *mut *const libc::c_char,
    mut sort_size: u_int,
    mut s: *mut *mut screen,
) -> *mut mode_tree_data {
    let mut mtd: *mut mode_tree_data = 0 as *mut mode_tree_data;
    let mut sort: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: u_int = 0;
    mtd = xcalloc(
        1u64,
        ::std::mem::size_of::<mode_tree_data>() as libc::c_ulong,
    ) as *mut mode_tree_data;
    (*mtd).references = 1u32;
    (*mtd).wp = wp;
    (*mtd).modedata = modedata;
    (*mtd).menu = menu;
    (*mtd).sort_list = sort_list;
    (*mtd).sort_size = sort_size;
    (*mtd).preview = (args_has(args, 'N' as u_char) == 0) as libc::c_int;
    sort = args_get(args, 'O' as u_char);
    if !sort.is_null() {
        i = 0u32;
        while i < sort_size {
            if strcasecmp(sort, *sort_list.offset(i as isize)) == 0i32 {
                (*mtd).sort_crit.field = i
            }
            i = i.wrapping_add(1)
        }
    }
    (*mtd).sort_crit.reversed = args_has(args, 'r' as u_char);
    if args_has(args, 'f' as u_char) != 0 {
        (*mtd).filter = xstrdup(args_get(args, 'f' as u_char))
    } else {
        (*mtd).filter = 0 as *mut libc::c_char
    }
    (*mtd).buildcb = buildcb;
    (*mtd).drawcb = drawcb;
    (*mtd).searchcb = searchcb;
    (*mtd).menucb = menucb;
    (*mtd).heightcb = heightcb;
    (*mtd).children.tqh_first = 0 as *mut mode_tree_item;
    (*mtd).children.tqh_last = &mut (*mtd).children.tqh_first;
    *s = &mut (*mtd).screen;
    screen_init(*s, (*(*wp).base.grid).sx, (*(*wp).base.grid).sy, 0u32);
    (**s).mode &= !(0x1i32);
    return mtd;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_zoom(mut mtd: *mut mode_tree_data, mut args: *mut args) {
    let mut wp: *mut window_pane = (*mtd).wp;
    if args_has(args, 'Z' as u_char) != 0 {
        (*mtd).zoomed = (*(*wp).window).flags & 0x8i32;
        if (*mtd).zoomed == 0 && window_zoom(wp) == 0i32 {
            server_redraw_window((*wp).window);
        }
    } else {
        (*mtd).zoomed = -(1i32)
    };
}
unsafe extern "C" fn mode_tree_set_height(mut mtd: *mut mode_tree_data) {
    let mut s: *mut screen = &mut (*mtd).screen;
    let mut height: u_int = 0;
    if (*mtd).heightcb.is_some() {
        height = (*mtd).heightcb.expect("non-null function pointer")(
            mtd as *mut libc::c_void,
            (*(*s).grid).sy,
        );
        if height < (*(*s).grid).sy {
            (*mtd).height = (*(*s).grid).sy.wrapping_sub(height)
        }
    } else {
        (*mtd).height = (*(*s).grid).sy.wrapping_div(3u32).wrapping_mul(2u32);
        if (*mtd).height > (*mtd).line_size {
            (*mtd).height = (*(*s).grid).sy.wrapping_div(2u32)
        }
    }
    if (*mtd).height < 10u32 {
        (*mtd).height = (*(*s).grid).sy
    }
    if (*(*s).grid).sy.wrapping_sub((*mtd).height) < 2u32 {
        (*mtd).height = (*(*s).grid).sy
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_build(mut mtd: *mut mode_tree_data) {
    let mut s: *mut screen = &mut (*mtd).screen;
    let mut tag: uint64_t = 0;
    if !(*mtd).line_list.is_null() {
        tag = (*(*(*mtd).line_list.offset((*mtd).current as isize)).item).tag
    } else {
        tag = 18446744073709551615u64
    }
    if !(*mtd).children.tqh_first.is_null() {
        *(*mtd).saved.tqh_last = (*mtd).children.tqh_first;
        (*(*mtd).children.tqh_first).entry.tqe_prev = (*mtd).saved.tqh_last;
        (*mtd).saved.tqh_last = (*mtd).children.tqh_last;
        (*mtd).children.tqh_first = 0 as *mut mode_tree_item;
        (*mtd).children.tqh_last = &mut (*mtd).children.tqh_first
    }
    (*mtd).children.tqh_first = 0 as *mut mode_tree_item;
    (*mtd).children.tqh_last = &mut (*mtd).children.tqh_first;
    (*mtd).buildcb.expect("non-null function pointer")(
        (*mtd).modedata,
        &mut (*mtd).sort_crit,
        &mut tag,
        (*mtd).filter,
    );
    (*mtd).no_matches = ((*mtd).children.tqh_first == 0 as *mut mode_tree_item) as libc::c_int;
    if (*mtd).no_matches != 0 {
        (*mtd).buildcb.expect("non-null function pointer")(
            (*mtd).modedata,
            &mut (*mtd).sort_crit,
            &mut tag,
            0 as *const libc::c_char,
        );
    }
    mode_tree_free_items(&mut (*mtd).saved);
    (*mtd).saved.tqh_first = 0 as *mut mode_tree_item;
    (*mtd).saved.tqh_last = &mut (*mtd).saved.tqh_first;
    mode_tree_clear_lines(mtd);
    mode_tree_build_lines(mtd, &mut (*mtd).children, 0u32);
    if tag == 18446744073709551615u64 {
        tag = (*(*(*mtd).line_list.offset((*mtd).current as isize)).item).tag
    }
    mode_tree_set_current(mtd, tag);
    (*mtd).width = (*(*s).grid).sx;
    if (*mtd).preview != 0 {
        mode_tree_set_height(mtd);
    } else {
        (*mtd).height = (*(*s).grid).sy
    }
    mode_tree_check_selected(mtd);
}
unsafe extern "C" fn mode_tree_remove_ref(mut mtd: *mut mode_tree_data) {
    (*mtd).references = (*mtd).references.wrapping_sub(1);
    if (*mtd).references == 0u32 {
        free(mtd as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_free(mut mtd: *mut mode_tree_data) {
    let mut wp: *mut window_pane = (*mtd).wp;
    if (*mtd).zoomed == 0i32 {
        server_unzoom_window((*wp).window);
    }
    mode_tree_free_items(&mut (*mtd).children);
    mode_tree_clear_lines(mtd);
    screen_free(&mut (*mtd).screen);
    free((*mtd).search as *mut libc::c_void);
    free((*mtd).filter as *mut libc::c_void);
    (*mtd).dead = 1i32;
    mode_tree_remove_ref(mtd);
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_resize(
    mut mtd: *mut mode_tree_data,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut s: *mut screen = &mut (*mtd).screen;
    screen_resize(s, sx, sy, 0i32);
    mode_tree_build(mtd);
    mode_tree_draw(mtd);
    (*(*mtd).wp).flags |= 0x1i32;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_add(
    mut mtd: *mut mode_tree_data,
    mut parent: *mut mode_tree_item,
    mut itemdata: *mut libc::c_void,
    mut tag: uint64_t,
    mut name: *const libc::c_char,
    mut text: *const libc::c_char,
    mut expanded: libc::c_int,
) -> *mut mode_tree_item {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut saved: *mut mode_tree_item = 0 as *mut mode_tree_item;
    log_debug(
        b"%s: %llu, %s %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"mode_tree_add\x00")).as_ptr(),
        tag,
        name,
        if text.is_null() {
            b"\x00" as *const u8 as *const libc::c_char
        } else {
            text
        },
    );
    mti = xcalloc(
        1u64,
        ::std::mem::size_of::<mode_tree_item>() as libc::c_ulong,
    ) as *mut mode_tree_item;
    (*mti).parent = parent;
    (*mti).itemdata = itemdata;
    (*mti).tag = tag;
    (*mti).name = xstrdup(name);
    if !text.is_null() {
        (*mti).text = xstrdup(text)
    }
    saved = mode_tree_find_item(&mut (*mtd).saved, tag);
    if !saved.is_null() {
        if parent.is_null() || (*parent).expanded != 0 {
            (*mti).tagged = (*saved).tagged
        }
        (*mti).expanded = (*saved).expanded
    } else if expanded == -(1i32) {
        (*mti).expanded = 1i32
    } else {
        (*mti).expanded = expanded
    }
    (*mti).children.tqh_first = 0 as *mut mode_tree_item;
    (*mti).children.tqh_last = &mut (*mti).children.tqh_first;
    if !parent.is_null() {
        (*mti).entry.tqe_next = 0 as *mut mode_tree_item;
        (*mti).entry.tqe_prev = (*parent).children.tqh_last;
        *(*parent).children.tqh_last = mti;
        (*parent).children.tqh_last = &mut (*mti).entry.tqe_next
    } else {
        (*mti).entry.tqe_next = 0 as *mut mode_tree_item;
        (*mti).entry.tqe_prev = (*mtd).children.tqh_last;
        *(*mtd).children.tqh_last = mti;
        (*mtd).children.tqh_last = &mut (*mti).entry.tqe_next
    }
    return mti;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_draw_as_parent(mut mti: *mut mode_tree_item) {
    (*mti).draw_as_parent = 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_no_tag(mut mti: *mut mode_tree_item) {
    (*mti).no_tag = 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_remove(
    mut mtd: *mut mode_tree_data,
    mut mti: *mut mode_tree_item,
) {
    let mut parent: *mut mode_tree_item = (*mti).parent;
    if !parent.is_null() {
        if !(*mti).entry.tqe_next.is_null() {
            (*(*mti).entry.tqe_next).entry.tqe_prev = (*mti).entry.tqe_prev
        } else {
            (*parent).children.tqh_last = (*mti).entry.tqe_prev
        }
        *(*mti).entry.tqe_prev = (*mti).entry.tqe_next
    } else {
        if !(*mti).entry.tqe_next.is_null() {
            (*(*mti).entry.tqe_next).entry.tqe_prev = (*mti).entry.tqe_prev
        } else {
            (*mtd).children.tqh_last = (*mti).entry.tqe_prev
        }
        *(*mti).entry.tqe_prev = (*mti).entry.tqe_next
    }
    mode_tree_free_item(mti);
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_draw(mut mtd: *mut mode_tree_data) {
    let mut wp: *mut window_pane = (*mtd).wp;
    let mut s: *mut screen = &mut (*mtd).screen;
    let mut line: *mut mode_tree_line = 0 as *mut mode_tree_line;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut oo: *mut crate::options::options = (*(*wp).window).options;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut gc0: GridCell = GridCell {
        data: Utf8Data {
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
    let mut gc: GridCell = GridCell {
        data: Utf8Data {
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
    let mut w: u_int = 0;
    let mut h: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut sy: u_int = 0;
    let mut box_x: u_int = 0;
    let mut box_y: u_int = 0;
    let mut width: u_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [libc::c_char; 7] = [0; 7];
    let mut tag: *const libc::c_char = 0 as *const libc::c_char;
    let mut symbol: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    let mut keylen: libc::c_int = 0;
    if (*mtd).line_size == 0u32 {
        return;
    }
    memcpy(
        &mut gc0 as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    memcpy(
        &mut gc as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    style_apply(
        &mut gc,
        oo,
        b"mode-style\x00" as *const u8 as *const libc::c_char,
        0 as *mut crate::format::format_tree,
    );
    w = (*mtd).width;
    h = (*mtd).height;
    screen_write_start(&mut ctx, s);
    screen_write_clearscreen(&mut ctx, 8u32);
    if (*mtd).line_size > 10u32 {
        keylen = 6i32
    } else {
        keylen = 4i32
    }
    i = 0u32;
    while i < (*mtd).line_size {
        if !(i < (*mtd).offset) {
            if i > (*mtd).offset.wrapping_add(h).wrapping_sub(1u32) {
                break;
            }
            line = &mut *(*mtd).line_list.offset(i as isize) as *mut mode_tree_line;
            mti = (*line).item;
            screen_write_cursormove(
                &mut ctx,
                0i32,
                i.wrapping_sub((*mtd).offset) as libc::c_int,
                0i32,
            );
            if i < 10u32 {
                snprintf(
                    key.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
                    b"(%c)  \x00" as *const u8 as *const libc::c_char,
                    ('0' as libc::c_uint).wrapping_add(i),
                );
            } else if i < 36u32 {
                snprintf(
                    key.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong,
                    b"(M-%c)\x00" as *const u8 as *const libc::c_char,
                    ('a' as libc::c_uint).wrapping_add(i.wrapping_sub(10u32)),
                );
            } else {
                *key.as_mut_ptr() = '\u{0}' as libc::c_char
            }
            if (*line).flat != 0 {
                symbol = b"\x00" as *const u8 as *const libc::c_char
            } else if (*mti).children.tqh_first.is_null() {
                symbol = b"  \x00" as *const u8 as *const libc::c_char
            } else if (*mti).expanded != 0 {
                symbol = b"- \x00" as *const u8 as *const libc::c_char
            } else {
                symbol = b"+ \x00" as *const u8 as *const libc::c_char
            }
            if (*line).depth == 0u32 {
                start = xstrdup(symbol)
            } else {
                size = (4u32).wrapping_mul((*line).depth).wrapping_add(32u32) as size_t;
                start = xcalloc(1u64, size) as *mut libc::c_char;
                j = 1u32;
                while j < (*line).depth {
                    if !(*mti).parent.is_null()
                        && (*(*mtd).line_list.offset((*(*mti).parent).line as isize)).last != 0
                    {
                        strlcat(start, b"    \x00" as *const u8 as *const libc::c_char, size);
                    } else {
                        strlcat(
                            start,
                            b"\x01x\x01   \x00" as *const u8 as *const libc::c_char,
                            size,
                        );
                    }
                    j = j.wrapping_add(1)
                }
                if (*line).last != 0 {
                    strlcat(
                        start,
                        b"\x01mq\x01> \x00" as *const u8 as *const libc::c_char,
                        size,
                    );
                } else {
                    strlcat(
                        start,
                        b"\x01tq\x01> \x00" as *const u8 as *const libc::c_char,
                        size,
                    );
                }
                strlcat(start, symbol, size);
            }
            if (*mti).tagged != 0 {
                tag = b"*\x00" as *const u8 as *const libc::c_char
            } else {
                tag = b"\x00" as *const u8 as *const libc::c_char
            }
            xasprintf(
                &mut text as *mut *mut libc::c_char,
                b"%-*s%s%s%s%s\x00" as *const u8 as *const libc::c_char,
                keylen,
                key.as_mut_ptr(),
                start,
                (*mti).name,
                tag,
                if !(*mti).text.is_null() {
                    b": \x00" as *const u8 as *const libc::c_char
                } else {
                    b"\x00" as *const u8 as *const libc::c_char
                },
            );
            width = utf8_cstrwidth(text);
            if width > w {
                width = w
            }
            free(start as *mut libc::c_void);
            if (*mti).tagged != 0 {
                gc.attr = (gc.attr as libc::c_int ^ 0x1i32) as u_short;
                gc0.attr = (gc0.attr as libc::c_int ^ 0x1i32) as u_short
            }
            if i != (*mtd).current {
                screen_write_clearendofline(&mut ctx, 8u32);
                screen_write_nputs(
                    &mut ctx as *mut screen_write_ctx,
                    w as ssize_t,
                    &mut gc0 as *mut GridCell,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    text,
                );
                if !(*mti).text.is_null() {
                    format_draw(
                        &mut ctx,
                        &mut gc0,
                        w.wrapping_sub(width),
                        (*mti).text,
                        0 as *mut style_ranges,
                    );
                }
            } else {
                screen_write_clearendofline(&mut ctx, gc.bg as u_int);
                screen_write_nputs(
                    &mut ctx as *mut screen_write_ctx,
                    w as ssize_t,
                    &mut gc as *mut GridCell,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    text,
                );
                if !(*mti).text.is_null() {
                    format_draw(
                        &mut ctx,
                        &mut gc,
                        w.wrapping_sub(width),
                        (*mti).text,
                        0 as *mut style_ranges,
                    );
                }
            }
            free(text as *mut libc::c_void);
            if (*mti).tagged != 0 {
                gc.attr = (gc.attr as libc::c_int ^ 0x1i32) as u_short;
                gc0.attr = (gc0.attr as libc::c_int ^ 0x1i32) as u_short
            }
        }
        i = i.wrapping_add(1)
    }
    sy = (*(*s).grid).sy;
    if (*mtd).preview == 0 || sy <= 4u32 || h <= 4u32 || sy.wrapping_sub(h) <= 4u32 || w <= 4u32 {
        screen_write_stop(&mut ctx);
        return;
    }
    line = &mut *(*mtd).line_list.offset((*mtd).current as isize) as *mut mode_tree_line;
    mti = (*line).item;
    if (*mti).draw_as_parent != 0 {
        mti = (*mti).parent
    }
    screen_write_cursormove(&mut ctx, 0i32, h as libc::c_int, 0i32);
    screen_write_box(&mut ctx, w, sy.wrapping_sub(h));
    if !(*mtd).sort_list.is_null() {
        xasprintf(
            &mut text as *mut *mut libc::c_char,
            b" %s (sort: %s%s)\x00" as *const u8 as *const libc::c_char,
            (*mti).name,
            *(*mtd).sort_list.offset((*mtd).sort_crit.field as isize),
            if (*mtd).sort_crit.reversed != 0 {
                b", reversed\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        );
    } else {
        xasprintf(
            &mut text as *mut *mut libc::c_char,
            b" %s\x00" as *const u8 as *const libc::c_char,
            (*mti).name,
        );
    }
    if w.wrapping_sub(2u32) as libc::c_ulong >= strlen(text) {
        screen_write_cursormove(&mut ctx, 1i32, h as libc::c_int, 0i32);
        screen_write_puts(
            &mut ctx as *mut screen_write_ctx,
            &mut gc0 as *mut GridCell,
            b"%s\x00" as *const u8 as *const libc::c_char,
            text,
        );
        if (*mtd).no_matches != 0 {
            n = (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong).wrapping_sub(1u64)
        } else {
            n = (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong).wrapping_sub(1u64)
        }
        if !(*mtd).filter.is_null()
            && w.wrapping_sub(2u32) as libc::c_ulong
                >= strlen(text)
                    .wrapping_add(10u64)
                    .wrapping_add(n)
                    .wrapping_add(2u64)
        {
            screen_write_puts(
                &mut ctx as *mut screen_write_ctx,
                &mut gc0 as *mut GridCell,
                b" (filter: \x00" as *const u8 as *const libc::c_char,
            );
            if (*mtd).no_matches != 0 {
                screen_write_puts(
                    &mut ctx as *mut screen_write_ctx,
                    &mut gc as *mut GridCell,
                    b"no matches\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                screen_write_puts(
                    &mut ctx as *mut screen_write_ctx,
                    &mut gc0 as *mut GridCell,
                    b"active\x00" as *const u8 as *const libc::c_char,
                );
            }
            screen_write_puts(
                &mut ctx as *mut screen_write_ctx,
                &mut gc0 as *mut GridCell,
                b") \x00" as *const u8 as *const libc::c_char,
            );
        } else {
            screen_write_puts(
                &mut ctx as *mut screen_write_ctx,
                &mut gc0 as *mut GridCell,
                b" \x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    free(text as *mut libc::c_void);
    box_x = w.wrapping_sub(4u32);
    box_y = sy.wrapping_sub(h).wrapping_sub(2u32);
    if box_x != 0u32 && box_y != 0u32 {
        screen_write_cursormove(&mut ctx, 2i32, h.wrapping_add(1u32) as libc::c_int, 0i32);
        (*mtd).drawcb.expect("non-null function pointer")(
            (*mtd).modedata,
            (*mti).itemdata,
            &mut ctx,
            box_x,
            box_y,
        );
    }
    screen_write_stop(&mut ctx);
}
unsafe extern "C" fn mode_tree_search_for(mut mtd: *mut mode_tree_data) -> *mut mode_tree_item {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut last: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut next: *mut mode_tree_item = 0 as *mut mode_tree_item;
    if (*mtd).search.is_null() {
        return 0 as *mut mode_tree_item;
    }
    last = (*(*mtd).line_list.offset((*mtd).current as isize)).item;
    mti = last;
    loop {
        if !(*mti).children.tqh_first.is_null() {
            mti = (*mti).children.tqh_first
        } else {
            next = (*mti).entry.tqe_next;
            if !next.is_null() {
                mti = next
            } else {
                loop {
                    mti = (*mti).parent;
                    if mti.is_null() {
                        break;
                    }
                    next = (*mti).entry.tqe_next;
                    if next.is_null() {
                        continue;
                    }
                    mti = next;
                    break;
                }
            }
        }
        if mti.is_null() {
            mti = (*mtd).children.tqh_first
        }
        if mti == last {
            break;
        }
        if (*mtd).searchcb.is_none() {
            if !strstr((*mti).name, (*mtd).search).is_null() {
                return mti;
            }
        } else if (*mtd).searchcb.expect("non-null function pointer")(
            (*mtd).modedata,
            (*mti).itemdata,
            (*mtd).search,
        ) != 0
        {
            return mti;
        }
    }
    return 0 as *mut mode_tree_item;
}
unsafe extern "C" fn mode_tree_search_set(mut mtd: *mut mode_tree_data) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut loop_0: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut tag: uint64_t = 0;
    mti = mode_tree_search_for(mtd);
    if mti.is_null() {
        return;
    }
    tag = (*mti).tag;
    loop_0 = (*mti).parent;
    while !loop_0.is_null() {
        (*loop_0).expanded = 1i32;
        loop_0 = (*loop_0).parent
    }
    mode_tree_build(mtd);
    mode_tree_set_current(mtd, tag);
    mode_tree_draw(mtd);
    (*(*mtd).wp).flags |= 0x1i32;
}
unsafe extern "C" fn mode_tree_search_callback(
    mut _c: *mut client,
    mut data: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut _done: libc::c_int,
) -> libc::c_int {
    let mut mtd: *mut mode_tree_data = data as *mut mode_tree_data;
    if (*mtd).dead != 0 {
        return 0i32;
    }
    free((*mtd).search as *mut libc::c_void);
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 {
        (*mtd).search = 0 as *mut libc::c_char;
        return 0i32;
    }
    (*mtd).search = xstrdup(s);
    mode_tree_search_set(mtd);
    return 0i32;
}
unsafe extern "C" fn mode_tree_search_free(mut data: *mut libc::c_void) {
    mode_tree_remove_ref(data as *mut mode_tree_data);
}
unsafe extern "C" fn mode_tree_filter_callback(
    mut _c: *mut client,
    mut data: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut _done: libc::c_int,
) -> libc::c_int {
    let mut mtd: *mut mode_tree_data = data as *mut mode_tree_data;
    if (*mtd).dead != 0 {
        return 0i32;
    }
    if !(*mtd).filter.is_null() {
        free((*mtd).filter as *mut libc::c_void);
    }
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 {
        (*mtd).filter = 0 as *mut libc::c_char
    } else {
        (*mtd).filter = xstrdup(s)
    }
    mode_tree_build(mtd);
    mode_tree_draw(mtd);
    (*(*mtd).wp).flags |= 0x1i32;
    return 0i32;
}
unsafe extern "C" fn mode_tree_filter_free(mut data: *mut libc::c_void) {
    mode_tree_remove_ref(data as *mut mode_tree_data);
}
unsafe extern "C" fn mode_tree_menu_callback(
    mut _menu: *mut menu,
    mut _idx: u_int,
    mut key: key_code,
    mut data: *mut libc::c_void,
) {
    let mut mtm: *mut mode_tree_menu = data as *mut mode_tree_menu;
    let mut mtd: *mut mode_tree_data = (*mtm).data;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    if !((*mtd).dead != 0 || key == 0xff000000000u64) {
        if !((*mtm).line >= (*mtd).line_size) {
            mti = (*(*mtd).line_list.offset((*mtm).line as isize)).item;
            if !((*mti).itemdata != (*mtm).itemdata) {
                (*mtd).current = (*mtm).line;
                (*mtd).menucb.expect("non-null function pointer")((*mtd).modedata, (*mtm).c, key);
            }
        }
    }
    mode_tree_remove_ref(mtd);
    free(mtm as *mut libc::c_void);
}
unsafe extern "C" fn mode_tree_display_menu(
    mut mtd: *mut mode_tree_data,
    mut c: *mut client,
    mut x: u_int,
    mut y: u_int,
    mut outside: libc::c_int,
) {
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut menu: *mut menu = 0 as *mut menu;
    let mut items: *const menu_item = 0 as *const menu_item;
    let mut mtm: *mut mode_tree_menu = 0 as *mut mode_tree_menu;
    let mut title: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: u_int = 0;
    if (*mtd).offset.wrapping_add(y) > (*mtd).line_size.wrapping_sub(1u32) {
        line = (*mtd).current
    } else {
        line = (*mtd).offset.wrapping_add(y)
    }
    mti = (*(*mtd).line_list.offset(line as isize)).item;
    if outside == 0 {
        items = (*mtd).menu;
        xasprintf(
            &mut title as *mut *mut libc::c_char,
            b"#[align=centre]%s\x00" as *const u8 as *const libc::c_char,
            (*mti).name,
        );
    } else {
        items = mode_tree_menu_items.as_ptr();
        title = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
    }
    menu = menu_create(title);
    menu_add_items(
        menu,
        items,
        0 as *mut crate::cmd_queue::cmdq_item,
        0 as *mut client,
        0 as *mut cmd_find_state,
    );
    free(title as *mut libc::c_void);
    mtm = xmalloc(::std::mem::size_of::<mode_tree_menu>() as libc::c_ulong) as *mut mode_tree_menu;
    (*mtm).data = mtd;
    (*mtm).c = c;
    (*mtm).line = line;
    (*mtm).itemdata = (*mti).itemdata;
    (*mtd).references = (*mtd).references.wrapping_add(1);
    if x >= (*menu).width.wrapping_add(4u32).wrapping_div(2u32) {
        x = (x).wrapping_sub((*menu).width.wrapping_add(4u32).wrapping_div(2u32))
    } else {
        x = 0u32
    }
    if menu_display(
        menu,
        0i32,
        0 as *mut crate::cmd_queue::cmdq_item,
        x,
        y,
        c,
        0 as *mut cmd_find_state,
        Some(
            mode_tree_menu_callback
                as unsafe extern "C" fn(
                    _: *mut menu,
                    _: u_int,
                    _: key_code,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        mtm as *mut libc::c_void,
    ) != 0i32
    {
        menu_free(menu);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_key(
    mut mtd: *mut mode_tree_data,
    mut c: *mut client,
    mut key: *mut key_code,
    mut m: *mut mouse_event,
    mut xp: *mut u_int,
    mut yp: *mut u_int,
) -> libc::c_int {
    let mut line: *mut mode_tree_line = 0 as *mut mode_tree_line;
    let mut current: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut parent: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut i: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut choice: libc::c_int = 0;
    let mut tmp: key_code = 0;
    if *key & 0xfffffffffffu64 >= key_code_code::MOUSE
        && (*key & 0xfffffffffffu64) < key_code_code::BSPACE
        && !m.is_null()
    {
        if cmd_mouse_at((*mtd).wp, m, &mut x, &mut y, 0i32) != 0i32 {
            *key = 0xff000000000u64;
            return 0i32;
        }
        if !xp.is_null() {
            *xp = x
        }
        if !yp.is_null() {
            *yp = y
        }
        if x > (*mtd).width || y > (*mtd).height {
            if *key == key_code_code::MOUSEDOWN3_PANE {
                mode_tree_display_menu(mtd, c, x, y, 1i32);
            }
            if (*mtd).preview == 0 {
                *key = 0xff000000000u64
            }
            return 0i32;
        }
        if (*mtd).offset.wrapping_add(y) < (*mtd).line_size {
            if *key == key_code_code::MOUSEDOWN1_PANE
                || *key == key_code_code::MOUSEDOWN3_PANE
                || *key == key_code_code::DOUBLECLICK1_PANE
            {
                (*mtd).current = (*mtd).offset.wrapping_add(y)
            }
            if *key == key_code_code::DOUBLECLICK1_PANE {
                *key = '\r' as i32 as key_code
            } else {
                if *key == key_code_code::MOUSEDOWN3_PANE {
                    mode_tree_display_menu(mtd, c, x, y, 0i32);
                }
                *key = 0xff000000000u64
            }
        } else {
            if *key == key_code_code::MOUSEDOWN3_PANE {
                mode_tree_display_menu(mtd, c, x, y, 0i32);
            }
            *key = 0xff000000000u64
        }
        return 0i32;
    }
    line = &mut *(*mtd).line_list.offset((*mtd).current as isize) as *mut mode_tree_line;
    current = (*line).item;
    choice = -(1i32);
    if *key >= '0' as i32 as libc::c_ulonglong && *key <= '9' as i32 as libc::c_ulonglong {
        choice = (*key).wrapping_sub('0' as i32 as libc::c_ulonglong) as libc::c_int
    } else if *key & 0xf00000000000u64 == 0x100000000000u64 {
        tmp = *key & 0xfffffffffffu64;
        if tmp >= 'a' as i32 as libc::c_ulonglong && tmp <= 'z' as i32 as libc::c_ulonglong {
            choice = (10u64).wrapping_add(tmp.wrapping_sub('a' as i32 as libc::c_ulonglong))
                as libc::c_int
        }
    }
    if choice != -(1i32) {
        if choice as u_int > (*mtd).line_size.wrapping_sub(1u32) {
            *key = 0xff000000000u64;
            return 0i32;
        }
        (*mtd).current = choice as u_int;
        *key = '\r' as i32 as key_code;
        return 0i32;
    }
    match *key {
        113 | 27 | 7 => {
            /* Escape */
            /* C-g */
            return 1i32;
        }
        68719476908 | 107 | 68719476822 | 16 => {
            /* C-p */
            mode_tree_up(mtd, 1i32);
        }
        68719476909 | 106 | 68719476828 | 14 => {
            /* C-n */
            mode_tree_down(mtd, 1i32);
        }
        103 | 68719476906 | 2 => {
            /* C-b */
            i = 0u32;
            while i < (*mtd).height {
                if (*mtd).current == 0u32 {
                    break;
                }
                mode_tree_up(mtd, 1i32);
                i = i.wrapping_add(1)
            }
        }
        71 | 68719476905 | 6 => {
            /* C-f */
            i = 0u32;
            while i < (*mtd).height {
                if (*mtd).current == (*mtd).line_size.wrapping_sub(1u32) {
                    break;
                }
                mode_tree_down(mtd, 1i32);
                i = i.wrapping_add(1)
            }
        }
        68719476903 => {
            (*mtd).current = 0u32;
            (*mtd).offset = 0u32
        }
        68719476904 => {
            (*mtd).current = (*mtd).line_size.wrapping_sub(1u32);
            if (*mtd).current > (*mtd).height.wrapping_sub(1u32) {
                (*mtd).offset = (*mtd)
                    .current
                    .wrapping_sub((*mtd).height)
                    .wrapping_add(1u32)
            } else {
                (*mtd).offset = 0u32
            }
        }
        116 => {
            /*
             * Do not allow parents and children to both be tagged: untag
             * all parents and children of current.
             */
            if !((*current).no_tag != 0) {
                if (*current).tagged == 0 {
                    parent = (*current).parent;
                    while !parent.is_null() {
                        (*parent).tagged = 0i32;
                        parent = (*parent).parent
                    }
                    mode_tree_clear_tagged(&mut (*current).children);
                    (*current).tagged = 1i32
                } else {
                    (*current).tagged = 0i32
                }
                if !m.is_null() {
                    mode_tree_down(mtd, 0i32);
                }
            }
        }
        84 => {
            i = 0u32;
            while i < (*mtd).line_size {
                (*(*(*mtd).line_list.offset(i as isize)).item).tagged = 0i32;
                i = i.wrapping_add(1)
            }
        }
        20 => {
            /* C-t */
            i = 0u32;
            while i < (*mtd).line_size {
                if (*(*(*mtd).line_list.offset(i as isize)).item)
                    .parent
                    .is_null()
                    && (*(*(*mtd).line_list.offset(i as isize)).item).no_tag == 0
                    || !(*(*(*mtd).line_list.offset(i as isize)).item)
                        .parent
                        .is_null()
                        && (*(*(*(*mtd).line_list.offset(i as isize)).item).parent).no_tag != 0
                {
                    (*(*(*mtd).line_list.offset(i as isize)).item).tagged = 1i32
                } else {
                    (*(*(*mtd).line_list.offset(i as isize)).item).tagged = 0i32
                }
                i = i.wrapping_add(1)
            }
        }
        79 => {
            (*mtd).sort_crit.field = (*mtd).sort_crit.field.wrapping_add(1);
            if (*mtd).sort_crit.field >= (*mtd).sort_size {
                (*mtd).sort_crit.field = 0u32
            }
            mode_tree_build(mtd);
        }
        114 => {
            (*mtd).sort_crit.reversed = ((*mtd).sort_crit.reversed == 0) as libc::c_int;
            mode_tree_build(mtd);
        }
        68719476910 | 104 | 45 => {
            if (*line).flat != 0 || (*current).expanded == 0 {
                current = (*current).parent
            }
            if current.is_null() {
                mode_tree_up(mtd, 0i32);
            } else {
                (*current).expanded = 0i32;
                (*mtd).current = (*current).line;
                mode_tree_build(mtd);
            }
        }
        68719476911 | 108 | 43 => {
            if (*line).flat != 0 || (*current).expanded != 0 {
                mode_tree_down(mtd, 0i32);
            } else if (*line).flat == 0 {
                (*current).expanded = 1i32;
                mode_tree_build(mtd);
            }
        }
        17592186044461 => {
            mti = (*mtd).children.tqh_first;
            while !mti.is_null() {
                (*mti).expanded = 0i32;
                mti = (*mti).entry.tqe_next
            }
            mode_tree_build(mtd);
        }
        17592186044459 => {
            mti = (*mtd).children.tqh_first;
            while !mti.is_null() {
                (*mti).expanded = 1i32;
                mti = (*mti).entry.tqe_next
            }
            mode_tree_build(mtd);
        }
        63 | 47 | 19 => {
            /* C-s */
            (*mtd).references = (*mtd).references.wrapping_add(1);
            status_prompt_set(
                c,
                0 as *mut cmd_find_state,
                b"(search) \x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char,
                Some(
                    mode_tree_search_callback
                        as unsafe extern "C" fn(
                            _: *mut client,
                            _: *mut libc::c_void,
                            _: *const libc::c_char,
                            _: libc::c_int,
                        ) -> libc::c_int,
                ),
                Some(mode_tree_search_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                mtd as *mut libc::c_void,
                0x8i32,
            );
        }
        110 => {
            mode_tree_search_set(mtd);
        }
        102 => {
            (*mtd).references = (*mtd).references.wrapping_add(1);
            status_prompt_set(
                c,
                0 as *mut cmd_find_state,
                b"(filter) \x00" as *const u8 as *const libc::c_char,
                (*mtd).filter,
                Some(
                    mode_tree_filter_callback
                        as unsafe extern "C" fn(
                            _: *mut client,
                            _: *mut libc::c_void,
                            _: *const libc::c_char,
                            _: libc::c_int,
                        ) -> libc::c_int,
                ),
                Some(mode_tree_filter_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                mtd as *mut libc::c_void,
                0x8i32,
            );
        }
        118 => {
            (*mtd).preview = ((*mtd).preview == 0) as libc::c_int;
            mode_tree_build(mtd);
            if (*mtd).preview != 0 {
                mode_tree_check_selected(mtd);
            }
        }
        _ => {}
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mode_tree_run_command(
    mut c: *mut client,
    mut fs: *mut cmd_find_state,
    mut template: *const libc::c_char,
    mut name: *const libc::c_char,
) {
    let mut state: *mut crate::cmd_queue::cmdq_state = 0 as *mut crate::cmd_queue::cmdq_state;
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut status: cmd_parse_status = CMD_PARSE_EMPTY;
    command = cmd_template_replace(template, name, 1i32);
    if !command.is_null() && *command as libc::c_int != '\u{0}' as i32 {
        state = cmdq_new_state(fs, 0 as *mut key_event, 0i32);
        status = cmd_parse_and_append(command, 0 as *mut cmd_parse_input, c, state, &mut error);
        if status == CMD_PARSE_ERROR {
            if !c.is_null() {
                *error = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<u_char>() as libc::c_ulong > 1u64 {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *error as u_char as libc::c_int;
                            __res = if __c < -(128i32) || __c > 255i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            }
                        } else {
                            __res = toupper(*error as u_char as libc::c_int)
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*error as u_char as libc::c_int as isize)
                    }
                    __res
                }) as libc::c_char;
                status_message_set(
                    c,
                    -(1i32),
                    1i32,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    error,
                );
            }
            free(error as *mut libc::c_void);
        }
        cmdq_free_state(state);
    }
    free(command as *mut libc::c_void);
}