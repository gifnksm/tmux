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
    pub type cmd;
    pub type cmdq_item;
    pub type options_array_item;
    pub type options_entry;
    pub type screen_write_collect_item;
    pub type paste_buffer;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
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
    fn fgetln(_: *mut FILE, _: *mut size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xstrndup(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xvasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut options;
    #[no_mangle]
    fn find_home() -> *const libc::c_char;
    #[no_mangle]
    fn paste_buffer_data(_: *mut paste_buffer, _: *mut size_t) -> *const libc::c_char;
    #[no_mangle]
    fn paste_get_top(_: *mut *const libc::c_char) -> *mut paste_buffer;
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
    fn format_expand_time(_: *mut format_tree, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn format_create_defaults(
        _: *mut cmdq_item,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: *mut window_pane,
    ) -> *mut format_tree;
    #[no_mangle]
    fn format_create_from_state(
        _: *mut cmdq_item,
        _: *mut client,
        _: *mut cmd_find_state,
    ) -> *mut format_tree;
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
    fn options_get_only(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    #[no_mangle]
    fn options_get(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    #[no_mangle]
    fn options_array_get(_: *mut options_entry, _: u_int) -> *mut options_value;
    #[no_mangle]
    fn options_array_first(_: *mut options_entry) -> *mut options_array_item;
    #[no_mangle]
    fn options_array_next(_: *mut options_array_item) -> *mut options_array_item;
    #[no_mangle]
    fn options_array_item_value(_: *mut options_array_item) -> *mut options_value;
    #[no_mangle]
    fn options_get_string(_: *mut options, _: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    static mut cmd_table: [*const cmd_entry; 0];
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn server_add_message(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_free(_: *mut screen);
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_cursormove(
        _: *mut screen_write_ctx,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_write_putc(_: *mut screen_write_ctx, _: *const grid_cell, _: u_char);
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut screen);
    #[no_mangle]
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int);
    #[no_mangle]
    fn grid_cells_equal(_: *const grid_cell, _: *const grid_cell) -> libc::c_int;
    #[no_mangle]
    fn grid_compare(_: *mut grid, _: *mut grid) -> libc::c_int;
    #[no_mangle]
    fn screen_write_nputs(
        _: *mut screen_write_ctx,
        _: ssize_t,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: ...
    );
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
    fn screen_write_strlen(_: *const libc::c_char, _: ...) -> size_t;
    #[no_mangle]
    fn screen_write_cell(_: *mut screen_write_ctx, _: *const grid_cell);
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn utf8_set(_: *mut utf8_data, _: u_char);
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    fn session_find(_: *const libc::c_char) -> *mut session;
    #[no_mangle]
    fn utf8_to_data(_: utf8_char, _: *mut utf8_data);
    #[no_mangle]
    fn menu_free(_: *mut menu);
    #[no_mangle]
    fn utf8_strlen(_: *const utf8_data) -> size_t;
    #[no_mangle]
    fn utf8_fromcstr(_: *const libc::c_char) -> *mut utf8_data;
    #[no_mangle]
    fn utf8_copy(_: *mut utf8_data, _: *const utf8_data);
    #[no_mangle]
    fn utf8_strwidth(_: *const utf8_data, _: ssize_t) -> u_int;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn utf8_tocstr(_: *mut utf8_data) -> *mut libc::c_char;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn menu_create(_: *const libc::c_char) -> *mut menu;
    #[no_mangle]
    fn menu_add_item(
        _: *mut menu,
        _: *const menu_item,
        _: *mut cmdq_item,
        _: *mut client,
        _: *mut cmd_find_state,
    );
    #[no_mangle]
    fn utf8_cstrwidth(_: *const libc::c_char) -> u_int;
    #[no_mangle]
    fn menu_display(
        _: *mut menu,
        _: libc::c_int,
        _: *mut cmdq_item,
        _: u_int,
        _: u_int,
        _: *mut client,
        _: *mut cmd_find_state,
        _: menu_choice_cb,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    fn style_apply(_: *mut grid_cell, _: *mut options, _: *const libc::c_char, _: *mut format_tree);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type va_list = __builtin_va_list;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
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
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
pub type style_align = libc::c_uint;
pub const STYLE_ALIGN_RIGHT: style_align = 3;
pub const STYLE_ALIGN_CENTRE: style_align = 2;
pub const STYLE_ALIGN_LEFT: style_align = 1;
pub const STYLE_ALIGN_DEFAULT: style_align = 0;
pub type style_list = libc::c_uint;
pub const STYLE_LIST_RIGHT_MARKER: style_list = 4;
pub const STYLE_LIST_LEFT_MARKER: style_list = 3;
pub const STYLE_LIST_FOCUS: style_list = 2;
pub const STYLE_LIST_ON: style_list = 1;
pub const STYLE_LIST_OFF: style_list = 0;
pub type style_default_type = libc::c_uint;
pub const STYLE_DEFAULT_POP: style_default_type = 2;
pub const STYLE_DEFAULT_PUSH: style_default_type = 1;
pub const STYLE_DEFAULT_BASE: style_default_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct style {
    pub gc: grid_cell,
    pub ignore: libc::c_int,
    pub fill: libc::c_int,
    pub align: style_align,
    pub list: style_list,
    pub range_type: style_range_type,
    pub range_argument: u_int,
    pub default_type: style_default_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_item {
    pub name: *const libc::c_char,
    pub key: key_code,
    pub command: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu {
    pub title: *const libc::c_char,
    pub items: *mut menu_item,
    pub count: u_int,
    pub width: u_int,
}
pub type menu_choice_cb =
    Option<unsafe extern "C" fn(_: *mut menu, _: u_int, _: key_code, _: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sessions {
    pub rbh_root: *mut session,
}
pub type cmd_find_type = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;
pub type cmd_retval = libc::c_int;
pub const CMD_RETURN_STOP: cmd_retval = 2;
pub const CMD_RETURN_WAIT: cmd_retval = 1;
pub const CMD_RETURN_NORMAL: cmd_retval = 0;
pub const CMD_RETURN_ERROR: cmd_retval = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry_flag {
    pub flag: libc::c_char,
    pub type_0: cmd_find_type,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: C2RustUnnamed_33,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item) -> cmd_retval>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_array {
    pub rbh_root: *mut options_array_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union options_value {
    pub string: *mut libc::c_char,
    pub number: libc::c_longlong,
    pub style: style,
    pub array: options_array,
    pub cmdlist: *mut cmd_list,
}
pub type options_table_type = libc::c_uint;
pub const OPTIONS_TABLE_COMMAND: options_table_type = 6;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 5;
pub const OPTIONS_TABLE_FLAG: options_table_type = 4;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_table_entry {
    pub name: *const libc::c_char,
    pub alternative_name: *const libc::c_char,
    pub type_0: options_table_type,
    pub scope: libc::c_int,
    pub flags: libc::c_int,
    pub minimum: u_int,
    pub maximum: u_int,
    pub choices: *mut *const libc::c_char,
    pub default_str: *const libc::c_char,
    pub default_num: libc::c_longlong,
    pub default_arr: *mut *const libc::c_char,
    pub separator: *const libc::c_char,
    pub pattern: *const libc::c_char,
    pub text: *const libc::c_char,
    pub unit: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct status_prompt_menu {
    pub c: *mut client,
    pub start: u_int,
    pub size: u_int,
    pub list: *mut *mut libc::c_char,
    pub flag: libc::c_char,
}
static mut status_prompt_hlist: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
static mut status_prompt_hsize: u_int = 0;
/* Find the history file to load/save from/to. */
unsafe extern "C" fn status_prompt_find_history_file() -> *mut libc::c_char {
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut history_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    history_file = options_get_string(
        global_options,
        b"history-file\x00" as *const u8 as *const libc::c_char,
    );
    if *history_file as libc::c_int == '\u{0}' as i32 {
        return 0 as *mut libc::c_char;
    }
    if *history_file as libc::c_int == '/' as i32 {
        return xstrdup(history_file);
    }
    if *history_file.offset(0 as libc::c_int as isize) as libc::c_int != '~' as i32
        || *history_file.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32
    {
        return 0 as *mut libc::c_char;
    }
    home = find_home();
    if home.is_null() {
        return 0 as *mut libc::c_char;
    }
    xasprintf(
        &mut path as *mut *mut libc::c_char,
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        home,
        history_file.offset(1 as libc::c_int as isize),
    );
    return path;
}
/* Load status prompt history from file. */
#[no_mangle]
pub unsafe extern "C" fn status_prompt_load_history() {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut history_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: size_t = 0;
    history_file = status_prompt_find_history_file();
    if history_file.is_null() {
        return;
    }
    log_debug(
        b"loading history from %s\x00" as *const u8 as *const libc::c_char,
        history_file,
    );
    f = fopen(history_file, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        log_debug(
            b"%s: %s\x00" as *const u8 as *const libc::c_char,
            history_file,
            strerror(*__errno_location()),
        );
        free(history_file as *mut libc::c_void);
        return;
    }
    free(history_file as *mut libc::c_void);
    loop {
        line = fgetln(f, &mut length);
        if line.is_null() {
            break;
        }
        if length > 0 as libc::c_int as libc::c_ulong {
            if *line.offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '\n' as i32
            {
                *line.offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
                    '\u{0}' as i32 as libc::c_char;
                status_prompt_add_history(line);
            } else {
                tmp = xmalloc(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as *mut libc::c_char;
                memcpy(
                    tmp as *mut libc::c_void,
                    line as *const libc::c_void,
                    length,
                );
                *tmp.offset(length as isize) = '\u{0}' as i32 as libc::c_char;
                status_prompt_add_history(tmp);
                free(tmp as *mut libc::c_void);
            }
        }
    }
    fclose(f);
}
/* Save status prompt history to file. */
#[no_mangle]
pub unsafe extern "C" fn status_prompt_save_history() {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: u_int = 0;
    let mut history_file: *mut libc::c_char = 0 as *mut libc::c_char;
    history_file = status_prompt_find_history_file();
    if history_file.is_null() {
        return;
    }
    log_debug(
        b"saving history to %s\x00" as *const u8 as *const libc::c_char,
        history_file,
    );
    f = fopen(history_file, b"w\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        log_debug(
            b"%s: %s\x00" as *const u8 as *const libc::c_char,
            history_file,
            strerror(*__errno_location()),
        );
        free(history_file as *mut libc::c_void);
        return;
    }
    free(history_file as *mut libc::c_void);
    i = 0 as libc::c_int as u_int;
    while i < status_prompt_hsize {
        fputs(*status_prompt_hlist.offset(i as isize), f);
        fputc('\n' as i32, f);
        i = i.wrapping_add(1)
    }
    fclose(f);
}
/* Status timer callback. */
unsafe extern "C" fn status_timer_callback(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut c: *mut client = arg as *mut client;
    let mut s: *mut session = (*c).session;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    event_del(&mut (*c).status.timer);
    if s.is_null() {
        return;
    }
    if (*c).message_string.is_null() && (*c).prompt_string.is_null() {
        (*c).flags |= 0x10 as libc::c_int as libc::c_ulong
    }
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    tv.tv_sec = tv.tv_usec;
    tv.tv_sec = options_get_number(
        (*s).options,
        b"status-interval\x00" as *const u8 as *const libc::c_char,
    ) as __time_t;
    if tv.tv_sec != 0 as libc::c_int as libc::c_long {
        event_add(&mut (*c).status.timer, &mut tv);
    }
    log_debug(
        b"client %p, status interval %d\x00" as *const u8 as *const libc::c_char,
        c,
        tv.tv_sec as libc::c_int,
    );
}
/* Start status timer for client. */
#[no_mangle]
pub unsafe extern "C" fn status_timer_start(mut c: *mut client) {
    let mut s: *mut session = (*c).session;
    if event_initialized(&mut (*c).status.timer) != 0 {
        event_del(&mut (*c).status.timer);
    } else {
        event_set(
            &mut (*c).status.timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                status_timer_callback
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            c as *mut libc::c_void,
        );
    }
    if !s.is_null()
        && options_get_number(
            (*s).options,
            b"status\x00" as *const u8 as *const libc::c_char,
        ) != 0
    {
        status_timer_callback(
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            c as *mut libc::c_void,
        );
    };
}
/* Start status timer for all clients. */
#[no_mangle]
pub unsafe extern "C" fn status_timer_start_all() {
    let mut c: *mut client = 0 as *mut client;
    c = clients.tqh_first;
    while !c.is_null() {
        status_timer_start(c);
        c = (*c).entry.tqe_next
    }
}
/* Update status cache. */
#[no_mangle]
pub unsafe extern "C" fn status_update_cache(mut s: *mut session) {
    (*s).statuslines = options_get_number(
        (*s).options,
        b"status\x00" as *const u8 as *const libc::c_char,
    ) as u_int;
    if (*s).statuslines == 0 as libc::c_int as libc::c_uint {
        (*s).statusat = -(1 as libc::c_int)
    } else if options_get_number(
        (*s).options,
        b"status-position\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int as libc::c_longlong
    {
        (*s).statusat = 0 as libc::c_int
    } else {
        (*s).statusat = 1 as libc::c_int
    };
}
/* Get screen line of status line. -1 means off. */
#[no_mangle]
pub unsafe extern "C" fn status_at_line(mut c: *mut client) -> libc::c_int {
    let mut s: *mut session = (*c).session;
    if (*c).flags & (0x800000 as libc::c_int | 0x2000 as libc::c_int) as libc::c_ulong != 0 {
        return -(1 as libc::c_int);
    }
    if (*s).statusat != 1 as libc::c_int {
        return (*s).statusat;
    }
    return (*c).tty.sy.wrapping_sub(status_line_size(c)) as libc::c_int;
}
/* Get size of status line for client's session. 0 means off. */
#[no_mangle]
pub unsafe extern "C" fn status_line_size(mut c: *mut client) -> u_int {
    let mut s: *mut session = (*c).session;
    if (*c).flags & (0x800000 as libc::c_int | 0x2000 as libc::c_int) as libc::c_ulong != 0 {
        return 0 as libc::c_int as u_int;
    }
    return (*s).statuslines;
}
/* Get window at window list position. */
#[no_mangle]
pub unsafe extern "C" fn status_get_range(
    mut c: *mut client,
    mut x: u_int,
    mut y: u_int,
) -> *mut style_range {
    let mut sl: *mut status_line = &mut (*c).status;
    let mut sr: *mut style_range = 0 as *mut style_range;
    if y as libc::c_ulong
        >= (::std::mem::size_of::<[status_line_entry; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<status_line_entry>() as libc::c_ulong)
    {
        return 0 as *mut style_range;
    }
    sr = (*sl).entries[y as usize].ranges.tqh_first;
    while !sr.is_null() {
        if x >= (*sr).start && x < (*sr).end {
            return sr;
        }
        sr = (*sr).entry.tqe_next
    }
    return 0 as *mut style_range;
}
/* Free all ranges. */
unsafe extern "C" fn status_free_ranges(mut srs: *mut style_ranges) {
    let mut sr: *mut style_range = 0 as *mut style_range;
    let mut sr1: *mut style_range = 0 as *mut style_range;
    sr = (*srs).tqh_first;
    while !sr.is_null() && {
        sr1 = (*sr).entry.tqe_next;
        (1 as libc::c_int) != 0
    } {
        if !(*sr).entry.tqe_next.is_null() {
            (*(*sr).entry.tqe_next).entry.tqe_prev = (*sr).entry.tqe_prev
        } else {
            (*srs).tqh_last = (*sr).entry.tqe_prev
        }
        *(*sr).entry.tqe_prev = (*sr).entry.tqe_next;
        free(sr as *mut libc::c_void);
        sr = sr1
    }
}
/* Save old status line. */
unsafe extern "C" fn status_push_screen(mut c: *mut client) {
    let mut sl: *mut status_line = &mut (*c).status;
    if (*sl).active == &mut (*sl).screen as *mut screen {
        (*sl).active = xmalloc(::std::mem::size_of::<screen>() as libc::c_ulong) as *mut screen;
        screen_init(
            (*sl).active,
            (*c).tty.sx,
            status_line_size(c),
            0 as libc::c_int as u_int,
        );
    }
    (*sl).references += 1;
}
/* Restore old status line. */
unsafe extern "C" fn status_pop_screen(mut c: *mut client) {
    let mut sl: *mut status_line = &mut (*c).status;
    (*sl).references -= 1;
    if (*sl).references == 0 as libc::c_int {
        screen_free((*sl).active);
        free((*sl).active as *mut libc::c_void);
        (*sl).active = &mut (*sl).screen
    };
}
/* Initialize status line. */
#[no_mangle]
pub unsafe extern "C" fn status_init(mut c: *mut client) {
    let mut sl: *mut status_line = &mut (*c).status;
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[status_line_entry; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<status_line_entry>() as libc::c_ulong)
    {
        (*sl).entries[i as usize].ranges.tqh_first = 0 as *mut style_range;
        (*sl).entries[i as usize].ranges.tqh_last =
            &mut (*(*sl).entries.as_mut_ptr().offset(i as isize))
                .ranges
                .tqh_first;
        i = i.wrapping_add(1)
    }
    screen_init(
        &mut (*sl).screen,
        (*c).tty.sx,
        1 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    (*sl).active = &mut (*sl).screen;
}
/* Free status line. */
#[no_mangle]
pub unsafe extern "C" fn status_free(mut c: *mut client) {
    let mut sl: *mut status_line = &mut (*c).status;
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[status_line_entry; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<status_line_entry>() as libc::c_ulong)
    {
        status_free_ranges(&mut (*(*sl).entries.as_mut_ptr().offset(i as isize)).ranges);
        free((*sl).entries[i as usize].expanded as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    if event_initialized(&mut (*sl).timer) != 0 {
        event_del(&mut (*sl).timer);
    }
    if (*sl).active != &mut (*sl).screen as *mut screen {
        screen_free((*sl).active);
        free((*sl).active as *mut libc::c_void);
    }
    screen_free(&mut (*sl).screen);
}
/* Draw status line for client. */
#[no_mangle]
pub unsafe extern "C" fn status_redraw(mut c: *mut client) -> libc::c_int {
    let mut sl: *mut status_line = &mut (*c).status;
    let mut sle: *mut status_line_entry = 0 as *mut status_line_entry;
    let mut s: *mut session = (*c).session;
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
    let mut gc: grid_cell = grid_cell {
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
    };
    let mut lines: u_int = 0;
    let mut i: u_int = 0;
    let mut n: u_int = 0;
    let mut width: u_int = (*c).tty.sx;
    let mut flags: libc::c_int = 0;
    let mut force: libc::c_int = 0 as libc::c_int;
    let mut changed: libc::c_int = 0 as libc::c_int;
    let mut fg: libc::c_int = 0;
    let mut bg: libc::c_int = 0;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut ov: *mut options_value = 0 as *mut options_value;
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    log_debug(
        b"%s enter\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"status_redraw\x00")).as_ptr(),
    );
    /* Shouldn't get here if not the active screen. */
    if (*sl).active != &mut (*sl).screen as *mut screen {
        fatalx(b"not the active screen\x00" as *const u8 as *const libc::c_char);
    }
    /* No status line? */
    lines = status_line_size(c);
    if (*c).tty.sy == 0 as libc::c_int as libc::c_uint || lines == 0 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    /* Create format tree. */
    flags = 0x1 as libc::c_int;
    if (*c).flags & 0x80000 as libc::c_int as libc::c_ulong != 0 {
        flags |= 0x2 as libc::c_int
    }
    ft = format_create(c, 0 as *mut cmdq_item, 0 as libc::c_int, flags);
    format_defaults(
        ft,
        c,
        0 as *mut session,
        0 as *mut winlink,
        0 as *mut window_pane,
    );
    /* Set up default colour. */
    style_apply(
        &mut gc,
        (*s).options,
        b"status-style\x00" as *const u8 as *const libc::c_char,
        ft,
    );
    fg = options_get_number(
        (*s).options,
        b"status-fg\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if fg != 8 as libc::c_int {
        gc.fg = fg
    }
    bg = options_get_number(
        (*s).options,
        b"status-bg\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if bg != 8 as libc::c_int {
        gc.bg = bg
    }
    if grid_cells_equal(&mut gc, &mut (*sl).style) == 0 {
        force = 1 as libc::c_int;
        memcpy(
            &mut (*sl).style as *mut grid_cell as *mut libc::c_void,
            &mut gc as *mut grid_cell as *const libc::c_void,
            ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
    }
    /* Resize the target screen. */
    if (*(*sl).screen.grid).sx != width || (*(*sl).screen.grid).sy != lines {
        screen_resize(&mut (*sl).screen, width, lines, 0 as libc::c_int);
        force = 1 as libc::c_int;
        changed = force
    }
    screen_write_start(&mut ctx, &mut (*sl).screen);
    /* Write the status lines. */
    o = options_get(
        (*s).options,
        b"status-format\x00" as *const u8 as *const libc::c_char,
    );
    if o.is_null() {
        n = 0 as libc::c_int as u_int;
        while n < width.wrapping_mul(lines) {
            screen_write_putc(&mut ctx, &mut gc, ' ' as i32 as u_char);
            n = n.wrapping_add(1)
        }
    } else {
        i = 0 as libc::c_int as u_int;
        while i < lines {
            screen_write_cursormove(
                &mut ctx,
                0 as libc::c_int,
                i as libc::c_int,
                0 as libc::c_int,
            );
            ov = options_array_get(o, i);
            if ov.is_null() {
                n = 0 as libc::c_int as u_int;
                while n < width {
                    screen_write_putc(&mut ctx, &mut gc, ' ' as i32 as u_char);
                    n = n.wrapping_add(1)
                }
            } else {
                sle = &mut *(*sl).entries.as_mut_ptr().offset(i as isize) as *mut status_line_entry;
                expanded = format_expand_time(ft, (*ov).string);
                if force == 0
                    && !(*sle).expanded.is_null()
                    && strcmp(expanded, (*sle).expanded) == 0 as libc::c_int
                {
                    free(expanded as *mut libc::c_void);
                } else {
                    changed = 1 as libc::c_int;
                    n = 0 as libc::c_int as u_int;
                    while n < width {
                        screen_write_putc(&mut ctx, &mut gc, ' ' as i32 as u_char);
                        n = n.wrapping_add(1)
                    }
                    screen_write_cursormove(
                        &mut ctx,
                        0 as libc::c_int,
                        i as libc::c_int,
                        0 as libc::c_int,
                    );
                    status_free_ranges(&mut (*sle).ranges);
                    format_draw(&mut ctx, &mut gc, width, expanded, &mut (*sle).ranges);
                    free((*sle).expanded as *mut libc::c_void);
                    (*sle).expanded = expanded
                }
            }
            i = i.wrapping_add(1)
        }
    }
    screen_write_stop(&mut ctx);
    /* Free the format tree. */
    format_free(ft);
    /* Return if the status line has changed. */
    log_debug(
        b"%s exit: force=%d, changed=%d\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"status_redraw\x00")).as_ptr(),
        force,
        changed,
    );
    return (force != 0 || changed != 0) as libc::c_int;
}
/* Set a status line message. */
#[no_mangle]
pub unsafe extern "C" fn status_message_set(
    mut c: *mut client,
    mut delay: libc::c_int,
    mut ignore_styles: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ap: ::std::ffi::VaListImpl;
    status_message_clear(c);
    status_push_screen(c);
    ap = args.clone();
    (*c).message_ignore_styles = ignore_styles;
    xvasprintf(&mut (*c).message_string, fmt, ap.as_va_list());
    server_add_message(
        b"%s message: %s\x00" as *const u8 as *const libc::c_char,
        (*c).name,
        (*c).message_string,
    );
    /*
     * With delay -1, the display-time option is used; zero means wait for
     * key press; more than zero is the actual delay time in milliseconds.
     */
    if delay == -(1 as libc::c_int) {
        delay = options_get_number(
            (*(*c).session).options,
            b"display-time\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int
    }
    if delay > 0 as libc::c_int {
        tv.tv_sec = (delay / 1000 as libc::c_int) as __time_t;
        tv.tv_usec = (delay % 1000 as libc::c_int) as libc::c_long * 1000 as libc::c_long;
        if event_initialized(&mut (*c).message_timer) != 0 {
            event_del(&mut (*c).message_timer);
        }
        event_set(
            &mut (*c).message_timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                status_message_callback
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            c as *mut libc::c_void,
        );
        event_add(&mut (*c).message_timer, &mut tv);
    }
    (*c).tty.flags |= 0x1 as libc::c_int | 0x2 as libc::c_int;
    (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
}
/* Clear status line message. */
#[no_mangle]
pub unsafe extern "C" fn status_message_clear(mut c: *mut client) {
    if (*c).message_string.is_null() {
        return;
    } /* was frozen and may have changed */
    free((*c).message_string as *mut libc::c_void);
    (*c).message_string = 0 as *mut libc::c_char;
    if (*c).prompt_string.is_null() {
        (*c).tty.flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int)
    }
    (*c).flags |= (0x8 as libc::c_int
        | 0x10 as libc::c_int
        | 0x1000000 as libc::c_int
        | 0x400 as libc::c_int
        | 0x2000000 as libc::c_int
        | 0x20000000 as libc::c_int) as libc::c_ulong;
    status_pop_screen(c);
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
/* Clear status line message after timer expires. */
unsafe extern "C" fn status_message_callback(
    mut fd: libc::c_int,
    mut event: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut c: *mut client = data as *mut client;
    status_message_clear(c);
}
/* Draw client message on status line of present else on last line. */
#[no_mangle]
pub unsafe extern "C" fn status_message_redraw(mut c: *mut client) -> libc::c_int {
    let mut sl: *mut status_line = &mut (*c).status;
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
    let mut s: *mut session = (*c).session;
    let mut old_screen: screen = screen {
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
    let mut len: size_t = 0;
    let mut lines: u_int = 0;
    let mut offset: u_int = 0;
    let mut gc: grid_cell = grid_cell {
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
    };
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    if (*c).tty.sx == 0 as libc::c_int as libc::c_uint
        || (*c).tty.sy == 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    memcpy(
        &mut old_screen as *mut screen as *mut libc::c_void,
        (*sl).active as *const libc::c_void,
        ::std::mem::size_of::<screen>() as libc::c_ulong,
    );
    lines = status_line_size(c);
    if lines <= 1 as libc::c_int as libc::c_uint {
        lines = 1 as libc::c_int as u_int
    }
    screen_init((*sl).active, (*c).tty.sx, lines, 0 as libc::c_int as u_int);
    len = screen_write_strlen(
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*c).message_string,
    );
    if len > (*c).tty.sx as libc::c_ulong {
        len = (*c).tty.sx as size_t
    }
    ft = format_create_defaults(
        0 as *mut cmdq_item,
        c,
        0 as *mut session,
        0 as *mut winlink,
        0 as *mut window_pane,
    );
    style_apply(
        &mut gc,
        (*s).options,
        b"message-style\x00" as *const u8 as *const libc::c_char,
        ft,
    );
    format_free(ft);
    screen_write_start(&mut ctx, (*sl).active);
    screen_write_fast_copy(
        &mut ctx,
        &mut (*sl).screen,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
        (*c).tty.sx,
        lines.wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int,
        lines.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
        0 as libc::c_int,
    );
    offset = 0 as libc::c_int as u_int;
    while offset < (*c).tty.sx {
        screen_write_putc(&mut ctx, &mut gc, ' ' as i32 as u_char);
        offset = offset.wrapping_add(1)
    }
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int,
        lines.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
        0 as libc::c_int,
    );
    if (*c).message_ignore_styles != 0 {
        screen_write_nputs(
            &mut ctx as *mut screen_write_ctx,
            len as ssize_t,
            &mut gc as *mut grid_cell,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*c).message_string,
        );
    } else {
        format_draw(
            &mut ctx,
            &mut gc,
            (*c).tty.sx,
            (*c).message_string,
            0 as *mut style_ranges,
        );
    }
    screen_write_stop(&mut ctx);
    if grid_compare((*(*sl).active).grid, old_screen.grid) == 0 as libc::c_int {
        screen_free(&mut old_screen);
        return 0 as libc::c_int;
    }
    screen_free(&mut old_screen);
    return 1 as libc::c_int;
}
/* Enable status line prompt. */
#[no_mangle]
pub unsafe extern "C" fn status_prompt_set(
    mut c: *mut client,
    mut fs: *mut cmd_find_state,
    mut msg: *const libc::c_char,
    mut input: *const libc::c_char,
    mut inputcb: prompt_input_cb,
    mut freecb: prompt_free_cb,
    mut data: *mut libc::c_void,
    mut flags: libc::c_int,
) {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if !fs.is_null() {
        ft = format_create_from_state(0 as *mut cmdq_item, c, fs)
    } else {
        ft = format_create_defaults(
            0 as *mut cmdq_item,
            c,
            0 as *mut session,
            0 as *mut winlink,
            0 as *mut window_pane,
        )
    }
    if input.is_null() {
        input = b"\x00" as *const u8 as *const libc::c_char
    }
    if flags & 0x8 as libc::c_int != 0 {
        tmp = xstrdup(input)
    } else {
        tmp = format_expand_time(ft, input)
    }
    status_message_clear(c);
    status_prompt_clear(c);
    status_push_screen(c);
    (*c).prompt_string = format_expand_time(ft, msg);
    (*c).prompt_buffer = utf8_fromcstr(tmp);
    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
    (*c).prompt_inputcb = inputcb;
    (*c).prompt_freecb = freecb;
    (*c).prompt_data = data;
    (*c).prompt_hindex = 0 as libc::c_int as u_int;
    (*c).prompt_flags = flags;
    (*c).prompt_mode = PROMPT_ENTRY;
    if !flags & 0x4 as libc::c_int != 0 {
        (*c).tty.flags |= 0x1 as libc::c_int | 0x2 as libc::c_int
    }
    (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
    if flags & 0x4 as libc::c_int != 0 && *tmp as libc::c_int != '\u{0}' as i32 {
        xasprintf(
            &mut cp as *mut *mut libc::c_char,
            b"=%s\x00" as *const u8 as *const libc::c_char,
            tmp,
        );
        (*c).prompt_inputcb.expect("non-null function pointer")(
            c,
            (*c).prompt_data,
            cp,
            0 as libc::c_int,
        );
        free(cp as *mut libc::c_void);
    }
    free(tmp as *mut libc::c_void);
    format_free(ft);
}
/* Remove status line prompt. */
#[no_mangle]
pub unsafe extern "C" fn status_prompt_clear(mut c: *mut client) {
    if (*c).prompt_string.is_null() {
        return;
    } /* was frozen and may have changed */
    if (*c).prompt_freecb.is_some() && !(*c).prompt_data.is_null() {
        (*c).prompt_freecb.expect("non-null function pointer")((*c).prompt_data);
    }
    free((*c).prompt_string as *mut libc::c_void);
    (*c).prompt_string = 0 as *mut libc::c_char;
    free((*c).prompt_buffer as *mut libc::c_void);
    (*c).prompt_buffer = 0 as *mut utf8_data;
    free((*c).prompt_saved as *mut libc::c_void);
    (*c).prompt_saved = 0 as *mut utf8_data;
    (*c).tty.flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int);
    (*c).flags |= (0x8 as libc::c_int
        | 0x10 as libc::c_int
        | 0x1000000 as libc::c_int
        | 0x400 as libc::c_int
        | 0x2000000 as libc::c_int
        | 0x20000000 as libc::c_int) as libc::c_ulong;
    status_pop_screen(c);
}
/* Update status line prompt with a new prompt string. */
#[no_mangle]
pub unsafe extern "C" fn status_prompt_update(
    mut c: *mut client,
    mut msg: *const libc::c_char,
    mut input: *const libc::c_char,
) {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    ft = format_create(c, 0 as *mut cmdq_item, 0 as libc::c_int, 0 as libc::c_int);
    format_defaults(
        ft,
        c,
        0 as *mut session,
        0 as *mut winlink,
        0 as *mut window_pane,
    );
    tmp = format_expand_time(ft, input);
    free((*c).prompt_string as *mut libc::c_void);
    (*c).prompt_string = format_expand_time(ft, msg);
    free((*c).prompt_buffer as *mut libc::c_void);
    (*c).prompt_buffer = utf8_fromcstr(tmp);
    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
    (*c).prompt_hindex = 0 as libc::c_int as u_int;
    (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
    free(tmp as *mut libc::c_void);
    format_free(ft);
}
/* Draw client prompt on status line of present else on last line. */
#[no_mangle]
pub unsafe extern "C" fn status_prompt_redraw(mut c: *mut client) -> libc::c_int {
    let mut sl: *mut status_line = &mut (*c).status;
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
    let mut s: *mut session = (*c).session;
    let mut old_screen: screen = screen {
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
    let mut i: u_int = 0;
    let mut lines: u_int = 0;
    let mut offset: u_int = 0;
    let mut left: u_int = 0;
    let mut start: u_int = 0;
    let mut width: u_int = 0;
    let mut pcursor: u_int = 0;
    let mut pwidth: u_int = 0;
    let mut gc: grid_cell = grid_cell {
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
    };
    let mut cursorgc: grid_cell = grid_cell {
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
    };
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    if (*c).tty.sx == 0 as libc::c_int as libc::c_uint
        || (*c).tty.sy == 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    memcpy(
        &mut old_screen as *mut screen as *mut libc::c_void,
        (*sl).active as *const libc::c_void,
        ::std::mem::size_of::<screen>() as libc::c_ulong,
    );
    lines = status_line_size(c);
    if lines <= 1 as libc::c_int as libc::c_uint {
        lines = 1 as libc::c_int as u_int
    }
    screen_init((*sl).active, (*c).tty.sx, lines, 0 as libc::c_int as u_int);
    ft = format_create_defaults(
        0 as *mut cmdq_item,
        c,
        0 as *mut session,
        0 as *mut winlink,
        0 as *mut window_pane,
    );
    if (*c).prompt_mode as libc::c_uint == PROMPT_COMMAND as libc::c_int as libc::c_uint {
        style_apply(
            &mut gc,
            (*s).options,
            b"message-command-style\x00" as *const u8 as *const libc::c_char,
            ft,
        );
    } else {
        style_apply(
            &mut gc,
            (*s).options,
            b"message-style\x00" as *const u8 as *const libc::c_char,
            ft,
        );
    }
    format_free(ft);
    memcpy(
        &mut cursorgc as *mut grid_cell as *mut libc::c_void,
        &mut gc as *mut grid_cell as *const libc::c_void,
        ::std::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    cursorgc.attr = (cursorgc.attr as libc::c_int ^ 0x10 as libc::c_int) as u_short;
    start = screen_write_strlen(
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*c).prompt_string,
    ) as u_int;
    if start > (*c).tty.sx {
        start = (*c).tty.sx
    }
    screen_write_start(&mut ctx, (*sl).active);
    screen_write_fast_copy(
        &mut ctx,
        &mut (*sl).screen,
        0 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
        (*c).tty.sx,
        lines.wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int,
        lines.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
        0 as libc::c_int,
    );
    offset = 0 as libc::c_int as u_int;
    while offset < (*c).tty.sx {
        screen_write_putc(&mut ctx, &mut gc, ' ' as i32 as u_char);
        offset = offset.wrapping_add(1)
    }
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int,
        lines.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
        0 as libc::c_int,
    );
    screen_write_nputs(
        &mut ctx as *mut screen_write_ctx,
        start as ssize_t,
        &mut gc as *mut grid_cell,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*c).prompt_string,
    );
    screen_write_cursormove(
        &mut ctx,
        start as libc::c_int,
        lines.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
        0 as libc::c_int,
    );
    left = (*c).tty.sx.wrapping_sub(start);
    if !(left == 0 as libc::c_int as libc::c_uint) {
        pcursor = utf8_strwidth((*c).prompt_buffer, (*c).prompt_index as ssize_t);
        pwidth = utf8_strwidth((*c).prompt_buffer, -(1 as libc::c_int) as ssize_t);
        if pcursor >= left {
            /*
             * The cursor would be outside the screen so start drawing
             * with it on the right.
             */
            offset = pcursor
                .wrapping_sub(left)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            pwidth = left
        } else {
            offset = 0 as libc::c_int as u_int
        }
        if pwidth > left {
            pwidth = left
        }
        width = 0 as libc::c_int as u_int;
        i = 0 as libc::c_int as u_int;
        while (*(*c).prompt_buffer.offset(i as isize)).size as libc::c_int != 0 as libc::c_int {
            if width < offset {
                width = (width as libc::c_uint)
                    .wrapping_add((*(*c).prompt_buffer.offset(i as isize)).width as libc::c_uint)
                    as u_int as u_int
            } else {
                if width >= offset.wrapping_add(pwidth) {
                    break;
                }
                width = (width as libc::c_uint)
                    .wrapping_add((*(*c).prompt_buffer.offset(i as isize)).width as libc::c_uint)
                    as u_int as u_int;
                if width > offset.wrapping_add(pwidth) {
                    break;
                }
                if i as libc::c_ulong != (*c).prompt_index {
                    utf8_copy(&mut gc.data, &mut *(*c).prompt_buffer.offset(i as isize));
                    screen_write_cell(&mut ctx, &mut gc);
                } else {
                    utf8_copy(
                        &mut cursorgc.data,
                        &mut *(*c).prompt_buffer.offset(i as isize),
                    );
                    screen_write_cell(&mut ctx, &mut cursorgc);
                }
            }
            i = i.wrapping_add(1)
        }
        if (*(*sl).active).cx < (*(*(*sl).active).grid).sx
            && (*c).prompt_index >= i as libc::c_ulong
        {
            screen_write_putc(&mut ctx, &mut cursorgc, ' ' as i32 as u_char);
        }
    }
    screen_write_stop(&mut ctx);
    if grid_compare((*(*sl).active).grid, old_screen.grid) == 0 as libc::c_int {
        screen_free(&mut old_screen);
        return 0 as libc::c_int;
    }
    screen_free(&mut old_screen);
    return 1 as libc::c_int;
}
/* Is this a separator? */
unsafe extern "C" fn status_prompt_in_list(
    mut ws: *const libc::c_char,
    mut ud: *const utf8_data,
) -> libc::c_int {
    if (*ud).size as libc::c_int != 1 as libc::c_int
        || (*ud).width as libc::c_int != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return (strchr(ws, *(*ud).data.as_ptr() as libc::c_int)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
}
/* Is this a space? */
unsafe extern "C" fn status_prompt_space(mut ud: *const utf8_data) -> libc::c_int {
    if (*ud).size as libc::c_int != 1 as libc::c_int
        || (*ud).width as libc::c_int != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return (*(*ud).data.as_ptr() as libc::c_int == ' ' as i32) as libc::c_int;
}
/*
 * Translate key from emacs to vi. Return 0 to drop key, 1 to process the key
 * as an emacs key; return 2 to append to the buffer.
 */
unsafe extern "C" fn status_prompt_translate_key(
    mut c: *mut client,
    mut key: key_code,
    mut new_key: *mut key_code,
) -> libc::c_int {
    if (*c).prompt_mode as libc::c_uint == PROMPT_ENTRY as libc::c_int as libc::c_uint {
        's_63: {
            let mut current_block_4: u64;
            match key {
                7 => {
                    /* C-g */
                    current_block_4 = 509724311878556287;
                }
                8 => {
                    current_block_4 = 509724311878556287;
                }
                9 => {
                    current_block_4 = 13850883808908694939;
                }
                21 => {
                    current_block_4 = 17517844533035367829;
                }
                23 => {
                    current_block_4 = 18408218966702913984;
                }
                3 | 10 | 13 | 68719476888 | 68719476902 | 68719476909 | 68719476904
                | 68719476903 | 68719476910 | 68719476911 | 68719476908 => {
                    current_block_4 = 1836777044871886902;
                }
                27 => {
                    /* Escape */
                    (*c).prompt_mode = PROMPT_COMMAND;
                    (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                    return 0 as libc::c_int;
                }
                _ => {
                    break 's_63;
                }
            }
            match current_block_4 {
                509724311878556287 =>
                /* C-h */
                {
                    current_block_4 = 13850883808908694939;
                }
                _ => {}
            }
            match current_block_4 {
                13850883808908694939 =>
                /* Tab */
                {
                    current_block_4 = 17517844533035367829;
                }
                _ => {}
            }
            match current_block_4 {
                17517844533035367829 =>
                /* C-u */
                {
                    current_block_4 = 18408218966702913984;
                }
                _ => {}
            }
            match current_block_4 {
                18408218966702913984 =>
                    /* C-w */
                    {}
                _ => {}
            }
            /* C-c */
            *new_key = key; /* C-u */
            return 1 as libc::c_int;
        }
        *new_key = key;
        return 2 as libc::c_int;
    }
    match key {
        65 | 73 | 67 | 115 | 97 => {
            (*c).prompt_mode = PROMPT_ENTRY;
            (*c).flags |= 0x10 as libc::c_int as libc::c_ulong
        }
        83 => {
            (*c).prompt_mode = PROMPT_ENTRY;
            (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
            *new_key = '\u{15}' as i32 as key_code;
            return 1 as libc::c_int;
        }
        105 | 27 => {
            /* Escape */
            (*c).prompt_mode = PROMPT_ENTRY; /* C-k */
            (*c).flags |= 0x10 as libc::c_int as libc::c_ulong; /* C-y */
            return 0 as libc::c_int;
        }
        _ => {}
    } /* C-c */
    's_308: {
        match key {
            65 | 36 => {
                *new_key = KEYC_END as libc::c_ulong as key_code;
                return 1 as libc::c_int;
            }
            73 | 48 | 94 => {
                *new_key = KEYC_HOME as libc::c_ulong as key_code;
                return 1 as libc::c_int;
            }
            67 | 68 => {
                *new_key = '\u{b}' as i32 as key_code;
                return 1 as libc::c_int;
            }
            68719476888 | 88 => {
                *new_key = KEYC_BSPACE as libc::c_ulong as key_code;
                return 1 as libc::c_int;
            }
            98 | 66 => {
                *new_key = 'b' as i32 as libc::c_ulonglong | 0x100000000000 as libc::c_ulonglong;
                return 1 as libc::c_int;
            }
            100 => {
                *new_key = '\u{15}' as i32 as key_code;
                return 1 as libc::c_int;
            }
            101 | 69 | 119 | 87 => {
                *new_key = 'f' as i32 as libc::c_ulonglong | 0x100000000000 as libc::c_ulonglong;
                return 1 as libc::c_int;
            }
            112 => {
                *new_key = '\u{19}' as i32 as key_code;
                return 1 as libc::c_int;
            }
            113 => {
                *new_key = '\u{3}' as i32 as key_code;
                return 1 as libc::c_int;
            }
            115 | 68719476902 | 120 => {
                *new_key = KEYC_DC as libc::c_ulong as key_code;
                return 1 as libc::c_int;
            }
            68719476909 | 106 => {
                *new_key = KEYC_DOWN as libc::c_ulong as key_code;
                return 1 as libc::c_int;
            }
            68719476910 | 104 => {
                *new_key = KEYC_LEFT as libc::c_ulong as key_code;
                return 1 as libc::c_int;
            }
            97 | 68719476911 | 108 => {
                *new_key = KEYC_RIGHT as libc::c_ulong as key_code;
                return 1 as libc::c_int;
            }
            68719476908 | 107 => {
                *new_key = KEYC_UP as libc::c_ulong as key_code;
                return 1 as libc::c_int;
            }
            3 => {}
            8 | 10 | 13 => {}
            _ => {
                break 's_308;
            }
        }
        /* C-h */
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* Paste into prompt. */
unsafe extern "C" fn status_prompt_paste(mut c: *mut client) -> libc::c_int {
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut bufdata: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    let mut bufsize: size_t = 0;
    let mut i: u_int = 0;
    let mut ud: *mut utf8_data = 0 as *mut utf8_data;
    let mut udp: *mut utf8_data = 0 as *mut utf8_data;
    let mut more: utf8_state = UTF8_MORE;
    size = utf8_strlen((*c).prompt_buffer);
    if !(*c).prompt_saved.is_null() {
        ud = (*c).prompt_saved;
        n = utf8_strlen((*c).prompt_saved)
    } else {
        pb = paste_get_top(0 as *mut *const libc::c_char);
        if pb.is_null() {
            return 0 as libc::c_int;
        }
        bufdata = paste_buffer_data(pb, &mut bufsize);
        ud = xreallocarray(
            0 as *mut libc::c_void,
            bufsize.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
        ) as *mut utf8_data;
        udp = ud;
        i = 0 as libc::c_int as u_int;
        while i as libc::c_ulong != bufsize {
            /* nothing */
            more = utf8_open(udp, *bufdata.offset(i as isize) as u_char);
            if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
                loop {
                    i = i.wrapping_add(1);
                    if !(i as libc::c_ulong != bufsize
                        && more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint)
                    {
                        break;
                    }
                    more = utf8_append(udp, *bufdata.offset(i as isize) as u_char)
                }
                if more as libc::c_uint == UTF8_DONE as libc::c_int as libc::c_uint {
                    udp = udp.offset(1);
                    continue;
                } else {
                    i = (i as libc::c_uint).wrapping_sub((*udp).have as libc::c_uint) as u_int
                        as u_int
                }
            }
            if *bufdata.offset(i as isize) as libc::c_int <= 31 as libc::c_int
                || *bufdata.offset(i as isize) as libc::c_int >= 127 as libc::c_int
            {
                break;
            }
            utf8_set(udp, *bufdata.offset(i as isize) as u_char);
            udp = udp.offset(1);
            i = i.wrapping_add(1)
        }
        (*udp).size = 0 as libc::c_int as u_char;
        n = udp.wrapping_offset_from(ud) as libc::c_long as size_t
    }
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    (*c).prompt_buffer = xreallocarray(
        (*c).prompt_buffer as *mut libc::c_void,
        size.wrapping_add(n)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
    ) as *mut utf8_data;
    if (*c).prompt_index == size {
        memcpy(
            (*c).prompt_buffer.offset((*c).prompt_index as isize) as *mut libc::c_void,
            ud as *const libc::c_void,
            n.wrapping_mul(::std::mem::size_of::<utf8_data>() as libc::c_ulong),
        );
        (*c).prompt_index =
            ((*c).prompt_index as libc::c_ulong).wrapping_add(n) as size_t as size_t;
        (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size = 0 as libc::c_int as u_char
    } else {
        memmove(
            (*c).prompt_buffer
                .offset((*c).prompt_index as isize)
                .offset(n as isize) as *mut libc::c_void,
            (*c).prompt_buffer.offset((*c).prompt_index as isize) as *const libc::c_void,
            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub((*c).prompt_index)
                .wrapping_mul(::std::mem::size_of::<utf8_data>() as libc::c_ulong),
        );
        memcpy(
            (*c).prompt_buffer.offset((*c).prompt_index as isize) as *mut libc::c_void,
            ud as *const libc::c_void,
            n.wrapping_mul(::std::mem::size_of::<utf8_data>() as libc::c_ulong),
        );
        (*c).prompt_index = ((*c).prompt_index as libc::c_ulong).wrapping_add(n) as size_t as size_t
    }
    if ud != (*c).prompt_saved {
        free(ud as *mut libc::c_void);
    }
    return 1 as libc::c_int;
}
/* Finish completion. */
unsafe extern "C" fn status_prompt_replace_complete(
    mut c: *mut client,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let mut word: [libc::c_char; 64] = [0; 64];
    let mut allocated: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    let mut off: size_t = 0;
    let mut idx: size_t = 0;
    let mut used: size_t = 0;
    let mut first: *mut utf8_data = 0 as *mut utf8_data;
    let mut last: *mut utf8_data = 0 as *mut utf8_data;
    let mut ud: *mut utf8_data = 0 as *mut utf8_data;
    /* Work out where the cursor currently is. */
    idx = (*c).prompt_index;
    if idx != 0 as libc::c_int as libc::c_ulong {
        idx = idx.wrapping_sub(1)
    }
    size = utf8_strlen((*c).prompt_buffer);
    /* Find the word we are in. */
    first = &mut *(*c).prompt_buffer.offset(idx as isize) as *mut utf8_data;
    while first > (*c).prompt_buffer && status_prompt_space(first) == 0 {
        first = first.offset(-1)
    }
    while (*first).size as libc::c_int != 0 as libc::c_int && status_prompt_space(first) != 0 {
        first = first.offset(1)
    }
    last = &mut *(*c).prompt_buffer.offset(idx as isize) as *mut utf8_data;
    while (*last).size as libc::c_int != 0 as libc::c_int && status_prompt_space(last) == 0 {
        last = last.offset(1)
    }
    while last > (*c).prompt_buffer && status_prompt_space(last) != 0 {
        last = last.offset(-1)
    }
    if (*last).size as libc::c_int != 0 as libc::c_int {
        last = last.offset(1)
    }
    if last < first {
        return 0 as libc::c_int;
    }
    if s.is_null() {
        used = 0 as libc::c_int as size_t;
        ud = first;
        while ud < last {
            if used.wrapping_add((*ud).size as libc::c_ulong)
                >= ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
            {
                break;
            }
            memcpy(
                word.as_mut_ptr().offset(used as isize) as *mut libc::c_void,
                (*ud).data.as_mut_ptr() as *const libc::c_void,
                (*ud).size as libc::c_ulong,
            );
            used = (used as libc::c_ulong).wrapping_add((*ud).size as libc::c_ulong) as size_t
                as size_t;
            ud = ud.offset(1)
        }
        if ud != last {
            return 0 as libc::c_int;
        }
        word[used as usize] = '\u{0}' as i32 as libc::c_char
    }
    /* Try to complete it. */
    if s.is_null() {
        allocated = status_prompt_complete(
            c,
            word.as_mut_ptr(),
            first.wrapping_offset_from((*c).prompt_buffer) as libc::c_long as u_int,
        );
        if allocated.is_null() {
            return 0 as libc::c_int;
        }
        s = allocated
    }
    /* Trim out word. */
    n = size
        .wrapping_sub(
            last.wrapping_offset_from((*c).prompt_buffer) as libc::c_long as libc::c_ulong,
        )
        .wrapping_add(1 as libc::c_int as libc::c_ulong); /* with \0 */
    memmove(
        first as *mut libc::c_void,
        last as *const libc::c_void,
        n.wrapping_mul(::std::mem::size_of::<utf8_data>() as libc::c_ulong),
    );
    size = (size as libc::c_ulong)
        .wrapping_sub(last.wrapping_offset_from(first) as libc::c_long as libc::c_ulong)
        as size_t as size_t;
    /* Insert the new word. */
    size = (size as libc::c_ulong).wrapping_add(strlen(s)) as size_t as size_t;
    off = first.wrapping_offset_from((*c).prompt_buffer) as libc::c_long as size_t;
    (*c).prompt_buffer = xreallocarray(
        (*c).prompt_buffer as *mut libc::c_void,
        size.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
    ) as *mut utf8_data;
    first = (*c).prompt_buffer.offset(off as isize);
    memmove(
        first.offset(strlen(s) as isize) as *mut libc::c_void,
        first as *const libc::c_void,
        n.wrapping_mul(::std::mem::size_of::<utf8_data>() as libc::c_ulong),
    );
    idx = 0 as libc::c_int as size_t;
    while idx < strlen(s) {
        utf8_set(
            &mut *first.offset(idx as isize),
            *s.offset(idx as isize) as u_char,
        );
        idx = idx.wrapping_add(1)
    }
    (*c).prompt_index = (first.wrapping_offset_from((*c).prompt_buffer) as libc::c_long
        as libc::c_ulong)
        .wrapping_add(strlen(s));
    free(allocated as *mut libc::c_void);
    return 1 as libc::c_int;
}
/* Handle keys in prompt. */
#[no_mangle]
pub unsafe extern "C" fn status_prompt_key(mut c: *mut client, mut key: key_code) -> libc::c_int {
    let mut current_block: u64;
    let mut oo: *mut options = (*(*c).session).options;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: libc::c_char = '=' as i32 as libc::c_char;
    let mut histstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ws: *const libc::c_char = 0 as *const libc::c_char;
    let mut keystring: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: size_t = 0;
    let mut idx: size_t = 0;
    let mut tmp: utf8_data = utf8_data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut keys: libc::c_int = 0;
    if (*c).prompt_flags & 0x10 as libc::c_int != 0 {
        keystring = key_string_lookup_key(key, 0 as libc::c_int);
        (*c).prompt_inputcb.expect("non-null function pointer")(
            c,
            (*c).prompt_data,
            keystring,
            1 as libc::c_int,
        );
        status_prompt_clear(c);
        return 0 as libc::c_int;
    }
    size = utf8_strlen((*c).prompt_buffer);
    if (*c).prompt_flags & 0x2 as libc::c_int != 0 {
        if key >= '0' as i32 as libc::c_ulonglong && key <= '9' as i32 as libc::c_ulonglong {
            current_block = 14272474440432613372;
        } else {
            s = utf8_tocstr((*c).prompt_buffer);
            (*c).prompt_inputcb.expect("non-null function pointer")(
                c,
                (*c).prompt_data,
                s,
                1 as libc::c_int,
            );
            status_prompt_clear(c);
            free(s as *mut libc::c_void);
            return 1 as libc::c_int;
        }
    } else {
        key &= !(0xff000000000000 as libc::c_ulonglong);
        keys = options_get_number(
            (*(*c).session).options,
            b"status-keys\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        if keys == 1 as libc::c_int {
            match status_prompt_translate_key(c, key, &mut key) {
                1 => {
                    current_block = 10489599262272764497;
                }
                2 => {
                    current_block = 14272474440432613372;
                }
                _ => return 0 as libc::c_int,
            }
        } else {
            current_block = 10489599262272764497;
        }
        match current_block {
            14272474440432613372 => {}
            _ => {
                match key {
                    68719476910 | 2 => {
                        current_block = 8595499525207474077;
                        match current_block {
                            13716905325739901580 =>
                            /* C-r */
                            {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                /* C-t */
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                /* C-n */
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                /* C-p */
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                /* Find a non-separator. */
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                /* Find the separator at the beginning of the word. */
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    /* Go back to the word. */
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                /* Find a word. */
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                /* Find the separator at the end of the word. */
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                /* Back up to the end-of-word like vi. */
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                /* C-w */
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                /* Find a non-separator. */
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                /* Find the separator at the beginning of the word. */
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    /* Go back to the word. */
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 =>
                            /* C-k */
                            {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                /* C-u */
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 =>
                            /* C-d */
                            {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 =>
                            /* C-h */
                            {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 =>
                            /* C-e */
                            {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 =>
                            /* C-a */
                            {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 =>
                            /* C-f */
                            {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 =>
                            /* C-b */
                            {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 =>
                            /* C-s */
                            {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 =>
                            /* Tab */
                            {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 =>
                            /* C-y */
                            {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 =>
                            /* Escape */
                            {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 =>
                                    /* C-c */
                                    /* C-g */
                                    {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    68719476911 | 6 => {
                        current_block = 12087305117219508006;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    68719476903 | 1 => {
                        current_block = 9649627425948382823;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    68719476904 | 5 => {
                        current_block = 15329334428063834850;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    9 => {
                        current_block = 9949027150079833961;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    68719476888 | 8 => {
                        current_block = 4303351861420576715;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    68719476902 | 4 => {
                        current_block = 11026770671687539224;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    21 => {
                        current_block = 7209921709130718133;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    11 => {
                        current_block = 10675534896202779322;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    23 => {
                        current_block = 7307066281430299153;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    17592186044518 | 35253091565743 => {
                        current_block = 11057756935608885628;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    17592186044514 | 35253091565742 => {
                        current_block = 120937800574074383;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    68719476908 | 16 => {
                        current_block = 11667326925293579197;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    68719476909 | 14 => {
                        current_block = 11812164207322118286;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    25 => {
                        current_block = 8555501604137689390;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    20 => {
                        current_block = 6699258362909208477;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    13 | 10 => {
                        current_block = 14996222313512956680;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    27 => {
                        current_block = 8200438068734264111;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    3 | 7 => {
                        current_block = 17252179160093326629;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    18 => {
                        current_block = 13716905325739901580;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    19 => {
                        current_block = 16592328677945139976;
                        match current_block {
                            13716905325739901580 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '-' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            14996222313512956680 => {
                                s = utf8_tocstr((*c).prompt_buffer);
                                if *s as libc::c_int != '\u{0}' as i32 {
                                    status_prompt_add_history(s);
                                }
                                if (*c).prompt_inputcb.expect("non-null function pointer")(
                                    c,
                                    (*c).prompt_data,
                                    s,
                                    1 as libc::c_int,
                                ) == 0 as libc::c_int
                                {
                                    status_prompt_clear(c);
                                }
                                free(s as *mut libc::c_void);
                                current_block = 16835199615365683821;
                            }
                            6699258362909208477 => {
                                idx = (*c).prompt_index;
                                if idx < size {
                                    idx = idx.wrapping_add(1)
                                }
                                if idx >= 2 as libc::c_int as libc::c_ulong {
                                    utf8_copy(
                                        &mut tmp,
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                    );
                                    utf8_copy(
                                        &mut *(*c).prompt_buffer.offset(
                                            idx.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                as isize,
                                        ),
                                        &mut tmp,
                                    );
                                    (*c).prompt_index = idx;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            11812164207322118286 => {
                                histstr = status_prompt_down_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            11667326925293579197 => {
                                histstr = status_prompt_up_history(&mut (*c).prompt_hindex);
                                if histstr.is_null() {
                                    current_block = 16835199615365683821;
                                } else {
                                    free((*c).prompt_buffer as *mut libc::c_void);
                                    (*c).prompt_buffer = utf8_fromcstr(histstr);
                                    (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
                                    current_block = 368077705793071303;
                                }
                            }
                            120937800574074383 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    idx = (*c).prompt_index;
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    break;
                                }
                                current_block = 368077705793071303;
                            }
                            11057756935608885628 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while (*c).prompt_index != size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    idx = (*c).prompt_index;
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0
                                    {
                                        break;
                                    }
                                }
                                if options_get_number(
                                    oo,
                                    b"status-keys\x00" as *const u8 as *const libc::c_char,
                                ) == 1 as libc::c_int as libc::c_longlong
                                    && (*c).prompt_index != 0 as libc::c_int as libc::c_ulong
                                {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                }
                                current_block = 368077705793071303;
                            }
                            7307066281430299153 => {
                                ws = options_get_string(
                                    oo,
                                    b"word-separators\x00" as *const u8 as *const libc::c_char,
                                );
                                idx = (*c).prompt_index;
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                while idx != 0 as libc::c_int as libc::c_ulong {
                                    idx = idx.wrapping_sub(1);
                                    if !(status_prompt_in_list(
                                        ws,
                                        &mut *(*c).prompt_buffer.offset(idx as isize),
                                    ) != 0)
                                    {
                                        continue;
                                    }
                                    idx = idx.wrapping_add(1);
                                    break;
                                }
                                free((*c).prompt_saved as *mut libc::c_void);
                                (*c).prompt_saved = xcalloc(
                                    ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    (*c).prompt_index
                                        .wrapping_sub(idx)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                )
                                    as *mut utf8_data;
                                memcpy(
                                    (*c).prompt_saved as *mut libc::c_void,
                                    (*c).prompt_buffer.offset(idx as isize) as *const libc::c_void,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                memmove(
                                    (*c).prompt_buffer.offset(idx as isize) as *mut libc::c_void,
                                    (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                        as *const libc::c_void,
                                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub((*c).prompt_index)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                        ),
                                );
                                memset(
                                    (*c).prompt_buffer
                                        .offset(size as isize)
                                        .offset(-((*c).prompt_index.wrapping_sub(idx) as isize))
                                        as *mut libc::c_void,
                                    '\u{0}' as i32,
                                    (*c).prompt_index.wrapping_sub(idx).wrapping_mul(
                                        ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
                                    ),
                                );
                                (*c).prompt_index = idx;
                                current_block = 368077705793071303;
                            }
                            10675534896202779322 => {
                                if (*c).prompt_index < size {
                                    (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                                        0 as libc::c_int as u_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            7209921709130718133 => {
                                (*(*c).prompt_buffer.offset(0 as libc::c_int as isize)).size =
                                    0 as libc::c_int as u_char;
                                (*c).prompt_index = 0 as libc::c_int as size_t;
                                current_block = 368077705793071303;
                            }
                            11026770671687539224 => {
                                if (*c).prompt_index != size {
                                    memmove(
                                        (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                            as *mut libc::c_void,
                                        (*c).prompt_buffer
                                            .offset((*c).prompt_index as isize)
                                            .offset(1 as libc::c_int as isize)
                                            as *const libc::c_void,
                                        size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub((*c).prompt_index)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<utf8_data>() as libc::c_ulong
                                            ),
                                    );
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            4303351861420576715 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    if (*c).prompt_index == size {
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                        (*(*c).prompt_buffer.offset((*c).prompt_index as isize))
                                            .size = 0 as libc::c_int as u_char
                                    } else {
                                        memmove(
                                            (*c).prompt_buffer
                                                .offset((*c).prompt_index as isize)
                                                .offset(-(1 as libc::c_int as isize))
                                                as *mut libc::c_void,
                                            (*c).prompt_buffer.offset((*c).prompt_index as isize)
                                                as *const libc::c_void,
                                            size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub((*c).prompt_index)
                                                .wrapping_mul(::std::mem::size_of::<utf8_data>()
                                                    as libc::c_ulong),
                                        );
                                        (*c).prompt_index = (*c).prompt_index.wrapping_sub(1)
                                    }
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            15329334428063834850 => {
                                if (*c).prompt_index != size {
                                    (*c).prompt_index = size;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9649627425948382823 => {
                                if (*c).prompt_index != 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = 0 as libc::c_int as size_t;
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            12087305117219508006 => {
                                if (*c).prompt_index < size {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8595499525207474077 => {
                                if (*c).prompt_index > 0 as libc::c_int as libc::c_ulong {
                                    (*c).prompt_index = (*c).prompt_index.wrapping_sub(1);
                                    current_block = 16835199615365683821;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            16592328677945139976 => {
                                if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
                                    prefix = '+' as i32 as libc::c_char;
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            9949027150079833961 => {
                                if status_prompt_replace_complete(c, 0 as *const libc::c_char) != 0
                                {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8555501604137689390 => {
                                if status_prompt_paste(c) != 0 {
                                    current_block = 368077705793071303;
                                } else {
                                    current_block = 16835199615365683821;
                                }
                            }
                            8200438068734264111 => {
                                current_block = 17252179160093326629;
                            }
                            _ => {}
                        }
                        match current_block {
                            368077705793071303 => {}
                            _ => {
                                match current_block {
                                    17252179160093326629 => {
                                        if (*c).prompt_inputcb.expect("non-null function pointer")(
                                            c,
                                            (*c).prompt_data,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int,
                                        ) == 0 as libc::c_int
                                        {
                                            status_prompt_clear(c);
                                        }
                                    }
                                    _ => {}
                                }
                                (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
                                return 0 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        current_block = 14272474440432613372;
                    }
                }
            }
        }
    }
    match current_block {
        14272474440432613372 => {
            if key <= 0x1f as libc::c_int as libc::c_ulonglong
                || key >= 0x1000000000 as libc::c_ulonglong
            {
                return 0 as libc::c_int;
            }
            if key <= 0x7f as libc::c_int as libc::c_ulonglong {
                utf8_set(&mut tmp, key as u_char);
            } else {
                utf8_to_data(key as utf8_char, &mut tmp);
            }
            (*c).prompt_buffer = xreallocarray(
                (*c).prompt_buffer as *mut libc::c_void,
                size.wrapping_add(2 as libc::c_int as libc::c_ulong),
                ::std::mem::size_of::<utf8_data>() as libc::c_ulong,
            ) as *mut utf8_data;
            if (*c).prompt_index == size {
                utf8_copy(
                    &mut *(*c).prompt_buffer.offset((*c).prompt_index as isize),
                    &mut tmp,
                );
                (*c).prompt_index = (*c).prompt_index.wrapping_add(1);
                (*(*c).prompt_buffer.offset((*c).prompt_index as isize)).size =
                    0 as libc::c_int as u_char
            } else {
                memmove(
                    (*c).prompt_buffer
                        .offset((*c).prompt_index as isize)
                        .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                    (*c).prompt_buffer.offset((*c).prompt_index as isize) as *const libc::c_void,
                    size.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub((*c).prompt_index)
                        .wrapping_mul(::std::mem::size_of::<utf8_data>() as libc::c_ulong),
                );
                utf8_copy(
                    &mut *(*c).prompt_buffer.offset((*c).prompt_index as isize),
                    &mut tmp,
                );
                (*c).prompt_index = (*c).prompt_index.wrapping_add(1)
            }
            if (*c).prompt_flags & 0x1 as libc::c_int != 0 {
                s = utf8_tocstr((*c).prompt_buffer);
                if strlen(s) != 1 as libc::c_int as libc::c_ulong {
                    status_prompt_clear(c);
                } else if (*c).prompt_inputcb.expect("non-null function pointer")(
                    c,
                    (*c).prompt_data,
                    s,
                    1 as libc::c_int,
                ) == 0 as libc::c_int
                {
                    status_prompt_clear(c);
                }
                free(s as *mut libc::c_void);
            }
        }
        _ => {}
    }
    (*c).flags |= 0x10 as libc::c_int as libc::c_ulong;
    if (*c).prompt_flags & 0x4 as libc::c_int != 0 {
        s = utf8_tocstr((*c).prompt_buffer);
        xasprintf(
            &mut cp as *mut *mut libc::c_char,
            b"%c%s\x00" as *const u8 as *const libc::c_char,
            prefix as libc::c_int,
            s,
        );
        (*c).prompt_inputcb.expect("non-null function pointer")(
            c,
            (*c).prompt_data,
            cp,
            0 as libc::c_int,
        );
        free(cp as *mut libc::c_void);
        free(s as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
/* Get previous line from the history. */
unsafe extern "C" fn status_prompt_up_history(mut idx: *mut u_int) -> *const libc::c_char {
    /*
     * History runs from 0 to size - 1. Index is from 0 to size. Zero is
     * empty.
     */
    if status_prompt_hsize == 0 as libc::c_int as libc::c_uint || *idx == status_prompt_hsize {
        return 0 as *const libc::c_char;
    }
    *idx = (*idx).wrapping_add(1);
    return *status_prompt_hlist.offset(status_prompt_hsize.wrapping_sub(*idx) as isize);
}
/* Get next line from the history. */
unsafe extern "C" fn status_prompt_down_history(mut idx: *mut u_int) -> *const libc::c_char {
    if status_prompt_hsize == 0 as libc::c_int as libc::c_uint
        || *idx == 0 as libc::c_int as libc::c_uint
    {
        return b"\x00" as *const u8 as *const libc::c_char;
    }
    *idx = (*idx).wrapping_sub(1);
    if *idx == 0 as libc::c_int as libc::c_uint {
        return b"\x00" as *const u8 as *const libc::c_char;
    }
    return *status_prompt_hlist.offset(status_prompt_hsize.wrapping_sub(*idx) as isize);
}
/* Add line to the history. */
unsafe extern "C" fn status_prompt_add_history(mut line: *const libc::c_char) {
    let mut size: size_t = 0;
    if status_prompt_hsize > 0 as libc::c_int as libc::c_uint
        && strcmp(
            *status_prompt_hlist.offset(
                status_prompt_hsize.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ),
            line,
        ) == 0 as libc::c_int
    {
        return;
    }
    if status_prompt_hsize == 100 as libc::c_int as libc::c_uint {
        free(*status_prompt_hlist.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        size = ((100 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong);
        memmove(
            &mut *status_prompt_hlist.offset(0 as libc::c_int as isize) as *mut *mut libc::c_char
                as *mut libc::c_void,
            &mut *status_prompt_hlist.offset(1 as libc::c_int as isize) as *mut *mut libc::c_char
                as *const libc::c_void,
            size,
        );
        let ref mut fresh0 = *status_prompt_hlist
            .offset(status_prompt_hsize.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        *fresh0 = xstrdup(line);
        return;
    }
    status_prompt_hlist = xreallocarray(
        status_prompt_hlist as *mut libc::c_void,
        status_prompt_hsize.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    let fresh1 = status_prompt_hsize;
    status_prompt_hsize = status_prompt_hsize.wrapping_add(1);
    let ref mut fresh2 = *status_prompt_hlist.offset(fresh1 as isize);
    *fresh2 = xstrdup(line);
}
/* Build completion list. */
unsafe extern "C" fn status_prompt_complete_list(
    mut size: *mut u_int,
    mut s: *const libc::c_char,
    mut at_start: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut layout: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmdent: *mut *const cmd_entry = 0 as *mut *const cmd_entry;
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    let mut slen: size_t = strlen(s);
    let mut valuelen: size_t = 0;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    let mut layouts: [*const libc::c_char; 6] = [
        b"even-horizontal\x00" as *const u8 as *const libc::c_char,
        b"even-vertical\x00" as *const u8 as *const libc::c_char,
        b"main-horizontal\x00" as *const u8 as *const libc::c_char,
        b"main-vertical\x00" as *const u8 as *const libc::c_char,
        b"tiled\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    *size = 0 as libc::c_int as u_int;
    cmdent = cmd_table.as_mut_ptr();
    while !(*cmdent).is_null() {
        if strncmp((**cmdent).name, s, slen) == 0 as libc::c_int {
            list = xreallocarray(
                list as *mut libc::c_void,
                (*size).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            let fresh3 = *size;
            *size = (*size).wrapping_add(1);
            let ref mut fresh4 = *list.offset(fresh3 as isize);
            *fresh4 = xstrdup((**cmdent).name)
        }
        if !(**cmdent).alias.is_null() && strncmp((**cmdent).alias, s, slen) == 0 as libc::c_int {
            list = xreallocarray(
                list as *mut libc::c_void,
                (*size).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            let fresh5 = *size;
            *size = (*size).wrapping_add(1);
            let ref mut fresh6 = *list.offset(fresh5 as isize);
            *fresh6 = xstrdup((**cmdent).alias)
        }
        cmdent = cmdent.offset(1)
    }
    o = options_get_only(
        global_options,
        b"command-alias\x00" as *const u8 as *const libc::c_char,
    );
    if !o.is_null() {
        a = options_array_first(o);
        while !a.is_null() {
            value = (*options_array_item_value(a)).string;
            cp = strchr(value, '=' as i32);
            if !cp.is_null() {
                valuelen = cp.wrapping_offset_from(value) as libc::c_long as size_t;
                if !(slen > valuelen || strncmp(value, s, slen) != 0 as libc::c_int) {
                    list = xreallocarray(
                        list as *mut libc::c_void,
                        (*size).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ) as *mut *mut libc::c_char;
                    let fresh7 = *size;
                    *size = (*size).wrapping_add(1);
                    let ref mut fresh8 = *list.offset(fresh7 as isize);
                    *fresh8 = xstrndup(value, valuelen)
                }
            }
            a = options_array_next(a)
        }
    }
    if at_start != 0 {
        return list;
    }
    oe = options_table.as_ptr();
    while !(*oe).name.is_null() {
        if strncmp((*oe).name, s, slen) == 0 as libc::c_int {
            list = xreallocarray(
                list as *mut libc::c_void,
                (*size).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            let fresh9 = *size;
            *size = (*size).wrapping_add(1);
            let ref mut fresh10 = *list.offset(fresh9 as isize);
            *fresh10 = xstrdup((*oe).name)
        }
        oe = oe.offset(1)
    }
    layout = layouts.as_mut_ptr();
    while !(*layout).is_null() {
        if strncmp(*layout, s, slen) == 0 as libc::c_int {
            list = xreallocarray(
                list as *mut libc::c_void,
                (*size).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            let fresh11 = *size;
            *size = (*size).wrapping_add(1);
            let ref mut fresh12 = *list.offset(fresh11 as isize);
            *fresh12 = xstrdup(*layout)
        }
        layout = layout.offset(1)
    }
    return list;
}
/* Find longest prefix. */
unsafe extern "C" fn status_prompt_complete_prefix(
    mut list: *mut *mut libc::c_char,
    mut size: u_int,
) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    let mut j: size_t = 0;
    if list.is_null() || size == 0 as libc::c_int as libc::c_uint {
        return 0 as *mut libc::c_char;
    }
    out = xstrdup(*list.offset(0 as libc::c_int as isize));
    i = 1 as libc::c_int as u_int;
    while i < size {
        j = strlen(*list.offset(i as isize));
        if j > strlen(out) {
            j = strlen(out)
        }
        while j > 0 as libc::c_int as libc::c_ulong {
            if *out.offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                != *(*list.offset(i as isize))
                    .offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
            {
                *out.offset(j.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
                    '\u{0}' as i32 as libc::c_char
            }
            j = j.wrapping_sub(1)
        }
        i = i.wrapping_add(1)
    }
    return out;
}
/* Complete word menu callback. */
unsafe extern "C" fn status_prompt_menu_callback(
    mut menu: *mut menu,
    mut idx: u_int,
    mut key: key_code,
    mut data: *mut libc::c_void,
) {
    let mut spm: *mut status_prompt_menu = data as *mut status_prompt_menu;
    let mut c: *mut client = (*spm).c;
    let mut i: u_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if key != 0xff000000000 as libc::c_ulonglong {
        idx = (idx as libc::c_uint).wrapping_add((*spm).start) as u_int as u_int;
        if (*spm).flag as libc::c_int == '\u{0}' as i32 {
            s = xstrdup(*(*spm).list.offset(idx as isize))
        } else {
            xasprintf(
                &mut s as *mut *mut libc::c_char,
                b"-%c%s\x00" as *const u8 as *const libc::c_char,
                (*spm).flag as libc::c_int,
                *(*spm).list.offset(idx as isize),
            );
        }
        if (*c).prompt_flags & 0x20 as libc::c_int != 0 {
            free((*c).prompt_buffer as *mut libc::c_void);
            (*c).prompt_buffer = utf8_fromcstr(s);
            (*c).prompt_index = utf8_strlen((*c).prompt_buffer);
            (*c).flags |= 0x10 as libc::c_int as libc::c_ulong
        } else if status_prompt_replace_complete(c, s) != 0 {
            (*c).flags |= 0x10 as libc::c_int as libc::c_ulong
        }
        free(s as *mut libc::c_void);
    }
    i = 0 as libc::c_int as u_int;
    while i < (*spm).size {
        free(*(*spm).list.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free((*spm).list as *mut libc::c_void);
}
/* Show complete word menu. */
unsafe extern "C" fn status_prompt_complete_list_menu(
    mut c: *mut client,
    mut list: *mut *mut libc::c_char,
    mut size: u_int,
    mut offset: u_int,
    mut flag: libc::c_char,
) -> libc::c_int {
    let mut menu: *mut menu = 0 as *mut menu;
    let mut item: menu_item = menu_item {
        name: 0 as *const libc::c_char,
        key: 0,
        command: 0 as *const libc::c_char,
    };
    let mut spm: *mut status_prompt_menu = 0 as *mut status_prompt_menu;
    let mut lines: u_int = status_line_size(c);
    let mut height: u_int = 0;
    let mut i: u_int = 0;
    let mut py: u_int = 0;
    if size <= 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*c).tty.sy.wrapping_sub(lines) < 3 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    spm = xmalloc(::std::mem::size_of::<status_prompt_menu>() as libc::c_ulong)
        as *mut status_prompt_menu;
    (*spm).c = c;
    (*spm).size = size;
    (*spm).list = list;
    (*spm).flag = flag;
    height = (*c)
        .tty
        .sy
        .wrapping_sub(lines)
        .wrapping_sub(2 as libc::c_int as libc::c_uint);
    if height > 10 as libc::c_int as libc::c_uint {
        height = 10 as libc::c_int as u_int
    }
    if height > size {
        height = size
    }
    (*spm).start = size.wrapping_sub(height);
    menu = menu_create(b"\x00" as *const u8 as *const libc::c_char);
    i = (*spm).start;
    while i < size {
        item.name = *list.offset(i as isize);
        item.key =
            ('0' as i32 as libc::c_uint).wrapping_add(i.wrapping_sub((*spm).start)) as key_code;
        item.command = 0 as *const libc::c_char;
        menu_add_item(
            menu,
            &mut item,
            0 as *mut cmdq_item,
            0 as *mut client,
            0 as *mut cmd_find_state,
        );
        i = i.wrapping_add(1)
    }
    if options_get_number(
        (*(*c).session).options,
        b"status-position\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int as libc::c_longlong
    {
        py = lines
    } else {
        py = (*c)
            .tty
            .sy
            .wrapping_sub(3 as libc::c_int as libc::c_uint)
            .wrapping_sub(height)
    }
    offset =
        (offset as libc::c_uint).wrapping_add(utf8_cstrwidth((*c).prompt_string)) as u_int as u_int;
    if offset > 2 as libc::c_int as libc::c_uint {
        offset = (offset as libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint) as u_int
            as u_int
    } else {
        offset = 0 as libc::c_int as u_int
    }
    if menu_display(
        menu,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0 as *mut cmdq_item,
        offset,
        py,
        c,
        0 as *mut cmd_find_state,
        Some(
            status_prompt_menu_callback
                as unsafe extern "C" fn(
                    _: *mut menu,
                    _: u_int,
                    _: key_code,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        spm as *mut libc::c_void,
    ) != 0 as libc::c_int
    {
        menu_free(menu);
        free(spm as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* Show complete word menu. */
unsafe extern "C" fn status_prompt_complete_window_menu(
    mut c: *mut client,
    mut s: *mut session,
    mut word: *const libc::c_char,
    mut offset: u_int,
    mut flag: libc::c_char,
) -> *mut libc::c_char {
    let mut menu: *mut menu = 0 as *mut menu;
    let mut item: menu_item = menu_item {
        name: 0 as *const libc::c_char,
        key: 0,
        command: 0 as *const libc::c_char,
    };
    let mut spm: *mut status_prompt_menu = 0 as *mut status_prompt_menu;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lines: u_int = status_line_size(c);
    let mut height: u_int = 0;
    let mut py: u_int = 0;
    let mut size: u_int = 0 as libc::c_int as u_int;
    if (*c).tty.sy.wrapping_sub(lines) < 3 as libc::c_int as libc::c_uint {
        return 0 as *mut libc::c_char;
    }
    spm = xmalloc(::std::mem::size_of::<status_prompt_menu>() as libc::c_ulong)
        as *mut status_prompt_menu;
    (*spm).c = c;
    (*spm).flag = flag;
    height = (*c)
        .tty
        .sy
        .wrapping_sub(lines)
        .wrapping_sub(2 as libc::c_int as libc::c_uint);
    if height > 10 as libc::c_int as libc::c_uint {
        height = 10 as libc::c_int as u_int
    }
    (*spm).start = 0 as libc::c_int as u_int;
    menu = menu_create(b"\x00" as *const u8 as *const libc::c_char);
    let mut current_block_26: u64;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if !word.is_null() && *word as libc::c_int != '\u{0}' as i32 {
            xasprintf(
                &mut tmp as *mut *mut libc::c_char,
                b"%d\x00" as *const u8 as *const libc::c_char,
                (*wl).idx,
            );
            if strncmp(tmp, word, strlen(word)) != 0 as libc::c_int {
                free(tmp as *mut libc::c_void);
                current_block_26 = 13586036798005543211;
            } else {
                free(tmp as *mut libc::c_void);
                current_block_26 = 13797916685926291137;
            }
        } else {
            current_block_26 = 13797916685926291137;
        }
        match current_block_26 {
            13797916685926291137 => {
                list = xreallocarray(
                    list as *mut libc::c_void,
                    size.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ) as *mut *mut libc::c_char;
                if (*c).prompt_flags & 0x20 as libc::c_int != 0 {
                    xasprintf(
                        &mut tmp as *mut *mut libc::c_char,
                        b"%d (%s)\x00" as *const u8 as *const libc::c_char,
                        (*wl).idx,
                        (*(*wl).window).name,
                    );
                    let fresh13 = size;
                    size = size.wrapping_add(1);
                    xasprintf(
                        &mut *list.offset(fresh13 as isize) as *mut *mut libc::c_char,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        (*wl).idx,
                    );
                } else {
                    xasprintf(
                        &mut tmp as *mut *mut libc::c_char,
                        b"%s:%d (%s)\x00" as *const u8 as *const libc::c_char,
                        (*s).name,
                        (*wl).idx,
                        (*(*wl).window).name,
                    );
                    let fresh14 = size;
                    size = size.wrapping_add(1);
                    xasprintf(
                        &mut *list.offset(fresh14 as isize) as *mut *mut libc::c_char,
                        b"%s:%d\x00" as *const u8 as *const libc::c_char,
                        (*s).name,
                        (*wl).idx,
                    );
                }
                item.name = tmp;
                item.key = ('0' as i32 as libc::c_uint)
                    .wrapping_add(size)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as key_code;
                item.command = 0 as *const libc::c_char;
                menu_add_item(
                    menu,
                    &mut item,
                    0 as *mut cmdq_item,
                    0 as *mut client,
                    0 as *mut cmd_find_state,
                );
                free(tmp as *mut libc::c_void);
                if size == height {
                    break;
                }
            }
            _ => {}
        }
        wl = winlinks_RB_NEXT(wl)
    }
    if size == 0 as libc::c_int as libc::c_uint {
        menu_free(menu);
        return 0 as *mut libc::c_char;
    }
    if size == 1 as libc::c_int as libc::c_uint {
        menu_free(menu);
        if flag as libc::c_int != '\u{0}' as i32 {
            xasprintf(
                &mut tmp as *mut *mut libc::c_char,
                b"-%c%s\x00" as *const u8 as *const libc::c_char,
                flag as libc::c_int,
                *list.offset(0 as libc::c_int as isize),
            );
            free(*list.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        } else {
            tmp = *list.offset(0 as libc::c_int as isize)
        }
        free(list as *mut libc::c_void);
        return tmp;
    }
    if height > size {
        height = size
    }
    (*spm).size = size;
    (*spm).list = list;
    if options_get_number(
        (*(*c).session).options,
        b"status-position\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int as libc::c_longlong
    {
        py = lines
    } else {
        py = (*c)
            .tty
            .sy
            .wrapping_sub(3 as libc::c_int as libc::c_uint)
            .wrapping_sub(height)
    }
    offset =
        (offset as libc::c_uint).wrapping_add(utf8_cstrwidth((*c).prompt_string)) as u_int as u_int;
    if offset > 2 as libc::c_int as libc::c_uint {
        offset = (offset as libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint) as u_int
            as u_int
    } else {
        offset = 0 as libc::c_int as u_int
    }
    if menu_display(
        menu,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0 as *mut cmdq_item,
        offset,
        py,
        c,
        0 as *mut cmd_find_state,
        Some(
            status_prompt_menu_callback
                as unsafe extern "C" fn(
                    _: *mut menu,
                    _: u_int,
                    _: key_code,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        spm as *mut libc::c_void,
    ) != 0 as libc::c_int
    {
        menu_free(menu);
        free(spm as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    return 0 as *mut libc::c_char;
}
/* Sort complete list. */
unsafe extern "C" fn status_prompt_complete_sort(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut aa: *mut *const libc::c_char = a as *mut *const libc::c_char;
    let mut bb: *mut *const libc::c_char = b as *mut *const libc::c_char;
    return strcmp(*aa, *bb);
}
/* Complete a session. */
unsafe extern "C" fn status_prompt_complete_session(
    mut list: *mut *mut *mut libc::c_char,
    mut size: *mut u_int,
    mut s: *const libc::c_char,
    mut flag: libc::c_char,
) -> *mut libc::c_char {
    let mut loop_0: *mut session = 0 as *mut session;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: [libc::c_char; 11] = [0; 11];
    loop_0 = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !loop_0.is_null() {
        if *s as libc::c_int == '\u{0}' as i32
            || strncmp((*loop_0).name, s, strlen(s)) == 0 as libc::c_int
        {
            *list = xreallocarray(
                *list as *mut libc::c_void,
                (*size).wrapping_add(2 as libc::c_int as libc::c_uint) as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            let fresh15 = *size;
            *size = (*size).wrapping_add(1);
            xasprintf(
                &mut *(*list).offset(fresh15 as isize) as *mut *mut libc::c_char,
                b"%s:\x00" as *const u8 as *const libc::c_char,
                (*loop_0).name,
            );
        } else if *s as libc::c_int == '$' as i32 {
            xsnprintf(
                n.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong,
                b"%u\x00" as *const u8 as *const libc::c_char,
                (*loop_0).id,
            );
            if *s.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                || strncmp(
                    n.as_mut_ptr(),
                    s.offset(1 as libc::c_int as isize),
                    strlen(s).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
            {
                *list = xreallocarray(
                    *list as *mut libc::c_void,
                    (*size).wrapping_add(2 as libc::c_int as libc::c_uint) as size_t,
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ) as *mut *mut libc::c_char;
                let fresh16 = *size;
                *size = (*size).wrapping_add(1);
                xasprintf(
                    &mut *(*list).offset(fresh16 as isize) as *mut *mut libc::c_char,
                    b"$%s:\x00" as *const u8 as *const libc::c_char,
                    n.as_mut_ptr(),
                );
            }
        }
        loop_0 = sessions_RB_NEXT(loop_0)
    }
    out = status_prompt_complete_prefix(*list, *size);
    if !out.is_null() && flag as libc::c_int != '\u{0}' as i32 {
        xasprintf(
            &mut tmp as *mut *mut libc::c_char,
            b"-%c%s\x00" as *const u8 as *const libc::c_char,
            flag as libc::c_int,
            out,
        );
        free(out as *mut libc::c_void);
        out = tmp
    }
    return out;
}
/* Complete word. */
unsafe extern "C" fn status_prompt_complete(
    mut c: *mut client,
    mut word: *const libc::c_char,
    mut offset: u_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut session: *mut session = 0 as *mut session;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut colon: *const libc::c_char = 0 as *const libc::c_char;
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flag: libc::c_char = '\u{0}' as i32 as libc::c_char;
    let mut size: u_int = 0 as libc::c_int as u_int;
    let mut i: u_int = 0;
    if *word as libc::c_int == '\u{0}' as i32
        && (*c).prompt_flags & (0x40 as libc::c_int | 0x20 as libc::c_int) == 0 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    if (*c).prompt_flags & (0x40 as libc::c_int | 0x20 as libc::c_int) == 0 as libc::c_int
        && strncmp(
            word,
            b"-t\x00" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        && strncmp(
            word,
            b"-s\x00" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        list = status_prompt_complete_list(
            &mut size,
            word,
            (offset == 0 as libc::c_int as libc::c_uint) as libc::c_int,
        );
        if size == 0 as libc::c_int as libc::c_uint {
            out = 0 as *mut libc::c_char
        } else if size == 1 as libc::c_int as libc::c_uint {
            xasprintf(
                &mut out as *mut *mut libc::c_char,
                b"%s \x00" as *const u8 as *const libc::c_char,
                *list.offset(0 as libc::c_int as isize),
            );
        } else {
            out = status_prompt_complete_prefix(list, size)
        }
    } else {
        if (*c).prompt_flags & (0x40 as libc::c_int | 0x20 as libc::c_int) != 0 {
            s = word;
            flag = '\u{0}' as i32 as libc::c_char
        } else {
            s = word.offset(2 as libc::c_int as isize);
            flag = *word.offset(1 as libc::c_int as isize);
            offset = (offset as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
                as u_int as u_int
        }
        /* If this is a window completion, open the window menu. */
        if (*c).prompt_flags & 0x20 as libc::c_int != 0 {
            out = status_prompt_complete_window_menu(
                c,
                (*c).session,
                s,
                offset,
                '\u{0}' as i32 as libc::c_char,
            )
        } else {
            colon = strchr(s, ':' as i32);
            /* If there is no colon, complete as a session. */
            if colon.is_null() {
                out = status_prompt_complete_session(&mut list, &mut size, s, flag)
            } else if strchr(colon.offset(1 as libc::c_int as isize), '.' as i32).is_null() {
                if *s as libc::c_int == ':' as i32 {
                    session = (*c).session;
                    current_block = 2891135413264362348;
                } else {
                    copy = xstrdup(s);
                    *strchr(copy, ':' as i32) = '\u{0}' as i32 as libc::c_char;
                    session = session_find(copy);
                    free(copy as *mut libc::c_void);
                    if session.is_null() {
                        current_block = 7402590507484993752;
                    } else {
                        current_block = 2891135413264362348;
                    }
                }
                match current_block {
                    7402590507484993752 => {}
                    _ => {
                        out = status_prompt_complete_window_menu(
                            c,
                            session,
                            colon.offset(1 as libc::c_int as isize),
                            offset,
                            flag,
                        );
                        if out.is_null() {
                            return 0 as *mut libc::c_char;
                        }
                    }
                }
            }
        }
    }
    if size != 0 as libc::c_int as libc::c_uint {
        qsort(
            list as *mut libc::c_void,
            size as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            Some(
                status_prompt_complete_sort
                    as unsafe extern "C" fn(
                        _: *const libc::c_void,
                        _: *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        i = 0 as libc::c_int as u_int;
        while i < size {
            log_debug(
                b"complete %u: %s\x00" as *const u8 as *const libc::c_char,
                i,
                *list.offset(i as isize),
            );
            i = i.wrapping_add(1)
        }
    }
    if !out.is_null() && strcmp(word, out) == 0 as libc::c_int {
        free(out as *mut libc::c_void);
        out = 0 as *mut libc::c_char
    }
    if !out.is_null() || status_prompt_complete_list_menu(c, list, size, offset, flag) == 0 {
        i = 0 as libc::c_int as u_int;
        while i < size {
            free(*list.offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        free(list as *mut libc::c_void);
    }
    return out;
}
/* If there is a colon but no period, find session and show a menu. */
