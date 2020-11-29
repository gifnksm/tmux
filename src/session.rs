use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_initialized(ev: *const event) -> libc::c_int;
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
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn notify_session(_: *const libc::c_char, _: *mut session);
    #[no_mangle]
    fn notify_session_window(_: *const libc::c_char, _: *mut session, _: *mut window);
    #[no_mangle]
    fn options_free(_: *mut crate::options::options);
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn environ_free(_: *mut crate::environ::environ);
    #[no_mangle]
    fn tty_update_window_offset(_: *mut window);
    #[no_mangle]
    fn server_lock_session(_: *mut session);
    #[no_mangle]
    fn status_update_cache(_: *mut session);
    #[no_mangle]
    fn recalculate_sizes();
    #[no_mangle]
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_find_by_window(_: *mut winlinks, _: *mut window) -> *mut winlink;
    #[no_mangle]
    fn winlink_find_by_window_id(_: *mut winlinks, _: u_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_add(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_set_window(_: *mut winlink, _: *mut window);
    #[no_mangle]
    fn winlink_remove(_: *mut winlinks, _: *mut winlink);
    #[no_mangle]
    fn winlink_next(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlink_previous(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlink_stack_push(_: *mut winlink_stack, _: *mut winlink);
    #[no_mangle]
    fn winlink_stack_remove(_: *mut winlink_stack, _: *mut winlink);
    #[no_mangle]
    fn window_update_activity(_: *mut window);
    #[no_mangle]
    fn winlink_clear_flags(_: *mut winlink);
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn utf8_stravis(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatal(_: *const libc::c_char, _: ...) -> !;
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
    pub celldata: *mut grid_cell_entry,
    pub extdsize: u_int,
    pub extddata: *mut grid_extd_entry,
    pub flags: libc::c_int,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct grid_extd_entry {
    pub data: crate::utf8::Utf8Char,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}

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
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: C2RustUnnamed_33,
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct session_groups {
    pub rbh_root: *mut session_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sessions {
    pub rbh_root: *mut session,
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
#[no_mangle]
pub static mut sessions: sessions = sessions {
    rbh_root: 0 as *const session as *mut session,
};
static mut next_session_id: u_int = 0;
#[no_mangle]
pub static mut session_groups: session_groups = {
    let mut init = session_groups {
        rbh_root: 0 as *const session_group as *mut session_group,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn session_cmp(mut s1: *mut session, mut s2: *mut session) -> libc::c_int {
    return strcmp((*s1).name, (*s2).name);
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_NEXT(mut elm: *mut session) -> *mut session {
    if !(*elm).entry.rbe_right.is_null() {
        elm = (*elm).entry.rbe_right;
        while !(*elm).entry.rbe_left.is_null() {
            elm = (*elm).entry.rbe_left
        }
    } else if !(*elm).entry.rbe_parent.is_null() && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
    {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null()
            && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
        {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_REMOVE_COLOR(
    mut head: *mut sessions,
    mut parent: *mut session,
    mut elm: *mut session,
) {
    let mut tmp: *mut session = 0 as *mut session;
    while (elm.is_null() || (*elm).entry.rbe_color == 0 as libc::c_int) && elm != (*head).rbh_root {
        if (*parent).entry.rbe_left == elm {
            tmp = (*parent).entry.rbe_right;
            if (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*parent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                tmp = (*parent).entry.rbe_right
            }
            if ((*tmp).entry.rbe_left.is_null()
                || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int)
            {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int
                {
                    let mut oleft: *mut session = 0 as *mut session;
                    oleft = (*tmp).entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).entry.rbe_color = 0 as libc::c_int
                    }
                    (*tmp).entry.rbe_color = 1 as libc::c_int;
                    oleft = (*tmp).entry.rbe_left;
                    (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                    if !(*tmp).entry.rbe_left.is_null() {
                        (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp
                    }
                    (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oleft).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft
                        }
                    } else {
                        (*head).rbh_root = oleft
                    }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    !(*oleft).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_right
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !(*tmp).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_color = 0 as libc::c_int
                }
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*parent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).entry.rbe_left;
            if (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*parent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                tmp = (*parent).entry.rbe_left
            }
            if ((*tmp).entry.rbe_left.is_null()
                || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int)
            {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_left.is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int
                {
                    let mut oright: *mut session = 0 as *mut session;
                    oright = (*tmp).entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).entry.rbe_color = 0 as libc::c_int
                    }
                    (*tmp).entry.rbe_color = 1 as libc::c_int;
                    oright = (*tmp).entry.rbe_right;
                    (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                    if !(*tmp).entry.rbe_right.is_null() {
                        (*(*oright).entry.rbe_left).entry.rbe_parent = tmp
                    }
                    (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oright).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oright
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oright
                        }
                    } else {
                        (*head).rbh_root = oright
                    }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    !(*oright).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_left
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !(*tmp).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_color = 0 as libc::c_int
                }
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*parent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).entry.rbe_color = 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_REMOVE(
    mut head: *mut sessions,
    mut elm: *mut session,
) -> *mut session {
    let mut current_block: u64;
    let mut child: *mut session = 0 as *mut session;
    let mut parent: *mut session = 0 as *mut session;
    let mut old: *mut session = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut session = 0 as *mut session;
        elm = (*elm).entry.rbe_right;
        loop {
            left = (*elm).entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() {
            (*child).entry.rbe_parent = parent
        }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child
            } else {
                (*parent).entry.rbe_right = child
            }
        } else {
            (*head).rbh_root = child
        }
        if (*elm).entry.rbe_parent == old {
            parent = elm
        }
        (*elm).entry = (*old).entry;
        if !(*old).entry.rbe_parent.is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm
            } else {
                (*(*old).entry.rbe_parent).entry.rbe_right = elm
            }
        } else {
            (*head).rbh_root = elm
        }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !(*old).entry.rbe_right.is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 10282816031538985721;
    }
    match current_block {
        7226443171521532240 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() {
                (*child).entry.rbe_parent = parent
            }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child
                } else {
                    (*parent).entry.rbe_right = child
                }
            } else {
                (*head).rbh_root = child
            }
        }
        _ => {}
    }
    if color == 0 as libc::c_int {
        sessions_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_INSERT(
    mut head: *mut sessions,
    mut elm: *mut session,
) -> *mut session {
    let mut tmp: *mut session = 0 as *mut session;
    let mut parent: *mut session = 0 as *mut session;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = session_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut session;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).entry.rbe_left = elm
        } else {
            (*parent).entry.rbe_right = elm
        }
    } else {
        (*head).rbh_root = elm
    }
    sessions_RB_INSERT_COLOR(head, elm);
    return 0 as *mut session;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_FIND(
    mut head: *mut sessions,
    mut elm: *mut session,
) -> *mut session {
    let mut tmp: *mut session = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = session_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut session;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_INSERT_COLOR(mut head: *mut sessions, mut elm: *mut session) {
    let mut parent: *mut session = 0 as *mut session;
    let mut gparent: *mut session = 0 as *mut session;
    let mut tmp: *mut session = 0 as *mut session;
    loop {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                elm = gparent
            } else {
                if (*parent).entry.rbe_right == elm {
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !(*parent).entry.rbe_right.is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_left;
                (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*gparent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = gparent
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_right = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
            }
        } else {
            tmp = (*gparent).entry.rbe_left;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                elm = gparent
            } else {
                if (*parent).entry.rbe_left == elm {
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !(*parent).entry.rbe_left.is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_right;
                (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*gparent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = gparent
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_left = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_PREV(mut elm: *mut session) -> *mut session {
    if !(*elm).entry.rbe_left.is_null() {
        elm = (*elm).entry.rbe_left;
        while !(*elm).entry.rbe_right.is_null() {
            elm = (*elm).entry.rbe_right
        }
    } else if !(*elm).entry.rbe_parent.is_null()
        && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
    {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
        {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_NFIND(
    mut head: *mut sessions,
    mut elm: *mut session,
) -> *mut session {
    let mut tmp: *mut session = (*head).rbh_root;
    let mut res: *mut session = 0 as *mut session;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = session_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            res = tmp;
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_RB_MINMAX(
    mut head: *mut sessions,
    mut val: libc::c_int,
) -> *mut session {
    let mut tmp: *mut session = (*head).rbh_root;
    let mut parent: *mut session = 0 as *mut session;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else {
            tmp = (*tmp).entry.rbe_right
        }
    }
    return parent;
}
unsafe extern "C" fn session_group_cmp(
    mut s1: *mut session_group,
    mut s2: *mut session_group,
) -> libc::c_int {
    return strcmp((*s1).name, (*s2).name);
}
unsafe extern "C" fn session_groups_RB_INSERT_COLOR(
    mut head: *mut session_groups,
    mut elm: *mut session_group,
) {
    let mut parent: *mut session_group = 0 as *mut session_group;
    let mut gparent: *mut session_group = 0 as *mut session_group;
    let mut tmp: *mut session_group = 0 as *mut session_group;
    loop {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                elm = gparent
            } else {
                if (*parent).entry.rbe_right == elm {
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !(*parent).entry.rbe_right.is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_left;
                (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*gparent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = gparent
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_right = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
            }
        } else {
            tmp = (*gparent).entry.rbe_left;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                elm = gparent
            } else {
                if (*parent).entry.rbe_left == elm {
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !(*parent).entry.rbe_left.is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                        }
                    } else {
                        (*head).rbh_root = tmp
                    }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_right;
                (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*gparent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = gparent
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_left = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn session_groups_RB_INSERT(
    mut head: *mut session_groups,
    mut elm: *mut session_group,
) -> *mut session_group {
    let mut tmp: *mut session_group = 0 as *mut session_group;
    let mut parent: *mut session_group = 0 as *mut session_group;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = session_group_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut session_group;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).entry.rbe_left = elm
        } else {
            (*parent).entry.rbe_right = elm
        }
    } else {
        (*head).rbh_root = elm
    }
    session_groups_RB_INSERT_COLOR(head, elm);
    return 0 as *mut session_group;
}
unsafe extern "C" fn session_groups_RB_FIND(
    mut head: *mut session_groups,
    mut elm: *mut session_group,
) -> *mut session_group {
    let mut tmp: *mut session_group = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = session_group_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut session_group;
}
unsafe extern "C" fn session_groups_RB_REMOVE_COLOR(
    mut head: *mut session_groups,
    mut parent: *mut session_group,
    mut elm: *mut session_group,
) {
    let mut tmp: *mut session_group = 0 as *mut session_group;
    while (elm.is_null() || (*elm).entry.rbe_color == 0 as libc::c_int) && elm != (*head).rbh_root {
        if (*parent).entry.rbe_left == elm {
            tmp = (*parent).entry.rbe_right;
            if (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*parent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                tmp = (*parent).entry.rbe_right
            }
            if ((*tmp).entry.rbe_left.is_null()
                || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int)
            {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int
                {
                    let mut oleft: *mut session_group = 0 as *mut session_group;
                    oleft = (*tmp).entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).entry.rbe_color = 0 as libc::c_int
                    }
                    (*tmp).entry.rbe_color = 1 as libc::c_int;
                    oleft = (*tmp).entry.rbe_left;
                    (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                    if !(*tmp).entry.rbe_left.is_null() {
                        (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp
                    }
                    (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oleft).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft
                        }
                    } else {
                        (*head).rbh_root = oleft
                    }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    !(*oleft).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_right
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !(*tmp).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_color = 0 as libc::c_int
                }
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*parent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).entry.rbe_left;
            if (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*parent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                tmp = (*parent).entry.rbe_left
            }
            if ((*tmp).entry.rbe_left.is_null()
                || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int)
            {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_left.is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int
                {
                    let mut oright: *mut session_group = 0 as *mut session_group;
                    oright = (*tmp).entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).entry.rbe_color = 0 as libc::c_int
                    }
                    (*tmp).entry.rbe_color = 1 as libc::c_int;
                    oright = (*tmp).entry.rbe_right;
                    (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                    if !(*tmp).entry.rbe_right.is_null() {
                        (*(*oright).entry.rbe_left).entry.rbe_parent = tmp
                    }
                    (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !(*oright).entry.rbe_parent.is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oright
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oright
                        }
                    } else {
                        (*head).rbh_root = oright
                    }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    !(*oright).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_left
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !(*tmp).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_color = 0 as libc::c_int
                }
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*parent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else {
                    (*head).rbh_root = tmp
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).entry.rbe_color = 0 as libc::c_int
    };
}
unsafe extern "C" fn session_groups_RB_NEXT(mut elm: *mut session_group) -> *mut session_group {
    if !(*elm).entry.rbe_right.is_null() {
        elm = (*elm).entry.rbe_right;
        while !(*elm).entry.rbe_left.is_null() {
            elm = (*elm).entry.rbe_left
        }
    } else if !(*elm).entry.rbe_parent.is_null() && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
    {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null()
            && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
        {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
unsafe extern "C" fn session_groups_RB_MINMAX(
    mut head: *mut session_groups,
    mut val: libc::c_int,
) -> *mut session_group {
    let mut tmp: *mut session_group = (*head).rbh_root;
    let mut parent: *mut session_group = 0 as *mut session_group;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else {
            tmp = (*tmp).entry.rbe_right
        }
    }
    return parent;
}
unsafe extern "C" fn session_groups_RB_REMOVE(
    mut head: *mut session_groups,
    mut elm: *mut session_group,
) -> *mut session_group {
    let mut current_block: u64;
    let mut child: *mut session_group = 0 as *mut session_group;
    let mut parent: *mut session_group = 0 as *mut session_group;
    let mut old: *mut session_group = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut session_group = 0 as *mut session_group;
        elm = (*elm).entry.rbe_right;
        loop {
            left = (*elm).entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() {
            (*child).entry.rbe_parent = parent
        }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child
            } else {
                (*parent).entry.rbe_right = child
            }
        } else {
            (*head).rbh_root = child
        }
        if (*elm).entry.rbe_parent == old {
            parent = elm
        }
        (*elm).entry = (*old).entry;
        if !(*old).entry.rbe_parent.is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm
            } else {
                (*(*old).entry.rbe_parent).entry.rbe_right = elm
            }
        } else {
            (*head).rbh_root = elm
        }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !(*old).entry.rbe_right.is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 17001369507561196147;
    }
    match current_block {
        7226443171521532240 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() {
                (*child).entry.rbe_parent = parent
            }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child
                } else {
                    (*parent).entry.rbe_right = child
                }
            } else {
                (*head).rbh_root = child
            }
        }
        _ => {}
    }
    if color == 0 as libc::c_int {
        session_groups_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
/*
 * Find if session is still alive. This is true if it is still on the global
 * sessions list.
 */
#[no_mangle]
pub unsafe extern "C" fn session_alive(mut s: *mut session) -> libc::c_int {
    let mut s_loop: *mut session = 0 as *mut session;
    s_loop = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s_loop.is_null() {
        if s_loop == s {
            return 1 as libc::c_int;
        }
        s_loop = sessions_RB_NEXT(s_loop)
    }
    return 0 as libc::c_int;
}
/* Find session by name. */
#[no_mangle]
pub unsafe extern "C" fn session_find(mut name: *const libc::c_char) -> *mut session {
    let mut s: session = session {
        id: 0,
        name: 0 as *mut libc::c_char,
        cwd: 0 as *const libc::c_char,
        creation_time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        last_attached_time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        activity_time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        last_activity_time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        lock_timer: event {
            ev_evcallback: event_callback {
                evcb_active_next: C2RustUnnamed_7 {
                    tqe_next: 0 as *mut event_callback,
                    tqe_prev: 0 as *mut *mut event_callback,
                },
                evcb_flags: 0,
                evcb_pri: 0,
                evcb_closure: 0,
                evcb_cb_union: C2RustUnnamed_6 {
                    evcb_callback: None,
                },
                evcb_arg: 0 as *mut libc::c_void,
            },
            ev_timeout_pos: C2RustUnnamed_4 {
                ev_next_with_common_timeout: C2RustUnnamed_5 {
                    tqe_next: 0 as *mut event,
                    tqe_prev: 0 as *mut *mut event,
                },
            },
            ev_fd: 0,
            ev_base: 0 as *mut event_base,
            ev_: C2RustUnnamed {
                ev_io: C2RustUnnamed_2 {
                    ev_io_next: C2RustUnnamed_3 {
                        le_next: 0 as *mut event,
                        le_prev: 0 as *mut *mut event,
                    },
                    ev_timeout: timeval {
                        tv_sec: 0,
                        tv_usec: 0,
                    },
                },
            },
            ev_events: 0,
            ev_res: 0,
            ev_timeout: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        },
        curw: 0 as *mut winlink,
        lastw: winlink_stack {
            tqh_first: 0 as *mut winlink,
            tqh_last: 0 as *mut *mut winlink,
        },
        windows: winlinks {
            rbh_root: 0 as *mut winlink,
        },
        statusat: 0,
        statuslines: 0,
        options: 0 as *mut crate::options::options,
        flags: 0,
        attached: 0,
        tio: 0 as *mut termios,
        environ: 0 as *mut crate::environ::environ,
        references: 0,
        gentry: C2RustUnnamed_13 {
            tqe_next: 0 as *mut session,
            tqe_prev: 0 as *mut *mut session,
        },
        entry: C2RustUnnamed_12 {
            rbe_left: 0 as *mut session,
            rbe_right: 0 as *mut session,
            rbe_parent: 0 as *mut session,
            rbe_color: 0,
        },
    };
    s.name = name as *mut libc::c_char;
    return sessions_RB_FIND(&mut sessions, &mut s);
}
/* Find session by id parsed from a string. */
#[no_mangle]
pub unsafe extern "C" fn session_find_by_id_str(mut s: *const libc::c_char) -> *mut session {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut id: u_int = 0;
    if *s as libc::c_int != '$' as i32 {
        return 0 as *mut session;
    }
    id = strtonum(
        s.offset(1 as libc::c_int as isize),
        0 as libc::c_int as libc::c_longlong,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_longlong,
        &mut errstr,
    ) as u_int;
    if !errstr.is_null() {
        return 0 as *mut session;
    }
    return session_find_by_id(id);
}
/* Find session by id. */
#[no_mangle]
pub unsafe extern "C" fn session_find_by_id(mut id: u_int) -> *mut session {
    let mut s: *mut session = 0 as *mut session;
    s = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s.is_null() {
        if (*s).id == id {
            return s;
        }
        s = sessions_RB_NEXT(s)
    }
    return 0 as *mut session;
}
/* Create a new session. */
#[no_mangle]
pub unsafe extern "C" fn session_create(
    mut prefix: *const libc::c_char,
    mut name: *const libc::c_char,
    mut cwd: *const libc::c_char,
    mut env: *mut crate::environ::environ,
    mut oo: *mut crate::options::options,
    mut tio: *mut termios,
) -> *mut session {
    let mut s: *mut session = 0 as *mut session;
    s = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<session>() as libc::c_ulong,
    ) as *mut session;
    (*s).references = 1 as libc::c_int;
    (*s).flags = 0 as libc::c_int;
    (*s).cwd = xstrdup(cwd);
    (*s).lastw.tqh_first = 0 as *mut winlink;
    (*s).lastw.tqh_last = &mut (*s).lastw.tqh_first;
    (*s).windows.rbh_root = 0 as *mut winlink;
    (*s).environ = env;
    (*s).options = oo;
    status_update_cache(s);
    (*s).tio = 0 as *mut termios;
    if !tio.is_null() {
        (*s).tio = xmalloc(::std::mem::size_of::<termios>() as libc::c_ulong) as *mut termios;
        memcpy(
            (*s).tio as *mut libc::c_void,
            tio as *const libc::c_void,
            ::std::mem::size_of::<termios>() as libc::c_ulong,
        );
    }
    if !name.is_null() {
        (*s).name = xstrdup(name);
        let fresh0 = next_session_id;
        next_session_id = next_session_id.wrapping_add(1);
        (*s).id = fresh0
    } else {
        loop {
            let fresh1 = next_session_id;
            next_session_id = next_session_id.wrapping_add(1);
            (*s).id = fresh1;
            free((*s).name as *mut libc::c_void);
            if !prefix.is_null() {
                xasprintf(
                    &mut (*s).name as *mut *mut libc::c_char,
                    b"%s-%u\x00" as *const u8 as *const libc::c_char,
                    prefix,
                    (*s).id,
                );
            } else {
                xasprintf(
                    &mut (*s).name as *mut *mut libc::c_char,
                    b"%u\x00" as *const u8 as *const libc::c_char,
                    (*s).id,
                );
            }
            if sessions_RB_FIND(&mut sessions, s).is_null() {
                break;
            }
        }
    }
    sessions_RB_INSERT(&mut sessions, s);
    log_debug(
        b"new session %s $%u\x00" as *const u8 as *const libc::c_char,
        (*s).name,
        (*s).id,
    );
    if gettimeofday(&mut (*s).creation_time, 0 as *mut libc::c_void) != 0 as libc::c_int {
        fatal(b"gettimeofday failed\x00" as *const u8 as *const libc::c_char);
    }
    session_update_activity(s, &mut (*s).creation_time);
    return s;
}
/* Add a reference to a session. */
#[no_mangle]
pub unsafe extern "C" fn session_add_ref(mut s: *mut session, mut from: *const libc::c_char) {
    (*s).references += 1;
    log_debug(
        b"%s: %s %s, now %d\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"session_add_ref\x00")).as_ptr(),
        (*s).name,
        from,
        (*s).references,
    );
}
/* Remove a reference from a session. */
#[no_mangle]
pub unsafe extern "C" fn session_remove_ref(mut s: *mut session, mut from: *const libc::c_char) {
    (*s).references -= 1;
    log_debug(
        b"%s: %s %s, now %d\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"session_remove_ref\x00"))
            .as_ptr(),
        (*s).name,
        from,
        (*s).references,
    );
    if (*s).references == 0 as libc::c_int {
        event_once(
            -(1 as libc::c_int),
            0x1 as libc::c_int as libc::c_short,
            Some(
                session_free
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            s as *mut libc::c_void,
            0 as *const timeval,
        );
    };
}
/* Free session. */
unsafe extern "C" fn session_free(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut s: *mut session = arg as *mut session;
    log_debug(
        b"session %s freed (%d references)\x00" as *const u8 as *const libc::c_char,
        (*s).name,
        (*s).references,
    );
    if (*s).references == 0 as libc::c_int {
        environ_free((*s).environ);
        options_free((*s).options);
        free((*s).name as *mut libc::c_void);
        free(s as *mut libc::c_void);
    };
}
/* Destroy a session. */
#[no_mangle]
pub unsafe extern "C" fn session_destroy(
    mut s: *mut session,
    mut notify: libc::c_int,
    mut from: *const libc::c_char,
) {
    let mut wl: *mut winlink = 0 as *mut winlink;
    log_debug(
        b"session %s destroyed (%s)\x00" as *const u8 as *const libc::c_char,
        (*s).name,
        from,
    );
    (*s).curw = 0 as *mut winlink;
    sessions_RB_REMOVE(&mut sessions, s);
    if notify != 0 {
        notify_session(b"session-closed\x00" as *const u8 as *const libc::c_char, s);
    }
    free((*s).tio as *mut libc::c_void);
    if event_initialized(&mut (*s).lock_timer) != 0 {
        event_del(&mut (*s).lock_timer);
    }
    session_group_remove(s);
    while !(*s).lastw.tqh_first.is_null() {
        winlink_stack_remove(&mut (*s).lastw, (*s).lastw.tqh_first);
    }
    while !(*s).windows.rbh_root.is_null() {
        wl = (*s).windows.rbh_root;
        notify_session_window(
            b"window-unlinked\x00" as *const u8 as *const libc::c_char,
            s,
            (*wl).window,
        );
        winlink_remove(&mut (*s).windows, wl);
    }
    free((*s).cwd as *mut libc::c_void);
    session_remove_ref(
        s,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"session_destroy\x00")).as_ptr(),
    );
}
/* Sanitize session name. */
#[no_mangle]
pub unsafe extern "C" fn session_check_name(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
    copy = xstrdup(name);
    cp = copy;
    while *cp as libc::c_int != '\u{0}' as i32 {
        if *cp as libc::c_int == ':' as i32 || *cp as libc::c_int == '.' as i32 {
            *cp = '_' as i32 as libc::c_char
        }
        cp = cp.offset(1)
    }
    utf8_stravis(
        &mut new_name,
        copy,
        0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int,
    );
    free(copy as *mut libc::c_void);
    return new_name;
}
/* Lock session if it has timed out. */
unsafe extern "C" fn session_lock_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut s: *mut session = arg as *mut session;
    if (*s).attached == 0 as libc::c_int as libc::c_uint {
        return;
    }
    log_debug(
        b"session %s locked, activity time %lld\x00" as *const u8 as *const libc::c_char,
        (*s).name,
        (*s).activity_time.tv_sec as libc::c_longlong,
    );
    server_lock_session(s);
    recalculate_sizes();
}
/* Update activity time. */
#[no_mangle]
pub unsafe extern "C" fn session_update_activity(mut s: *mut session, mut from: *mut timeval) {
    let mut last: *mut timeval = &mut (*s).last_activity_time;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    memcpy(
        last as *mut libc::c_void,
        &mut (*s).activity_time as *mut timeval as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong,
    );
    if from.is_null() {
        gettimeofday(&mut (*s).activity_time, 0 as *mut libc::c_void);
    } else {
        memcpy(
            &mut (*s).activity_time as *mut timeval as *mut libc::c_void,
            from as *const libc::c_void,
            ::std::mem::size_of::<timeval>() as libc::c_ulong,
        );
    }
    log_debug(
        b"session $%u %s activity %lld.%06d (last %lld.%06d)\x00" as *const u8
            as *const libc::c_char,
        (*s).id,
        (*s).name,
        (*s).activity_time.tv_sec as libc::c_longlong,
        (*s).activity_time.tv_usec as libc::c_int,
        (*last).tv_sec as libc::c_longlong,
        (*last).tv_usec as libc::c_int,
    );
    if event_initialized(&mut (*s).lock_timer) != 0 {
        event_del(&mut (*s).lock_timer);
    } else {
        event_set(
            &mut (*s).lock_timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                session_lock_timer
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            s as *mut libc::c_void,
        );
    }
    if (*s).attached != 0 as libc::c_int as libc::c_uint {
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        tv.tv_sec = tv.tv_usec;
        tv.tv_sec = options_get_number(
            (*s).options,
            b"lock-after-time\x00" as *const u8 as *const libc::c_char,
        ) as __time_t;
        if tv.tv_sec != 0 as libc::c_int as libc::c_long {
            event_add(&mut (*s).lock_timer, &mut tv);
        }
    };
}
/* Find the next usable session. */
#[no_mangle]
pub unsafe extern "C" fn session_next_session(mut s: *mut session) -> *mut session {
    let mut s2: *mut session = 0 as *mut session;
    if sessions.rbh_root.is_null() || session_alive(s) == 0 {
        return 0 as *mut session;
    }
    s2 = sessions_RB_NEXT(s);
    if s2.is_null() {
        s2 = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int))
    }
    if s2 == s {
        return 0 as *mut session;
    }
    return s2;
}
/* Find the previous usable session. */
#[no_mangle]
pub unsafe extern "C" fn session_previous_session(mut s: *mut session) -> *mut session {
    let mut s2: *mut session = 0 as *mut session;
    if sessions.rbh_root.is_null() || session_alive(s) == 0 {
        return 0 as *mut session;
    }
    s2 = sessions_RB_PREV(s);
    if s2.is_null() {
        s2 = sessions_RB_MINMAX(&mut sessions, 1 as libc::c_int)
    }
    if s2 == s {
        return 0 as *mut session;
    }
    return s2;
}
/* Attach a window to a session. */
#[no_mangle]
pub unsafe extern "C" fn session_attach(
    mut s: *mut session,
    mut w: *mut window,
    mut idx: libc::c_int,
    mut cause: *mut *mut libc::c_char,
) -> *mut winlink {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlink_add(&mut (*s).windows, idx);
    if wl.is_null() {
        xasprintf(
            cause,
            b"index in use: %d\x00" as *const u8 as *const libc::c_char,
            idx,
        );
        return 0 as *mut winlink;
    }
    (*wl).session = s;
    winlink_set_window(wl, w);
    notify_session_window(
        b"window-linked\x00" as *const u8 as *const libc::c_char,
        s,
        w,
    );
    session_group_synchronize_from(s);
    return wl;
}
/* Detach a window from a session. */
#[no_mangle]
pub unsafe extern "C" fn session_detach(mut s: *mut session, mut wl: *mut winlink) -> libc::c_int {
    if (*s).curw == wl
        && session_last(s) != 0 as libc::c_int
        && session_previous(s, 0 as libc::c_int) != 0 as libc::c_int
    {
        session_next(s, 0 as libc::c_int);
    }
    (*wl).flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int);
    notify_session_window(
        b"window-unlinked\x00" as *const u8 as *const libc::c_char,
        s,
        (*wl).window,
    );
    winlink_stack_remove(&mut (*s).lastw, wl);
    winlink_remove(&mut (*s).windows, wl);
    session_group_synchronize_from(s);
    if (*s).windows.rbh_root.is_null() {
        session_destroy(
            s,
            1 as libc::c_int,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"session_detach\x00"))
                .as_ptr(),
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* Return if session has window. */
#[no_mangle]
pub unsafe extern "C" fn session_has(mut s: *mut session, mut w: *mut window) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if (*wl).session == s {
            return 1 as libc::c_int;
        }
        wl = (*wl).wentry.tqe_next
    }
    return 0 as libc::c_int;
}
/*
 * Return 1 if a window is linked outside this session (not including session
 * groups). The window must be in this session!
 */
#[no_mangle]
pub unsafe extern "C" fn session_is_linked(mut s: *mut session, mut w: *mut window) -> libc::c_int {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_contains(s);
    if !sg.is_null() {
        return ((*w).references != session_group_count(sg)) as libc::c_int;
    }
    return ((*w).references != 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn session_next_alert(mut wl: *mut winlink) -> *mut winlink {
    while !wl.is_null() {
        if (*wl).flags & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) != 0 {
            break;
        }
        wl = winlink_next(wl)
    }
    return wl;
}
/* Move session to next window. */
#[no_mangle]
pub unsafe extern "C" fn session_next(mut s: *mut session, mut alert: libc::c_int) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if (*s).curw.is_null() {
        return -(1 as libc::c_int);
    }
    wl = winlink_next((*s).curw);
    if alert != 0 {
        wl = session_next_alert(wl)
    }
    if wl.is_null() {
        wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
        if alert != 0 && {
            wl = session_next_alert(wl);
            wl.is_null()
        } {
            return -(1 as libc::c_int);
        }
    }
    return session_set_current(s, wl);
}
unsafe extern "C" fn session_previous_alert(mut wl: *mut winlink) -> *mut winlink {
    while !wl.is_null() {
        if (*wl).flags & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) != 0 {
            break;
        }
        wl = winlink_previous(wl)
    }
    return wl;
}
/* Move session to previous window. */
#[no_mangle]
pub unsafe extern "C" fn session_previous(
    mut s: *mut session,
    mut alert: libc::c_int,
) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if (*s).curw.is_null() {
        return -(1 as libc::c_int);
    }
    wl = winlink_previous((*s).curw);
    if alert != 0 {
        wl = session_previous_alert(wl)
    }
    if wl.is_null() {
        wl = winlinks_RB_MINMAX(&mut (*s).windows, 1 as libc::c_int);
        if alert != 0 && {
            wl = session_previous_alert(wl);
            wl.is_null()
        } {
            return -(1 as libc::c_int);
        }
    }
    return session_set_current(s, wl);
}
/* Move session to specific window. */
#[no_mangle]
pub unsafe extern "C" fn session_select(mut s: *mut session, mut idx: libc::c_int) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = winlink_find_by_index(&mut (*s).windows, idx);
    return session_set_current(s, wl);
}
/* Move session to last used window. */
#[no_mangle]
pub unsafe extern "C" fn session_last(mut s: *mut session) -> libc::c_int {
    let mut wl: *mut winlink = 0 as *mut winlink;
    wl = (*s).lastw.tqh_first;
    if wl.is_null() {
        return -(1 as libc::c_int);
    }
    if wl == (*s).curw {
        return 1 as libc::c_int;
    }
    return session_set_current(s, wl);
}
/* Set current winlink to wl .*/
#[no_mangle]
pub unsafe extern "C" fn session_set_current(
    mut s: *mut session,
    mut wl: *mut winlink,
) -> libc::c_int {
    if wl.is_null() {
        return -(1 as libc::c_int);
    }
    if wl == (*s).curw {
        return 1 as libc::c_int;
    }
    winlink_stack_remove(&mut (*s).lastw, wl);
    winlink_stack_push(&mut (*s).lastw, (*s).curw);
    (*s).curw = wl;
    winlink_clear_flags(wl);
    window_update_activity((*wl).window);
    tty_update_window_offset((*wl).window);
    notify_session(
        b"session-window-changed\x00" as *const u8 as *const libc::c_char,
        s,
    );
    return 0 as libc::c_int;
}
/* Find the session group containing a session. */
#[no_mangle]
pub unsafe extern "C" fn session_group_contains(mut target: *mut session) -> *mut session_group {
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut s: *mut session = 0 as *mut session;
    sg = session_groups_RB_MINMAX(&mut session_groups, -(1 as libc::c_int));
    while !sg.is_null() {
        s = (*sg).sessions.tqh_first;
        while !s.is_null() {
            if s == target {
                return sg;
            }
            s = (*s).gentry.tqe_next
        }
        sg = session_groups_RB_NEXT(sg)
    }
    return 0 as *mut session_group;
}
/* Find session group by name. */
#[no_mangle]
pub unsafe extern "C" fn session_group_find(mut name: *const libc::c_char) -> *mut session_group {
    let mut sg: session_group = session_group {
        name: 0 as *const libc::c_char,
        sessions: C2RustUnnamed_33 {
            tqh_first: 0 as *mut session,
            tqh_last: 0 as *mut *mut session,
        },
        entry: C2RustUnnamed_32 {
            rbe_left: 0 as *mut session_group,
            rbe_right: 0 as *mut session_group,
            rbe_parent: 0 as *mut session_group,
            rbe_color: 0,
        },
    };
    sg.name = name;
    return session_groups_RB_FIND(&mut session_groups, &mut sg);
}
/* Create a new session group. */
#[no_mangle]
pub unsafe extern "C" fn session_group_new(mut name: *const libc::c_char) -> *mut session_group {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_find(name);
    if !sg.is_null() {
        return sg;
    }
    sg = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<session_group>() as libc::c_ulong,
    ) as *mut session_group;
    (*sg).name = xstrdup(name);
    (*sg).sessions.tqh_first = 0 as *mut session;
    (*sg).sessions.tqh_last = &mut (*sg).sessions.tqh_first;
    session_groups_RB_INSERT(&mut session_groups, sg);
    return sg;
}
/* Add a session to a session group. */
#[no_mangle]
pub unsafe extern "C" fn session_group_add(mut sg: *mut session_group, mut s: *mut session) {
    if session_group_contains(s).is_null() {
        (*s).gentry.tqe_next = 0 as *mut session;
        (*s).gentry.tqe_prev = (*sg).sessions.tqh_last;
        *(*sg).sessions.tqh_last = s;
        (*sg).sessions.tqh_last = &mut (*s).gentry.tqe_next
    };
}
/* Remove a session from its group and destroy the group if empty. */
unsafe extern "C" fn session_group_remove(mut s: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    sg = session_group_contains(s);
    if sg.is_null() {
        return;
    }
    if !(*s).gentry.tqe_next.is_null() {
        (*(*s).gentry.tqe_next).gentry.tqe_prev = (*s).gentry.tqe_prev
    } else {
        (*sg).sessions.tqh_last = (*s).gentry.tqe_prev
    }
    *(*s).gentry.tqe_prev = (*s).gentry.tqe_next;
    if (*sg).sessions.tqh_first.is_null() {
        session_groups_RB_REMOVE(&mut session_groups, sg);
        free((*sg).name as *mut libc::c_void);
        free(sg as *mut libc::c_void);
    };
}
/* Count number of sessions in session group. */
#[no_mangle]
pub unsafe extern "C" fn session_group_count(mut sg: *mut session_group) -> u_int {
    let mut s: *mut session = 0 as *mut session;
    let mut n: u_int = 0;
    n = 0 as libc::c_int as u_int;
    s = (*sg).sessions.tqh_first;
    while !s.is_null() {
        n = n.wrapping_add(1);
        s = (*s).gentry.tqe_next
    }
    return n;
}
/* Count number of clients attached to sessions in session group. */
#[no_mangle]
pub unsafe extern "C" fn session_group_attached_count(mut sg: *mut session_group) -> u_int {
    let mut s: *mut session = 0 as *mut session;
    let mut n: u_int = 0;
    n = 0 as libc::c_int as u_int;
    s = (*sg).sessions.tqh_first;
    while !s.is_null() {
        n = (n as libc::c_uint).wrapping_add((*s).attached) as u_int as u_int;
        s = (*s).gentry.tqe_next
    }
    return n;
}
/* Synchronize a session to its session group. */
#[no_mangle]
pub unsafe extern "C" fn session_group_synchronize_to(mut s: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut target: *mut session = 0 as *mut session;
    sg = session_group_contains(s);
    if sg.is_null() {
        return;
    }
    target = 0 as *mut session;
    target = (*sg).sessions.tqh_first;
    while !target.is_null() {
        if target != s {
            break;
        }
        target = (*target).gentry.tqe_next
    }
    if !target.is_null() {
        session_group_synchronize1(target, s);
    };
}
/* Synchronize a session group to a session. */
#[no_mangle]
pub unsafe extern "C" fn session_group_synchronize_from(mut target: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut s: *mut session = 0 as *mut session;
    sg = session_group_contains(target);
    if sg.is_null() {
        return;
    }
    s = (*sg).sessions.tqh_first;
    while !s.is_null() {
        if s != target {
            session_group_synchronize1(target, s);
        }
        s = (*s).gentry.tqe_next
    }
}
/*
 * Synchronize a session with a target session. This means destroying all
 * winlinks then recreating them, then updating the current window, last window
 * stack and alerts.
 */
