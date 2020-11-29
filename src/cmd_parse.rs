use ::libc;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    #[no_mangle]
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wctomb(__s: *mut libc::c_char, __wchar: wchar_t) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn getuid() -> __uid_t;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xvasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    static mut global_environ: *mut crate::environ::environ;
    #[no_mangle]
    fn format_true(_: *const libc::c_char) -> libc::c_int;
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
    fn environ_find(_: *mut crate::environ::environ, _: *const libc::c_char) -> *mut environ_entry;
    #[no_mangle]
    fn environ_put(_: *mut crate::environ::environ, _: *const libc::c_char, _: libc::c_int);
    #[no_mangle]
    fn cmd_find_valid_state(_: *mut cmd_find_state) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_from_client(_: *mut cmd_find_state, _: *mut client, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_log_argv(_: libc::c_int, _: *mut *mut libc::c_char, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn cmd_prepend_argv(_: *mut libc::c_int, _: *mut *mut *mut libc::c_char, _: *mut libc::c_char);
    #[no_mangle]
    fn cmd_append_argv(_: *mut libc::c_int, _: *mut *mut *mut libc::c_char, _: *mut libc::c_char);
    #[no_mangle]
    fn cmd_copy_argv(_: libc::c_int, _: *mut *mut libc::c_char) -> *mut *mut libc::c_char;
    #[no_mangle]
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char);
    #[no_mangle]
    fn cmd_stringify_argv(_: libc::c_int, _: *mut *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn cmd_get_alias(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn cmd_parse(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: u_int,
        _: *mut *mut libc::c_char,
    ) -> *mut crate::cmd::cmd;
    #[no_mangle]
    fn cmd_list_new() -> *mut cmd_list;
    #[no_mangle]
    fn cmd_list_append(_: *mut cmd_list, _: *mut crate::cmd::cmd);
    #[no_mangle]
    fn cmd_list_move(_: *mut cmd_list, _: *mut cmd_list);
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list);
    #[no_mangle]
    fn cmd_list_print(_: *mut cmd_list, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn cmdq_print(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn cmdq_insert_after(
        _: *mut crate::cmd_queue::cmdq_item,
        _: *mut crate::cmd_queue::cmdq_item,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_get_command(
        _: *mut cmd_list,
        _: *mut crate::cmd_queue::cmdq_state,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_append(
        _: *mut client,
        _: *mut crate::cmd_queue::cmdq_item,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
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
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
pub type wchar_t = libc::c_int;
pub type va_list = __builtin_va_list;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_5,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_0,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub ev_io: C2RustUnnamed_3,
    pub ev_signal: C2RustUnnamed_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub ev_signal_next: C2RustUnnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub ev_io_next: C2RustUnnamed_4,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_5 {
    pub ev_next_with_common_timeout: C2RustUnnamed_6,
    pub min_heap_idx: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_8,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_7,
    pub evcb_arg: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
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
    pub exit_type: C2RustUnnamed_29,
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
    pub prompt_mode: C2RustUnnamed_26,
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
    pub entry: C2RustUnnamed_9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
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
    pub entry: C2RustUnnamed_10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
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
    pub extddata: *mut crate::grid::ExtdEntry,
    pub flags: libc::c_int,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub c2rust_unnamed: C2RustUnnamed_11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_11 {
    pub offset: u_int,
    pub data: C2RustUnnamed_12,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_12 {
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
    pub gentry: C2RustUnnamed_14,
    pub entry: C2RustUnnamed_13,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_14 {
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
    pub entry: C2RustUnnamed_17,
    pub wentry: C2RustUnnamed_16,
    pub sentry: C2RustUnnamed_15,
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
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_17 {
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
    pub alerts_entry: C2RustUnnamed_20,
    pub options: *mut crate::options::options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_19,
    pub entry: C2RustUnnamed_18,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_18 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_19 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_20 {
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
    pub entry: C2RustUnnamed_21,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_21 {
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
    pub modes: C2RustUnnamed_24,
    pub searchstr: *mut libc::c_char,
    pub searchregex: libc::c_int,
    pub written: size_t,
    pub skipped: size_t,
    pub border_gc_set: libc::c_int,
    pub border_gc: crate::grid::Cell,
    pub entry: C2RustUnnamed_23,
    pub tree_entry: C2RustUnnamed_22,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_22 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_23 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_24 {
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
    pub entry: C2RustUnnamed_25,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_25 {
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
pub type C2RustUnnamed_26 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_26 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_26 = 0;
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
    pub entry: C2RustUnnamed_27,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_27 {
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
    pub entry: C2RustUnnamed_28,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_28 {
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
pub type C2RustUnnamed_29 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_29 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_29 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_29 = 0;

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
    pub entry: C2RustUnnamed_30,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_30 {
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
    pub entry: C2RustUnnamed_31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_31 {
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
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub rbe_left: *mut client_window,
    pub rbe_right: *mut client_window,
    pub rbe_parent: *mut client_window,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
}
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_parse_commands {
    pub tqh_first: *mut cmd_parse_command,
    pub tqh_last: *mut *mut cmd_parse_command,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_parse_command {
    pub line: u_int,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub entry: C2RustUnnamed_34,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_34 {
    pub tqe_next: *mut cmd_parse_command,
    pub tqe_prev: *mut *mut cmd_parse_command,
}
/* Skeleton name.  */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_parse_state {
    pub f: *mut FILE,
    pub buf: *const libc::c_char,
    pub len: size_t,
    pub off: size_t,
    pub condition: libc::c_int,
    pub eol: libc::c_int,
    pub eof: libc::c_int,
    pub input: *mut cmd_parse_input,
    pub escapes: u_int,
    pub error: *mut libc::c_char,
    pub commands: *mut cmd_parse_commands,
    pub scope: *mut cmd_parse_scope,
    pub stack: C2RustUnnamed_35,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_35 {
    pub tqh_first: *mut cmd_parse_scope,
    pub tqh_last: *mut *mut cmd_parse_scope,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_parse_scope {
    pub flag: libc::c_int,
    pub entry: C2RustUnnamed_36,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_36 {
    pub tqe_next: *mut cmd_parse_scope,
    pub tqe_prev: *mut *mut cmd_parse_scope,
}
/* Stored state numbers (used for stacks). */
pub type yy_state_t = yytype_int8;
/* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
<limits.h> and (if available) <stdint.h> are included
so that the code can choose integer types of a good width.  */
/* Narrow types that promote to a signed type and that can represent a
signed or unsigned integer of at least N bits.  In tables they can
save space and decrease cache pressure.  Promoting to a signed type
helps avoid bugs in integer arithmetic.  */
pub type yytype_int8 = libc::c_schar;
/* Value type.  */

#[repr(C)]
#[derive(Copy, Clone)]
pub union YYSTYPE {
    pub token: *mut libc::c_char,
    pub arguments: C2RustUnnamed_38,
    pub flag: libc::c_int,
    pub elif: C2RustUnnamed_37,
    pub commands: *mut cmd_parse_commands,
    pub command: *mut cmd_parse_command,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_37 {
    pub flag: libc::c_int,
    pub commands: *mut cmd_parse_commands,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_38 {
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
/* Symbol kind.  */
pub type yysymbol_kind_t = libc::c_int;
/* argument_statements  */
/* argument  */
pub const YYSYMBOL_argument_statements: yysymbol_kind_t = 37;
/* arguments  */
pub const YYSYMBOL_argument: yysymbol_kind_t = 36;
/* elif1  */
pub const YYSYMBOL_arguments: yysymbol_kind_t = 35;
/* condition1  */
pub const YYSYMBOL_elif1: yysymbol_kind_t = 34;
/* command  */
pub const YYSYMBOL_condition1: yysymbol_kind_t = 33;
/* commands  */
pub const YYSYMBOL_command: yysymbol_kind_t = 32;
/* elif  */
pub const YYSYMBOL_commands: yysymbol_kind_t = 31;
/* condition  */
pub const YYSYMBOL_elif: yysymbol_kind_t = 30;
/* if_close  */
pub const YYSYMBOL_condition: yysymbol_kind_t = 29;
/* if_elif  */
pub const YYSYMBOL_if_close: yysymbol_kind_t = 28;
/* if_else  */
pub const YYSYMBOL_if_elif: yysymbol_kind_t = 27;
/* if_open  */
pub const YYSYMBOL_if_else: yysymbol_kind_t = 26;
/* hidden_assignment  */
pub const YYSYMBOL_if_open: yysymbol_kind_t = 25;
/* assignment  */
pub const YYSYMBOL_hidden_assignment: yysymbol_kind_t = 24;
/* optional_assignment  */
pub const YYSYMBOL_assignment: yysymbol_kind_t = 23;
/* expanded  */
pub const YYSYMBOL_optional_assignment: yysymbol_kind_t = 22;
/* format  */
pub const YYSYMBOL_expanded: yysymbol_kind_t = 21;
/* statement  */
pub const YYSYMBOL_format: yysymbol_kind_t = 20;
/* statements  */
pub const YYSYMBOL_statement: yysymbol_kind_t = 19;
/* lines  */
pub const YYSYMBOL_statements: yysymbol_kind_t = 18;
/* $accept  */
pub const YYSYMBOL_lines: yysymbol_kind_t = 17;
/* '}'  */
pub const YYSYMBOL_YYACCEPT: yysymbol_kind_t = 16;
/* '{'  */
pub const YYSYMBOL_15_: yysymbol_kind_t = 15;
/* ';'  */
pub const YYSYMBOL_14_: yysymbol_kind_t = 14;
/* '\n'  */
pub const YYSYMBOL_13_: yysymbol_kind_t = 13;
/* EQUALS  */
pub const YYSYMBOL_12_n_: yysymbol_kind_t = 12;
/* TOKEN  */
pub const YYSYMBOL_EQUALS: yysymbol_kind_t = 11;
/* FORMAT  */
pub const YYSYMBOL_TOKEN: yysymbol_kind_t = 10;
/* ENDIF  */
pub const YYSYMBOL_FORMAT: yysymbol_kind_t = 9;
/* ELIF  */
pub const YYSYMBOL_ENDIF: yysymbol_kind_t = 8;
/* ELSE  */
pub const YYSYMBOL_ELIF: yysymbol_kind_t = 7;
/* IF  */
pub const YYSYMBOL_ELSE: yysymbol_kind_t = 6;
/* HIDDEN  */
pub const YYSYMBOL_IF: yysymbol_kind_t = 5;
/* ERROR  */
pub const YYSYMBOL_HIDDEN: yysymbol_kind_t = 4;
/* "invalid token"  */
pub const YYSYMBOL_ERROR: yysymbol_kind_t = 3;
/* error  */
pub const YYSYMBOL_YYUNDEF: yysymbol_kind_t = 2;
/* "end of file"  */
pub const YYSYMBOL_YYerror: yysymbol_kind_t = 1;
pub const YYSYMBOL_YYEOF: yysymbol_kind_t = 0;
pub const YYSYMBOL_YYEMPTY: yysymbol_kind_t = -2;
pub const YYEMPTY: yytokentype = -2;
/* State numbers in computations.  */
pub type yy_state_fast_t = libc::c_int;
pub type C2RustUnnamed_39 = libc::c_uint;
pub const SINGLE_QUOTES: C2RustUnnamed_39 = 3;
pub const DOUBLE_QUOTES: C2RustUnnamed_39 = 2;
pub const NONE: C2RustUnnamed_39 = 1;
pub const START: C2RustUnnamed_39 = 0;
/* !defined yyoverflow */
/* A type that is properly aligned for any stack member.  */

#[repr(C)]
#[derive(Copy, Clone)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
/* Debug traces.  */
/* Token kinds.  */
pub type yytokentype = libc::c_int;
/* EQUALS  */
/* TOKEN  */
pub const EQUALS: yytokentype = 266;
/* FORMAT  */
pub const TOKEN: yytokentype = 265;
/* ENDIF  */
pub const FORMAT: yytokentype = 264;
/* ELIF  */
pub const ENDIF: yytokentype = 263;
/* ELSE  */
pub const ELIF: yytokentype = 262;
/* IF  */
pub const ELSE: yytokentype = 261;
/* HIDDEN  */
pub const IF: yytokentype = 260;
/* ERROR  */
pub const HIDDEN: yytokentype = 259;
/* "invalid token"  */
pub const ERROR: yytokentype = 258;
/* error  */
pub const YYUNDEF_0: yytokentype = 257;
/* "end of file"  */
pub const YYerror_0: yytokentype = 256;
pub const YYEOF_0: yytokentype = 0;
static mut parse_state: cmd_parse_state = cmd_parse_state {
    f: 0 as *mut FILE,
    buf: 0 as *const libc::c_char,
    len: 0,
    off: 0,
    condition: 0,
    eol: 0,
    eof: 0,
    input: 0 as *mut cmd_parse_input,
    escapes: 0,
    error: 0 as *mut libc::c_char,
    commands: 0 as *mut cmd_parse_commands,
    scope: 0 as *mut cmd_parse_scope,
    stack: C2RustUnnamed_35 {
        tqh_first: 0 as *mut cmd_parse_scope,
        tqh_last: 0 as *mut *mut cmd_parse_scope,
    },
};
/* The size of an array large to enough to hold all stacks, each with
N elements.  */
unsafe extern "C" fn cmd_parse_get_error(
    mut file: *const libc::c_char,
    mut line: u_int,
    mut error: *const libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if file.is_null() {
        s = xstrdup(error)
    } else {
        xasprintf(
            &mut s as *mut *mut libc::c_char,
            b"%s:%u: %s\x00" as *const u8 as *const libc::c_char,
            file,
            line,
            error,
        );
    }
    return s;
}
unsafe extern "C" fn cmd_parse_print_commands(
    mut pi: *mut cmd_parse_input,
    mut line: u_int,
    mut cmdlist: *mut cmd_list,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*pi).item.is_null() && (*pi).flags & 0x8i32 != 0 {
        /* Copy COUNT objects from SRC to DST.  The source and destination do
        not overlap.  */
        s = cmd_list_print(cmdlist, 0i32);
        if !(*pi).file.is_null() {
            cmdq_print(
                (*pi).item,
                b"%s:%u: %s\x00" as *const u8 as *const libc::c_char,
                (*pi).file,
                line,
                s,
            );
        } else {
            cmdq_print(
                (*pi).item,
                b"%u: %s\x00" as *const u8 as *const libc::c_char,
                line,
                s,
            );
        }
        free(s as *mut libc::c_void);
    };
}
unsafe extern "C" fn cmd_parse_free_command(mut cmd: *mut cmd_parse_command) {
    cmd_free_argv((*cmd).argc, (*cmd).argv);
    free(cmd as *mut libc::c_void);
}
unsafe extern "C" fn cmd_parse_new_commands() -> *mut cmd_parse_commands
/* !YYCOPY_NEEDED */ {
    /* YYFINAL -- State number of the termination state.  */
    let mut cmds: *mut cmd_parse_commands = 0 as *mut cmd_parse_commands;
    /* YYLAST -- Last index in YYTABLE.  */
    cmds = xmalloc(::std::mem::size_of::<cmd_parse_commands>() as libc::c_ulong)
        as *mut cmd_parse_commands;
    (*cmds).tqh_first = 0 as *mut cmd_parse_command;
    (*cmds).tqh_last = &mut (*cmds).tqh_first;
    return cmds;
}
unsafe extern "C" fn cmd_parse_free_commands(mut cmds: *mut cmd_parse_commands) {
    /* YYNRULES -- Number of rules.  */
    let mut cmd: *mut cmd_parse_command = 0 as *mut cmd_parse_command;
    let mut cmd1: *mut cmd_parse_command = 0 as *mut cmd_parse_command;
    /* YYNSTATES -- Number of states.  */
    cmd = (*cmds).tqh_first;
    while !cmd.is_null() && {
        cmd1 = (*cmd).entry.tqe_next;
        (1i32) != 0
    } {
        if !(*cmd).entry.tqe_next.is_null() {
            (*(*cmd).entry.tqe_next).entry.tqe_prev = (*cmd).entry.tqe_prev
        } else {
            (*cmds).tqh_last = (*cmd).entry.tqe_prev
        }
        *(*cmd).entry.tqe_prev = (*cmd).entry.tqe_next;
        /* YYMAXUTOK -- Last valid token kind.  */
        cmd_parse_free_command(cmd);
        cmd = cmd1
    }
    free(cmds as *mut libc::c_void);
}
/* YYTRANSLATE(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
as returned by yylex, with out-of-bounds checking.  */
unsafe extern "C" fn cmd_parse_commands_to_string(
    mut cmds: *mut cmd_parse_commands,
) -> *mut libc::c_char {
    let mut cmd: *mut cmd_parse_command = 0 as *mut cmd_parse_command;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    /* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
    as returned by yylex.  */
    cmd = (*cmds).tqh_first;
    while !cmd.is_null() {
        line = cmd_stringify_argv((*cmd).argc, (*cmd).argv);
        if string.is_null() {
            s = line
        } else {
            xasprintf(
                &mut s as *mut *mut libc::c_char,
                b"%s ; %s\x00" as *const u8 as *const libc::c_char,
                s,
                line,
            );
            free(line as *mut libc::c_void);
        }
        free(string as *mut libc::c_void);
        string = s;
        cmd = (*cmd).entry.tqe_next
    }
    if string.is_null() {
        string = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
    }
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
            b"cmd_parse_commands_to_string\x00",
        ))
        .as_ptr(),
        string,
    );
    return string;
}
static mut yytranslate: [yytype_int8; 267] = [
    0i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 12i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 13i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 14i8, 2i8, 15i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8,
    2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 2i8, 1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8, 8i8, 9i8,
    10i8, 11i8,
];
unsafe extern "C" fn cmd_parse_run_parser(
    mut cause: *mut *mut libc::c_char,
) -> *mut cmd_parse_commands {
    let mut ps: *mut cmd_parse_state = &mut parse_state;
    let mut scope: *mut cmd_parse_scope = 0 as *mut cmd_parse_scope;
    let mut scope1: *mut cmd_parse_scope = 0 as *mut cmd_parse_scope;
    let mut retval: libc::c_int = 0;
    (*ps).commands = 0 as *mut cmd_parse_commands;
    (*ps).stack.tqh_first = 0 as *mut cmd_parse_scope;
    (*ps).stack.tqh_last = &mut (*ps).stack.tqh_first;
    retval = yyparse();
    scope = (*ps).stack.tqh_first;
    while !scope.is_null() && {
        scope1 = (*scope).entry.tqe_next;
        (1i32) != 0
    } {
        if !(*scope).entry.tqe_next.is_null() {
            (*(*scope).entry.tqe_next).entry.tqe_prev = (*scope).entry.tqe_prev
        } else {
            (*ps).stack.tqh_last = (*scope).entry.tqe_prev
        }
        *(*scope).entry.tqe_prev = (*scope).entry.tqe_next;
        free(scope as *mut libc::c_void);
        scope = scope1
    }
    if retval != 0i32 {
        *cause = (*ps).error;
        return 0 as *mut cmd_parse_commands;
    }
    if (*ps).commands.is_null() {
        return cmd_parse_new_commands();
    }
    return (*ps).commands;
}
unsafe extern "C" fn cmd_parse_do_file(
    mut f: *mut FILE,
    mut pi: *mut cmd_parse_input,
    mut cause: *mut *mut libc::c_char,
) -> *mut cmd_parse_commands {
    /* * Accessing symbol of state STATE.  */
    let mut ps: *mut cmd_parse_state = &mut parse_state;
    memset(
        ps as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<cmd_parse_state>() as libc::c_ulong,
    );
    (*ps).input = pi;
    (*ps).f = f;
    return cmd_parse_run_parser(cause);
}
unsafe extern "C" fn cmd_parse_do_buffer(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut pi: *mut cmd_parse_input,
    mut cause: *mut *mut libc::c_char,
) -> *mut cmd_parse_commands {
    let mut ps: *mut cmd_parse_state = &mut parse_state;
    memset(
        ps as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<cmd_parse_state>() as libc::c_ulong,
    );
    (*ps).input = pi;
    (*ps).buf = buf;
    (*ps).len = len;
    return cmd_parse_run_parser(cause);
}
unsafe extern "C" fn cmd_parse_build_commands(
    mut cmds: *mut cmd_parse_commands,
    mut pi: *mut cmd_parse_input,
) -> *mut cmd_parse_result {
    let mut current_block: u64;
    static mut pr: cmd_parse_result = cmd_parse_result {
        status: CMD_PARSE_EMPTY,
        cmdlist: 0 as *mut cmd_list,
        error: 0 as *mut libc::c_char,
    };
    let mut cmds2: *mut cmd_parse_commands = 0 as *mut cmd_parse_commands;
    let mut cmd: *mut cmd_parse_command = 0 as *mut cmd_parse_command;
    let mut cmd2: *mut cmd_parse_command = 0 as *mut cmd_parse_command;
    let mut next: *mut cmd_parse_command = 0 as *mut cmd_parse_command;
    let mut next2: *mut cmd_parse_command = 0 as *mut cmd_parse_command;
    let mut after: *mut cmd_parse_command = 0 as *mut cmd_parse_command;
    let mut line: u_int = (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32);
    let mut i: libc::c_int = 0;
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    let mut result: *mut cmd_list = 0 as *mut cmd_list;
    let mut add: *mut crate::cmd::cmd = 0 as *mut crate::cmd::cmd;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alias: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Check for an empty list. */
    if (*cmds).tqh_first.is_null() {
        cmd_parse_free_commands(cmds);
        pr.status = CMD_PARSE_EMPTY;
        return &mut pr;
    }
    /*
     * Walk the commands and expand any aliases. Each alias is parsed
     * individually to a new command list, any trailing arguments appended
     * to the last command, and all commands inserted into the original
     * command list.
     */
    cmd = (*cmds).tqh_first;
    loop {
        if !(!cmd.is_null() && {
            next = (*cmd).entry.tqe_next;
            (1i32) != 0
        }) {
            current_block = 5807581744382915773;
            break;
        }
        name = *(*cmd).argv.offset(0isize);
        alias = cmd_get_alias(name);
        if !alias.is_null() {
            line = (*cmd).line;
            log_debug(
                b"%s: %u %s = %s\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"cmd_parse_build_commands\x00",
                ))
                .as_ptr(),
                line,
                name,
                alias,
            );
            (*pi).line = line;
            cmds2 = cmd_parse_do_buffer(alias, strlen(alias), pi, &mut cause);
            free(alias as *mut libc::c_void);
            if cmds2.is_null() {
                /* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
                Performed when YYTABLE does not specify something else to do.  Zero
                means the default is an error.  */
                pr.status = CMD_PARSE_ERROR;
                pr.error = cause;
                current_block = 14163653266681795021;
                break;
            } else {
                cmd2 = *(*((*cmds2).tqh_last as *mut cmd_parse_commands)).tqh_last;
                if cmd2.is_null() {
                    if !(*cmd).entry.tqe_next.is_null() {
                        (*(*cmd).entry.tqe_next).entry.tqe_prev = (*cmd).entry.tqe_prev
                    } else {
                        (*cmds).tqh_last = (*cmd).entry.tqe_prev
                    }
                    *(*cmd).entry.tqe_prev = (*cmd).entry.tqe_next;
                    cmd_parse_free_command(cmd);
                } else {
                    i = 1i32;
                    while i < (*cmd).argc {
                        cmd_append_argv(
                            &mut (*cmd2).argc,
                            &mut (*cmd2).argv,
                            *(*cmd).argv.offset(i as isize),
                        );
                        i += 1
                    }
                    after = cmd;
                    cmd2 = (*cmds2).tqh_first;
                    while !cmd2.is_null() && {
                        next2 = (*cmd2).entry.tqe_next;
                        (1i32) != 0
                    } {
                        (*cmd2).line = line;
                        if !(*cmd2).entry.tqe_next.is_null() {
                            (*(*cmd2).entry.tqe_next).entry.tqe_prev = (*cmd2).entry.tqe_prev
                        } else {
                            (*cmds2).tqh_last = (*cmd2).entry.tqe_prev
                        }
                        *(*cmd2).entry.tqe_prev = (*cmd2).entry.tqe_next;
                        (*cmd2).entry.tqe_next = (*after).entry.tqe_next;
                        if !(*cmd2).entry.tqe_next.is_null() {
                            (*(*cmd2).entry.tqe_next).entry.tqe_prev = &mut (*cmd2).entry.tqe_next
                        } else {
                            (*cmds).tqh_last = &mut (*cmd2).entry.tqe_next
                        }
                        (*after).entry.tqe_next = cmd2;
                        (*cmd2).entry.tqe_prev = &mut (*after).entry.tqe_next;
                        after = cmd2;
                        cmd2 = next2
                    }
                    cmd_parse_free_commands(cmds2);
                    if !(*cmd).entry.tqe_next.is_null() {
                        (*(*cmd).entry.tqe_next).entry.tqe_prev = (*cmd).entry.tqe_prev
                    } else {
                        (*cmds).tqh_last = (*cmd).entry.tqe_prev
                    }
                    *(*cmd).entry.tqe_prev = (*cmd).entry.tqe_next;
                    cmd_parse_free_command(cmd);
                }
            }
        }
        cmd = next
    }
    match current_block {
        5807581744382915773 => {
            /*
             * Parse each command into a command list. Create a new command list
             * for each line (unless the flag is set) so they get a new group (so
             * the queue knows which ones to remove if a command fails when
             * executed).
             */
            /* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
            positive, shift that token.  If negative, reduce the rule whose
            number is the opposite.  If YYTABLE_NINF, syntax error.  */
            result = cmd_list_new();
            cmd = (*cmds).tqh_first;
            loop {
                if cmd.is_null() {
                    current_block = 12027283704867122503;
                    break;
                }
                name = *(*cmd).argv.offset(0isize);
                log_debug(
                    b"%s: %u %s\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                        b"cmd_parse_build_commands\x00",
                    ))
                    .as_ptr(),
                    (*cmd).line,
                    name,
                );
                cmd_log_argv(
                    (*cmd).argc,
                    (*cmd).argv,
                    (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                        b"cmd_parse_build_commands\x00",
                    ))
                    .as_ptr(),
                );
                if cmdlist.is_null() || !(*pi).flags & 0x10i32 != 0 && (*cmd).line != line {
                    if !cmdlist.is_null() {
                        cmd_parse_print_commands(pi, line, cmdlist);
                        cmd_list_move(result, cmdlist);
                        cmd_list_free(cmdlist);
                    }
                    cmdlist = cmd_list_new()
                }
                line = (*cmd).line;
                add = cmd_parse((*cmd).argc, (*cmd).argv, (*pi).file, line, &mut cause);
                if add.is_null() {
                    cmd_list_free(result);
                    pr.status = CMD_PARSE_ERROR;
                    pr.error = cmd_parse_get_error((*pi).file, line, cause);
                    free(cause as *mut libc::c_void);
                    cmd_list_free(cmdlist);
                    current_block = 14163653266681795021;
                    break;
                } else {
                    cmd_list_append(cmdlist, add);
                    cmd = (*cmd).entry.tqe_next
                }
            }
            match current_block {
                14163653266681795021 => {}
                _ => {
                    if !cmdlist.is_null() {
                        cmd_parse_print_commands(pi, line, cmdlist);
                        cmd_list_move(result, cmdlist);
                        cmd_list_free(cmdlist);
                    }
                    s = cmd_list_print(result, 0i32);
                    log_debug(
                        b"%s: %s\x00" as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                            b"cmd_parse_build_commands\x00",
                        ))
                        .as_ptr(),
                        s,
                    );
                    free(s as *mut libc::c_void);
                    pr.status = CMD_PARSE_SUCCESS;
                    pr.cmdlist = result
                }
            }
        }
        _ => {}
    }
    cmd_parse_free_commands(cmds);
    return &mut pr;
}
static mut yypact: [yytype_int8; 75] = [
    49i8, 3i8, 14i8, -32i8, 33i8, 49i8, 29i8, 47i8, 60i8, -32i8, 4i8, -32i8, 35i8, -32i8, -32i8,
    -32i8, -32i8, -32i8, -32i8, -32i8, -32i8, 68i8, -32i8, 83i8, 81i8, 38i8, 5i8, 17i8, -32i8,
    -32i8, -32i8, 81i8, -32i8, 83i8, 71i8, -32i8, 14i8, -32i8, 38i8, 38i8, -32i8, -1i8, -32i8,
    -32i8, 81i8, 40i8, -32i8, -32i8, 76i8, 86i8, -32i8, -1i8, -32i8, 32i8, 58i8, 38i8, -32i8, 84i8,
    -32i8, 81i8, 81i8, 88i8, -32i8, -32i8, -32i8, 32i8, -32i8, 79i8, 62i8, 81i8, -32i8, -32i8,
    -32i8, 79i8, -32i8,
];
static mut yydefact: [yytype_int8; 75] = [
    2i8, 0i8, 0i8, 15i8, 0i8, 3i8, 0i8, 0i8, 32i8, 7i8, 13i8, 8i8, 9i8, 27i8, 31i8, 16i8, 10i8,
    11i8, 12i8, 17i8, 1i8, 0i8, 4i8, 33i8, 6i8, 13i8, 0i8, 28i8, 5i8, 43i8, 44i8, 6i8, 34i8, 41i8,
    6i8, 18i8, 0i8, 20i8, 13i8, 13i8, 35i8, 0i8, 30i8, 29i8, 6i8, 0i8, 45i8, 42i8, 0i8, 0i8, 21i8,
    0i8, 19i8, 0i8, 39i8, 13i8, 37i8, 0i8, 46i8, 6i8, 6i8, 0i8, 23i8, 36i8, 40i8, 0i8, 47i8, 6i8,
    25i8, 6i8, 38i8, 22i8, 26i8, 6i8, 24i8,
];
static mut yypgoto: [yytype_int8; 22] = [
    -32i8, -32i8, -23i8, -5i8, -32i8, 59i8, -32i8, -32i8, -32i8, -8i8, -31i8, -30i8, -9i8, -32i8,
    -18i8, -4i8, 74i8, 75i8, 50i8, 70i8, -32i8, -32i8,
];
static mut yydefgoto: [yytype_int8; 22] = [
    -1i8, 4i8, 5i8, 6i8, 18i8, 19i8, 7i8, 8i8, 9i8, 10i8, 38i8, 39i8, 40i8, 11i8, 51i8, 12i8, 13i8,
    14i8, 41i8, 32i8, 33i8, 46i8,
];
static mut yytable: [yytype_int8; 105] = [
    21i8, 34i8, 25i8, 48i8, 49i8, 35i8, 26i8, 37i8, 44i8, 2i8, 55i8, 35i8, 36i8, 37i8, 15i8, 3i8,
    24i8, 25i8, 27i8, 25i8, 61i8, 26i8, 2i8, 16i8, 17i8, 50i8, 45i8, -13i8, 3i8, 21i8, 25i8, 25i8,
    56i8, 20i8, 53i8, 54i8, 67i8, 68i8, 49i8, 57i8, 37i8, 22i8, 62i8, 2i8, 63i8, 27i8, 73i8, 25i8,
    27i8, 3i8, 72i8, 65i8, 22i8, 1i8, 2i8, 58i8, 70i8, 23i8, 71i8, -13i8, 3i8, -6i8, 21i8, 21i8,
    74i8, 36i8, 1i8, 2i8, 21i8, 36i8, -14i8, 27i8, -13i8, 3i8, -6i8, 1i8, 2i8, 35i8, 36i8, 37i8,
    28i8, -13i8, 3i8, 1i8, 2i8, 1i8, 2i8, 37i8, 59i8, -13i8, 3i8, -13i8, 3i8, 29i8, 30i8, 52i8,
    28i8, 31i8, 60i8, 66i8, 69i8, 42i8, 43i8, 47i8, 64i8,
];
static mut yycheck: [yytype_int8; 105] = [
    5i8, 24i8, 10i8, 34i8, 34i8, 6i8, 10i8, 8i8, 31i8, 5i8, 41i8, 6i8, 7i8, 8i8, 11i8, 11i8, 12i8,
    25i8, 13i8, 27i8, 51i8, 25i8, 5i8, 9i8, 10i8, 34i8, 31i8, 10i8, 11i8, 34i8, 38i8, 39i8, 41i8,
    0i8, 38i8, 39i8, 59i8, 60i8, 68i8, 44i8, 8i8, 12i8, 51i8, 5i8, 53i8, 13i8, 69i8, 55i8, 13i8,
    11i8, 68i8, 55i8, 12i8, 4i8, 5i8, 15i8, 65i8, 10i8, 67i8, 10i8, 11i8, 12i8, 67i8, 68i8, 73i8,
    7i8, 4i8, 5i8, 73i8, 7i8, 10i8, 13i8, 10i8, 11i8, 12i8, 4i8, 5i8, 6i8, 7i8, 8i8, 12i8, 10i8,
    11i8, 4i8, 5i8, 4i8, 5i8, 8i8, 12i8, 10i8, 11i8, 10i8, 11i8, 10i8, 11i8, 36i8, 12i8, 14i8,
    12i8, 15i8, 12i8, 27i8, 27i8, 33i8, 54i8,
];
static mut yystos: [yytype_int8; 75] = [
    0i8, 4i8, 5i8, 11i8, 17i8, 18i8, 19i8, 22i8, 23i8, 24i8, 25i8, 29i8, 31i8, 32i8, 33i8, 11i8,
    9i8, 10i8, 20i8, 21i8, 0i8, 19i8, 12i8, 10i8, 12i8, 25i8, 31i8, 13i8, 12i8, 10i8, 11i8, 14i8,
    35i8, 36i8, 18i8, 6i8, 7i8, 8i8, 26i8, 27i8, 28i8, 34i8, 32i8, 33i8, 18i8, 19i8, 37i8, 35i8,
    26i8, 27i8, 28i8, 30i8, 21i8, 31i8, 31i8, 26i8, 28i8, 19i8, 15i8, 12i8, 12i8, 26i8, 28i8, 28i8,
    34i8, 31i8, 15i8, 18i8, 18i8, 12i8, 28i8, 28i8, 30i8, 18i8, 28i8,
];
/* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static mut yyr1: [yytype_int8; 48] = [
    0i8, 16i8, 17i8, 17i8, 18i8, 18i8, 19i8, 19i8, 19i8, 19i8, 20i8, 20i8, 21i8, 22i8, 22i8, 23i8,
    24i8, 25i8, 26i8, 27i8, 28i8, 29i8, 29i8, 29i8, 29i8, 30i8, 30i8, 31i8, 31i8, 31i8, 31i8, 31i8,
    32i8, 32i8, 32i8, 33i8, 33i8, 33i8, 33i8, 34i8, 34i8, 35i8, 35i8, 36i8, 36i8, 36i8, 37i8, 37i8,
];
#[no_mangle]
pub unsafe extern "C" fn cmd_parse_from_file(
    mut f: *mut FILE,
    mut pi: *mut cmd_parse_input,
) -> *mut cmd_parse_result {
    static mut pr: cmd_parse_result = cmd_parse_result {
        status: CMD_PARSE_EMPTY,
        cmdlist: 0 as *mut cmd_list,
        error: 0 as *mut libc::c_char,
    };
    let mut input: cmd_parse_input = cmd_parse_input {
        flags: 0,
        file: 0 as *const libc::c_char,
        line: 0,
        item: 0 as *mut crate::cmd_queue::cmdq_item,
        c: 0 as *mut client,
        fs: cmd_find_state {
            flags: 0,
            current: 0 as *mut cmd_find_state,
            s: 0 as *mut session,
            wl: 0 as *mut winlink,
            w: 0 as *mut window,
            wp: 0 as *mut window_pane,
            idx: 0,
        },
    };
    let mut cmds: *mut cmd_parse_commands = 0 as *mut cmd_parse_commands;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    if pi.is_null() {
        memset(
            &mut input as *mut cmd_parse_input as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<cmd_parse_input>() as libc::c_ulong,
        );
        pi = &mut input
    }
    memset(
        &mut pr as *mut cmd_parse_result as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<cmd_parse_result>() as libc::c_ulong,
    );
    cmds = cmd_parse_do_file(f, pi, &mut cause);
    if cmds.is_null() {
        pr.status = CMD_PARSE_ERROR;
        pr.error = cause;
        return &mut pr;
    }
    return cmd_parse_build_commands(cmds, pi);
}
static mut yyr2: [yytype_int8; 48] = [
    0i8, 2i8, 0i8, 1i8, 2i8, 3i8, 0i8, 1i8, 1i8, 1i8, 1i8, 1i8, 1i8, 0i8, 1i8, 1i8, 2i8, 2i8, 1i8,
    2i8, 1i8, 4i8, 7i8, 5i8, 8i8, 3i8, 4i8, 1i8, 2i8, 3i8, 3i8, 1i8, 1i8, 2i8, 3i8, 3i8, 5i8, 4i8,
    6i8, 2i8, 3i8, 1i8, 2i8, 1i8, 1i8, 2i8, 2i8, 3i8,
];
#[no_mangle]
pub unsafe extern "C" fn cmd_parse_from_string(
    mut s: *const libc::c_char,
    mut pi: *mut cmd_parse_input,
) -> *mut cmd_parse_result {
    let mut input: cmd_parse_input = cmd_parse_input {
        flags: 0,
        file: 0 as *const libc::c_char,
        line: 0,
        item: 0 as *mut crate::cmd_queue::cmdq_item,
        c: 0 as *mut client,
        fs: cmd_find_state {
            flags: 0,
            current: 0 as *mut cmd_find_state,
            s: 0 as *mut session,
            wl: 0 as *mut winlink,
            w: 0 as *mut window,
            wp: 0 as *mut window_pane,
            idx: 0,
        },
    };
    if pi.is_null() {
        memset(
            &mut input as *mut cmd_parse_input as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<cmd_parse_input>() as libc::c_ulong,
        );
        pi = &mut input
    }
    /*
     * When parsing a string, put commands in one group even if there are
     * multiple lines. This means { a \n b } is identical to "a ; b" when
     * given as an argument to another command.
     */
    (*pi).flags |= 0x10i32;
    return cmd_parse_from_buffer(s as *const libc::c_void, strlen(s), pi);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_parse_and_insert(
    mut s: *const libc::c_char,
    mut pi: *mut cmd_parse_input,
    mut after: *mut crate::cmd_queue::cmdq_item,
    mut state: *mut crate::cmd_queue::cmdq_state,
    mut error: *mut *mut libc::c_char,
) -> cmd_parse_status {
    let mut pr: *mut cmd_parse_result = 0 as *mut cmd_parse_result;
    let mut item: *mut crate::cmd_queue::cmdq_item = 0 as *mut crate::cmd_queue::cmdq_item;
    /* Backward compatibility with an undocumented macro.
    Use YYerror or YYUNDEF. */
    pr = cmd_parse_from_string(s, pi);
    match (*pr).status {
        1 => {
            /* Enable debugging if requested.  */
            if !error.is_null() {
                *error = (*pr).error
            } else {
                free((*pr).error as *mut libc::c_void);
            }
        }
        2 => {
            item = cmdq_get_command((*pr).cmdlist, state);
            cmdq_insert_after(after, item);
            cmd_list_free((*pr).cmdlist);
        }
        0 | _ => {}
    }
    return (*pr).status;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_parse_and_append(
    mut s: *const libc::c_char,
    mut pi: *mut cmd_parse_input,
    mut c: *mut client,
    mut state: *mut crate::cmd_queue::cmdq_state,
    mut error: *mut *mut libc::c_char,
) -> cmd_parse_status {
    let mut pr: *mut cmd_parse_result = 0 as *mut cmd_parse_result;
    let mut item: *mut crate::cmd_queue::cmdq_item = 0 as *mut crate::cmd_queue::cmdq_item;
    pr = cmd_parse_from_string(s, pi);
    match (*pr).status {
        1 => {
            if !error.is_null() {
                *error = (*pr).error
            } else {
                free((*pr).error as *mut libc::c_void);
            }
        }
        2 => {
            item = cmdq_get_command((*pr).cmdlist, state);
            cmdq_append(c, item);
            cmd_list_free((*pr).cmdlist);
        }
        0 | _ => {}
    }
    return (*pr).status;
}
#[no_mangle]
pub unsafe extern "C" fn cmd_parse_from_buffer(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut pi: *mut cmd_parse_input,
) -> *mut cmd_parse_result {
    static mut pr: cmd_parse_result = cmd_parse_result {
        status: CMD_PARSE_EMPTY,
        cmdlist: 0 as *mut cmd_list,
        error: 0 as *mut libc::c_char,
    };
    let mut input: cmd_parse_input = cmd_parse_input {
        flags: 0,
        file: 0 as *const libc::c_char,
        line: 0,
        item: 0 as *mut crate::cmd_queue::cmdq_item,
        c: 0 as *mut client,
        fs: cmd_find_state {
            flags: 0,
            current: 0 as *mut cmd_find_state,
            s: 0 as *mut session,
            wl: 0 as *mut winlink,
            w: 0 as *mut window,
            wp: 0 as *mut window_pane,
            idx: 0,
        },
    };
    let mut cmds: *mut cmd_parse_commands = 0 as *mut cmd_parse_commands;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    if pi.is_null() {
        memset(
            &mut input as *mut cmd_parse_input as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<cmd_parse_input>() as libc::c_ulong,
        );
        pi = &mut input
    }
    memset(
        &mut pr as *mut cmd_parse_result as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<cmd_parse_result>() as libc::c_ulong,
    );
    if len == 0u64 {
        pr.status = CMD_PARSE_EMPTY;
        pr.cmdlist = 0 as *mut cmd_list;
        pr.error = 0 as *mut libc::c_char;
        return &mut pr;
    }
    cmds = cmd_parse_do_buffer(buf as *const libc::c_char, len, pi, &mut cause);
    if cmds.is_null() {
        pr.status = CMD_PARSE_ERROR;
        pr.error = cause;
        return &mut pr;
    }
    return cmd_parse_build_commands(cmds, pi);
}
#[no_mangle]
pub unsafe extern "C" fn cmd_parse_from_arguments(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut pi: *mut cmd_parse_input,
) -> *mut cmd_parse_result {
    let mut input: cmd_parse_input = cmd_parse_input {
        flags: 0,
        file: 0 as *const libc::c_char,
        line: 0,
        item: 0 as *mut crate::cmd_queue::cmdq_item,
        c: 0 as *mut client,
        fs: cmd_find_state {
            flags: 0,
            current: 0 as *mut cmd_find_state,
            s: 0 as *mut session,
            wl: 0 as *mut winlink,
            w: 0 as *mut window,
            wp: 0 as *mut window_pane,
            idx: 0,
        },
    };
    let mut cmds: *mut cmd_parse_commands = 0 as *mut cmd_parse_commands;
    let mut cmd: *mut cmd_parse_command = 0 as *mut cmd_parse_command;
    let mut copy: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut size: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut new_argc: libc::c_int = 0;
    /*
     * The commands are already split up into arguments, so just separate
     * into a set of commands by ';'.
     */
    if pi.is_null() {
        memset(
            &mut input as *mut cmd_parse_input as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<cmd_parse_input>() as libc::c_ulong,
        );
        pi = &mut input
    }
    cmd_log_argv(
        argc,
        argv,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"cmd_parse_from_arguments\x00"))
            .as_ptr(),
    );
    cmds = cmd_parse_new_commands();
    copy = cmd_copy_argv(argc, argv);
    last = 0i32;
    i = 0i32;
    while i < argc {
        size = strlen(*copy.offset(i as isize));
        if !(size == 0u64
            || *(*copy.offset(i as isize)).offset(size.wrapping_sub(1u64) as isize) as libc::c_int
                != ';' as i32)
        {
            size = size.wrapping_sub(1);
            *(*copy.offset(i as isize)).offset(size as isize) = '\u{0}' as libc::c_char;
            if size > 0u64
                && *(*copy.offset(i as isize)).offset(size.wrapping_sub(1u64) as isize)
                    as libc::c_int
                    == '\\' as i32
            {
                *(*copy.offset(i as isize)).offset(size.wrapping_sub(1u64) as isize) =
                    ';' as libc::c_char
            } else {
                new_argc = i - last;
                new_argv = copy.offset(last as isize);
                if size != 0u64 {
                    new_argc += 1
                }
                if new_argc != 0i32 {
                    cmd_log_argv(
                        new_argc,
                        new_argv,
                        b"%s: at %u\x00" as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                            b"cmd_parse_from_arguments\x00",
                        ))
                        .as_ptr(),
                        i,
                    );
                    cmd = xcalloc(
                        1u64,
                        ::std::mem::size_of::<cmd_parse_command>() as libc::c_ulong,
                    ) as *mut cmd_parse_command;
                    (*cmd).line = (*pi).line;
                    (*cmd).argc = new_argc;
                    (*cmd).argv = cmd_copy_argv(new_argc, new_argv);
                    (*cmd).entry.tqe_next = 0 as *mut cmd_parse_command;
                    (*cmd).entry.tqe_prev = (*cmds).tqh_last;
                    *(*cmds).tqh_last = cmd;
                    (*cmds).tqh_last = &mut (*cmd).entry.tqe_next
                }
                last = i + 1i32
            }
        }
        i += 1
    }
    if last != argc {
        new_argv = copy.offset(last as isize);
        new_argc = argc - last;
        if new_argc != 0i32 {
            cmd_log_argv(
                new_argc,
                new_argv,
                b"%s: at %u\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"cmd_parse_from_arguments\x00",
                ))
                .as_ptr(),
                last,
            );
            /* YYINITDEPTH -- initial size of the parser's stacks.  */
            cmd = xcalloc(
                1u64,
                ::std::mem::size_of::<cmd_parse_command>() as libc::c_ulong,
            ) as *mut cmd_parse_command;
            (*cmd).line = (*pi).line;
            (*cmd).argc = new_argc;
            (*cmd).argv = cmd_copy_argv(new_argc, new_argv);
            /* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
            if the built-in stack extension method is used).

            Do not make this value too large; the results are undefined if
            YYSTACK_ALLOC_MAXIMUM < YYSTACK_BYTES (YYMAXDEPTH)
            evaluated with infinite-precision integer arithmetic.  */
            (*cmd).entry.tqe_next = 0 as *mut cmd_parse_command;
            (*cmd).entry.tqe_prev = (*cmds).tqh_last;
            *(*cmds).tqh_last = cmd;
            (*cmds).tqh_last = &mut (*cmd).entry.tqe_next
        }
    }
    cmd_free_argv(argc, copy);
    return cmd_parse_build_commands(cmds, pi);
}
unsafe extern "C" fn yyerror(mut fmt: *const libc::c_char, mut args: ...) -> libc::c_int {
    let mut ps: *mut cmd_parse_state = &mut parse_state;
    let mut pi: *mut cmd_parse_input = (*ps).input;
    let mut ap: ::std::ffi::VaListImpl;
    let mut error: *mut libc::c_char = 0 as *mut libc::c_char;
    /*-----------------------------------------------.
    | Release the memory associated to this symbol.  |
    `-----------------------------------------------*/
    if !(*ps).error.is_null() {
        return 0i32;
    }
    ap = args.clone();
    xvasprintf(&mut error, fmt, ap.as_va_list());
    (*ps).error = cmd_parse_get_error((*pi).file, (*pi).line, error);
    free(error as *mut libc::c_void);
    return 0i32;
}
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut _yykind: yysymbol_kind_t,
    mut _yyvaluep: *mut YYSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\x00" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn yylex_is_var(mut ch: libc::c_char, mut first: libc::c_int) -> libc::c_int {
    if ch as libc::c_int == '=' as i32 {
        return 0i32;
    }
    if first != 0
        && *(*__ctype_b_loc()).offset(ch as u_char as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_ushort as libc::c_int
            != 0
    {
        /* Lookahead token kind.  */
        return 0i32;
    }
    return (*(*__ctype_b_loc()).offset(ch as u_char as libc::c_int as isize) as libc::c_int
        & _ISalnum as libc::c_ushort as libc::c_int
        != 0
        || ch as libc::c_int == '_' as i32) as libc::c_int;
}
#[no_mangle]
pub static mut yychar: libc::c_int = 0;
/* The semantic value of the lookahead symbol.  */
unsafe extern "C" fn yylex_append(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut add: *const libc::c_char,
    mut addlen: size_t,
) {
    if addlen > (18446744073709551615u64).wrapping_sub(1u64)
        || *len
            > (18446744073709551615u64)
                .wrapping_sub(1u64)
                .wrapping_sub(addlen)
    {
        fatalx(b"buffer is too big\x00" as *const u8 as *const libc::c_char);
    }
    *buf = xrealloc(
        *buf as *mut libc::c_void,
        (*len).wrapping_add(1u64).wrapping_add(addlen),
    ) as *mut libc::c_char;
    memcpy(
        (*buf).offset(*len as isize) as *mut libc::c_void,
        add as *const libc::c_void,
        addlen,
    );
    /*----------.
    | yyparse.  |
    `----------*/
    *len = (*len).wrapping_add(addlen);
}
#[no_mangle]
pub static mut yylval: YYSTYPE = YYSTYPE {
    token: 0 as *mut libc::c_char,
};
#[no_mangle]
pub static mut yynerrs: libc::c_int = 0;
unsafe extern "C" fn yylex_append1(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut add: libc::c_char,
) {
    yylex_append(buf, len, &mut add, 1u64);
}
unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0i32;
    /* Number of tokens to shift before error messages enabled.  */
    let mut yyerrstatus: libc::c_int = 0i32;
    /* Refer to the stacks through separate pointers, to allow yyoverflow
    to reallocate them elsewhere.  */
    /* Their size.  */
    let mut yystacksize: libc::c_long = 200i64;
    /* The state stack: array, bottom, top.  */
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = yyssa.as_mut_ptr();
    let mut yyssp: *mut yy_state_t = yyss;
    /* The semantic value stack: array, bottom, top.  */
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE {
        token: 0 as *mut libc::c_char,
    }; 200];
    let mut yyvs: *mut YYSTYPE = yyvsa.as_mut_ptr();
    let mut yyvsp: *mut YYSTYPE = yyvs;
    let mut yyn: libc::c_int = 0;
    /* The return value of yyparse.  */
    let mut yyresult: libc::c_int = 0;
    /* Lookahead symbol kind.  */
    let mut yytoken: yysymbol_kind_t = YYSYMBOL_YYEMPTY;
    /* The variables used to return semantic value and location from the
    action routines.  */
    let mut yyval: YYSTYPE = YYSTYPE {
        token: 0 as *mut libc::c_char,
    };
    /* The number of symbols on the RHS of the reduced rule.
    Keep to zero when no symbol should be popped.  */
    let mut yylen: libc::c_int = 0i32; /* Cause a token to be read.  */
    yychar = YYEMPTY;
    's_64: loop
    /*--------------------------------------------------------------------.
    | yysetstate -- set current state (the top of the stack) to yystate.  |
    `--------------------------------------------------------------------*/
    {
        (0i32 != 0 && (0i32 <= yystate && yystate < 75i32)) as libc::c_int;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1isize)) <= yyssp {
            /* Get the current used size of the three stacks, in elements.  */
            let mut yysize: libc::c_long = yyssp.wrapping_offset_from(yyss) as libc::c_long + 1i64;
            /* defined YYSTACK_RELOCATE */
            /* Extend the stack our own way.  */
            if 10000i64 <= yystacksize {
                current_block = 14688867023895834084;
                break;
            }
            yystacksize *= 2i64;
            if (10000i64) < yystacksize {
                yystacksize = 10000i64
            }
            /*
             * Ensure every file or string is terminated by a
             * newline. This keeps the parser simpler and avoids
             * having to add a newline to each string.
             */
            let mut yyss1: *mut yy_state_t = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                (yystacksize
                    * (::std::mem::size_of::<yy_state_t>() as libc::c_long
                        + ::std::mem::size_of::<YYSTYPE>() as libc::c_long)
                    + (::std::mem::size_of::<yyalloc>() as libc::c_long - 1i64))
                    as libc::c_ulong,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 14688867023895834084;
                break;
            }
            let mut yynewbytes: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yy_state_t as *mut libc::c_void,
                yyss as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<yy_state_t>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize * ::std::mem::size_of::<yy_state_t>() as libc::c_long
                + (::std::mem::size_of::<yyalloc>() as libc::c_long - 1i64);
            yyptr = yyptr
                .offset((yynewbytes / ::std::mem::size_of::<yyalloc>() as libc::c_long) as isize);
            let mut yynewbytes_0: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut YYSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize * ::std::mem::size_of::<YYSTYPE>() as libc::c_long
                + (::std::mem::size_of::<yyalloc>() as libc::c_long - 1i64);
            yyptr = yyptr
                .offset((yynewbytes_0 / ::std::mem::size_of::<yyalloc>() as libc::c_long) as isize);
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1isize));
            /*
             * Ignore whitespace.
             */
            if yyss.offset(yystacksize as isize).offset(-(1isize)) <= yyssp
            /*
             * End of line. Update the line number.
             */
            {
                current_block = 10035012600013456863;
                break;
            }
        }
        /* !defined yyoverflow && !defined YYSTACK_RELOCATE */
        /*
         * A semicolon or { or } is itself.
         */
        if yystate == 20i32 {
            /*-------------------------------------.
            | yyacceptlab -- YYACCEPT comes here.  |
            `-------------------------------------*/
            yyresult = 0i32;
            current_block = 3567665828411497206;
            break;
        } else {
            /*-----------.
            | yybackup.  |
            `-----------*/
            /*
             * #{ after a condition opens a format; anything else
             * is a comment, ignore up to the end of the line.
             */
            /* Do appropriate processing given the current state.  Read a
            lookahead token if we need one and don't already have one.  */
            /* First try to decide what to do without reference to lookahead token.  */
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(32i32) {
                current_block = 8820106547087071178;
            } else {
                /* Not known => get a lookahead token if don't already have one.  */
                /* YYCHAR is either empty, or end-of-input, or a valid lookahead.  */
                if yychar == YYEMPTY {
                    yychar = yylex()
                }
                if yychar <= 0i32 {
                    yychar = 0i32;
                    yytoken = YYSYMBOL_YYEOF;
                    current_block = 17728966195399430138;
                } else if yychar == 256i32 {
                    /* The scanner already issued an error message, process directly
                    to error recovery.  But do not keep the error token as
                    lookahead, it is too special and may lead us to an endless
                    loop in error recovery. */
                    yychar = 257i32;
                    yytoken = YYSYMBOL_YYerror;
                    current_block = 6983641760769012672;
                } else {
                    yytoken = if 0i32 <= yychar && yychar <= 266i32 {
                        yytranslate[yychar as usize] as libc::c_int
                    } else {
                        YYSYMBOL_YYUNDEF
                    };
                    current_block = 17728966195399430138;
                }
                match current_block {
                    6983641760769012672 => {}
                    _ => {
                        /* If the proper action on seeing token YYTOKEN is to reduce or to
                        detect an error, take that action.  */
                        yyn += yytoken;
                        if yyn < 0i32
                            || (104i32) < yyn
                            || yycheck[yyn as usize] as libc::c_int != yytoken
                        {
                            current_block = 8820106547087071178;
                        } else {
                            yyn = yytable[yyn as usize] as libc::c_int;
                            if yyn <= 0i32 {
                                yyn = -yyn;
                                current_block = 16998598652869609653;
                            } else {
                                /*
                                 * Otherwise this is a token.
                                 */
                                /* Count tokens shifted since error; after three, turn off error
                                status.  */
                                if yyerrstatus != 0 {
                                    yyerrstatus -= 1
                                }
                                /* Shift the lookahead token.  */
                                yystate = yyn;
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                /* Discard the shifted token.  */
                                yychar = YYEMPTY;
                                current_block = 4382416473683093495;
                            }
                        }
                    }
                }
            }
            match current_block {
                8820106547087071178 =>
                /*-----------------------------------------------------------.
                | yydefault -- do the default action for the current state.  |
                `-----------------------------------------------------------*/
                {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0i32 {
                        /*--------------------------------------.
                        | yyerrlab -- here on detecting error.  |
                        `--------------------------------------*/
                        /* Make sure we have latest lookahead translation.  See comments at
                        user semantic actions for why this is necessary.  */
                        yytoken = if yychar == YYEMPTY {
                            YYSYMBOL_YYEMPTY
                        } else if 0i32 <= yychar && yychar <= 266i32 {
                            yytranslate[yychar as usize] as libc::c_int
                        } else {
                            YYSYMBOL_YYUNDEF
                        };
                        /* If not already recovering from an error, report this error.  */
                        if yyerrstatus == 0 {
                            yynerrs += 1;
                            yyerror(b"syntax error\x00" as *const u8 as *const libc::c_char);
                        }
                        if yyerrstatus == 3i32 {
                            /* If just tried and failed to reuse lookahead token after an
                            error, discard it.  */
                            if yychar <= 0i32 {
                                /* Return failure if at end of input.  */
                                if yychar == 0i32 {
                                    current_block = 10035012600013456863;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\x00" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut yylval,
                                );
                                yychar = YYEMPTY
                            }
                        }
                        /* Else will try to reuse lookahead token after shifting the error
                        token.  */
                        current_block = 6983641760769012672;
                    } else {
                        current_block = 16998598652869609653;
                    }
                }
                _ => {}
            }
            match current_block {
                6983641760769012672 =>
                /*-------------------------------------------------------------.
                | yyerrlab1 -- common code for both syntax error and YYERROR.  |
                `-------------------------------------------------------------*/
                {
                    yyerrstatus = 3i32; /* Each real token shifted decrements this.  */
                    loop
                    /* Pop stack until we find a state that shifts the error token.  */
                    {
                        yyn = yypact[yystate as usize] as libc::c_int;
                        if !(yyn == -(32i32)) {
                            yyn += YYSYMBOL_YYerror;
                            if 0i32 <= yyn
                                && yyn <= 104i32
                                && yycheck[yyn as usize] as libc::c_int == YYSYMBOL_YYerror
                            {
                                yyn = yytable[yyn as usize] as libc::c_int;
                                if (0i32) < yyn {
                                    break;
                                }
                            }
                        }
                        /* Pop the current state because it cannot handle the error token.  */
                        if yyssp == yyss {
                            current_block = 10035012600013456863;
                            break 's_64;
                        }
                        yydestruct(
                            b"Error: popping\x00" as *const u8 as *const libc::c_char,
                            yystos[yystate as usize] as yysymbol_kind_t,
                            yyvsp,
                        );
                        yyvsp = yyvsp.offset(-(1isize));
                        yyssp = yyssp.offset(-(1isize));
                        yystate = *yyssp as yy_state_fast_t
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    /* Shift the error token.  */
                    yystate = yyn
                }
                16998598652869609653 =>
                /*-----------------------------.
                | yyreduce -- do a reduction.  |
                `-----------------------------*/
                                    /* yyn is the number of a rule to reduce with.  */
                {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    /* If YYLEN is nonzero, implement the default value of the action:
                    '$$ = $1'.

                    Otherwise, the following line sets YYVAL to garbage.
                    This behavior is undocumented and Bison
                    users should not rely upon it.  Assigning to YYVAL
                    unconditionally makes the parser a bit smaller, and it avoids a
                    GCC warning that YYVAL may be used uninitialized.  */
                    yyval = *yyvsp.offset((1i32 - yylen) as isize);
                    match yyn {
                        3 => {
                            let mut ps: *mut cmd_parse_state = &mut parse_state;
                            (*ps).commands = (*yyvsp.offset(0isize)).commands
                        }
                        4 => yyval.commands = (*yyvsp.offset(-1isize)).commands,
                        5 => {
                            yyval.commands = (*yyvsp.offset(-2isize)).commands;
                            if !(*(*yyvsp.offset(-1isize)).commands).tqh_first.is_null() {
                                *(*yyval.commands).tqh_last =
                                    (*(*yyvsp.offset(-1isize)).commands).tqh_first;
                                let ref mut fresh0 = (*(*(*yyvsp.offset(-1isize)).commands)
                                    .tqh_first)
                                    .entry
                                    .tqe_prev;
                                *fresh0 = (*yyval.commands).tqh_last;
                                (*yyval.commands).tqh_last =
                                    (*(*yyvsp.offset(-1isize)).commands).tqh_last;
                                let ref mut fresh1 = (*(*yyvsp.offset(-1isize)).commands).tqh_first;
                                *fresh1 = 0 as *mut cmd_parse_command;
                                let ref mut fresh2 = (*(*yyvsp.offset(-1isize)).commands).tqh_last;
                                *fresh2 = &mut (*(*yyvsp.offset(-1isize)).commands).tqh_first
                            }
                            free((*yyvsp.offset(-1isize)).commands as *mut libc::c_void);
                        }
                        6 => {
                            yyval.commands = xmalloc(
                                ::std::mem::size_of::<cmd_parse_commands>() as libc::c_ulong
                            )
                                as *mut cmd_parse_commands;
                            (*yyval.commands).tqh_first = 0 as *mut cmd_parse_command;
                            (*yyval.commands).tqh_last = &mut (*yyval.commands).tqh_first
                        }
                        7 => {
                            yyval.commands = xmalloc(
                                ::std::mem::size_of::<cmd_parse_commands>() as libc::c_ulong
                            )
                                as *mut cmd_parse_commands;
                            (*yyval.commands).tqh_first = 0 as *mut cmd_parse_command;
                            (*yyval.commands).tqh_last = &mut (*yyval.commands).tqh_first
                        }
                        8 => {
                            let mut ps_0: *mut cmd_parse_state = &mut parse_state;
                            if (*ps_0).scope.is_null() || (*(*ps_0).scope).flag != 0 {
                                yyval.commands = (*yyvsp.offset(0isize)).commands
                            } else {
                                yyval.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(0isize)).commands);
                            }
                        }
                        9 => {
                            let mut ps_1: *mut cmd_parse_state = &mut parse_state;
                            if (*ps_1).scope.is_null() || (*(*ps_1).scope).flag != 0 {
                                yyval.commands = (*yyvsp.offset(0isize)).commands
                            } else {
                                yyval.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(0isize)).commands);
                            }
                        }
                        10 => yyval.token = (*yyvsp.offset(0isize)).token,
                        11 => yyval.token = (*yyvsp.offset(0isize)).token,
                        12 => {
                            let mut ps_2: *mut cmd_parse_state = &mut parse_state;
                            let mut pi: *mut cmd_parse_input = (*ps_2).input;
                            let mut ft: *mut crate::format::format_tree =
                                0 as *mut crate::format::format_tree;
                            let mut c: *mut client = (*pi).c;
                            let mut fsp: *mut cmd_find_state = 0 as *mut cmd_find_state;
                            let mut fs: cmd_find_state = cmd_find_state {
                                flags: 0,
                                current: 0 as *mut cmd_find_state,
                                s: 0 as *mut session,
                                wl: 0 as *mut winlink,
                                w: 0 as *mut window,
                                wp: 0 as *mut window_pane,
                                idx: 0,
                            };
                            let mut flags: libc::c_int = 0x4i32;
                            if cmd_find_valid_state(&mut (*pi).fs) != 0 {
                                fsp = &mut (*pi).fs
                            } else {
                                cmd_find_from_client(&mut fs, c, 0i32);
                                fsp = &mut fs
                            }
                            ft = format_create(0 as *mut client, (*pi).item, 0i32, flags);
                            format_defaults(ft, c, (*fsp).s, (*fsp).wl, (*fsp).wp);
                            yyval.token = format_expand(ft, (*yyvsp.offset(0isize)).token);
                            format_free(ft);
                            free((*yyvsp.offset(0isize)).token as *mut libc::c_void);
                        }
                        15 => {
                            let mut ps_3: *mut cmd_parse_state = &mut parse_state;
                            let mut flags_0: libc::c_int = (*(*ps_3).input).flags;
                            if !flags_0 & 0x2i32 != 0
                                && ((*ps_3).scope.is_null() || (*(*ps_3).scope).flag != 0)
                            {
                                environ_put(global_environ, (*yyvsp.offset(0isize)).token, 0i32);
                            }
                            free((*yyvsp.offset(0isize)).token as *mut libc::c_void);
                        }
                        16 => {
                            let mut ps_4: *mut cmd_parse_state = &mut parse_state;
                            let mut flags_1: libc::c_int = (*(*ps_4).input).flags;
                            if !flags_1 & 0x2i32 != 0
                                && ((*ps_4).scope.is_null() || (*(*ps_4).scope).flag != 0)
                            {
                                environ_put(global_environ, (*yyvsp.offset(0isize)).token, 0x1i32);
                            }
                            free((*yyvsp.offset(0isize)).token as *mut libc::c_void);
                        }
                        17 => {
                            let mut ps_5: *mut cmd_parse_state = &mut parse_state;
                            let mut scope: *mut cmd_parse_scope = 0 as *mut cmd_parse_scope;
                            scope =
                                xmalloc(::std::mem::size_of::<cmd_parse_scope>() as libc::c_ulong)
                                    as *mut cmd_parse_scope;
                            (*scope).flag = format_true((*yyvsp.offset(0isize)).token);
                            yyval.flag = (*scope).flag;
                            free((*yyvsp.offset(0isize)).token as *mut libc::c_void);
                            if !(*ps_5).scope.is_null() {
                                (*(*ps_5).scope).entry.tqe_next = (*ps_5).stack.tqh_first;
                                if !(*(*ps_5).scope).entry.tqe_next.is_null() {
                                    (*(*ps_5).stack.tqh_first).entry.tqe_prev =
                                        &mut (*(*ps_5).scope).entry.tqe_next
                                } else {
                                    (*ps_5).stack.tqh_last = &mut (*(*ps_5).scope).entry.tqe_next
                                }
                                (*ps_5).stack.tqh_first = (*ps_5).scope;
                                (*(*ps_5).scope).entry.tqe_prev = &mut (*ps_5).stack.tqh_first
                            }
                            (*ps_5).scope = scope
                        }
                        18 => {
                            let mut ps_6: *mut cmd_parse_state = &mut parse_state;
                            let mut scope_0: *mut cmd_parse_scope = 0 as *mut cmd_parse_scope;
                            scope_0 =
                                xmalloc(::std::mem::size_of::<cmd_parse_scope>() as libc::c_ulong)
                                    as *mut cmd_parse_scope;
                            (*scope_0).flag = ((*(*ps_6).scope).flag == 0) as libc::c_int;
                            free((*ps_6).scope as *mut libc::c_void);
                            (*ps_6).scope = scope_0
                        }
                        19 => {
                            let mut ps_7: *mut cmd_parse_state = &mut parse_state;
                            let mut scope_1: *mut cmd_parse_scope = 0 as *mut cmd_parse_scope;
                            scope_1 =
                                xmalloc(::std::mem::size_of::<cmd_parse_scope>() as libc::c_ulong)
                                    as *mut cmd_parse_scope;
                            (*scope_1).flag = format_true((*yyvsp.offset(0isize)).token);
                            yyval.flag = (*scope_1).flag;
                            free((*yyvsp.offset(0isize)).token as *mut libc::c_void);
                            free((*ps_7).scope as *mut libc::c_void);
                            (*ps_7).scope = scope_1
                        }
                        20 => {
                            let mut ps_8: *mut cmd_parse_state = &mut parse_state;
                            free((*ps_8).scope as *mut libc::c_void);
                            (*ps_8).scope = (*ps_8).stack.tqh_first;
                            if !(*ps_8).scope.is_null() {
                                if !(*(*ps_8).scope).entry.tqe_next.is_null() {
                                    (*(*(*ps_8).scope).entry.tqe_next).entry.tqe_prev =
                                        (*(*ps_8).scope).entry.tqe_prev
                                } else {
                                    (*ps_8).stack.tqh_last = (*(*ps_8).scope).entry.tqe_prev
                                }
                                *(*(*ps_8).scope).entry.tqe_prev = (*(*ps_8).scope).entry.tqe_next
                            }
                        }
                        21 => {
                            if (*yyvsp.offset(-3isize)).flag != 0 {
                                yyval.commands = (*yyvsp.offset(-1isize)).commands
                            } else {
                                yyval.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                            }
                        }
                        22 => {
                            /* EOF or \n are always the end of the token. */
                            if (*yyvsp.offset(-6isize)).flag != 0 {
                                yyval.commands = (*yyvsp.offset(-4isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                            } else {
                                yyval.commands = (*yyvsp.offset(-1isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(-4isize)).commands);
                            }
                        }
                        23 => {
                            if (*yyvsp.offset(-4isize)).flag != 0 {
                                yyval.commands = (*yyvsp.offset(-2isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).elif.commands);
                            } else if (*yyvsp.offset(-1isize)).elif.flag != 0 {
                                yyval.commands = (*yyvsp.offset(-1isize)).elif.commands;
                                cmd_parse_free_commands((*yyvsp.offset(-2isize)).commands);
                            } else {
                                yyval.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(-2isize)).commands);
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).elif.commands);
                            }
                        }
                        24 => {
                            if (*yyvsp.offset(-7isize)).flag != 0 {
                                yyval.commands = (*yyvsp.offset(-5isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(-4isize)).elif.commands);
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                            } else if (*yyvsp.offset(-4isize)).elif.flag != 0 {
                                yyval.commands = (*yyvsp.offset(-4isize)).elif.commands;
                                cmd_parse_free_commands((*yyvsp.offset(-5isize)).commands);
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                            } else {
                                yyval.commands = (*yyvsp.offset(-1isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(-5isize)).commands);
                                cmd_parse_free_commands((*yyvsp.offset(-4isize)).elif.commands);
                            }
                        }
                        25 => {
                            if (*yyvsp.offset(-2isize)).flag != 0 {
                                yyval.elif.flag = 1i32;
                                yyval.elif.commands = (*yyvsp.offset(0isize)).commands
                            } else {
                                yyval.elif.flag = 0i32;
                                yyval.elif.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(0isize)).commands);
                            }
                        }
                        26 => {
                            /* Otherwise add the character to the buffer. */
                            if (*yyvsp.offset(-3isize)).flag != 0 {
                                yyval.elif.flag = 1i32;
                                yyval.elif.commands = (*yyvsp.offset(-1isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(0isize)).elif.commands);
                            } else if (*yyvsp.offset(0isize)).elif.flag != 0 {
                                yyval.elif.flag = 1i32;
                                yyval.elif.commands = (*yyvsp.offset(0isize)).elif.commands;
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                            } else {
                                yyval.elif.flag = 0i32;
                                yyval.elif.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                                cmd_parse_free_commands((*yyvsp.offset(0isize)).elif.commands);
                            }
                        }
                        27 => {
                            let mut ps_9: *mut cmd_parse_state = &mut parse_state;
                            yyval.commands = cmd_parse_new_commands();
                            if (*(*yyvsp.offset(0isize)).command).argc != 0i32
                                && ((*ps_9).scope.is_null() || (*(*ps_9).scope).flag != 0)
                            {
                                let ref mut fresh3 =
                                    (*(*yyvsp.offset(0isize)).command).entry.tqe_next;
                                *fresh3 = 0 as *mut cmd_parse_command;
                                let ref mut fresh4 =
                                    (*(*yyvsp.offset(0isize)).command).entry.tqe_prev;
                                *fresh4 = (*yyval.commands).tqh_last;
                                *(*yyval.commands).tqh_last = (*yyvsp.offset(0isize)).command;
                                (*yyval.commands).tqh_last =
                                    &mut (*(*yyvsp.offset(0isize)).command).entry.tqe_next
                            } else {
                                cmd_parse_free_command((*yyvsp.offset(0isize)).command);
                            }
                        }
                        28 => yyval.commands = (*yyvsp.offset(-1isize)).commands,
                        29 => {
                            yyval.commands = (*yyvsp.offset(-2isize)).commands;
                            if !(*(*yyvsp.offset(0isize)).commands).tqh_first.is_null() {
                                *(*yyval.commands).tqh_last =
                                    (*(*yyvsp.offset(0isize)).commands).tqh_first;
                                let ref mut fresh5 = (*(*(*yyvsp.offset(0isize)).commands)
                                    .tqh_first)
                                    .entry
                                    .tqe_prev;
                                *fresh5 = (*yyval.commands).tqh_last;
                                (*yyval.commands).tqh_last =
                                    (*(*yyvsp.offset(0isize)).commands).tqh_last;
                                let ref mut fresh6 = (*(*yyvsp.offset(0isize)).commands).tqh_first;
                                *fresh6 = 0 as *mut cmd_parse_command;
                                let ref mut fresh7 = (*(*yyvsp.offset(0isize)).commands).tqh_last;
                                *fresh7 = &mut (*(*yyvsp.offset(0isize)).commands).tqh_first
                            }
                            free((*yyvsp.offset(0isize)).commands as *mut libc::c_void);
                        }
                        30 => {
                            let mut ps_10: *mut cmd_parse_state = &mut parse_state;
                            if (*(*yyvsp.offset(0isize)).command).argc != 0i32
                                && ((*ps_10).scope.is_null() || (*(*ps_10).scope).flag != 0)
                            {
                                yyval.commands = (*yyvsp.offset(-2isize)).commands;
                                let ref mut fresh8 =
                                    (*(*yyvsp.offset(0isize)).command).entry.tqe_next;
                                *fresh8 = 0 as *mut cmd_parse_command;
                                let ref mut fresh9 =
                                    (*(*yyvsp.offset(0isize)).command).entry.tqe_prev;
                                *fresh9 = (*yyval.commands).tqh_last;
                                *(*yyval.commands).tqh_last = (*yyvsp.offset(0isize)).command;
                                (*yyval.commands).tqh_last =
                                    &mut (*(*yyvsp.offset(0isize)).command).entry.tqe_next
                            } else {
                                yyval.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(-2isize)).commands);
                                cmd_parse_free_command((*yyvsp.offset(0isize)).command);
                            }
                        }
                        31 => yyval.commands = (*yyvsp.offset(0isize)).commands,
                        32 => {
                            let mut ps_11: *mut cmd_parse_state = &mut parse_state;
                            yyval.command = xcalloc(
                                1u64,
                                ::std::mem::size_of::<cmd_parse_command>() as libc::c_ulong,
                            ) as *mut cmd_parse_command;
                            (*yyval.command).line = (*(*ps_11).input).line
                        }
                        33 => {
                            let mut ps_12: *mut cmd_parse_state = &mut parse_state;
                            yyval.command = xcalloc(
                                1u64,
                                ::std::mem::size_of::<cmd_parse_command>() as libc::c_ulong,
                            ) as *mut cmd_parse_command;
                            (*yyval.command).line = (*(*ps_12).input).line;
                            cmd_prepend_argv(
                                &mut (*yyval.command).argc,
                                &mut (*yyval.command).argv,
                                (*yyvsp.offset(0isize)).token,
                            );
                        }
                        34 => {
                            let mut ps_13: *mut cmd_parse_state = &mut parse_state;
                            yyval.command = xcalloc(
                                1u64,
                                ::std::mem::size_of::<cmd_parse_command>() as libc::c_ulong,
                            ) as *mut cmd_parse_command;
                            (*yyval.command).line = (*(*ps_13).input).line;
                            (*yyval.command).argc = (*yyvsp.offset(0isize)).arguments.argc;
                            (*yyval.command).argv = (*yyvsp.offset(0isize)).arguments.argv;
                            cmd_prepend_argv(
                                &mut (*yyval.command).argc,
                                &mut (*yyval.command).argv,
                                (*yyvsp.offset(-1isize)).token,
                            );
                        }
                        35 => {
                            if (*yyvsp.offset(-2isize)).flag != 0 {
                                yyval.commands = (*yyvsp.offset(-1isize)).commands
                            } else {
                                yyval.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                            }
                        }
                        36 => {
                            if (*yyvsp.offset(-4isize)).flag != 0 {
                                yyval.commands = (*yyvsp.offset(-3isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                            } else {
                                yyval.commands = (*yyvsp.offset(-1isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(-3isize)).commands);
                            }
                        }
                        37 => {
                            if (*yyvsp.offset(-3isize)).flag != 0 {
                                yyval.commands = (*yyvsp.offset(-2isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).elif.commands);
                            } else if (*yyvsp.offset(-1isize)).elif.flag != 0 {
                                yyval.commands = (*yyvsp.offset(-1isize)).elif.commands;
                                cmd_parse_free_commands((*yyvsp.offset(-2isize)).commands);
                            } else {
                                yyval.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(-2isize)).commands);
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).elif.commands);
                            }
                        }
                        38 => {
                            if (*yyvsp.offset(-5isize)).flag != 0 {
                                yyval.commands = (*yyvsp.offset(-4isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(-3isize)).elif.commands);
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                            } else if (*yyvsp.offset(-3isize)).elif.flag != 0 {
                                yyval.commands = (*yyvsp.offset(-3isize)).elif.commands;
                                cmd_parse_free_commands((*yyvsp.offset(-4isize)).commands);
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                            } else {
                                yyval.commands = (*yyvsp.offset(-1isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(-4isize)).commands);
                                cmd_parse_free_commands((*yyvsp.offset(-3isize)).elif.commands);
                            }
                        }
                        39 => {
                            if (*yyvsp.offset(-1isize)).flag != 0 {
                                yyval.elif.flag = 1i32;
                                yyval.elif.commands = (*yyvsp.offset(0isize)).commands
                            } else {
                                yyval.elif.flag = 0i32;
                                yyval.elif.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(0isize)).commands);
                            }
                        }
                        40 => {
                            if (*yyvsp.offset(-2isize)).flag != 0 {
                                yyval.elif.flag = 1i32;
                                yyval.elif.commands = (*yyvsp.offset(-1isize)).commands;
                                cmd_parse_free_commands((*yyvsp.offset(0isize)).elif.commands);
                            } else if (*yyvsp.offset(0isize)).elif.flag != 0 {
                                yyval.elif.flag = 1i32;
                                yyval.elif.commands = (*yyvsp.offset(0isize)).elif.commands;
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                            } else {
                                yyval.elif.flag = 0i32;
                                yyval.elif.commands = cmd_parse_new_commands();
                                cmd_parse_free_commands((*yyvsp.offset(-1isize)).commands);
                                cmd_parse_free_commands((*yyvsp.offset(0isize)).elif.commands);
                            }
                        }
                        41 => {
                            yyval.arguments.argc = 1i32;
                            yyval.arguments.argv = xreallocarray(
                                0 as *mut libc::c_void,
                                1u64,
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            )
                                as *mut *mut libc::c_char;
                            let ref mut fresh10 = *yyval.arguments.argv.offset(0isize);
                            *fresh10 = (*yyvsp.offset(0isize)).token
                        }
                        42 => {
                            cmd_prepend_argv(
                                &mut (*yyvsp.offset(0isize)).arguments.argc,
                                &mut (*yyvsp.offset(0isize)).arguments.argv,
                                (*yyvsp.offset(-1isize)).token,
                            );
                            free((*yyvsp.offset(-1isize)).token as *mut libc::c_void);
                            yyval.arguments = (*yyvsp.offset(0isize)).arguments
                        }
                        43 => yyval.token = (*yyvsp.offset(0isize)).token,
                        44 => yyval.token = (*yyvsp.offset(0isize)).token,
                        45 => {
                            yyval.token =
                                cmd_parse_commands_to_string((*yyvsp.offset(0isize)).commands);
                            cmd_parse_free_commands((*yyvsp.offset(0isize)).commands);
                        }
                        46 => yyval.commands = (*yyvsp.offset(-1isize)).commands,
                        47 => {
                            yyval.commands = (*yyvsp.offset(-2isize)).commands;
                            if !(*(*yyvsp.offset(-1isize)).commands).tqh_first.is_null() {
                                *(*yyval.commands).tqh_last =
                                    (*(*yyvsp.offset(-1isize)).commands).tqh_first;
                                let ref mut fresh11 = (*(*(*yyvsp.offset(-1isize)).commands)
                                    .tqh_first)
                                    .entry
                                    .tqe_prev;
                                *fresh11 = (*yyval.commands).tqh_last;
                                (*yyval.commands).tqh_last =
                                    (*(*yyvsp.offset(-1isize)).commands).tqh_last;
                                let ref mut fresh12 =
                                    (*(*yyvsp.offset(-1isize)).commands).tqh_first;
                                *fresh12 = 0 as *mut cmd_parse_command;
                                let ref mut fresh13 = (*(*yyvsp.offset(-1isize)).commands).tqh_last;
                                *fresh13 = &mut (*(*yyvsp.offset(-1isize)).commands).tqh_first
                            }
                            free((*yyvsp.offset(-1isize)).commands as *mut libc::c_void);
                        }
                        _ => {}
                    }
                    /* User semantic actions sometimes alter yychar, and that requires
                    that yytoken be updated with the new translation.  We take the
                    approach of translating immediately before every use of yytoken.
                    One alternative is translating here after every semantic action,
                    but that translation would be missed if the semantic action invokes
                    YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
                    if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
                    incorrect destructor might then be invoked immediately.  In the
                    case of YYERROR or YYBACKUP, subsequent parser actions might lead
                    to an incorrect destructor call or verbose syntax error message
                    before the lookahead is translated.  */
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0i32;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    /* Now 'shift' the result of the reduction.  Determine what state
                    that goes to, based on the state we popped back to and the rule
                    number reduced by.  */
                    let yylhs: libc::c_int = yyr1[yyn as usize] as libc::c_int - 16i32;
                    let yyi: libc::c_int =
                        yypgoto[yylhs as usize] as libc::c_int + *yyssp as libc::c_int;
                    yystate = if 0i32 <= yyi
                        && yyi <= 104i32
                        && yycheck[yyi as usize] as libc::c_int == *yyssp as libc::c_int
                    {
                        yytable[yyi as usize] as libc::c_int
                    } else {
                        yydefgoto[yylhs as usize] as libc::c_int
                    }
                }
                _ => {}
            }
            /*------------------------------------------------------------.
            | yynewstate -- push a new state, which is found in yystate.  |
            `------------------------------------------------------------*/
            /* In all cases, when you get here, the value and location stacks
            have just been pushed.  So pushing a state here evens the stacks.  */
            yyssp = yyssp.offset(1)
        }
    }
    match current_block {
        14688867023895834084 =>
        /*-------------------------------------------------.
        | yyexhaustedlab -- memory exhaustion comes here.  |
        `-------------------------------------------------*/
        {
            yyerror(b"memory exhausted\x00" as *const u8 as *const libc::c_char);
            yyresult = 2i32
        }
        10035012600013456863 =>
        /*-----------------------------------.
        | yyabortlab -- YYABORT comes here.  |
        `-----------------------------------*/
        {
            yyresult = 1i32
        }
        _ => {}
    }
    /*-------------------------------------------------------.
    | yyreturn -- parsing is finished, clean up and return.  |
    `-------------------------------------------------------*/
    if yychar != YYEMPTY {
        /* Make sure we have latest lookahead translation.  See comments at
        user semantic actions for why this is necessary.  */
        yytoken = if 0i32 <= yychar && yychar <= 266i32 {
            yytranslate[yychar as usize] as libc::c_int
        } else {
            YYSYMBOL_YYUNDEF
        };
        yydestruct(
            b"Cleanup: discarding lookahead\x00" as *const u8 as *const libc::c_char,
            yytoken,
            &mut yylval,
        );
    }
    /* Do not reclaim the symbols of the rule whose action triggered
    this YYABORT or YYACCEPT.  */
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\x00" as *const u8 as *const libc::c_char,
            yystos[*yyssp as libc::c_int as usize] as yysymbol_kind_t,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1isize));
        yyssp = yyssp.offset(-(1isize))
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
unsafe extern "C" fn yylex_getc1() -> libc::c_int {
    let mut ps: *mut cmd_parse_state = &mut parse_state;
    let mut ch: libc::c_int = 0;
    if !(*ps).f.is_null() {
        ch = getc((*ps).f)
    } else if (*ps).off == (*ps).len {
        ch = -(1i32)
    } else {
        let fresh14 = (*ps).off;
        (*ps).off = (*ps).off.wrapping_add(1);
        ch = *(*ps).buf.offset(fresh14 as isize) as libc::c_int
    }
    return ch;
}
unsafe extern "C" fn yylex_ungetc(mut ch: libc::c_int) {
    let mut ps: *mut cmd_parse_state = &mut parse_state;
    if !(*ps).f.is_null() {
        ungetc(ch, (*ps).f);
    } else if (*ps).off > 0u64 && ch != -(1i32) {
        (*ps).off = (*ps).off.wrapping_sub(1)
    };
}
unsafe extern "C" fn yylex_getc() -> libc::c_int {
    let mut ps: *mut cmd_parse_state = &mut parse_state;
    let mut ch: libc::c_int = 0;
    if (*ps).escapes != 0u32 {
        (*ps).escapes = (*ps).escapes.wrapping_sub(1);
        return '\\' as i32;
    }
    loop {
        ch = yylex_getc1();
        if ch == '\\' as i32 {
            (*ps).escapes = (*ps).escapes.wrapping_add(1)
        } else if ch == '\n' as i32 && (*ps).escapes.wrapping_rem(2u32) == 1u32 {
            (*(*ps).input).line = (*(*ps).input).line.wrapping_add(1);
            (*ps).escapes = (*ps).escapes.wrapping_sub(1)
        } else {
            if (*ps).escapes != 0u32 {
                yylex_ungetc(ch);
                (*ps).escapes = (*ps).escapes.wrapping_sub(1);
                return '\\' as i32;
            }
            return ch;
        }
    }
}
unsafe extern "C" fn yylex_get_word(mut ch: libc::c_int) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    len = 0u64;
    buf = xmalloc(1u64) as *mut libc::c_char;
    loop {
        yylex_append1(&mut buf, &mut len, ch as libc::c_char);
        ch = yylex_getc();
        if !(ch != -(1i32)
            && strchr(b" \t\n\x00" as *const u8 as *const libc::c_char, ch).is_null())
        {
            break;
        }
    }
    yylex_ungetc(ch);
    *buf.offset(len as isize) = '\u{0}' as libc::c_char;
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"yylex_get_word\x00")).as_ptr(),
        buf,
    );
    return buf;
}
/* A Bison parser, made by GNU Bison 3.7.2.  */
/* Bison implementation for Yacc-like parsers in C

Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2020 Free Software Foundation,
Inc.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.  */
/* As a special exception, you may create a larger work that contains
part or all of the Bison parser skeleton and distribute that work
under terms of your choice, so long as that work isn't itself a
parser generator using the skeleton or a modified version thereof
as a parser skeleton.  Alternatively, if you modify or redistribute
the parser skeleton itself, you may (at your option) remove this
special exception, which will cause the skeleton and the resulting
Bison output files to be licensed under the GNU General Public
License without this special exception.

This special exception was added by the Free Software Foundation in
version 2.2 of Bison.  */
unsafe extern "C" fn yylex() -> libc::c_int {
    let mut ps: *mut cmd_parse_state = &mut parse_state;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut condition: libc::c_int = 0;
    if (*ps).eol != 0 {
        (*(*ps).input).line = (*(*ps).input).line.wrapping_add(1)
    }
    (*ps).eol = 0i32;
    condition = (*ps).condition;
    (*ps).condition = 0i32;
    loop {
        ch = yylex_getc();
        if ch == -(1i32) {
            if (*ps).eof != 0 {
                break;
            }
            (*ps).eof = 1i32;
            return '\n' as i32;
        } else {
            if ch == ' ' as i32 || ch == '\t' as i32 {
                continue;
            }
            if ch == '\n' as i32 {
                (*ps).eol = 1i32;
                return '\n' as i32;
            }
            if ch == ';' as i32 || ch == '{' as i32 || ch == '}' as i32 {
                return ch;
            }
            if ch == '#' as i32 {
                next = yylex_getc();
                if condition != 0 && next == '{' as i32 {
                    yylval.token = yylex_format();
                    if yylval.token.is_null() {
                        return 258i32;
                    }
                    return 264i32;
                }
                while next != '\n' as i32 && next != -(1i32) {
                    next = yylex_getc()
                }
                if next == '\n' as i32 {
                    (*(*ps).input).line = (*(*ps).input).line.wrapping_add(1);
                    return '\n' as i32;
                }
            } else {
                if ch == '%' as i32 {
                    yylval.token = yylex_get_word('%' as i32);
                    cp = yylval.token;
                    while *cp as libc::c_int != '\u{0}' as i32 {
                        if *cp as libc::c_int != '%' as i32
                            && *(*__ctype_b_loc()).offset(*cp as u_char as libc::c_int as isize)
                                as libc::c_int
                                & _ISdigit as libc::c_ushort as libc::c_int
                                == 0
                        {
                            break;
                        }
                        cp = cp.offset(1)
                    }
                    if *cp as libc::c_int == '\u{0}' as i32 {
                        return 265i32;
                    }
                    (*ps).condition = 1i32;
                    if strcmp(
                        yylval.token,
                        b"%hidden\x00" as *const u8 as *const libc::c_char,
                    ) == 0i32
                    {
                        free(yylval.token as *mut libc::c_void);
                        return 259i32;
                    }
                    if strcmp(yylval.token, b"%if\x00" as *const u8 as *const libc::c_char) == 0i32
                    {
                        free(yylval.token as *mut libc::c_void);
                        return 260i32;
                    }
                    if strcmp(
                        yylval.token,
                        b"%else\x00" as *const u8 as *const libc::c_char,
                    ) == 0i32
                    {
                        free(yylval.token as *mut libc::c_void);
                        return 261i32;
                    }
                    if strcmp(
                        yylval.token,
                        b"%elif\x00" as *const u8 as *const libc::c_char,
                    ) == 0i32
                    {
                        free(yylval.token as *mut libc::c_void);
                        return 262i32;
                    }
                    if strcmp(
                        yylval.token,
                        b"%endif\x00" as *const u8 as *const libc::c_char,
                    ) == 0i32
                    {
                        free(yylval.token as *mut libc::c_void);
                        return 263i32;
                    }
                    free(yylval.token as *mut libc::c_void);
                    return 258i32;
                }
                token = yylex_token(ch);
                if token.is_null() {
                    return 258i32;
                }
                yylval.token = token;
                if !strchr(token, '=' as i32).is_null() && yylex_is_var(*token, 1i32) != 0 {
                    cp = token.offset(1isize);
                    while *cp as libc::c_int != '=' as i32 {
                        if yylex_is_var(*cp, 0i32) == 0 {
                            break;
                        }
                        cp = cp.offset(1)
                    }
                    if *cp as libc::c_int == '=' as i32 {
                        return 266i32;
                    }
                }
                return 265i32;
            }
        }
    }
    return 0i32;
}
unsafe extern "C" fn yylex_format() -> *mut libc::c_char {
    let mut current_block: u64;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut ch: libc::c_int = 0;
    let mut brackets: libc::c_int = 1i32;
    len = 0u64;
    buf = xmalloc(1u64) as *mut libc::c_char;
    yylex_append(
        &mut buf,
        &mut len,
        b"#{\x00" as *const u8 as *const libc::c_char,
        2u64,
    );
    loop {
        ch = yylex_getc();
        if ch == -(1i32) || ch == '\n' as i32 {
            current_block = 10567711132991650300;
            break;
        }
        if ch == '#' as i32 {
            ch = yylex_getc();
            if ch == -(1i32) || ch == '\n' as i32 {
                current_block = 10567711132991650300;
                break;
            }
            if ch == '{' as i32 {
                brackets += 1
            }
            yylex_append1(&mut buf, &mut len, '#' as libc::c_char);
        } else if ch == '}' as i32 {
            if brackets != 0i32 && {
                brackets -= 1;
                (brackets) == 0i32
            } {
                yylex_append1(&mut buf, &mut len, ch as libc::c_char);
                current_block = 8457315219000651999;
                break;
            }
        }
        yylex_append1(&mut buf, &mut len, ch as libc::c_char);
    }
    match current_block {
        8457315219000651999 => {
            if !(brackets != 0i32) {
                *buf.offset(len as isize) = '\u{0}' as libc::c_char;
                log_debug(
                    b"%s: %s\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"yylex_format\x00"))
                        .as_ptr(),
                    buf,
                );
                return buf;
            }
        }
        _ => {}
    }
    free(buf as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn yylex_token_escape(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ch: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut o2: libc::c_int = 0;
    let mut o3: libc::c_int = 0;
    let mut mlen: libc::c_int = 0;
    let mut size: u_int = 0;
    let mut i: u_int = 0;
    let mut tmp: u_int = 0;
    let mut s: [libc::c_char; 9] = [0; 9];
    let mut m: [libc::c_char; 16] = [0; 16];
    ch = yylex_getc();
    if ch >= '4' as i32 && ch <= '7' as i32 {
        yyerror(b"invalid octal escape\x00" as *const u8 as *const libc::c_char);
        return 0i32;
    }
    if ch >= '0' as i32 && ch <= '3' as i32 {
        o2 = yylex_getc();
        if o2 >= '0' as i32 && o2 <= '7' as i32 {
            o3 = yylex_getc();
            if o3 >= '0' as i32 && o3 <= '7' as i32 {
                ch = 64i32 * (ch - '0' as i32) + 8i32 * (o2 - '0' as i32) + (o3 - '0' as i32);
                yylex_append1(buf, len, ch as libc::c_char);
                return 1i32;
            }
        }
        yyerror(b"invalid octal escape\x00" as *const u8 as *const libc::c_char);
        return 0i32;
    }
    match ch {
        -1 => return 0i32,
        97 => {
            ch = '\u{7}' as i32;
            current_block = 14832935472441733737;
        }
        98 => {
            ch = '\u{8}' as i32;
            current_block = 14832935472441733737;
        }
        101 => {
            ch = '\u{1b}' as i32;
            current_block = 14832935472441733737;
        }
        102 => {
            ch = '\u{c}' as i32;
            current_block = 14832935472441733737;
        }
        115 => {
            ch = ' ' as i32;
            current_block = 14832935472441733737;
        }
        118 => {
            ch = '\u{b}' as i32;
            current_block = 14832935472441733737;
        }
        114 => {
            ch = '\r' as i32;
            current_block = 14832935472441733737;
        }
        110 => {
            ch = '\n' as i32;
            current_block = 14832935472441733737;
        }
        116 => {
            ch = '\t' as i32;
            current_block = 14832935472441733737;
        }
        117 => {
            type_0 = 'u' as i32;
            size = 4u32;
            current_block = 5366602412045410634;
        }
        85 => {
            type_0 = 'U' as i32;
            size = 8u32;
            current_block = 5366602412045410634;
        }
        _ => {
            current_block = 14832935472441733737;
        }
    }
    match current_block {
        5366602412045410634 => {
            i = 0u32;
            while i < size {
                ch = yylex_getc();
                if ch == -(1i32) || ch == '\n' as i32 {
                    return 0i32;
                }
                if *(*__ctype_b_loc()).offset(ch as u_char as libc::c_int as isize) as libc::c_int
                    & _ISxdigit as libc::c_ushort as libc::c_int
                    == 0
                {
                    yyerror(
                        b"invalid \\%c argument\x00" as *const u8 as *const libc::c_char,
                        type_0,
                    );
                    return 0i32;
                }
                s[i as usize] = ch as libc::c_char;
                i = i.wrapping_add(1)
            }
            s[i as usize] = '\u{0}' as libc::c_char;
            if size == 4u32
                && sscanf(
                    s.as_mut_ptr(),
                    b"%4x\x00" as *const u8 as *const libc::c_char,
                    &mut tmp as *mut u_int,
                ) != 1i32
                || size == 8u32
                    && sscanf(
                        s.as_mut_ptr(),
                        b"%8x\x00" as *const u8 as *const libc::c_char,
                        &mut tmp as *mut u_int,
                    ) != 1i32
            {
                yyerror(
                    b"invalid \\%c argument\x00" as *const u8 as *const libc::c_char,
                    type_0,
                );
                return 0i32;
            }
            mlen = wctomb(m.as_mut_ptr(), tmp as wchar_t);
            if mlen <= 0i32 || mlen > ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_int {
                yyerror(
                    b"invalid \\%c argument\x00" as *const u8 as *const libc::c_char,
                    type_0,
                );
                return 0i32;
            }
            yylex_append(buf, len, m.as_mut_ptr(), mlen as size_t);
            return 1i32;
        }
        _ => {
            yylex_append1(buf, len, ch as libc::c_char);
            return 1i32;
        }
    };
}
unsafe extern "C" fn yylex_token_variable(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
) -> libc::c_int {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut ch: libc::c_int = 0;
    let mut brackets: libc::c_int = 0i32;
    let mut name: [libc::c_char; 1024] = [0; 1024];
    let mut namelen: size_t = 0u64;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    ch = yylex_getc();
    if ch == -(1i32) {
        return 0i32;
    }
    if ch == '{' as i32 {
        brackets = 1i32
    } else {
        if yylex_is_var(ch as libc::c_char, 1i32) == 0 {
            yylex_append1(buf, len, '$' as libc::c_char);
            yylex_ungetc(ch);
            return 1i32;
        }
        let fresh15 = namelen;
        namelen = namelen.wrapping_add(1);
        name[fresh15 as usize] = ch as libc::c_char
    }
    loop {
        ch = yylex_getc();
        if brackets != 0 && ch == '}' as i32 {
            break;
        }
        if ch == -(1i32) || yylex_is_var(ch as libc::c_char, 0i32) == 0 {
            if brackets == 0 {
                yylex_ungetc(ch);
                break;
            } else {
                yyerror(b"invalid environment variable\x00" as *const u8 as *const libc::c_char);
                return 0i32;
            }
        } else {
            if namelen
                == (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(2u64)
            {
                yyerror(
                    b"environment variable is too long\x00" as *const u8 as *const libc::c_char,
                );
                return 0i32;
            }
            let fresh16 = namelen;
            namelen = namelen.wrapping_add(1);
            name[fresh16 as usize] = ch as libc::c_char
        }
    }
    name[namelen as usize] = '\u{0}' as libc::c_char;
    envent = environ_find(global_environ, name.as_mut_ptr());
    if !envent.is_null() && !(*envent).value.is_null() {
        value = (*envent).value;
        log_debug(
            b"%s: %s -> %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"yylex_token_variable\x00"))
                .as_ptr(),
            name.as_mut_ptr(),
            value,
        );
        yylex_append(buf, len, value, strlen(value));
    }
    return 1i32;
}
unsafe extern "C" fn yylex_token_tilde(
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
) -> libc::c_int {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut ch: libc::c_int = 0;
    let mut name: [libc::c_char; 1024] = [0; 1024];
    let mut namelen: size_t = 0u64;
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        ch = yylex_getc();
        if ch == -(1i32)
            || !strchr(b"/ \t\n\"\'\x00" as *const u8 as *const libc::c_char, ch).is_null()
        {
            yylex_ungetc(ch);
            break;
        } else {
            if namelen
                == (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(2u64)
            {
                yyerror(b"user name is too long\x00" as *const u8 as *const libc::c_char);
                return 0i32;
            }
            let fresh17 = namelen;
            namelen = namelen.wrapping_add(1);
            name[fresh17 as usize] = ch as libc::c_char
        }
    }
    name[namelen as usize] = '\u{0}' as libc::c_char;
    if *name.as_mut_ptr() as libc::c_int == '\u{0}' as i32 {
        envent = environ_find(
            global_environ,
            b"HOME\x00" as *const u8 as *const libc::c_char,
        );
        if !envent.is_null() && *(*envent).value as libc::c_int != '\u{0}' as i32 {
            home = (*envent).value
        } else {
            pw = getpwuid(getuid());
            if !pw.is_null() {
                home = (*pw).pw_dir
            }
        }
    } else {
        pw = getpwnam(name.as_mut_ptr());
        if !pw.is_null() {
            home = (*pw).pw_dir
        }
    }
    if home.is_null() {
        return 0i32;
    }
    log_debug(
        b"%s: ~%s -> %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"yylex_token_tilde\x00"))
            .as_ptr(),
        name.as_mut_ptr(),
        home,
    );
    yylex_append(buf, len, home, strlen(home));
    return 1i32;
}
unsafe extern "C" fn yylex_token(mut ch: libc::c_int) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut state: C2RustUnnamed_39 = NONE;
    let mut last: C2RustUnnamed_39 = START;
    len = 0u64;
    buf = xmalloc(1u64) as *mut libc::c_char;
    loop {
        if ch == -(1i32) || state == NONE && ch == '\n' as i32 {
            current_block = 10399321362245223758;
            break;
        }
        if (ch == ' ' as i32 || ch == '\t' as i32 || ch == ';' as i32 || ch == '}' as i32)
            && state == NONE
        {
            current_block = 10399321362245223758;
            break;
        }
        if ch == '\n' as i32 && state != NONE {
            loop {
                ch = yylex_getc();
                if !(ch == ' ' as i32 || ch == '\t' as i32) {
                    break;
                }
            }
            if ch != '#' as i32 {
                continue;
            }
            ch = yylex_getc();
            if !strchr(b",#{}:\x00" as *const u8 as *const libc::c_char, ch).is_null() {
                yylex_ungetc(ch);
                ch = '#' as i32
            } else {
                loop {
                    ch = yylex_getc();
                    if !(ch != '\n' as i32 && ch != -(1i32)) {
                        break;
                    }
                }
            }
        } else {
            if ch == '\\' as i32 && state != SINGLE_QUOTES {
                if yylex_token_escape(&mut buf, &mut len) == 0 {
                    current_block = 10067117643406552625;
                    break;
                }
                current_block = 6304803324601632414;
            } else if ch == '~' as i32 && last != state && state != SINGLE_QUOTES {
                if yylex_token_tilde(&mut buf, &mut len) == 0 {
                    current_block = 10067117643406552625;
                    break;
                }
                current_block = 6304803324601632414;
            } else if ch == '$' as i32 && state != SINGLE_QUOTES {
                if yylex_token_variable(&mut buf, &mut len) == 0 {
                    current_block = 10067117643406552625;
                    break;
                }
                current_block = 6304803324601632414;
            } else {
                if ch == '}' as i32 && state == NONE {
                    current_block = 10067117643406552625;
                    break;
                }
                if ch == '\'' as i32 {
                    if state == NONE {
                        state = SINGLE_QUOTES;
                        current_block = 7314257418503315463;
                    } else if state == SINGLE_QUOTES {
                        state = NONE;
                        current_block = 7314257418503315463;
                    } else {
                        current_block = 1836292691772056875;
                    }
                } else {
                    current_block = 1836292691772056875;
                }
                match current_block {
                    7314257418503315463 => {}
                    _ => {
                        if ch == '\"' as i32 {
                            if state == NONE {
                                state = DOUBLE_QUOTES;
                                current_block = 7314257418503315463;
                            } else if state == DOUBLE_QUOTES {
                                state = NONE;
                                current_block = 7314257418503315463;
                            } else {
                                current_block = 2543120759711851213;
                            }
                        } else {
                            current_block = 2543120759711851213;
                        }
                        match current_block {
                            7314257418503315463 => {}
                            _ => {
                                yylex_append1(&mut buf, &mut len, ch as libc::c_char);
                                current_block = 6304803324601632414;
                            }
                        }
                    }
                }
            }
            match current_block {
                6304803324601632414 => last = state,
                _ => {}
            }
            ch = yylex_getc()
        }
    }
    match current_block {
        10067117643406552625 => {
            free(buf as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        _ => {
            yylex_ungetc(ch);
            *buf.offset(len as isize) = '\u{0}' as libc::c_char;
            log_debug(
                b"%s: %s\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"yylex_token\x00"))
                    .as_ptr(),
                buf,
            );
            return buf;
        }
    };
}
