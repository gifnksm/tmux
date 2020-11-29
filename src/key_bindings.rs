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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list);
    #[no_mangle]
    fn cmd_list_all_have(_: *mut cmd_list, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_parse_from_string(
        _: *const libc::c_char,
        _: *mut cmd_parse_input,
    ) -> *mut cmd_parse_result;
    #[no_mangle]
    fn cmdq_new_state(
        _: *mut cmd_find_state,
        _: *mut key_event,
        _: libc::c_int,
    ) -> *mut crate::cmd_queue::cmdq_state;
    #[no_mangle]
    fn cmdq_free_state(_: *mut crate::cmd_queue::cmdq_state);
    #[no_mangle]
    fn cmdq_get_command(
        _: *mut cmd_list,
        _: *mut crate::cmd_queue::cmdq_state,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_get_callback1(
        _: *const libc::c_char,
        _: cmdq_cb,
        _: *mut libc::c_void,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_insert_after(
        _: *mut crate::cmd_queue::cmdq_item,
        _: *mut crate::cmd_queue::cmdq_item,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_append(
        _: *mut client,
        _: *mut crate::cmd_queue::cmdq_item,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_error(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char);
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
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
pub type cmd_retval = libc::c_int;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
pub type cmd_parse_status = libc::c_uint;
pub const CMD_PARSE_SUCCESS: cmd_parse_status = 2;
pub const CMD_PARSE_ERROR: cmd_parse_status = 1;
pub const CMD_PARSE_EMPTY: cmd_parse_status = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_parse_result {
    pub status: cmd_parse_status,
    pub cmdlist: *mut cmd_list,
    pub error: *mut libc::c_char,
}

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
pub type cmdq_cb = Option<
    unsafe extern "C" fn(_: *mut crate::cmd_queue::cmdq_item, _: *mut libc::c_void) -> cmd_retval,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_tables {
    pub rbh_root: *mut key_table,
}
unsafe extern "C" fn key_bindings_RB_MINMAX(
    mut head: *mut key_bindings,
    mut val: libc::c_int,
) -> *mut key_binding {
    let mut tmp: *mut key_binding = (*head).rbh_root;
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else {
            tmp = (*tmp).entry.rbe_right
        }
    }
    return parent;
}
unsafe extern "C" fn key_bindings_RB_FIND(
    mut head: *mut key_bindings,
    mut elm: *mut key_binding,
) -> *mut key_binding {
    let mut tmp: *mut key_binding = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = key_bindings_cmp(elm, tmp);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut key_binding;
}
unsafe extern "C" fn key_bindings_RB_INSERT(
    mut head: *mut key_bindings,
    mut elm: *mut key_binding,
) -> *mut key_binding {
    let mut tmp: *mut key_binding = 0 as *mut key_binding;
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = key_bindings_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut key_binding;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1i32;
    if !parent.is_null() {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else {
            (*parent).entry.rbe_right = elm
        }
    } else {
        (*head).rbh_root = elm
    }
    key_bindings_RB_INSERT_COLOR(head, elm);
    return 0 as *mut key_binding;
}
unsafe extern "C" fn key_bindings_RB_INSERT_COLOR(
    mut head: *mut key_bindings,
    mut elm: *mut key_binding,
) {
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    let mut gparent: *mut key_binding = 0 as *mut key_binding;
    let mut tmp: *mut key_binding = 0 as *mut key_binding;
    loop {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1i32) {
            break;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
    (*(*head).rbh_root).entry.rbe_color = 0i32;
}
unsafe extern "C" fn key_bindings_RB_NEXT(mut elm: *mut key_binding) -> *mut key_binding {
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
unsafe extern "C" fn key_bindings_RB_REMOVE_COLOR(
    mut head: *mut key_bindings,
    mut parent: *mut key_binding,
    mut elm: *mut key_binding,
) {
    let mut tmp: *mut key_binding = 0 as *mut key_binding;
    while (elm.is_null() || (*elm).entry.rbe_color == 0i32) && elm != (*head).rbh_root {
        if (*parent).entry.rbe_left == elm {
            tmp = (*parent).entry.rbe_right;
            if (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 1i32;
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
            if ((*tmp).entry.rbe_left.is_null() || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32)
            {
                (*tmp).entry.rbe_color = 1i32;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32
                {
                    let mut oleft: *mut key_binding = 0 as *mut key_binding;
                    oleft = (*tmp).entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).entry.rbe_color = 0i32
                    }
                    (*tmp).entry.rbe_color = 1i32;
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
                (*parent).entry.rbe_color = 0i32;
                if !(*tmp).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_color = 0i32
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
            if (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 1i32;
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
            if ((*tmp).entry.rbe_left.is_null() || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32)
            {
                (*tmp).entry.rbe_color = 1i32;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_left.is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32
                {
                    let mut oright: *mut key_binding = 0 as *mut key_binding;
                    oright = (*tmp).entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).entry.rbe_color = 0i32
                    }
                    (*tmp).entry.rbe_color = 1i32;
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
                (*parent).entry.rbe_color = 0i32;
                if !(*tmp).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_color = 0i32
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
        (*elm).entry.rbe_color = 0i32
    };
}
unsafe extern "C" fn key_bindings_RB_REMOVE(
    mut head: *mut key_bindings,
    mut elm: *mut key_binding,
) -> *mut key_binding {
    let mut current_block: u64;
    let mut child: *mut key_binding = 0 as *mut key_binding;
    let mut parent: *mut key_binding = 0 as *mut key_binding;
    let mut old: *mut key_binding = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut key_binding = 0 as *mut key_binding;
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
        current_block = 15424578912580842520;
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
    if color == 0i32 {
        key_bindings_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn key_tables_RB_REMOVE(
    mut head: *mut key_tables,
    mut elm: *mut key_table,
) -> *mut key_table {
    let mut current_block: u64;
    let mut child: *mut key_table = 0 as *mut key_table;
    let mut parent: *mut key_table = 0 as *mut key_table;
    let mut old: *mut key_table = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut key_table = 0 as *mut key_table;
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
        current_block = 14290833548688150372;
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
    if color == 0i32 {
        key_tables_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn key_tables_RB_NEXT(mut elm: *mut key_table) -> *mut key_table {
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
unsafe extern "C" fn key_tables_RB_FIND(
    mut head: *mut key_tables,
    mut elm: *mut key_table,
) -> *mut key_table {
    let mut tmp: *mut key_table = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = key_table_cmp(elm, tmp);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut key_table;
}
unsafe extern "C" fn key_tables_RB_INSERT_COLOR(
    mut head: *mut key_tables,
    mut elm: *mut key_table,
) {
    let mut parent: *mut key_table = 0 as *mut key_table;
    let mut gparent: *mut key_table = 0 as *mut key_table;
    let mut tmp: *mut key_table = 0 as *mut key_table;
    loop {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1i32) {
            break;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
                (*parent).entry.rbe_color = 0i32;
                (*gparent).entry.rbe_color = 1i32;
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
    (*(*head).rbh_root).entry.rbe_color = 0i32;
}
unsafe extern "C" fn key_tables_RB_INSERT(
    mut head: *mut key_tables,
    mut elm: *mut key_table,
) -> *mut key_table {
    let mut tmp: *mut key_table = 0 as *mut key_table;
    let mut parent: *mut key_table = 0 as *mut key_table;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = key_table_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut key_table;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1i32;
    if !parent.is_null() {
        if comp < 0i32 {
            (*parent).entry.rbe_left = elm
        } else {
            (*parent).entry.rbe_right = elm
        }
    } else {
        (*head).rbh_root = elm
    }
    key_tables_RB_INSERT_COLOR(head, elm);
    return 0 as *mut key_table;
}
unsafe extern "C" fn key_tables_RB_MINMAX(
    mut head: *mut key_tables,
    mut val: libc::c_int,
) -> *mut key_table {
    let mut tmp: *mut key_table = (*head).rbh_root;
    let mut parent: *mut key_table = 0 as *mut key_table;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else {
            tmp = (*tmp).entry.rbe_right
        }
    }
    return parent;
}
unsafe extern "C" fn key_tables_RB_REMOVE_COLOR(
    mut head: *mut key_tables,
    mut parent: *mut key_table,
    mut elm: *mut key_table,
) {
    let mut tmp: *mut key_table = 0 as *mut key_table;
    while (elm.is_null() || (*elm).entry.rbe_color == 0i32) && elm != (*head).rbh_root {
        if (*parent).entry.rbe_left == elm {
            tmp = (*parent).entry.rbe_right;
            if (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 1i32;
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
            if ((*tmp).entry.rbe_left.is_null() || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32)
            {
                (*tmp).entry.rbe_color = 1i32;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32
                {
                    let mut oleft: *mut key_table = 0 as *mut key_table;
                    oleft = (*tmp).entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).entry.rbe_color = 0i32
                    }
                    (*tmp).entry.rbe_color = 1i32;
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
                (*parent).entry.rbe_color = 0i32;
                if !(*tmp).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_color = 0i32
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
            if (*tmp).entry.rbe_color == 1i32 {
                (*tmp).entry.rbe_color = 0i32;
                (*parent).entry.rbe_color = 1i32;
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
            if ((*tmp).entry.rbe_left.is_null() || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32)
                && ((*tmp).entry.rbe_right.is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0i32)
            {
                (*tmp).entry.rbe_color = 1i32;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_left.is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == 0i32
                {
                    let mut oright: *mut key_table = 0 as *mut key_table;
                    oright = (*tmp).entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).entry.rbe_color = 0i32
                    }
                    (*tmp).entry.rbe_color = 1i32;
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
                (*parent).entry.rbe_color = 0i32;
                if !(*tmp).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_color = 0i32
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
        (*elm).entry.rbe_color = 0i32
    };
}
static mut key_tables: key_tables = {
    let mut init = key_tables {
        rbh_root: 0 as *mut key_table,
    };
    init
};
unsafe extern "C" fn key_table_cmp(
    mut table1: *mut key_table,
    mut table2: *mut key_table,
) -> libc::c_int {
    return strcmp((*table1).name, (*table2).name);
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
unsafe extern "C" fn key_bindings_cmp(
    mut bd1: *mut key_binding,
    mut bd2: *mut key_binding,
) -> libc::c_int {
    if (*bd1).key < (*bd2).key {
        return -(1i32);
    } /* one reference in key_tables */
    if (*bd1).key > (*bd2).key {
        return 1i32;
    }
    return 0i32;
}
unsafe extern "C" fn key_bindings_free(mut bd: *mut key_binding) {
    cmd_list_free((*bd).cmdlist);
    free((*bd).note as *mut libc::c_void);
    free(bd as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_get_table(
    mut name: *const libc::c_char,
    mut create: libc::c_int,
) -> *mut key_table {
    let mut table_find: key_table = key_table {
        name: 0 as *const libc::c_char,
        key_bindings: key_bindings {
            rbh_root: 0 as *mut key_binding,
        },
        default_key_bindings: key_bindings {
            rbh_root: 0 as *mut key_binding,
        },
        references: 0,
        entry: C2RustUnnamed_26 {
            rbe_left: 0 as *mut key_table,
            rbe_right: 0 as *mut key_table,
            rbe_parent: 0 as *mut key_table,
            rbe_color: 0,
        },
    };
    let mut table: *mut key_table = 0 as *mut key_table;
    table_find.name = name;
    table = key_tables_RB_FIND(&mut key_tables, &mut table_find);
    if !table.is_null() || create == 0 {
        return table;
    }
    table = xmalloc(::std::mem::size_of::<key_table>() as libc::c_ulong) as *mut key_table;
    (*table).name = xstrdup(name);
    (*table).key_bindings.rbh_root = 0 as *mut key_binding;
    (*table).default_key_bindings.rbh_root = 0 as *mut key_binding;
    (*table).references = 1u32;
    key_tables_RB_INSERT(&mut key_tables, table);
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_first_table() -> *mut key_table {
    return key_tables_RB_MINMAX(&mut key_tables, -(1i32));
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_next_table(mut table: *mut key_table) -> *mut key_table {
    return key_tables_RB_NEXT(table);
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_unref_table(mut table: *mut key_table) {
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut bd1: *mut key_binding = 0 as *mut key_binding;
    (*table).references = (*table).references.wrapping_sub(1);
    if (*table).references != 0u32 {
        return;
    }
    bd = key_bindings_RB_MINMAX(&mut (*table).key_bindings, -(1i32));
    while !bd.is_null() && {
        bd1 = key_bindings_RB_NEXT(bd);
        (1i32) != 0
    } {
        key_bindings_RB_REMOVE(&mut (*table).key_bindings, bd);
        key_bindings_free(bd);
        bd = bd1
    }
    bd = key_bindings_RB_MINMAX(&mut (*table).default_key_bindings, -(1i32));
    while !bd.is_null() && {
        bd1 = key_bindings_RB_NEXT(bd);
        (1i32) != 0
    } {
        key_bindings_RB_REMOVE(&mut (*table).default_key_bindings, bd);
        key_bindings_free(bd);
        bd = bd1
    }
    free((*table).name as *mut libc::c_void);
    free(table as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_get(
    mut table: *mut key_table,
    mut key: key_code,
) -> *mut key_binding {
    let mut bd: key_binding = key_binding {
        key: 0,
        cmdlist: 0 as *mut cmd_list,
        note: 0 as *const libc::c_char,
        flags: 0,
        entry: C2RustUnnamed_27 {
            rbe_left: 0 as *mut key_binding,
            rbe_right: 0 as *mut key_binding,
            rbe_parent: 0 as *mut key_binding,
            rbe_color: 0,
        },
    };
    bd.key = key;
    return key_bindings_RB_FIND(&mut (*table).key_bindings, &mut bd);
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_get_default(
    mut table: *mut key_table,
    mut key: key_code,
) -> *mut key_binding {
    let mut bd: key_binding = key_binding {
        key: 0,
        cmdlist: 0 as *mut cmd_list,
        note: 0 as *const libc::c_char,
        flags: 0,
        entry: C2RustUnnamed_27 {
            rbe_left: 0 as *mut key_binding,
            rbe_right: 0 as *mut key_binding,
            rbe_parent: 0 as *mut key_binding,
            rbe_color: 0,
        },
    };
    bd.key = key;
    return key_bindings_RB_FIND(&mut (*table).default_key_bindings, &mut bd);
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_first(mut table: *mut key_table) -> *mut key_binding {
    return key_bindings_RB_MINMAX(&mut (*table).key_bindings, -(1i32));
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_next(
    mut _table: *mut key_table,
    mut bd: *mut key_binding,
) -> *mut key_binding {
    return key_bindings_RB_NEXT(bd);
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_add(
    mut name: *const libc::c_char,
    mut key: key_code,
    mut note: *const libc::c_char,
    mut repeat: libc::c_int,
    mut cmdlist: *mut cmd_list,
) {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    table = key_bindings_get_table(name, 1i32);
    bd = key_bindings_get(table, key & !(0xff000000000000u64));
    if cmdlist.is_null() {
        if !bd.is_null() {
            free((*bd).note as *mut libc::c_void);
            if !note.is_null() {
                (*bd).note = xstrdup(note)
            } else {
                (*bd).note = 0 as *const libc::c_char
            }
        }
        return;
    }
    if !bd.is_null() {
        key_bindings_RB_REMOVE(&mut (*table).key_bindings, bd);
        key_bindings_free(bd);
    }
    bd = xcalloc(1u64, ::std::mem::size_of::<key_binding>() as libc::c_ulong) as *mut key_binding;
    (*bd).key = key & !(0xff000000000000u64);
    if !note.is_null() {
        (*bd).note = xstrdup(note)
    }
    key_bindings_RB_INSERT(&mut (*table).key_bindings, bd);
    if repeat != 0 {
        (*bd).flags |= 0x1i32
    }
    (*bd).cmdlist = cmdlist;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_remove(mut name: *const libc::c_char, mut key: key_code) {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    table = key_bindings_get_table(name, 0i32);
    if table.is_null() {
        return;
    }
    bd = key_bindings_get(table, key & !(0xff000000000000u64));
    if bd.is_null() {
        return;
    }
    key_bindings_RB_REMOVE(&mut (*table).key_bindings, bd);
    key_bindings_free(bd);
    if (*table).key_bindings.rbh_root.is_null() && (*table).default_key_bindings.rbh_root.is_null()
    {
        key_tables_RB_REMOVE(&mut key_tables, table);
        key_bindings_unref_table(table);
    };
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_reset(mut name: *const libc::c_char, mut key: key_code) {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut dd: *mut key_binding = 0 as *mut key_binding;
    table = key_bindings_get_table(name, 0i32);
    if table.is_null() {
        return;
    }
    bd = key_bindings_get(table, key & !(0xff000000000000u64));
    if bd.is_null() {
        return;
    }
    dd = key_bindings_get_default(table, (*bd).key);
    if dd.is_null() {
        key_bindings_remove(name, (*bd).key);
        return;
    }
    cmd_list_free((*bd).cmdlist);
    (*bd).cmdlist = (*dd).cmdlist;
    (*(*bd).cmdlist).references += 1;
    free((*bd).note as *mut libc::c_void);
    if !(*dd).note.is_null() {
        (*bd).note = xstrdup((*dd).note)
    } else {
        (*bd).note = 0 as *const libc::c_char
    }
    (*bd).flags = (*dd).flags;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_remove_table(mut name: *const libc::c_char) {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut c: *mut client = 0 as *mut client;
    table = key_bindings_get_table(name, 0i32);
    if !table.is_null() {
        key_tables_RB_REMOVE(&mut key_tables, table);
        key_bindings_unref_table(table);
    }
    c = clients.tqh_first;
    while !c.is_null() {
        if (*c).keytable == table {
            server_client_set_key_table(c, 0 as *const libc::c_char);
        }
        c = (*c).entry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_reset_table(mut name: *const libc::c_char) {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut bd1: *mut key_binding = 0 as *mut key_binding;
    table = key_bindings_get_table(name, 0i32);
    if table.is_null() {
        return;
    }
    if (*table).default_key_bindings.rbh_root.is_null() {
        key_bindings_remove_table(name);
        return;
    }
    bd = key_bindings_RB_MINMAX(&mut (*table).key_bindings, -(1i32));
    while !bd.is_null() && {
        bd1 = key_bindings_RB_NEXT(bd);
        (1i32) != 0
    } {
        key_bindings_reset(name, (*bd).key);
        bd = bd1
    }
}
unsafe extern "C" fn key_bindings_init_done(
    mut _item: *mut crate::cmd_queue::cmdq_item,
    mut _data: *mut libc::c_void,
) -> cmd_retval {
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut new_bd: *mut key_binding = 0 as *mut key_binding;
    table = key_tables_RB_MINMAX(&mut key_tables, -(1i32));
    while !table.is_null() {
        bd = key_bindings_RB_MINMAX(&mut (*table).key_bindings, -(1i32));
        while !bd.is_null() {
            new_bd = xcalloc(1u64, ::std::mem::size_of::<key_binding>() as libc::c_ulong)
                as *mut key_binding;
            (*new_bd).key = (*bd).key;
            if !(*bd).note.is_null() {
                (*new_bd).note = xstrdup((*bd).note)
            }
            (*new_bd).flags = (*bd).flags;
            (*new_bd).cmdlist = (*bd).cmdlist;
            (*(*new_bd).cmdlist).references += 1;
            key_bindings_RB_INSERT(&mut (*table).default_key_bindings, new_bd);
            bd = key_bindings_RB_NEXT(bd)
        }
        table = key_tables_RB_NEXT(table)
    }
    return CMD_RETURN_NORMAL;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_init() {
    static mut defaults: [*const libc::c_char; 253] =
        [b"bind -N \'Send the prefix key\' C-b send-prefix\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Rotate through the panes\' C-o rotate-window\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Suspend the current client\' C-z suspend-client\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Select next layout\' Space next-layout\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Break pane to a new window\' ! break-pane\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Split window vertically\' \'\"\' split-window\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'List all paste buffers\' \'#\' list-buffers\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Rename current session\' \'$\' command-prompt -I\'#S\' \"rename-session -- \'%%\'\"\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Split window horizontally\' % split-window -h\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Kill current window\' & confirm-before -p\"kill-window #W? (y/n)\" kill-window\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Prompt for window index to select\' \"\'\" command-prompt -Wpindex \"select-window -t \':%%\'\"\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Switch to previous client\' ( switch-client -p\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Switch to next client\' ) switch-client -n\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Rename current window\' , command-prompt -I\'#W\' \"rename-window -- \'%%\'\"\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Delete the most recent paste buffer\' - delete-buffer\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Move the current window\' . command-prompt -T \"move-window -t \'%%\'\"\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Describe key binding\' \'/\' command-prompt -kpkey \'list-keys -1N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Select window 0\' 0 select-window -t:=0\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select window 1\' 1 select-window -t:=1\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select window 2\' 2 select-window -t:=2\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select window 3\' 3 select-window -t:=3\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select window 4\' 4 select-window -t:=4\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select window 5\' 5 select-window -t:=5\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select window 6\' 6 select-window -t:=6\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select window 7\' 7 select-window -t:=7\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select window 8\' 8 select-window -t:=8\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select window 9\' 9 select-window -t:=9\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Prompt for a command\' : command-prompt\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Move to the previously active pane\' \\; last-pane\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Choose a paste buffer from a list\' = choose-buffer -Z\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'List key bindings\' ? list-keys -N\x00" as *const u8 as
             *const libc::c_char,
         b"bind -N \'Choose a client from a list\' D choose-client -Z\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Spread panes out evenly\' E select-layout -E\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Switch to the last client\' L switch-client -l\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Clear the marked pane\' M select-pane -M\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Enter copy mode\' [ copy-mode\x00" as *const u8 as
             *const libc::c_char,
         b"bind -N \'Paste the most recent paste buffer\' ] paste-buffer -p\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Create a new window\' c new-window\x00" as *const u8 as
             *const libc::c_char,
         b"bind -N \'Detach the current client\' d detach-client\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Search for a pane\' f command-prompt \"find-window -Z -- \'%%\'\"\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Display window information\' i display-message\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Select the previously current window\' l last-window\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Toggle the marked pane\' m select-pane -m\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Select the next window\' n next-window\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select the next pane\' o select-pane -t:.+\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Customize options\' C customize-mode -Z\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Select the previous window\' p previous-window\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Display pane numbers\' q display-panes\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Redraw the current client\' r refresh-client\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Choose a session from a list\' s choose-tree -Zs\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Show a clock\' t clock-mode\x00" as *const u8 as
             *const libc::c_char,
         b"bind -N \'Choose a window from a list\' w choose-tree -Zw\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Kill the active pane\' x confirm-before -p\"kill-pane #P? (y/n)\" kill-pane\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Zoom the active pane\' z resize-pane -Z\x00" as *const u8
             as *const libc::c_char,
         b"bind -N \'Swap the active pane with the pane above\' \'{\' swap-pane -U\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Swap the active pane with the pane below\' \'}\' swap-pane -D\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Show messages\' \'~\' show-messages\x00" as *const u8 as
             *const libc::c_char,
         b"bind -N \'Enter copy mode and scroll up\' PPage copy-mode -u\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Select the pane above the active pane\' -r Up select-pane -U\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Select the pane below the active pane\' -r Down select-pane -D\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Select the pane to the left of the active pane\' -r Left select-pane -L\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Select the pane to the right of the active pane\' -r Right select-pane -R\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Set the even-horizontal layout\' M-1 select-layout even-horizontal\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Set the even-vertical layout\' M-2 select-layout even-vertical\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Set the main-horizontal layout\' M-3 select-layout main-horizontal\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Set the main-vertical layout\' M-4 select-layout main-vertical\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Select the tiled layout\' M-5 select-layout tiled\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Select the next window with an alert\' M-n next-window -a\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Rotate through the panes in reverse\' M-o rotate-window -D\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Select the previous window with an alert\' M-p previous-window -a\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Move the visible part of the window up\' -r S-Up refresh-client -U 10\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Move the visible part of the window down\' -r S-Down refresh-client -D 10\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Move the visible part of the window left\' -r S-Left refresh-client -L 10\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Move the visible part of the window right\' -r S-Right refresh-client -R 10\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Reset so the visible part of the window follows the cursor\' -r DC refresh-client -c\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Resize the pane up by 5\' -r M-Up resize-pane -U 5\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Resize the pane down by 5\' -r M-Down resize-pane -D 5\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Resize the pane left by 5\' -r M-Left resize-pane -L 5\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Resize the pane right by 5\' -r M-Right resize-pane -R 5\x00"
             as *const u8 as *const libc::c_char,
         b"bind -N \'Resize the pane up\' -r C-Up resize-pane -U\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Resize the pane down\' -r C-Down resize-pane -D\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Resize the pane left\' -r C-Left resize-pane -L\x00" as
             *const u8 as *const libc::c_char,
         b"bind -N \'Resize the pane right\' -r C-Right resize-pane -R\x00" as
             *const u8 as *const libc::c_char,
         b"bind < display-menu -xW -yW -T \'#[align=centre]#{window_index}:#{window_name}\'  \'#{?#{>:#{session_windows},1},,-}Swap Left\' \'l\' {swap-window -t:-1} \'#{?#{>:#{session_windows},1},,-}Swap Right\' \'r\' {swap-window -t:+1} \'#{?pane_marked_set,,-}Swap Marked\' \'s\' {swap-window} \'\' \'Kill\' \'X\' {kill-window} \'Respawn\' \'R\' {respawn-window -k} \'#{?pane_marked,Unmark,Mark}\' \'m\' {select-pane -m} \'Rename\' \'n\' {command-prompt -I \"#W\" \"rename-window -- \'%%\'\"} \'\' \'New After\' \'w\' {new-window -a} \'New At End\' \'W\' {new-window}\x00"
             as *const u8 as *const libc::c_char,
         b"bind > display-menu -xP -yP -T \'#[align=centre]#{pane_index} (#{pane_id})\'  \'#{?#{m/r:(copy|view)-mode,#{pane_mode}},Go To Top,}\' \'<\' {send -X history-top} \'#{?#{m/r:(copy|view)-mode,#{pane_mode}},Go To Bottom,}\' \'>\' {send -X history-bottom} \'\' \'#{?mouse_word,Search For #[underscore]#{=/9/...:mouse_word},}\' \'C-r\' {if -F \'#{?#{m/r:(copy|view)-mode,#{pane_mode}},0,1}\' \'copy-mode -t=\'; send -Xt= search-backward \"#{q:mouse_word}\"} \'#{?mouse_word,Type #[underscore]#{=/9/...:mouse_word},}\' \'C-y\' {copy-mode -q; send-keys -l -- \"#{q:mouse_word}\"} \'#{?mouse_word,Copy #[underscore]#{=/9/...:mouse_word},}\' \'c\' {copy-mode -q; set-buffer -- \"#{q:mouse_word}\"} \'#{?mouse_line,Copy Line,}\' \'l\' {copy-mode -q; set-buffer -- \"#{q:mouse_line}\"} \'\' \'Horizontal Split\' \'h\' {split-window -h} \'Vertical Split\' \'v\' {split-window -v} \'\' \'#{?#{>:#{window_panes},1},,-}Swap Up\' \'u\' {swap-pane -U} \'#{?#{>:#{window_panes},1},,-}Swap Down\' \'d\' {swap-pane -D} \'#{?pane_marked_set,,-}Swap Marked\' \'s\' {swap-pane} \'\' \'Kill\' \'X\' {kill-pane} \'Respawn\' \'R\' {respawn-pane -k} \'#{?pane_marked,Unmark,Mark}\' \'m\' {select-pane -m} \'#{?#{>:#{window_panes},1},,-}#{?window_zoomed_flag,Unzoom,Zoom}\' \'z\' {resize-pane -Z}\x00"
             as *const u8 as *const libc::c_char,
         b"bind -n MouseDown1Pane select-pane -t=\\; send -M\x00" as *const u8
             as *const libc::c_char,
         b"bind -n MouseDrag1Pane if -F \'#{||:#{pane_in_mode},#{mouse_any_flag}}\' { send -M } { copy-mode -M }\x00"
             as *const u8 as *const libc::c_char,
         b"bind -n WheelUpPane if -F \'#{||:#{pane_in_mode},#{mouse_any_flag}}\' { send -M } { copy-mode -e }\x00"
             as *const u8 as *const libc::c_char,
         b"bind -n MouseDown2Pane select-pane -t=\\; if -F \'#{||:#{pane_in_mode},#{mouse_any_flag}}\' { send -M } { paste -p }\x00"
             as *const u8 as *const libc::c_char,
         b"bind -n DoubleClick1Pane select-pane -t=\\; if -F \'#{||:#{pane_in_mode},#{mouse_any_flag}}\' { send -M } { copy-mode -H; send -X select-word; run -d0.3; send -X copy-pipe-and-cancel }\x00"
             as *const u8 as *const libc::c_char,
         b"bind -n TripleClick1Pane select-pane -t=\\; if -F \'#{||:#{pane_in_mode},#{mouse_any_flag}}\' { send -M } { copy-mode -H; send -X select-line; run -d0.3; send -X copy-pipe-and-cancel }\x00"
             as *const u8 as *const libc::c_char,
         b"bind -n MouseDrag1Border resize-pane -M\x00" as *const u8 as
             *const libc::c_char,
         b"bind -n MouseDown1Status select-window -t=\x00" as *const u8 as
             *const libc::c_char,
         b"bind -n WheelDownStatus next-window\x00" as *const u8 as
             *const libc::c_char,
         b"bind -n WheelUpStatus previous-window\x00" as *const u8 as
             *const libc::c_char,
         b"bind -n MouseDown3StatusLeft display-menu -t= -xM -yW -T \'#[align=centre]#{session_name}\'  \'Next\' \'n\' {switch-client -n} \'Previous\' \'p\' {switch-client -p} \'\' \'Renumber\' \'N\' {move-window -r} \'Rename\' \'n\' {command-prompt -I \"#S\" \"rename-session -- \'%%\'\"} \'\' \'New Session\' \'s\' {new-session} \'New Window\' \'w\' {new-window}\x00"
             as *const u8 as *const libc::c_char,
         b"bind -n MouseDown3Status display-menu -t= -xW -yW -T \'#[align=centre]#{window_index}:#{window_name}\'  \'#{?#{>:#{session_windows},1},,-}Swap Left\' \'l\' {swap-window -t:-1} \'#{?#{>:#{session_windows},1},,-}Swap Right\' \'r\' {swap-window -t:+1} \'#{?pane_marked_set,,-}Swap Marked\' \'s\' {swap-window} \'\' \'Kill\' \'X\' {kill-window} \'Respawn\' \'R\' {respawn-window -k} \'#{?pane_marked,Unmark,Mark}\' \'m\' {select-pane -m} \'Rename\' \'n\' {command-prompt -I \"#W\" \"rename-window -- \'%%\'\"} \'\' \'New After\' \'w\' {new-window -a} \'New At End\' \'W\' {new-window}\x00"
             as *const u8 as *const libc::c_char,
         b"bind -n MouseDown3Pane if -Ft= \'#{||:#{mouse_any_flag},#{&&:#{pane_in_mode},#{?#{m/r:(copy|view)-mode,#{pane_mode}},0,1}}}\' { select-pane -t=; send -M } { display-menu -t= -xM -yM -T \'#[align=centre]#{pane_index} (#{pane_id})\'  \'#{?#{m/r:(copy|view)-mode,#{pane_mode}},Go To Top,}\' \'<\' {send -X history-top} \'#{?#{m/r:(copy|view)-mode,#{pane_mode}},Go To Bottom,}\' \'>\' {send -X history-bottom} \'\' \'#{?mouse_word,Search For #[underscore]#{=/9/...:mouse_word},}\' \'C-r\' {if -F \'#{?#{m/r:(copy|view)-mode,#{pane_mode}},0,1}\' \'copy-mode -t=\'; send -Xt= search-backward \"#{q:mouse_word}\"} \'#{?mouse_word,Type #[underscore]#{=/9/...:mouse_word},}\' \'C-y\' {copy-mode -q; send-keys -l -- \"#{q:mouse_word}\"} \'#{?mouse_word,Copy #[underscore]#{=/9/...:mouse_word},}\' \'c\' {copy-mode -q; set-buffer -- \"#{q:mouse_word}\"} \'#{?mouse_line,Copy Line,}\' \'l\' {copy-mode -q; set-buffer -- \"#{q:mouse_line}\"} \'\' \'Horizontal Split\' \'h\' {split-window -h} \'Vertical Split\' \'v\' {split-window -v} \'\' \'#{?#{>:#{window_panes},1},,-}Swap Up\' \'u\' {swap-pane -U} \'#{?#{>:#{window_panes},1},,-}Swap Down\' \'d\' {swap-pane -D} \'#{?pane_marked_set,,-}Swap Marked\' \'s\' {swap-pane} \'\' \'Kill\' \'X\' {kill-pane} \'Respawn\' \'R\' {respawn-pane -k} \'#{?pane_marked,Unmark,Mark}\' \'m\' {select-pane -m} \'#{?#{>:#{window_panes},1},,-}#{?window_zoomed_flag,Unzoom,Zoom}\' \'z\' {resize-pane -Z} }\x00"
             as *const u8 as *const libc::c_char,
         b"bind -n M-MouseDown3Pane display-menu -t= -xM -yM -T \'#[align=centre]#{pane_index} (#{pane_id})\'  \'#{?#{m/r:(copy|view)-mode,#{pane_mode}},Go To Top,}\' \'<\' {send -X history-top} \'#{?#{m/r:(copy|view)-mode,#{pane_mode}},Go To Bottom,}\' \'>\' {send -X history-bottom} \'\' \'#{?mouse_word,Search For #[underscore]#{=/9/...:mouse_word},}\' \'C-r\' {if -F \'#{?#{m/r:(copy|view)-mode,#{pane_mode}},0,1}\' \'copy-mode -t=\'; send -Xt= search-backward \"#{q:mouse_word}\"} \'#{?mouse_word,Type #[underscore]#{=/9/...:mouse_word},}\' \'C-y\' {copy-mode -q; send-keys -l -- \"#{q:mouse_word}\"} \'#{?mouse_word,Copy #[underscore]#{=/9/...:mouse_word},}\' \'c\' {copy-mode -q; set-buffer -- \"#{q:mouse_word}\"} \'#{?mouse_line,Copy Line,}\' \'l\' {copy-mode -q; set-buffer -- \"#{q:mouse_line}\"} \'\' \'Horizontal Split\' \'h\' {split-window -h} \'Vertical Split\' \'v\' {split-window -v} \'\' \'#{?#{>:#{window_panes},1},,-}Swap Up\' \'u\' {swap-pane -U} \'#{?#{>:#{window_panes},1},,-}Swap Down\' \'d\' {swap-pane -D} \'#{?pane_marked_set,,-}Swap Marked\' \'s\' {swap-pane} \'\' \'Kill\' \'X\' {kill-pane} \'Respawn\' \'R\' {respawn-pane -k} \'#{?pane_marked,Unmark,Mark}\' \'m\' {select-pane -m} \'#{?#{>:#{window_panes},1},,-}#{?window_zoomed_flag,Unzoom,Zoom}\' \'z\' {resize-pane -Z}\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode C-Space send -X begin-selection\x00" as *const u8
             as *const libc::c_char,
         b"bind -Tcopy-mode C-a send -X start-of-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-c send -X cancel\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-e send -X end-of-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-f send -X cursor-right\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-b send -X cursor-left\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-g send -X clear-selection\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-k send -X copy-end-of-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-n send -X cursor-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-p send -X cursor-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-r command-prompt -ip\'(search up)\' -I\'#{pane_search_string}\' \'send -X search-backward-incremental \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode C-s command-prompt -ip\'(search down)\' -I\'#{pane_search_string}\' \'send -X search-forward-incremental \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode C-v send -X page-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-w send -X copy-pipe-and-cancel\x00" as *const u8
             as *const libc::c_char,
         b"bind -Tcopy-mode Escape send -X cancel\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode Space send -X page-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode , send -X jump-reverse\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode \\; send -X jump-again\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode F command-prompt -1p\'(jump backward)\' \'send -X jump-backward \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode N send -X search-reverse\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode R send -X rectangle-toggle\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode T command-prompt -1p\'(jump to backward)\' \'send -X jump-to-backward \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode X send -X set-mark\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode f command-prompt -1p\'(jump forward)\' \'send -X jump-forward \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode g command-prompt -p\'(goto line)\' \'send -X goto-line \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode n send -X search-again\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode q send -X cancel\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode r send -X refresh-from-pane\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode t command-prompt -1p\'(jump to forward)\' \'send -X jump-to-forward \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode Home send -X start-of-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode End send -X end-of-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode MouseDown1Pane select-pane\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode MouseDrag1Pane select-pane\\; send -X begin-selection\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode MouseDragEnd1Pane send -X copy-pipe-and-cancel\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode WheelUpPane select-pane\\; send -N5 -X scroll-up\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode WheelDownPane select-pane\\; send -N5 -X scroll-down\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode DoubleClick1Pane select-pane\\; send -X select-word\\; run -d0.3\\; send -X copy-pipe-and-cancel\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode TripleClick1Pane select-pane\\; send -X select-line\\; run -d0.3\\; send -X copy-pipe-and-cancel\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode NPage send -X page-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode PPage send -X page-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode Up send -X cursor-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode Down send -X cursor-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode Left send -X cursor-left\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode Right send -X cursor-right\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode M-1 command-prompt -Np\'(repeat)\' -I1 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-2 command-prompt -Np\'(repeat)\' -I2 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-3 command-prompt -Np\'(repeat)\' -I3 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-4 command-prompt -Np\'(repeat)\' -I4 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-5 command-prompt -Np\'(repeat)\' -I5 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-6 command-prompt -Np\'(repeat)\' -I6 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-7 command-prompt -Np\'(repeat)\' -I7 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-8 command-prompt -Np\'(repeat)\' -I8 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-9 command-prompt -Np\'(repeat)\' -I9 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-< send -X history-top\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode M-> send -X history-bottom\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode M-R send -X top-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode M-b send -X previous-word\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-M-b send -X previous-matching-bracket\x00" as
             *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-f send -X next-word-end\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-M-f send -X next-matching-bracket\x00" as
             *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode M-m send -X back-to-indentation\x00" as *const u8
             as *const libc::c_char,
         b"bind -Tcopy-mode M-r send -X middle-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode M-v send -X page-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode M-w send -X copy-pipe-and-cancel\x00" as *const u8
             as *const libc::c_char,
         b"bind -Tcopy-mode M-x send -X jump-to-mark\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode \'M-{\' send -X previous-paragraph\x00" as
             *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode \'M-}\' send -X next-paragraph\x00" as *const u8
             as *const libc::c_char,
         b"bind -Tcopy-mode M-Up send -X halfpage-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode M-Down send -X halfpage-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-Up send -X scroll-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode C-Down send -X scroll-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi \'#\' send -FX search-backward \'#{copy_cursor_word}\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi * send -FX search-forward \'#{copy_cursor_word}\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi C-c send -X cancel\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi C-d send -X halfpage-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi C-e send -X scroll-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi C-b send -X page-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi C-f send -X page-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi C-h send -X cursor-left\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi C-j send -X copy-pipe-and-cancel\x00" as
             *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi Enter send -X copy-pipe-and-cancel\x00" as
             *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi C-u send -X halfpage-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi C-v send -X rectangle-toggle\x00" as *const u8
             as *const libc::c_char,
         b"bind -Tcopy-mode-vi C-y send -X scroll-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi Escape send -X clear-selection\x00" as
             *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi Space send -X begin-selection\x00" as *const u8
             as *const libc::c_char,
         b"bind -Tcopy-mode-vi \'$\' send -X end-of-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi , send -X jump-reverse\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi / command-prompt -p\'(search down)\' \'send -X search-forward \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi 0 send -X start-of-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi 1 command-prompt -Np\'(repeat)\' -I1 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi 2 command-prompt -Np\'(repeat)\' -I2 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi 3 command-prompt -Np\'(repeat)\' -I3 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi 4 command-prompt -Np\'(repeat)\' -I4 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi 5 command-prompt -Np\'(repeat)\' -I5 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi 6 command-prompt -Np\'(repeat)\' -I6 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi 7 command-prompt -Np\'(repeat)\' -I7 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi 8 command-prompt -Np\'(repeat)\' -I8 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi 9 command-prompt -Np\'(repeat)\' -I9 \'send -N \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi : command-prompt -p\'(goto line)\' \'send -X goto-line \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi \\; send -X jump-again\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi ? command-prompt -p\'(search up)\' \'send -X search-backward \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi A send -X append-selection-and-cancel\x00" as
             *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi B send -X previous-space\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi D send -X copy-end-of-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi E send -X next-space-end\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi F command-prompt -1p\'(jump backward)\' \'send -X jump-backward \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi G send -X history-bottom\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi H send -X top-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi J send -X scroll-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi K send -X scroll-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi L send -X bottom-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi M send -X middle-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi N send -X search-reverse\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi T command-prompt -1p\'(jump to backward)\' \'send -X jump-to-backward \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi V send -X select-line\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi W send -X next-space\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi X send -X set-mark\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi ^ send -X back-to-indentation\x00" as *const u8
             as *const libc::c_char,
         b"bind -Tcopy-mode-vi b send -X previous-word\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi e send -X next-word-end\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi f command-prompt -1p\'(jump forward)\' \'send -X jump-forward \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi g send -X history-top\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi h send -X cursor-left\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi j send -X cursor-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi k send -X cursor-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi l send -X cursor-right\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi n send -X search-again\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi o send -X other-end\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi q send -X cancel\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi r send -X refresh-from-pane\x00" as *const u8
             as *const libc::c_char,
         b"bind -Tcopy-mode-vi t command-prompt -1p\'(jump to forward)\' \'send -X jump-to-forward \"%%%\"\'\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi v send -X rectangle-toggle\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi w send -X next-word\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi \'{\' send -X previous-paragraph\x00" as
             *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi \'}\' send -X next-paragraph\x00" as *const u8
             as *const libc::c_char,
         b"bind -Tcopy-mode-vi % send -X next-matching-bracket\x00" as
             *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi MouseDown1Pane select-pane\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi MouseDrag1Pane select-pane\\; send -X begin-selection\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi MouseDragEnd1Pane send -X copy-pipe-and-cancel\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi WheelUpPane select-pane\\; send -N5 -X scroll-up\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi WheelDownPane select-pane\\; send -N5 -X scroll-down\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi DoubleClick1Pane select-pane\\; send -X select-word\\; run -d0.3\\; send -X copy-pipe-and-cancel\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi TripleClick1Pane select-pane\\; send -X select-line\\; run -d0.3\\; send -X copy-pipe-and-cancel\x00"
             as *const u8 as *const libc::c_char,
         b"bind -Tcopy-mode-vi BSpace send -X cursor-left\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi NPage send -X page-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi PPage send -X page-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi Up send -X cursor-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi Down send -X cursor-down\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi Left send -X cursor-left\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi Right send -X cursor-right\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi M-x send -X jump-to-mark\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi C-Up send -X scroll-up\x00" as *const u8 as
             *const libc::c_char,
         b"bind -Tcopy-mode-vi C-Down send -X scroll-down\x00" as *const u8 as
             *const libc::c_char];
    let mut i: u_int = 0;
    let mut pr: *mut cmd_parse_result = 0 as *mut cmd_parse_result;
    i = 0u32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*const libc::c_char; 253]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        pr = cmd_parse_from_string(defaults[i as usize], 0 as *mut cmd_parse_input);
        if (*pr).status != CMD_PARSE_SUCCESS {
            fatalx(
                b"bad default key: %s\x00" as *const u8 as *const libc::c_char,
                defaults[i as usize],
            );
        }
        cmdq_append(
            0 as *mut client,
            cmdq_get_command((*pr).cmdlist, 0 as *mut crate::cmd_queue::cmdq_state),
        );
        cmd_list_free((*pr).cmdlist);
        i = i.wrapping_add(1)
    }
    cmdq_append(
        0 as *mut client,
        cmdq_get_callback1(
            b"key_bindings_init_done\x00" as *const u8 as *const libc::c_char,
            Some(
                key_bindings_init_done
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd_queue::cmdq_item,
                        _: *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            0 as *mut libc::c_void,
        ),
    );
}
unsafe extern "C" fn key_bindings_read_only(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut _data: *mut libc::c_void,
) -> cmd_retval {
    cmdq_error(
        item,
        b"client is read-only\x00" as *const u8 as *const libc::c_char,
    );
    return CMD_RETURN_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn key_bindings_dispatch(
    mut bd: *mut key_binding,
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut c: *mut client,
    mut event: *mut key_event,
    mut fs: *mut cmd_find_state,
) -> *mut crate::cmd_queue::cmdq_item {
    let mut new_item: *mut crate::cmd_queue::cmdq_item = 0 as *mut crate::cmd_queue::cmdq_item;
    let mut new_state: *mut crate::cmd_queue::cmdq_state = 0 as *mut crate::cmd_queue::cmdq_state;
    let mut readonly: libc::c_int = 0;
    let mut flags: libc::c_int = 0i32;
    if c.is_null() || !(*c).flags & 0x800u64 != 0 {
        readonly = 1i32
    } else {
        readonly = cmd_list_all_have((*bd).cmdlist, 0x2i32)
    }
    if readonly == 0 {
        new_item = cmdq_get_callback1(
            b"key_bindings_read_only\x00" as *const u8 as *const libc::c_char,
            Some(
                key_bindings_read_only
                    as unsafe extern "C" fn(
                        _: *mut crate::cmd_queue::cmdq_item,
                        _: *mut libc::c_void,
                    ) -> cmd_retval,
            ),
            0 as *mut libc::c_void,
        )
    } else {
        if (*bd).flags & 0x1i32 != 0 {
            flags |= 0x1i32
        }
        new_state = cmdq_new_state(fs, event, flags);
        new_item = cmdq_get_command((*bd).cmdlist, new_state);
        cmdq_free_state(new_state);
    }
    if !item.is_null() {
        new_item = cmdq_insert_after(item, new_item)
    } else {
        new_item = cmdq_append(c, new_item)
    }
    return new_item;
}
