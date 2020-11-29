use crate::tty_code::{code as tty_code_code, Code as TtyCode};
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn event_once(
        _: libc::c_int,
        _: libc::c_short,
        _: Option<
            unsafe extern "C" fn(_: libc::c_int, _: libc::c_short, _: *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
        _: *const timeval,
    ) -> libc::c_int;
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
    fn event_initialized(ev: *const event) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn notify_winlink(_: *const libc::c_char, _: *mut winlink);
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn tty_putcode(_: *mut tty, _: TtyCode);
    #[no_mangle]
    fn status_message_set(
        _: *mut client,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn server_status_session(_: *mut session);
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    #[no_mangle]
    fn window_add_ref(_: *mut window, _: *const libc::c_char);
    #[no_mangle]
    fn window_remove_ref(_: *mut window, _: *const libc::c_char);
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
    pub linedata: *mut crate::grid::Line,
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
pub struct windows {
    pub rbh_root: *mut window,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub tqh_first: *mut window,
    pub tqh_last: *mut *mut window,
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2015 Nicholas Marriott <nicholas.marriott@gmail.com>
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
static mut alerts_fired: libc::c_int = 0;
// Initialized in run_static_initializers
static mut alerts_list: C2RustUnnamed_32 = C2RustUnnamed_32 {
    tqh_first: 0 as *mut window,
    tqh_last: 0 as *mut *mut window,
};
unsafe extern "C" fn alerts_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut w: *mut window = arg as *mut window;
    log_debug(
        b"@%u alerts timer expired\x00" as *const u8 as *const libc::c_char,
        (*w).id,
    );
    alerts_queue(w, 0x4i32);
}
unsafe extern "C" fn alerts_callback(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut _arg: *mut libc::c_void,
) {
    let mut w: *mut window = 0 as *mut window;
    let mut w1: *mut window = 0 as *mut window;
    let mut alerts: libc::c_int = 0;
    w = alerts_list.tqh_first;
    while !w.is_null() && {
        w1 = (*w).alerts_entry.tqe_next;
        (1i32) != 0
    } {
        alerts = alerts_check_all(w);
        log_debug(
            b"@%u alerts check, alerts %#x\x00" as *const u8 as *const libc::c_char,
            (*w).id,
            alerts,
        );
        (*w).alerts_queued = 0i32;
        if !(*w).alerts_entry.tqe_next.is_null() {
            (*(*w).alerts_entry.tqe_next).alerts_entry.tqe_prev = (*w).alerts_entry.tqe_prev
        } else {
            alerts_list.tqh_last = (*w).alerts_entry.tqe_prev
        }
        *(*w).alerts_entry.tqe_prev = (*w).alerts_entry.tqe_next;
        (*w).flags &= !(0x1i32 | 0x2i32 | 0x4i32);
        window_remove_ref(
            w,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"alerts_callback\x00"))
                .as_ptr(),
        );
        w = w1
    }
    alerts_fired = 0i32;
}
unsafe extern "C" fn alerts_action_applies(
    mut wl: *mut winlink,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut action: libc::c_int = 0;
    /*
     * {bell,activity,silence}-action determines when to alert: none means
     * nothing happens, current means only do something for the current
     * window and other means only for windows other than the current.
     */
    action = options_get_number((*(*wl).session).options, name) as libc::c_int;
    if action == 1i32 {
        return 1i32;
    }
    if action == 2i32 {
        return (wl == (*(*wl).session).curw) as libc::c_int;
    }
    if action == 3i32 {
        return (wl != (*(*wl).session).curw) as libc::c_int;
    }
    return 0i32;
}
unsafe extern "C" fn alerts_check_all(mut w: *mut window) -> libc::c_int {
    let mut alerts: libc::c_int = 0;
    alerts = alerts_check_bell(w);
    alerts |= alerts_check_activity(w);
    alerts |= alerts_check_silence(w);
    return alerts;
}
#[no_mangle]
pub unsafe extern "C" fn alerts_check_session(mut s: *mut session) {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1i32));
    while !wl.is_null() {
        alerts_check_all((*wl).window);
        wl = winlinks_RB_NEXT(wl)
    }
}
unsafe extern "C" fn alerts_enabled(mut w: *mut window, mut flags: libc::c_int) -> libc::c_int {
    if flags & 0x1i32 != 0 {
        if options_get_number(
            (*w).options,
            b"monitor-bell\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
            return 1i32;
        }
    }
    if flags & 0x2i32 != 0 {
        if options_get_number(
            (*w).options,
            b"monitor-activity\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
            return 1i32;
        }
    }
    if flags & 0x4i32 != 0 {
        if options_get_number(
            (*w).options,
            b"monitor-silence\x00" as *const u8 as *const libc::c_char,
        ) != 0i64
        {
            return 1i32;
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn alerts_reset_all() {
    let mut w: *mut window = 0 as *mut window;
    w = windows_RB_MINMAX(&mut windows, -(1i32));
    while !w.is_null() {
        alerts_reset(w);
        w = windows_RB_NEXT(w)
    }
}
unsafe extern "C" fn alerts_reset(mut w: *mut window) {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if event_initialized(&mut (*w).alerts_timer) == 0 {
        event_set(
            &mut (*w).alerts_timer,
            -(1i32),
            0i16,
            Some(
                alerts_timer
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            w as *mut libc::c_void,
        );
    }
    (*w).flags &= !(0x4i32);
    event_del(&mut (*w).alerts_timer);
    tv.tv_usec = 0i64;
    tv.tv_sec = tv.tv_usec;
    tv.tv_sec = options_get_number(
        (*w).options,
        b"monitor-silence\x00" as *const u8 as *const libc::c_char,
    );
    log_debug(
        b"@%u alerts timer reset %u\x00" as *const u8 as *const libc::c_char,
        (*w).id,
        tv.tv_sec as u_int,
    );
    if tv.tv_sec != 0i64 {
        event_add(&mut (*w).alerts_timer, &mut tv);
    };
}
#[no_mangle]
pub unsafe extern "C" fn alerts_queue(mut w: *mut window, mut flags: libc::c_int) {
    alerts_reset(w);
    if (*w).flags & flags != flags {
        (*w).flags |= flags;
        log_debug(
            b"@%u alerts flags added %#x\x00" as *const u8 as *const libc::c_char,
            (*w).id,
            flags,
        );
    }
    if alerts_enabled(w, flags) != 0 {
        if (*w).alerts_queued == 0 {
            (*w).alerts_queued = 1i32;
            (*w).alerts_entry.tqe_next = 0 as *mut window;
            (*w).alerts_entry.tqe_prev = alerts_list.tqh_last;
            *alerts_list.tqh_last = w;
            alerts_list.tqh_last = &mut (*w).alerts_entry.tqe_next;
            window_add_ref(
                w,
                (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"alerts_queue\x00"))
                    .as_ptr(),
            );
        }
        if alerts_fired == 0 {
            log_debug(
                b"alerts check queued (by @%u)\x00" as *const u8 as *const libc::c_char,
                (*w).id,
            );
            event_once(
                -(1i32),
                0x1i16,
                Some(
                    alerts_callback
                        as unsafe extern "C" fn(
                            _: libc::c_int,
                            _: libc::c_short,
                            _: *mut libc::c_void,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
                0 as *const timeval,
            );
            alerts_fired = 1i32
        }
    };
}
unsafe extern "C" fn alerts_check_bell(mut w: *mut window) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut s: *mut session = 0 as *mut session;
    if !(*w).flags & 0x1i32 != 0 {
        return 0i32;
    }
    if options_get_number(
        (*w).options,
        b"monitor-bell\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 0i32;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        (*(*wl).session).flags &= !(0x2i32);
        wl = (*wl).wentry.tqe_next
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        /*
         * Bells are allowed even if there is an existing bell (so do
         * not check WINLINK_BELL).
         */
        s = (*wl).session;
        if (*s).curw != wl || (*s).attached == 0u32 {
            (*wl).flags |= 0x1i32;
            server_status_session(s);
        }
        if !(alerts_action_applies(wl, b"bell-action\x00" as *const u8 as *const libc::c_char) == 0)
        {
            notify_winlink(b"alert-bell\x00" as *const u8 as *const libc::c_char, wl);
            if !((*s).flags & 0x2i32 != 0) {
                (*s).flags |= 0x2i32;
                alerts_set_message(
                    wl,
                    b"Bell\x00" as *const u8 as *const libc::c_char,
                    b"visual-bell\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        wl = (*wl).wentry.tqe_next
    }
    return 0x1i32;
}
unsafe extern "C" fn alerts_check_activity(mut w: *mut window) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut s: *mut session = 0 as *mut session;
    if !(*w).flags & 0x2i32 != 0 {
        return 0i32;
    }
    if options_get_number(
        (*w).options,
        b"monitor-activity\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 0i32;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        (*(*wl).session).flags &= !(0x2i32);
        wl = (*wl).wentry.tqe_next
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if !((*wl).flags & 0x2i32 != 0) {
            s = (*wl).session;
            if (*s).curw != wl || (*s).attached == 0u32 {
                (*wl).flags |= 0x2i32;
                server_status_session(s);
            }
            if !(alerts_action_applies(
                wl,
                b"activity-action\x00" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                notify_winlink(
                    b"alert-activity\x00" as *const u8 as *const libc::c_char,
                    wl,
                );
                if !((*s).flags & 0x2i32 != 0) {
                    (*s).flags |= 0x2i32;
                    alerts_set_message(
                        wl,
                        b"Activity\x00" as *const u8 as *const libc::c_char,
                        b"visual-activity\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        wl = (*wl).wentry.tqe_next
    }
    return 0x2i32;
}
unsafe extern "C" fn alerts_check_silence(mut w: *mut window) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut s: *mut session = 0 as *mut session;
    if !(*w).flags & 0x4i32 != 0 {
        return 0i32;
    }
    if options_get_number(
        (*w).options,
        b"monitor-silence\x00" as *const u8 as *const libc::c_char,
    ) == 0i64
    {
        return 0i32;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        (*(*wl).session).flags &= !(0x2i32);
        wl = (*wl).wentry.tqe_next
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if !((*wl).flags & 0x4i32 != 0) {
            s = (*wl).session;
            if (*s).curw != wl || (*s).attached == 0u32 {
                (*wl).flags |= 0x4i32;
                server_status_session(s);
            }
            if !(alerts_action_applies(
                wl,
                b"silence-action\x00" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                notify_winlink(b"alert-silence\x00" as *const u8 as *const libc::c_char, wl);
                if !((*s).flags & 0x2i32 != 0) {
                    (*s).flags |= 0x2i32;
                    alerts_set_message(
                        wl,
                        b"Silence\x00" as *const u8 as *const libc::c_char,
                        b"visual-silence\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        wl = (*wl).wentry.tqe_next
    }
    return 0x4i32;
}
unsafe extern "C" fn alerts_set_message(
    mut wl: *mut winlink,
    mut type_0: *const libc::c_char,
    mut option: *const libc::c_char,
) {
    let mut c: *mut client = 0 as *mut client;
    let mut visual: libc::c_int = 0;
    /*
     * We have found an alert (bell, activity or silence), so we need to
     * pass it on to the user. For each client attached to this session,
     * decide whether a bell, message or both is needed.
     *
     * If visual-{bell,activity,silence} is on, then a message is
     * substituted for a bell; if it is off, a bell is sent as normal; both
     * mean both a bell and message is sent.
     */
    visual = options_get_number((*(*wl).session).options, option) as libc::c_int;
    c = clients.tqh_first;
    while !c.is_null() {
        if !((*c).session != (*wl).session || (*c).flags & 0x2000u64 != 0) {
            if visual == 0i32 || visual == 2i32 {
                tty_putcode(&mut (*c).tty, tty_code_code::BEL);
            }
            if !(visual == 0i32) {
                if (*(*c).session).curw == wl {
                    status_message_set(
                        c,
                        -(1i32),
                        1i32,
                        b"%s in current window\x00" as *const u8 as *const libc::c_char,
                        type_0,
                    );
                } else {
                    status_message_set(
                        c,
                        -(1i32),
                        1i32,
                        b"%s in window %d\x00" as *const u8 as *const libc::c_char,
                        type_0,
                        (*wl).idx,
                    );
                }
            }
        }
        c = (*c).entry.tqe_next
    }
}
unsafe extern "C" fn run_static_initializers() {
    alerts_list = {
        let mut init = C2RustUnnamed_32 {
            tqh_first: 0 as *mut window,
            tqh_last: &mut alerts_list.tqh_first,
        };
        init
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