unsafe extern "C" fn session_group_synchronize1(mut target: *mut session, mut s: *mut session) {
    let mut old_windows: winlinks = winlinks {
        rbh_root: 0 as *mut winlink,
    };
    let mut ww: *mut winlinks = 0 as *mut winlinks;
    let mut old_lastw: winlink_stack = winlink_stack {
        tqh_first: 0 as *mut winlink,
        tqh_last: 0 as *mut *mut winlink,
    };
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wl2: *mut winlink = 0 as *mut winlink;
    /* Don't do anything if the session is empty (it'll be destroyed). */
    ww = &mut (*target).windows;
    if (*ww).rbh_root.is_null() {
        return;
    }
    /* If the current window has vanished, move to the next now. */
    if !(*s).curw.is_null()
        && winlink_find_by_index(ww, (*(*s).curw).idx).is_null()
        && session_last(s) != 0 as libc::c_int
        && session_previous(s, 0 as libc::c_int) != 0 as libc::c_int
    {
        session_next(s, 0 as libc::c_int);
    }
    /* Save the old pointer and reset it. */
    memcpy(
        &mut old_windows as *mut winlinks as *mut libc::c_void,
        &mut (*s).windows as *mut winlinks as *const libc::c_void,
        ::std::mem::size_of::<winlinks>() as libc::c_ulong,
    );
    (*s).windows.rbh_root = 0 as *mut winlink;
    /* Link all the windows from the target. */
    wl = winlinks_RB_MINMAX(ww, -(1 as libc::c_int));
    while !wl.is_null() {
        wl2 = winlink_add(&mut (*s).windows, (*wl).idx);
        (*wl2).session = s;
        winlink_set_window(wl2, (*wl).window);
        notify_session_window(
            b"window-linked\x00" as *const u8 as *const libc::c_char,
            s,
            (*wl2).window,
        );
        (*wl2).flags |=
            (*wl).flags & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int);
        wl = winlinks_RB_NEXT(wl)
    }
    /* Fix up the current window. */
    if !(*s).curw.is_null() {
        (*s).curw = winlink_find_by_index(&mut (*s).windows, (*(*s).curw).idx)
    } else {
        (*s).curw = winlink_find_by_index(&mut (*s).windows, (*(*target).curw).idx)
    }
    /* Fix up the last window stack. */
    memcpy(
        &mut old_lastw as *mut winlink_stack as *mut libc::c_void,
        &mut (*s).lastw as *mut winlink_stack as *const libc::c_void,
        ::std::mem::size_of::<winlink_stack>() as libc::c_ulong,
    );
    (*s).lastw.tqh_first = 0 as *mut winlink;
    (*s).lastw.tqh_last = &mut (*s).lastw.tqh_first;
    wl = old_lastw.tqh_first;
    while !wl.is_null() {
        wl2 = winlink_find_by_index(&mut (*s).windows, (*wl).idx);
        if !wl2.is_null() {
            (*wl2).sentry.tqe_next = 0 as *mut winlink;
            (*wl2).sentry.tqe_prev = (*s).lastw.tqh_last;
            *(*s).lastw.tqh_last = wl2;
            (*s).lastw.tqh_last = &mut (*wl2).sentry.tqe_next
        }
        wl = (*wl).sentry.tqe_next
    }
    /* Then free the old winlinks list. */
    while !old_windows.rbh_root.is_null() {
        wl = old_windows.rbh_root;
        wl2 = winlink_find_by_window_id(&mut (*s).windows, (*(*wl).window).id);
        if wl2.is_null() {
            notify_session_window(
                b"window-unlinked\x00" as *const u8 as *const libc::c_char,
                s,
                (*wl).window,
            );
        }
        winlink_remove(&mut old_windows, wl);
    }
}
/* Renumber the windows across winlinks attached to a specific session. */
#[no_mangle]
pub unsafe extern "C" fn session_renumber_windows(mut s: *mut session) {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wl1: *mut winlink = 0 as *mut winlink;
    let mut wl_new: *mut winlink = 0 as *mut winlink;
    let mut old_wins: winlinks = winlinks {
        rbh_root: 0 as *mut winlink,
    };
    let mut old_lastw: winlink_stack = winlink_stack {
        tqh_first: 0 as *mut winlink,
        tqh_last: 0 as *mut *mut winlink,
    };
    let mut new_idx: libc::c_int = 0;
    let mut new_curw_idx: libc::c_int = 0;
    /* Save and replace old window list. */
    memcpy(
        &mut old_wins as *mut winlinks as *mut libc::c_void,
        &mut (*s).windows as *mut winlinks as *const libc::c_void,
        ::std::mem::size_of::<winlinks>() as libc::c_ulong,
    );
    (*s).windows.rbh_root = 0 as *mut winlink;
    /* Start renumbering from the base-index if it's set. */
    new_idx = options_get_number(
        (*s).options,
        b"base-index\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    new_curw_idx = 0 as libc::c_int;
    /* Go through the winlinks and assign new indexes. */
    wl = winlinks_RB_MINMAX(&mut old_wins, -(1 as libc::c_int));
    while !wl.is_null() {
        wl_new = winlink_add(&mut (*s).windows, new_idx);
        (*wl_new).session = s;
        winlink_set_window(wl_new, (*wl).window);
        (*wl_new).flags |=
            (*wl).flags & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int);
        if wl == (*s).curw {
            new_curw_idx = (*wl_new).idx
        }
        new_idx += 1;
        wl = winlinks_RB_NEXT(wl)
    }
    /* Fix the stack of last windows now. */
    memcpy(
        &mut old_lastw as *mut winlink_stack as *mut libc::c_void,
        &mut (*s).lastw as *mut winlink_stack as *const libc::c_void,
        ::std::mem::size_of::<winlink_stack>() as libc::c_ulong,
    );
    (*s).lastw.tqh_first = 0 as *mut winlink;
    (*s).lastw.tqh_last = &mut (*s).lastw.tqh_first;
    wl = old_lastw.tqh_first;
    while !wl.is_null() {
        wl_new = winlink_find_by_window(&mut (*s).windows, (*wl).window);
        if !wl_new.is_null() {
            (*wl_new).sentry.tqe_next = 0 as *mut winlink;
            (*wl_new).sentry.tqe_prev = (*s).lastw.tqh_last;
            *(*s).lastw.tqh_last = wl_new;
            (*s).lastw.tqh_last = &mut (*wl_new).sentry.tqe_next
        }
        wl = (*wl).sentry.tqe_next
    }
    /* Set the current window. */
    (*s).curw = winlink_find_by_index(&mut (*s).windows, new_curw_idx);
    /* Free the old winlinks (reducing window references too). */
    wl = winlinks_RB_MINMAX(&mut old_wins, -(1 as libc::c_int));
    while !wl.is_null() && {
        wl1 = winlinks_RB_NEXT(wl);
        (1 as libc::c_int) != 0
    } {
        winlink_remove(&mut old_wins, wl);
        wl = wl1
    }
}
