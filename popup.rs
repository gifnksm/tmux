use ::libc;
extern "C" {
    pub type event_base;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type screen_write_collect_line;
    pub type screen_sel;
    pub type screen_titles;
    pub type environ;
    pub type options;
    pub type format_tree;
    pub type input_ctx;
    pub type cmds;
    pub type tty_code;
    pub type format_job_tree;
    pub type control_state;
    pub type cmdq_list;
    pub type tmuxpeer;
    pub type cmdq_item;
    pub type cmdq_state;
    pub type job;
    pub type screen_write_collect_item;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char)
        -> *mut libc::c_char;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn fseeko(__stream: *mut FILE, __off: __off_t, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ftello(__stream: *mut FILE) -> __off_t;
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t) -> *mut libc::c_uchar;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut options;
    #[no_mangle]
    fn format_create(
        _: *mut client,
        _: *mut cmdq_item,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut format_tree;
    #[no_mangle]
    fn format_free(_: *mut format_tree);
    #[no_mangle]
    fn format_add(_: *mut format_tree, _: *const libc::c_char, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn format_expand(_: *mut format_tree, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn format_defaults(
        _: *mut format_tree,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    );
    #[no_mangle]
    fn format_draw(
        _: *mut screen_write_ctx,
        _: *const grid_cell,
        _: u_int,
        _: *const libc::c_char,
        _: *mut style_ranges,
    );
    #[no_mangle]
    fn format_width(_: *const libc::c_char) -> u_int;
    #[no_mangle]
    fn options_get_string(_: *mut options, _: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn job_run(
        _: *const libc::c_char,
        _: *mut session,
        _: *const libc::c_char,
        _: job_update_cb,
        _: job_complete_cb,
        _: job_free_cb,
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut job;
    #[no_mangle]
    fn job_free(_: *mut job);
    #[no_mangle]
    fn job_resize(_: *mut job, _: u_int, _: u_int);
    #[no_mangle]
    fn job_get_status(_: *mut job) -> libc::c_int;
    #[no_mangle]
    fn job_get_data(_: *mut job) -> *mut libc::c_void;
    #[no_mangle]
    fn job_get_event(_: *mut job) -> *mut bufferevent;
    #[no_mangle]
    fn tty_draw_line(
        _: *mut tty,
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: *const grid_cell,
        _: *mut libc::c_int,
    );
    #[no_mangle]
    fn cmd_find_valid_state(_: *mut cmd_find_state) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_copy_state(_: *mut cmd_find_state, _: *mut cmd_find_state);
    #[no_mangle]
    fn cmd_parse_and_append(
        _: *const libc::c_char,
        _: *mut cmd_parse_input,
        _: *mut client,
        _: *mut cmdq_state,
        _: *mut *mut libc::c_char,
    ) -> cmd_parse_status;
    #[no_mangle]
    fn cmdq_new_state(_: *mut cmd_find_state, _: *mut key_event, _: libc::c_int)
        -> *mut cmdq_state;
    #[no_mangle]
    fn cmdq_free_state(_: *mut cmdq_state);
    #[no_mangle]
    fn cmdq_get_client(_: *mut cmdq_item) -> *mut client;
    #[no_mangle]
    fn cmdq_get_event(_: *mut cmdq_item) -> *mut key_event;
    #[no_mangle]
    fn cmdq_get_error(_: *const libc::c_char) -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_append(_: *mut client, _: *mut cmdq_item) -> *mut cmdq_item;
    #[no_mangle]
    fn cmdq_continue(_: *mut cmdq_item);
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn server_client_set_overlay(
        _: *mut client,
        _: u_int,
        _: overlay_check_cb,
        _: overlay_mode_cb,
        _: overlay_draw_cb,
        _: overlay_key_cb,
        _: overlay_free_cb,
        _: *mut libc::c_void,
    );
    #[no_mangle]
    fn server_client_clear_overlay(_: *mut client);
    #[no_mangle]
    fn server_client_unref(_: *mut client);
    #[no_mangle]
    fn server_redraw_client(_: *mut client);
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut screen);
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_fast_copy(
        _: *mut screen_write_ctx,
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
    );
    #[no_mangle]
    fn screen_write_box(_: *mut screen_write_ctx, _: u_int, _: u_int);
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
    fn input_init(_: *mut window_pane, _: *mut bufferevent) -> *mut input_ctx;
    #[no_mangle]
    fn input_free(_: *mut input_ctx);
    #[no_mangle]
    fn input_parse_screen(
        _: *mut input_ctx,
        _: *mut screen,
        _: screen_write_init_ctx_cb,
        _: *mut libc::c_void,
        _: *mut u_char,
        _: size_t,
    );
    #[no_mangle]
    fn input_key(_: *mut screen, _: *mut bufferevent, _: key_code) -> libc::c_int;
    #[no_mangle]
    fn input_key_get_mouse(
        _: *mut screen,
        _: *mut mouse_event,
        _: u_int,
        _: u_int,
        _: *mut *const libc::c_char,
        _: *mut size_t,
    ) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ev_io: C2RustUnnamed_2,
    pub ev_signal: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ev_signal_next: C2RustUnnamed_1,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ev_io_next: C2RustUnnamed_3,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ev_next_with_common_timeout: C2RustUnnamed_5,
    pub min_heap_idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_7,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_6,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub evcb_callback:
        Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_short, _: *mut libc::c_void) -> ()>,
    pub evcb_selfcb:
        Option<unsafe extern "C" fn(_: *mut event_callback, _: *mut libc::c_void) -> ()>,
    pub evcb_evfinalize: Option<unsafe extern "C" fn(_: *mut event, _: *mut libc::c_void) -> ()>,
    pub evcb_cbfinalize:
        Option<unsafe extern "C" fn(_: *mut event_callback, _: *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_watermark {
    pub low: size_t,
    pub high: size_t,
}
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client {
    pub name: *const libc::c_char,
    pub peer: *mut tmuxpeer,
    pub queue: *mut cmdq_list,
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
    pub environ: *mut environ,
    pub jobs: *mut format_job_tree,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut client,
    pub tqe_prev: *mut *mut client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_files {
    pub rbh_root: *mut client_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_event {
    pub key: key_code,
    pub m: mouse_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen {
    pub title: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub titles: *mut screen_titles,
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
    pub sel: *mut screen_sel,
    pub write_list: *mut screen_write_collect_line,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid_cell {
    pub data: utf8_data,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utf8_data {
    pub data: [u_char; 21],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid {
    pub flags: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub hscrolled: u_int,
    pub hsize: u_int,
    pub hlimit: u_int,
    pub linedata: *mut grid_line,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct grid_line {
    pub cellused: u_int,
    pub cellsize: u_int,
    pub celldata: *mut grid_cell_entry,
    pub extdsize: u_int,
    pub extddata: *mut grid_extd_entry,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct grid_extd_entry {
    pub data: utf8_char,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}
pub type utf8_char = u_int;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub c2rust_unnamed: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub offset: u_int,
    pub data: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
pub type overlay_check_cb =
    Option<unsafe extern "C" fn(_: *mut client, _: u_int, _: u_int) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub options: *mut options,
    pub flags: libc::c_int,
    pub attached: u_int,
    pub tio: *mut termios,
    pub environ: *mut environ,
    pub references: libc::c_int,
    pub gentry: C2RustUnnamed_13,
    pub entry: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub tqe_next: *mut session,
    pub tqe_prev: *mut *mut session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winlinks {
    pub rbh_root: *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winlink {
    pub idx: libc::c_int,
    pub session: *mut session,
    pub window: *mut window,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_16,
    pub wentry: C2RustUnnamed_15,
    pub sentry: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub rbe_left: *mut winlink,
    pub rbe_right: *mut winlink,
    pub rbe_parent: *mut winlink,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub options: *mut options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_18,
    pub entry: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub tqe_next: *mut window,
    pub tqe_prev: *mut *mut window,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub tqe_next: *mut layout_cell,
    pub tqe_prev: *mut *mut layout_cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layout_cells {
    pub tqh_first: *mut layout_cell,
    pub tqh_last: *mut *mut layout_cell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_pane {
    pub id: u_int,
    pub active_point: u_int,
    pub window: *mut window,
    pub options: *mut options,
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
    pub ictx: *mut input_ctx,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub tqh_first: *mut window_mode_entry,
    pub tqh_last: *mut *mut window_mode_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_mode_entry {
    pub wp: *mut window_pane,
    pub swp: *mut window_pane,
    pub mode: *const window_mode,
    pub data: *mut libc::c_void,
    pub screen: *mut screen,
    pub prefix: u_int,
    pub entry: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub tqe_next: *mut window_mode_entry,
    pub tqe_prev: *mut *mut window_mode_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub formats: Option<unsafe extern "C" fn(_: *mut window_mode_entry, _: *mut format_tree) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_find_state {
    pub flags: libc::c_int,
    pub current: *mut cmd_find_state,
    pub s: *mut session,
    pub wl: *mut winlink,
    pub w: *mut window,
    pub wp: *mut window_pane,
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_pane_offset {
    pub used: size_t,
}
pub type layout_type = libc::c_uint;
pub const LAYOUT_WINDOWPANE: layout_type = 2;
pub const LAYOUT_TOPBOTTOM: layout_type = 1;
pub const LAYOUT_LEFTRIGHT: layout_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_panes {
    pub tqh_first: *mut window_pane,
    pub tqh_last: *mut *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub default_key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub rbe_left: *mut key_table,
    pub rbe_right: *mut key_table,
    pub rbe_parent: *mut key_table,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_bindings {
    pub rbh_root: *mut key_binding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_binding {
    pub key: key_code,
    pub cmdlist: *mut cmd_list,
    pub note: *const libc::c_char,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub rbe_left: *mut key_binding,
    pub rbe_right: *mut key_binding,
    pub rbe_parent: *mut key_binding,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_list {
    pub references: libc::c_int,
    pub group: u_int,
    pub list: *mut cmds,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_line {
    pub timer: event,
    pub screen: screen,
    pub active: *mut screen,
    pub references: libc::c_int,
    pub style: grid_cell,
    pub entries: [status_line_entry; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_line_entry {
    pub expanded: *mut libc::c_char,
    pub ranges: style_ranges,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct style_ranges {
    pub tqh_first: *mut style_range,
    pub tqh_last: *mut *mut style_range,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct style_range {
    pub type_0: style_range_type,
    pub argument: u_int,
    pub start: u_int,
    pub end: u_int,
    pub entry: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub tqe_next: *mut style_range,
    pub tqe_prev: *mut *mut style_range,
}
pub type style_range_type = libc::c_uint;
pub const STYLE_RANGE_WINDOW: style_range_type = 3;
pub const STYLE_RANGE_RIGHT: style_range_type = 2;
pub const STYLE_RANGE_LEFT: style_range_type = 1;
pub const STYLE_RANGE_NONE: style_range_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_key {
    pub ch: libc::c_char,
    pub key: key_code,
    pub left: *mut tty_key,
    pub right: *mut tty_key,
    pub next: *mut tty_key,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_term {
    pub name: *mut libc::c_char,
    pub tty: *mut tty,
    pub features: libc::c_int,
    pub acs: [[libc::c_char; 2]; 256],
    pub codes: *mut tty_code,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_windows {
    pub rbh_root: *mut client_window,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_window {
    pub window: u_int,
    pub pane: *mut window_pane,
    pub entry: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub rbe_left: *mut client_window,
    pub rbe_right: *mut client_window,
    pub rbe_parent: *mut client_window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_write_ctx {
    pub wp: *mut window_pane,
    pub s: *mut screen,
    pub flags: libc::c_int,
    pub init_ctx_cb: screen_write_init_ctx_cb,
    pub arg: *mut libc::c_void,
    pub item: *mut screen_write_collect_item,
    pub scrolled: u_int,
    pub bg: u_int,
    pub cells: u_int,
    pub written: u_int,
    pub skipped: u_int,
}
pub type screen_write_init_ctx_cb =
    Option<unsafe extern "C" fn(_: *mut screen_write_ctx, _: *mut tty_ctx) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tty_ctx {
    pub s: *mut screen,
    pub redraw_cb: tty_ctx_redraw_cb,
    pub set_client_cb: tty_ctx_set_client_cb,
    pub arg: *mut libc::c_void,
    pub cell: *const grid_cell,
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
    pub defaults: grid_cell,
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
pub type cmd_parse_status = libc::c_uint;
pub const CMD_PARSE_SUCCESS: cmd_parse_status = 2;
pub const CMD_PARSE_ERROR: cmd_parse_status = 1;
pub const CMD_PARSE_EMPTY: cmd_parse_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_parse_input {
    pub flags: libc::c_int,
    pub file: *const libc::c_char,
    pub line: u_int,
    pub item: *mut cmdq_item,
    pub c: *mut client,
    pub fs: cmd_find_state,
}
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut job) -> ()>;
pub type job_free_cb = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type popup_close_cb = Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_void) -> ()>;
pub type popup_finish_edit_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: *mut libc::c_void) -> ()>;
/* $OpenBSD$ */
/*
 * Copyright (c) 2020 Nicholas Marriott <nicholas.marriott@gmail.com>
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct popup_data {
    pub c: *mut client,
    pub item: *mut cmdq_item,
    pub flags: libc::c_int,
    pub lines: *mut *mut libc::c_char,
    pub nlines: u_int,
    pub cmd: *mut libc::c_char,
    pub fs: cmd_find_state,
    pub s: screen,
    pub job: *mut job,
    pub ictx: *mut input_ctx,
    pub status: libc::c_int,
    pub cb: popup_close_cb,
    pub arg: *mut libc::c_void,
    pub px: u_int,
    pub py: u_int,
    pub sx: u_int,
    pub sy: u_int,
    pub dragging: C2RustUnnamed_33,
    pub dx: u_int,
    pub dy: u_int,
    pub lx: u_int,
    pub ly: u_int,
    pub lb: u_int,
}
pub type C2RustUnnamed_33 = libc::c_uint;
pub const SIZE: C2RustUnnamed_33 = 2;
pub const MOVE: C2RustUnnamed_33 = 1;
pub const OFF: C2RustUnnamed_33 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct popup_editor {
    pub path: *mut libc::c_char,
    pub cb: popup_finish_edit_cb,
    pub arg: *mut libc::c_void,
}
unsafe extern "C" fn popup_redraw_cb(mut ttyctx: *const tty_ctx) {
    let mut pd: *mut popup_data = (*ttyctx).arg as *mut popup_data;
    (*(*pd).c).flags |= 0x2000000 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn popup_set_client_cb(
    mut ttyctx: *mut tty_ctx,
    mut c: *mut client,
) -> libc::c_int {
    let mut pd: *mut popup_data = (*ttyctx).arg as *mut popup_data;
    if c != (*pd).c {
        return 0 as libc::c_int;
    }
    if (*(*pd).c).flags & 0x2000000 as libc::c_int as libc::c_ulong != 0 {
        return 0 as libc::c_int;
    }
    (*ttyctx).bigger = 0 as libc::c_int;
    (*ttyctx).wox = 0 as libc::c_int as u_int;
    (*ttyctx).woy = 0 as libc::c_int as u_int;
    (*ttyctx).wsx = (*c).tty.sx;
    (*ttyctx).wsy = (*c).tty.sy;
    (*ttyctx).rxoff = (*pd).px.wrapping_add(1 as libc::c_int as libc::c_uint);
    (*ttyctx).xoff = (*ttyctx).rxoff;
    (*ttyctx).ryoff = (*pd).py.wrapping_add(1 as libc::c_int as libc::c_uint);
    (*ttyctx).yoff = (*ttyctx).ryoff;
    return 1 as libc::c_int;
}
unsafe extern "C" fn popup_init_ctx_cb(mut ctx: *mut screen_write_ctx, mut ttyctx: *mut tty_ctx) {
    let mut pd: *mut popup_data = (*ctx).arg as *mut popup_data;
    (*ttyctx).redraw_cb = Some(popup_redraw_cb as unsafe extern "C" fn(_: *const tty_ctx) -> ());
    (*ttyctx).set_client_cb = Some(
        popup_set_client_cb as unsafe extern "C" fn(_: *mut tty_ctx, _: *mut client) -> libc::c_int,
    );
    (*ttyctx).arg = pd as *mut libc::c_void;
}
unsafe extern "C" fn popup_write_screen(mut c: *mut client, mut pd: *mut popup_data) {
    let mut item: *mut cmdq_item = (*pd).item;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut loop_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut i: u_int = 0;
    let mut y: u_int = 0;
    ft = format_create(c, item, 0 as libc::c_int, 0 as libc::c_int);
    if cmd_find_valid_state(&mut (*pd).fs) != 0 {
        format_defaults(ft, c, (*pd).fs.s, (*pd).fs.wl, (*pd).fs.wp);
    } else {
        format_defaults(
            ft,
            c,
            0 as *mut session,
            0 as *mut winlink,
            0 as *mut window_pane,
        );
    }
    screen_write_start(&mut ctx, &mut (*pd).s);
    screen_write_clearscreen(&mut ctx, 8 as libc::c_int as u_int);
    y = 0 as libc::c_int as u_int;
    i = 0 as libc::c_int as u_int;
    while i < (*pd).nlines {
        if y == (*pd).sy.wrapping_sub(2 as libc::c_int as libc::c_uint) {
            break;
        }
        next = xstrdup(*(*pd).lines.offset(i as isize));
        copy = next;
        loop {
            loop_0 = strsep(&mut next, b"\n\x00" as *const u8 as *const libc::c_char);
            if loop_0.is_null() {
                break;
            }
            if y == (*pd).sy.wrapping_sub(2 as libc::c_int as libc::c_uint) {
                break;
            }
            tmp = format_expand(ft, loop_0);
            screen_write_cursormove(
                &mut ctx,
                0 as libc::c_int,
                y as libc::c_int,
                0 as libc::c_int,
            );
            format_draw(
                &mut ctx,
                &grid_default_cell,
                (*pd).sx.wrapping_sub(2 as libc::c_int as libc::c_uint),
                tmp,
                0 as *mut style_ranges,
            );
            free(tmp as *mut libc::c_void);
            y = y.wrapping_add(1)
        }
        free(copy as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    format_free(ft);
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int,
        y as libc::c_int,
        0 as libc::c_int,
    );
    screen_write_stop(&mut ctx);
}
unsafe extern "C" fn popup_mode_cb(
    mut c: *mut client,
    mut cx: *mut u_int,
    mut cy: *mut u_int,
) -> *mut screen {
    let mut pd: *mut popup_data = (*c).overlay_data as *mut popup_data;
    if (*pd).ictx.is_null() {
        return 0 as *mut screen;
    }
    *cx = (*pd)
        .px
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_add((*pd).s.cx);
    *cy = (*pd)
        .py
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_add((*pd).s.cy);
    return &mut (*pd).s;
}
unsafe extern "C" fn popup_check_cb(
    mut c: *mut client,
    mut px: u_int,
    mut py: u_int,
) -> libc::c_int {
    let mut pd: *mut popup_data = (*c).overlay_data as *mut popup_data;
    if px < (*pd).px
        || px
            > (*pd)
                .px
                .wrapping_add((*pd).sx)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return 1 as libc::c_int;
    }
    if py < (*pd).py
        || py
            > (*pd)
                .py
                .wrapping_add((*pd).sy)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn popup_draw_cb(mut c: *mut client, mut ctx0: *mut screen_redraw_ctx) {
    let mut pd: *mut popup_data = (*c).overlay_data as *mut popup_data;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut s: screen = screen {
        title: 0 as *mut libc::c_char,
        path: 0 as *mut libc::c_char,
        titles: 0 as *mut screen_titles,
        grid: 0 as *mut grid,
        cx: 0,
        cy: 0,
        cstyle: 0,
        ccolour: 0 as *mut libc::c_char,
        rupper: 0,
        rlower: 0,
        mode: 0,
        saved_cx: 0,
        saved_cy: 0,
        saved_grid: 0 as *mut grid,
        saved_cell: grid_cell {
            data: utf8_data {
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
        },
        saved_flags: 0,
        tabs: 0 as *mut bitstr_t,
        sel: 0 as *mut screen_sel,
        write_list: 0 as *mut screen_write_collect_line,
    };
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut i: u_int = 0;
    let mut px: u_int = (*pd).px;
    let mut py: u_int = (*pd).py;
    screen_init(&mut s, (*pd).sx, (*pd).sy, 0 as libc::c_int as u_int);
    screen_write_start(&mut ctx, &mut s);
    screen_write_clearscreen(&mut ctx, 8 as libc::c_int as u_int);
    screen_write_box(&mut ctx, (*pd).sx, (*pd).sy);
    screen_write_cursormove(
        &mut ctx,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    screen_write_fast_copy(
        &mut ctx,
        &mut (*pd).s,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
        (*pd).sx.wrapping_sub(2 as libc::c_int as libc::c_uint),
        (*pd).sy.wrapping_sub(2 as libc::c_int as libc::c_uint),
    );
    screen_write_stop(&mut ctx);
    (*c).overlay_check = None;
    i = 0 as libc::c_int as u_int;
    while i < (*pd).sy {
        tty_draw_line(
            tty,
            &mut s,
            0 as libc::c_int as u_int,
            i,
            (*pd).sx,
            px,
            py.wrapping_add(i),
            &grid_default_cell,
            0 as *mut libc::c_int,
        );
        i = i.wrapping_add(1)
    }
    (*c).overlay_check = Some(
        popup_check_cb as unsafe extern "C" fn(_: *mut client, _: u_int, _: u_int) -> libc::c_int,
    );
}
unsafe extern "C" fn popup_free_cb(mut c: *mut client) {
    let mut pd: *mut popup_data = (*c).overlay_data as *mut popup_data;
    let mut item: *mut cmdq_item = (*pd).item;
    let mut i: u_int = 0;
    if (*pd).cb.is_some() {
        (*pd).cb.expect("non-null function pointer")((*pd).status, (*pd).arg);
    }
    if !item.is_null() {
        if !(*pd).ictx.is_null()
            && !cmdq_get_client(item).is_null()
            && (*cmdq_get_client(item)).session.is_null()
        {
            (*cmdq_get_client(item)).retval = (*pd).status
        }
        cmdq_continue(item);
    }
    server_client_unref((*pd).c);
    if !(*pd).job.is_null() {
        job_free((*pd).job);
    }
    if !(*pd).ictx.is_null() {
        input_free((*pd).ictx);
    }
    i = 0 as libc::c_int as u_int;
    while i < (*pd).nlines {
        free(*(*pd).lines.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free((*pd).lines as *mut libc::c_void);
    screen_free(&mut (*pd).s);
    free((*pd).cmd as *mut libc::c_void);
    free(pd as *mut libc::c_void);
}
unsafe extern "C" fn popup_handle_drag(
    mut c: *mut client,
    mut pd: *mut popup_data,
    mut m: *mut mouse_event,
) {
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    if (*m).b & 32 as libc::c_int as libc::c_uint == 0 {
        (*pd).dragging = OFF
    } else if (*pd).dragging as libc::c_uint == MOVE as libc::c_int as libc::c_uint {
        if (*m).x < (*pd).dx {
            px = 0 as libc::c_int as u_int
        } else if (*m).x.wrapping_sub((*pd).dx).wrapping_add((*pd).sx) > (*c).tty.sx {
            px = (*c).tty.sx.wrapping_sub((*pd).sx)
        } else {
            px = (*m).x.wrapping_sub((*pd).dx)
        }
        if (*m).y < (*pd).dy {
            py = 0 as libc::c_int as u_int
        } else if (*m).y.wrapping_sub((*pd).dy).wrapping_add((*pd).sy) > (*c).tty.sy {
            py = (*c).tty.sy.wrapping_sub((*pd).sy)
        } else {
            py = (*m).y.wrapping_sub((*pd).dy)
        }
        (*pd).px = px;
        (*pd).py = py;
        (*pd).dx = (*m).x.wrapping_sub((*pd).px);
        (*pd).dy = (*m).y.wrapping_sub((*pd).py);
        server_redraw_client(c);
    } else if (*pd).dragging as libc::c_uint == SIZE as libc::c_int as libc::c_uint {
        if (*m).x < (*pd).px.wrapping_add(3 as libc::c_int as libc::c_uint) {
            return;
        }
        if (*m).y < (*pd).py.wrapping_add(3 as libc::c_int as libc::c_uint) {
            return;
        }
        (*pd).sx = (*m).x.wrapping_sub((*pd).px);
        (*pd).sy = (*m).y.wrapping_sub((*pd).py);
        screen_resize(
            &mut (*pd).s,
            (*pd).sx.wrapping_sub(2 as libc::c_int as libc::c_uint),
            (*pd).sy.wrapping_sub(2 as libc::c_int as libc::c_uint),
            0 as libc::c_int,
        );
        if (*pd).ictx.is_null() {
            popup_write_screen(c, pd);
        } else if !(*pd).job.is_null() {
            job_resize(
                (*pd).job,
                (*pd).sx.wrapping_sub(2 as libc::c_int as libc::c_uint),
                (*pd).sy.wrapping_sub(2 as libc::c_int as libc::c_uint),
            );
        }
        server_redraw_client(c);
    };
}
unsafe extern "C" fn popup_key_cb(mut c: *mut client, mut event: *mut key_event) -> libc::c_int {
    let mut current_block: u64;
    let mut pd: *mut popup_data = (*c).overlay_data as *mut popup_data;
    let mut m: *mut mouse_event = &mut (*event).m;
    let mut fs: *mut cmd_find_state = &mut (*pd).fs;
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut state: *mut cmdq_state = 0 as *mut cmdq_state;
    let mut status: cmd_parse_status = CMD_PARSE_EMPTY;
    let mut error: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*event).key & 0xfffffffffff as libc::c_ulonglong
        >= KEYC_MOUSE as libc::c_ulong as libc::c_ulonglong
        && ((*event).key & 0xfffffffffff as libc::c_ulonglong)
            < KEYC_BSPACE as libc::c_ulong as libc::c_ulonglong
    {
        if (*pd).dragging as libc::c_uint != OFF as libc::c_int as libc::c_uint {
            popup_handle_drag(c, pd, m);
            current_block = 937408387176680137;
        } else {
            if (*m).x < (*pd).px
                || (*m).x
                    > (*pd)
                        .px
                        .wrapping_add((*pd).sx)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                || (*m).y < (*pd).py
                || (*m).y
                    > (*pd)
                        .py
                        .wrapping_add((*pd).sy)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
            {
                if (*m).b & 3 as libc::c_int as libc::c_uint == 1 as libc::c_int as libc::c_uint {
                    return 1 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
            if (*m).b & 8 as libc::c_int as libc::c_uint != 0
                || (*m).x == (*pd).px
                || (*m).x
                    == (*pd)
                        .px
                        .wrapping_add((*pd).sx)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                || (*m).y == (*pd).py
                || (*m).y
                    == (*pd)
                        .py
                        .wrapping_add((*pd).sy)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
            {
                if (*m).b & 32 as libc::c_int as libc::c_uint == 0 {
                    current_block = 937408387176680137;
                } else {
                    if (*m).lb & 3 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        (*pd).dragging = MOVE
                    } else if (*m).lb & 3 as libc::c_int as libc::c_uint
                        == 2 as libc::c_int as libc::c_uint
                    {
                        (*pd).dragging = SIZE
                    }
                    (*pd).dx = (*m).lx.wrapping_sub((*pd).px);
                    (*pd).dy = (*m).ly.wrapping_sub((*pd).py);
                    current_block = 937408387176680137;
                }
            } else {
                current_block = 15089075282327824602;
            }
        }
        match current_block {
            15089075282327824602 => {}
            _ => {
                (*pd).lx = (*m).x;
                (*pd).ly = (*m).y;
                (*pd).lb = (*m).b;
                return 0 as libc::c_int;
            }
        }
    }
    if !(*pd).ictx.is_null() && (*pd).flags & 0x1 as libc::c_int != 0 {
        if ((*pd).flags & (0x2 as libc::c_int | 0x4 as libc::c_int) == 0 as libc::c_int
            || (*pd).job.is_null())
            && ((*event).key == '\u{1b}' as i32 as libc::c_ulonglong
                || (*event).key == '\u{3}' as i32 as libc::c_ulonglong)
        {
            return 1 as libc::c_int;
        }
        if (*pd).job.is_null() {
            return 0 as libc::c_int;
        }
        if (*event).key & 0xfffffffffff as libc::c_ulonglong
            >= KEYC_MOUSE as libc::c_ulong as libc::c_ulonglong
            && ((*event).key & 0xfffffffffff as libc::c_ulonglong)
                < KEYC_BSPACE as libc::c_ulong as libc::c_ulonglong
        {
            /* Must be inside, checked already. */
            if input_key_get_mouse(
                &mut (*pd).s,
                m,
                (*m).x.wrapping_sub((*pd).px),
                (*m).y.wrapping_sub((*pd).py),
                &mut buf,
                &mut len,
            ) == 0
            {
                return 0 as libc::c_int;
            } /* callback now owns buffer */
            bufferevent_write(job_get_event((*pd).job), buf as *const libc::c_void, len);
            return 0 as libc::c_int;
        }
        input_key(&mut (*pd).s, job_get_event((*pd).job), (*event).key);
        return 0 as libc::c_int;
    }
    if (*pd).cmd.is_null() {
        return 1 as libc::c_int;
    }
    ft = format_create(
        0 as *mut client,
        (*pd).item,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if cmd_find_valid_state(fs) != 0 {
        format_defaults(ft, c, (*fs).s, (*fs).wl, (*fs).wp);
    } else {
        format_defaults(
            ft,
            c,
            0 as *mut session,
            0 as *mut winlink,
            0 as *mut window_pane,
        );
    }
    format_add(
        ft,
        b"popup_key\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        key_string_lookup_key((*event).key, 0 as libc::c_int),
    );
    if (*event).key & 0xfffffffffff as libc::c_ulonglong
        >= KEYC_MOUSE as libc::c_ulong as libc::c_ulonglong
        && ((*event).key & 0xfffffffffff as libc::c_ulonglong)
            < KEYC_BSPACE as libc::c_ulong as libc::c_ulonglong
    {
        format_add(
            ft,
            b"popup_mouse\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
        format_add(
            ft,
            b"popup_mouse_x\x00" as *const u8 as *const libc::c_char,
            b"%u\x00" as *const u8 as *const libc::c_char,
            (*m).x.wrapping_sub((*pd).px),
        );
        format_add(
            ft,
            b"popup_mouse_y\x00" as *const u8 as *const libc::c_char,
            b"%u\x00" as *const u8 as *const libc::c_char,
            (*m).y.wrapping_sub((*pd).py),
        );
    }
    cmd = format_expand(ft, (*pd).cmd);
    format_free(ft);
    if !(*pd).item.is_null() {
        event = cmdq_get_event((*pd).item)
    } else {
        event = 0 as *mut key_event
    }
    state = cmdq_new_state(&mut (*pd).fs, event, 0 as libc::c_int);
    status = cmd_parse_and_append(cmd, 0 as *mut cmd_parse_input, c, state, &mut error);
    if status as libc::c_uint == CMD_PARSE_ERROR as libc::c_int as libc::c_uint {
        cmdq_append(c, cmdq_get_error(error));
        free(error as *mut libc::c_void);
    }
    cmdq_free_state(state);
    return 1 as libc::c_int;
}
unsafe extern "C" fn popup_job_update_cb(mut job: *mut job) {
    let mut pd: *mut popup_data = job_get_data(job) as *mut popup_data;
    let mut evb: *mut evbuffer = (*job_get_event(job)).input;
    let mut c: *mut client = (*pd).c;
    let mut s: *mut screen = &mut (*pd).s;
    let mut data: *mut libc::c_void =
        evbuffer_pullup(evb, -(1 as libc::c_int) as ssize_t) as *mut libc::c_void;
    let mut size: size_t = evbuffer_get_length(evb);
    if size == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    (*c).overlay_check = None;
    (*c).tty.flags &= !(0x2 as libc::c_int);
    input_parse_screen(
        (*pd).ictx,
        s,
        Some(
            popup_init_ctx_cb
                as unsafe extern "C" fn(_: *mut screen_write_ctx, _: *mut tty_ctx) -> (),
        ),
        pd as *mut libc::c_void,
        data as *mut u_char,
        size,
    );
    (*c).tty.flags |= 0x2 as libc::c_int;
    (*c).overlay_check = Some(
        popup_check_cb as unsafe extern "C" fn(_: *mut client, _: u_int, _: u_int) -> libc::c_int,
    );
    evbuffer_drain(evb, size);
}
unsafe extern "C" fn popup_job_complete_cb(mut job: *mut job) {
    let mut pd: *mut popup_data = job_get_data(job) as *mut popup_data;
    let mut status: libc::c_int = 0;
    status = job_get_status((*pd).job);
    if status & 0x7f as libc::c_int == 0 as libc::c_int {
        (*pd).status = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int
    } else if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar as libc::c_int
        >> 1 as libc::c_int
        > 0 as libc::c_int
    {
        (*pd).status = status & 0x7f as libc::c_int
    } else {
        (*pd).status = 0 as libc::c_int
    }
    (*pd).job = 0 as *mut job;
    if (*pd).flags & 0x2 as libc::c_int != 0
        || (*pd).flags & 0x4 as libc::c_int != 0 && (*pd).status == 0 as libc::c_int
    {
        server_client_clear_overlay((*pd).c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn popup_height(
    mut nlines: u_int,
    mut lines: *mut *const libc::c_char,
) -> u_int {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut loop_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    let mut height: u_int = 0 as libc::c_int as u_int;
    i = 0 as libc::c_int as u_int;
    while i < nlines {
        next = xstrdup(*lines.offset(i as isize));
        copy = next;
        loop {
            loop_0 = strsep(&mut next, b"\n\x00" as *const u8 as *const libc::c_char);
            if loop_0.is_null() {
                break;
            }
            height = height.wrapping_add(1)
        }
        free(copy as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    return height;
}
#[no_mangle]
pub unsafe extern "C" fn popup_width(
    mut item: *mut cmdq_item,
    mut nlines: u_int,
    mut lines: *mut *const libc::c_char,
    mut c: *mut client,
    mut fs: *mut cmd_find_state,
) -> u_int {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut loop_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut i: u_int = 0;
    let mut width: u_int = 0 as libc::c_int as u_int;
    let mut tmpwidth: u_int = 0;
    ft = format_create(
        cmdq_get_client(item),
        item,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if !fs.is_null() && cmd_find_valid_state(fs) != 0 {
        format_defaults(ft, c, (*fs).s, (*fs).wl, (*fs).wp);
    } else {
        format_defaults(
            ft,
            c,
            0 as *mut session,
            0 as *mut winlink,
            0 as *mut window_pane,
        );
    }
    i = 0 as libc::c_int as u_int;
    while i < nlines {
        next = xstrdup(*lines.offset(i as isize));
        copy = next;
        loop {
            loop_0 = strsep(&mut next, b"\n\x00" as *const u8 as *const libc::c_char);
            if loop_0.is_null() {
                break;
            }
            tmp = format_expand(ft, loop_0);
            tmpwidth = format_width(tmp);
            if tmpwidth > width {
                width = tmpwidth
            }
            free(tmp as *mut libc::c_void);
        }
        free(copy as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    format_free(ft);
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn popup_display(
    mut flags: libc::c_int,
    mut item: *mut cmdq_item,
    mut px: u_int,
    mut py: u_int,
    mut sx: u_int,
    mut sy: u_int,
    mut nlines: u_int,
    mut lines: *mut *const libc::c_char,
    mut shellcmd: *const libc::c_char,
    mut cmd: *const libc::c_char,
    mut cwd: *const libc::c_char,
    mut c: *mut client,
    mut fs: *mut cmd_find_state,
    mut cb: popup_close_cb,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut pd: *mut popup_data = 0 as *mut popup_data;
    let mut i: u_int = 0;
    let mut s: *mut session = 0 as *mut session;
    let mut jobflags: libc::c_int = 0;
    if sx < 3 as libc::c_int as libc::c_uint || sy < 3 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    if (*c).tty.sx < sx || (*c).tty.sy < sy {
        return -(1 as libc::c_int);
    }
    pd = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<popup_data>() as libc::c_ulong,
    ) as *mut popup_data;
    (*pd).item = item;
    (*pd).flags = flags;
    (*pd).c = c;
    (*(*pd).c).references += 1;
    (*pd).cb = cb;
    (*pd).arg = arg;
    (*pd).status = 128 as libc::c_int + 1 as libc::c_int;
    if !fs.is_null() {
        cmd_find_copy_state(&mut (*pd).fs, fs);
    }
    screen_init(
        &mut (*pd).s,
        sx.wrapping_sub(2 as libc::c_int as libc::c_uint),
        sy.wrapping_sub(2 as libc::c_int as libc::c_uint),
        0 as libc::c_int as u_int,
    );
    if !cmd.is_null() {
        (*pd).cmd = xstrdup(cmd)
    }
    (*pd).px = px;
    (*pd).py = py;
    (*pd).sx = sx;
    (*pd).sy = sy;
    (*pd).nlines = nlines;
    if (*pd).nlines != 0 as libc::c_int as libc::c_uint {
        (*pd).lines = xreallocarray(
            0 as *mut libc::c_void,
            (*pd).nlines as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char
    }
    i = 0 as libc::c_int as u_int;
    while i < (*pd).nlines {
        let ref mut fresh0 = *(*pd).lines.offset(i as isize);
        *fresh0 = xstrdup(*lines.offset(i as isize));
        i = i.wrapping_add(1)
    }
    popup_write_screen(c, pd);
    if !shellcmd.is_null() {
        if !fs.is_null() {
            s = (*fs).s
        } else {
            s = 0 as *mut session
        }
        jobflags = 0x1 as libc::c_int | 0x4 as libc::c_int;
        if flags & 0x1 as libc::c_int != 0 {
            jobflags |= 0x2 as libc::c_int
        }
        (*pd).job = job_run(
            shellcmd,
            s,
            cwd,
            Some(popup_job_update_cb as unsafe extern "C" fn(_: *mut job) -> ()),
            Some(popup_job_complete_cb as unsafe extern "C" fn(_: *mut job) -> ()),
            None,
            pd as *mut libc::c_void,
            jobflags,
            (*pd).sx.wrapping_sub(2 as libc::c_int as libc::c_uint) as libc::c_int,
            (*pd).sy.wrapping_sub(2 as libc::c_int as libc::c_uint) as libc::c_int,
        );
        (*pd).ictx = input_init(0 as *mut window_pane, job_get_event((*pd).job))
    }
    server_client_set_overlay(
        c,
        0 as libc::c_int as u_int,
        Some(
            popup_check_cb
                as unsafe extern "C" fn(_: *mut client, _: u_int, _: u_int) -> libc::c_int,
        ),
        Some(
            popup_mode_cb
                as unsafe extern "C" fn(
                    _: *mut client,
                    _: *mut u_int,
                    _: *mut u_int,
                ) -> *mut screen,
        ),
        Some(
            popup_draw_cb as unsafe extern "C" fn(_: *mut client, _: *mut screen_redraw_ctx) -> (),
        ),
        Some(
            popup_key_cb as unsafe extern "C" fn(_: *mut client, _: *mut key_event) -> libc::c_int,
        ),
        Some(popup_free_cb as unsafe extern "C" fn(_: *mut client) -> ()),
        pd as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn popup_editor_free(mut pe: *mut popup_editor) {
    unlink((*pe).path);
    free((*pe).path as *mut libc::c_void);
    free(pe as *mut libc::c_void);
}
unsafe extern "C" fn popup_editor_close_cb(mut status: libc::c_int, mut arg: *mut libc::c_void) {
    let mut pe: *mut popup_editor = arg as *mut popup_editor;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: off_t = 0 as libc::c_int as off_t;
    if status != 0 as libc::c_int {
        (*pe).cb.expect("non-null function pointer")(
            0 as *mut libc::c_char,
            0 as libc::c_int as size_t,
            (*pe).arg,
        );
        popup_editor_free(pe);
        return;
    }
    f = fopen((*pe).path, b"r\x00" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        fseeko(f, 0 as libc::c_int as __off_t, 2 as libc::c_int);
        len = ftello(f);
        fseeko(f, 0 as libc::c_int as __off_t, 0 as libc::c_int);
        if len == 0 as libc::c_int as libc::c_long
            || len as uintmax_t > 18446744073709551615 as libc::c_ulong
            || {
                buf = malloc(len as libc::c_ulong) as *mut libc::c_char;
                buf.is_null()
            }
            || fread(
                buf as *mut libc::c_void,
                len as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                f,
            ) != 1 as libc::c_int as libc::c_ulong
        {
            free(buf as *mut libc::c_void);
            buf = 0 as *mut libc::c_char;
            len = 0 as libc::c_int as off_t
        }
        fclose(f);
    }
    (*pe).cb.expect("non-null function pointer")(buf, len as size_t, (*pe).arg);
    popup_editor_free(pe);
}
#[no_mangle]
pub unsafe extern "C" fn popup_editor(
    mut c: *mut client,
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut cb: popup_finish_edit_cb,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut pe: *mut popup_editor = 0 as *mut popup_editor;
    let mut fd: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: [libc::c_char; 19] =
        *::std::mem::transmute::<&[u8; 19], &mut [libc::c_char; 19]>(b"/tmp/tmux.XXXXXXXX\x00");
    let mut editor: *const libc::c_char = 0 as *const libc::c_char;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    editor = options_get_string(
        global_options,
        b"editor\x00" as *const u8 as *const libc::c_char,
    );
    if *editor as libc::c_int == '\u{0}' as i32 {
        return -(1 as libc::c_int);
    }
    fd = mkstemp(path.as_mut_ptr());
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    f = fdopen(fd, b"w\x00" as *const u8 as *const libc::c_char);
    if fwrite(
        buf as *const libc::c_void,
        len,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        fclose(f);
        return -(1 as libc::c_int);
    }
    fclose(f);
    pe = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<popup_editor>() as libc::c_ulong,
    ) as *mut popup_editor;
    (*pe).path = xstrdup(path.as_mut_ptr());
    (*pe).cb = cb;
    (*pe).arg = arg;
    sx = (*c)
        .tty
        .sx
        .wrapping_mul(9 as libc::c_int as libc::c_uint)
        .wrapping_div(10 as libc::c_int as libc::c_uint);
    sy = (*c)
        .tty
        .sy
        .wrapping_mul(9 as libc::c_int as libc::c_uint)
        .wrapping_div(10 as libc::c_int as libc::c_uint);
    px = (*c)
        .tty
        .sx
        .wrapping_div(2 as libc::c_int as libc::c_uint)
        .wrapping_sub(sx.wrapping_div(2 as libc::c_int as libc::c_uint));
    py = (*c)
        .tty
        .sy
        .wrapping_div(2 as libc::c_int as libc::c_uint)
        .wrapping_sub(sy.wrapping_div(2 as libc::c_int as libc::c_uint));
    xasprintf(
        &mut cmd as *mut *mut libc::c_char,
        b"%s %s\x00" as *const u8 as *const libc::c_char,
        editor,
        path.as_mut_ptr(),
    );
    if popup_display(
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0 as *mut cmdq_item,
        px,
        py,
        sx,
        sy,
        0 as libc::c_int as u_int,
        0 as *mut *const libc::c_char,
        cmd,
        0 as *const libc::c_char,
        b"/tmp/\x00" as *const u8 as *const libc::c_char,
        c,
        0 as *mut cmd_find_state,
        Some(
            popup_editor_close_cb
                as unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_void) -> (),
        ),
        pe as *mut libc::c_void,
    ) != 0 as libc::c_int
    {
        popup_editor_free(pe);
        free(cmd as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    free(cmd as *mut libc::c_void);
    return 0 as libc::c_int;
}
