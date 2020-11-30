use crate::{
    menu::Item as MenuItem,
    msg::{code as msgtype_code, Msgtype},
    screen::Screen,
};
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
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
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn server_client_detach(_: *mut client, _: Msgtype);
    #[no_mangle]
    fn status_at_line(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn status_line_size(_: *mut client) -> u_int;
    #[no_mangle]
    fn screen_write_fast_copy(
        _: *mut screen_write_ctx,
        _: *mut crate::screen::Screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
    );
    #[no_mangle]
    fn screen_write_hline(_: *mut screen_write_ctx, _: u_int, _: libc::c_int, _: libc::c_int);
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
    fn server_client_suspend(_: *mut client);
    #[no_mangle]
    fn server_client_how_many() -> u_int;
    #[no_mangle]
    fn server_client_unref(_: *mut client);
    #[no_mangle]
    fn window_pane_reset_mode(_: *mut window_pane);
    #[no_mangle]
    fn mode_tree_get_current(_: *mut crate::mode_tree::mode_tree_data) -> *mut libc::c_void;
    #[no_mangle]
    fn mode_tree_each_tagged(
        _: *mut crate::mode_tree::mode_tree_data,
        _: mode_tree_each_cb,
        _: *mut client,
        _: key_code,
        _: libc::c_int,
    );
    #[no_mangle]
    fn mode_tree_down(_: *mut crate::mode_tree::mode_tree_data, _: libc::c_int);
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
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}

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
pub struct window_client_modedata {
    pub wp: *mut window_pane,
    pub data: *mut crate::mode_tree::mode_tree_data,
    pub format: *mut libc::c_char,
    pub command: *mut libc::c_char,
    pub item_list: *mut *mut window_client_itemdata,
    pub item_size: u_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_client_itemdata {
    pub c: *mut client,
}
pub const WINDOW_CLIENT_BY_ACTIVITY_TIME: window_client_sort_type = 3;
pub const WINDOW_CLIENT_BY_CREATION_TIME: window_client_sort_type = 2;
pub const WINDOW_CLIENT_BY_SIZE: window_client_sort_type = 1;
pub type window_client_sort_type = libc::c_uint;
pub const WINDOW_CLIENT_BY_NAME: window_client_sort_type = 0;
static mut window_client_menu_items: [MenuItem; 9] = [
    {
        let mut init = MenuItem {
            name: b"Detach\x00" as *const u8 as *const libc::c_char,
            key: 'd' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = MenuItem {
            name: b"Detach Tagged\x00" as *const u8 as *const libc::c_char,
            key: 'D' as i32 as key_code,
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
pub static mut window_client_mode: window_mode = {
    {
        let mut init = window_mode {
            name: b"client-mode\x00" as *const u8 as *const libc::c_char,
            default_format: b"#{t/p:client_activity}: session #{session_name}\x00" as *const u8
                as *const libc::c_char,
            init: Some(
                window_client_init
                    as unsafe extern "C" fn(
                        _: *mut window_mode_entry,
                        _: *mut cmd_find_state,
                        _: *mut args,
                    ) -> *mut crate::screen::Screen,
            ),
            free: Some(window_client_free as unsafe extern "C" fn(_: *mut window_mode_entry) -> ()),
            resize: Some(
                window_client_resize
                    as unsafe extern "C" fn(_: *mut window_mode_entry, _: u_int, _: u_int) -> (),
            ),
            key: Some(
                window_client_key
                    as unsafe extern "C" fn(
                        _: *mut window_mode_entry,
                        _: *mut client,
                        _: *mut session,
                        _: *mut winlink,
                        _: key_code,
                        _: *mut mouse_event,
                    ) -> (),
            ),
            key_table: None,
            command: None,
            formats: None,
        };
        init
    }
};
static mut window_client_sort_list: [*const libc::c_char; 4] = [
    b"name\x00" as *const u8 as *const libc::c_char,
    b"size\x00" as *const u8 as *const libc::c_char,
    b"creation\x00" as *const u8 as *const libc::c_char,
    b"activity\x00" as *const u8 as *const libc::c_char,
];
static mut window_client_sort: *mut mode_tree_sort_criteria = 0 as *mut mode_tree_sort_criteria;
unsafe extern "C" fn window_client_add_item(
    mut data: *mut window_client_modedata,
) -> *mut window_client_itemdata {
    let mut item: *mut window_client_itemdata = 0 as *mut window_client_itemdata;
    (*data).item_list = xreallocarray(
        (*data).item_list as *mut libc::c_void,
        (*data).item_size.wrapping_add(1u32) as size_t,
        ::std::mem::size_of::<*mut window_client_itemdata>() as libc::c_ulong,
    ) as *mut *mut window_client_itemdata;
    let fresh0 = (*data).item_size;
    (*data).item_size = (*data).item_size.wrapping_add(1);
    let ref mut fresh1 = *(*data).item_list.offset(fresh0 as isize);
    *fresh1 = xcalloc(
        1u64,
        ::std::mem::size_of::<window_client_itemdata>() as libc::c_ulong,
    ) as *mut window_client_itemdata;
    item = *fresh1;
    return item;
}
unsafe extern "C" fn window_client_free_item(mut item: *mut window_client_itemdata) {
    server_client_unref((*item).c);
    free(item as *mut libc::c_void);
}
unsafe extern "C" fn window_client_cmp(
    mut a0: *const libc::c_void,
    mut b0: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const *const window_client_itemdata = a0 as *const *const window_client_itemdata;
    let mut b: *const *const window_client_itemdata = b0 as *const *const window_client_itemdata;
    let mut itema: *const window_client_itemdata = *a;
    let mut itemb: *const window_client_itemdata = *b;
    let mut ca: *mut client = (*itema).c;
    let mut cb: *mut client = (*itemb).c;
    let mut result: libc::c_int = 0i32;
    match (*window_client_sort).field {
        1 => {
            result = (*ca).tty.sx.wrapping_sub((*cb).tty.sx) as libc::c_int;
            if result == 0i32 {
                result = (*ca).tty.sy.wrapping_sub((*cb).tty.sy) as libc::c_int
            }
        }
        2 => {
            if if (*ca).creation_time.tv_sec == (*cb).creation_time.tv_sec {
                ((*ca).creation_time.tv_usec > (*cb).creation_time.tv_usec) as libc::c_int
            } else {
                ((*ca).creation_time.tv_sec > (*cb).creation_time.tv_sec) as libc::c_int
            } != 0
            {
                result = -(1i32)
            } else if if (*ca).creation_time.tv_sec == (*cb).creation_time.tv_sec {
                ((*ca).creation_time.tv_usec < (*cb).creation_time.tv_usec) as libc::c_int
            } else {
                ((*ca).creation_time.tv_sec < (*cb).creation_time.tv_sec) as libc::c_int
            } != 0
            {
                result = 1i32
            }
        }
        3 => {
            if if (*ca).activity_time.tv_sec == (*cb).activity_time.tv_sec {
                ((*ca).activity_time.tv_usec > (*cb).activity_time.tv_usec) as libc::c_int
            } else {
                ((*ca).activity_time.tv_sec > (*cb).activity_time.tv_sec) as libc::c_int
            } != 0
            {
                result = -(1i32)
            } else if if (*ca).activity_time.tv_sec == (*cb).activity_time.tv_sec {
                ((*ca).activity_time.tv_usec < (*cb).activity_time.tv_usec) as libc::c_int
            } else {
                ((*ca).activity_time.tv_sec < (*cb).activity_time.tv_sec) as libc::c_int
            } != 0
            {
                result = 1i32
            }
        }
        _ => {}
    }
    /* Use WINDOW_CLIENT_BY_NAME as default order and tie breaker. */
    if result == 0i32 {
        result = strcmp((*ca).name, (*cb).name)
    }
    if (*window_client_sort).reversed != 0 {
        result = -result
    }
    return result;
}
unsafe extern "C" fn window_client_build(
    mut modedata: *mut libc::c_void,
    mut sort_crit: *mut mode_tree_sort_criteria,
    mut _tag: *mut uint64_t,
    mut filter: *const libc::c_char,
) {
    let mut data: *mut window_client_modedata = modedata as *mut window_client_modedata;
    let mut item: *mut window_client_itemdata = 0 as *mut window_client_itemdata;
    let mut i: u_int = 0;
    let mut c: *mut client = 0 as *mut client;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0u32;
    while i < (*data).item_size {
        window_client_free_item(*(*data).item_list.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*data).item_list as *mut libc::c_void);
    (*data).item_list = 0 as *mut *mut window_client_itemdata;
    (*data).item_size = 0u32;
    c = clients.tqh_first;
    while !c.is_null() {
        if !((*c).session.is_null()
            || (*c).flags & (0x200i32 | 0x40i32 | 0x4i32) as libc::c_ulong != 0)
        {
            item = window_client_add_item(data);
            (*item).c = c;
            (*c).references += 1
        }
        c = (*c).entry.tqe_next
    }
    window_client_sort = sort_crit;
    qsort(
        (*data).item_list as *mut libc::c_void,
        (*data).item_size as size_t,
        ::std::mem::size_of::<*mut window_client_itemdata>() as libc::c_ulong,
        Some(
            window_client_cmp
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut current_block_21: u64;
    i = 0u32;
    while i < (*data).item_size {
        item = *(*data).item_list.offset(i as isize);
        c = (*item).c;
        if !filter.is_null() {
            cp = format_single(
                0 as *mut crate::cmd_queue::cmdq_item,
                filter,
                c,
                0 as *mut session,
                0 as *mut winlink,
                0 as *mut window_pane,
            );
            if format_true(cp) == 0 {
                free(cp as *mut libc::c_void);
                current_block_21 = 15652330335145281839;
            } else {
                free(cp as *mut libc::c_void);
                current_block_21 = 18386322304582297246;
            }
        } else {
            current_block_21 = 18386322304582297246;
        }
        match current_block_21 {
            18386322304582297246 => {
                text = format_single(
                    0 as *mut crate::cmd_queue::cmdq_item,
                    (*data).format,
                    c,
                    0 as *mut session,
                    0 as *mut winlink,
                    0 as *mut window_pane,
                );
                mode_tree_add(
                    (*data).data,
                    0 as *mut crate::mode_tree::mode_tree_item,
                    item as *mut libc::c_void,
                    c as uint64_t,
                    (*c).name,
                    text,
                    -(1i32),
                );
                free(text as *mut libc::c_void);
            }
            _ => {}
        }
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn window_client_draw(
    mut _modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut item: *mut window_client_itemdata = itemdata as *mut window_client_itemdata;
    let mut c: *mut client = (*item).c;
    let mut s: *mut Screen = (*ctx).s;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut cx: u_int = (*s).cx;
    let mut cy: u_int = (*s).cy;
    let mut lines: u_int = 0;
    let mut at: u_int = 0;
    if (*c).session.is_null() || (*c).flags & (0x200i32 | 0x40i32 | 0x4i32) as libc::c_ulong != 0 {
        return;
    }
    wp = (*(*(*(*c).session).curw).window).active;
    lines = status_line_size(c);
    if lines >= sy {
        lines = 0u32
    }
    if status_at_line(c) == 0i32 {
        at = lines
    } else {
        at = 0u32
    }
    screen_write_cursormove(
        ctx,
        cx as libc::c_int,
        cy.wrapping_add(at) as libc::c_int,
        0i32,
    );
    screen_write_preview(
        ctx,
        &mut (*wp).base,
        sx,
        sy.wrapping_sub(2u32).wrapping_sub(lines),
    );
    if at != 0u32 {
        screen_write_cursormove(
            ctx,
            cx as libc::c_int,
            cy.wrapping_add(2u32) as libc::c_int,
            0i32,
        );
    } else {
        screen_write_cursormove(
            ctx,
            cx as libc::c_int,
            cy.wrapping_add(sy).wrapping_sub(1u32).wrapping_sub(lines) as libc::c_int,
            0i32,
        );
    }
    screen_write_hline(ctx, sx, 0i32, 0i32);
    if at != 0u32 {
        screen_write_cursormove(ctx, cx as libc::c_int, cy as libc::c_int, 0i32);
    } else {
        screen_write_cursormove(
            ctx,
            cx as libc::c_int,
            cy.wrapping_add(sy).wrapping_sub(lines) as libc::c_int,
            0i32,
        );
    }
    screen_write_fast_copy(ctx, &mut (*c).status.screen, 0u32, 0u32, sx, lines);
}
unsafe extern "C" fn window_client_menu(
    mut modedata: *mut libc::c_void,
    mut c: *mut client,
    mut key: key_code,
) {
    let mut data: *mut window_client_modedata = modedata as *mut window_client_modedata;
    let mut wp: *mut window_pane = (*data).wp;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    wme = (*wp).modes.tqh_first;
    if wme.is_null() || (*wme).data != modedata {
        return;
    }
    window_client_key(
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
unsafe extern "C" fn window_client_init(
    mut wme: *mut window_mode_entry,
    mut _fs: *mut cmd_find_state,
    mut args: *mut args,
) -> *mut Screen {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_client_modedata = 0 as *mut window_client_modedata;
    let mut s: *mut Screen = 0 as *mut Screen;
    data = xcalloc(
        1u64,
        ::std::mem::size_of::<window_client_modedata>() as libc::c_ulong,
    ) as *mut window_client_modedata;
    (*wme).data = data as *mut libc::c_void;
    (*data).wp = wp;
    if args.is_null() || args_has(args, 'F' as u_char) == 0 {
        (*data).format = xstrdup(
            b"#{t/p:client_activity}: session #{session_name}\x00" as *const u8
                as *const libc::c_char,
        )
    } else {
        (*data).format = xstrdup(args_get(args, 'F' as u_char))
    }
    if args.is_null() || (*args).argc == 0i32 {
        (*data).command =
            xstrdup(b"detach-client -t \'%%\'\x00" as *const u8 as *const libc::c_char)
    } else {
        (*data).command = xstrdup(*(*args).argv.offset(0isize))
    }
    (*data).data = mode_tree_start(
        wp,
        args,
        Some(
            window_client_build
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut mode_tree_sort_criteria,
                    _: *mut uint64_t,
                    _: *const libc::c_char,
                ) -> (),
        ),
        Some(
            window_client_draw
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                    _: *mut screen_write_ctx,
                    _: u_int,
                    _: u_int,
                ) -> (),
        ),
        None,
        Some(
            window_client_menu
                as unsafe extern "C" fn(_: *mut libc::c_void, _: *mut client, _: key_code) -> (),
        ),
        None,
        data as *mut libc::c_void,
        window_client_menu_items.as_ptr(),
        window_client_sort_list.as_mut_ptr(),
        (::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as u_int,
        &mut s,
    );
    mode_tree_zoom((*data).data, args);
    mode_tree_build((*data).data);
    mode_tree_draw((*data).data);
    return s;
}
unsafe extern "C" fn window_client_free(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_client_modedata = (*wme).data as *mut window_client_modedata;
    let mut i: u_int = 0;
    if data.is_null() {
        return;
    }
    mode_tree_free((*data).data);
    i = 0u32;
    while i < (*data).item_size {
        window_client_free_item(*(*data).item_list.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*data).item_list as *mut libc::c_void);
    free((*data).format as *mut libc::c_void);
    free((*data).command as *mut libc::c_void);
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn window_client_resize(
    mut wme: *mut window_mode_entry,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut data: *mut window_client_modedata = (*wme).data as *mut window_client_modedata;
    mode_tree_resize((*data).data, sx, sy);
}
unsafe extern "C" fn window_client_do_detach(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut _c: *mut client,
    mut key: key_code,
) {
    let mut data: *mut window_client_modedata = modedata as *mut window_client_modedata;
    let mut item: *mut window_client_itemdata = itemdata as *mut window_client_itemdata;
    if item == mode_tree_get_current((*data).data) as *mut window_client_itemdata {
        mode_tree_down((*data).data, 0i32);
    }
    if key == 'd' as i32 as libc::c_ulonglong || key == 'D' as i32 as libc::c_ulonglong {
        server_client_detach((*item).c, msgtype_code::DETACH);
    } else if key == 'x' as i32 as libc::c_ulonglong || key == 'X' as i32 as libc::c_ulonglong {
        server_client_detach((*item).c, msgtype_code::DETACHKILL);
    } else if key == 'z' as i32 as libc::c_ulonglong || key == 'Z' as i32 as libc::c_ulonglong {
        server_client_suspend((*item).c);
    };
}
unsafe extern "C" fn window_client_key(
    mut wme: *mut window_mode_entry,
    mut c: *mut client,
    mut _s: *mut session,
    mut _wl: *mut winlink,
    mut key: key_code,
    mut m: *mut mouse_event,
) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_client_modedata = (*wme).data as *mut window_client_modedata;
    let mut mtd: *mut crate::mode_tree::mode_tree_data = (*data).data;
    let mut item: *mut window_client_itemdata = 0 as *mut window_client_itemdata;
    let mut finished: libc::c_int = 0;
    finished = mode_tree_key(mtd, c, &mut key, m, 0 as *mut u_int, 0 as *mut u_int);
    match key {
        100 | 120 | 122 => {
            item = mode_tree_get_current(mtd) as *mut window_client_itemdata;
            window_client_do_detach(data as *mut libc::c_void, item as *mut libc::c_void, c, key);
            mode_tree_build(mtd);
        }
        68 | 88 | 90 => {
            mode_tree_each_tagged(
                mtd,
                Some(
                    window_client_do_detach
                        as unsafe extern "C" fn(
                            _: *mut libc::c_void,
                            _: *mut libc::c_void,
                            _: *mut client,
                            _: key_code,
                        ) -> (),
                ),
                c,
                key,
                0i32,
            );
            mode_tree_build(mtd);
        }
        13 => {
            item = mode_tree_get_current(mtd) as *mut window_client_itemdata;
            mode_tree_run_command(
                c,
                0 as *mut cmd_find_state,
                (*data).command,
                (*(*item).c).ttyname,
            );
            finished = 1i32
        }
        _ => {}
    }
    if finished != 0 || server_client_how_many() == 0u32 {
        window_pane_reset_mode(wp);
    } else {
        mode_tree_draw(mtd);
        (*wp).flags |= 0x1i32
    };
}
