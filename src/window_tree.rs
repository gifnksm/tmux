use crate::{
    grid::Cell as GridCell, key_code::code as key_code_code, menu::Item as MenuItem,
    screen::Screen, utf8::Utf8Data,
};
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn format_true(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn format_single(
        _: *mut crate::cmd_queue::cmdq_item,
        _: *const libc::c_char,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_find_clear_state(_: *mut cmd_find_state, _: libc::c_int);
    #[no_mangle]
    fn cmd_find_from_winlink_pane(
        _: *mut cmd_find_state,
        _: *mut winlink,
        _: *mut window_pane,
        _: libc::c_int,
    );
    #[no_mangle]
    fn cmdq_get_callback1(
        _: *const libc::c_char,
        _: cmdq_cb,
        _: *mut libc::c_void,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_append(
        _: *mut client,
        _: *mut crate::cmd_queue::cmdq_item,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn server_set_marked(_: *mut session, _: *mut winlink, _: *mut window_pane);
    #[no_mangle]
    fn server_clear_marked();
    #[no_mangle]
    fn server_kill_pane(_: *mut window_pane);
    #[no_mangle]
    fn server_kill_window(_: *mut window, _: libc::c_int);
    #[no_mangle]
    fn server_renumber_all();
    #[no_mangle]
    fn server_destroy_session(_: *mut session);
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn screen_write_puts(
        _: *mut screen_write_ctx,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn screen_write_vline(_: *mut screen_write_ctx, _: u_int, _: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn screen_write_box(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_preview(
        _: *mut screen_write_ctx,
        _: *mut crate::screen::Screen,
        _: u_int,
        _: u_int,
    );
    #[no_mangle]
    fn screen_write_cursormove(
        _: *mut screen_write_ctx,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_count(_: *mut winlinks) -> u_int;
    #[no_mangle]
    fn window_has_pane(_: *mut window, _: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    #[no_mangle]
    fn window_count_panes(_: *mut window) -> u_int;
    #[no_mangle]
    fn window_pane_find_by_id(_: u_int) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_reset_mode(_: *mut window_pane);
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
    fn mode_tree_count_tagged(_: *mut crate::mode_tree::mode_tree_data) -> u_int;
    #[no_mangle]
    fn mode_tree_get_current(_: *mut crate::mode_tree::mode_tree_data) -> *mut libc::c_void;
    #[no_mangle]
    fn mode_tree_expand_current(_: *mut crate::mode_tree::mode_tree_data);
    #[no_mangle]
    fn mode_tree_expand(_: *mut crate::mode_tree::mode_tree_data, _: uint64_t);
    #[no_mangle]
    fn mode_tree_set_current(_: *mut crate::mode_tree::mode_tree_data, _: uint64_t) -> libc::c_int;
    #[no_mangle]
    fn mode_tree_each_tagged(
        _: *mut crate::mode_tree::mode_tree_data,
        _: mode_tree_each_cb,
        _: *mut client,
        _: key_code,
        _: libc::c_int,
    );
    #[no_mangle]
    fn mode_tree_start(
        _: *mut window_pane,
        _: *mut args,
        _: mode_tree_build_cb,
        _: mode_tree_draw_cb,
        _: mode_tree_search_cb,
        _: mode_tree_menu_cb,
        _: mode_tree_height_cb,
        _: *mut libc::c_void,
        _: *const MenuItem,
        _: *mut *const libc::c_char,
        _: u_int,
        _: *mut *mut crate::screen::Screen,
    ) -> *mut crate::mode_tree::mode_tree_data;
    #[no_mangle]
    fn mode_tree_zoom(_: *mut crate::mode_tree::mode_tree_data, _: *mut args);
    #[no_mangle]
    fn mode_tree_build(_: *mut crate::mode_tree::mode_tree_data);
    #[no_mangle]
    fn mode_tree_free(_: *mut crate::mode_tree::mode_tree_data);
    #[no_mangle]
    fn mode_tree_resize(_: *mut crate::mode_tree::mode_tree_data, _: u_int, _: u_int);
    #[no_mangle]
    fn mode_tree_add(
        _: *mut crate::mode_tree::mode_tree_data,
        _: *mut crate::mode_tree::mode_tree_item,
        _: *mut libc::c_void,
        _: uint64_t,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut crate::mode_tree::mode_tree_item;
    #[no_mangle]
    fn mode_tree_remove(
        _: *mut crate::mode_tree::mode_tree_data,
        _: *mut crate::mode_tree::mode_tree_item,
    );
    #[no_mangle]
    fn mode_tree_draw(_: *mut crate::mode_tree::mode_tree_data);
    #[no_mangle]
    fn mode_tree_key(
        _: *mut crate::mode_tree::mode_tree_data,
        _: *mut client,
        _: *mut key_code,
        _: *mut mouse_event,
        _: *mut u_int,
        _: *mut u_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn mode_tree_run_command(
        _: *mut client,
        _: *mut cmd_find_state,
        _: *const libc::c_char,
        _: *const libc::c_char,
    );
    #[no_mangle]
    fn session_find_by_id(_: u_int) -> *mut session;
    #[no_mangle]
    fn session_destroy(_: *mut session, _: libc::c_int, _: *const libc::c_char);
    #[no_mangle]
    fn session_group_contains(_: *mut session) -> *mut session_group;
    #[no_mangle]
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    fn osdep_get_name(_: libc::c_int, _: *mut libc::c_char) -> *mut libc::c_char;
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
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
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
pub type overlay_mode_cb = Option<
    unsafe extern "C" fn(
        _: *mut client,
        _: *mut u_int,
        _: *mut u_int,
    ) -> *mut crate::screen::Screen,
>;

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
    pub screen: *mut crate::screen::Screen,
    pub base: crate::screen::Screen,
    pub status_screen: crate::screen::Screen,
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
    pub screen: *mut crate::screen::Screen,
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
        ) -> *mut crate::screen::Screen,
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
    pub screen: crate::screen::Screen,
    pub active: *mut crate::screen::Screen,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct screen_write_ctx {
    pub wp: *mut window_pane,
    pub s: *mut crate::screen::Screen,
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
    pub s: *mut crate::screen::Screen,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: C2RustUnnamed_34,
    pub entry: C2RustUnnamed_33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_34 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub type cmd_retval = libc::c_int;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type cmdq_cb = Option<
    unsafe extern "C" fn(_: *mut crate::cmd_queue::cmdq_item, _: *mut libc::c_void) -> cmd_retval,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_tree_sort_criteria {
    pub field: u_int,
    pub reversed: libc::c_int,
}
pub type mode_tree_build_cb = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut mode_tree_sort_criteria,
        _: *mut uint64_t,
        _: *const libc::c_char,
    ) -> (),
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
pub type mode_tree_search_cb = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *const libc::c_char,
    ) -> libc::c_int,
>;
pub type mode_tree_menu_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut client, _: key_code) -> ()>;
pub type mode_tree_height_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u_int) -> u_int>;
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
pub struct window_tree_modedata {
    pub wp: *mut window_pane,
    pub dead: libc::c_int,
    pub references: libc::c_int,
    pub data: *mut crate::mode_tree::mode_tree_data,
    pub format: *mut libc::c_char,
    pub command: *mut libc::c_char,
    pub squash_groups: libc::c_int,
    pub item_list: *mut *mut window_tree_itemdata,
    pub item_size: u_int,
    pub entered: *const libc::c_char,
    pub fs: cmd_find_state,
    pub type_0: window_tree_type,
    pub offset: libc::c_int,
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub start: u_int,
    pub end: u_int,
    pub each: u_int,
}
pub type window_tree_type = libc::c_uint;
pub const WINDOW_TREE_PANE: window_tree_type = 3;
pub const WINDOW_TREE_WINDOW: window_tree_type = 2;
pub const WINDOW_TREE_SESSION: window_tree_type = 1;
pub const WINDOW_TREE_NONE: window_tree_type = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_tree_itemdata {
    pub type_0: window_tree_type,
    pub session: libc::c_int,
    pub winlink: libc::c_int,
    pub pane: libc::c_int,
}
pub const WINDOW_TREE_BY_TIME: window_tree_sort_type = 2;
pub const WINDOW_TREE_BY_NAME: window_tree_sort_type = 1;
pub const WINDOW_TREE_BY_INDEX: window_tree_sort_type = 0;
pub type window_tree_sort_type = libc::c_uint;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128i32) && __c < 256i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut window_tree_menu_items: [MenuItem; 13] = [
    {
        let mut init = MenuItem {
            name: b"Select\x00" as *const u8 as *const libc::c_char,
            key: '\r' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"Expand\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::RIGHT,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"Mark\x00" as *const u8 as *const libc::c_char,
            key: 'm' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"\x00" as *const u8 as *const libc::c_char,
            key: 0xff000000000u64,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"Tag\x00" as *const u8 as *const libc::c_char,
            key: 't' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"Tag All\x00" as *const u8 as *const libc::c_char,
            key: '\u{14}' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"Tag None\x00" as *const u8 as *const libc::c_char,
            key: 'T' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"\x00" as *const u8 as *const libc::c_char,
            key: 0xff000000000u64,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"Kill\x00" as *const u8 as *const libc::c_char,
            key: 'x' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"Kill Tagged\x00" as *const u8 as *const libc::c_char,
            key: 'X' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"\x00" as *const u8 as *const libc::c_char,
            key: 0xff000000000u64,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"Cancel\x00" as *const u8 as *const libc::c_char,
            key: 'q' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: 0 as *const libc::c_char,
            key: 0xff000000000u64,
            command: 0 as *const libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub static mut window_tree_mode: window_mode = {
    {
        let mut init =
                window_mode{name:
                                b"tree-mode\x00" as *const u8 as
                                    *const libc::c_char,
                            default_format:
                                b"#{?pane_format,#{?pane_marked,#[reverse],}#{pane_current_command}#{?pane_active,*,}#{?pane_marked,M,}#{?#{&&:#{pane_title},#{!=:#{pane_title},#{host_short}}},: \"#{pane_title}\",},#{?window_format,#{?window_marked_flag,#[reverse],}#{window_name}#{window_flags}#{?#{&&:#{==:#{window_panes},1},#{&&:#{pane_title},#{!=:#{pane_title},#{host_short}}}},: \"#{pane_title}\",},#{session_windows} windows#{?session_grouped, (group #{session_group}: #{session_group_list}),}#{?session_attached, (attached),}}}\x00"
                                    as *const u8 as *const libc::c_char,
                            init:
                                Some(window_tree_init as
                                         unsafe extern "C" fn(_:
                                                                  *mut window_mode_entry,
                                                              _:
                                                                  *mut cmd_find_state,
                                                              _: *mut args)
                                             -> *mut crate::screen::Screen),
                            free:
                                Some(window_tree_free as
                                         unsafe extern "C" fn(_:
                                                                  *mut window_mode_entry)
                                             -> ()),
                            resize:
                                Some(window_tree_resize as
                                         unsafe extern "C" fn(_:
                                                                  *mut window_mode_entry,
                                                              _: u_int,
                                                              _: u_int)
                                             -> ()),
                            key:
                                Some(window_tree_key as
                                         unsafe extern "C" fn(_:
                                                                  *mut window_mode_entry,
                                                              _: *mut client,
                                                              _: *mut session,
                                                              _: *mut winlink,
                                                              _: key_code,
                                                              _:
                                                                  *mut mouse_event)
                                             -> ()),
                            key_table: None,
                            command: None,
                            formats: None,};
        init
    }
};
static mut window_tree_sort_list: [*const libc::c_char; 3] = [
    b"index\x00" as *const u8 as *const libc::c_char,
    b"name\x00" as *const u8 as *const libc::c_char,
    b"time\x00" as *const u8 as *const libc::c_char,
];
static mut window_tree_sort: *mut mode_tree_sort_criteria = 0 as *mut mode_tree_sort_criteria;
unsafe extern "C" fn window_tree_pull_item(
    mut item: *mut window_tree_itemdata,
    mut sp: *mut *mut session,
    mut wlp: *mut *mut winlink,
    mut wp: *mut *mut window_pane,
) {
    *wp = 0 as *mut window_pane;
    *wlp = 0 as *mut winlink;
    *sp = session_find_by_id((*item).session as u_int);
    if (*sp).is_null() {
        return;
    }
    if (*item).type_0 == WINDOW_TREE_SESSION {
        *wlp = (**sp).curw;
        *wp = (*(**wlp).window).active;
        return;
    }
    *wlp = winlink_find_by_index(&mut (**sp).windows, (*item).winlink);
    if (*wlp).is_null() {
        *sp = 0 as *mut session;
        return;
    }
    if (*item).type_0 == WINDOW_TREE_WINDOW {
        *wp = (*(**wlp).window).active;
        return;
    }
    *wp = window_pane_find_by_id((*item).pane as u_int);
    if window_has_pane((**wlp).window, *wp) == 0 {
        *wp = 0 as *mut window_pane
    }
    if (*wp).is_null() {
        *sp = 0 as *mut session;
        *wlp = 0 as *mut winlink;
        return;
    };
}
unsafe extern "C" fn window_tree_add_item(
    mut data: *mut window_tree_modedata,
) -> *mut window_tree_itemdata {
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    (*data).item_list = xreallocarray(
        (*data).item_list as *mut libc::c_void,
        (*data).item_size.wrapping_add(1u32) as size_t,
        ::std::mem::size_of::<*mut window_tree_itemdata>() as libc::c_ulong,
    ) as *mut *mut window_tree_itemdata;
    let fresh0 = (*data).item_size;
    (*data).item_size = (*data).item_size.wrapping_add(1);
    let ref mut fresh1 = *(*data).item_list.offset(fresh0 as isize);
    *fresh1 = xcalloc(
        1u64,
        ::std::mem::size_of::<window_tree_itemdata>() as libc::c_ulong,
    ) as *mut window_tree_itemdata;
    item = *fresh1;
    return item;
}
unsafe extern "C" fn window_tree_free_item(mut item: *mut window_tree_itemdata) {
    free(item as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_cmp_session(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const session = a0 as *const *const session;
    let mut b: *const *const session = b0 as *const *const session;
    let mut sa: *const session = *a;
    let mut sb: *const session = *b;
    let mut result: libc::c_int = 0i32;
    let mut current_block_3: u64;
    match (*window_tree_sort).field {
        0 => {
            result = (*sa).id.wrapping_sub((*sb).id) as libc::c_int;
            current_block_3 = 2979737022853876585;
        }
        2 => {
            if if (*sa).activity_time.tv_sec == (*sb).activity_time.tv_sec {
                ((*sa).activity_time.tv_usec > (*sb).activity_time.tv_usec) as libc::c_int
            } else {
                ((*sa).activity_time.tv_sec > (*sb).activity_time.tv_sec) as libc::c_int
            } != 0
            {
                result = -(1i32);
                current_block_3 = 2979737022853876585;
            } else if if (*sa).activity_time.tv_sec == (*sb).activity_time.tv_sec {
                ((*sa).activity_time.tv_usec < (*sb).activity_time.tv_usec) as libc::c_int
            } else {
                ((*sa).activity_time.tv_sec < (*sb).activity_time.tv_sec) as libc::c_int
            } != 0
            {
                result = 1i32;
                current_block_3 = 2979737022853876585;
            } else {
                current_block_3 = 6916742544122075610;
            }
        }
        1 => {
            current_block_3 = 6916742544122075610;
        }
        _ => {
            current_block_3 = 2979737022853876585;
        }
    }
    match current_block_3 {
        6916742544122075610 =>
        /* FALLTHROUGH */
        {
            result = strcmp((*sa).name, (*sb).name)
        }
        _ => {}
    }
    if (*window_tree_sort).reversed != 0 {
        result = -result
    }
    return result;
}
unsafe extern "C" fn window_tree_cmp_window(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const winlink = a0 as *const *const winlink;
    let mut b: *const *const winlink = b0 as *const *const winlink;
    let mut wla: *const winlink = *a;
    let mut wlb: *const winlink = *b;
    let mut wa: *mut window = (*wla).window;
    let mut wb: *mut window = (*wlb).window;
    let mut result: libc::c_int = 0i32;
    let mut current_block_3: u64;
    match (*window_tree_sort).field {
        0 => {
            result = (*wla).idx - (*wlb).idx;
            current_block_3 = 3512920355445576850;
        }
        2 => {
            if if (*wa).activity_time.tv_sec == (*wb).activity_time.tv_sec {
                ((*wa).activity_time.tv_usec > (*wb).activity_time.tv_usec) as libc::c_int
            } else {
                ((*wa).activity_time.tv_sec > (*wb).activity_time.tv_sec) as libc::c_int
            } != 0
            {
                result = -(1i32);
                current_block_3 = 3512920355445576850;
            } else if if (*wa).activity_time.tv_sec == (*wb).activity_time.tv_sec {
                ((*wa).activity_time.tv_usec < (*wb).activity_time.tv_usec) as libc::c_int
            } else {
                ((*wa).activity_time.tv_sec < (*wb).activity_time.tv_sec) as libc::c_int
            } != 0
            {
                result = 1i32;
                current_block_3 = 3512920355445576850;
            } else {
                current_block_3 = 2202846541499471484;
            }
        }
        1 => {
            current_block_3 = 2202846541499471484;
        }
        _ => {
            current_block_3 = 3512920355445576850;
        }
    }
    match current_block_3 {
        2202846541499471484 =>
        /* FALLTHROUGH */
        {
            result = strcmp((*wa).name, (*wb).name)
        }
        _ => {}
    }
    if (*window_tree_sort).reversed != 0 {
        result = -result
    }
    return result;
}
unsafe extern "C" fn window_tree_cmp_pane(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const window_pane = a0 as *const *const window_pane;
    let mut b: *const *const window_pane = b0 as *const *const window_pane;
    let mut result: libc::c_int = 0;
    if (*window_tree_sort).field == WINDOW_TREE_BY_TIME {
        result = (**a).active_point.wrapping_sub((**b).active_point) as libc::c_int
    } else {
        /*
         * Panes don't have names, so use number order for any other
         * sort field.
         */
        result = (**a).id.wrapping_sub((**b).id) as libc::c_int
    }
    if (*window_tree_sort).reversed != 0 {
        result = -result
    }
    return result;
}
unsafe extern "C" fn window_tree_build_pane(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
    mut modedata: *mut libc::c_void,
    mut parent: *mut crate::mode_tree::mode_tree_item,
) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: u_int = 0;
    window_pane_index(wp, &mut idx);
    item = window_tree_add_item(data);
    (*item).type_0 = WINDOW_TREE_PANE;
    (*item).session = (*s).id as libc::c_int;
    (*item).winlink = (*wl).idx;
    (*item).pane = (*wp).id as libc::c_int;
    text = format_single(
        0 as *mut crate::cmd_queue::cmdq_item,
        (*data).format,
        0 as *mut client,
        s,
        wl,
        wp,
    );
    xasprintf(
        &mut name as *mut *mut libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        idx,
    );
    mode_tree_add(
        (*data).data,
        parent,
        item as *mut libc::c_void,
        wp as uint64_t,
        name,
        text,
        -(1i32),
    );
    free(text as *mut libc::c_void);
    free(name as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_filter_pane(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
    mut filter: *const libc::c_char,
) -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    if filter.is_null() {
        return 1i32;
    }
    cp = format_single(
        0 as *mut crate::cmd_queue::cmdq_item,
        filter,
        0 as *mut client,
        s,
        wl,
        wp,
    );
    result = format_true(cp);
    free(cp as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn window_tree_build_window(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut modedata: *mut libc::c_void,
    mut sort_crit: *mut mode_tree_sort_criteria,
    mut parent: *mut crate::mode_tree::mode_tree_item,
    mut filter: *const libc::c_char,
) -> libc::c_int {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut mti: *mut crate::mode_tree::mode_tree_item = 0 as *mut crate::mode_tree::mode_tree_item;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut l: *mut *mut window_pane = 0 as *mut *mut window_pane;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    let mut expanded: libc::c_int = 0;
    item = window_tree_add_item(data);
    (*item).type_0 = WINDOW_TREE_WINDOW;
    (*item).session = (*s).id as libc::c_int;
    (*item).winlink = (*wl).idx;
    (*item).pane = -(1i32);
    text = format_single(
        0 as *mut crate::cmd_queue::cmdq_item,
        (*data).format,
        0 as *mut client,
        s,
        wl,
        0 as *mut window_pane,
    );
    xasprintf(
        &mut name as *mut *mut libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wl).idx,
    );
    if (*data).type_0 == WINDOW_TREE_SESSION || (*data).type_0 == WINDOW_TREE_WINDOW {
        expanded = 0i32
    } else {
        expanded = 1i32
    }
    mti = mode_tree_add(
        (*data).data,
        parent,
        item as *mut libc::c_void,
        wl as uint64_t,
        name,
        text,
        expanded,
    );
    free(text as *mut libc::c_void);
    free(name as *mut libc::c_void);
    wp = (*(*wl).window).panes.tqh_first;
    if !wp.is_null() {
        if (*wp).entry.tqe_next.is_null() {
            if !(window_tree_filter_pane(s, wl, wp, filter) == 0) {
                return 1i32;
            }
        } else {
            l = 0 as *mut *mut window_pane;
            n = 0u32;
            wp = (*(*wl).window).panes.tqh_first;
            while !wp.is_null() {
                if !(window_tree_filter_pane(s, wl, wp, filter) == 0) {
                    l = xreallocarray(
                        l as *mut libc::c_void,
                        n.wrapping_add(1u32) as size_t,
                        ::std::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    ) as *mut *mut window_pane;
                    let fresh2 = n;
                    n = n.wrapping_add(1);
                    let ref mut fresh3 = *l.offset(fresh2 as isize);
                    *fresh3 = wp
                }
                wp = (*wp).entry.tqe_next
            }
            if !(n == 0u32) {
                window_tree_sort = sort_crit;
                qsort(
                    l as *mut libc::c_void,
                    n as size_t,
                    ::std::mem::size_of::<*mut window_pane>() as libc::c_ulong,
                    Some(
                        window_tree_cmp_pane
                            as unsafe extern "C" fn(
                                _: *const libc::c_void,
                                _: *const libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                i = 0u32;
                while i < n {
                    window_tree_build_pane(s, wl, *l.offset(i as isize), modedata, mti);
                    i = i.wrapping_add(1)
                }
                free(l as *mut libc::c_void);
                return 1i32;
            }
        }
    }
    window_tree_free_item(item);
    (*data).item_size = (*data).item_size.wrapping_sub(1);
    mode_tree_remove((*data).data, mti);
    return 0i32;
}
unsafe extern "C" fn window_tree_build_session(
    mut s: *mut session,
    mut modedata: *mut libc::c_void,
    mut sort_crit: *mut mode_tree_sort_criteria,
    mut filter: *const libc::c_char,
) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut mti: *mut crate::mode_tree::mode_tree_item = 0 as *mut crate::mode_tree::mode_tree_item;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut l: *mut *mut winlink = 0 as *mut *mut winlink;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    let mut empty: u_int = 0;
    let mut expanded: libc::c_int = 0;
    item = window_tree_add_item(data);
    (*item).type_0 = WINDOW_TREE_SESSION;
    (*item).session = (*s).id as libc::c_int;
    (*item).winlink = -(1i32);
    (*item).pane = -(1i32);
    text = format_single(
        0 as *mut crate::cmd_queue::cmdq_item,
        (*data).format,
        0 as *mut client,
        s,
        0 as *mut winlink,
        0 as *mut window_pane,
    );
    if (*data).type_0 == WINDOW_TREE_SESSION {
        expanded = 0i32
    } else {
        expanded = 1i32
    }
    mti = mode_tree_add(
        (*data).data,
        0 as *mut crate::mode_tree::mode_tree_item,
        item as *mut libc::c_void,
        s as uint64_t,
        (*s).name,
        text,
        expanded,
    );
    free(text as *mut libc::c_void);
    l = 0 as *mut *mut winlink;
    n = 0u32;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1i32));
    while !wl.is_null() {
        l = xreallocarray(
            l as *mut libc::c_void,
            n.wrapping_add(1u32) as size_t,
            ::std::mem::size_of::<*mut winlink>() as libc::c_ulong,
        ) as *mut *mut winlink;
        let fresh4 = n;
        n = n.wrapping_add(1);
        let ref mut fresh5 = *l.offset(fresh4 as isize);
        *fresh5 = wl;
        wl = winlinks_RB_NEXT(wl)
    }
    window_tree_sort = sort_crit;
    qsort(
        l as *mut libc::c_void,
        n as size_t,
        ::std::mem::size_of::<*mut winlink>() as libc::c_ulong,
        Some(
            window_tree_cmp_window
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    empty = 0u32;
    i = 0u32;
    while i < n {
        if window_tree_build_window(s, *l.offset(i as isize), modedata, sort_crit, mti, filter) == 0
        {
            empty = empty.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if empty == n {
        window_tree_free_item(item);
        (*data).item_size = (*data).item_size.wrapping_sub(1);
        mode_tree_remove((*data).data, mti);
    }
    free(l as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_build(
    mut modedata: *mut libc::c_void,
    mut sort_crit: *mut mode_tree_sort_criteria,
    mut tag: *mut uint64_t,
    mut filter: *const libc::c_char,
) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut s: *mut session = 0 as *mut session;
    let mut l: *mut *mut session = 0 as *mut *mut session;
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut current: *mut session_group = 0 as *mut session_group;
    let mut n: u_int = 0;
    let mut i: u_int = 0;
    current = session_group_contains((*data).fs.s);
    i = 0u32;
    while i < (*data).item_size {
        window_tree_free_item(*(*data).item_list.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*data).item_list as *mut libc::c_void);
    (*data).item_list = 0 as *mut *mut window_tree_itemdata;
    (*data).item_size = 0u32;
    l = 0 as *mut *mut session;
    n = 0u32;
    let mut current_block_11: u64;
    s = sessions_RB_MINMAX(&mut sessions, -(1i32));
    while !s.is_null() {
        if (*data).squash_groups != 0 && {
            sg = session_group_contains(s);
            !sg.is_null()
        } {
            if sg == current && s != (*data).fs.s || sg != current && s != (*sg).sessions.tqh_first
            {
                current_block_11 = 13536709405535804910;
            } else {
                current_block_11 = 8236137900636309791;
            }
        } else {
            current_block_11 = 8236137900636309791;
        }
        match current_block_11 {
            8236137900636309791 => {
                l = xreallocarray(
                    l as *mut libc::c_void,
                    n.wrapping_add(1u32) as size_t,
                    ::std::mem::size_of::<*mut session>() as libc::c_ulong,
                ) as *mut *mut session;
                let fresh6 = n;
                n = n.wrapping_add(1);
                let ref mut fresh7 = *l.offset(fresh6 as isize);
                *fresh7 = s
            }
            _ => {}
        }
        s = sessions_RB_NEXT(s)
    }
    window_tree_sort = sort_crit;
    qsort(
        l as *mut libc::c_void,
        n as size_t,
        ::std::mem::size_of::<*mut session>() as libc::c_ulong,
        Some(
            window_tree_cmp_session
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0u32;
    while i < n {
        window_tree_build_session(*l.offset(i as isize), modedata, sort_crit, filter);
        i = i.wrapping_add(1)
    }
    free(l as *mut libc::c_void);
    match (*data).type_0 {
        1 => *tag = (*data).fs.s as uint64_t,
        2 => *tag = (*data).fs.wl as uint64_t,
        3 => {
            if window_count_panes((*(*data).fs.wl).window) == 1u32 {
                *tag = (*data).fs.wl as uint64_t
            } else {
                *tag = (*data).fs.wp as uint64_t
            }
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn window_tree_draw_label(
    mut ctx: *mut screen_write_ctx,
    mut px: u_int,
    mut py: u_int,
    mut sx: u_int,
    mut sy: u_int,
    mut gc: *const GridCell,
    mut label: *const libc::c_char,
) {
    let mut len: size_t = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    len = strlen(label);
    if sx == 0u32 || sy == 1u32 || len > sx as libc::c_ulong {
        return;
    }
    ox = (sx as libc::c_ulong)
        .wrapping_sub(len)
        .wrapping_add(1u64)
        .wrapping_div(2u64) as u_int;
    oy = sy.wrapping_add(1u32).wrapping_div(2u32);
    if ox > 1u32
        && (ox as libc::c_ulong).wrapping_add(len) < sx.wrapping_sub(1u32) as libc::c_ulong
        && sy >= 3u32
    {
        screen_write_cursormove(
            ctx,
            px.wrapping_add(ox).wrapping_sub(1u32) as libc::c_int,
            py.wrapping_add(oy).wrapping_sub(1u32) as libc::c_int,
            0i32,
        );
        screen_write_box(ctx, len.wrapping_add(2u64) as u_int, 3u32);
    }
    screen_write_cursormove(
        ctx,
        px.wrapping_add(ox) as libc::c_int,
        py.wrapping_add(oy) as libc::c_int,
        0i32,
    );
    screen_write_puts(
        ctx,
        gc,
        b"%s\x00" as *const u8 as *const libc::c_char,
        label,
    );
}
unsafe extern "C" fn window_tree_draw_session(
    mut data: *mut window_tree_modedata,
    mut s: *mut session,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut oo: *mut crate::options::options = (*s).options;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut w: *mut window = 0 as *mut window;
    let mut cx: u_int = (*(*ctx).s).cx;
    let mut cy: u_int = (*(*ctx).s).cy;
    let mut loop_0: u_int = 0;
    let mut total: u_int = 0;
    let mut visible: u_int = 0;
    let mut each: u_int = 0;
    let mut width: u_int = 0;
    let mut offset: u_int = 0;
    let mut current: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    let mut remaining: u_int = 0;
    let mut i: u_int = 0;
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
    let mut colour: libc::c_int = 0;
    let mut active_colour: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    total = winlink_count(&mut (*s).windows);
    memcpy(
        &mut gc as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    colour = options_get_number(
        oo,
        b"display-panes-colour\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    active_colour = options_get_number(
        oo,
        b"display-panes-active-colour\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if sx.wrapping_div(total) < 24u32 {
        visible = sx.wrapping_div(24u32);
        if visible == 0u32 {
            visible = 1u32
        }
    } else {
        visible = total
    }
    current = 0u32;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1i32));
    while !wl.is_null() {
        if wl == (*s).curw {
            break;
        }
        current = current.wrapping_add(1);
        wl = winlinks_RB_NEXT(wl)
    }
    if current < visible {
        start = 0u32;
        end = visible
    } else if current >= total.wrapping_sub(visible) {
        start = total.wrapping_sub(visible);
        end = total
    } else {
        start = current.wrapping_sub(visible.wrapping_div(2u32));
        end = start.wrapping_add(visible)
    }
    if (*data).offset < -(start as libc::c_int) {
        (*data).offset = -(start as libc::c_int)
    }
    if (*data).offset > total.wrapping_sub(end) as libc::c_int {
        (*data).offset = total.wrapping_sub(end) as libc::c_int
    }
    start = (start).wrapping_add((*data).offset as libc::c_uint);
    end = (end).wrapping_add((*data).offset as libc::c_uint);
    left = (start != 0u32) as libc::c_int;
    right = (end != total) as libc::c_int;
    if left != 0 && right != 0 && sx <= 6u32 || (left != 0 || right != 0) && sx <= 3u32 {
        right = 0i32;
        left = right
    }
    if left != 0 && right != 0 {
        each = sx.wrapping_sub(6u32).wrapping_div(visible);
        remaining = sx
            .wrapping_sub(6u32)
            .wrapping_sub(visible.wrapping_mul(each))
    } else if left != 0 || right != 0 {
        each = sx.wrapping_sub(3u32).wrapping_div(visible);
        remaining = sx
            .wrapping_sub(3u32)
            .wrapping_sub(visible.wrapping_mul(each))
    } else {
        each = sx.wrapping_div(visible);
        remaining = sx.wrapping_sub(visible.wrapping_mul(each))
    }
    if each == 0u32 {
        return;
    }
    if left != 0 {
        (*data).left = cx.wrapping_add(2u32) as libc::c_int;
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(2u32) as libc::c_int,
            cy as libc::c_int,
            0i32,
        );
        screen_write_vline(ctx, sy, 0i32, 0i32);
        screen_write_cursormove(
            ctx,
            cx as libc::c_int,
            cy.wrapping_add(sy.wrapping_div(2u32)) as libc::c_int,
            0i32,
        );
        screen_write_puts(
            ctx,
            &grid_default_cell as *const GridCell,
            b"<\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).left = -(1i32)
    }
    if right != 0 {
        (*data).right = cx.wrapping_add(sx).wrapping_sub(3u32) as libc::c_int;
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(sx).wrapping_sub(3u32) as libc::c_int,
            cy as libc::c_int,
            0i32,
        );
        screen_write_vline(ctx, sy, 0i32, 0i32);
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(sx).wrapping_sub(1u32) as libc::c_int,
            cy.wrapping_add(sy.wrapping_div(2u32)) as libc::c_int,
            0i32,
        );
        screen_write_puts(
            ctx,
            &grid_default_cell as *const GridCell,
            b">\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).right = -(1i32)
    }
    (*data).start = start;
    (*data).end = end;
    (*data).each = each;
    loop_0 = 0u32;
    i = loop_0;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1i32));
    while !wl.is_null() {
        if loop_0 == end {
            break;
        }
        if loop_0 < start {
            loop_0 = loop_0.wrapping_add(1)
        } else {
            w = (*wl).window;
            if wl == (*s).curw {
                gc.fg = active_colour
            } else {
                gc.fg = colour
            }
            if left != 0 {
                offset = (3u32).wrapping_add(i.wrapping_mul(each))
            } else {
                offset = i.wrapping_mul(each)
            }
            if loop_0 == end.wrapping_sub(1u32) {
                width = each.wrapping_add(remaining)
            } else {
                width = each.wrapping_sub(1u32)
            }
            screen_write_cursormove(
                ctx,
                cx.wrapping_add(offset) as libc::c_int,
                cy as libc::c_int,
                0i32,
            );
            screen_write_preview(ctx, &mut (*(*w).active).base, width, sy);
            xasprintf(
                &mut label as *mut *mut libc::c_char,
                b" %u:%s \x00" as *const u8 as *const libc::c_char,
                (*wl).idx,
                (*w).name,
            );
            if strlen(label) > width as libc::c_ulong {
                xasprintf(
                    &mut label as *mut *mut libc::c_char,
                    b" %u \x00" as *const u8 as *const libc::c_char,
                    (*wl).idx,
                );
            }
            window_tree_draw_label(ctx, cx.wrapping_add(offset), cy, width, sy, &mut gc, label);
            free(label as *mut libc::c_void);
            if loop_0 != end.wrapping_sub(1u32) {
                screen_write_cursormove(
                    ctx,
                    cx.wrapping_add(offset).wrapping_add(width) as libc::c_int,
                    cy as libc::c_int,
                    0i32,
                );
                screen_write_vline(ctx, sy, 0i32, 0i32);
            }
            loop_0 = loop_0.wrapping_add(1);
            i = i.wrapping_add(1)
        }
        wl = winlinks_RB_NEXT(wl)
    }
}
unsafe extern "C" fn window_tree_draw_window(
    mut data: *mut window_tree_modedata,
    mut s: *mut session,
    mut w: *mut window,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut oo: *mut crate::options::options = (*s).options;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut cx: u_int = (*(*ctx).s).cx;
    let mut cy: u_int = (*(*ctx).s).cy;
    let mut loop_0: u_int = 0;
    let mut total: u_int = 0;
    let mut visible: u_int = 0;
    let mut each: u_int = 0;
    let mut width: u_int = 0;
    let mut offset: u_int = 0;
    let mut current: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    let mut remaining: u_int = 0;
    let mut i: u_int = 0;
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
    let mut colour: libc::c_int = 0;
    let mut active_colour: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut pane_idx: libc::c_int = 0;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    total = window_count_panes(w);
    memcpy(
        &mut gc as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    colour = options_get_number(
        oo,
        b"display-panes-colour\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    active_colour = options_get_number(
        oo,
        b"display-panes-active-colour\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if sx.wrapping_div(total) < 24u32 {
        visible = sx.wrapping_div(24u32);
        if visible == 0u32 {
            visible = 1u32
        }
    } else {
        visible = total
    }
    current = 0u32;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if wp == (*w).active {
            break;
        }
        current = current.wrapping_add(1);
        wp = (*wp).entry.tqe_next
    }
    if current < visible {
        start = 0u32;
        end = visible
    } else if current >= total.wrapping_sub(visible) {
        start = total.wrapping_sub(visible);
        end = total
    } else {
        start = current.wrapping_sub(visible.wrapping_div(2u32));
        end = start.wrapping_add(visible)
    }
    if (*data).offset < -(start as libc::c_int) {
        (*data).offset = -(start as libc::c_int)
    }
    if (*data).offset > total.wrapping_sub(end) as libc::c_int {
        (*data).offset = total.wrapping_sub(end) as libc::c_int
    }
    start = (start).wrapping_add((*data).offset as libc::c_uint);
    end = (end).wrapping_add((*data).offset as libc::c_uint);
    left = (start != 0u32) as libc::c_int;
    right = (end != total) as libc::c_int;
    if left != 0 && right != 0 && sx <= 6u32 || (left != 0 || right != 0) && sx <= 3u32 {
        right = 0i32;
        left = right
    }
    if left != 0 && right != 0 {
        each = sx.wrapping_sub(6u32).wrapping_div(visible);
        remaining = sx
            .wrapping_sub(6u32)
            .wrapping_sub(visible.wrapping_mul(each))
    } else if left != 0 || right != 0 {
        each = sx.wrapping_sub(3u32).wrapping_div(visible);
        remaining = sx
            .wrapping_sub(3u32)
            .wrapping_sub(visible.wrapping_mul(each))
    } else {
        each = sx.wrapping_div(visible);
        remaining = sx.wrapping_sub(visible.wrapping_mul(each))
    }
    if each == 0u32 {
        return;
    }
    if left != 0 {
        (*data).left = cx.wrapping_add(2u32) as libc::c_int;
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(2u32) as libc::c_int,
            cy as libc::c_int,
            0i32,
        );
        screen_write_vline(ctx, sy, 0i32, 0i32);
        screen_write_cursormove(
            ctx,
            cx as libc::c_int,
            cy.wrapping_add(sy.wrapping_div(2u32)) as libc::c_int,
            0i32,
        );
        screen_write_puts(
            ctx,
            &grid_default_cell as *const GridCell,
            b"<\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).left = -(1i32)
    }
    if right != 0 {
        (*data).right = cx.wrapping_add(sx).wrapping_sub(3u32) as libc::c_int;
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(sx).wrapping_sub(3u32) as libc::c_int,
            cy as libc::c_int,
            0i32,
        );
        screen_write_vline(ctx, sy, 0i32, 0i32);
        screen_write_cursormove(
            ctx,
            cx.wrapping_add(sx).wrapping_sub(1u32) as libc::c_int,
            cy.wrapping_add(sy.wrapping_div(2u32)) as libc::c_int,
            0i32,
        );
        screen_write_puts(
            ctx,
            &grid_default_cell as *const GridCell,
            b">\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        (*data).right = -(1i32)
    }
    (*data).start = start;
    (*data).end = end;
    (*data).each = each;
    loop_0 = 0u32;
    i = loop_0;
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        if loop_0 == end {
            break;
        }
        if loop_0 < start {
            loop_0 = loop_0.wrapping_add(1)
        } else {
            if wp == (*w).active {
                gc.fg = active_colour
            } else {
                gc.fg = colour
            }
            if left != 0 {
                offset = (3u32).wrapping_add(i.wrapping_mul(each))
            } else {
                offset = i.wrapping_mul(each)
            }
            if loop_0 == end.wrapping_sub(1u32) {
                width = each.wrapping_add(remaining)
            } else {
                width = each.wrapping_sub(1u32)
            }
            screen_write_cursormove(
                ctx,
                cx.wrapping_add(offset) as libc::c_int,
                cy as libc::c_int,
                0i32,
            );
            screen_write_preview(ctx, &mut (*wp).base, width, sy);
            if window_pane_index(wp, &mut pane_idx as *mut libc::c_int as *mut u_int) != 0i32 {
                pane_idx = loop_0 as libc::c_int
            }
            xasprintf(
                &mut label as *mut *mut libc::c_char,
                b" %u \x00" as *const u8 as *const libc::c_char,
                pane_idx,
            );
            window_tree_draw_label(ctx, cx.wrapping_add(offset), cy, each, sy, &mut gc, label);
            free(label as *mut libc::c_void);
            if loop_0 != end.wrapping_sub(1u32) {
                screen_write_cursormove(
                    ctx,
                    cx.wrapping_add(offset).wrapping_add(width) as libc::c_int,
                    cy as libc::c_int,
                    0i32,
                );
                screen_write_vline(ctx, sy, 0i32, 0i32);
            }
            loop_0 = loop_0.wrapping_add(1);
            i = i.wrapping_add(1)
        }
        wp = (*wp).entry.tqe_next
    }
}
unsafe extern "C" fn window_tree_draw(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut item: *mut window_tree_itemdata = itemdata as *mut window_tree_itemdata;
    let mut sp: *mut session = 0 as *mut session;
    let mut wlp: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    window_tree_pull_item(item, &mut sp, &mut wlp, &mut wp);
    if wp.is_null() {
        return;
    }
    match (*item).type_0 {
        1 => {
            window_tree_draw_session(modedata as *mut window_tree_modedata, sp, ctx, sx, sy);
        }
        2 => {
            window_tree_draw_window(
                modedata as *mut window_tree_modedata,
                sp,
                (*wlp).window,
                ctx,
                sx,
                sy,
            );
        }
        3 => {
            screen_write_preview(ctx, &mut (*wp).base, sx, sy);
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn window_tree_search(
    mut _modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut ss: *const libc::c_char,
) -> libc::c_int {
    let mut item: *mut window_tree_itemdata = itemdata as *mut window_tree_itemdata;
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: libc::c_int = 0;
    window_tree_pull_item(item, &mut s, &mut wl, &mut wp);
    match (*item).type_0 {
        0 => return 0i32,
        1 => {
            if s.is_null() {
                return 0i32;
            }
            return (strstr((*s).name, ss) != 0 as *mut libc::c_char) as libc::c_int;
        }
        2 => {
            if s.is_null() || wl.is_null() {
                return 0i32;
            }
            return (strstr((*(*wl).window).name, ss) != 0 as *mut libc::c_char) as libc::c_int;
        }
        3 => {
            if !(s.is_null() || wl.is_null() || wp.is_null()) {
                cmd = osdep_get_name((*wp).fd, (*wp).tty.as_mut_ptr());
                if cmd.is_null() || *cmd as libc::c_int == '\u{0}' as i32 {
                    return 0i32;
                }
                retval = (strstr(cmd, ss) != 0 as *mut libc::c_char) as libc::c_int;
                free(cmd as *mut libc::c_void);
                return retval;
            }
        }
        _ => {}
    }
    return 0i32;
}
unsafe extern "C" fn window_tree_menu(
    mut modedata: *mut libc::c_void,
    mut c: *mut client,
    mut key: key_code,
) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut wp: *mut window_pane = (*data).wp;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    wme = (*wp).modes.tqh_first;
    if wme.is_null() || (*wme).data != modedata {
        return;
    }
    window_tree_key(
        wme,
        c,
        0 as *mut session,
        0 as *mut winlink,
        key,
        0 as *mut mouse_event,
    );
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
unsafe extern "C" fn window_tree_init(
    mut wme: *mut window_mode_entry,
    mut fs: *mut cmd_find_state,
    mut args: *mut args,
) -> *mut Screen {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_tree_modedata = 0 as *mut window_tree_modedata;
    let mut s: *mut Screen = 0 as *mut Screen;
    data = xcalloc(
        1u64,
        ::std::mem::size_of::<window_tree_modedata>() as libc::c_ulong,
    ) as *mut window_tree_modedata;
    (*wme).data = data as *mut libc::c_void;
    (*data).wp = wp;
    (*data).references = 1i32;
    if args_has(args, 's' as u_char) != 0 {
        (*data).type_0 = WINDOW_TREE_SESSION
    } else if args_has(args, 'w' as u_char) != 0 {
        (*data).type_0 = WINDOW_TREE_WINDOW
    } else {
        (*data).type_0 = WINDOW_TREE_PANE
    }
    memcpy(
        &mut (*data).fs as *mut cmd_find_state as *mut libc::c_void,
        fs as *const libc::c_void,
        ::std::mem::size_of::<cmd_find_state>() as libc::c_ulong,
    );
    if args.is_null() || args_has(args, 'F' as u_char) == 0 {
        (*data).format =
            xstrdup(b"#{?pane_format,#{?pane_marked,#[reverse],}#{pane_current_command}#{?pane_active,*,}#{?pane_marked,M,}#{?#{&&:#{pane_title},#{!=:#{pane_title},#{host_short}}},: \"#{pane_title}\",},#{?window_format,#{?window_marked_flag,#[reverse],}#{window_name}#{window_flags}#{?#{&&:#{==:#{window_panes},1},#{&&:#{pane_title},#{!=:#{pane_title},#{host_short}}}},: \"#{pane_title}\",},#{session_windows} windows#{?session_grouped, (group #{session_group}: #{session_group_list}),}#{?session_attached, (attached),}}}\x00"
                        as *const u8 as *const libc::c_char)
    } else {
        (*data).format = xstrdup(args_get(args, 'F' as u_char))
    }
    if args.is_null() || (*args).argc == 0i32 {
        (*data).command =
            xstrdup(b"switch-client -Zt \'%%\'\x00" as *const u8 as *const libc::c_char)
    } else {
        (*data).command = xstrdup(*(*args).argv.offset(0isize))
    }
    (*data).squash_groups = (args_has(args, 'G' as u_char) == 0) as libc::c_int;
    (*data).data = mode_tree_start(
        wp,
        args,
        Some(
            window_tree_build
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut mode_tree_sort_criteria,
                    _: *mut uint64_t,
                    _: *const libc::c_char,
                ) -> (),
        ),
        Some(
            window_tree_draw
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                    _: *mut screen_write_ctx,
                    _: u_int,
                    _: u_int,
                ) -> (),
        ),
        Some(
            window_tree_search
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            window_tree_menu
                as unsafe extern "C" fn(_: *mut libc::c_void, _: *mut client, _: key_code) -> (),
        ),
        None,
        data as *mut libc::c_void,
        window_tree_menu_items.as_ptr(),
        window_tree_sort_list.as_mut_ptr(),
        (::std::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as u_int,
        &mut s,
    );
    mode_tree_zoom((*data).data, args);
    mode_tree_build((*data).data);
    mode_tree_draw((*data).data);
    (*data).type_0 = WINDOW_TREE_NONE;
    return s;
}
unsafe extern "C" fn window_tree_destroy(mut data: *mut window_tree_modedata) {
    let mut i: u_int = 0;
    (*data).references -= 1;
    if (*data).references != 0i32 {
        return;
    }
    i = 0u32;
    while i < (*data).item_size {
        window_tree_free_item(*(*data).item_list.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*data).item_list as *mut libc::c_void);
    free((*data).format as *mut libc::c_void);
    free((*data).command as *mut libc::c_void);
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_free(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_tree_modedata = (*wme).data as *mut window_tree_modedata;
    if data.is_null() {
        return;
    }
    (*data).dead = 1i32;
    mode_tree_free((*data).data);
    window_tree_destroy(data);
}
unsafe extern "C" fn window_tree_resize(
    mut wme: *mut window_mode_entry,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut data: *mut window_tree_modedata = (*wme).data as *mut window_tree_modedata;
    mode_tree_resize((*data).data, sx, sy);
}
unsafe extern "C" fn window_tree_get_target(
    mut item: *mut window_tree_itemdata,
    mut fs: *mut cmd_find_state,
) -> *mut libc::c_char {
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    window_tree_pull_item(item, &mut s, &mut wl, &mut wp);
    target = 0 as *mut libc::c_char;
    match (*item).type_0 {
        1 => {
            if !s.is_null() {
                xasprintf(
                    &mut target as *mut *mut libc::c_char,
                    b"=%s:\x00" as *const u8 as *const libc::c_char,
                    (*s).name,
                );
            }
        }
        2 => {
            if !(s.is_null() || wl.is_null()) {
                xasprintf(
                    &mut target as *mut *mut libc::c_char,
                    b"=%s:%u.\x00" as *const u8 as *const libc::c_char,
                    (*s).name,
                    (*wl).idx,
                );
            }
        }
        3 => {
            if !(s.is_null() || wl.is_null() || wp.is_null()) {
                xasprintf(
                    &mut target as *mut *mut libc::c_char,
                    b"=%s:%u.%%%u\x00" as *const u8 as *const libc::c_char,
                    (*s).name,
                    (*wl).idx,
                    (*wp).id,
                );
            }
        }
        0 | _ => {}
    }
    if target.is_null() {
        cmd_find_clear_state(fs, 0i32);
    } else {
        cmd_find_from_winlink_pane(fs, wl, wp, 0i32);
    }
    return target;
}
unsafe extern "C" fn window_tree_command_each(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut c: *mut client,
    mut _key: key_code,
) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = itemdata as *mut window_tree_itemdata;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    name = window_tree_get_target(item, &mut fs);
    if !name.is_null() {
        mode_tree_run_command(c, &mut fs, (*data).entered, name);
    }
    free(name as *mut libc::c_void);
}
unsafe extern "C" fn window_tree_command_done(
    mut _item: *mut crate::cmd_queue::cmdq_item,
    mut modedata: *mut libc::c_void,
) -> cmd_retval {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    if (*data).dead == 0 {
        mode_tree_build((*data).data);
        mode_tree_draw((*data).data);
        (*(*data).wp).flags |= 0x1i32
    }
    window_tree_destroy(data);
    return CMD_RETURN_NORMAL;
}
unsafe extern "C" fn window_tree_command_callback(
    mut c: *mut client,
    mut modedata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut _done: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 || (*data).dead != 0 {
        return 0i32;
    }
    (*data).entered = s;
    mode_tree_each_tagged(
        (*data).data,
        Some(
            window_tree_command_each
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                    _: *mut client,
                    _: key_code,
                ) -> (),
        ),
        c,
        0xff000000000u64,
        1i32,
    );
    (*data).entered = 0 as *const libc::c_char;
    (*data).references += 1;
    cmdq_append(
        c,
        cmdq_get_callback1(
            b"window_tree_command_done\x00" as *const u8 as *const libc::c_char,
            Some(
                window_tree_command_done
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd_queue::cmdq_item,
                        _: *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            data as *mut libc::c_void,
        ),
    );
    return 0i32;
}
unsafe extern "C" fn window_tree_command_free(mut modedata: *mut libc::c_void) {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    window_tree_destroy(data);
}
unsafe extern "C" fn window_tree_kill_each(
    mut _modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut _c: *mut client,
    mut _key: key_code,
) {
    let mut item: *mut window_tree_itemdata = itemdata as *mut window_tree_itemdata;
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    window_tree_pull_item(item, &mut s, &mut wl, &mut wp);
    match (*item).type_0 {
        1 => {
            if !s.is_null() {
                server_destroy_session(s);
                session_destroy(
                    s,
                    1i32,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"window_tree_kill_each\x00",
                    ))
                    .as_ptr(),
                );
            }
        }
        2 => {
            if !wl.is_null() {
                server_kill_window((*wl).window, 0i32);
            }
        }
        3 => {
            if !wp.is_null() {
                server_kill_pane(wp);
            }
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn window_tree_kill_current_callback(
    mut c: *mut client,
    mut modedata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut _done: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut mtd: *mut crate::mode_tree::mode_tree_data = (*data).data;
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 || (*data).dead != 0 {
        return 0i32;
    }
    if ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<u_char>() as libc::c_ulong > 1u64 {
            if 0 != 0 {
                let mut __c: libc::c_int = *s.offset(0isize) as u_char as libc::c_int;
                __res = if __c < -(128i32) || __c > 255i32 {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                }
            } else {
                __res = tolower(*s.offset(0isize) as u_char as libc::c_int)
            }
        } else {
            __res = *(*__ctype_tolower_loc())
                .offset(*s.offset(0isize) as u_char as libc::c_int as isize)
        }
        __res
    }) != 'y' as i32
        || *s.offset(1isize) as libc::c_int != '\u{0}' as i32
    {
        return 0i32;
    }
    window_tree_kill_each(
        data as *mut libc::c_void,
        mode_tree_get_current(mtd),
        c,
        0xff000000000u64,
    );
    server_renumber_all();
    (*data).references += 1;
    cmdq_append(
        c,
        cmdq_get_callback1(
            b"window_tree_command_done\x00" as *const u8 as *const libc::c_char,
            Some(
                window_tree_command_done
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd_queue::cmdq_item,
                        _: *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            data as *mut libc::c_void,
        ),
    );
    return 0i32;
}
unsafe extern "C" fn window_tree_kill_tagged_callback(
    mut c: *mut client,
    mut modedata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut _done: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_tree_modedata = modedata as *mut window_tree_modedata;
    let mut mtd: *mut crate::mode_tree::mode_tree_data = (*data).data;
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 || (*data).dead != 0 {
        return 0i32;
    }
    if ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<u_char>() as libc::c_ulong > 1u64 {
            if 0 != 0 {
                let mut __c: libc::c_int = *s.offset(0isize) as u_char as libc::c_int;
                __res = if __c < -(128i32) || __c > 255i32 {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                }
            } else {
                __res = tolower(*s.offset(0isize) as u_char as libc::c_int)
            }
        } else {
            __res = *(*__ctype_tolower_loc())
                .offset(*s.offset(0isize) as u_char as libc::c_int as isize)
        }
        __res
    }) != 'y' as i32
        || *s.offset(1isize) as libc::c_int != '\u{0}' as i32
    {
        return 0i32;
    }
    mode_tree_each_tagged(
        mtd,
        Some(
            window_tree_kill_each
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                    _: *mut client,
                    _: key_code,
                ) -> (),
        ),
        c,
        0xff000000000u64,
        1i32,
    );
    server_renumber_all();
    (*data).references += 1;
    cmdq_append(
        c,
        cmdq_get_callback1(
            b"window_tree_command_done\x00" as *const u8 as *const libc::c_char,
            Some(
                window_tree_command_done
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd_queue::cmdq_item,
                        _: *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            data as *mut libc::c_void,
        ),
    );
    return 0i32;
}
unsafe extern "C" fn window_tree_mouse(
    mut data: *mut window_tree_modedata,
    mut key: key_code,
    mut x: u_int,
    mut item: *mut window_tree_itemdata,
) -> key_code {
    let mut s: *mut session = 0 as *mut session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut loop_0: u_int = 0;
    if key != key_code_code::MOUSEDOWN1_PANE {
        return 0xff000000000u64;
    }
    if (*data).left != -(1i32) && x <= (*data).left as u_int {
        return '<' as i32 as key_code;
    }
    if (*data).right != -(1i32) && x >= (*data).right as u_int {
        return '>' as i32 as key_code;
    }
    if (*data).left != -(1i32) {
        x = (x).wrapping_sub((*data).left as libc::c_uint)
    } else if x != 0u32 {
        x = x.wrapping_sub(1)
    }
    if x == 0u32 || (*data).end == 0u32 {
        x = 0u32
    } else {
        x = x.wrapping_div((*data).each);
        if (*data).start.wrapping_add(x) >= (*data).end {
            x = (*data).end.wrapping_sub(1u32)
        }
    }
    window_tree_pull_item(item, &mut s, &mut wl, &mut wp);
    if (*item).type_0 == WINDOW_TREE_SESSION {
        if s.is_null() {
            return 0xff000000000u64;
        }
        mode_tree_expand_current((*data).data);
        loop_0 = 0u32;
        wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1i32));
        while !wl.is_null() {
            if loop_0 == (*data).start.wrapping_add(x) {
                break;
            }
            loop_0 = loop_0.wrapping_add(1);
            wl = winlinks_RB_NEXT(wl)
        }
        if !wl.is_null() {
            mode_tree_set_current((*data).data, wl as uint64_t);
        }
        return '\r' as i32 as key_code;
    }
    if (*item).type_0 == WINDOW_TREE_WINDOW {
        if wl.is_null() {
            return 0xff000000000u64;
        }
        mode_tree_expand_current((*data).data);
        loop_0 = 0u32;
        wp = (*(*wl).window).panes.tqh_first;
        while !wp.is_null() {
            if loop_0 == (*data).start.wrapping_add(x) {
                break;
            }
            loop_0 = loop_0.wrapping_add(1);
            wp = (*wp).entry.tqe_next
        }
        if !wp.is_null() {
            mode_tree_set_current((*data).data, wp as uint64_t);
        }
        return '\r' as i32 as key_code;
    }
    return 0xff000000000u64;
}
unsafe extern "C" fn window_tree_key(
    mut wme: *mut window_mode_entry,
    mut c: *mut client,
    mut _s: *mut session,
    mut _wl: *mut winlink,
    mut key: key_code,
    mut m: *mut mouse_event,
) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_tree_modedata = (*wme).data as *mut window_tree_modedata;
    let mut item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut new_item: *mut window_tree_itemdata = 0 as *mut window_tree_itemdata;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    let mut fsp: *mut cmd_find_state = &mut (*data).fs;
    let mut finished: libc::c_int = 0;
    let mut tagged: u_int = 0;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut idx: u_int = 0;
    let mut ns: *mut session = 0 as *mut session;
    let mut nwl: *mut winlink = 0 as *mut winlink;
    let mut nwp: *mut window_pane = 0 as *mut window_pane;
    item = mode_tree_get_current((*data).data) as *mut window_tree_itemdata;
    finished = mode_tree_key((*data).data, c, &mut key, m, &mut x, &mut y);
    new_item = mode_tree_get_current((*data).data) as *mut window_tree_itemdata;
    if item != new_item {
        item = new_item;
        (*data).offset = 0i32
    }
    if key & 0xfffffffffffu64 >= key_code_code::MOUSE
        && (key & 0xfffffffffffu64) < key_code_code::BSPACE
        && !m.is_null()
    {
        key = window_tree_mouse(data, key, x, item)
    }
    match key {
        60 => (*data).offset -= 1,
        62 => (*data).offset += 1,
        72 => {
            mode_tree_expand((*data).data, (*fsp).s as uint64_t);
            mode_tree_expand((*data).data, (*fsp).wl as uint64_t);
            if mode_tree_set_current((*data).data, (*wme).wp as uint64_t) == 0 {
                mode_tree_set_current((*data).data, (*fsp).wl as uint64_t);
            }
        }
        109 => {
            window_tree_pull_item(item, &mut ns, &mut nwl, &mut nwp);
            server_set_marked(ns, nwl, nwp);
            mode_tree_build((*data).data);
        }
        77 => {
            server_clear_marked();
            mode_tree_build((*data).data);
        }
        120 => {
            window_tree_pull_item(item, &mut ns, &mut nwl, &mut nwp);
            match (*item).type_0 {
                1 => {
                    if !ns.is_null() {
                        xasprintf(
                            &mut prompt as *mut *mut libc::c_char,
                            b"Kill session %s? \x00" as *const u8 as *const libc::c_char,
                            (*ns).name,
                        );
                    }
                }
                2 => {
                    if !nwl.is_null() {
                        xasprintf(
                            &mut prompt as *mut *mut libc::c_char,
                            b"Kill window %u? \x00" as *const u8 as *const libc::c_char,
                            (*nwl).idx,
                        );
                    }
                }
                3 => {
                    if !(nwp.is_null() || window_pane_index(nwp, &mut idx) != 0i32) {
                        xasprintf(
                            &mut prompt as *mut *mut libc::c_char,
                            b"Kill pane %u? \x00" as *const u8 as *const libc::c_char,
                            idx,
                        );
                    }
                }
                0 | _ => {}
            }
            if !prompt.is_null() {
                (*data).references += 1;
                status_prompt_set(
                    c,
                    0 as *mut cmd_find_state,
                    prompt,
                    b"\x00" as *const u8 as *const libc::c_char,
                    Some(
                        window_tree_kill_current_callback
                            as unsafe extern "C" fn(
                                _: *mut client,
                                _: *mut libc::c_void,
                                _: *const libc::c_char,
                                _: libc::c_int,
                            ) -> libc::c_int,
                    ),
                    Some(
                        window_tree_command_free
                            as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
                    ),
                    data as *mut libc::c_void,
                    0x1i32 | 0x8i32,
                );
                free(prompt as *mut libc::c_void);
            }
        }
        88 => {
            tagged = mode_tree_count_tagged((*data).data);
            if !(tagged == 0u32) {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"Kill %u tagged? \x00" as *const u8 as *const libc::c_char,
                    tagged,
                );
                (*data).references += 1;
                status_prompt_set(
                    c,
                    0 as *mut cmd_find_state,
                    prompt,
                    b"\x00" as *const u8 as *const libc::c_char,
                    Some(
                        window_tree_kill_tagged_callback
                            as unsafe extern "C" fn(
                                _: *mut client,
                                _: *mut libc::c_void,
                                _: *const libc::c_char,
                                _: libc::c_int,
                            ) -> libc::c_int,
                    ),
                    Some(
                        window_tree_command_free
                            as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
                    ),
                    data as *mut libc::c_void,
                    0x1i32 | 0x8i32,
                );
                free(prompt as *mut libc::c_void);
            }
        }
        58 => {
            tagged = mode_tree_count_tagged((*data).data);
            if tagged != 0u32 {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"(%u tagged) \x00" as *const u8 as *const libc::c_char,
                    tagged,
                );
            } else {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"(current) \x00" as *const u8 as *const libc::c_char,
                );
            }
            (*data).references += 1;
            status_prompt_set(
                c,
                0 as *mut cmd_find_state,
                prompt,
                b"\x00" as *const u8 as *const libc::c_char,
                Some(
                    window_tree_command_callback
                        as unsafe extern "C" fn(
                            _: *mut client,
                            _: *mut libc::c_void,
                            _: *const libc::c_char,
                            _: libc::c_int,
                        ) -> libc::c_int,
                ),
                Some(window_tree_command_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
                data as *mut libc::c_void,
                0x8i32,
            );
            free(prompt as *mut libc::c_void);
        }
        13 => {
            name = window_tree_get_target(item, &mut fs);
            if !name.is_null() {
                mode_tree_run_command(c, 0 as *mut cmd_find_state, (*data).command, name);
            }
            finished = 1i32;
            free(name as *mut libc::c_void);
        }
        _ => {}
    }
    if finished != 0 {
        window_pane_reset_mode(wp);
    } else {
        mode_tree_draw((*data).data);
        (*wp).flags |= 0x1i32
    };
}
