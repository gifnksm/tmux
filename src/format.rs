use crate::{grid::Cell as GridCell, utf8::Utf8Data};
use ::c2rust_bitfields;
use ::libc;

extern "C" {
    pub type re_dfa_t;
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn regfree(__preg: *mut regex_t);
    #[no_mangle]
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn ctime_r(__timer: *const time_t, __buf: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    #[no_mangle]
    static mut socket_path: *const libc::c_char;
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
    fn evbuffer_new() -> *mut evbuffer;
    #[no_mangle]
    fn evbuffer_free(buf: *mut evbuffer);
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_add(buf: *mut evbuffer, data: *const libc::c_void, datlen: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_add_printf(buf: *mut evbuffer, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t) -> *mut libc::c_uchar;
    #[no_mangle]
    fn evbuffer_readline(buffer: *mut evbuffer) -> *mut libc::c_char;
    #[no_mangle]
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
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
    static mut global_options: *mut crate::options::options;
    #[no_mangle]
    static mut global_s_options: *mut crate::options::options;
    #[no_mangle]
    static mut global_w_options: *mut crate::options::options;
    #[no_mangle]
    static mut global_environ: *mut crate::environ::environ;
    #[no_mangle]
    static mut start_time: timeval;
    #[no_mangle]
    fn cmd_mouse_pane(
        _: *mut mouse_event,
        _: *mut *mut session,
        _: *mut *mut winlink,
    ) -> *mut window_pane;
    #[no_mangle]
    fn cmdq_get_event(_: *mut crate::cmd_queue::cmdq_item) -> *mut key_event;
    #[no_mangle]
    fn options_get_string(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn cmdq_merge_formats(_: *mut crate::cmd_queue::cmdq_item, _: *mut format_tree);
    #[no_mangle]
    fn cmd_mouse_at(
        _: *mut window_pane,
        _: *mut mouse_event,
        _: *mut u_int,
        _: *mut u_int,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn job_free(_: *mut crate::job::job);
    #[no_mangle]
    fn paste_make_sample(_: *mut crate::paste::paste_buffer) -> *mut libc::c_char;
    #[no_mangle]
    fn cmdq_print(_: *mut crate::cmd_queue::cmdq_item, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char);
    #[no_mangle]
    fn format_trim_right(_: *const libc::c_char, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    fn format_trim_left(_: *const libc::c_char, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    fn environ_find(_: *mut crate::environ::environ, _: *const libc::c_char) -> *mut environ_entry;
    #[no_mangle]
    fn options_to_string(
        _: *mut crate::options::options_entry,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn options_parse_get(
        _: *mut crate::options::options,
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: libc::c_int,
    ) -> *mut crate::options::options_entry;
    #[no_mangle]
    fn paste_get_top(_: *mut *const libc::c_char) -> *mut crate::paste::paste_buffer;
    #[no_mangle]
    fn paste_buffer_data(_: *mut crate::paste::paste_buffer, _: *mut size_t)
        -> *const libc::c_char;
    #[no_mangle]
    fn paste_buffer_created(_: *mut crate::paste::paste_buffer) -> time_t;
    #[no_mangle]
    fn paste_buffer_name(_: *mut crate::paste::paste_buffer) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_stringify_argv(_: libc::c_int, _: *mut *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn getversion() -> *const libc::c_char;
    #[no_mangle]
    fn tty_window_offset(
        _: *mut tty,
        _: *mut u_int,
        _: *mut u_int,
        _: *mut u_int,
        _: *mut u_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn tty_get_features(_: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn job_get_data(_: *mut crate::job::job) -> *mut libc::c_void;
    #[no_mangle]
    fn job_get_event(_: *mut crate::job::job) -> *mut bufferevent;
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
    ) -> *mut crate::job::job;
    #[no_mangle]
    fn cmdq_get_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
    #[no_mangle]
    fn cmdq_get_target(_: *mut crate::cmd_queue::cmdq_item) -> *mut cmd_find_state;
    #[no_mangle]
    fn cmdq_get_target_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
    #[no_mangle]
    fn grid_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut crate::grid::Cell);
    #[no_mangle]
    fn grid_line_length(_: *mut grid, _: u_int) -> u_int;
    #[no_mangle]
    fn grid_peek_line(_: *mut grid, _: u_int) -> *const grid_line;
    #[no_mangle]
    fn server_client_get_cwd(_: *mut client, _: *mut session) -> *const libc::c_char;
    #[no_mangle]
    fn server_status_client(_: *mut client);
    #[no_mangle]
    fn server_client_get_key_table(_: *mut client) -> *const libc::c_char;
    #[no_mangle]
    fn server_client_get_flags(_: *mut client) -> *const libc::c_char;
    #[no_mangle]
    fn grid_get_line(_: *mut grid, _: u_int) -> *mut grid_line;
    #[no_mangle]
    static mut marked_pane: cmd_find_state;
    #[no_mangle]
    fn server_check_marked() -> libc::c_int;
    #[no_mangle]
    fn grid_view_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut crate::grid::Cell);
    #[no_mangle]
    fn server_client_unref(_: *mut client);
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static window_copy_mode: window_mode;
    #[no_mangle]
    static window_view_mode: window_mode;
    #[no_mangle]
    fn window_copy_get_line(_: *mut window_pane, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    fn window_copy_get_word(_: *mut window_pane, _: u_int, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    static mut all_window_modes: [*const window_mode; 0];
    #[no_mangle]
    fn window_pane_search(
        _: *mut window_pane,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
    ) -> u_int;
    #[no_mangle]
    fn parse_window_name(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    #[no_mangle]
    fn window_count_panes(_: *mut window) -> u_int;
    #[no_mangle]
    fn layout_dump(_: *mut layout_cell) -> *mut libc::c_char;
    #[no_mangle]
    fn winlinks_RB_MINMAX(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn window_printable_flags(_: *mut winlink) -> *const libc::c_char;
    #[no_mangle]
    fn winlinks_RB_NEXT(_: *mut winlink) -> *mut winlink;
    #[no_mangle]
    fn winlink_count(_: *mut winlinks) -> u_int;
    #[no_mangle]
    fn session_alive(_: *mut session) -> libc::c_int;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    fn utf8_tocstr(_: *mut Utf8Data) -> *mut libc::c_char;
    #[no_mangle]
    fn utf8_cstrhas(_: *const libc::c_char, _: *const Utf8Data) -> libc::c_int;
    #[no_mangle]
    fn session_group_count(_: *mut session_group) -> u_int;
    #[no_mangle]
    fn session_group_attached_count(_: *mut session_group) -> u_int;
    #[no_mangle]
    fn session_group_contains(_: *mut session) -> *mut session_group;
    #[no_mangle]
    fn session_is_linked(_: *mut session, _: *mut window) -> libc::c_int;
    #[no_mangle]
    fn log_get_level() -> libc::c_int;
    #[no_mangle]
    fn utf8_rpadcstr(_: *const libc::c_char, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    fn osdep_get_name(_: libc::c_int, _: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn utf8_padcstr(_: *const libc::c_char, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    fn osdep_get_cwd(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn regsub(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
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
pub type time_t = __time_t;
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
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type va_list = __builtin_va_list;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
    pub exit_type: C2RustUnnamed_30,
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
    pub prompt_mode: C2RustUnnamed_27,
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
    pub formats: Option<unsafe extern "C" fn(_: *mut window_mode_entry, _: *mut format_tree) -> ()>,
}
/* Format entry tree. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_tree {
    pub c: *mut client,
    pub s: *mut session,
    pub wl: *mut winlink,
    pub w: *mut window,
    pub wp: *mut window_pane,
    pub item: *mut crate::cmd_queue::cmdq_item,
    pub client: *mut client,
    pub flags: libc::c_int,
    pub tag: u_int,
    pub m: mouse_event,
    pub tree: format_entry_tree,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_entry_tree {
    pub rbh_root: *mut format_entry,
}
/* Entry in format tree. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_entry {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub time: time_t,
    pub cb: format_cb,
    pub entry: C2RustUnnamed_26,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_26 {
    pub rbe_left: *mut format_entry,
    pub rbe_right: *mut format_entry,
    pub rbe_parent: *mut format_entry,
    pub rbe_color: libc::c_int,
}
pub type format_cb = Option<unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char>;

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
pub type C2RustUnnamed_27 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_27 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_27 = 0;
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
    pub entry: C2RustUnnamed_28,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_28 {
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
    pub entry: C2RustUnnamed_29,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_29 {
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
pub type C2RustUnnamed_30 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_30 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_30 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_30 = 0;

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
    pub entry: C2RustUnnamed_31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_31 {
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
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
    pub le_next: *mut tty_term,
    pub le_prev: *mut *mut tty_term,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_job_tree {
    pub rbh_root: *mut format_job,
}
/* Entry in format job tree. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_job {
    pub client: *mut client,
    pub tag: u_int,
    pub cmd: *const libc::c_char,
    pub expanded: *const libc::c_char,
    pub last: time_t,
    pub out: *mut libc::c_char,
    pub updated: libc::c_int,
    pub job: *mut crate::job::job,
    pub status: libc::c_int,
    pub entry: C2RustUnnamed_33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub rbe_left: *mut format_job,
    pub rbe_right: *mut format_job,
    pub rbe_parent: *mut format_job,
    pub rbe_color: libc::c_int,
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
    pub entry: C2RustUnnamed_34,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_34 {
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
    pub entry: C2RustUnnamed_35,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_35 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct session_group {
    pub name: *const libc::c_char,
    pub sessions: C2RustUnnamed_37,
    pub entry: C2RustUnnamed_36,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_36 {
    pub rbe_left: *mut session_group,
    pub rbe_right: *mut session_group,
    pub rbe_parent: *mut session_group,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_37 {
    pub tqh_first: *mut session,
    pub tqh_last: *mut *mut session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sessions {
    pub rbh_root: *mut session,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
/* Format modifier. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_modifier {
    pub modifier: [libc::c_char; 3],
    pub size: u_int,
    pub argv: *mut *mut libc::c_char,
    pub argc: libc::c_int,
}
/* Format expand state. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_expand_state {
    pub ft: *mut format_tree,
    pub loop_0: u_int,
    pub time: time_t,
    pub flags: libc::c_int,
}
pub const LESS_THAN_EQUAL: C2RustUnnamed_38 = 10;
pub const LESS_THAN: C2RustUnnamed_38 = 9;
pub const GREATER_THAN_EQUAL: C2RustUnnamed_38 = 8;
pub const GREATER_THAN: C2RustUnnamed_38 = 7;
pub const NOT_EQUAL: C2RustUnnamed_38 = 6;
pub const EQUAL: C2RustUnnamed_38 = 5;
pub const MODULUS: C2RustUnnamed_38 = 4;
pub const DIVIDE: C2RustUnnamed_38 = 3;
pub const MULTIPLY: C2RustUnnamed_38 = 2;
pub const SUBTRACT: C2RustUnnamed_38 = 1;
pub const ADD: C2RustUnnamed_38 = 0;
pub type C2RustUnnamed_38 = libc::c_uint;
pub type job_free_cb = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut crate::job::job) -> ()>;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut crate::job::job) -> ()>;
/* Format job tree. */
static mut format_job_event: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: 0 as *const event_callback as *mut event_callback,
            tqe_prev: 0 as *const *mut event_callback as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: 0 as *const libc::c_void as *mut libc::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: 0 as *const event as *mut event,
            tqe_prev: 0 as *const *mut event as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: 0 as *const event_base as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: 0 as *const event as *mut event,
                le_prev: 0 as *const *mut event as *mut *mut event,
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
};
static mut format_jobs: format_job_tree = {
    let mut init = format_job_tree {
        rbh_root: 0 as *const format_job as *mut format_job,
    };
    init
};
unsafe extern "C" fn format_job_tree_RB_REMOVE_COLOR(
    mut head: *mut format_job_tree,
    mut parent: *mut format_job,
    mut elm: *mut format_job,
) {
    let mut tmp: *mut format_job = 0 as *mut format_job;
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
                    let mut oleft: *mut format_job = 0 as *mut format_job;
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
                    let mut oright: *mut format_job = 0 as *mut format_job;
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
unsafe extern "C" fn format_job_tree_RB_FIND(
    mut head: *mut format_job_tree,
    mut elm: *mut format_job,
) -> *mut format_job {
    let mut tmp: *mut format_job = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = format_job_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut format_job;
}
unsafe extern "C" fn format_job_tree_RB_INSERT(
    mut head: *mut format_job_tree,
    mut elm: *mut format_job,
) -> *mut format_job {
    let mut tmp: *mut format_job = 0 as *mut format_job;
    let mut parent: *mut format_job = 0 as *mut format_job;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = format_job_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut format_job;
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
    format_job_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut format_job;
}
unsafe extern "C" fn format_job_tree_RB_MINMAX(
    mut head: *mut format_job_tree,
    mut val: libc::c_int,
) -> *mut format_job {
    let mut tmp: *mut format_job = (*head).rbh_root;
    let mut parent: *mut format_job = 0 as *mut format_job;
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
unsafe extern "C" fn format_job_tree_RB_NEXT(mut elm: *mut format_job) -> *mut format_job {
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
unsafe extern "C" fn format_job_tree_RB_REMOVE(
    mut head: *mut format_job_tree,
    mut elm: *mut format_job,
) -> *mut format_job {
    let mut current_block: u64;
    let mut child: *mut format_job = 0 as *mut format_job;
    let mut parent: *mut format_job = 0 as *mut format_job;
    let mut old: *mut format_job = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut format_job = 0 as *mut format_job;
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
        current_block = 8522321847195001863;
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
        format_job_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn format_job_tree_RB_INSERT_COLOR(
    mut head: *mut format_job_tree,
    mut elm: *mut format_job,
) {
    let mut parent: *mut format_job = 0 as *mut format_job;
    let mut gparent: *mut format_job = 0 as *mut format_job;
    let mut tmp: *mut format_job = 0 as *mut format_job;
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
/* Format job tree comparison function. */
unsafe extern "C" fn format_job_cmp(
    mut fj1: *mut format_job,
    mut fj2: *mut format_job,
) -> libc::c_int {
    if (*fj1).tag < (*fj2).tag {
        return -(1 as libc::c_int);
    }
    if (*fj1).tag > (*fj2).tag {
        return 1 as libc::c_int;
    }
    return strcmp((*fj1).cmd, (*fj2).cmd);
}
unsafe extern "C" fn format_entry_tree_RB_INSERT_COLOR(
    mut head: *mut format_entry_tree,
    mut elm: *mut format_entry,
) {
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    let mut gparent: *mut format_entry = 0 as *mut format_entry;
    let mut tmp: *mut format_entry = 0 as *mut format_entry;
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
unsafe extern "C" fn format_entry_tree_RB_REMOVE(
    mut head: *mut format_entry_tree,
    mut elm: *mut format_entry,
) -> *mut format_entry {
    let mut current_block: u64;
    let mut child: *mut format_entry = 0 as *mut format_entry;
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    let mut old: *mut format_entry = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut format_entry = 0 as *mut format_entry;
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
        current_block = 14923674370861153555;
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
        format_entry_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn format_entry_tree_RB_REMOVE_COLOR(
    mut head: *mut format_entry_tree,
    mut parent: *mut format_entry,
    mut elm: *mut format_entry,
) {
    let mut tmp: *mut format_entry = 0 as *mut format_entry;
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
                    let mut oleft: *mut format_entry = 0 as *mut format_entry;
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
                    let mut oright: *mut format_entry = 0 as *mut format_entry;
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
unsafe extern "C" fn format_entry_tree_RB_FIND(
    mut head: *mut format_entry_tree,
    mut elm: *mut format_entry,
) -> *mut format_entry {
    let mut tmp: *mut format_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = format_entry_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut format_entry;
}
unsafe extern "C" fn format_entry_tree_RB_INSERT(
    mut head: *mut format_entry_tree,
    mut elm: *mut format_entry,
) -> *mut format_entry {
    let mut tmp: *mut format_entry = 0 as *mut format_entry;
    let mut parent: *mut format_entry = 0 as *mut format_entry;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = format_entry_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut format_entry;
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
    format_entry_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut format_entry;
}
unsafe extern "C" fn format_entry_tree_RB_NEXT(mut elm: *mut format_entry) -> *mut format_entry {
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
unsafe extern "C" fn format_entry_tree_RB_MINMAX(
    mut head: *mut format_entry_tree,
    mut val: libc::c_int,
) -> *mut format_entry {
    let mut tmp: *mut format_entry = (*head).rbh_root;
    let mut parent: *mut format_entry = 0 as *mut format_entry;
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
/* Format entry tree comparison function. */
unsafe extern "C" fn format_entry_cmp(
    mut fe1: *mut format_entry,
    mut fe2: *mut format_entry,
) -> libc::c_int {
    return strcmp((*fe1).key, (*fe2).key);
}
/* Single-character uppercase aliases. */
static mut format_upper: [*const libc::c_char; 26] = [
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"pane_id\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    b"window_flags\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    b"host\x00" as *const u8 as *const libc::c_char,
    b"window_index\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"pane_index\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"session_name\x00" as *const u8 as *const libc::c_char,
    b"pane_title\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"window_name\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
];
/* Single-character lowercase aliases. */
static mut format_lower: [*const libc::c_char; 26] = [
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"host_short\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
];
/* Is logging enabled? */
#[inline]
unsafe extern "C" fn format_logging(mut ft: *mut format_tree) -> libc::c_int {
    return (log_get_level() != 0 as libc::c_int || (*ft).flags & 0x8 as libc::c_int != 0)
        as libc::c_int;
}
/* Log a message if verbose. */
unsafe extern "C" fn format_log1(
    mut es: *mut format_expand_state,
    mut from: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ft: *mut format_tree = (*es).ft;
    let mut ap: ::std::ffi::VaListImpl;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut spaces: [libc::c_char; 11] =
        unsafe { *::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"          \x00") };
    if format_logging(ft) == 0 {
        return;
    }
    ap = args.clone();
    xvasprintf(&mut s, fmt, ap.as_va_list());
    log_debug(b"%s: %s\x00" as *const u8 as *const libc::c_char, from, s);
    if !(*ft).item.is_null() && (*ft).flags & 0x8 as libc::c_int != 0 {
        cmdq_print(
            (*ft).item,
            b"#%.*s%s\x00" as *const u8 as *const libc::c_char,
            (*es).loop_0,
            spaces.as_ptr(),
            s,
        );
    }
    free(s as *mut libc::c_void);
}
/* Copy expand state. */
unsafe extern "C" fn format_copy_state(
    mut to: *mut format_expand_state,
    mut from: *mut format_expand_state,
    mut flags: libc::c_int,
) {
    (*to).ft = (*from).ft;
    (*to).loop_0 = (*from).loop_0;
    (*to).time = (*from).time;
    (*to).flags = (*from).flags | flags;
}
/* Format job update callback. */
unsafe extern "C" fn format_job_update(mut job: *mut crate::job::job) {
    let mut fj: *mut format_job = job_get_data(job) as *mut format_job;
    let mut evb: *mut evbuffer = (*job_get_event(job)).input;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: time_t = 0;
    loop {
        next = evbuffer_readline(evb);
        if next.is_null() {
            break;
        }
        free(line as *mut libc::c_void);
        line = next
    }
    if line.is_null() {
        return;
    }
    (*fj).updated = 1 as libc::c_int;
    free((*fj).out as *mut libc::c_void);
    (*fj).out = line;
    log_debug(
        b"%s: %p %s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"format_job_update\x00"))
            .as_ptr(),
        fj,
        (*fj).cmd,
        (*fj).out,
    );
    t = time(0 as *mut time_t);
    if (*fj).status != 0 && (*fj).last != t {
        if !(*fj).client.is_null() {
            server_status_client((*fj).client);
        }
        (*fj).last = t
    };
}
/* Format job complete callback. */
unsafe extern "C" fn format_job_complete(mut job: *mut crate::job::job) {
    let mut fj: *mut format_job = job_get_data(job) as *mut format_job;
    let mut evb: *mut evbuffer = (*job_get_event(job)).input;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    (*fj).job = 0 as *mut crate::job::job;
    buf = 0 as *mut libc::c_char;
    line = evbuffer_readline(evb);
    if line.is_null() {
        len = evbuffer_get_length(evb);
        buf = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
        if len != 0 as libc::c_int as libc::c_ulong {
            memcpy(
                buf as *mut libc::c_void,
                evbuffer_pullup(evb, -(1 as libc::c_int) as ssize_t) as *const libc::c_void,
                len,
            );
        }
        *buf.offset(len as isize) = '\u{0}' as i32 as libc::c_char
    } else {
        buf = line
    }
    log_debug(
        b"%s: %p %s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"format_job_complete\x00"))
            .as_ptr(),
        fj,
        (*fj).cmd,
        buf,
    );
    if *buf as libc::c_int != '\u{0}' as i32 || (*fj).updated == 0 {
        free((*fj).out as *mut libc::c_void);
        (*fj).out = buf
    } else {
        free(buf as *mut libc::c_void);
    }
    if (*fj).status != 0 {
        if !(*fj).client.is_null() {
            server_status_client((*fj).client);
        }
        (*fj).status = 0 as libc::c_int
    };
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2011 Nicholas Marriott <nicholas.marriott@gmail.com>
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
 * Build a list of key-value pairs and use them to expand #{key} entries in a
 * string.
 */
/* Find a job. */
unsafe extern "C" fn format_job_get(
    mut es: *mut format_expand_state,
    mut cmd: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ft: *mut format_tree = (*es).ft;
    let mut jobs: *mut format_job_tree = 0 as *mut format_job_tree;
    let mut fj0: format_job = format_job {
        client: 0 as *mut client,
        tag: 0,
        cmd: 0 as *const libc::c_char,
        expanded: 0 as *const libc::c_char,
        last: 0,
        out: 0 as *mut libc::c_char,
        updated: 0,
        job: 0 as *mut crate::job::job,
        status: 0,
        entry: C2RustUnnamed_33 {
            rbe_left: 0 as *mut format_job,
            rbe_right: 0 as *mut format_job,
            rbe_parent: 0 as *mut format_job,
            rbe_color: 0,
        },
    };
    let mut fj: *mut format_job = 0 as *mut format_job;
    let mut t: time_t = 0;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut force: libc::c_int = 0;
    let mut next: format_expand_state = format_expand_state {
        ft: 0 as *mut format_tree,
        loop_0: 0,
        time: 0,
        flags: 0,
    };
    if (*ft).client.is_null() {
        jobs = &mut format_jobs
    } else if !(*(*ft).client).jobs.is_null() {
        jobs = (*(*ft).client).jobs
    } else {
        (*(*ft).client).jobs = xmalloc(::std::mem::size_of::<format_job_tree>() as libc::c_ulong)
            as *mut format_job_tree;
        jobs = (*(*ft).client).jobs;
        (*jobs).rbh_root = 0 as *mut format_job
    }
    fj0.tag = (*ft).tag;
    fj0.cmd = cmd;
    fj = format_job_tree_RB_FIND(jobs, &mut fj0);
    if fj.is_null() {
        fj = xcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<format_job>() as libc::c_ulong,
        ) as *mut format_job;
        (*fj).client = (*ft).client;
        (*fj).tag = (*ft).tag;
        (*fj).cmd = xstrdup(cmd);
        (*fj).expanded = 0 as *const libc::c_char;
        xasprintf(
            &mut (*fj).out as *mut *mut libc::c_char,
            b"<\'%s\' not ready>\x00" as *const u8 as *const libc::c_char,
            (*fj).cmd,
        );
        format_job_tree_RB_INSERT(jobs, fj);
    }
    expanded = format_expand1(es, cmd);
    if (*fj).expanded.is_null() || strcmp(expanded, (*fj).expanded) != 0 as libc::c_int {
        free((*fj).expanded as *mut libc::c_void);
        (*fj).expanded = xstrdup(expanded);
        force = 1 as libc::c_int
    } else {
        force = (*ft).flags & 0x2 as libc::c_int
    }
    t = time(0 as *mut time_t);
    if force != 0 && !(*fj).job.is_null() {
        job_free((*fj).job);
    }
    if force != 0 || (*fj).job.is_null() && (*fj).last != t {
        (*fj).job = job_run(
            expanded,
            0 as *mut session,
            server_client_get_cwd((*ft).client, 0 as *mut session),
            Some(format_job_update as unsafe extern "C" fn(_: *mut crate::job::job) -> ()),
            Some(format_job_complete as unsafe extern "C" fn(_: *mut crate::job::job) -> ()),
            None,
            fj as *mut libc::c_void,
            0x1 as libc::c_int,
            -(1 as libc::c_int),
            -(1 as libc::c_int),
        );
        if (*fj).job.is_null() {
            free((*fj).out as *mut libc::c_void);
            xasprintf(
                &mut (*fj).out as *mut *mut libc::c_char,
                b"<\'%s\' didn\'t start>\x00" as *const u8 as *const libc::c_char,
                (*fj).cmd,
            );
        }
        (*fj).last = t;
        (*fj).updated = 0 as libc::c_int
    }
    free(expanded as *mut libc::c_void);
    if (*ft).flags & 0x1 as libc::c_int != 0 {
        (*fj).status = 1 as libc::c_int
    }
    format_copy_state(&mut next, es, 0x2 as libc::c_int);
    return format_expand1(&mut next, (*fj).out);
}
/* Remove old jobs. */
unsafe extern "C" fn format_job_tidy(mut jobs: *mut format_job_tree, mut force: libc::c_int) {
    let mut fj: *mut format_job = 0 as *mut format_job;
    let mut fj1: *mut format_job = 0 as *mut format_job;
    let mut now: time_t = 0;
    now = time(0 as *mut time_t);
    fj = format_job_tree_RB_MINMAX(jobs, -(1 as libc::c_int));
    while !fj.is_null() && {
        fj1 = format_job_tree_RB_NEXT(fj);
        (1 as libc::c_int) != 0
    } {
        if !(force == 0
            && ((*fj).last > now || now - (*fj).last < 3600 as libc::c_int as libc::c_long))
        {
            format_job_tree_RB_REMOVE(jobs, fj);
            log_debug(
                b"%s: %s\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"format_job_tidy\x00"))
                    .as_ptr(),
                (*fj).cmd,
            );
            if !(*fj).job.is_null() {
                job_free((*fj).job);
            }
            free((*fj).expanded as *mut libc::c_void);
            free((*fj).cmd as *mut libc::c_void);
            free((*fj).out as *mut libc::c_void);
            free(fj as *mut libc::c_void);
        }
        fj = fj1
    }
}
/* Remove old jobs for client. */
#[no_mangle]
pub unsafe extern "C" fn format_lost_client(mut c: *mut client) {
    if !(*c).jobs.is_null() {
        format_job_tidy((*c).jobs, 1 as libc::c_int);
    }
    free((*c).jobs as *mut libc::c_void);
}
/* Remove old jobs periodically. */
unsafe extern "C" fn format_job_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut _arg: *mut libc::c_void,
) {
    let mut c: *mut client = 0 as *mut client;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 60 as libc::c_int as __time_t,
            tv_usec: 0,
        };
        init
    };
    format_job_tidy(&mut format_jobs, 0 as libc::c_int);
    c = clients.tqh_first;
    while !c.is_null() {
        if !(*c).jobs.is_null() {
            format_job_tidy((*c).jobs, 0 as libc::c_int);
        }
        c = (*c).entry.tqe_next
    }
    event_del(&mut format_job_event);
    event_add(&mut format_job_event, &mut tv);
}
/* Callback for host. */
unsafe extern "C" fn format_cb_host(mut _ft: *mut format_tree) -> *mut libc::c_char {
    let mut host: [libc::c_char; 65] = [0; 65];
    if gethostname(
        host.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        return xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    }
    return xstrdup(host.as_mut_ptr());
}
/* Callback for host_short. */
unsafe extern "C" fn format_cb_host_short(mut _ft: *mut format_tree) -> *mut libc::c_char {
    let mut host: [libc::c_char; 65] = [0; 65];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if gethostname(
        host.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        return xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    }
    cp = strchr(host.as_mut_ptr(), '.' as i32);
    if !cp.is_null() {
        *cp = '\u{0}' as i32 as libc::c_char
    }
    return xstrdup(host.as_mut_ptr());
}
/* Callback for pid. */
unsafe extern "C" fn format_cb_pid(mut _ft: *mut format_tree) -> *mut libc::c_char {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    xasprintf(
        &mut value as *mut *mut libc::c_char,
        b"%ld\x00" as *const u8 as *const libc::c_char,
        getpid() as libc::c_long,
    );
    return value;
}
/* Callback for session_attached_list. */
unsafe extern "C" fn format_cb_session_attached_list(
    mut ft: *mut format_tree,
) -> *mut libc::c_char {
    let mut s: *mut session = (*ft).s;
    let mut loop_0: *mut client = 0 as *mut client;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut size: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    buffer = evbuffer_new();
    if buffer.is_null() {
        fatalx(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    loop_0 = clients.tqh_first;
    while !loop_0.is_null() {
        if (*loop_0).session == s {
            if evbuffer_get_length(buffer) > 0 as libc::c_int as libc::c_ulong {
                evbuffer_add(
                    buffer,
                    b",\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
            }
            evbuffer_add_printf(
                buffer,
                b"%s\x00" as *const u8 as *const libc::c_char,
                (*loop_0).name,
            );
        }
        loop_0 = (*loop_0).entry.tqe_next
    }
    size = evbuffer_get_length(buffer) as libc::c_int;
    if size != 0 as libc::c_int {
        xasprintf(
            &mut value as *mut *mut libc::c_char,
            b"%.*s\x00" as *const u8 as *const libc::c_char,
            size,
            evbuffer_pullup(buffer, -(1 as libc::c_int) as ssize_t),
        );
    }
    evbuffer_free(buffer);
    return value;
}
/* Callback for session_alerts. */
unsafe extern "C" fn format_cb_session_alerts(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut s: *mut session = (*ft).s;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut alerts: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: [libc::c_char; 16] = [0; 16];
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    *alerts.as_mut_ptr() = '\u{0}' as i32 as libc::c_char;
    wl = winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        if !((*wl).flags & (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int)
            == 0 as libc::c_int)
        {
            xsnprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                b"%u\x00" as *const u8 as *const libc::c_char,
                (*wl).idx,
            );
            if *alerts.as_mut_ptr() as libc::c_int != '\u{0}' as i32 {
                strlcat(
                    alerts.as_mut_ptr(),
                    b",\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            }
            strlcat(
                alerts.as_mut_ptr(),
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
            if (*wl).flags & 0x2 as libc::c_int != 0 {
                strlcat(
                    alerts.as_mut_ptr(),
                    b"#\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            }
            if (*wl).flags & 0x1 as libc::c_int != 0 {
                strlcat(
                    alerts.as_mut_ptr(),
                    b"!\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            }
            if (*wl).flags & 0x4 as libc::c_int != 0 {
                strlcat(
                    alerts.as_mut_ptr(),
                    b"~\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                );
            }
        }
        wl = winlinks_RB_NEXT(wl)
    }
    return xstrdup(alerts.as_mut_ptr());
}
/* Callback for session_stack. */
unsafe extern "C" fn format_cb_session_stack(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut s: *mut session = (*ft).s;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut result: [libc::c_char; 1024] = [0; 1024];
    let mut tmp: [libc::c_char; 16] = [0; 16];
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    xsnprintf(
        result.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*(*s).curw).idx,
    );
    wl = (*s).lastw.tqh_first;
    while !wl.is_null() {
        xsnprintf(
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"%u\x00" as *const u8 as *const libc::c_char,
            (*wl).idx,
        );
        if *result.as_mut_ptr() as libc::c_int != '\u{0}' as i32 {
            strlcat(
                result.as_mut_ptr(),
                b",\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            );
        }
        strlcat(
            result.as_mut_ptr(),
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        wl = (*wl).sentry.tqe_next
    }
    return xstrdup(result.as_mut_ptr());
}
/* Callback for window_stack_index. */
unsafe extern "C" fn format_cb_window_stack_index(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut s: *mut session = (*(*ft).wl).session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut idx: u_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    idx = 0 as libc::c_int as u_int;
    wl = (*s).lastw.tqh_first;
    while !wl.is_null() {
        idx = idx.wrapping_add(1);
        if wl == (*ft).wl {
            break;
        }
        wl = (*wl).sentry.tqe_next
    }
    if wl.is_null() {
        return xstrdup(b"0\x00" as *const u8 as *const libc::c_char);
    }
    xasprintf(
        &mut value as *mut *mut libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        idx,
    );
    return value;
}
/* Callback for window_linked_sessions_list. */
unsafe extern "C" fn format_cb_window_linked_sessions_list(
    mut ft: *mut format_tree,
) -> *mut libc::c_char {
    let mut w: *mut window = (*(*ft).wl).window;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut size: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    buffer = evbuffer_new();
    if buffer.is_null() {
        fatalx(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if evbuffer_get_length(buffer) > 0 as libc::c_int as libc::c_ulong {
            evbuffer_add(
                buffer,
                b",\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        evbuffer_add_printf(
            buffer,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*(*wl).session).name,
        );
        wl = (*wl).wentry.tqe_next
    }
    size = evbuffer_get_length(buffer) as libc::c_int;
    if size != 0 as libc::c_int {
        xasprintf(
            &mut value as *mut *mut libc::c_char,
            b"%.*s\x00" as *const u8 as *const libc::c_char,
            size,
            evbuffer_pullup(buffer, -(1 as libc::c_int) as ssize_t),
        );
    }
    evbuffer_free(buffer);
    return value;
}
/* Callback for window_active_sessions. */
unsafe extern "C" fn format_cb_window_active_sessions(
    mut ft: *mut format_tree,
) -> *mut libc::c_char {
    let mut w: *mut window = (*(*ft).wl).window;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut n: u_int = 0 as libc::c_int as u_int;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if (*(*wl).session).curw == wl {
            n = n.wrapping_add(1)
        }
        wl = (*wl).wentry.tqe_next
    }
    xasprintf(
        &mut value as *mut *mut libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        n,
    );
    return value;
}
/* Callback for window_active_sessions_list. */
unsafe extern "C" fn format_cb_window_active_sessions_list(
    mut ft: *mut format_tree,
) -> *mut libc::c_char {
    let mut w: *mut window = (*(*ft).wl).window;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut size: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    buffer = evbuffer_new();
    if buffer.is_null() {
        fatalx(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if (*(*wl).session).curw == wl {
            if evbuffer_get_length(buffer) > 0 as libc::c_int as libc::c_ulong {
                evbuffer_add(
                    buffer,
                    b",\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
            }
            evbuffer_add_printf(
                buffer,
                b"%s\x00" as *const u8 as *const libc::c_char,
                (*(*wl).session).name,
            );
        }
        wl = (*wl).wentry.tqe_next
    }
    size = evbuffer_get_length(buffer) as libc::c_int;
    if size != 0 as libc::c_int {
        xasprintf(
            &mut value as *mut *mut libc::c_char,
            b"%.*s\x00" as *const u8 as *const libc::c_char,
            size,
            evbuffer_pullup(buffer, -(1 as libc::c_int) as ssize_t),
        );
    }
    evbuffer_free(buffer);
    return value;
}
/* Callback for window_active_clients. */
unsafe extern "C" fn format_cb_window_active_clients(
    mut ft: *mut format_tree,
) -> *mut libc::c_char {
    let mut w: *mut window = (*(*ft).wl).window;
    let mut loop_0: *mut client = 0 as *mut client;
    let mut client_session: *mut session = 0 as *mut session;
    let mut n: u_int = 0 as libc::c_int as u_int;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    loop_0 = clients.tqh_first;
    while !loop_0.is_null() {
        client_session = (*loop_0).session;
        if !client_session.is_null() {
            if w == (*(*client_session).curw).window {
                n = n.wrapping_add(1)
            }
        }
        loop_0 = (*loop_0).entry.tqe_next
    }
    xasprintf(
        &mut value as *mut *mut libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        n,
    );
    return value;
}
/* Callback for window_active_clients_list. */
unsafe extern "C" fn format_cb_window_active_clients_list(
    mut ft: *mut format_tree,
) -> *mut libc::c_char {
    let mut w: *mut window = (*(*ft).wl).window;
    let mut loop_0: *mut client = 0 as *mut client;
    let mut client_session: *mut session = 0 as *mut session;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut size: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    buffer = evbuffer_new();
    if buffer.is_null() {
        fatalx(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    loop_0 = clients.tqh_first;
    while !loop_0.is_null() {
        client_session = (*loop_0).session;
        if !client_session.is_null() {
            if w == (*(*client_session).curw).window {
                if evbuffer_get_length(buffer) > 0 as libc::c_int as libc::c_ulong {
                    evbuffer_add(
                        buffer,
                        b",\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        1 as libc::c_int as size_t,
                    );
                }
                evbuffer_add_printf(
                    buffer,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    (*loop_0).name,
                );
            }
        }
        loop_0 = (*loop_0).entry.tqe_next
    }
    size = evbuffer_get_length(buffer) as libc::c_int;
    if size != 0 as libc::c_int {
        xasprintf(
            &mut value as *mut *mut libc::c_char,
            b"%.*s\x00" as *const u8 as *const libc::c_char,
            size,
            evbuffer_pullup(buffer, -(1 as libc::c_int) as ssize_t),
        );
    }
    evbuffer_free(buffer);
    return value;
}
/* Callback for window_layout. */
unsafe extern "C" fn format_cb_window_layout(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut w: *mut window = (*ft).w;
    if w.is_null() {
        return 0 as *mut libc::c_char;
    }
    if !(*w).saved_layout_root.is_null() {
        return layout_dump((*w).saved_layout_root);
    }
    return layout_dump((*w).layout_root);
}
/* Callback for window_visible_layout. */
unsafe extern "C" fn format_cb_window_visible_layout(
    mut ft: *mut format_tree,
) -> *mut libc::c_char {
    let mut w: *mut window = (*ft).w;
    if w.is_null() {
        return 0 as *mut libc::c_char;
    }
    return layout_dump((*w).layout_root);
}
/* Callback for pane_start_command. */
unsafe extern "C" fn format_cb_start_command(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = (*ft).wp;
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    return cmd_stringify_argv((*wp).argc, (*wp).argv);
}
/* Callback for pane_current_command. */
unsafe extern "C" fn format_cb_current_command(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() || (*wp).shell.is_null() {
        return 0 as *mut libc::c_char;
    }
    cmd = osdep_get_name((*wp).fd, (*wp).tty.as_mut_ptr());
    if cmd.is_null() || *cmd as libc::c_int == '\u{0}' as i32 {
        free(cmd as *mut libc::c_void);
        cmd = cmd_stringify_argv((*wp).argc, (*wp).argv);
        if cmd.is_null() || *cmd as libc::c_int == '\u{0}' as i32 {
            free(cmd as *mut libc::c_void);
            cmd = xstrdup((*wp).shell)
        }
    }
    value = parse_window_name(cmd);
    free(cmd as *mut libc::c_void);
    return value;
}
/* Callback for pane_current_path. */
unsafe extern "C" fn format_cb_current_path(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut cwd: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    cwd = osdep_get_cwd((*wp).fd);
    if cwd.is_null() {
        return 0 as *mut libc::c_char;
    }
    return xstrdup(cwd);
}
/* Callback for history_bytes. */
unsafe extern "C" fn format_cb_history_bytes(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut gd: *mut grid = 0 as *mut grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut i: u_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    gd = (*wp).base.grid;
    i = 0 as libc::c_int as u_int;
    while i < (*gd).hsize.wrapping_add((*gd).sy) {
        gl = grid_get_line(gd, i);
        size = (size as libc::c_ulong).wrapping_add(
            ((*gl).cellsize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<grid_cell_entry>() as libc::c_ulong),
        ) as size_t as size_t;
        size = (size as libc::c_ulong).wrapping_add(
            ((*gl).extdsize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<grid_extd_entry>() as libc::c_ulong),
        ) as size_t as size_t;
        i = i.wrapping_add(1)
    }
    size = (size as libc::c_ulong).wrapping_add(
        ((*gd).hsize.wrapping_add((*gd).sy) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<grid_line>() as libc::c_ulong),
    ) as size_t as size_t;
    xasprintf(
        &mut value as *mut *mut libc::c_char,
        b"%zu\x00" as *const u8 as *const libc::c_char,
        size,
    );
    return value;
}
/* Callback for history_all_bytes. */
unsafe extern "C" fn format_cb_history_all_bytes(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut gd: *mut grid = 0 as *mut grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut i: u_int = 0;
    let mut lines: u_int = 0;
    let mut cells: u_int = 0 as libc::c_int as u_int;
    let mut extended_cells: u_int = 0 as libc::c_int as u_int;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    gd = (*wp).base.grid;
    lines = (*gd).hsize.wrapping_add((*gd).sy);
    i = 0 as libc::c_int as u_int;
    while i < lines {
        gl = grid_get_line(gd, i);
        cells = (cells as libc::c_uint).wrapping_add((*gl).cellsize) as u_int as u_int;
        extended_cells =
            (extended_cells as libc::c_uint).wrapping_add((*gl).extdsize) as u_int as u_int;
        i = i.wrapping_add(1)
    }
    xasprintf(
        &mut value as *mut *mut libc::c_char,
        b"%u,%zu,%u,%zu,%u,%zu\x00" as *const u8 as *const libc::c_char,
        lines,
        (lines as libc::c_ulong).wrapping_mul(::std::mem::size_of::<grid_line>() as libc::c_ulong),
        cells,
        (cells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<grid_cell_entry>() as libc::c_ulong),
        extended_cells,
        (extended_cells as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<grid_extd_entry>() as libc::c_ulong),
    );
    return value;
}
/* Callback for pane_tabs. */
unsafe extern "C" fn format_cb_pane_tabs(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut i: u_int = 0;
    let mut size: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    buffer = evbuffer_new();
    if buffer.is_null() {
        fatalx(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int as u_int;
    while i < (*(*wp).base.grid).sx {
        if !(*(*wp).base.tabs.offset((i >> 3 as libc::c_int) as isize) as libc::c_int
            & (1 as libc::c_int) << (i & 0x7 as libc::c_int as libc::c_uint)
            == 0)
        {
            if evbuffer_get_length(buffer) > 0 as libc::c_int as libc::c_ulong {
                evbuffer_add(
                    buffer,
                    b",\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
            }
            evbuffer_add_printf(buffer, b"%u\x00" as *const u8 as *const libc::c_char, i);
        }
        i = i.wrapping_add(1)
    }
    size = evbuffer_get_length(buffer) as libc::c_int;
    if size != 0 as libc::c_int {
        xasprintf(
            &mut value as *mut *mut libc::c_char,
            b"%.*s\x00" as *const u8 as *const libc::c_char,
            size,
            evbuffer_pullup(buffer, -(1 as libc::c_int) as ssize_t),
        );
    }
    evbuffer_free(buffer);
    return value;
}
/* Callback for session_group_list. */
unsafe extern "C" fn format_cb_session_group_list(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut s: *mut session = (*ft).s;
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut loop_0: *mut session = 0 as *mut session;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut size: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    sg = session_group_contains(s);
    if sg.is_null() {
        return 0 as *mut libc::c_char;
    }
    buffer = evbuffer_new();
    if buffer.is_null() {
        fatalx(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    loop_0 = (*sg).sessions.tqh_first;
    while !loop_0.is_null() {
        if evbuffer_get_length(buffer) > 0 as libc::c_int as libc::c_ulong {
            evbuffer_add(
                buffer,
                b",\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        evbuffer_add_printf(
            buffer,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*loop_0).name,
        );
        loop_0 = (*loop_0).gentry.tqe_next
    }
    size = evbuffer_get_length(buffer) as libc::c_int;
    if size != 0 as libc::c_int {
        xasprintf(
            &mut value as *mut *mut libc::c_char,
            b"%.*s\x00" as *const u8 as *const libc::c_char,
            size,
            evbuffer_pullup(buffer, -(1 as libc::c_int) as ssize_t),
        );
    }
    evbuffer_free(buffer);
    return value;
}
/* Callback for session_group_attached_list. */
unsafe extern "C" fn format_cb_session_group_attached_list(
    mut ft: *mut format_tree,
) -> *mut libc::c_char {
    let mut s: *mut session = (*ft).s;
    let mut client_session: *mut session = 0 as *mut session;
    let mut session_loop: *mut session = 0 as *mut session;
    let mut sg: *mut session_group = 0 as *mut session_group;
    let mut loop_0: *mut client = 0 as *mut client;
    let mut buffer: *mut evbuffer = 0 as *mut evbuffer;
    let mut size: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    sg = session_group_contains(s);
    if sg.is_null() {
        return 0 as *mut libc::c_char;
    }
    buffer = evbuffer_new();
    if buffer.is_null() {
        fatalx(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    loop_0 = clients.tqh_first;
    while !loop_0.is_null() {
        client_session = (*loop_0).session;
        if !client_session.is_null() {
            session_loop = (*sg).sessions.tqh_first;
            while !session_loop.is_null() {
                if session_loop == client_session {
                    if evbuffer_get_length(buffer) > 0 as libc::c_int as libc::c_ulong {
                        evbuffer_add(
                            buffer,
                            b",\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                            1 as libc::c_int as size_t,
                        );
                    }
                    evbuffer_add_printf(
                        buffer,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        (*loop_0).name,
                    );
                }
                session_loop = (*session_loop).gentry.tqe_next
            }
        }
        loop_0 = (*loop_0).entry.tqe_next
    }
    size = evbuffer_get_length(buffer) as libc::c_int;
    if size != 0 as libc::c_int {
        xasprintf(
            &mut value as *mut *mut libc::c_char,
            b"%.*s\x00" as *const u8 as *const libc::c_char,
            size,
            evbuffer_pullup(buffer, -(1 as libc::c_int) as ssize_t),
        );
    }
    evbuffer_free(buffer);
    return value;
}
/* Callback for pane_in_mode. */
unsafe extern "C" fn format_cb_pane_in_mode(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut n: u_int = 0 as libc::c_int as u_int;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    wme = (*wp).modes.tqh_first;
    while !wme.is_null() {
        n = n.wrapping_add(1);
        wme = (*wme).entry.tqe_next
    }
    xasprintf(
        &mut value as *mut *mut libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        n,
    );
    return value;
}
/* Callback for pane_at_top. */
unsafe extern "C" fn format_cb_pane_at_top(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut w: *mut window = 0 as *mut window;
    let mut status: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    w = (*wp).window;
    status = options_get_number(
        (*w).options,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if status == 1 as libc::c_int {
        flag = ((*wp).yoff == 1 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        flag = ((*wp).yoff == 0 as libc::c_int as libc::c_uint) as libc::c_int
    }
    xasprintf(
        &mut value as *mut *mut libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        flag,
    );
    return value;
}
/* Callback for pane_at_bottom. */
unsafe extern "C" fn format_cb_pane_at_bottom(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = (*ft).wp;
    let mut w: *mut window = 0 as *mut window;
    let mut status: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    w = (*wp).window;
    status = options_get_number(
        (*w).options,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if status == 2 as libc::c_int {
        flag = ((*wp).yoff.wrapping_add((*wp).sy)
            == (*w).sy.wrapping_sub(1 as libc::c_int as libc::c_uint)) as libc::c_int
    } else {
        flag = ((*wp).yoff.wrapping_add((*wp).sy) == (*w).sy) as libc::c_int
    }
    xasprintf(
        &mut value as *mut *mut libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        flag,
    );
    return value;
}
/* Callback for cursor_character. */
unsafe extern "C" fn format_cb_cursor_character(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = (*ft).wp;
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
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    grid_view_get_cell((*wp).base.grid, (*wp).base.cx, (*wp).base.cy, &mut gc);
    if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
        xasprintf(
            &mut value as *mut *mut libc::c_char,
            b"%.*s\x00" as *const u8 as *const libc::c_char,
            gc.data.size as libc::c_int,
            gc.data.data.as_mut_ptr(),
        );
    }
    return value;
}
/* Return word at given coordinates. Caller frees. */
#[no_mangle]
pub unsafe extern "C" fn format_grid_word(
    mut gd: *mut grid,
    mut x: u_int,
    mut y: u_int,
) -> *mut libc::c_char {
    let mut gl: *const grid_line = 0 as *const grid_line;
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
    let mut ws: *const libc::c_char = 0 as *const libc::c_char;
    let mut ud: *mut Utf8Data = 0 as *mut Utf8Data;
    let mut end: u_int = 0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    ws = options_get_string(
        global_s_options,
        b"word-separators\x00" as *const u8 as *const libc::c_char,
    );
    loop {
        grid_get_cell(gd, x, y, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int != 0 {
            break;
        }
        if utf8_cstrhas(ws, &mut gc.data) != 0 {
            found = 1 as libc::c_int;
            break;
        } else {
            if x == 0 as libc::c_int as libc::c_uint {
                if y == 0 as libc::c_int as libc::c_uint {
                    break;
                }
                gl = grid_peek_line(gd, y.wrapping_sub(1 as libc::c_int as libc::c_uint));
                if !(*gl).flags & 0x1 as libc::c_int != 0 {
                    break;
                }
                y = y.wrapping_sub(1);
                x = grid_line_length(gd, y);
                if x == 0 as libc::c_int as libc::c_uint {
                    break;
                }
            }
            x = x.wrapping_sub(1)
        }
    }
    loop {
        if found != 0 {
            end = grid_line_length(gd, y);
            if end == 0 as libc::c_int as libc::c_uint
                || x == end.wrapping_sub(1 as libc::c_int as libc::c_uint)
            {
                if y == (*gd)
                    .hsize
                    .wrapping_add((*gd).sy)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                {
                    break;
                }
                gl = grid_peek_line(gd, y);
                if !(*gl).flags & 0x1 as libc::c_int != 0 {
                    break;
                }
                y = y.wrapping_add(1);
                x = 0 as libc::c_int as u_int
            } else {
                x = x.wrapping_add(1)
            }
        }
        found = 1 as libc::c_int;
        grid_get_cell(gd, x, y, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int != 0 {
            break;
        }
        if utf8_cstrhas(ws, &mut gc.data) != 0 {
            break;
        }
        ud = xreallocarray(
            ud as *mut libc::c_void,
            size.wrapping_add(2 as libc::c_int as libc::c_ulong),
            ::std::mem::size_of::<Utf8Data>() as libc::c_ulong,
        ) as *mut Utf8Data;
        let fresh0 = size;
        size = size.wrapping_add(1);
        memcpy(
            &mut *ud.offset(fresh0 as isize) as *mut Utf8Data as *mut libc::c_void,
            &mut gc.data as *mut Utf8Data as *const libc::c_void,
            ::std::mem::size_of::<Utf8Data>() as libc::c_ulong,
        );
    }
    if size != 0 as libc::c_int as libc::c_ulong {
        (*ud.offset(size as isize)).size = 0 as libc::c_int as u_char;
        s = utf8_tocstr(ud);
        free(ud as *mut libc::c_void);
    }
    return s;
}
/* Callback for mouse_word. */
unsafe extern "C" fn format_cb_mouse_word(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut gd: *mut grid = 0 as *mut grid;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*ft).m.valid == 0 {
        return 0 as *mut libc::c_char;
    }
    wp = cmd_mouse_pane(&mut (*ft).m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    if cmd_mouse_at(wp, &mut (*ft).m, &mut x, &mut y, 0 as libc::c_int) != 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if !(*wp).modes.tqh_first.is_null() {
        if (*(*wp).modes.tqh_first).mode == &window_copy_mode as *const window_mode
            || (*(*wp).modes.tqh_first).mode == &window_view_mode as *const window_mode
        {
            s = window_copy_get_word(wp, x, y);
            return s;
        }
        return 0 as *mut libc::c_char;
    }
    gd = (*wp).base.grid;
    return format_grid_word(gd, x, (*gd).hsize.wrapping_add(y));
}
/* Return line at given coordinates. Caller frees. */
#[no_mangle]
pub unsafe extern "C" fn format_grid_line(mut gd: *mut grid, mut y: u_int) -> *mut libc::c_char {
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
    let mut ud: *mut Utf8Data = 0 as *mut Utf8Data;
    let mut x: u_int = 0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    x = 0 as libc::c_int as u_int;
    while x < grid_line_length(gd, y) {
        grid_get_cell(gd, x, y, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int != 0 {
            break;
        }
        ud = xreallocarray(
            ud as *mut libc::c_void,
            size.wrapping_add(2 as libc::c_int as libc::c_ulong),
            ::std::mem::size_of::<Utf8Data>() as libc::c_ulong,
        ) as *mut Utf8Data;
        let fresh1 = size;
        size = size.wrapping_add(1);
        memcpy(
            &mut *ud.offset(fresh1 as isize) as *mut Utf8Data as *mut libc::c_void,
            &mut gc.data as *mut Utf8Data as *const libc::c_void,
            ::std::mem::size_of::<Utf8Data>() as libc::c_ulong,
        );
        x = x.wrapping_add(1)
    }
    if size != 0 as libc::c_int as libc::c_ulong {
        (*ud.offset(size as isize)).size = 0 as libc::c_int as u_char;
        s = utf8_tocstr(ud);
        free(ud as *mut libc::c_void);
    }
    return s;
}
/* Callback for mouse_line. */
unsafe extern "C" fn format_cb_mouse_line(mut ft: *mut format_tree) -> *mut libc::c_char {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut gd: *mut grid = 0 as *mut grid;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    if (*ft).m.valid == 0 {
        return 0 as *mut libc::c_char;
    }
    wp = cmd_mouse_pane(&mut (*ft).m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp.is_null() {
        return 0 as *mut libc::c_char;
    }
    if cmd_mouse_at(wp, &mut (*ft).m, &mut x, &mut y, 0 as libc::c_int) != 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if !(*wp).modes.tqh_first.is_null() {
        if (*(*wp).modes.tqh_first).mode == &window_copy_mode as *const window_mode
            || (*(*wp).modes.tqh_first).mode == &window_view_mode as *const window_mode
        {
            return window_copy_get_line(wp, y);
        }
        return 0 as *mut libc::c_char;
    }
    gd = (*wp).base.grid;
    return format_grid_line(gd, (*gd).hsize.wrapping_add(y));
}
/* Merge one format tree into another. */
#[no_mangle]
pub unsafe extern "C" fn format_merge(mut ft: *mut format_tree, mut from: *mut format_tree) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    fe = format_entry_tree_RB_MINMAX(&mut (*from).tree, -(1 as libc::c_int));
    while !fe.is_null() {
        if !(*fe).value.is_null() {
            format_add(
                ft,
                (*fe).key,
                b"%s\x00" as *const u8 as *const libc::c_char,
                (*fe).value,
            );
        }
        fe = format_entry_tree_RB_NEXT(fe)
    }
}
/* Get format pane. */
#[no_mangle]
pub unsafe extern "C" fn format_get_pane(mut ft: *mut format_tree) -> *mut window_pane {
    return (*ft).wp;
}
/* Add item bits to tree. */
unsafe extern "C" fn format_create_add_item(
    mut ft: *mut format_tree,
    mut item: *mut crate::cmd_queue::cmdq_item,
) {
    let mut event: *mut key_event = cmdq_get_event(item);
    let mut m: *mut mouse_event = &mut (*event).m;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    cmdq_merge_formats(item, ft);
    if (*m).valid != 0 && {
        wp = cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
        !wp.is_null()
    } {
        format_add(
            ft,
            b"mouse_pane\x00" as *const u8 as *const libc::c_char,
            b"%%%u\x00" as *const u8 as *const libc::c_char,
            (*wp).id,
        );
        if cmd_mouse_at(wp, m, &mut x, &mut y, 0 as libc::c_int) == 0 as libc::c_int {
            format_add(
                ft,
                b"mouse_x\x00" as *const u8 as *const libc::c_char,
                b"%u\x00" as *const u8 as *const libc::c_char,
                x,
            );
            format_add(
                ft,
                b"mouse_y\x00" as *const u8 as *const libc::c_char,
                b"%u\x00" as *const u8 as *const libc::c_char,
                y,
            );
            format_add_cb(
                ft,
                b"mouse_word\x00" as *const u8 as *const libc::c_char,
                Some(
                    format_cb_mouse_word
                        as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
                ),
            );
            format_add_cb(
                ft,
                b"mouse_line\x00" as *const u8 as *const libc::c_char,
                Some(
                    format_cb_mouse_line
                        as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
                ),
            );
        }
    }
    memcpy(
        &mut (*ft).m as *mut mouse_event as *mut libc::c_void,
        m as *const libc::c_void,
        ::std::mem::size_of::<mouse_event>() as libc::c_ulong,
    );
}
/* Create a new tree. */
#[no_mangle]
pub unsafe extern "C" fn format_create(
    mut c: *mut client,
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut tag: libc::c_int,
    mut flags: libc::c_int,
) -> *mut format_tree {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut wm: *mut *const window_mode = 0 as *mut *const window_mode;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    if event_initialized(&mut format_job_event) == 0 {
        event_set(
            &mut format_job_event,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                format_job_timer
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        format_job_timer(
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            0 as *mut libc::c_void,
        );
    }
    ft = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<format_tree>() as libc::c_ulong,
    ) as *mut format_tree;
    (*ft).tree.rbh_root = 0 as *mut format_entry;
    if !c.is_null() {
        (*ft).client = c;
        (*(*ft).client).references += 1
    }
    (*ft).item = item;
    (*ft).tag = tag as u_int;
    (*ft).flags = flags;
    format_add(
        ft,
        b"version\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        getversion(),
    );
    format_add_cb(
        ft,
        b"host\x00" as *const u8 as *const libc::c_char,
        Some(format_cb_host as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char),
    );
    format_add_cb(
        ft,
        b"host_short\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_host_short as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"pid\x00" as *const u8 as *const libc::c_char,
        Some(format_cb_pid as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char),
    );
    format_add(
        ft,
        b"socket_path\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        socket_path,
    );
    format_add_tv(
        ft,
        b"start_time\x00" as *const u8 as *const libc::c_char,
        &mut start_time,
    );
    wm = all_window_modes.as_mut_ptr();
    while !(*wm).is_null() {
        if !(**wm).default_format.is_null() {
            xsnprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                b"%s_format\x00" as *const u8 as *const libc::c_char,
                (**wm).name,
            );
            tmp[strcspn(
                tmp.as_mut_ptr(),
                b"-\x00" as *const u8 as *const libc::c_char,
            ) as usize] = '_' as i32 as libc::c_char;
            format_add(
                ft,
                tmp.as_mut_ptr(),
                b"%s\x00" as *const u8 as *const libc::c_char,
                (**wm).default_format,
            );
        }
        wm = wm.offset(1)
    }
    if !item.is_null() {
        format_create_add_item(ft, item);
    }
    return ft;
}
/* Free a tree. */
#[no_mangle]
pub unsafe extern "C" fn format_free(mut ft: *mut format_tree) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe1: *mut format_entry = 0 as *mut format_entry;
    fe = format_entry_tree_RB_MINMAX(&mut (*ft).tree, -(1 as libc::c_int));
    while !fe.is_null() && {
        fe1 = format_entry_tree_RB_NEXT(fe);
        (1 as libc::c_int) != 0
    } {
        format_entry_tree_RB_REMOVE(&mut (*ft).tree, fe);
        free((*fe).value as *mut libc::c_void);
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        fe = fe1
    }
    if !(*ft).client.is_null() {
        server_client_unref((*ft).client);
    }
    free(ft as *mut libc::c_void);
}
/* Walk each format. */
#[no_mangle]
pub unsafe extern "C" fn format_each(
    mut ft: *mut format_tree,
    mut cb: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: *mut libc::c_void,
        ) -> (),
    >,
    mut arg: *mut libc::c_void,
) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut s: [libc::c_char; 64] = [0; 64];
    fe = format_entry_tree_RB_MINMAX(&mut (*ft).tree, -(1 as libc::c_int));
    while !fe.is_null() {
        if (*fe).time != 0 as libc::c_int as libc::c_long {
            xsnprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                b"%lld\x00" as *const u8 as *const libc::c_char,
                (*fe).time as libc::c_longlong,
            );
            cb.expect("non-null function pointer")((*fe).key, s.as_mut_ptr(), arg);
        } else {
            if (*fe).value.is_null() && (*fe).cb.is_some() {
                (*fe).value = (*fe).cb.expect("non-null function pointer")(ft);
                if (*fe).value.is_null() {
                    (*fe).value = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
                }
            }
            cb.expect("non-null function pointer")((*fe).key, (*fe).value, arg);
        }
        fe = format_entry_tree_RB_NEXT(fe)
    }
}
/* Add a key-value pair. */
#[no_mangle]
pub unsafe extern "C" fn format_add(
    mut ft: *mut format_tree,
    mut key: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_now: *mut format_entry = 0 as *mut format_entry;
    let mut ap: ::std::ffi::VaListImpl;
    fe = xmalloc(::std::mem::size_of::<format_entry>() as libc::c_ulong) as *mut format_entry;
    (*fe).key = xstrdup(key);
    fe_now = format_entry_tree_RB_INSERT(&mut (*ft).tree, fe);
    if !fe_now.is_null() {
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        free((*fe_now).value as *mut libc::c_void);
        fe = fe_now
    }
    (*fe).cb = None;
    (*fe).time = 0 as libc::c_int as time_t;
    ap = args.clone();
    xvasprintf(&mut (*fe).value, fmt, ap.as_va_list());
}
/* Add a key and time. */
#[no_mangle]
pub unsafe extern "C" fn format_add_tv(
    mut ft: *mut format_tree,
    mut key: *const libc::c_char,
    mut tv: *mut timeval,
) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_now: *mut format_entry = 0 as *mut format_entry;
    fe = xmalloc(::std::mem::size_of::<format_entry>() as libc::c_ulong) as *mut format_entry;
    (*fe).key = xstrdup(key);
    fe_now = format_entry_tree_RB_INSERT(&mut (*ft).tree, fe);
    if !fe_now.is_null() {
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        free((*fe_now).value as *mut libc::c_void);
        fe = fe_now
    }
    (*fe).cb = None;
    (*fe).time = (*tv).tv_sec;
    (*fe).value = 0 as *mut libc::c_char;
}
/* Add a key and function. */
#[no_mangle]
pub unsafe extern "C" fn format_add_cb(
    mut ft: *mut format_tree,
    mut key: *const libc::c_char,
    mut cb: format_cb,
) {
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_now: *mut format_entry = 0 as *mut format_entry;
    fe = xmalloc(::std::mem::size_of::<format_entry>() as libc::c_ulong) as *mut format_entry;
    (*fe).key = xstrdup(key);
    fe_now = format_entry_tree_RB_INSERT(&mut (*ft).tree, fe);
    if !fe_now.is_null() {
        free((*fe).key as *mut libc::c_void);
        free(fe as *mut libc::c_void);
        free((*fe_now).value as *mut libc::c_void);
        fe = fe_now
    }
    (*fe).cb = cb;
    (*fe).time = 0 as libc::c_int as time_t;
    (*fe).value = 0 as *mut libc::c_char;
}
/* Quote special characters in string. */
unsafe extern "C" fn format_quote(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut at: *mut libc::c_char = 0 as *mut libc::c_char;
    out = xmalloc(
        strlen(s)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    at = out;
    cp = s;
    while *cp as libc::c_int != '\u{0}' as i32 {
        if !strchr(
            b"|&;<>()$`\\\"\'*?[# =%\x00" as *const u8 as *const libc::c_char,
            *cp as libc::c_int,
        )
        .is_null()
        {
            let fresh2 = at;
            at = at.offset(1);
            *fresh2 = '\\' as i32 as libc::c_char
        }
        let fresh3 = at;
        at = at.offset(1);
        *fresh3 = *cp;
        cp = cp.offset(1)
    }
    *at = '\u{0}' as i32 as libc::c_char;
    return out;
}
/* Make a prettier time. */
unsafe extern "C" fn format_pretty_time(mut t: time_t) -> *mut libc::c_char {
    let mut now_tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut now: time_t = 0;
    let mut age: time_t = 0;
    let mut s: [libc::c_char; 6] = [0; 6];
    time(&mut now);
    if now < t {
        now = t
    }
    age = now - t;
    localtime_r(&mut now, &mut now_tm);
    localtime_r(&mut t, &mut tm);
    /* Last 24 hours. */
    if age < (24 as libc::c_int * 3600 as libc::c_int) as libc::c_long {
        strftime(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
            b"%H:%M\x00" as *const u8 as *const libc::c_char,
            &mut tm,
        );
        return xstrdup(s.as_mut_ptr());
    }
    /* This month or last 28 days. */
    if tm.tm_year == now_tm.tm_year && tm.tm_mon == now_tm.tm_mon
        || age < (28 as libc::c_int * 24 as libc::c_int * 3600 as libc::c_int) as libc::c_long
    {
        strftime(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
            b"%a%d\x00" as *const u8 as *const libc::c_char,
            &mut tm,
        );
        return xstrdup(s.as_mut_ptr());
    }
    /* Last 12 months. */
    if tm.tm_year == now_tm.tm_year && tm.tm_mon < now_tm.tm_mon
        || tm.tm_year == now_tm.tm_year - 1 as libc::c_int && tm.tm_mon > now_tm.tm_mon
    {
        strftime(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
            b"%d%b\x00" as *const u8 as *const libc::c_char,
            &mut tm,
        );
        return xstrdup(s.as_mut_ptr());
    }
    /* Older than that. */
    strftime(
        s.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        b"%h%y\x00" as *const u8 as *const libc::c_char,
        &mut tm,
    );
    return xstrdup(s.as_mut_ptr());
}
/* Find a format entry. */
unsafe extern "C" fn format_find(
    mut ft: *mut format_tree,
    mut key: *const libc::c_char,
    mut modifiers: libc::c_int,
    mut time_format: *const libc::c_char,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut fe: *mut format_entry = 0 as *mut format_entry;
    let mut fe_find: format_entry = format_entry {
        key: 0 as *mut libc::c_char,
        value: 0 as *mut libc::c_char,
        time: 0,
        cb: None,
        entry: C2RustUnnamed_26 {
            rbe_left: 0 as *mut format_entry,
            rbe_right: 0 as *mut format_entry,
            rbe_parent: 0 as *mut format_entry,
            rbe_color: 0,
        },
    };
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut o: *mut crate::options::options_entry = 0 as *mut crate::options::options_entry;
    let mut idx: libc::c_int = 0;
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saved: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: [libc::c_char; 512] = [0; 512];
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: time_t = 0 as libc::c_int as time_t;
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    o = options_parse_get(global_options, key, &mut idx, 0 as libc::c_int);
    if o.is_null() && !(*ft).wp.is_null() {
        o = options_parse_get((*(*ft).wp).options, key, &mut idx, 0 as libc::c_int)
    }
    if o.is_null() && !(*ft).w.is_null() {
        o = options_parse_get((*(*ft).w).options, key, &mut idx, 0 as libc::c_int)
    }
    if o.is_null() {
        o = options_parse_get(global_w_options, key, &mut idx, 0 as libc::c_int)
    }
    if o.is_null() && !(*ft).s.is_null() {
        o = options_parse_get((*(*ft).s).options, key, &mut idx, 0 as libc::c_int)
    }
    if o.is_null() {
        o = options_parse_get(global_s_options, key, &mut idx, 0 as libc::c_int)
    }
    if !o.is_null() {
        found = options_to_string(o, idx, 1 as libc::c_int)
    } else {
        fe_find.key = key as *mut libc::c_char;
        fe = format_entry_tree_RB_FIND(&mut (*ft).tree, &mut fe_find);
        if !fe.is_null() {
            if (*fe).time != 0 as libc::c_int as libc::c_long {
                t = (*fe).time
            } else {
                if (*fe).value.is_null() && (*fe).cb.is_some() {
                    (*fe).value = (*fe).cb.expect("non-null function pointer")(ft);
                    if (*fe).value.is_null() {
                        (*fe).value = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
                    }
                }
                found = xstrdup((*fe).value)
            }
        } else {
            if !modifiers & 0x1 as libc::c_int != 0 {
                envent = 0 as *mut environ_entry;
                if !(*ft).s.is_null() {
                    envent = environ_find((*(*ft).s).environ, key)
                }
                if envent.is_null() {
                    envent = environ_find(global_environ, key)
                }
                if !envent.is_null() && !(*envent).value.is_null() {
                    found = xstrdup((*envent).value);
                    current_block = 7121915885831525550;
                } else {
                    current_block = 8180496224585318153;
                }
            } else {
                current_block = 8180496224585318153;
            }
            match current_block {
                7121915885831525550 => {}
                _ => return 0 as *mut libc::c_char,
            }
        }
    }
    if modifiers & 0x1 as libc::c_int != 0 {
        if t == 0 as libc::c_int as libc::c_long && !found.is_null() {
            t = strtonum(
                found,
                0 as libc::c_int as libc::c_longlong,
                9223372036854775807 as libc::c_long as libc::c_longlong,
                &mut errstr,
            ) as time_t;
            if !errstr.is_null() {
                t = 0 as libc::c_int as time_t
            }
            free(found as *mut libc::c_void);
        }
        if t == 0 as libc::c_int as libc::c_long {
            return 0 as *mut libc::c_char;
        }
        if modifiers & 0x400 as libc::c_int != 0 {
            found = format_pretty_time(t)
        } else {
            if !time_format.is_null() {
                localtime_r(&mut t, &mut tm);
                strftime(
                    s.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                    time_format,
                    &mut tm,
                );
            } else {
                ctime_r(&mut t, s.as_mut_ptr());
                s[strcspn(
                    s.as_mut_ptr(),
                    b"\n\x00" as *const u8 as *const libc::c_char,
                ) as usize] = '\u{0}' as i32 as libc::c_char
            }
            found = xstrdup(s.as_mut_ptr())
        }
        return found;
    }
    if t != 0 as libc::c_int as libc::c_long {
        xasprintf(
            &mut found as *mut *mut libc::c_char,
            b"%lld\x00" as *const u8 as *const libc::c_char,
            t as libc::c_longlong,
        );
    } else if found.is_null() {
        return 0 as *mut libc::c_char;
    }
    if modifiers & 0x2 as libc::c_int != 0 {
        saved = found;
        found = xstrdup(__xpg_basename(saved));
        free(saved as *mut libc::c_void);
    }
    if modifiers & 0x4 as libc::c_int != 0 {
        saved = found;
        found = xstrdup(dirname(saved));
        free(saved as *mut libc::c_void);
    }
    if modifiers & 0x8 as libc::c_int != 0 {
        saved = found;
        found = xstrdup(format_quote(saved));
        free(saved as *mut libc::c_void);
    }
    return found;
}
/* Remove escaped characters from string. */
unsafe extern "C" fn format_strip(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut brackets: libc::c_int = 0 as libc::c_int;
    out = xmalloc(strlen(s).wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    cp = out;
    while *s as libc::c_int != '\u{0}' as i32 {
        if *s as libc::c_int == '#' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == '{' as i32
        {
            brackets += 1
        }
        if *s as libc::c_int == '#' as i32
            && !strchr(
                b",#{}:\x00" as *const u8 as *const libc::c_char,
                *s.offset(1 as libc::c_int as isize) as libc::c_int,
            )
            .is_null()
        {
            if brackets != 0 as libc::c_int {
                let fresh4 = cp;
                cp = cp.offset(1);
                *fresh4 = *s
            }
        } else {
            if *s as libc::c_int == '}' as i32 {
                brackets -= 1
            }
            let fresh5 = cp;
            cp = cp.offset(1);
            *fresh5 = *s
        }
        s = s.offset(1)
    }
    *cp = '\u{0}' as i32 as libc::c_char;
    return out;
}
/* Skip until end. */
#[no_mangle]
pub unsafe extern "C" fn format_skip(
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *const libc::c_char {
    let mut brackets: libc::c_int = 0 as libc::c_int;
    while *s as libc::c_int != '\u{0}' as i32 {
        if *s as libc::c_int == '#' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == '{' as i32
        {
            brackets += 1
        }
        if *s as libc::c_int == '#' as i32
            && !strchr(
                b",#{}:\x00" as *const u8 as *const libc::c_char,
                *s.offset(1 as libc::c_int as isize) as libc::c_int,
            )
            .is_null()
        {
            s = s.offset(1)
        } else {
            if *s as libc::c_int == '}' as i32 {
                brackets -= 1
            }
            if !strchr(end, *s as libc::c_int).is_null() && brackets == 0 as libc::c_int {
                break;
            }
        }
        s = s.offset(1)
    }
    if *s as libc::c_int == '\u{0}' as i32 {
        return 0 as *const libc::c_char;
    }
    return s;
}
/* Return left and right alternatives separated by commas. */
unsafe extern "C" fn format_choose(
    mut es: *mut format_expand_state,
    mut s: *const libc::c_char,
    mut left: *mut *mut libc::c_char,
    mut right: *mut *mut libc::c_char,
    mut expand: libc::c_int,
) -> libc::c_int {
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut left0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right0: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = format_skip(s, b",\x00" as *const u8 as *const libc::c_char);
    if cp.is_null() {
        return -(1 as libc::c_int);
    }
    left0 = xstrndup(s, cp.wrapping_offset_from(s) as libc::c_long as size_t);
    right0 = xstrdup(cp.offset(1 as libc::c_int as isize));
    if expand != 0 {
        *left = format_expand1(es, left0);
        free(left0 as *mut libc::c_void);
        *right = format_expand1(es, right0);
        free(right0 as *mut libc::c_void);
    } else {
        *left = left0;
        *right = right0
    }
    return 0 as libc::c_int;
}
/* Is this true? */
#[no_mangle]
pub unsafe extern "C" fn format_true(mut s: *const libc::c_char) -> libc::c_int {
    if !s.is_null()
        && *s as libc::c_int != '\u{0}' as i32
        && (*s.offset(0 as libc::c_int as isize) as libc::c_int != '0' as i32
            || *s.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* Check if modifier end. */
unsafe extern "C" fn format_is_end(mut c: libc::c_char) -> libc::c_int {
    return (c as libc::c_int == ';' as i32 || c as libc::c_int == ':' as i32) as libc::c_int;
}
/* Add to modifier list. */
unsafe extern "C" fn format_add_modifier(
    mut list: *mut *mut format_modifier,
    mut count: *mut u_int,
    mut c: *const libc::c_char,
    mut n: size_t,
    mut argv: *mut *mut libc::c_char,
    mut argc: libc::c_int,
) {
    let mut fm: *mut format_modifier = 0 as *mut format_modifier;
    *list = xreallocarray(
        *list as *mut libc::c_void,
        (*count).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
        ::std::mem::size_of::<format_modifier>() as libc::c_ulong,
    ) as *mut format_modifier;
    let fresh6 = *count;
    *count = (*count).wrapping_add(1);
    fm = &mut *(*list).offset(fresh6 as isize) as *mut format_modifier;
    memcpy(
        (*fm).modifier.as_mut_ptr() as *mut libc::c_void,
        c as *const libc::c_void,
        n,
    );
    (*fm).modifier[n as usize] = '\u{0}' as i32 as libc::c_char;
    (*fm).size = n as u_int;
    (*fm).argv = argv;
    (*fm).argc = argc;
}
/* Free modifier list. */
unsafe extern "C" fn format_free_modifiers(mut list: *mut format_modifier, mut count: u_int) {
    let mut i: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while i < count {
        cmd_free_argv(
            (*list.offset(i as isize)).argc,
            (*list.offset(i as isize)).argv,
        );
        i = i.wrapping_add(1)
    }
    free(list as *mut libc::c_void);
}
/* Build modifier list. */
unsafe extern "C" fn format_build_modifiers(
    mut es: *mut format_expand_state,
    mut s: *mut *const libc::c_char,
    mut count: *mut u_int,
) -> *mut format_modifier {
    let mut cp: *const libc::c_char = *s;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut list: *mut format_modifier = 0 as *mut format_modifier;
    let mut c: libc::c_char = 0;
    let mut last: [libc::c_char; 4] =
        *::std::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"X;:\x00");
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argc: libc::c_int = 0;
    /*
     * Modifiers are a ; separated list of the forms:
     *      l,m,C,b,d,n,t,q,E,T,S,W,P,<,>
     *	=a
     *	=/a
     *      =/a/
     *	s/a/b/
     *	s/a/b
     *	||,&&,!=,==,<=,>=
     */
    *count = 0 as libc::c_int as u_int;
    while *cp as libc::c_int != '\u{0}' as i32 && *cp as libc::c_int != ':' as i32 {
        /* Skip any separator character. */
        if *cp as libc::c_int == ';' as i32 {
            cp = cp.offset(1)
        }
        /* Check single character modifiers with no arguments. */
        if !strchr(
            b"lbdnqETSWP<>\x00" as *const u8 as *const libc::c_char,
            *cp.offset(0 as libc::c_int as isize) as libc::c_int,
        )
        .is_null()
            && format_is_end(*cp.offset(1 as libc::c_int as isize)) != 0
        {
            format_add_modifier(
                &mut list,
                count,
                cp,
                1 as libc::c_int as size_t,
                0 as *mut *mut libc::c_char,
                0 as libc::c_int,
            );
            cp = cp.offset(1)
        } else if (memcmp(
            b"||\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            cp as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || memcmp(
                b"&&\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                cp as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            || memcmp(
                b"!=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                cp as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            || memcmp(
                b"==\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                cp as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            || memcmp(
                b"<=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                cp as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            || memcmp(
                b">=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                cp as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
            && format_is_end(*cp.offset(2 as libc::c_int as isize)) != 0
        {
            format_add_modifier(
                &mut list,
                count,
                cp,
                2 as libc::c_int as size_t,
                0 as *mut *mut libc::c_char,
                0 as libc::c_int,
            );
            cp = cp.offset(2 as libc::c_int as isize)
        } else {
            /* Then try double character with no arguments. */
            /* Now try single character with arguments. */
            if strchr(
                b"mCst=pe\x00" as *const u8 as *const libc::c_char,
                *cp.offset(0 as libc::c_int as isize) as libc::c_int,
            )
            .is_null()
            {
                break;
            }
            c = *cp.offset(0 as libc::c_int as isize);
            /* No arguments provided. */
            if format_is_end(*cp.offset(1 as libc::c_int as isize)) != 0 {
                format_add_modifier(
                    &mut list,
                    count,
                    cp,
                    1 as libc::c_int as size_t,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                );
                cp = cp.offset(1)
            } else {
                argv = 0 as *mut *mut libc::c_char;
                argc = 0 as libc::c_int;
                /* Single argument with no wrapper character. */
                if *(*__ctype_b_loc())
                    .offset(*cp.offset(1 as libc::c_int as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int
                    == 0
                    || *cp.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
                {
                    end = format_skip(
                        cp.offset(1 as libc::c_int as isize),
                        b":;\x00" as *const u8 as *const libc::c_char,
                    );
                    if end.is_null() {
                        break;
                    }
                    argv = xcalloc(
                        1 as libc::c_int as size_t,
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ) as *mut *mut libc::c_char;
                    value = xstrndup(
                        cp.offset(1 as libc::c_int as isize),
                        end.wrapping_offset_from(cp.offset(1 as libc::c_int as isize))
                            as libc::c_long as size_t,
                    );
                    let ref mut fresh7 = *argv.offset(0 as libc::c_int as isize);
                    *fresh7 = format_expand1(es, value);
                    free(value as *mut libc::c_void);
                    argc = 1 as libc::c_int;
                    format_add_modifier(
                        &mut list,
                        count,
                        &mut c,
                        1 as libc::c_int as size_t,
                        argv,
                        argc,
                    );
                    cp = end
                } else {
                    /* Multiple arguments with a wrapper character. */
                    last[0 as libc::c_int as usize] = *cp.offset(1 as libc::c_int as isize);
                    cp = cp.offset(1);
                    loop {
                        if *cp.offset(0 as libc::c_int as isize) as libc::c_int
                            == last[0 as libc::c_int as usize] as libc::c_int
                            && format_is_end(*cp.offset(1 as libc::c_int as isize)) != 0
                        {
                            cp = cp.offset(1);
                            break;
                        } else {
                            end = format_skip(
                                cp.offset(1 as libc::c_int as isize),
                                last.as_mut_ptr(),
                            );
                            if end.is_null() {
                                break;
                            }
                            cp = cp.offset(1);
                            argv = xreallocarray(
                                argv as *mut libc::c_void,
                                (argc + 1 as libc::c_int) as size_t,
                                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            ) as *mut *mut libc::c_char;
                            value = xstrndup(
                                cp,
                                end.wrapping_offset_from(cp) as libc::c_long as size_t,
                            );
                            let fresh8 = argc;
                            argc = argc + 1;
                            let ref mut fresh9 = *argv.offset(fresh8 as isize);
                            *fresh9 = format_expand1(es, value);
                            free(value as *mut libc::c_void);
                            cp = end;
                            if !(format_is_end(*cp.offset(0 as libc::c_int as isize)) == 0) {
                                break;
                            }
                        }
                    }
                    format_add_modifier(
                        &mut list,
                        count,
                        &mut c,
                        1 as libc::c_int as size_t,
                        argv,
                        argc,
                    );
                }
            }
        }
    }
    if *cp as libc::c_int != ':' as i32 {
        format_free_modifiers(list, *count);
        *count = 0 as libc::c_int as u_int;
        return 0 as *mut format_modifier;
    }
    *s = cp.offset(1 as libc::c_int as isize);
    return list;
}
/* Match against an fnmatch(3) pattern or regular expression. */
unsafe extern "C" fn format_match(
    mut fm: *mut format_modifier,
    mut pattern: *const libc::c_char,
    mut text: *const libc::c_char,
) -> *mut libc::c_char {
    let mut s: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    let mut r: regex_t = regex_t {
        buffer: 0 as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *mut libc::c_char,
        translate: 0 as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut flags: libc::c_int = 0 as libc::c_int;
    if (*fm).argc >= 1 as libc::c_int {
        s = *(*fm).argv.offset(0 as libc::c_int as isize)
    }
    if strchr(s, 'r' as i32).is_null() {
        if !strchr(s, 'i' as i32).is_null() {
            flags |= (1 as libc::c_int) << 4 as libc::c_int
        }
        if fnmatch(pattern, text, flags) != 0 as libc::c_int {
            return xstrdup(b"0\x00" as *const u8 as *const libc::c_char);
        }
    } else {
        flags = 1 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int;
        if !strchr(s, 'i' as i32).is_null() {
            flags |= (1 as libc::c_int) << 1 as libc::c_int
        }
        if regcomp(&mut r, pattern, flags) != 0 as libc::c_int {
            return xstrdup(b"0\x00" as *const u8 as *const libc::c_char);
        }
        if regexec(
            &mut r,
            text,
            0 as libc::c_int as size_t,
            0 as *mut regmatch_t,
            0 as libc::c_int,
        ) != 0 as libc::c_int
        {
            regfree(&mut r);
            return xstrdup(b"0\x00" as *const u8 as *const libc::c_char);
        }
        regfree(&mut r);
    }
    return xstrdup(b"1\x00" as *const u8 as *const libc::c_char);
}
/* Perform substitution in string. */
unsafe extern "C" fn format_sub(
    mut fm: *mut format_modifier,
    mut text: *const libc::c_char,
    mut pattern: *const libc::c_char,
    mut with: *const libc::c_char,
) -> *mut libc::c_char {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: libc::c_int = 1 as libc::c_int;
    if (*fm).argc >= 3 as libc::c_int
        && !strchr(*(*fm).argv.offset(2 as libc::c_int as isize), 'i' as i32).is_null()
    {
        flags |= (1 as libc::c_int) << 1 as libc::c_int
    }
    value = regsub(pattern, with, text, flags);
    if value.is_null() {
        return xstrdup(text);
    }
    return value;
}
/* Search inside pane. */
unsafe extern "C" fn format_search(
    mut fm: *mut format_modifier,
    mut wp: *mut window_pane,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ignore: libc::c_int = 0 as libc::c_int;
    let mut regex: libc::c_int = 0 as libc::c_int;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*fm).argc >= 1 as libc::c_int {
        if !strchr(*(*fm).argv.offset(0 as libc::c_int as isize), 'i' as i32).is_null() {
            ignore = 1 as libc::c_int
        }
        if !strchr(*(*fm).argv.offset(0 as libc::c_int as isize), 'r' as i32).is_null() {
            regex = 1 as libc::c_int
        }
    }
    xasprintf(
        &mut value as *mut *mut libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        window_pane_search(wp, s, regex, ignore),
    );
    return value;
}
/* Loop over sessions. */
unsafe extern "C" fn format_loop_sessions(
    mut es: *mut format_expand_state,
    mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ft: *mut format_tree = (*es).ft;
    let mut c: *mut client = (*ft).client;
    let mut item: *mut crate::cmd_queue::cmdq_item = (*ft).item;
    let mut nft: *mut format_tree = 0 as *mut format_tree;
    let mut next: format_expand_state = format_expand_state {
        ft: 0 as *mut format_tree,
        loop_0: 0,
        time: 0,
        flags: 0,
    };
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valuelen: size_t = 0;
    let mut s: *mut session = 0 as *mut session;
    value = xcalloc(1 as libc::c_int as size_t, 1 as libc::c_int as size_t) as *mut libc::c_char;
    valuelen = 1 as libc::c_int as size_t;
    s = sessions_RB_MINMAX(&mut sessions, -(1 as libc::c_int));
    while !s.is_null() {
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"format_loop_sessions\x00"))
                .as_ptr(),
            b"session loop: $%u\x00" as *const u8 as *const libc::c_char,
            (*s).id,
        );
        nft = format_create(c, item, 0 as libc::c_int, (*ft).flags);
        format_defaults(
            next.ft,
            (*ft).c,
            s,
            0 as *mut winlink,
            0 as *mut window_pane,
        );
        format_copy_state(&mut next, es, 0 as libc::c_int);
        next.ft = nft;
        expanded = format_expand1(&mut next, fmt);
        format_free(next.ft);
        valuelen = (valuelen as libc::c_ulong).wrapping_add(strlen(expanded)) as size_t as size_t;
        value = xrealloc(value as *mut libc::c_void, valuelen) as *mut libc::c_char;
        strlcat(value, expanded, valuelen);
        free(expanded as *mut libc::c_void);
        s = sessions_RB_NEXT(s)
    }
    return value;
}
/* Loop over windows. */
unsafe extern "C" fn format_loop_windows(
    mut es: *mut format_expand_state,
    mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ft: *mut format_tree = (*es).ft;
    let mut c: *mut client = (*ft).client;
    let mut item: *mut crate::cmd_queue::cmdq_item = (*ft).item;
    let mut nft: *mut format_tree = 0 as *mut format_tree;
    let mut next: format_expand_state = format_expand_state {
        ft: 0 as *mut format_tree,
        loop_0: 0,
        time: 0,
        flags: 0,
    };
    let mut all: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut active: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut use_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valuelen: size_t = 0;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut w: *mut window = 0 as *mut window;
    if (*ft).s.is_null() {
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"format_loop_windows\x00"))
                .as_ptr(),
            b"window loop but no session\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    if format_choose(es, fmt, &mut all, &mut active, 0 as libc::c_int) != 0 as libc::c_int {
        all = xstrdup(fmt);
        active = 0 as *mut libc::c_char
    }
    value = xcalloc(1 as libc::c_int as size_t, 1 as libc::c_int as size_t) as *mut libc::c_char;
    valuelen = 1 as libc::c_int as size_t;
    wl = winlinks_RB_MINMAX(&mut (*(*ft).s).windows, -(1 as libc::c_int));
    while !wl.is_null() {
        w = (*wl).window;
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"format_loop_windows\x00"))
                .as_ptr(),
            b"window loop: %u @%u\x00" as *const u8 as *const libc::c_char,
            (*wl).idx,
            (*w).id,
        );
        if !active.is_null() && wl == (*(*ft).s).curw {
            use_0 = active
        } else {
            use_0 = all
        }
        nft = format_create(
            c,
            item,
            (0x40000000 as libc::c_uint | (*w).id) as libc::c_int,
            (*ft).flags,
        );
        format_defaults(nft, (*ft).c, (*ft).s, wl, 0 as *mut window_pane);
        format_copy_state(&mut next, es, 0 as libc::c_int);
        next.ft = nft;
        expanded = format_expand1(&mut next, use_0);
        format_free(nft);
        valuelen = (valuelen as libc::c_ulong).wrapping_add(strlen(expanded)) as size_t as size_t;
        value = xrealloc(value as *mut libc::c_void, valuelen) as *mut libc::c_char;
        strlcat(value, expanded, valuelen);
        free(expanded as *mut libc::c_void);
        wl = winlinks_RB_NEXT(wl)
    }
    free(active as *mut libc::c_void);
    free(all as *mut libc::c_void);
    return value;
}
/* Loop over panes. */
unsafe extern "C" fn format_loop_panes(
    mut es: *mut format_expand_state,
    mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ft: *mut format_tree = (*es).ft;
    let mut c: *mut client = (*ft).client;
    let mut item: *mut crate::cmd_queue::cmdq_item = (*ft).item;
    let mut nft: *mut format_tree = 0 as *mut format_tree;
    let mut next: format_expand_state = format_expand_state {
        ft: 0 as *mut format_tree,
        loop_0: 0,
        time: 0,
        flags: 0,
    };
    let mut all: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut active: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut use_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valuelen: size_t = 0;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if (*ft).w.is_null() {
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"format_loop_panes\x00"))
                .as_ptr(),
            b"pane loop but no window\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    if format_choose(es, fmt, &mut all, &mut active, 0 as libc::c_int) != 0 as libc::c_int {
        all = xstrdup(fmt);
        active = 0 as *mut libc::c_char
    }
    value = xcalloc(1 as libc::c_int as size_t, 1 as libc::c_int as size_t) as *mut libc::c_char;
    valuelen = 1 as libc::c_int as size_t;
    wp = (*(*ft).w).panes.tqh_first;
    while !wp.is_null() {
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"format_loop_panes\x00"))
                .as_ptr(),
            b"pane loop: %%%u\x00" as *const u8 as *const libc::c_char,
            (*wp).id,
        );
        if !active.is_null() && wp == (*(*ft).w).active {
            use_0 = active
        } else {
            use_0 = all
        }
        nft = format_create(
            c,
            item,
            (0x80000000 as libc::c_uint | (*wp).id) as libc::c_int,
            (*ft).flags,
        );
        format_defaults(nft, (*ft).c, (*ft).s, (*ft).wl, wp);
        format_copy_state(&mut next, es, 0 as libc::c_int);
        next.ft = nft;
        expanded = format_expand1(&mut next, use_0);
        format_free(nft);
        valuelen = (valuelen as libc::c_ulong).wrapping_add(strlen(expanded)) as size_t as size_t;
        value = xrealloc(value as *mut libc::c_void, valuelen) as *mut libc::c_char;
        strlcat(value, expanded, valuelen);
        free(expanded as *mut libc::c_void);
        wp = (*wp).entry.tqe_next
    }
    free(active as *mut libc::c_void);
    free(all as *mut libc::c_void);
    return value;
}
unsafe extern "C" fn format_replace_expression(
    mut mexp: *mut format_modifier,
    mut es: *mut format_expand_state,
    mut copy: *const libc::c_char,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut argc: libc::c_int = (*mexp).argc;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut endch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut use_fp: libc::c_int = 0 as libc::c_int;
    let mut prec: u_int = 0 as libc::c_int as u_int;
    let mut mleft: libc::c_double = 0.;
    let mut mright: libc::c_double = 0.;
    let mut result: libc::c_double = 0.;
    let mut operator: C2RustUnnamed_38 = ADD;
    if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b"+\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        operator = ADD;
        current_block = 11307063007268554308;
    } else if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b"-\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        operator = SUBTRACT;
        current_block = 11307063007268554308;
    } else if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b"*\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        operator = MULTIPLY;
        current_block = 11307063007268554308;
    } else if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b"/\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        operator = DIVIDE;
        current_block = 11307063007268554308;
    } else if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b"%\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        || strcmp(
            *(*mexp).argv.offset(0 as libc::c_int as isize),
            b"m\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        operator = MODULUS;
        current_block = 11307063007268554308;
    } else if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b"==\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        operator = EQUAL;
        current_block = 11307063007268554308;
    } else if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b"!=\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        operator = NOT_EQUAL;
        current_block = 11307063007268554308;
    } else if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b">\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        operator = GREATER_THAN;
        current_block = 11307063007268554308;
    } else if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b"<\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        operator = LESS_THAN;
        current_block = 11307063007268554308;
    } else if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b">=\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        operator = GREATER_THAN_EQUAL;
        current_block = 11307063007268554308;
    } else if strcmp(
        *(*mexp).argv.offset(0 as libc::c_int as isize),
        b"<=\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        operator = LESS_THAN_EQUAL;
        current_block = 11307063007268554308;
    } else {
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"format_replace_expression\x00",
            ))
            .as_ptr(),
            b"expression has no valid operator: \'%s\'\x00" as *const u8 as *const libc::c_char,
            *(*mexp).argv.offset(0 as libc::c_int as isize),
        );
        current_block = 2045625362422199595;
    }
    match current_block {
        11307063007268554308 => {
            /* The second argument may be flags. */
            if argc >= 2 as libc::c_int
                && !strchr(*(*mexp).argv.offset(1 as libc::c_int as isize), 'f' as i32).is_null()
            {
                use_fp = 1 as libc::c_int;
                prec = 2 as libc::c_int as u_int
            }
            /* The third argument may be precision. */
            if argc >= 3 as libc::c_int {
                prec = strtonum(
                    *(*mexp).argv.offset(2 as libc::c_int as isize),
                    (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_longlong,
                    2147483647 as libc::c_int as libc::c_longlong,
                    &mut errstr,
                ) as u_int;
                if !errstr.is_null() {
                    format_log1(
                        es,
                        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                            b"format_replace_expression\x00",
                        ))
                        .as_ptr(),
                        b"expression precision %s: %s\x00" as *const u8 as *const libc::c_char,
                        errstr,
                        *(*mexp).argv.offset(2 as libc::c_int as isize),
                    );
                    current_block = 2045625362422199595;
                } else {
                    current_block = 2891135413264362348;
                }
            } else {
                current_block = 2891135413264362348;
            }
            match current_block {
                2045625362422199595 => {}
                _ => {
                    if format_choose(es, copy, &mut left, &mut right, 1 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        format_log1(
                            es,
                            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                                b"format_replace_expression\x00",
                            ))
                            .as_ptr(),
                            b"expression syntax error\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        mleft = strtod(left, &mut endch);
                        if *endch as libc::c_int != '\u{0}' as i32 {
                            format_log1(
                                es,
                                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                                    b"format_replace_expression\x00",
                                ))
                                .as_ptr(),
                                b"expression left side is invalid: %s\x00" as *const u8
                                    as *const libc::c_char,
                                left,
                            );
                        } else {
                            mright = strtod(right, &mut endch);
                            if *endch as libc::c_int != '\u{0}' as i32 {
                                format_log1(
                                    es,
                                    (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                                        b"format_replace_expression\x00",
                                    ))
                                    .as_ptr(),
                                    b"expression right side is invalid: %s\x00" as *const u8
                                        as *const libc::c_char,
                                    right,
                                );
                            } else {
                                if use_fp == 0 {
                                    mleft = mleft as libc::c_longlong as libc::c_double;
                                    mright = mright as libc::c_longlong as libc::c_double
                                }
                                format_log1(
                                    es,
                                    (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                                        b"format_replace_expression\x00",
                                    ))
                                    .as_ptr(),
                                    b"expression left side is: %.*f\x00" as *const u8
                                        as *const libc::c_char,
                                    prec,
                                    mleft,
                                );
                                format_log1(
                                    es,
                                    (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                                        b"format_replace_expression\x00",
                                    ))
                                    .as_ptr(),
                                    b"expression right side is:  %.*f\x00" as *const u8
                                        as *const libc::c_char,
                                    prec,
                                    mright,
                                );
                                match operator as libc::c_uint {
                                    0 => result = mleft + mright,
                                    1 => result = mleft - mright,
                                    2 => result = mleft * mright,
                                    3 => result = mleft / mright,
                                    4 => result = fmod(mleft, mright),
                                    5 => {
                                        result = (fabs(mleft - mright) < 1e-9f64) as libc::c_int
                                            as libc::c_double
                                    }
                                    6 => {
                                        result = (fabs(mleft - mright) > 1e-9f64) as libc::c_int
                                            as libc::c_double
                                    }
                                    7 => result = (mleft > mright) as libc::c_int as libc::c_double,
                                    8 => {
                                        result = (mleft >= mright) as libc::c_int as libc::c_double
                                    }
                                    9 => result = (mleft < mright) as libc::c_int as libc::c_double,
                                    10 => {
                                        result = (mleft > mright) as libc::c_int as libc::c_double
                                    }
                                    _ => {}
                                }
                                if use_fp != 0 {
                                    xasprintf(
                                        &mut value as *mut *mut libc::c_char,
                                        b"%.*f\x00" as *const u8 as *const libc::c_char,
                                        prec,
                                        result,
                                    );
                                } else {
                                    xasprintf(
                                        &mut value as *mut *mut libc::c_char,
                                        b"%.*f\x00" as *const u8 as *const libc::c_char,
                                        prec,
                                        result as libc::c_longlong as libc::c_double,
                                    );
                                }
                                format_log1(
                                    es,
                                    (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                                        b"format_replace_expression\x00",
                                    ))
                                    .as_ptr(),
                                    b"expression result is %s\x00" as *const u8
                                        as *const libc::c_char,
                                    value,
                                );
                                free(right as *mut libc::c_void);
                                free(left as *mut libc::c_void);
                                return value;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    free(right as *mut libc::c_void);
    free(left as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
/* Replace a key. */
unsafe extern "C" fn format_replace(
    mut es: *mut format_expand_state,
    mut key: *const libc::c_char,
    mut keylen: size_t,
    mut buf: *mut *mut libc::c_char,
    mut len: *mut size_t,
    mut off: *mut size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ft: *mut format_tree = (*es).ft;
    let mut wp: *mut window_pane = (*ft).wp;
    let mut errptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut copy: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut marker: *const libc::c_char = 0 as *const libc::c_char;
    let mut time_format: *const libc::c_char = 0 as *const libc::c_char;
    let mut copy0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut condition: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut left: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut right: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valuelen: size_t = 0;
    let mut modifiers: libc::c_int = 0 as libc::c_int;
    let mut limit: libc::c_int = 0 as libc::c_int;
    let mut width: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut list: *mut format_modifier = 0 as *mut format_modifier;
    let mut cmp: *mut format_modifier = 0 as *mut format_modifier;
    let mut search: *mut format_modifier = 0 as *mut format_modifier;
    let mut sub: *mut *mut format_modifier = 0 as *mut *mut format_modifier;
    let mut mexp: *mut format_modifier = 0 as *mut format_modifier;
    let mut fm: *mut format_modifier = 0 as *mut format_modifier;
    let mut i: u_int = 0;
    let mut count: u_int = 0;
    let mut nsub: u_int = 0 as libc::c_int as u_int;
    let mut next: format_expand_state = format_expand_state {
        ft: 0 as *mut format_tree,
        loop_0: 0,
        time: 0,
        flags: 0,
    };
    /* Make a copy of the key. */
    copy0 = xstrndup(key, keylen);
    copy = copy0;
    /* Process modifier list. */
    list = format_build_modifiers(es, &mut copy, &mut count);
    i = 0 as libc::c_int as u_int;
    while i < count {
        fm = &mut *list.offset(i as isize) as *mut format_modifier;
        if format_logging(ft) != 0 {
            format_log1(
                es,
                (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_replace\x00"))
                    .as_ptr(),
                b"modifier %u is %s\x00" as *const u8 as *const libc::c_char,
                i,
                (*fm).modifier.as_mut_ptr(),
            );
            j = 0 as libc::c_int;
            while j < (*fm).argc {
                format_log1(
                    es,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"format_replace\x00",
                    ))
                    .as_ptr(),
                    b"modifier %u argument %d: %s\x00" as *const u8 as *const libc::c_char,
                    i,
                    j,
                    *(*fm).argv.offset(j as isize),
                );
                j += 1
            }
        }
        if (*fm).size == 1 as libc::c_int as libc::c_uint {
            match (*fm).modifier[0 as libc::c_int as usize] as libc::c_int {
                109 | 60 | 62 => cmp = fm,
                67 => search = fm,
                115 => {
                    if !((*fm).argc < 2 as libc::c_int) {
                        sub = xreallocarray(
                            sub as *mut libc::c_void,
                            nsub.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                            ::std::mem::size_of::<*mut format_modifier>() as libc::c_ulong,
                        ) as *mut *mut format_modifier;
                        let fresh10 = nsub;
                        nsub = nsub.wrapping_add(1);
                        let ref mut fresh11 = *sub.offset(fresh10 as isize);
                        *fresh11 = fm
                    }
                }
                61 => {
                    if !((*fm).argc < 1 as libc::c_int) {
                        limit = strtonum(
                            *(*fm).argv.offset(0 as libc::c_int as isize),
                            (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_longlong,
                            2147483647 as libc::c_int as libc::c_longlong,
                            &mut errptr,
                        ) as libc::c_int;
                        if !errptr.is_null() {
                            limit = 0 as libc::c_int
                        }
                        if (*fm).argc >= 2 as libc::c_int
                            && !(*(*fm).argv.offset(1 as libc::c_int as isize)).is_null()
                        {
                            marker = *(*fm).argv.offset(1 as libc::c_int as isize)
                        }
                    }
                }
                112 => {
                    if !((*fm).argc < 1 as libc::c_int) {
                        width = strtonum(
                            *(*fm).argv.offset(0 as libc::c_int as isize),
                            (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_longlong,
                            2147483647 as libc::c_int as libc::c_longlong,
                            &mut errptr,
                        ) as libc::c_int;
                        if !errptr.is_null() {
                            width = 0 as libc::c_int
                        }
                    }
                }
                101 => {
                    if !((*fm).argc < 1 as libc::c_int || (*fm).argc > 3 as libc::c_int) {
                        mexp = fm
                    }
                }
                108 => modifiers |= 0x10 as libc::c_int,
                98 => modifiers |= 0x2 as libc::c_int,
                100 => modifiers |= 0x4 as libc::c_int,
                110 => modifiers |= 0x800 as libc::c_int,
                116 => {
                    modifiers |= 0x1 as libc::c_int;
                    if !((*fm).argc < 1 as libc::c_int) {
                        if !strchr(*(*fm).argv.offset(0 as libc::c_int as isize), 'p' as i32)
                            .is_null()
                        {
                            modifiers |= 0x400 as libc::c_int
                        } else if (*fm).argc >= 2 as libc::c_int
                            && !strchr(*(*fm).argv.offset(0 as libc::c_int as isize), 'f' as i32)
                                .is_null()
                        {
                            time_format =
                                format_strip(*(*fm).argv.offset(1 as libc::c_int as isize))
                        }
                    }
                }
                113 => modifiers |= 0x8 as libc::c_int,
                69 => modifiers |= 0x20 as libc::c_int,
                84 => modifiers |= 0x40 as libc::c_int,
                83 => modifiers |= 0x80 as libc::c_int,
                87 => modifiers |= 0x100 as libc::c_int,
                80 => modifiers |= 0x200 as libc::c_int,
                _ => {}
            }
        } else if (*fm).size == 2 as libc::c_int as libc::c_uint {
            if strcmp(
                (*fm).modifier.as_mut_ptr(),
                b"||\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    (*fm).modifier.as_mut_ptr(),
                    b"&&\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    (*fm).modifier.as_mut_ptr(),
                    b"==\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    (*fm).modifier.as_mut_ptr(),
                    b"!=\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    (*fm).modifier.as_mut_ptr(),
                    b">=\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                || strcmp(
                    (*fm).modifier.as_mut_ptr(),
                    b"<=\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                cmp = fm
            }
        }
        i = i.wrapping_add(1)
    }
    /* Is this a literal string? */
    if modifiers & 0x10 as libc::c_int != 0 {
        value = xstrdup(copy)
    } else {
        /* Is this a loop, comparison or condition? */
        if modifiers & 0x80 as libc::c_int != 0 {
            value = format_loop_sessions(es, copy);
            if value.is_null() {
                current_block = 4023792313809996375;
            } else {
                current_block = 10449397189332721644;
            }
        } else if modifiers & 0x100 as libc::c_int != 0 {
            value = format_loop_windows(es, copy);
            if value.is_null() {
                current_block = 4023792313809996375;
            } else {
                current_block = 10449397189332721644;
            }
        } else if modifiers & 0x200 as libc::c_int != 0 {
            value = format_loop_panes(es, copy);
            if value.is_null() {
                current_block = 4023792313809996375;
            } else {
                current_block = 10449397189332721644;
            }
        } else if !search.is_null() {
            /* Search in pane. */
            new = format_expand1(es, copy);
            if wp.is_null() {
                format_log1(
                    es,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"format_replace\x00",
                    ))
                    .as_ptr(),
                    b"search \'%s\' but no pane\x00" as *const u8 as *const libc::c_char,
                    new,
                );
                value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
            } else {
                format_log1(
                    es,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"format_replace\x00",
                    ))
                    .as_ptr(),
                    b"search \'%s\' pane %%%u\x00" as *const u8 as *const libc::c_char,
                    new,
                    (*wp).id,
                );
                value = format_search(fm, wp, new)
            }
            free(new as *mut libc::c_void);
            current_block = 10449397189332721644;
        } else if !cmp.is_null() {
            /* Comparison of left and right. */
            if format_choose(es, copy, &mut left, &mut right, 1 as libc::c_int) != 0 as libc::c_int
            {
                format_log1(
                    es,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"format_replace\x00",
                    ))
                    .as_ptr(),
                    b"compare %s syntax error: %s\x00" as *const u8 as *const libc::c_char,
                    (*cmp).modifier.as_mut_ptr(),
                    copy,
                );
                current_block = 4023792313809996375;
            } else {
                format_log1(
                    es,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"format_replace\x00",
                    ))
                    .as_ptr(),
                    b"compare %s left is: %s\x00" as *const u8 as *const libc::c_char,
                    (*cmp).modifier.as_mut_ptr(),
                    left,
                );
                format_log1(
                    es,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"format_replace\x00",
                    ))
                    .as_ptr(),
                    b"compare %s right is: %s\x00" as *const u8 as *const libc::c_char,
                    (*cmp).modifier.as_mut_ptr(),
                    right,
                );
                if strcmp(
                    (*cmp).modifier.as_mut_ptr(),
                    b"||\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if format_true(left) != 0 || format_true(right) != 0 {
                        value = xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                    } else {
                        value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
                    }
                } else if strcmp(
                    (*cmp).modifier.as_mut_ptr(),
                    b"&&\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if format_true(left) != 0 && format_true(right) != 0 {
                        value = xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                    } else {
                        value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
                    }
                } else if strcmp(
                    (*cmp).modifier.as_mut_ptr(),
                    b"==\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if strcmp(left, right) == 0 as libc::c_int {
                        value = xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                    } else {
                        value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
                    }
                } else if strcmp(
                    (*cmp).modifier.as_mut_ptr(),
                    b"!=\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if strcmp(left, right) != 0 as libc::c_int {
                        value = xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                    } else {
                        value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
                    }
                } else if strcmp(
                    (*cmp).modifier.as_mut_ptr(),
                    b"<\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if strcmp(left, right) < 0 as libc::c_int {
                        value = xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                    } else {
                        value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
                    }
                } else if strcmp(
                    (*cmp).modifier.as_mut_ptr(),
                    b">\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if strcmp(left, right) > 0 as libc::c_int {
                        value = xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                    } else {
                        value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
                    }
                } else if strcmp(
                    (*cmp).modifier.as_mut_ptr(),
                    b"<=\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if strcmp(left, right) <= 0 as libc::c_int {
                        value = xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                    } else {
                        value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
                    }
                } else if strcmp(
                    (*cmp).modifier.as_mut_ptr(),
                    b">=\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if strcmp(left, right) >= 0 as libc::c_int {
                        value = xstrdup(b"1\x00" as *const u8 as *const libc::c_char)
                    } else {
                        value = xstrdup(b"0\x00" as *const u8 as *const libc::c_char)
                    }
                } else if strcmp(
                    (*cmp).modifier.as_mut_ptr(),
                    b"m\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    value = format_match(cmp, left, right)
                }
                free(right as *mut libc::c_void);
                free(left as *mut libc::c_void);
                current_block = 10449397189332721644;
            }
        } else if *copy as libc::c_int == '?' as i32 {
            /* Conditional: check first and choose second or third. */
            cp = format_skip(
                copy.offset(1 as libc::c_int as isize),
                b",\x00" as *const u8 as *const libc::c_char,
            );
            if cp.is_null() {
                format_log1(
                    es,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"format_replace\x00",
                    ))
                    .as_ptr(),
                    b"condition syntax error: %s\x00" as *const u8 as *const libc::c_char,
                    copy.offset(1 as libc::c_int as isize),
                );
                current_block = 4023792313809996375;
            } else {
                condition = xstrndup(
                    copy.offset(1 as libc::c_int as isize),
                    cp.wrapping_offset_from(copy.offset(1 as libc::c_int as isize)) as libc::c_long
                        as size_t,
                );
                format_log1(
                    es,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"format_replace\x00",
                    ))
                    .as_ptr(),
                    b"condition is: %s\x00" as *const u8 as *const libc::c_char,
                    condition,
                );
                found = format_find(ft, condition, modifiers, time_format);
                if found.is_null() {
                    /*
                     * If the condition not found, try to expand it. If
                     * the expansion doesn't have any effect, then assume
                     * false.
                     */
                    found = format_expand1(es, condition);
                    if strcmp(found, condition) == 0 as libc::c_int {
                        free(found as *mut libc::c_void);
                        found = xstrdup(b"\x00" as *const u8 as *const libc::c_char);
                        format_log1(
                            es,
                            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                b"format_replace\x00",
                            ))
                            .as_ptr(),
                            b"condition \'%s\' found: %s\x00" as *const u8 as *const libc::c_char,
                            condition,
                            found,
                        );
                    } else {
                        format_log1(
                            es,
                            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                b"format_replace\x00",
                            ))
                            .as_ptr(),
                            b"condition \'%s\' not found; assuming false\x00" as *const u8
                                as *const libc::c_char,
                            condition,
                        );
                    }
                } else {
                    format_log1(
                        es,
                        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                            b"format_replace\x00",
                        ))
                        .as_ptr(),
                        b"condition \'%s\' found\x00" as *const u8 as *const libc::c_char,
                        condition,
                    );
                }
                if format_choose(
                    es,
                    cp.offset(1 as libc::c_int as isize),
                    &mut left,
                    &mut right,
                    0 as libc::c_int,
                ) != 0 as libc::c_int
                {
                    format_log1(
                        es,
                        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                            b"format_replace\x00",
                        ))
                        .as_ptr(),
                        b"condition \'%s\' syntax error: %s\x00" as *const u8
                            as *const libc::c_char,
                        condition,
                        cp.offset(1 as libc::c_int as isize),
                    );
                    free(found as *mut libc::c_void);
                    current_block = 4023792313809996375;
                } else {
                    if format_true(found) != 0 {
                        format_log1(
                            es,
                            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                b"format_replace\x00",
                            ))
                            .as_ptr(),
                            b"condition \'%s\' is true\x00" as *const u8 as *const libc::c_char,
                            condition,
                        );
                        value = format_expand1(es, left)
                    } else {
                        format_log1(
                            es,
                            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                b"format_replace\x00",
                            ))
                            .as_ptr(),
                            b"condition \'%s\' is false\x00" as *const u8 as *const libc::c_char,
                            condition,
                        );
                        value = format_expand1(es, right)
                    }
                    free(right as *mut libc::c_void);
                    free(left as *mut libc::c_void);
                    free(condition as *mut libc::c_void);
                    free(found as *mut libc::c_void);
                    current_block = 10449397189332721644;
                }
            }
        } else {
            if !mexp.is_null() {
                value = format_replace_expression(mexp, es, copy);
                if value.is_null() {
                    value = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
                }
            } else if !strstr(copy, b"#{\x00" as *const u8 as *const libc::c_char).is_null() {
                format_log1(
                    es,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"format_replace\x00",
                    ))
                    .as_ptr(),
                    b"expanding inner format \'%s\'\x00" as *const u8 as *const libc::c_char,
                    copy,
                );
                value = format_expand1(es, copy)
            } else {
                value = format_find(ft, copy, modifiers, time_format);
                if value.is_null() {
                    format_log1(
                        es,
                        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                            b"format_replace\x00",
                        ))
                        .as_ptr(),
                        b"format \'%s\' not found\x00" as *const u8 as *const libc::c_char,
                        copy,
                    );
                    value = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
                } else {
                    format_log1(
                        es,
                        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                            b"format_replace\x00",
                        ))
                        .as_ptr(),
                        b"format \'%s\' found: %s\x00" as *const u8 as *const libc::c_char,
                        copy,
                        value,
                    );
                }
            }
            current_block = 10449397189332721644;
        }
        match current_block {
            10449397189332721644 => {}
            _ => {
                format_log1(
                    es,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"format_replace\x00",
                    ))
                    .as_ptr(),
                    b"failed %s\x00" as *const u8 as *const libc::c_char,
                    copy0,
                );
                free(sub as *mut libc::c_void);
                format_free_modifiers(list, count);
                free(copy0 as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
        }
    }
    /* Expand again if required. */
    if modifiers & 0x20 as libc::c_int != 0 {
        new = format_expand1(es, value);
        free(value as *mut libc::c_void);
        value = new
    } else if modifiers & 0x40 as libc::c_int != 0 {
        format_copy_state(&mut next, es, 0x1 as libc::c_int);
        new = format_expand1(&mut next, value);
        free(value as *mut libc::c_void);
        value = new
    }
    /* Perform substitution if any. */
    i = 0 as libc::c_int as u_int;
    while i < nsub {
        left = format_expand1(
            es,
            *(**sub.offset(i as isize))
                .argv
                .offset(0 as libc::c_int as isize),
        );
        right = format_expand1(
            es,
            *(**sub.offset(i as isize))
                .argv
                .offset(1 as libc::c_int as isize),
        );
        new = format_sub(*sub.offset(i as isize), value, left, right);
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_replace\x00"))
                .as_ptr(),
            b"substitute \'%s\' to \'%s\': %s\x00" as *const u8 as *const libc::c_char,
            left,
            right,
            new,
        );
        free(value as *mut libc::c_void);
        value = new;
        free(right as *mut libc::c_void);
        free(left as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    /* Truncate the value if needed. */
    if limit > 0 as libc::c_int {
        new = format_trim_left(value, limit as u_int);
        if !marker.is_null() && strcmp(new, value) != 0 as libc::c_int {
            free(value as *mut libc::c_void);
            xasprintf(
                &mut value as *mut *mut libc::c_char,
                b"%s%s\x00" as *const u8 as *const libc::c_char,
                new,
                marker,
            );
        } else {
            free(value as *mut libc::c_void);
            value = new
        }
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_replace\x00"))
                .as_ptr(),
            b"applied length limit %d: %s\x00" as *const u8 as *const libc::c_char,
            limit,
            value,
        );
    } else if limit < 0 as libc::c_int {
        new = format_trim_right(value, -limit as u_int);
        if !marker.is_null() && strcmp(new, value) != 0 as libc::c_int {
            free(value as *mut libc::c_void);
            xasprintf(
                &mut value as *mut *mut libc::c_char,
                b"%s%s\x00" as *const u8 as *const libc::c_char,
                marker,
                new,
            );
        } else {
            free(value as *mut libc::c_void);
            value = new
        }
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_replace\x00"))
                .as_ptr(),
            b"applied length limit %d: %s\x00" as *const u8 as *const libc::c_char,
            limit,
            value,
        );
    }
    /* Pad the value if needed. */
    if width > 0 as libc::c_int {
        new = utf8_padcstr(value, width as u_int);
        free(value as *mut libc::c_void);
        value = new;
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_replace\x00"))
                .as_ptr(),
            b"applied padding width %d: %s\x00" as *const u8 as *const libc::c_char,
            width,
            value,
        );
    } else if width < 0 as libc::c_int {
        new = utf8_rpadcstr(value, -width as u_int);
        free(value as *mut libc::c_void);
        value = new;
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_replace\x00"))
                .as_ptr(),
            b"applied padding width %d: %s\x00" as *const u8 as *const libc::c_char,
            width,
            value,
        );
    }
    /* Replace with the length if needed. */
    if modifiers & 0x800 as libc::c_int != 0 {
        xasprintf(
            &mut new as *mut *mut libc::c_char,
            b"%zu\x00" as *const u8 as *const libc::c_char,
            strlen(value),
        );
        free(value as *mut libc::c_void);
        value = new;
        format_log1(
            es,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_replace\x00"))
                .as_ptr(),
            b"replacing with length: %s\x00" as *const u8 as *const libc::c_char,
            new,
        );
    }
    /* Expand the buffer and copy in the value. */
    valuelen = strlen(value);
    while (*len).wrapping_sub(*off) < valuelen.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        *buf = xreallocarray(*buf as *mut libc::c_void, 2 as libc::c_int as size_t, *len)
            as *mut libc::c_char;
        *len = (*len as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    }
    memcpy(
        (*buf).offset(*off as isize) as *mut libc::c_void,
        value as *const libc::c_void,
        valuelen,
    );
    *off = (*off as libc::c_ulong).wrapping_add(valuelen) as size_t as size_t;
    format_log1(
        es,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_replace\x00")).as_ptr(),
        b"replaced \'%s\' with \'%s\'\x00" as *const u8 as *const libc::c_char,
        copy0,
        value,
    );
    free(value as *mut libc::c_void);
    free(sub as *mut libc::c_void);
    format_free_modifiers(list, count);
    free(copy0 as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* Expand keys in a template. */
unsafe extern "C" fn format_expand1(
    mut es: *mut format_expand_state,
    mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut ft: *mut format_tree = (*es).ft;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut off: size_t = 0;
    let mut len: size_t = 0;
    let mut n: size_t = 0;
    let mut outlen: size_t = 0;
    let mut ch: libc::c_int = 0;
    let mut brackets: libc::c_int = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut expanded: [libc::c_char; 8192] = [0; 8192];
    if fmt.is_null() || *fmt as libc::c_int == '\u{0}' as i32 {
        return xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    }
    if (*es).loop_0 == 10 as libc::c_int as libc::c_uint {
        return xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    }
    (*es).loop_0 = (*es).loop_0.wrapping_add(1);
    format_log1(
        es,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_expand1\x00")).as_ptr(),
        b"expanding format: %s\x00" as *const u8 as *const libc::c_char,
        fmt,
    );
    if (*es).flags & 0x1 as libc::c_int != 0 {
        if (*es).time == 0 as libc::c_int as libc::c_long {
            (*es).time = time(0 as *mut time_t)
        }
        tm = localtime(&mut (*es).time);
        if strftime(
            expanded.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            fmt,
            tm,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            format_log1(
                es,
                (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_expand1\x00"))
                    .as_ptr(),
                b"format is too long\x00" as *const u8 as *const libc::c_char,
            );
            return xstrdup(b"\x00" as *const u8 as *const libc::c_char);
        }
        if format_logging(ft) != 0 && strcmp(expanded.as_mut_ptr(), fmt) != 0 as libc::c_int {
            format_log1(
                es,
                (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_expand1\x00"))
                    .as_ptr(),
                b"after time expanded: %s\x00" as *const u8 as *const libc::c_char,
                expanded.as_mut_ptr(),
            );
        }
        fmt = expanded.as_mut_ptr()
    }
    len = 64 as libc::c_int as size_t;
    buf = xmalloc(len) as *mut libc::c_char;
    off = 0 as libc::c_int as size_t;
    while *fmt as libc::c_int != '\u{0}' as i32 {
        if *fmt as libc::c_int != '#' as i32 {
            while len.wrapping_sub(off) < 2 as libc::c_int as libc::c_ulong {
                buf = xreallocarray(buf as *mut libc::c_void, 2 as libc::c_int as size_t, len)
                    as *mut libc::c_char;
                len = (len as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            }
            let fresh12 = fmt;
            fmt = fmt.offset(1);
            let fresh13 = off;
            off = off.wrapping_add(1);
            *buf.offset(fresh13 as isize) = *fresh12
        } else {
            fmt = fmt.offset(1);
            let fresh14 = fmt;
            fmt = fmt.offset(1);
            ch = *fresh14 as u_char as libc::c_int;
            match ch {
                40 => {
                    brackets = 1 as libc::c_int;
                    ptr = fmt;
                    while *ptr as libc::c_int != '\u{0}' as i32 {
                        if *ptr as libc::c_int == '(' as i32 {
                            brackets += 1
                        }
                        if *ptr as libc::c_int == ')' as i32 && {
                            brackets -= 1;
                            (brackets) == 0 as libc::c_int
                        } {
                            break;
                        }
                        ptr = ptr.offset(1)
                    }
                    if *ptr as libc::c_int != ')' as i32 || brackets != 0 as libc::c_int {
                        break;
                    }
                    n = ptr.wrapping_offset_from(fmt) as libc::c_long as size_t;
                    name = xstrndup(fmt, n);
                    format_log1(
                        es,
                        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                            b"format_expand1\x00",
                        ))
                        .as_ptr(),
                        b"found #(): %s\x00" as *const u8 as *const libc::c_char,
                        name,
                    );
                    if (*ft).flags & 0x4 as libc::c_int != 0
                        || (*es).flags & 0x2 as libc::c_int != 0
                    {
                        out = xstrdup(b"\x00" as *const u8 as *const libc::c_char);
                        format_log1(
                            es,
                            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                b"format_expand1\x00",
                            ))
                            .as_ptr(),
                            b"#() is disabled\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        out = format_job_get(es, name);
                        format_log1(
                            es,
                            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                b"format_expand1\x00",
                            ))
                            .as_ptr(),
                            b"#() result: %s\x00" as *const u8 as *const libc::c_char,
                            out,
                        );
                    }
                    free(name as *mut libc::c_void);
                    outlen = strlen(out);
                    while len.wrapping_sub(off)
                        < outlen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    {
                        buf = xreallocarray(
                            buf as *mut libc::c_void,
                            2 as libc::c_int as size_t,
                            len,
                        ) as *mut libc::c_char;
                        len = (len as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            as size_t as size_t
                    }
                    memcpy(
                        buf.offset(off as isize) as *mut libc::c_void,
                        out as *const libc::c_void,
                        outlen,
                    );
                    off = (off as libc::c_ulong).wrapping_add(outlen) as size_t as size_t;
                    free(out as *mut libc::c_void);
                    fmt = fmt.offset(n.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                }
                123 => {
                    ptr = format_skip(
                        (fmt as *mut libc::c_char).offset(-(2 as libc::c_int as isize)),
                        b"}\x00" as *const u8 as *const libc::c_char,
                    );
                    if ptr.is_null() {
                        break;
                    }
                    n = ptr.wrapping_offset_from(fmt) as libc::c_long as size_t;
                    format_log1(
                        es,
                        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                            b"format_expand1\x00",
                        ))
                        .as_ptr(),
                        b"found #{}: %.*s\x00" as *const u8 as *const libc::c_char,
                        n as libc::c_int,
                        fmt,
                    );
                    if format_replace(es, fmt, n, &mut buf, &mut len, &mut off) != 0 as libc::c_int
                    {
                        break;
                    }
                    fmt = fmt.offset(n.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                }
                125 | 35 | 44 => {
                    format_log1(
                        es,
                        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                            b"format_expand1\x00",
                        ))
                        .as_ptr(),
                        b"found #%c\x00" as *const u8 as *const libc::c_char,
                        ch,
                    );
                    while len.wrapping_sub(off) < 2 as libc::c_int as libc::c_ulong {
                        buf = xreallocarray(
                            buf as *mut libc::c_void,
                            2 as libc::c_int as size_t,
                            len,
                        ) as *mut libc::c_char;
                        len = (len as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            as size_t as size_t
                    }
                    let fresh15 = off;
                    off = off.wrapping_add(1);
                    *buf.offset(fresh15 as isize) = ch as libc::c_char
                }
                _ => {
                    s = 0 as *const libc::c_char;
                    if ch >= 'A' as i32 && ch <= 'Z' as i32 {
                        s = format_upper[(ch - 'A' as i32) as usize]
                    } else if ch >= 'a' as i32 && ch <= 'z' as i32 {
                        s = format_lower[(ch - 'a' as i32) as usize]
                    }
                    if s.is_null() {
                        while len.wrapping_sub(off) < 3 as libc::c_int as libc::c_ulong {
                            buf = xreallocarray(
                                buf as *mut libc::c_void,
                                2 as libc::c_int as size_t,
                                len,
                            ) as *mut libc::c_char;
                            len = (len as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                as size_t as size_t
                        }
                        let fresh16 = off;
                        off = off.wrapping_add(1);
                        *buf.offset(fresh16 as isize) = '#' as i32 as libc::c_char;
                        let fresh17 = off;
                        off = off.wrapping_add(1);
                        *buf.offset(fresh17 as isize) = ch as libc::c_char
                    } else {
                        n = strlen(s);
                        format_log1(
                            es,
                            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                                b"format_expand1\x00",
                            ))
                            .as_ptr(),
                            b"found #%c: %s\x00" as *const u8 as *const libc::c_char,
                            ch,
                            s,
                        );
                        if format_replace(es, s, n, &mut buf, &mut len, &mut off)
                            != 0 as libc::c_int
                        {
                            break;
                        }
                    }
                }
            }
        }
    }
    *buf.offset(off as isize) = '\u{0}' as i32 as libc::c_char;
    format_log1(
        es,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"format_expand1\x00")).as_ptr(),
        b"result is: %s\x00" as *const u8 as *const libc::c_char,
        buf,
    );
    (*es).loop_0 = (*es).loop_0.wrapping_sub(1);
    return buf;
}
/* Expand keys in a template, passing through strftime first. */
#[no_mangle]
pub unsafe extern "C" fn format_expand_time(
    mut ft: *mut format_tree,
    mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut es: format_expand_state = format_expand_state {
        ft: 0 as *mut format_tree,
        loop_0: 0,
        time: 0,
        flags: 0,
    };
    memset(
        &mut es as *mut format_expand_state as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<format_expand_state>() as libc::c_ulong,
    );
    es.ft = ft;
    es.flags = 0x1 as libc::c_int;
    return format_expand1(&mut es, fmt);
}
/* Expand keys in a template. */
#[no_mangle]
pub unsafe extern "C" fn format_expand(
    mut ft: *mut format_tree,
    mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut es: format_expand_state = format_expand_state {
        ft: 0 as *mut format_tree,
        loop_0: 0,
        time: 0,
        flags: 0,
    };
    memset(
        &mut es as *mut format_expand_state as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<format_expand_state>() as libc::c_ulong,
    );
    es.ft = ft;
    es.flags = 0 as libc::c_int;
    return format_expand1(&mut es, fmt);
}
/* Expand a single string. */
#[no_mangle]
pub unsafe extern "C" fn format_single(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut fmt: *const libc::c_char,
    mut c: *mut client,
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
) -> *mut libc::c_char {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    ft = format_create_defaults(item, c, s, wl, wp);
    expanded = format_expand(ft, fmt);
    format_free(ft);
    return expanded;
}
/* Expand a single string using state. */
#[no_mangle]
pub unsafe extern "C" fn format_single_from_state(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut fmt: *const libc::c_char,
    mut c: *mut client,
    mut fs: *mut cmd_find_state,
) -> *mut libc::c_char {
    return format_single(item, fmt, c, (*fs).s, (*fs).wl, (*fs).wp);
}
/* Expand a single string using target. */
#[no_mangle]
pub unsafe extern "C" fn format_single_from_target(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut fmt: *const libc::c_char,
) -> *mut libc::c_char {
    let mut tc: *mut client = cmdq_get_target_client(item);
    return format_single_from_state(item, fmt, tc, cmdq_get_target(item));
}
/* Create and add defaults. */
#[no_mangle]
pub unsafe extern "C" fn format_create_defaults(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut c: *mut client,
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
) -> *mut format_tree {
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    if !item.is_null() {
        ft = format_create(
            cmdq_get_client(item),
            item,
            0 as libc::c_int,
            0 as libc::c_int,
        )
    } else {
        ft = format_create(0 as *mut client, item, 0 as libc::c_int, 0 as libc::c_int)
    }
    format_defaults(ft, c, s, wl, wp);
    return ft;
}
/* Create and add defaults using state. */
#[no_mangle]
pub unsafe extern "C" fn format_create_from_state(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut c: *mut client,
    mut fs: *mut cmd_find_state,
) -> *mut format_tree {
    return format_create_defaults(item, c, (*fs).s, (*fs).wl, (*fs).wp);
}
/* Create and add defaults using target. */
#[no_mangle]
pub unsafe extern "C" fn format_create_from_target(
    mut item: *mut crate::cmd_queue::cmdq_item,
) -> *mut format_tree {
    let mut tc: *mut client = cmdq_get_target_client(item);
    return format_create_from_state(item, tc, cmdq_get_target(item));
}
/* Set defaults for any of arguments that are not NULL. */
#[no_mangle]
pub unsafe extern "C" fn format_defaults(
    mut ft: *mut format_tree,
    mut c: *mut client,
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
) {
    let mut pb: *mut crate::paste::paste_buffer = 0 as *mut crate::paste::paste_buffer;
    if !c.is_null() && !(*c).name.is_null() {
        log_debug(
            b"%s: c=%s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"format_defaults\x00"))
                .as_ptr(),
            (*c).name,
        );
    } else {
        log_debug(
            b"%s: c=none\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"format_defaults\x00"))
                .as_ptr(),
        );
    }
    if !s.is_null() {
        log_debug(
            b"%s: s=$%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"format_defaults\x00"))
                .as_ptr(),
            (*s).id,
        );
    } else {
        log_debug(
            b"%s: s=none\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"format_defaults\x00"))
                .as_ptr(),
        );
    }
    if !wl.is_null() {
        log_debug(
            b"%s: wl=%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"format_defaults\x00"))
                .as_ptr(),
            (*wl).idx,
        );
    } else {
        log_debug(
            b"%s: wl=none\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"format_defaults\x00"))
                .as_ptr(),
        );
    }
    if !wp.is_null() {
        log_debug(
            b"%s: wp=%%%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"format_defaults\x00"))
                .as_ptr(),
            (*wp).id,
        );
    } else {
        log_debug(
            b"%s: wp=none\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"format_defaults\x00"))
                .as_ptr(),
        );
    }
    if !c.is_null() && !s.is_null() && (*c).session != s {
        log_debug(
            b"%s: session does not match\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"format_defaults\x00"))
                .as_ptr(),
        );
    }
    format_add(
        ft,
        b"session_format\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (s != 0 as *mut libc::c_void as *mut session) as libc::c_int,
    );
    format_add(
        ft,
        b"window_format\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (wl != 0 as *mut libc::c_void as *mut winlink) as libc::c_int,
    );
    format_add(
        ft,
        b"pane_format\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (wp != 0 as *mut libc::c_void as *mut window_pane) as libc::c_int,
    );
    if s.is_null() && !c.is_null() {
        s = (*c).session
    }
    if wl.is_null() && !s.is_null() {
        wl = (*s).curw
    }
    if wp.is_null() && !wl.is_null() {
        wp = (*(*wl).window).active
    }
    if !c.is_null() {
        format_defaults_client(ft, c);
    }
    if !s.is_null() {
        format_defaults_session(ft, s);
    }
    if !wl.is_null() {
        format_defaults_winlink(ft, wl);
    }
    if !wp.is_null() {
        format_defaults_pane(ft, wp);
    }
    pb = paste_get_top(0 as *mut *const libc::c_char);
    if !pb.is_null() {
        format_defaults_paste_buffer(ft, pb);
    };
}
/* Set default format keys for a session. */
unsafe extern "C" fn format_defaults_session(mut ft: *mut format_tree, mut s: *mut session) {
    let mut sg: *mut session_group = 0 as *mut session_group;
    (*ft).s = s;
    format_add(
        ft,
        b"session_name\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*s).name,
    );
    format_add(
        ft,
        b"session_path\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*s).cwd,
    );
    format_add(
        ft,
        b"session_windows\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        winlink_count(&mut (*s).windows),
    );
    format_add(
        ft,
        b"session_id\x00" as *const u8 as *const libc::c_char,
        b"$%u\x00" as *const u8 as *const libc::c_char,
        (*s).id,
    );
    sg = session_group_contains(s);
    format_add(
        ft,
        b"session_grouped\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (sg != 0 as *mut libc::c_void as *mut session_group) as libc::c_int,
    );
    if !sg.is_null() {
        format_add(
            ft,
            b"session_group\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*sg).name,
        );
        format_add(
            ft,
            b"session_group_size\x00" as *const u8 as *const libc::c_char,
            b"%u\x00" as *const u8 as *const libc::c_char,
            session_group_count(sg),
        );
        format_add(
            ft,
            b"session_group_attached\x00" as *const u8 as *const libc::c_char,
            b"%u\x00" as *const u8 as *const libc::c_char,
            session_group_attached_count(sg),
        );
        format_add(
            ft,
            b"session_group_many_attached\x00" as *const u8 as *const libc::c_char,
            b"%u\x00" as *const u8 as *const libc::c_char,
            (session_group_attached_count(sg) > 1 as libc::c_int as libc::c_uint) as libc::c_int,
        );
        format_add_cb(
            ft,
            b"session_group_list\x00" as *const u8 as *const libc::c_char,
            Some(
                format_cb_session_group_list
                    as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
            ),
        );
        format_add_cb(
            ft,
            b"session_group_attached_list\x00" as *const u8 as *const libc::c_char,
            Some(
                format_cb_session_group_attached_list
                    as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
            ),
        );
    }
    format_add_tv(
        ft,
        b"session_created\x00" as *const u8 as *const libc::c_char,
        &mut (*s).creation_time,
    );
    format_add_tv(
        ft,
        b"session_last_attached\x00" as *const u8 as *const libc::c_char,
        &mut (*s).last_attached_time,
    );
    format_add_tv(
        ft,
        b"session_activity\x00" as *const u8 as *const libc::c_char,
        &mut (*s).activity_time,
    );
    format_add(
        ft,
        b"session_attached\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*s).attached,
    );
    format_add(
        ft,
        b"session_many_attached\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*s).attached > 1 as libc::c_int as libc::c_uint) as libc::c_int,
    );
    format_add_cb(
        ft,
        b"session_attached_list\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_session_attached_list
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"session_alerts\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_session_alerts
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"session_stack\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_session_stack
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    if server_check_marked() != 0 && marked_pane.s == s {
        format_add(
            ft,
            b"session_marked\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        format_add(
            ft,
            b"session_marked\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/* Set default format keys for a client. */
unsafe extern "C" fn format_defaults_client(mut ft: *mut format_tree, mut c: *mut client) {
    let mut s: *mut session = 0 as *mut session;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut tty: *mut tty = &mut (*c).tty;
    if (*ft).s.is_null() {
        (*ft).s = (*c).session
    }
    (*ft).c = c;
    format_add(
        ft,
        b"client_name\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*c).name,
    );
    format_add(
        ft,
        b"client_pid\x00" as *const u8 as *const libc::c_char,
        b"%ld\x00" as *const u8 as *const libc::c_char,
        (*c).pid as libc::c_long,
    );
    format_add(
        ft,
        b"client_height\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*tty).sy,
    );
    format_add(
        ft,
        b"client_width\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*tty).sx,
    );
    format_add(
        ft,
        b"client_cell_width\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*tty).xpixel,
    );
    format_add(
        ft,
        b"client_cell_height\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*tty).ypixel,
    );
    format_add(
        ft,
        b"client_tty\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*c).ttyname,
    );
    format_add(
        ft,
        b"client_control_mode\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"client_termname\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*c).term_name,
    );
    format_add(
        ft,
        b"client_termfeatures\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        tty_get_features((*c).term_features),
    );
    if !(*c).term_type.is_null() {
        format_add(
            ft,
            b"client_termtype\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*c).term_type,
        );
    }
    format_add_tv(
        ft,
        b"client_created\x00" as *const u8 as *const libc::c_char,
        &mut (*c).creation_time,
    );
    format_add_tv(
        ft,
        b"client_activity\x00" as *const u8 as *const libc::c_char,
        &mut (*c).activity_time,
    );
    format_add(
        ft,
        b"client_written\x00" as *const u8 as *const libc::c_char,
        b"%zu\x00" as *const u8 as *const libc::c_char,
        (*c).written,
    );
    format_add(
        ft,
        b"client_discarded\x00" as *const u8 as *const libc::c_char,
        b"%zu\x00" as *const u8 as *const libc::c_char,
        (*c).discarded,
    );
    name = server_client_get_key_table(c);
    if strcmp((*(*c).keytable).name, name) == 0 as libc::c_int {
        format_add(
            ft,
            b"client_prefix\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    } else {
        format_add(
            ft,
            b"client_prefix\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    format_add(
        ft,
        b"client_key_table\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*(*c).keytable).name,
    );
    if (*c).flags & 0x10000 as libc::c_int as libc::c_ulong != 0 {
        format_add(
            ft,
            b"client_utf8\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    } else {
        format_add(
            ft,
            b"client_utf8\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if (*c).flags & 0x800 as libc::c_int as libc::c_ulong != 0 {
        format_add(
            ft,
            b"client_readonly\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    } else {
        format_add(
            ft,
            b"client_readonly\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    format_add(
        ft,
        b"client_flags\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        server_client_get_flags(c),
    );
    s = (*c).session;
    if !s.is_null() {
        format_add(
            ft,
            b"client_session\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*s).name,
        );
    }
    s = (*c).last_session;
    if !s.is_null() && session_alive(s) != 0 {
        format_add(
            ft,
            b"client_last_session\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*s).name,
        );
    };
}
/* Set default format keys for a window. */
#[no_mangle]
pub unsafe extern "C" fn format_defaults_window(mut ft: *mut format_tree, mut w: *mut window) {
    (*ft).w = w;
    format_add_tv(
        ft,
        b"window_activity\x00" as *const u8 as *const libc::c_char,
        &mut (*w).activity_time,
    );
    format_add(
        ft,
        b"window_id\x00" as *const u8 as *const libc::c_char,
        b"@%u\x00" as *const u8 as *const libc::c_char,
        (*w).id,
    );
    format_add(
        ft,
        b"window_name\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*w).name,
    );
    format_add(
        ft,
        b"window_width\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*w).sx,
    );
    format_add(
        ft,
        b"window_height\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*w).sy,
    );
    format_add(
        ft,
        b"window_cell_width\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*w).xpixel,
    );
    format_add(
        ft,
        b"window_cell_height\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*w).ypixel,
    );
    format_add_cb(
        ft,
        b"window_layout\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_layout
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"window_visible_layout\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_visible_layout
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add(
        ft,
        b"window_panes\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        window_count_panes(w),
    );
    format_add(
        ft,
        b"window_zoomed_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*w).flags & 0x8 as libc::c_int != 0) as libc::c_int,
    );
}
/* Set default format keys for a winlink. */
unsafe extern "C" fn format_defaults_winlink(mut ft: *mut format_tree, mut wl: *mut winlink) {
    let mut c: *mut client = (*ft).c;
    let mut s: *mut session = (*wl).session;
    let mut w: *mut window = (*wl).window;
    let mut flag: libc::c_int = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    if (*ft).w.is_null() {
        format_defaults_window(ft, w);
    }
    (*ft).wl = wl;
    if !c.is_null() {
        flag = tty_window_offset(&mut (*c).tty, &mut ox, &mut oy, &mut sx, &mut sy);
        format_add(
            ft,
            b"window_bigger\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            flag,
        );
        if flag != 0 {
            format_add(
                ft,
                b"window_offset_x\x00" as *const u8 as *const libc::c_char,
                b"%u\x00" as *const u8 as *const libc::c_char,
                ox,
            );
            format_add(
                ft,
                b"window_offset_y\x00" as *const u8 as *const libc::c_char,
                b"%u\x00" as *const u8 as *const libc::c_char,
                oy,
            );
        }
    }
    format_add(
        ft,
        b"window_index\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (*wl).idx,
    );
    format_add_cb(
        ft,
        b"window_stack_index\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_stack_index
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add(
        ft,
        b"window_flags\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        window_printable_flags(wl),
    );
    format_add(
        ft,
        b"window_active\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (wl == (*s).curw) as libc::c_int,
    );
    format_add_cb(
        ft,
        b"window_active_sessions\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_active_sessions
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"window_active_sessions_list\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_active_sessions_list
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"window_active_clients\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_active_clients
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"window_active_clients_list\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_active_clients_list
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add(
        ft,
        b"window_start_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (wl == winlinks_RB_MINMAX(&mut (*s).windows, -(1 as libc::c_int))) as libc::c_int,
    );
    format_add(
        ft,
        b"window_end_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (wl == winlinks_RB_MINMAX(&mut (*s).windows, 1 as libc::c_int)) as libc::c_int,
    );
    if server_check_marked() != 0 && marked_pane.wl == wl {
        format_add(
            ft,
            b"window_marked_flag\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        format_add(
            ft,
            b"window_marked_flag\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    format_add(
        ft,
        b"window_bell_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wl).flags & 0x1 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"window_activity_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wl).flags & 0x2 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"window_silence_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wl).flags & 0x4 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"window_last_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (wl == (*s).lastw.tqh_first) as libc::c_int,
    );
    format_add(
        ft,
        b"window_linked\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        session_is_linked(s, (*wl).window),
    );
    format_add_cb(
        ft,
        b"window_linked_sessions_list\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_window_linked_sessions_list
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add(
        ft,
        b"window_linked_sessions\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*(*wl).window).references,
    );
}
/* Set default format keys for a window pane. */
#[no_mangle]
pub unsafe extern "C" fn format_defaults_pane(mut ft: *mut format_tree, mut wp: *mut window_pane) {
    let mut w: *mut window = (*wp).window;
    let mut gd: *mut grid = (*wp).base.grid;
    let mut status: libc::c_int = (*wp).status;
    let mut idx: u_int = 0;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    if (*ft).w.is_null() {
        format_defaults_window(ft, w);
    }
    (*ft).wp = wp;
    format_add(
        ft,
        b"history_size\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*gd).hsize,
    );
    format_add(
        ft,
        b"history_limit\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*gd).hlimit,
    );
    format_add_cb(
        ft,
        b"history_bytes\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_history_bytes
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"history_all_bytes\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_history_all_bytes
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add(
        ft,
        b"pane_written\x00" as *const u8 as *const libc::c_char,
        b"%zu\x00" as *const u8 as *const libc::c_char,
        (*wp).written,
    );
    format_add(
        ft,
        b"pane_skipped\x00" as *const u8 as *const libc::c_char,
        b"%zu\x00" as *const u8 as *const libc::c_char,
        (*wp).skipped,
    );
    if window_pane_index(wp, &mut idx) != 0 as libc::c_int {
        fatalx(b"index not found\x00" as *const u8 as *const libc::c_char);
    }
    format_add(
        ft,
        b"pane_index\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        idx,
    );
    format_add(
        ft,
        b"pane_width\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wp).sx,
    );
    format_add(
        ft,
        b"pane_height\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wp).sy,
    );
    format_add(
        ft,
        b"pane_title\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*wp).base.title,
    );
    if !(*wp).base.path.is_null() {
        format_add(
            ft,
            b"pane_path\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*wp).base.path,
        );
    }
    format_add(
        ft,
        b"pane_id\x00" as *const u8 as *const libc::c_char,
        b"%%%u\x00" as *const u8 as *const libc::c_char,
        (*wp).id,
    );
    format_add(
        ft,
        b"pane_active\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (wp == (*w).active) as libc::c_int,
    );
    format_add(
        ft,
        b"pane_input_off\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).flags & 0x40 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"pane_pipe\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).pipe_fd != -(1 as libc::c_int)) as libc::c_int,
    );
    if (*wp).flags & 0x200 as libc::c_int != 0 && status & 0x7f as libc::c_int == 0 as libc::c_int {
        format_add(
            ft,
            b"pane_dead_status\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            (status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
        );
    }
    if !(*wp).flags & 0x800 as libc::c_int != 0 {
        format_add(
            ft,
            b"pane_dead\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            ((*wp).fd == -(1 as libc::c_int)) as libc::c_int,
        );
    } else {
        format_add(
            ft,
            b"pane_dead\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    format_add(
        ft,
        b"pane_last\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (wp == (*w).last) as libc::c_int,
    );
    if server_check_marked() != 0 && marked_pane.wp == wp {
        format_add(
            ft,
            b"pane_marked\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        format_add(
            ft,
            b"pane_marked\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    format_add(
        ft,
        b"pane_marked_set\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        server_check_marked(),
    );
    format_add(
        ft,
        b"pane_left\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wp).xoff,
    );
    format_add(
        ft,
        b"pane_top\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wp).yoff,
    );
    format_add(
        ft,
        b"pane_right\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wp)
            .xoff
            .wrapping_add((*wp).sx)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    format_add(
        ft,
        b"pane_bottom\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wp)
            .yoff
            .wrapping_add((*wp).sy)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    format_add(
        ft,
        b"pane_at_left\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).xoff == 0 as libc::c_int as libc::c_uint) as libc::c_int,
    );
    format_add_cb(
        ft,
        b"pane_at_top\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_pane_at_top as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add(
        ft,
        b"pane_at_right\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).xoff.wrapping_add((*wp).sx) == (*w).sx) as libc::c_int,
    );
    format_add_cb(
        ft,
        b"pane_at_bottom\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_pane_at_bottom
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    wme = (*wp).modes.tqh_first;
    if !wme.is_null() {
        format_add(
            ft,
            b"pane_mode\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*(*wme).mode).name,
        );
        if (*(*wme).mode).formats.is_some() {
            (*(*wme).mode).formats.expect("non-null function pointer")(wme, ft);
        }
    }
    format_add_cb(
        ft,
        b"pane_in_mode\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_pane_in_mode
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add(
        ft,
        b"pane_synchronized\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (options_get_number(
            (*w).options,
            b"synchronize-panes\x00" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int,
    );
    if !(*wp).searchstr.is_null() {
        format_add(
            ft,
            b"pane_search_string\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*wp).searchstr,
        );
    }
    format_add(
        ft,
        b"pane_tty\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*wp).tty.as_mut_ptr(),
    );
    format_add(
        ft,
        b"pane_pid\x00" as *const u8 as *const libc::c_char,
        b"%ld\x00" as *const u8 as *const libc::c_char,
        (*wp).pid as libc::c_long,
    );
    format_add_cb(
        ft,
        b"pane_start_command\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_start_command
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"pane_current_command\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_current_command
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"pane_current_path\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_current_path
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add(
        ft,
        b"cursor_x\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wp).base.cx,
    );
    format_add(
        ft,
        b"cursor_y\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wp).base.cy,
    );
    format_add_cb(
        ft,
        b"cursor_character\x00" as *const u8 as *const libc::c_char,
        Some(
            format_cb_cursor_character
                as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char,
        ),
    );
    format_add(
        ft,
        b"scroll_region_upper\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wp).base.rupper,
    );
    format_add(
        ft,
        b"scroll_region_lower\x00" as *const u8 as *const libc::c_char,
        b"%u\x00" as *const u8 as *const libc::c_char,
        (*wp).base.rlower,
    );
    format_add(
        ft,
        b"alternate_on\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.saved_grid != 0 as *mut libc::c_void as *mut grid) as libc::c_int,
    );
    if (*wp).base.saved_cx
        != (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
    {
        format_add(
            ft,
            b"alternate_saved_x\x00" as *const u8 as *const libc::c_char,
            b"%u\x00" as *const u8 as *const libc::c_char,
            (*wp).base.saved_cx,
        );
    }
    if (*wp).base.saved_cy
        != (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
    {
        format_add(
            ft,
            b"alternate_saved_y\x00" as *const u8 as *const libc::c_char,
            b"%u\x00" as *const u8 as *const libc::c_char,
            (*wp).base.saved_cy,
        );
    }
    format_add(
        ft,
        b"cursor_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x1 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"insert_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x2 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"keypad_cursor_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x4 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"keypad_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x8 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"wrap_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x10 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"origin_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x2000 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"mouse_any_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & (0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int) != 0)
            as libc::c_int,
    );
    format_add(
        ft,
        b"mouse_standard_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x20 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"mouse_button_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x40 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"mouse_all_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x1000 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"mouse_utf8_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x100 as libc::c_int != 0) as libc::c_int,
    );
    format_add(
        ft,
        b"mouse_sgr_flag\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*wp).base.mode & 0x200 as libc::c_int != 0) as libc::c_int,
    );
    format_add_cb(
        ft,
        b"pane_tabs\x00" as *const u8 as *const libc::c_char,
        Some(format_cb_pane_tabs as unsafe extern "C" fn(_: *mut format_tree) -> *mut libc::c_char),
    );
}
/* Set default format keys for paste buffer. */
#[no_mangle]
pub unsafe extern "C" fn format_defaults_paste_buffer(
    mut ft: *mut format_tree,
    mut pb: *mut crate::paste::paste_buffer,
) {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut size: size_t = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    tv.tv_sec = tv.tv_usec;
    tv.tv_sec = paste_buffer_created(pb);
    paste_buffer_data(pb, &mut size);
    format_add(
        ft,
        b"buffer_size\x00" as *const u8 as *const libc::c_char,
        b"%zu\x00" as *const u8 as *const libc::c_char,
        size,
    );
    format_add(
        ft,
        b"buffer_name\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        paste_buffer_name(pb),
    );
    format_add_tv(
        ft,
        b"buffer_created\x00" as *const u8 as *const libc::c_char,
        &mut tv,
    );
    s = paste_make_sample(pb);
    format_add(
        ft,
        b"buffer_sample\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        s,
    );
    free(s as *mut libc::c_void);
}
