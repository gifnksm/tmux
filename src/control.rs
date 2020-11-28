use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_pending(ev: *const event, events: libc::c_short, tv: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn event_initialized(ev: *const event) -> libc::c_int;
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
    fn evbuffer_new() -> *mut evbuffer;
    #[no_mangle]
    fn evbuffer_free(buf: *mut evbuffer);
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_add(buf: *mut evbuffer, data: *const libc::c_void, datlen: size_t) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_new(
        fd: libc::c_int,
        readcb: bufferevent_data_cb,
        writecb: bufferevent_data_cb,
        errorcb: bufferevent_event_cb,
        cbarg: *mut libc::c_void,
    ) -> *mut bufferevent;
    #[no_mangle]
    fn bufferevent_free(bufev: *mut bufferevent);
    #[no_mangle]
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_write_buffer(bufev: *mut bufferevent, buf: *mut evbuffer) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_enable(bufev: *mut bufferevent, event: libc::c_short) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_disable(bufev: *mut bufferevent, event: libc::c_short) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_setwatermark(
        bufev: *mut bufferevent,
        events: libc::c_short,
        lowmark: size_t,
        highmark: size_t,
    );
    #[no_mangle]
    fn evbuffer_add_printf(buf: *mut evbuffer, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_readln(
        buffer: *mut evbuffer,
        n_read_out: *mut size_t,
        eol_style: evbuffer_eol_style,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t) -> *mut libc::c_uchar;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xvasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn setblocking(_: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn get_timer() -> uint64_t;
    #[no_mangle]
    fn format_free(_: *mut crate::format::format_tree);
    #[no_mangle]
    fn format_expand(
        _: *mut crate::format::format_tree,
        _: *const libc::c_char,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn format_create_defaults(
        _: *mut crate::cmd_queue::cmdq_item,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut crate::format::format_tree;
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
    fn cmdq_guard(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: libc::c_int);
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn winlink_find_by_window(_: *mut winlinks, _: *mut window) -> *mut winlink;
    #[no_mangle]
    fn window_find_by_id(_: u_int) -> *mut window;
    #[no_mangle]
    fn cmdq_get_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
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
    fn window_pane_get_new_data(
        _: *mut window_pane,
        _: *mut window_pane_offset,
        _: *mut size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn window_pane_update_used_data(_: *mut window_pane, _: *mut window_pane_offset, _: size_t);
    #[no_mangle]
    fn window_pane_find_by_id(_: u_int) -> *mut window_pane;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
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
pub type evbuffer_eol_style = libc::c_uint;
pub const EVBUFFER_EOL_NUL: evbuffer_eol_style = 4;
pub const EVBUFFER_EOL_LF: evbuffer_eol_style = 3;
pub const EVBUFFER_EOL_CRLF_STRICT: evbuffer_eol_style = 2;
pub const EVBUFFER_EOL_CRLF: evbuffer_eol_style = 1;
pub const EVBUFFER_EOL_ANY: evbuffer_eol_style = 0;

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
    pub control_state: *mut control_state,
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
    pub saved_cell: grid_cell,
    pub saved_flags: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: *mut crate::screen::screen_sel,
    pub write_list: *mut crate::screen_write::screen_write_collect_line,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grid_cell {
    pub data: crate::utf8::Utf8Data,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
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
/* Control client state. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_state {
    pub panes: control_panes,
    pub pending_list: C2RustUnnamed_37,
    pub pending_count: u_int,
    pub all_blocks: C2RustUnnamed_34,
    pub read_event: *mut bufferevent,
    pub write_event: *mut bufferevent,
    pub subs: control_subs,
    pub subs_timer: event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_subs {
    pub rbh_root: *mut control_sub,
}
/* Control client subscription. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_sub {
    pub name: *mut libc::c_char,
    pub format: *mut libc::c_char,
    pub type_0: control_sub_type,
    pub id: u_int,
    pub last: *mut libc::c_char,
    pub panes: control_sub_panes,
    pub windows: control_sub_windows,
    pub entry: C2RustUnnamed_31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_31 {
    pub rbe_left: *mut control_sub,
    pub rbe_right: *mut control_sub,
    pub rbe_parent: *mut control_sub,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_sub_windows {
    pub rbh_root: *mut control_sub_window,
}
/* Subscription window. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_sub_window {
    pub window: u_int,
    pub idx: u_int,
    pub last: *mut libc::c_char,
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub rbe_left: *mut control_sub_window,
    pub rbe_right: *mut control_sub_window,
    pub rbe_parent: *mut control_sub_window,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_sub_panes {
    pub rbh_root: *mut control_sub_pane,
}
/* Subscription pane. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_sub_pane {
    pub pane: u_int,
    pub idx: u_int,
    pub last: *mut libc::c_char,
    pub entry: C2RustUnnamed_33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub rbe_left: *mut control_sub_pane,
    pub rbe_right: *mut control_sub_pane,
    pub rbe_parent: *mut control_sub_pane,
    pub rbe_color: libc::c_int,
}
pub type control_sub_type = libc::c_uint;
pub const CONTROL_SUB_ALL_WINDOWS: control_sub_type = 4;
pub const CONTROL_SUB_WINDOW: control_sub_type = 3;
pub const CONTROL_SUB_ALL_PANES: control_sub_type = 2;
pub const CONTROL_SUB_PANE: control_sub_type = 1;
pub const CONTROL_SUB_SESSION: control_sub_type = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_34 {
    pub tqh_first: *mut control_block,
    pub tqh_last: *mut *mut control_block,
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2012 Nicholas Marriott <nicholas.marriott@gmail.com>
 * Copyright (c) 2012 George Nachman <tmux@georgester.com>
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
 * Block of data to output. Each client has one "all" queue of blocks and
 * another queue for each pane (in struct client_offset). %output blocks are
 * added to both queues and other output lines (notifications) added only to
 * the client queue.
 *
 * When a client becomes writeable, data from blocks on the pane queue are sent
 * up to the maximum size (CLIENT_BUFFER_HIGH). If a block is entirely written,
 * it is removed from both pane and client queues and if this means non-%output
 * blocks are now at the head of the client queue, they are written.
 *
 * This means a %output block holds up any subsequent non-%output blocks until
 * it is written which enforces ordering even if the client cannot accept the
 * entire block in one go.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_block {
    pub size: size_t,
    pub line: *mut libc::c_char,
    pub t: uint64_t,
    pub entry: C2RustUnnamed_36,
    pub all_entry: C2RustUnnamed_35,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_35 {
    pub tqe_next: *mut control_block,
    pub tqe_prev: *mut *mut control_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_36 {
    pub tqe_next: *mut control_block,
    pub tqe_prev: *mut *mut control_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_37 {
    pub tqh_first: *mut control_pane,
    pub tqh_last: *mut *mut control_pane,
}
/* Control client pane. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_pane {
    pub pane: u_int,
    pub offset: window_pane_offset,
    pub queued: window_pane_offset,
    pub flags: libc::c_int,
    pub pending_flag: libc::c_int,
    pub pending_entry: C2RustUnnamed_40,
    pub blocks: C2RustUnnamed_39,
    pub entry: C2RustUnnamed_38,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_38 {
    pub rbe_left: *mut control_pane,
    pub rbe_right: *mut control_pane,
    pub rbe_parent: *mut control_pane,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_39 {
    pub tqh_first: *mut control_block,
    pub tqh_last: *mut *mut control_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_40 {
    pub tqe_next: *mut control_pane,
    pub tqe_prev: *mut *mut control_pane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_panes {
    pub rbh_root: *mut control_pane,
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
    pub entry: C2RustUnnamed_41,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_41 {
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
/* Compare client panes. */
unsafe extern "C" fn control_pane_cmp(
    mut cp1: *mut control_pane,
    mut cp2: *mut control_pane,
) -> libc::c_int {
    if (*cp1).pane < (*cp2).pane {
        return -(1 as libc::c_int);
    }
    if (*cp1).pane > (*cp2).pane {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn control_panes_RB_NEXT(mut elm: *mut control_pane) -> *mut control_pane {
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
unsafe extern "C" fn control_panes_RB_FIND(
    mut head: *mut control_panes,
    mut elm: *mut control_pane,
) -> *mut control_pane {
    let mut tmp: *mut control_pane = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = control_pane_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut control_pane;
}
unsafe extern "C" fn control_panes_RB_INSERT(
    mut head: *mut control_panes,
    mut elm: *mut control_pane,
) -> *mut control_pane {
    let mut tmp: *mut control_pane = 0 as *mut control_pane;
    let mut parent: *mut control_pane = 0 as *mut control_pane;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = control_pane_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut control_pane;
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
    control_panes_RB_INSERT_COLOR(head, elm);
    return 0 as *mut control_pane;
}
unsafe extern "C" fn control_panes_RB_INSERT_COLOR(
    mut head: *mut control_panes,
    mut elm: *mut control_pane,
) {
    let mut parent: *mut control_pane = 0 as *mut control_pane;
    let mut gparent: *mut control_pane = 0 as *mut control_pane;
    let mut tmp: *mut control_pane = 0 as *mut control_pane;
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
unsafe extern "C" fn control_panes_RB_MINMAX(
    mut head: *mut control_panes,
    mut val: libc::c_int,
) -> *mut control_pane {
    let mut tmp: *mut control_pane = (*head).rbh_root;
    let mut parent: *mut control_pane = 0 as *mut control_pane;
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
unsafe extern "C" fn control_panes_RB_REMOVE(
    mut head: *mut control_panes,
    mut elm: *mut control_pane,
) -> *mut control_pane {
    let mut current_block: u64;
    let mut child: *mut control_pane = 0 as *mut control_pane;
    let mut parent: *mut control_pane = 0 as *mut control_pane;
    let mut old: *mut control_pane = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut control_pane = 0 as *mut control_pane;
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
        current_block = 13366267178683865170;
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
        control_panes_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn control_panes_RB_REMOVE_COLOR(
    mut head: *mut control_panes,
    mut parent: *mut control_pane,
    mut elm: *mut control_pane,
) {
    let mut tmp: *mut control_pane = 0 as *mut control_pane;
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
                    let mut oleft: *mut control_pane = 0 as *mut control_pane;
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
                    let mut oright: *mut control_pane = 0 as *mut control_pane;
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
/* Compare client subs. */
unsafe extern "C" fn control_sub_cmp(
    mut csub1: *mut control_sub,
    mut csub2: *mut control_sub,
) -> libc::c_int {
    return strcmp((*csub1).name, (*csub2).name);
}
unsafe extern "C" fn control_subs_RB_NEXT(mut elm: *mut control_sub) -> *mut control_sub {
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
unsafe extern "C" fn control_subs_RB_MINMAX(
    mut head: *mut control_subs,
    mut val: libc::c_int,
) -> *mut control_sub {
    let mut tmp: *mut control_sub = (*head).rbh_root;
    let mut parent: *mut control_sub = 0 as *mut control_sub;
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
unsafe extern "C" fn control_subs_RB_REMOVE_COLOR(
    mut head: *mut control_subs,
    mut parent: *mut control_sub,
    mut elm: *mut control_sub,
) {
    let mut tmp: *mut control_sub = 0 as *mut control_sub;
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
                    let mut oleft: *mut control_sub = 0 as *mut control_sub;
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
                    let mut oright: *mut control_sub = 0 as *mut control_sub;
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
unsafe extern "C" fn control_subs_RB_FIND(
    mut head: *mut control_subs,
    mut elm: *mut control_sub,
) -> *mut control_sub {
    let mut tmp: *mut control_sub = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = control_sub_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut control_sub;
}
unsafe extern "C" fn control_subs_RB_INSERT_COLOR(
    mut head: *mut control_subs,
    mut elm: *mut control_sub,
) {
    let mut parent: *mut control_sub = 0 as *mut control_sub;
    let mut gparent: *mut control_sub = 0 as *mut control_sub;
    let mut tmp: *mut control_sub = 0 as *mut control_sub;
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
unsafe extern "C" fn control_subs_RB_INSERT(
    mut head: *mut control_subs,
    mut elm: *mut control_sub,
) -> *mut control_sub {
    let mut tmp: *mut control_sub = 0 as *mut control_sub;
    let mut parent: *mut control_sub = 0 as *mut control_sub;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = control_sub_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut control_sub;
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
    control_subs_RB_INSERT_COLOR(head, elm);
    return 0 as *mut control_sub;
}
unsafe extern "C" fn control_subs_RB_REMOVE(
    mut head: *mut control_subs,
    mut elm: *mut control_sub,
) -> *mut control_sub {
    let mut current_block: u64;
    let mut child: *mut control_sub = 0 as *mut control_sub;
    let mut parent: *mut control_sub = 0 as *mut control_sub;
    let mut old: *mut control_sub = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut control_sub = 0 as *mut control_sub;
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
        current_block = 7666550289102718160;
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
        control_subs_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
/* Compare client subscription panes. */
unsafe extern "C" fn control_sub_pane_cmp(
    mut csp1: *mut control_sub_pane,
    mut csp2: *mut control_sub_pane,
) -> libc::c_int {
    if (*csp1).pane < (*csp2).pane {
        return -(1 as libc::c_int);
    }
    if (*csp1).pane > (*csp2).pane {
        return 1 as libc::c_int;
    }
    if (*csp1).idx < (*csp2).idx {
        return -(1 as libc::c_int);
    }
    if (*csp1).idx > (*csp2).idx {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn control_sub_panes_RB_INSERT(
    mut head: *mut control_sub_panes,
    mut elm: *mut control_sub_pane,
) -> *mut control_sub_pane {
    let mut tmp: *mut control_sub_pane = 0 as *mut control_sub_pane;
    let mut parent: *mut control_sub_pane = 0 as *mut control_sub_pane;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = control_sub_pane_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut control_sub_pane;
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
    control_sub_panes_RB_INSERT_COLOR(head, elm);
    return 0 as *mut control_sub_pane;
}
unsafe extern "C" fn control_sub_panes_RB_REMOVE(
    mut head: *mut control_sub_panes,
    mut elm: *mut control_sub_pane,
) -> *mut control_sub_pane {
    let mut current_block: u64;
    let mut child: *mut control_sub_pane = 0 as *mut control_sub_pane;
    let mut parent: *mut control_sub_pane = 0 as *mut control_sub_pane;
    let mut old: *mut control_sub_pane = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut control_sub_pane = 0 as *mut control_sub_pane;
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
        current_block = 14227616053818022051;
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
        control_sub_panes_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn control_sub_panes_RB_FIND(
    mut head: *mut control_sub_panes,
    mut elm: *mut control_sub_pane,
) -> *mut control_sub_pane {
    let mut tmp: *mut control_sub_pane = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = control_sub_pane_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut control_sub_pane;
}
unsafe extern "C" fn control_sub_panes_RB_INSERT_COLOR(
    mut head: *mut control_sub_panes,
    mut elm: *mut control_sub_pane,
) {
    let mut parent: *mut control_sub_pane = 0 as *mut control_sub_pane;
    let mut gparent: *mut control_sub_pane = 0 as *mut control_sub_pane;
    let mut tmp: *mut control_sub_pane = 0 as *mut control_sub_pane;
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
unsafe extern "C" fn control_sub_panes_RB_REMOVE_COLOR(
    mut head: *mut control_sub_panes,
    mut parent: *mut control_sub_pane,
    mut elm: *mut control_sub_pane,
) {
    let mut tmp: *mut control_sub_pane = 0 as *mut control_sub_pane;
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
                    let mut oleft: *mut control_sub_pane = 0 as *mut control_sub_pane;
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
                    let mut oright: *mut control_sub_pane = 0 as *mut control_sub_pane;
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
unsafe extern "C" fn control_sub_panes_RB_MINMAX(
    mut head: *mut control_sub_panes,
    mut val: libc::c_int,
) -> *mut control_sub_pane {
    let mut tmp: *mut control_sub_pane = (*head).rbh_root;
    let mut parent: *mut control_sub_pane = 0 as *mut control_sub_pane;
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
unsafe extern "C" fn control_sub_panes_RB_NEXT(
    mut elm: *mut control_sub_pane,
) -> *mut control_sub_pane {
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
/* Compare client subscription windows. */
unsafe extern "C" fn control_sub_window_cmp(
    mut csw1: *mut control_sub_window,
    mut csw2: *mut control_sub_window,
) -> libc::c_int {
    if (*csw1).window < (*csw2).window {
        return -(1 as libc::c_int);
    }
    if (*csw1).window > (*csw2).window {
        return 1 as libc::c_int;
    }
    if (*csw1).idx < (*csw2).idx {
        return -(1 as libc::c_int);
    }
    if (*csw1).idx > (*csw2).idx {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn control_sub_windows_RB_INSERT(
    mut head: *mut control_sub_windows,
    mut elm: *mut control_sub_window,
) -> *mut control_sub_window {
    let mut tmp: *mut control_sub_window = 0 as *mut control_sub_window;
    let mut parent: *mut control_sub_window = 0 as *mut control_sub_window;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = control_sub_window_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut control_sub_window;
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
    control_sub_windows_RB_INSERT_COLOR(head, elm);
    return 0 as *mut control_sub_window;
}
unsafe extern "C" fn control_sub_windows_RB_INSERT_COLOR(
    mut head: *mut control_sub_windows,
    mut elm: *mut control_sub_window,
) {
    let mut parent: *mut control_sub_window = 0 as *mut control_sub_window;
    let mut gparent: *mut control_sub_window = 0 as *mut control_sub_window;
    let mut tmp: *mut control_sub_window = 0 as *mut control_sub_window;
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
unsafe extern "C" fn control_sub_windows_RB_REMOVE(
    mut head: *mut control_sub_windows,
    mut elm: *mut control_sub_window,
) -> *mut control_sub_window {
    let mut current_block: u64;
    let mut child: *mut control_sub_window = 0 as *mut control_sub_window;
    let mut parent: *mut control_sub_window = 0 as *mut control_sub_window;
    let mut old: *mut control_sub_window = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut control_sub_window = 0 as *mut control_sub_window;
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
        current_block = 1295324108926466853;
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
        control_sub_windows_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn control_sub_windows_RB_FIND(
    mut head: *mut control_sub_windows,
    mut elm: *mut control_sub_window,
) -> *mut control_sub_window {
    let mut tmp: *mut control_sub_window = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = control_sub_window_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut control_sub_window;
}
unsafe extern "C" fn control_sub_windows_RB_MINMAX(
    mut head: *mut control_sub_windows,
    mut val: libc::c_int,
) -> *mut control_sub_window {
    let mut tmp: *mut control_sub_window = (*head).rbh_root;
    let mut parent: *mut control_sub_window = 0 as *mut control_sub_window;
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
unsafe extern "C" fn control_sub_windows_RB_NEXT(
    mut elm: *mut control_sub_window,
) -> *mut control_sub_window {
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
unsafe extern "C" fn control_sub_windows_RB_REMOVE_COLOR(
    mut head: *mut control_sub_windows,
    mut parent: *mut control_sub_window,
    mut elm: *mut control_sub_window,
) {
    let mut tmp: *mut control_sub_window = 0 as *mut control_sub_window;
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
                    let mut oleft: *mut control_sub_window = 0 as *mut control_sub_window;
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
                    let mut oright: *mut control_sub_window = 0 as *mut control_sub_window;
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
/* Free a subscription. */
unsafe extern "C" fn control_free_sub(mut cs: *mut control_state, mut csub: *mut control_sub) {
    let mut csp: *mut control_sub_pane = 0 as *mut control_sub_pane;
    let mut csp1: *mut control_sub_pane = 0 as *mut control_sub_pane;
    let mut csw: *mut control_sub_window = 0 as *mut control_sub_window;
    let mut csw1: *mut control_sub_window = 0 as *mut control_sub_window;
    csp = control_sub_panes_RB_MINMAX(&mut (*csub).panes, -(1 as libc::c_int));
    while !csp.is_null() && {
        csp1 = control_sub_panes_RB_NEXT(csp);
        (1 as libc::c_int) != 0
    } {
        control_sub_panes_RB_REMOVE(&mut (*csub).panes, csp);
        free(csp as *mut libc::c_void);
        csp = csp1
    }
    csw = control_sub_windows_RB_MINMAX(&mut (*csub).windows, -(1 as libc::c_int));
    while !csw.is_null() && {
        csw1 = control_sub_windows_RB_NEXT(csw);
        (1 as libc::c_int) != 0
    } {
        control_sub_windows_RB_REMOVE(&mut (*csub).windows, csw);
        free(csw as *mut libc::c_void);
        csw = csw1
    }
    free((*csub).last as *mut libc::c_void);
    control_subs_RB_REMOVE(&mut (*cs).subs, csub);
    free((*csub).name as *mut libc::c_void);
    free((*csub).format as *mut libc::c_void);
    free(csub as *mut libc::c_void);
}
/* Free a block. */
unsafe extern "C" fn control_free_block(mut cs: *mut control_state, mut cb: *mut control_block) {
    free((*cb).line as *mut libc::c_void);
    if !(*cb).all_entry.tqe_next.is_null() {
        (*(*cb).all_entry.tqe_next).all_entry.tqe_prev = (*cb).all_entry.tqe_prev
    } else {
        (*cs).all_blocks.tqh_last = (*cb).all_entry.tqe_prev
    }
    *(*cb).all_entry.tqe_prev = (*cb).all_entry.tqe_next;
    free(cb as *mut libc::c_void);
}
/* Get pane offsets for this client. */
unsafe extern "C" fn control_get_pane(
    mut c: *mut client,
    mut wp: *mut window_pane,
) -> *mut control_pane {
    let mut cs: *mut control_state = (*c).control_state;
    let mut cp: control_pane = {
        let mut init = control_pane {
            pane: (*wp).id,
            offset: window_pane_offset { used: 0 },
            queued: window_pane_offset { used: 0 },
            flags: 0,
            pending_flag: 0,
            pending_entry: C2RustUnnamed_40 {
                tqe_next: 0 as *mut control_pane,
                tqe_prev: 0 as *mut *mut control_pane,
            },
            blocks: C2RustUnnamed_39 {
                tqh_first: 0 as *mut control_block,
                tqh_last: 0 as *mut *mut control_block,
            },
            entry: C2RustUnnamed_38 {
                rbe_left: 0 as *mut control_pane,
                rbe_right: 0 as *mut control_pane,
                rbe_parent: 0 as *mut control_pane,
                rbe_color: 0,
            },
        };
        init
    };
    return control_panes_RB_FIND(&mut (*cs).panes, &mut cp);
}
/* Add pane offsets for this client. */
unsafe extern "C" fn control_add_pane(
    mut c: *mut client,
    mut wp: *mut window_pane,
) -> *mut control_pane {
    let mut cs: *mut control_state = (*c).control_state;
    let mut cp: *mut control_pane = 0 as *mut control_pane;
    cp = control_get_pane(c, wp);
    if !cp.is_null() {
        return cp;
    }
    cp = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<control_pane>() as libc::c_ulong,
    ) as *mut control_pane;
    (*cp).pane = (*wp).id;
    control_panes_RB_INSERT(&mut (*cs).panes, cp);
    memcpy(
        &mut (*cp).offset as *mut window_pane_offset as *mut libc::c_void,
        &mut (*wp).offset as *mut window_pane_offset as *const libc::c_void,
        ::std::mem::size_of::<window_pane_offset>() as libc::c_ulong,
    );
    memcpy(
        &mut (*cp).queued as *mut window_pane_offset as *mut libc::c_void,
        &mut (*wp).offset as *mut window_pane_offset as *const libc::c_void,
        ::std::mem::size_of::<window_pane_offset>() as libc::c_ulong,
    );
    (*cp).blocks.tqh_first = 0 as *mut control_block;
    (*cp).blocks.tqh_last = &mut (*cp).blocks.tqh_first;
    return cp;
}
/* Discard output for a pane. */
unsafe extern "C" fn control_discard_pane(mut c: *mut client, mut cp: *mut control_pane) {
    let mut cs: *mut control_state = (*c).control_state;
    let mut cb: *mut control_block = 0 as *mut control_block;
    let mut cb1: *mut control_block = 0 as *mut control_block;
    cb = (*cp).blocks.tqh_first;
    while !cb.is_null() && {
        cb1 = (*cb).entry.tqe_next;
        (1 as libc::c_int) != 0
    } {
        if !(*cb).entry.tqe_next.is_null() {
            (*(*cb).entry.tqe_next).entry.tqe_prev = (*cb).entry.tqe_prev
        } else {
            (*cp).blocks.tqh_last = (*cb).entry.tqe_prev
        }
        *(*cb).entry.tqe_prev = (*cb).entry.tqe_next;
        control_free_block(cs, cb);
        cb = cb1
    }
}
/* Get actual pane for this client. */
unsafe extern "C" fn control_window_pane(mut c: *mut client, mut pane: u_int) -> *mut window_pane {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if (*c).session.is_null() {
        return 0 as *mut window_pane;
    }
    wp = window_pane_find_by_id(pane);
    if wp.is_null() {
        return 0 as *mut window_pane;
    }
    if winlink_find_by_window(&mut (*(*c).session).windows, (*wp).window).is_null() {
        return 0 as *mut window_pane;
    }
    return wp;
}
/* Reset control offsets. */
#[no_mangle]
pub unsafe extern "C" fn control_reset_offsets(mut c: *mut client) {
    let mut cs: *mut control_state = (*c).control_state;
    let mut cp: *mut control_pane = 0 as *mut control_pane;
    let mut cp1: *mut control_pane = 0 as *mut control_pane;
    cp = control_panes_RB_MINMAX(&mut (*cs).panes, -(1 as libc::c_int));
    while !cp.is_null() && {
        cp1 = control_panes_RB_NEXT(cp);
        (1 as libc::c_int) != 0
    } {
        control_panes_RB_REMOVE(&mut (*cs).panes, cp);
        free(cp as *mut libc::c_void);
        cp = cp1
    }
    (*cs).pending_list.tqh_first = 0 as *mut control_pane;
    (*cs).pending_list.tqh_last = &mut (*cs).pending_list.tqh_first;
    (*cs).pending_count = 0 as libc::c_int as u_int;
}
/* Get offsets for client. */
#[no_mangle]
pub unsafe extern "C" fn control_pane_offset(
    mut c: *mut client,
    mut wp: *mut window_pane,
    mut off: *mut libc::c_int,
) -> *mut window_pane_offset {
    let mut cs: *mut control_state = (*c).control_state;
    let mut cp: *mut control_pane = 0 as *mut control_pane;
    if (*c).flags & 0x4000000 as libc::c_int as libc::c_ulong != 0 {
        *off = 0 as libc::c_int;
        return 0 as *mut window_pane_offset;
    }
    cp = control_get_pane(c, wp);
    if cp.is_null() || (*cp).flags & 0x2 as libc::c_int != 0 {
        *off = 0 as libc::c_int;
        return 0 as *mut window_pane_offset;
    }
    if (*cp).flags & 0x1 as libc::c_int != 0 {
        *off = 1 as libc::c_int;
        return 0 as *mut window_pane_offset;
    }
    *off = (evbuffer_get_length((*(*cs).write_event).output) >= 512 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    return &mut (*cp).offset;
}
/* Set pane as on. */
#[no_mangle]
pub unsafe extern "C" fn control_set_pane_on(mut c: *mut client, mut wp: *mut window_pane) {
    let mut cp: *mut control_pane = 0 as *mut control_pane;
    cp = control_get_pane(c, wp);
    if !cp.is_null() && (*cp).flags & 0x1 as libc::c_int != 0 {
        (*cp).flags &= !(0x1 as libc::c_int);
        memcpy(
            &mut (*cp).offset as *mut window_pane_offset as *mut libc::c_void,
            &mut (*wp).offset as *mut window_pane_offset as *const libc::c_void,
            ::std::mem::size_of::<window_pane_offset>() as libc::c_ulong,
        );
        memcpy(
            &mut (*cp).queued as *mut window_pane_offset as *mut libc::c_void,
            &mut (*wp).offset as *mut window_pane_offset as *const libc::c_void,
            ::std::mem::size_of::<window_pane_offset>() as libc::c_ulong,
        );
    };
}
/* Set pane as off. */
#[no_mangle]
pub unsafe extern "C" fn control_set_pane_off(mut c: *mut client, mut wp: *mut window_pane) {
    let mut cp: *mut control_pane = 0 as *mut control_pane;
    cp = control_add_pane(c, wp);
    (*cp).flags |= 0x1 as libc::c_int;
}
/* Continue a paused pane. */
#[no_mangle]
pub unsafe extern "C" fn control_continue_pane(mut c: *mut client, mut wp: *mut window_pane) {
    let mut cp: *mut control_pane = 0 as *mut control_pane;
    cp = control_get_pane(c, wp);
    if !cp.is_null() && (*cp).flags & 0x2 as libc::c_int != 0 {
        (*cp).flags &= !(0x2 as libc::c_int);
        memcpy(
            &mut (*cp).offset as *mut window_pane_offset as *mut libc::c_void,
            &mut (*wp).offset as *mut window_pane_offset as *const libc::c_void,
            ::std::mem::size_of::<window_pane_offset>() as libc::c_ulong,
        );
        memcpy(
            &mut (*cp).queued as *mut window_pane_offset as *mut libc::c_void,
            &mut (*wp).offset as *mut window_pane_offset as *const libc::c_void,
            ::std::mem::size_of::<window_pane_offset>() as libc::c_ulong,
        );
        control_write(
            c,
            b"%%continue %%%u\x00" as *const u8 as *const libc::c_char,
            (*wp).id,
        );
    };
}
/* Pause a pane. */
#[no_mangle]
pub unsafe extern "C" fn control_pause_pane(mut c: *mut client, mut wp: *mut window_pane) {
    let mut cp: *mut control_pane = 0 as *mut control_pane;
    cp = control_add_pane(c, wp);
    if !(*cp).flags & 0x2 as libc::c_int != 0 {
        (*cp).flags |= 0x2 as libc::c_int;
        control_discard_pane(c, cp);
        control_write(
            c,
            b"%%pause %%%u\x00" as *const u8 as *const libc::c_char,
            (*wp).id,
        );
    };
}
/* Write a line. */
unsafe extern "C" fn control_vwrite(
    mut c: *mut client,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut cs: *mut control_state = (*c).control_state;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    xvasprintf(&mut s, fmt, ap.as_va_list());
    log_debug(
        b"%s: %s: writing line: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"control_vwrite\x00")).as_ptr(),
        (*c).name,
        s,
    );
    bufferevent_write((*cs).write_event, s as *const libc::c_void, strlen(s));
    bufferevent_write(
        (*cs).write_event,
        b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    bufferevent_enable((*cs).write_event, 0x4 as libc::c_int as libc::c_short);
    free(s as *mut libc::c_void);
}
/* Write a line. */
#[no_mangle]
pub unsafe extern "C" fn control_write(
    mut c: *mut client,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut cs: *mut control_state = (*c).control_state;
    let mut cb: *mut control_block = 0 as *mut control_block;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if (*cs).all_blocks.tqh_first.is_null() {
        control_vwrite(c, fmt, ap.as_va_list());
        return;
    }
    cb = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<control_block>() as libc::c_ulong,
    ) as *mut control_block;
    xvasprintf(&mut (*cb).line, fmt, ap.as_va_list());
    (*cb).all_entry.tqe_next = 0 as *mut control_block;
    (*cb).all_entry.tqe_prev = (*cs).all_blocks.tqh_last;
    *(*cs).all_blocks.tqh_last = cb;
    (*cs).all_blocks.tqh_last = &mut (*cb).all_entry.tqe_next;
    (*cb).t = get_timer();
    log_debug(
        b"%s: %s: storing line: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"control_write\x00")).as_ptr(),
        (*c).name,
        (*cb).line,
    );
    bufferevent_enable((*cs).write_event, 0x4 as libc::c_int as libc::c_short);
}
/* Check age for this pane. */
unsafe extern "C" fn control_check_age(
    mut c: *mut client,
    mut wp: *mut window_pane,
    mut cp: *mut control_pane,
) -> libc::c_int {
    let mut cb: *mut control_block = 0 as *mut control_block;
    let mut t: uint64_t = 0;
    let mut age: uint64_t = 0;
    cb = (*cp).blocks.tqh_first;
    if cb.is_null() {
        return 0 as libc::c_int;
    }
    t = get_timer();
    if (*cb).t >= t {
        return 0 as libc::c_int;
    }
    age = t.wrapping_sub((*cb).t);
    log_debug(
        b"%s: %s: %%%u is %llu behind\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"control_check_age\x00"))
            .as_ptr(),
        (*c).name,
        (*wp).id,
        age as libc::c_ulonglong,
    );
    if (*c).flags as libc::c_ulonglong & 0x100000000 as libc::c_ulonglong != 0 {
        if age < (*c).pause_age as libc::c_ulong {
            return 0 as libc::c_int;
        }
        (*cp).flags |= 0x2 as libc::c_int;
        control_discard_pane(c, cp);
        control_write(
            c,
            b"%%pause %%%u\x00" as *const u8 as *const libc::c_char,
            (*wp).id,
        );
    } else {
        if age < 300000 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        (*c).exit_message = xstrdup(b"too far behind\x00" as *const u8 as *const libc::c_char);
        (*c).flags |= 0x4 as libc::c_int as libc::c_ulong;
        control_discard(c);
    }
    return 1 as libc::c_int;
}
/* Write output from a pane. */
#[no_mangle]
pub unsafe extern "C" fn control_write_output(mut c: *mut client, mut wp: *mut window_pane) {
    let mut cs: *mut control_state = (*c).control_state;
    let mut cp: *mut control_pane = 0 as *mut control_pane;
    let mut cb: *mut control_block = 0 as *mut control_block;
    let mut new_size: size_t = 0;
    if winlink_find_by_window(&mut (*(*c).session).windows, (*wp).window).is_null() {
        return;
    }
    if (*c).flags
        & (0x4000000 as libc::c_int
            | (0x200 as libc::c_int | 0x40 as libc::c_int | 0x4 as libc::c_int))
            as libc::c_ulong
        != 0
    {
        cp = control_get_pane(c, wp);
        if cp.is_null() {
            return;
        }
    } else {
        cp = control_add_pane(c, wp);
        if !((*cp).flags & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0) {
            if control_check_age(c, wp, cp) != 0 {
                return;
            }
            window_pane_get_new_data(wp, &mut (*cp).queued, &mut new_size);
            if new_size == 0 as libc::c_int as libc::c_ulong {
                return;
            }
            window_pane_update_used_data(wp, &mut (*cp).queued, new_size);
            cb = xcalloc(
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<control_block>() as libc::c_ulong,
            ) as *mut control_block;
            (*cb).size = new_size;
            (*cb).all_entry.tqe_next = 0 as *mut control_block;
            (*cb).all_entry.tqe_prev = (*cs).all_blocks.tqh_last;
            *(*cs).all_blocks.tqh_last = cb;
            (*cs).all_blocks.tqh_last = &mut (*cb).all_entry.tqe_next;
            (*cb).t = get_timer();
            (*cb).entry.tqe_next = 0 as *mut control_block;
            (*cb).entry.tqe_prev = (*cp).blocks.tqh_last;
            *(*cp).blocks.tqh_last = cb;
            (*cp).blocks.tqh_last = &mut (*cb).entry.tqe_next;
            log_debug(
                b"%s: %s: new output block of %zu for %%%u\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"control_write_output\x00",
                ))
                .as_ptr(),
                (*c).name,
                (*cb).size,
                (*wp).id,
            );
            if (*cp).pending_flag == 0 {
                log_debug(
                    b"%s: %s: %%%u now pending\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                        b"control_write_output\x00",
                    ))
                    .as_ptr(),
                    (*c).name,
                    (*wp).id,
                );
                (*cp).pending_entry.tqe_next = 0 as *mut control_pane;
                (*cp).pending_entry.tqe_prev = (*cs).pending_list.tqh_last;
                *(*cs).pending_list.tqh_last = cp;
                (*cs).pending_list.tqh_last = &mut (*cp).pending_entry.tqe_next;
                (*cp).pending_flag = 1 as libc::c_int;
                (*cs).pending_count = (*cs).pending_count.wrapping_add(1)
            }
            bufferevent_enable((*cs).write_event, 0x4 as libc::c_int as libc::c_short);
            return;
        }
    }
    log_debug(
        b"%s: %s: ignoring pane %%%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"control_write_output\x00"))
            .as_ptr(),
        (*c).name,
        (*wp).id,
    );
    window_pane_update_used_data(wp, &mut (*cp).offset, 18446744073709551615 as libc::c_ulong);
    window_pane_update_used_data(wp, &mut (*cp).queued, 18446744073709551615 as libc::c_ulong);
}
/* Control client error callback. */
unsafe extern "C" fn control_error(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut data: *mut libc::c_void,
) -> cmd_retval {
    let mut c: *mut client = cmdq_get_client(item);
    let mut error: *mut libc::c_char = data as *mut libc::c_char;
    cmdq_guard(
        item,
        b"begin\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    control_write(
        c,
        b"parse error: %s\x00" as *const u8 as *const libc::c_char,
        error,
    );
    cmdq_guard(
        item,
        b"error\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    free(error as *mut libc::c_void);
    return CMD_RETURN_NORMAL;
}
/* Control client error callback. */
unsafe extern "C" fn control_error_callback(
    mut _bufev: *mut bufferevent,
    mut _what: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut c: *mut client = data as *mut client;
    (*c).flags |= 0x4 as libc::c_int as libc::c_ulong;
}
/* Control client input callback. Read lines and fire commands. */
unsafe extern "C" fn control_read_callback(
    mut _bufev: *mut bufferevent,
    mut data: *mut libc::c_void,
) {
    let mut c: *mut client = data as *mut client;
    let mut cs: *mut control_state = (*c).control_state;
    let mut buffer: *mut evbuffer = (*(*cs).read_event).input;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut state: *mut crate::cmd_queue::cmdq_state = 0 as *mut crate::cmd_queue::cmdq_state;
    let mut status: cmd_parse_status = CMD_PARSE_EMPTY;
    loop {
        line = evbuffer_readln(buffer, 0 as *mut size_t, EVBUFFER_EOL_LF);
        if line.is_null() {
            break;
        }
        log_debug(
            b"%s: %s: %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"control_read_callback\x00",
            ))
            .as_ptr(),
            (*c).name,
            line,
        );
        if *line as libc::c_int == '\u{0}' as i32 {
            /* empty line detach */
            free(line as *mut libc::c_void);
            (*c).flags |= 0x4 as libc::c_int as libc::c_ulong;
            break;
        } else {
            state = cmdq_new_state(
                0 as *mut cmd_find_state,
                0 as *mut key_event,
                0x2 as libc::c_int,
            );
            status = cmd_parse_and_append(line, 0 as *mut cmd_parse_input, c, state, &mut error);
            if status as libc::c_uint == CMD_PARSE_ERROR as libc::c_int as libc::c_uint {
                cmdq_append(
                    c,
                    cmdq_get_callback1(
                        b"control_error\x00" as *const u8 as *const libc::c_char,
                        Some(
                            control_error
                                as unsafe extern "C" fn(
                                    _: *mut crate::cmd_queue::cmdq_item,
                                    _: *mut libc::c_void,
                                )
                                    -> cmd_retval,
                        ),
                        error as *mut libc::c_void,
                    ),
                );
            }
            cmdq_free_state(state);
            free(line as *mut libc::c_void);
        }
    }
}
/* Does this control client have outstanding data to write? */
#[no_mangle]
pub unsafe extern "C" fn control_all_done(mut c: *mut client) -> libc::c_int {
    let mut cs: *mut control_state = (*c).control_state;
    if !(*cs).all_blocks.tqh_first.is_null() {
        return 0 as libc::c_int;
    }
    return (evbuffer_get_length((*(*cs).write_event).output) == 0 as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
/* Flush all blocks until output. */
unsafe extern "C" fn control_flush_all_blocks(mut c: *mut client) {
    let mut cs: *mut control_state = (*c).control_state;
    let mut cb: *mut control_block = 0 as *mut control_block;
    let mut cb1: *mut control_block = 0 as *mut control_block;
    cb = (*cs).all_blocks.tqh_first;
    while !cb.is_null() && {
        cb1 = (*cb).all_entry.tqe_next;
        (1 as libc::c_int) != 0
    } {
        if (*cb).size != 0 as libc::c_int as libc::c_ulong {
            break;
        }
        log_debug(
            b"%s: %s: flushing line: %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"control_flush_all_blocks\x00",
            ))
            .as_ptr(),
            (*c).name,
            (*cb).line,
        );
        bufferevent_write(
            (*cs).write_event,
            (*cb).line as *const libc::c_void,
            strlen((*cb).line),
        );
        bufferevent_write(
            (*cs).write_event,
            b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        control_free_block(cs, cb);
        cb = cb1
    }
}
/* Append data to buffer. */
unsafe extern "C" fn control_append_data(
    mut c: *mut client,
    mut cp: *mut control_pane,
    mut age: uint64_t,
    mut message: *mut evbuffer,
    mut wp: *mut window_pane,
    mut size: size_t,
) -> *mut evbuffer {
    let mut new_data: *mut u_char = 0 as *mut u_char;
    let mut new_size: size_t = 0;
    let mut i: u_int = 0;
    if message.is_null() {
        message = evbuffer_new();
        if message.is_null() {
            fatalx(b"out of memory\x00" as *const u8 as *const libc::c_char);
        }
        if (*c).flags as libc::c_ulonglong & 0x100000000 as libc::c_ulonglong != 0 {
            evbuffer_add_printf(
                message,
                b"%%extended-output %%%u %llu : \x00" as *const u8 as *const libc::c_char,
                (*wp).id,
                age as libc::c_ulonglong,
            );
        } else {
            evbuffer_add_printf(
                message,
                b"%%output %%%u \x00" as *const u8 as *const libc::c_char,
                (*wp).id,
            );
        }
    }
    new_data = window_pane_get_new_data(wp, &mut (*cp).offset, &mut new_size) as *mut u_char;
    if new_size < size {
        fatalx(
            b"not enough data: %zu < %zu\x00" as *const u8 as *const libc::c_char,
            new_size,
            size,
        );
    }
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong) < size {
        if (*new_data.offset(i as isize) as libc::c_int) < ' ' as i32
            || *new_data.offset(i as isize) as libc::c_int == '\\' as i32
        {
            evbuffer_add_printf(
                message,
                b"\\%03o\x00" as *const u8 as *const libc::c_char,
                *new_data.offset(i as isize) as libc::c_int,
            );
        } else {
            evbuffer_add_printf(
                message,
                b"%c\x00" as *const u8 as *const libc::c_char,
                *new_data.offset(i as isize) as libc::c_int,
            );
        }
        i = i.wrapping_add(1)
    }
    window_pane_update_used_data(wp, &mut (*cp).offset, size);
    return message;
}
/* Write buffer. */
unsafe extern "C" fn control_write_data(mut c: *mut client, mut message: *mut evbuffer) {
    let mut cs: *mut control_state = (*c).control_state;
    log_debug(
        b"%s: %s: %.*s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"control_write_data\x00"))
            .as_ptr(),
        (*c).name,
        evbuffer_get_length(message) as libc::c_int,
        evbuffer_pullup(message, -(1 as libc::c_int) as ssize_t),
    );
    evbuffer_add(
        message,
        b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    bufferevent_write_buffer((*cs).write_event, message);
    evbuffer_free(message);
}
/* Write output to client. */
unsafe extern "C" fn control_write_pending(
    mut c: *mut client,
    mut cp: *mut control_pane,
    mut limit: size_t,
) -> libc::c_int {
    let mut cs: *mut control_state = (*c).control_state;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut message: *mut evbuffer = 0 as *mut evbuffer;
    let mut used: size_t = 0 as libc::c_int as size_t;
    let mut size: size_t = 0;
    let mut cb: *mut control_block = 0 as *mut control_block;
    let mut cb1: *mut control_block = 0 as *mut control_block;
    let mut age: uint64_t = 0;
    let mut t: uint64_t = get_timer();
    wp = control_window_pane(c, (*cp).pane);
    if wp.is_null() {
        cb = (*cp).blocks.tqh_first;
        while !cb.is_null() && {
            cb1 = (*cb).entry.tqe_next;
            (1 as libc::c_int) != 0
        } {
            if !(*cb).entry.tqe_next.is_null() {
                (*(*cb).entry.tqe_next).entry.tqe_prev = (*cb).entry.tqe_prev
            } else {
                (*cp).blocks.tqh_last = (*cb).entry.tqe_prev
            }
            *(*cb).entry.tqe_prev = (*cb).entry.tqe_next;
            control_free_block(cs, cb);
            cb = cb1
        }
        control_flush_all_blocks(c);
        return 0 as libc::c_int;
    }
    while used != limit && !(*cp).blocks.tqh_first.is_null() {
        if control_check_age(c, wp, cp) != 0 {
            if !message.is_null() {
                evbuffer_free(message);
            }
            message = 0 as *mut evbuffer;
            break;
        } else {
            cb = (*cp).blocks.tqh_first;
            if (*cb).t < t {
                age = t.wrapping_sub((*cb).t)
            } else {
                age = 0 as libc::c_int as uint64_t
            }
            log_debug(
                b"%s: %s: output block %zu (age %llu) for %%%u (used %zu/%zu)\x00" as *const u8
                    as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"control_write_pending\x00",
                ))
                .as_ptr(),
                (*c).name,
                (*cb).size,
                age as libc::c_ulonglong,
                (*cp).pane,
                used,
                limit,
            );
            size = (*cb).size;
            if size > limit.wrapping_sub(used) {
                size = limit.wrapping_sub(used)
            }
            used = (used as libc::c_ulong).wrapping_add(size) as size_t as size_t;
            message = control_append_data(c, cp, age, message, wp, size);
            (*cb).size = ((*cb).size as libc::c_ulong).wrapping_sub(size) as size_t as size_t;
            if (*cb).size == 0 as libc::c_int as libc::c_ulong {
                if !(*cb).entry.tqe_next.is_null() {
                    (*(*cb).entry.tqe_next).entry.tqe_prev = (*cb).entry.tqe_prev
                } else {
                    (*cp).blocks.tqh_last = (*cb).entry.tqe_prev
                }
                *(*cb).entry.tqe_prev = (*cb).entry.tqe_next;
                control_free_block(cs, cb);
                cb = (*cs).all_blocks.tqh_first;
                if !cb.is_null() && (*cb).size == 0 as libc::c_int as libc::c_ulong {
                    if !wp.is_null() && !message.is_null() {
                        control_write_data(c, message);
                        message = 0 as *mut evbuffer
                    }
                    control_flush_all_blocks(c);
                }
            }
        }
    }
    if !message.is_null() {
        control_write_data(c, message);
    }
    return !(*cp).blocks.tqh_first.is_null() as libc::c_int;
}
/* Control client write callback. */
unsafe extern "C" fn control_write_callback(
    mut _bufev: *mut bufferevent,
    mut data: *mut libc::c_void,
) {
    let mut c: *mut client = data as *mut client; /* 3 bytes for \xxx */
    let mut cs: *mut control_state = (*c).control_state;
    let mut cp: *mut control_pane = 0 as *mut control_pane;
    let mut cp1: *mut control_pane = 0 as *mut control_pane;
    let mut evb: *mut evbuffer = (*(*cs).write_event).output;
    let mut space: size_t = 0;
    let mut limit: size_t = 0;
    control_flush_all_blocks(c);
    while evbuffer_get_length(evb) < 8192 as libc::c_int as libc::c_ulong {
        if (*cs).pending_count == 0 as libc::c_int as libc::c_uint {
            break;
        }
        space = (8192 as libc::c_int as libc::c_ulong).wrapping_sub(evbuffer_get_length(evb));
        log_debug(
            b"%s: %s: %zu bytes available, %u panes\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"control_write_callback\x00",
            ))
            .as_ptr(),
            (*c).name,
            space,
            (*cs).pending_count,
        );
        limit = space
            .wrapping_div((*cs).pending_count as libc::c_ulong)
            .wrapping_div(3 as libc::c_int as libc::c_ulong);
        if limit < 32 as libc::c_int as libc::c_ulong {
            limit = 32 as libc::c_int as size_t
        }
        cp = (*cs).pending_list.tqh_first;
        while !cp.is_null() && {
            cp1 = (*cp).pending_entry.tqe_next;
            (1 as libc::c_int) != 0
        } {
            if evbuffer_get_length(evb) >= 8192 as libc::c_int as libc::c_ulong {
                break;
            }
            if !(control_write_pending(c, cp, limit) != 0) {
                if !(*cp).pending_entry.tqe_next.is_null() {
                    (*(*cp).pending_entry.tqe_next).pending_entry.tqe_prev =
                        (*cp).pending_entry.tqe_prev
                } else {
                    (*cs).pending_list.tqh_last = (*cp).pending_entry.tqe_prev
                }
                *(*cp).pending_entry.tqe_prev = (*cp).pending_entry.tqe_next;
                (*cp).pending_flag = 0 as libc::c_int;
                (*cs).pending_count = (*cs).pending_count.wrapping_sub(1)
            }
            cp = cp1
        }
    }
    if evbuffer_get_length(evb) == 0 as libc::c_int as libc::c_ulong {
        bufferevent_disable((*cs).write_event, 0x4 as libc::c_int as libc::c_short);
    };
}
/* Initialize for control mode. */
#[no_mangle]
pub unsafe extern "C" fn control_start(mut c: *mut client) {
    let mut cs: *mut control_state = 0 as *mut control_state;
    if (*c).flags & 0x4000 as libc::c_int as libc::c_ulong != 0 {
        close((*c).out_fd);
        (*c).out_fd = -(1 as libc::c_int)
    } else {
        setblocking((*c).out_fd, 0 as libc::c_int);
    }
    setblocking((*c).fd, 0 as libc::c_int);
    (*c).control_state = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<control_state>() as libc::c_ulong,
    ) as *mut control_state;
    cs = (*c).control_state;
    (*cs).panes.rbh_root = 0 as *mut control_pane;
    (*cs).pending_list.tqh_first = 0 as *mut control_pane;
    (*cs).pending_list.tqh_last = &mut (*cs).pending_list.tqh_first;
    (*cs).all_blocks.tqh_first = 0 as *mut control_block;
    (*cs).all_blocks.tqh_last = &mut (*cs).all_blocks.tqh_first;
    (*cs).subs.rbh_root = 0 as *mut control_sub;
    (*cs).read_event = bufferevent_new(
        (*c).fd,
        Some(
            control_read_callback
                as unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void) -> (),
        ),
        Some(
            control_write_callback
                as unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void) -> (),
        ),
        Some(
            control_error_callback
                as unsafe extern "C" fn(
                    _: *mut bufferevent,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        c as *mut libc::c_void,
    );
    bufferevent_enable((*cs).read_event, 0x2 as libc::c_int as libc::c_short);
    if (*c).flags & 0x4000 as libc::c_int as libc::c_ulong != 0 {
        (*cs).write_event = (*cs).read_event
    } else {
        (*cs).write_event = bufferevent_new(
            (*c).out_fd,
            None,
            Some(
                control_write_callback
                    as unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void) -> (),
            ),
            Some(
                control_error_callback
                    as unsafe extern "C" fn(
                        _: *mut bufferevent,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            c as *mut libc::c_void,
        )
    }
    bufferevent_setwatermark(
        (*cs).write_event,
        0x4 as libc::c_int as libc::c_short,
        512 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
    if (*c).flags & 0x4000 as libc::c_int as libc::c_ulong != 0 {
        bufferevent_write(
            (*cs).write_event,
            b"\x1bP1000p\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            7 as libc::c_int as size_t,
        );
        bufferevent_enable((*cs).write_event, 0x4 as libc::c_int as libc::c_short);
    };
}
/* Discard all output for a client. */
#[no_mangle]
pub unsafe extern "C" fn control_discard(mut c: *mut client) {
    let mut cs: *mut control_state = (*c).control_state;
    let mut cp: *mut control_pane = 0 as *mut control_pane;
    cp = control_panes_RB_MINMAX(&mut (*cs).panes, -(1 as libc::c_int));
    while !cp.is_null() {
        control_discard_pane(c, cp);
        cp = control_panes_RB_NEXT(cp)
    }
    bufferevent_disable((*cs).read_event, 0x2 as libc::c_int as libc::c_short);
}
/* Stop control mode. */
#[no_mangle]
pub unsafe extern "C" fn control_stop(mut c: *mut client) {
    let mut cs: *mut control_state = (*c).control_state;
    let mut cb: *mut control_block = 0 as *mut control_block;
    let mut cb1: *mut control_block = 0 as *mut control_block;
    let mut csub: *mut control_sub = 0 as *mut control_sub;
    let mut csub1: *mut control_sub = 0 as *mut control_sub;
    if !(*c).flags & 0x4000 as libc::c_int as libc::c_ulong != 0 {
        bufferevent_free((*cs).write_event);
    }
    bufferevent_free((*cs).read_event);
    csub = control_subs_RB_MINMAX(&mut (*cs).subs, -(1 as libc::c_int));
    while !csub.is_null() && {
        csub1 = control_subs_RB_NEXT(csub);
        (1 as libc::c_int) != 0
    } {
        control_free_sub(cs, csub);
        csub = csub1
    }
    if event_initialized(&mut (*cs).subs_timer) != 0 {
        event_del(&mut (*cs).subs_timer);
    }
    cb = (*cs).all_blocks.tqh_first;
    while !cb.is_null() && {
        cb1 = (*cb).all_entry.tqe_next;
        (1 as libc::c_int) != 0
    } {
        control_free_block(cs, cb);
        cb = cb1
    }
    control_reset_offsets(c);
    free(cs as *mut libc::c_void);
}
/* Check session subscription. */
unsafe extern "C" fn control_check_subs_session(mut c: *mut client, mut csub: *mut control_sub) {
    let mut s: *mut session = (*c).session;
    let mut ft: *mut crate::format::format_tree = 0 as *mut crate::format::format_tree;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    ft = format_create_defaults(
        0 as *mut crate::cmd_queue::cmdq_item,
        c,
        s,
        0 as *mut winlink,
        0 as *mut window_pane,
    );
    value = format_expand(ft, (*csub).format);
    format_free(ft);
    if !(*csub).last.is_null() && strcmp(value, (*csub).last) == 0 as libc::c_int {
        free(value as *mut libc::c_void);
        return;
    }
    control_write(
        c,
        b"%%subscription-changed %s $%u - - - : %s\x00" as *const u8 as *const libc::c_char,
        (*csub).name,
        (*s).id,
        value,
    );
    free((*csub).last as *mut libc::c_void);
    (*csub).last = value;
}
/* Check pane subscription. */
unsafe extern "C" fn control_check_subs_pane(mut c: *mut client, mut csub: *mut control_sub) {
    let mut s: *mut session = (*c).session;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut w: *mut window = 0 as *mut window;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut ft: *mut crate::format::format_tree = 0 as *mut crate::format::format_tree;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut csp: *mut control_sub_pane = 0 as *mut control_sub_pane;
    let mut find: control_sub_pane = control_sub_pane {
        pane: 0,
        idx: 0,
        last: 0 as *mut libc::c_char,
        entry: C2RustUnnamed_33 {
            rbe_left: 0 as *mut control_sub_pane,
            rbe_right: 0 as *mut control_sub_pane,
            rbe_parent: 0 as *mut control_sub_pane,
            rbe_color: 0,
        },
    };
    wp = window_pane_find_by_id((*csub).id);
    if wp.is_null() {
        return;
    }
    w = (*wp).window;
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if !((*wl).session != s) {
            ft = format_create_defaults(0 as *mut crate::cmd_queue::cmdq_item, c, s, wl, wp);
            value = format_expand(ft, (*csub).format);
            format_free(ft);
            find.pane = (*wp).id;
            find.idx = (*wl).idx as u_int;
            csp = control_sub_panes_RB_FIND(&mut (*csub).panes, &mut find);
            if csp.is_null() {
                csp = xcalloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<control_sub_pane>() as libc::c_ulong,
                ) as *mut control_sub_pane;
                (*csp).pane = (*wp).id;
                (*csp).idx = (*wl).idx as u_int;
                control_sub_panes_RB_INSERT(&mut (*csub).panes, csp);
            }
            if !(*csp).last.is_null() && strcmp(value, (*csp).last) == 0 as libc::c_int {
                free(value as *mut libc::c_void);
            } else {
                control_write(
                    c,
                    b"%%subscription-changed %s $%u @%u %u %%%u : %s\x00" as *const u8
                        as *const libc::c_char,
                    (*csub).name,
                    (*s).id,
                    (*w).id,
                    (*wl).idx,
                    (*wp).id,
                    value,
                );
                free((*csp).last as *mut libc::c_void);
                (*csp).last = value
            }
        }
        wl = (*wl).wentry.tqe_next
    }
}
/* Check all panes subscription. */
unsafe extern "C" fn control_check_subs_all_panes(mut c: *mut client, mut csub: *mut control_sub) {
    let mut s: *mut session = (*c).session;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut w: *mut window = 0 as *mut window;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut ft: *mut crate::format::format_tree = 0 as *mut crate::format::format_tree;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut csp: *mut control_sub_pane = 0 as *mut control_sub_pane;
    let mut find: control_sub_pane = control_sub_pane {
        pane: 0,
        idx: 0,
        last: 0 as *mut libc::c_char,
        entry: C2RustUnnamed_33 {
            rbe_left: 0 as *mut control_sub_pane,
            rbe_right: 0 as *mut control_sub_pane,
            rbe_parent: 0 as *mut control_sub_pane,
            rbe_color: 0,
        },
    };
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        w = (*wl).window;
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            ft = format_create_defaults(0 as *mut crate::cmd_queue::cmdq_item, c, s, wl, wp);
            value = format_expand(ft, (*csub).format);
            format_free(ft);
            find.pane = (*wp).id;
            find.idx = (*wl).idx as u_int;
            csp = control_sub_panes_RB_FIND(&mut (*csub).panes, &mut find);
            if csp.is_null() {
                csp = xcalloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<control_sub_pane>() as libc::c_ulong,
                ) as *mut control_sub_pane;
                (*csp).pane = (*wp).id;
                (*csp).idx = (*wl).idx as u_int;
                control_sub_panes_RB_INSERT(&mut (*csub).panes, csp);
            }
            if !(*csp).last.is_null() && strcmp(value, (*csp).last) == 0 as libc::c_int {
                free(value as *mut libc::c_void);
            } else {
                control_write(
                    c,
                    b"%%subscription-changed %s $%u @%u %u %%%u : %s\x00" as *const u8
                        as *const libc::c_char,
                    (*csub).name,
                    (*s).id,
                    (*w).id,
                    (*wl).idx,
                    (*wp).id,
                    value,
                );
                free((*csp).last as *mut libc::c_void);
                (*csp).last = value
            }
            wp = (*wp).entry.tqe_next
        }
        wl = winlinks_RB_NEXT(wl)
    }
}
/* Check window subscription. */
unsafe extern "C" fn control_check_subs_window(mut c: *mut client, mut csub: *mut control_sub) {
    let mut s: *mut session = (*c).session;
    let mut w: *mut window = 0 as *mut window;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut ft: *mut crate::format::format_tree = 0 as *mut crate::format::format_tree;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut csw: *mut control_sub_window = 0 as *mut control_sub_window;
    let mut find: control_sub_window = control_sub_window {
        window: 0,
        idx: 0,
        last: 0 as *mut libc::c_char,
        entry: C2RustUnnamed_32 {
            rbe_left: 0 as *mut control_sub_window,
            rbe_right: 0 as *mut control_sub_window,
            rbe_parent: 0 as *mut control_sub_window,
            rbe_color: 0,
        },
    };
    w = window_find_by_id((*csub).id);
    if w.is_null() {
        return;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if !((*wl).session != s) {
            ft = format_create_defaults(
                0 as *mut crate::cmd_queue::cmdq_item,
                c,
                s,
                wl,
                0 as *mut window_pane,
            );
            value = format_expand(ft, (*csub).format);
            format_free(ft);
            find.window = (*w).id;
            find.idx = (*wl).idx as u_int;
            csw = control_sub_windows_RB_FIND(&mut (*csub).windows, &mut find);
            if csw.is_null() {
                csw = xcalloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<control_sub_window>() as libc::c_ulong,
                ) as *mut control_sub_window;
                (*csw).window = (*w).id;
                (*csw).idx = (*wl).idx as u_int;
                control_sub_windows_RB_INSERT(&mut (*csub).windows, csw);
            }
            if !(*csw).last.is_null() && strcmp(value, (*csw).last) == 0 as libc::c_int {
                free(value as *mut libc::c_void);
            } else {
                control_write(
                    c,
                    b"%%subscription-changed %s $%u @%u %u - : %s\x00" as *const u8
                        as *const libc::c_char,
                    (*csub).name,
                    (*s).id,
                    (*w).id,
                    (*wl).idx,
                    value,
                );
                free((*csw).last as *mut libc::c_void);
                (*csw).last = value
            }
        }
        wl = (*wl).wentry.tqe_next
    }
}
/* Check all windows subscription. */
unsafe extern "C" fn control_check_subs_all_windows(
    mut c: *mut client,
    mut csub: *mut control_sub,
) {
    let mut s: *mut session = (*c).session;
    let mut w: *mut window = 0 as *mut window;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut ft: *mut crate::format::format_tree = 0 as *mut crate::format::format_tree;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut csw: *mut control_sub_window = 0 as *mut control_sub_window;
    let mut find: control_sub_window = control_sub_window {
        window: 0,
        idx: 0,
        last: 0 as *mut libc::c_char,
        entry: C2RustUnnamed_32 {
            rbe_left: 0 as *mut control_sub_window,
            rbe_right: 0 as *mut control_sub_window,
            rbe_parent: 0 as *mut control_sub_window,
            rbe_color: 0,
        },
    };
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        w = (*wl).window;
        ft = format_create_defaults(
            0 as *mut crate::cmd_queue::cmdq_item,
            c,
            s,
            wl,
            0 as *mut window_pane,
        );
        value = format_expand(ft, (*csub).format);
        format_free(ft);
        find.window = (*w).id;
        find.idx = (*wl).idx as u_int;
        csw = control_sub_windows_RB_FIND(&mut (*csub).windows, &mut find);
        if csw.is_null() {
            csw = xcalloc(
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<control_sub_window>() as libc::c_ulong,
            ) as *mut control_sub_window;
            (*csw).window = (*w).id;
            (*csw).idx = (*wl).idx as u_int;
            control_sub_windows_RB_INSERT(&mut (*csub).windows, csw);
        }
        if !(*csw).last.is_null() && strcmp(value, (*csw).last) == 0 as libc::c_int {
            free(value as *mut libc::c_void);
        } else {
            control_write(
                c,
                b"%%subscription-changed %s $%u @%u %u - : %s\x00" as *const u8
                    as *const libc::c_char,
                (*csub).name,
                (*s).id,
                (*w).id,
                (*wl).idx,
                value,
            );
            free((*csw).last as *mut libc::c_void);
            (*csw).last = value
        }
        wl = winlinks_RB_NEXT(wl)
    }
}
/* Check subscriptions timer. */
unsafe extern "C" fn control_check_subs_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut c: *mut client = data as *mut client;
    let mut cs: *mut control_state = (*c).control_state;
    let mut csub: *mut control_sub = 0 as *mut control_sub;
    let mut csub1: *mut control_sub = 0 as *mut control_sub;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 1 as libc::c_int as __time_t,
            tv_usec: 0,
        };
        init
    };
    log_debug(
        b"%s: timer fired\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"control_check_subs_timer\x00"))
            .as_ptr(),
    );
    event_add(&mut (*cs).subs_timer, &mut tv);
    csub = control_subs_RB_MINMAX(&mut (*cs).subs, -(1 as libc::c_int));
    while !csub.is_null() && {
        csub1 = control_subs_RB_NEXT(csub);
        (1 as libc::c_int) != 0
    } {
        match (*csub).type_0 as libc::c_uint {
            0 => {
                control_check_subs_session(c, csub);
            }
            1 => {
                control_check_subs_pane(c, csub);
            }
            2 => {
                control_check_subs_all_panes(c, csub);
            }
            3 => {
                control_check_subs_window(c, csub);
            }
            4 => {
                control_check_subs_all_windows(c, csub);
            }
            _ => {}
        }
        csub = csub1
    }
}
/* Add a subscription. */
#[no_mangle]
pub unsafe extern "C" fn control_add_sub(
    mut c: *mut client,
    mut name: *const libc::c_char,
    mut type_0: control_sub_type,
    mut id: libc::c_int,
    mut format: *const libc::c_char,
) {
    let mut cs: *mut control_state = (*c).control_state;
    let mut csub: *mut control_sub = 0 as *mut control_sub;
    let mut find: control_sub = control_sub {
        name: 0 as *mut libc::c_char,
        format: 0 as *mut libc::c_char,
        type_0: CONTROL_SUB_SESSION,
        id: 0,
        last: 0 as *mut libc::c_char,
        panes: control_sub_panes {
            rbh_root: 0 as *mut control_sub_pane,
        },
        windows: control_sub_windows {
            rbh_root: 0 as *mut control_sub_window,
        },
        entry: C2RustUnnamed_31 {
            rbe_left: 0 as *mut control_sub,
            rbe_right: 0 as *mut control_sub,
            rbe_parent: 0 as *mut control_sub,
            rbe_color: 0,
        },
    };
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 1 as libc::c_int as __time_t,
            tv_usec: 0,
        };
        init
    };
    find.name = name as *mut libc::c_char;
    csub = control_subs_RB_FIND(&mut (*cs).subs, &mut find);
    if !csub.is_null() {
        control_free_sub(cs, csub);
    }
    csub = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<control_sub>() as libc::c_ulong,
    ) as *mut control_sub;
    (*csub).name = xstrdup(name);
    (*csub).type_0 = type_0;
    (*csub).id = id as u_int;
    (*csub).format = xstrdup(format);
    control_subs_RB_INSERT(&mut (*cs).subs, csub);
    (*csub).panes.rbh_root = 0 as *mut control_sub_pane;
    (*csub).windows.rbh_root = 0 as *mut control_sub_window;
    if event_initialized(&mut (*cs).subs_timer) == 0 {
        event_set(
            &mut (*cs).subs_timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                control_check_subs_timer
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            c as *mut libc::c_void,
        );
    }
    if event_pending(
        &mut (*cs).subs_timer,
        0x1 as libc::c_int as libc::c_short,
        0 as *mut timeval,
    ) == 0
    {
        event_add(&mut (*cs).subs_timer, &mut tv);
    };
}
/* Remove a subscription. */
#[no_mangle]
pub unsafe extern "C" fn control_remove_sub(mut c: *mut client, mut name: *const libc::c_char) {
    let mut cs: *mut control_state = (*c).control_state;
    let mut csub: *mut control_sub = 0 as *mut control_sub;
    let mut find: control_sub = control_sub {
        name: 0 as *mut libc::c_char,
        format: 0 as *mut libc::c_char,
        type_0: CONTROL_SUB_SESSION,
        id: 0,
        last: 0 as *mut libc::c_char,
        panes: control_sub_panes {
            rbh_root: 0 as *mut control_sub_pane,
        },
        windows: control_sub_windows {
            rbh_root: 0 as *mut control_sub_window,
        },
        entry: C2RustUnnamed_31 {
            rbe_left: 0 as *mut control_sub,
            rbe_right: 0 as *mut control_sub,
            rbe_parent: 0 as *mut control_sub,
            rbe_color: 0,
        },
    };
    find.name = name as *mut libc::c_char;
    csub = control_subs_RB_FIND(&mut (*cs).subs, &mut find);
    if !csub.is_null() {
        control_free_sub(cs, csub);
    }
    if (*cs).subs.rbh_root.is_null() {
        event_del(&mut (*cs).subs_timer);
    };
}
