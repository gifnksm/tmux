use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn notify_window(_: *const libc::c_char, _: *mut window);
    #[no_mangle]
    fn options_get_string(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn args_string_percentage(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *mut libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn server_redraw_window(_: *mut window);
    #[no_mangle]
    fn window_resize(_: *mut window, _: u_int, _: u_int, _: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn layout_create_cell(_: *mut layout_cell) -> *mut layout_cell;
    #[no_mangle]
    fn layout_print_cell(_: *mut layout_cell, _: *const libc::c_char, _: u_int);
    #[no_mangle]
    fn layout_set_size(_: *mut layout_cell, _: u_int, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn layout_make_leaf(_: *mut layout_cell, _: *mut window_pane);
    #[no_mangle]
    fn layout_make_node(_: *mut layout_cell, _: layout_type);
    #[no_mangle]
    fn layout_fix_offsets(_: *mut window);
    #[no_mangle]
    fn layout_fix_panes(_: *mut window);
    #[no_mangle]
    fn layout_resize_adjust(_: *mut window, _: *mut layout_cell, _: layout_type, _: libc::c_int);
    #[no_mangle]
    fn layout_free(_: *mut window);
    #[no_mangle]
    fn layout_spread_cell(_: *mut window, _: *mut layout_cell) -> libc::c_int;
    #[no_mangle]
    fn window_count_panes(_: *mut window) -> u_int;
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
pub struct C2RustUnnamed_32 {
    pub name: *const libc::c_char,
    pub arrange: Option<unsafe extern "C" fn(_: *mut window) -> ()>,
}
static mut layout_sets: [C2RustUnnamed_32; 5] = {
    [
        {
            let mut init = C2RustUnnamed_32 {
                name: b"even-horizontal\x00" as *const u8 as *const libc::c_char,
                arrange: Some(layout_set_even_h as unsafe extern "C" fn(_: *mut window) -> ()),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_32 {
                name: b"even-vertical\x00" as *const u8 as *const libc::c_char,
                arrange: Some(layout_set_even_v as unsafe extern "C" fn(_: *mut window) -> ()),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_32 {
                name: b"main-horizontal\x00" as *const u8 as *const libc::c_char,
                arrange: Some(layout_set_main_h as unsafe extern "C" fn(_: *mut window) -> ()),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_32 {
                name: b"main-vertical\x00" as *const u8 as *const libc::c_char,
                arrange: Some(layout_set_main_v as unsafe extern "C" fn(_: *mut window) -> ()),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_32 {
                name: b"tiled\x00" as *const u8 as *const libc::c_char,
                arrange: Some(layout_set_tiled as unsafe extern "C" fn(_: *mut window) -> ()),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn layout_set_lookup(mut name: *const libc::c_char) -> libc::c_int {
    let mut i: u_int = 0;
    let mut matched: libc::c_int = -(1i32);
    i = 0u32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[C2RustUnnamed_32; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_32>() as libc::c_ulong)
    {
        if strncmp(layout_sets[i as usize].name, name, strlen(name)) == 0i32 {
            if matched != -(1i32) {
                /* ambiguous */
                return -(1i32);
            }
            matched = i as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return matched;
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_select(mut w: *mut window, mut layout: u_int) -> u_int {
    if layout as libc::c_ulong
        > (::std::mem::size_of::<[C2RustUnnamed_32; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_32>() as libc::c_ulong)
            .wrapping_sub(1u64)
    {
        layout = (::std::mem::size_of::<[C2RustUnnamed_32; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_32>() as libc::c_ulong)
            .wrapping_sub(1u64) as u_int
    }
    if layout_sets[layout as usize].arrange.is_some() {
        layout_sets[layout as usize]
            .arrange
            .expect("non-null function pointer")(w);
    }
    (*w).lastlayout = layout as libc::c_int;
    return layout;
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_next(mut w: *mut window) -> u_int {
    let mut layout: u_int = 0;
    if (*w).lastlayout == -(1i32) {
        layout = 0u32
    } else {
        layout = ((*w).lastlayout + 1i32) as u_int;
        if layout as libc::c_ulong
            > (::std::mem::size_of::<[C2RustUnnamed_32; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<C2RustUnnamed_32>() as libc::c_ulong)
                .wrapping_sub(1u64)
        {
            layout = 0u32
        }
    }
    if layout_sets[layout as usize].arrange.is_some() {
        layout_sets[layout as usize]
            .arrange
            .expect("non-null function pointer")(w);
    }
    (*w).lastlayout = layout as libc::c_int;
    return layout;
}
#[no_mangle]
pub unsafe extern "C" fn layout_set_previous(mut w: *mut window) -> u_int {
    let mut layout: u_int = 0;
    if (*w).lastlayout == -(1i32) {
        layout = (::std::mem::size_of::<[C2RustUnnamed_32; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_32>() as libc::c_ulong)
            .wrapping_sub(1u64) as u_int
    } else {
        layout = (*w).lastlayout as u_int;
        if layout == 0u32 {
            layout = (::std::mem::size_of::<[C2RustUnnamed_32; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<C2RustUnnamed_32>() as libc::c_ulong)
                .wrapping_sub(1u64) as u_int
        } else {
            layout = layout.wrapping_sub(1)
        }
    }
    if layout_sets[layout as usize].arrange.is_some() {
        layout_sets[layout as usize]
            .arrange
            .expect("non-null function pointer")(w);
    }
    (*w).lastlayout = layout as libc::c_int;
    return layout;
}
unsafe extern "C" fn layout_set_even(mut w: *mut window, mut type_0: layout_type) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcnew: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    layout_print_cell(
        (*w).layout_root,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"layout_set_even\x00")).as_ptr(),
        1u32,
    );
    /* Get number of panes. */
    n = window_count_panes(w);
    if n <= 1u32 {
        return;
    }
    /* Free the old root and construct a new. */
    layout_free(w);
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    if type_0 == LAYOUT_LEFTRIGHT {
        sx = n
            .wrapping_mul((1i32 + 1i32) as libc::c_uint)
            .wrapping_sub(1u32);
        if sx < (*w).sx {
            sx = (*w).sx
        }
        sy = (*w).sy
    } else {
        sy = n
            .wrapping_mul((1i32 + 1i32) as libc::c_uint)
            .wrapping_sub(1u32);
        if sy < (*w).sy {
            sy = (*w).sy
        }
        sx = (*w).sx
    }
    layout_set_size(lc, sx, sy, 0u32, 0u32);
    layout_make_node(lc, type_0);
    /* Build new leaf cells. */
    wp = (*w).panes.tqh_first;
    while !wp.is_null() {
        lcnew = layout_create_cell(lc);
        layout_make_leaf(lcnew, wp);
        (*lcnew).sx = (*w).sx;
        (*lcnew).sy = (*w).sy;
        (*lcnew).entry.tqe_next = 0 as *mut layout_cell;
        (*lcnew).entry.tqe_prev = (*lc).cells.tqh_last;
        *(*lc).cells.tqh_last = lcnew;
        (*lc).cells.tqh_last = &mut (*lcnew).entry.tqe_next;
        wp = (*wp).entry.tqe_next
    }
    /* Spread out cells. */
    layout_spread_cell(w, lc);
    /* Fix cell offsets. */
    layout_fix_offsets(w);
    layout_fix_panes(w);
    layout_print_cell(
        (*w).layout_root,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"layout_set_even\x00")).as_ptr(),
        1u32,
    );
    window_resize(w, (*lc).sx, (*lc).sy, -(1i32), -(1i32));
    notify_window(
        b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
        w,
    );
    server_redraw_window(w);
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
 * Set window layouts - predefined methods to arrange windows. These are
 * one-off and generate a layout tree.
 */
unsafe extern "C" fn layout_set_even_h(mut w: *mut window) {
    layout_set_even(w, LAYOUT_LEFTRIGHT);
}
unsafe extern "C" fn layout_set_even_v(mut w: *mut window) {
    layout_set_even(w, LAYOUT_TOPBOTTOM);
}
unsafe extern "C" fn layout_set_main_h(mut w: *mut window) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcmain: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcother: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    let mut mainh: u_int = 0;
    let mut otherh: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    layout_print_cell(
        (*w).layout_root,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"layout_set_main_h\x00"))
            .as_ptr(),
        1u32,
    );
    /* Get number of panes. */
    n = window_count_panes(w); /* take off main pane */
    if n <= 1u32 {
        return;
    }
    n = n.wrapping_sub(1);
    /* Find available height - take off one line for the border. */
    sy = (*w).sy.wrapping_sub(1u32);
    /* Get the main pane height. */
    s = options_get_string(
        (*w).options,
        b"main-pane-height\x00" as *const u8 as *const libc::c_char,
    );
    mainh = args_string_percentage(
        s,
        0i64,
        sy as libc::c_longlong,
        sy as libc::c_longlong,
        &mut cause,
    ) as u_int;
    if !cause.is_null() {
        mainh = 24u32;
        free(cause as *mut libc::c_void);
    }
    /* Work out the other pane height. */
    if mainh.wrapping_add(1u32) >= sy {
        if sy <= (1i32 + 1i32) as libc::c_uint {
            mainh = 1u32
        } else {
            mainh = sy.wrapping_sub(1u32)
        }
        otherh = 1u32
    } else {
        s = options_get_string(
            (*w).options,
            b"other-pane-height\x00" as *const u8 as *const libc::c_char,
        );
        otherh = args_string_percentage(
            s,
            0i64,
            sy as libc::c_longlong,
            sy as libc::c_longlong,
            &mut cause,
        ) as u_int;
        if !cause.is_null() || otherh == 0u32 {
            otherh = sy.wrapping_sub(mainh);
            free(cause as *mut libc::c_void);
        } else if otherh > sy || sy.wrapping_sub(otherh) < mainh {
            otherh = sy.wrapping_sub(mainh)
        } else {
            mainh = sy.wrapping_sub(otherh)
        }
    }
    /* Work out what width is needed. */
    sx = n
        .wrapping_mul((1i32 + 1i32) as libc::c_uint)
        .wrapping_sub(1u32);
    if sx < (*w).sx {
        sx = (*w).sx
    }
    /* Free old tree and create a new root. */
    layout_free(w);
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    layout_set_size(
        lc,
        sx,
        mainh.wrapping_add(otherh).wrapping_add(1u32),
        0u32,
        0u32,
    );
    layout_make_node(lc, LAYOUT_TOPBOTTOM);
    /* Create the main pane. */
    lcmain = layout_create_cell(lc);
    layout_set_size(lcmain, sx, mainh, 0u32, 0u32);
    layout_make_leaf(lcmain, (*w).panes.tqh_first);
    (*lcmain).entry.tqe_next = 0 as *mut layout_cell;
    (*lcmain).entry.tqe_prev = (*lc).cells.tqh_last;
    *(*lc).cells.tqh_last = lcmain;
    (*lc).cells.tqh_last = &mut (*lcmain).entry.tqe_next;
    /* Create the other pane. */
    lcother = layout_create_cell(lc);
    layout_set_size(lcother, sx, otherh, 0u32, 0u32);
    if n == 1u32 {
        wp = (*(*w).panes.tqh_first).entry.tqe_next;
        layout_make_leaf(lcother, wp);
        (*lcother).entry.tqe_next = 0 as *mut layout_cell;
        (*lcother).entry.tqe_prev = (*lc).cells.tqh_last;
        *(*lc).cells.tqh_last = lcother;
        (*lc).cells.tqh_last = &mut (*lcother).entry.tqe_next
    } else {
        layout_make_node(lcother, LAYOUT_LEFTRIGHT);
        (*lcother).entry.tqe_next = 0 as *mut layout_cell;
        (*lcother).entry.tqe_prev = (*lc).cells.tqh_last;
        *(*lc).cells.tqh_last = lcother;
        (*lc).cells.tqh_last = &mut (*lcother).entry.tqe_next;
        /* Add the remaining panes as children. */
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if !(wp == (*w).panes.tqh_first) {
                lcchild = layout_create_cell(lcother);
                layout_set_size(lcchild, 1u32, otherh, 0u32, 0u32);
                layout_make_leaf(lcchild, wp);
                (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                (*lcchild).entry.tqe_prev = (*lcother).cells.tqh_last;
                *(*lcother).cells.tqh_last = lcchild;
                (*lcother).cells.tqh_last = &mut (*lcchild).entry.tqe_next
            }
            wp = (*wp).entry.tqe_next
        }
        layout_spread_cell(w, lcother);
    }
    /* Fix cell offsets. */
    layout_fix_offsets(w);
    layout_fix_panes(w);
    layout_print_cell(
        (*w).layout_root,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"layout_set_main_h\x00"))
            .as_ptr(),
        1u32,
    );
    window_resize(w, (*lc).sx, (*lc).sy, -(1i32), -(1i32));
    notify_window(
        b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
        w,
    );
    server_redraw_window(w);
}
unsafe extern "C" fn layout_set_main_v(mut w: *mut window) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcmain: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcother: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    let mut mainw: u_int = 0;
    let mut otherw: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    layout_print_cell(
        (*w).layout_root,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"layout_set_main_v\x00"))
            .as_ptr(),
        1u32,
    );
    /* Get number of panes. */
    n = window_count_panes(w); /* take off main pane */
    if n <= 1u32 {
        return;
    }
    n = n.wrapping_sub(1);
    /* Find available width - take off one line for the border. */
    sx = (*w).sx.wrapping_sub(1u32);
    /* Get the main pane width. */
    s = options_get_string(
        (*w).options,
        b"main-pane-width\x00" as *const u8 as *const libc::c_char,
    );
    mainw = args_string_percentage(
        s,
        0i64,
        sx as libc::c_longlong,
        sx as libc::c_longlong,
        &mut cause,
    ) as u_int;
    if !cause.is_null() {
        mainw = 80u32;
        free(cause as *mut libc::c_void);
    }
    /* Work out the other pane width. */
    if mainw.wrapping_add(1u32) >= sx {
        if sx <= (1i32 + 1i32) as libc::c_uint {
            mainw = 1u32
        } else {
            mainw = sx.wrapping_sub(1u32)
        }
        otherw = 1u32
    } else {
        s = options_get_string(
            (*w).options,
            b"other-pane-width\x00" as *const u8 as *const libc::c_char,
        );
        otherw = args_string_percentage(
            s,
            0i64,
            sx as libc::c_longlong,
            sx as libc::c_longlong,
            &mut cause,
        ) as u_int;
        if !cause.is_null() || otherw == 0u32 {
            otherw = sx.wrapping_sub(mainw);
            free(cause as *mut libc::c_void);
        } else if otherw > sx || sx.wrapping_sub(otherw) < mainw {
            otherw = sx.wrapping_sub(mainw)
        } else {
            mainw = sx.wrapping_sub(otherw)
        }
    }
    /* Work out what height is needed. */
    sy = n
        .wrapping_mul((1i32 + 1i32) as libc::c_uint)
        .wrapping_sub(1u32);
    if sy < (*w).sy {
        sy = (*w).sy
    }
    /* Free old tree and create a new root. */
    layout_free(w);
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    layout_set_size(
        lc,
        mainw.wrapping_add(otherw).wrapping_add(1u32),
        sy,
        0u32,
        0u32,
    );
    layout_make_node(lc, LAYOUT_LEFTRIGHT);
    /* Create the main pane. */
    lcmain = layout_create_cell(lc);
    layout_set_size(lcmain, mainw, sy, 0u32, 0u32);
    layout_make_leaf(lcmain, (*w).panes.tqh_first);
    (*lcmain).entry.tqe_next = 0 as *mut layout_cell;
    (*lcmain).entry.tqe_prev = (*lc).cells.tqh_last;
    *(*lc).cells.tqh_last = lcmain;
    (*lc).cells.tqh_last = &mut (*lcmain).entry.tqe_next;
    /* Create the other pane. */
    lcother = layout_create_cell(lc);
    layout_set_size(lcother, otherw, sy, 0u32, 0u32);
    if n == 1u32 {
        wp = (*(*w).panes.tqh_first).entry.tqe_next;
        layout_make_leaf(lcother, wp);
        (*lcother).entry.tqe_next = 0 as *mut layout_cell;
        (*lcother).entry.tqe_prev = (*lc).cells.tqh_last;
        *(*lc).cells.tqh_last = lcother;
        (*lc).cells.tqh_last = &mut (*lcother).entry.tqe_next
    } else {
        layout_make_node(lcother, LAYOUT_TOPBOTTOM);
        (*lcother).entry.tqe_next = 0 as *mut layout_cell;
        (*lcother).entry.tqe_prev = (*lc).cells.tqh_last;
        *(*lc).cells.tqh_last = lcother;
        (*lc).cells.tqh_last = &mut (*lcother).entry.tqe_next;
        /* Add the remaining panes as children. */
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if !(wp == (*w).panes.tqh_first) {
                lcchild = layout_create_cell(lcother);
                layout_set_size(lcchild, otherw, 1u32, 0u32, 0u32);
                layout_make_leaf(lcchild, wp);
                (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                (*lcchild).entry.tqe_prev = (*lcother).cells.tqh_last;
                *(*lcother).cells.tqh_last = lcchild;
                (*lcother).cells.tqh_last = &mut (*lcchild).entry.tqe_next
            }
            wp = (*wp).entry.tqe_next
        }
        layout_spread_cell(w, lcother);
    }
    /* Fix cell offsets. */
    layout_fix_offsets(w);
    layout_fix_panes(w);
    layout_print_cell(
        (*w).layout_root,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"layout_set_main_v\x00"))
            .as_ptr(),
        1u32,
    );
    window_resize(w, (*lc).sx, (*lc).sy, -(1i32), -(1i32));
    notify_window(
        b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
        w,
    );
    server_redraw_window(w);
}
unsafe extern "C" fn layout_set_tiled(mut w: *mut window) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut lc: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcrow: *mut layout_cell = 0 as *mut layout_cell;
    let mut lcchild: *mut layout_cell = 0 as *mut layout_cell;
    let mut n: u_int = 0;
    let mut width: u_int = 0;
    let mut height: u_int = 0;
    let mut used: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut i: u_int = 0;
    let mut j: u_int = 0;
    let mut columns: u_int = 0;
    let mut rows: u_int = 0;
    layout_print_cell(
        (*w).layout_root,
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"layout_set_tiled\x00"))
            .as_ptr(),
        1u32,
    );
    /* Get number of panes. */
    n = window_count_panes(w);
    if n <= 1u32 {
        return;
    }
    /* How many rows and columns are wanted? */
    columns = 1u32;
    rows = columns;
    while rows.wrapping_mul(columns) < n {
        rows = rows.wrapping_add(1);
        if rows.wrapping_mul(columns) < n {
            columns = columns.wrapping_add(1)
        }
    }
    /* What width and height should they be? */
    width = (*w)
        .sx
        .wrapping_sub(columns.wrapping_sub(1u32))
        .wrapping_div(columns);
    if width < 1u32 {
        width = 1u32
    }
    height = (*w)
        .sy
        .wrapping_sub(rows.wrapping_sub(1u32))
        .wrapping_div(rows);
    if height < 1u32 {
        height = 1u32
    }
    /* Free old tree and create a new root. */
    layout_free(w);
    (*w).layout_root = layout_create_cell(0 as *mut layout_cell);
    lc = (*w).layout_root;
    sx = width
        .wrapping_add(1u32)
        .wrapping_mul(columns)
        .wrapping_sub(1u32);
    if sx < (*w).sx {
        sx = (*w).sx
    }
    sy = height
        .wrapping_add(1u32)
        .wrapping_mul(rows)
        .wrapping_sub(1u32);
    if sy < (*w).sy {
        sy = (*w).sy
    }
    layout_set_size(lc, sx, sy, 0u32, 0u32);
    layout_make_node(lc, LAYOUT_TOPBOTTOM);
    /* Create a grid of the cells. */
    wp = (*w).panes.tqh_first;
    j = 0u32;
    while j < rows {
        /* If this is the last cell, all done. */
        if wp.is_null() {
            break;
        }
        /* Create the new row. */
        lcrow = layout_create_cell(lc);
        layout_set_size(lcrow, (*w).sx, height, 0u32, 0u32);
        (*lcrow).entry.tqe_next = 0 as *mut layout_cell;
        (*lcrow).entry.tqe_prev = (*lc).cells.tqh_last;
        *(*lc).cells.tqh_last = lcrow;
        (*lc).cells.tqh_last = &mut (*lcrow).entry.tqe_next;
        /* If only one column, just use the row directly. */
        if n.wrapping_sub(j.wrapping_mul(columns)) == 1u32 || columns == 1u32 {
            layout_make_leaf(lcrow, wp);
            wp = (*wp).entry.tqe_next
        } else {
            /* Add in the columns. */
            layout_make_node(lcrow, LAYOUT_LEFTRIGHT);
            i = 0u32;
            while i < columns {
                /* Create and add a pane cell. */
                lcchild = layout_create_cell(lcrow);
                layout_set_size(lcchild, width, height, 0u32, 0u32);
                layout_make_leaf(lcchild, wp);
                (*lcchild).entry.tqe_next = 0 as *mut layout_cell;
                (*lcchild).entry.tqe_prev = (*lcrow).cells.tqh_last;
                *(*lcrow).cells.tqh_last = lcchild;
                (*lcrow).cells.tqh_last = &mut (*lcchild).entry.tqe_next;
                /* Move to the next cell. */
                wp = (*wp).entry.tqe_next;
                if wp.is_null() {
                    break;
                }
                i = i.wrapping_add(1)
            }
            /*
             * Adjust the row and columns to fit the full width if
             * necessary.
             */
            if i == columns {
                i = i.wrapping_sub(1)
            }
            used = i
                .wrapping_add(1u32)
                .wrapping_mul(width.wrapping_add(1u32))
                .wrapping_sub(1u32);
            if !((*w).sx <= used) {
                lcchild = *(*((*lcrow).cells.tqh_last as *mut layout_cells)).tqh_last;
                layout_resize_adjust(
                    w,
                    lcchild,
                    LAYOUT_LEFTRIGHT,
                    (*w).sx.wrapping_sub(used) as libc::c_int,
                );
            }
        }
        j = j.wrapping_add(1)
    }
    /* Adjust the last row height to fit if necessary. */
    used = rows
        .wrapping_mul(height)
        .wrapping_add(rows)
        .wrapping_sub(1u32);
    if (*w).sy > used {
        lcrow = *(*((*lc).cells.tqh_last as *mut layout_cells)).tqh_last;
        layout_resize_adjust(
            w,
            lcrow,
            LAYOUT_TOPBOTTOM,
            (*w).sy.wrapping_sub(used) as libc::c_int,
        );
    }
    /* Fix cell offsets. */
    layout_fix_offsets(w);
    layout_fix_panes(w);
    layout_print_cell(
        (*w).layout_root,
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"layout_set_tiled\x00"))
            .as_ptr(),
        1u32,
    );
    window_resize(w, (*lc).sx, (*lc).sy, -(1i32), -(1i32));
    notify_window(
        b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
        w,
    );
    server_redraw_window(w);
}
