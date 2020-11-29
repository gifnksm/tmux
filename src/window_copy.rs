use crate::{
    grid::Cell as GridCell,
    utf8::{Utf8Char, Utf8Data},
};
use ::c2rust_bitfields;
use ::libc;

extern "C" {
    pub type re_dfa_t;
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
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
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
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
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
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
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut crate::options::options;
    #[no_mangle]
    fn get_timer() -> uint64_t;
    #[no_mangle]
    fn paste_buffer_data(_: *mut crate::paste::paste_buffer, _: *mut size_t)
        -> *const libc::c_char;
    #[no_mangle]
    fn paste_get_top(_: *mut *const libc::c_char) -> *mut crate::paste::paste_buffer;
    #[no_mangle]
    fn paste_add(_: *const libc::c_char, _: *mut libc::c_char, _: size_t);
    #[no_mangle]
    fn paste_set(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn format_get_pane(_: *mut crate::format::format_tree) -> *mut window_pane;
    #[no_mangle]
    fn format_add(
        _: *mut crate::format::format_tree,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn format_add_cb(_: *mut crate::format::format_tree, _: *const libc::c_char, _: format_cb);
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
    fn format_grid_word(_: *mut grid, _: u_int, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    fn format_grid_line(_: *mut grid, _: u_int) -> *mut libc::c_char;
    #[no_mangle]
    fn options_get_string(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
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
    fn job_get_event(_: *mut crate::job::job) -> *mut bufferevent;
    #[no_mangle]
    fn tty_acs_get(_: *mut tty, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn cmd_mouse_at(
        _: *mut window_pane,
        _: *mut mouse_event,
        _: *mut u_int,
        _: *mut u_int,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn cmd_mouse_pane(
        _: *mut mouse_event,
        _: *mut *mut session,
        _: *mut *mut winlink,
    ) -> *mut window_pane;
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn grid_peek_line(_: *mut grid, _: u_int) -> *const grid_line;
    #[no_mangle]
    fn grid_get_cell(_: *mut grid, _: u_int, _: u_int, _: *mut crate::grid::Cell);
    #[no_mangle]
    fn grid_get_line(_: *mut grid, _: u_int) -> *mut grid_line;
    #[no_mangle]
    fn grid_duplicate_lines(_: *mut grid, _: u_int, _: *mut grid, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_wrap_position(_: *mut grid, _: u_int, _: u_int, _: *mut u_int, _: *mut u_int);
    #[no_mangle]
    fn grid_unwrap_position(_: *mut grid, _: *mut u_int, _: *mut u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn grid_line_length(_: *mut grid, _: u_int) -> u_int;
    #[no_mangle]
    fn screen_write_start_pane(_: *mut screen_write_ctx, _: *mut window_pane, _: *mut screen);
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut screen);
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_strlen(_: *const libc::c_char, _: ...) -> size_t;
    #[no_mangle]
    fn screen_write_puts(
        _: *mut screen_write_ctx,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn screen_write_nputs(
        _: *mut screen_write_ctx,
        _: ssize_t,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn screen_write_vnputs(
        _: *mut screen_write_ctx,
        _: ssize_t,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    );
    #[no_mangle]
    fn screen_write_putc(_: *mut screen_write_ctx, _: *const crate::grid::Cell, _: u_char);
    #[no_mangle]
    fn screen_write_insertline(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_deleteline(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_cursormove(
        _: *mut screen_write_ctx,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_write_linefeed(_: *mut screen_write_ctx, _: libc::c_int, _: u_int);
    #[no_mangle]
    fn screen_write_carriagereturn(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_cell(_: *mut screen_write_ctx, _: *const crate::grid::Cell);
    #[no_mangle]
    fn screen_write_setselection(_: *mut screen_write_ctx, _: *mut u_char, _: u_int);
    #[no_mangle]
    fn screen_init(_: *mut screen, _: u_int, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_free(_: *mut screen);
    #[no_mangle]
    fn screen_resize(_: *mut screen, _: u_int, _: u_int, _: libc::c_int);
    #[no_mangle]
    fn screen_resize_cursor(
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_set_selection(
        _: *mut screen,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: u_int,
        _: libc::c_int,
        _: *mut crate::grid::Cell,
    );
    #[no_mangle]
    fn screen_clear_selection(_: *mut screen);
    #[no_mangle]
    fn screen_hide_selection(_: *mut screen);
    #[no_mangle]
    fn notify_pane(_: *const libc::c_char, _: *mut window_pane);
    #[no_mangle]
    fn window_pane_reset_mode(_: *mut window_pane);
    #[no_mangle]
    fn utf8_copy(_: *mut Utf8Data, _: *const Utf8Data);
    #[no_mangle]
    fn utf8_to_data(_: Utf8Char, _: *mut Utf8Data);
    #[no_mangle]
    fn utf8_cstrhas(_: *const libc::c_char, _: *const Utf8Data) -> libc::c_int;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn style_apply(
        _: *mut crate::grid::Cell,
        _: *mut crate::options::options,
        _: *const libc::c_char,
        _: *mut crate::format::format_tree,
    );
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
pub type __int32_t = libc::c_int;
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
pub struct screen_write_ctx {
    pub wp: *mut window_pane,
    pub s: *mut screen,
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
    pub s: *mut screen,
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
pub type format_cb =
    Option<unsafe extern "C" fn(_: *mut crate::format::format_tree) -> *mut libc::c_char>;
pub type job_update_cb = Option<unsafe extern "C" fn(_: *mut crate::job::job) -> ()>;
pub type job_complete_cb = Option<unsafe extern "C" fn(_: *mut crate::job::job) -> ()>;
pub type job_free_cb = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
/*
 * Copy mode's visible screen (the "screen" field) is filled from one of two
 * sources: the original contents of the pane (used when we actually enter via
 * the "copy-mode" command, to copy the contents of the current pane), or else
 * a series of lines containing the output from an output-writing tmux command
 * (such as any of the "show-*" or "list-*" commands).
 *
 * In either case, the full content of the copy-mode grid is pointed at by the
 * "backing" field, and is copied into "screen" as needed (that is, when
 * scrolling occurs). When copy-mode is backed by a pane, backing points
 * directly at that pane's screen structure (&wp->base); when backed by a list
 * of output-lines from a command, it points at a newly-allocated screen
 * structure (which is deallocated when the mode ends).
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_copy_mode_data {
    pub screen: screen,
    pub backing: *mut screen,
    pub backing_written: libc::c_int,
    pub viewmode: libc::c_int,
    pub oy: u_int,
    pub selx: u_int,
    pub sely: u_int,
    pub endselx: u_int,
    pub endsely: u_int,
    pub cursordrag: C2RustUnnamed_34,
    pub modekeys: libc::c_int,
    pub lineflag: C2RustUnnamed_33,
    pub rectflag: libc::c_int,
    pub scroll_exit: libc::c_int,
    pub hide_position: libc::c_int,
    pub selflag: C2RustUnnamed_32,
    pub ws: *const libc::c_char,
    pub dx: u_int,
    pub dy: u_int,
    pub selrx: u_int,
    pub selry: u_int,
    pub endselrx: u_int,
    pub endselry: u_int,
    pub cx: u_int,
    pub cy: u_int,
    pub lastcx: u_int,
    pub lastsx: u_int,
    pub mx: u_int,
    pub my: u_int,
    pub showmark: libc::c_int,
    pub searchtype: libc::c_int,
    pub searchregex: libc::c_int,
    pub searchstr: *mut libc::c_char,
    pub searchmark: *mut u_char,
    pub searchcount: libc::c_int,
    pub searchmore: libc::c_int,
    pub searchthis: libc::c_int,
    pub searchx: libc::c_int,
    pub searchy: libc::c_int,
    pub searcho: libc::c_int,
    pub searchgen: u_char,
    pub timeout: libc::c_int,
    pub jumptype: libc::c_int,
    pub jumpchar: libc::c_char,
    pub dragtimer: event,
}
pub type C2RustUnnamed_32 = libc::c_uint;
pub const SEL_LINE: C2RustUnnamed_32 = 2;
pub const SEL_WORD: C2RustUnnamed_32 = 1;
pub const SEL_CHAR: C2RustUnnamed_32 = 0;
pub type C2RustUnnamed_33 = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: C2RustUnnamed_33 = 2;
pub const LINE_SEL_LEFT_RIGHT: C2RustUnnamed_33 = 1;
pub const LINE_SEL_NONE: C2RustUnnamed_33 = 0;
pub type C2RustUnnamed_34 = libc::c_uint;
pub const CURSORDRAG_SEL: C2RustUnnamed_34 = 2;
pub const CURSORDRAG_ENDSEL: C2RustUnnamed_34 = 1;
pub const CURSORDRAG_NONE: C2RustUnnamed_34 = 0;
pub const WINDOW_COPY_CMD_REDRAW: window_copy_cmd_action = 1;
pub type window_copy_cmd_action = libc::c_uint;
pub const WINDOW_COPY_CMD_CANCEL: window_copy_cmd_action = 2;
pub const WINDOW_COPY_CMD_NOTHING: window_copy_cmd_action = 0;
pub const WINDOW_COPY_CMD_CLEAR_NEVER: window_copy_cmd_clear = 1;
pub type window_copy_cmd_clear = libc::c_uint;
pub const WINDOW_COPY_CMD_CLEAR_EMACS_ONLY: window_copy_cmd_clear = 2;
pub const WINDOW_COPY_CMD_CLEAR_ALWAYS: window_copy_cmd_clear = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_copy_cmd_state {
    pub wme: *mut window_mode_entry,
    pub args: *mut args,
    pub m: *mut mouse_event,
    pub c: *mut client,
    pub s: *mut session,
    pub wl: *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_35 {
    pub command: *const libc::c_char,
    pub minargs: libc::c_int,
    pub maxargs: libc::c_int,
    pub clear: window_copy_cmd_clear,
    pub f: Option<unsafe extern "C" fn(_: *mut window_copy_cmd_state) -> window_copy_cmd_action>,
}
pub const WINDOW_COPY_REL_POS_ON_SCREEN: C2RustUnnamed_38 = 1;
pub const WINDOW_COPY_REL_POS_BELOW: C2RustUnnamed_38 = 2;
pub const WINDOW_COPY_REL_POS_ABOVE: C2RustUnnamed_38 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_36 {
    pub d: *const libc::c_char,
    pub dlen: size_t,
    pub allocated: libc::c_int,
}
pub const WINDOW_COPY_SEARCHDOWN: C2RustUnnamed_37 = 2;
pub const WINDOW_COPY_SEARCHUP: C2RustUnnamed_37 = 1;
pub const WINDOW_COPY_JUMPTOFORWARD: C2RustUnnamed_37 = 5;
pub const WINDOW_COPY_JUMPTOBACKWARD: C2RustUnnamed_37 = 6;
pub const WINDOW_COPY_JUMPBACKWARD: C2RustUnnamed_37 = 4;
pub const WINDOW_COPY_JUMPFORWARD: C2RustUnnamed_37 = 3;
pub const WINDOW_COPY_OFF: C2RustUnnamed_37 = 0;
pub type C2RustUnnamed_37 = libc::c_uint;
pub type C2RustUnnamed_38 = libc::c_uint;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub static mut window_copy_mode: window_mode = {
    {
        let mut init = window_mode {
            name: b"copy-mode\x00" as *const u8 as *const libc::c_char,
            default_format: 0 as *const libc::c_char,
            init: Some(
                window_copy_init
                    as unsafe extern "C" fn(
                        _: *mut window_mode_entry,
                        _: *mut cmd_find_state,
                        _: *mut args,
                    ) -> *mut screen,
            ),
            free: Some(window_copy_free as unsafe extern "C" fn(_: *mut window_mode_entry) -> ()),
            resize: Some(
                window_copy_resize
                    as unsafe extern "C" fn(_: *mut window_mode_entry, _: u_int, _: u_int) -> (),
            ),
            key: None,
            key_table: Some(
                window_copy_key_table
                    as unsafe extern "C" fn(_: *mut window_mode_entry) -> *const libc::c_char,
            ),
            command: Some(
                window_copy_command
                    as unsafe extern "C" fn(
                        _: *mut window_mode_entry,
                        _: *mut client,
                        _: *mut session,
                        _: *mut winlink,
                        _: *mut args,
                        _: *mut mouse_event,
                    ) -> (),
            ),
            formats: Some(
                window_copy_formats
                    as unsafe extern "C" fn(
                        _: *mut window_mode_entry,
                        _: *mut crate::format::format_tree,
                    ) -> (),
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut window_view_mode: window_mode = {
    {
        let mut init = window_mode {
            name: b"view-mode\x00" as *const u8 as *const libc::c_char,
            default_format: 0 as *const libc::c_char,
            init: Some(
                window_copy_view_init
                    as unsafe extern "C" fn(
                        _: *mut window_mode_entry,
                        _: *mut cmd_find_state,
                        _: *mut args,
                    ) -> *mut screen,
            ),
            free: Some(window_copy_free as unsafe extern "C" fn(_: *mut window_mode_entry) -> ()),
            resize: Some(
                window_copy_resize
                    as unsafe extern "C" fn(_: *mut window_mode_entry, _: u_int, _: u_int) -> (),
            ),
            key: None,
            key_table: Some(
                window_copy_key_table
                    as unsafe extern "C" fn(_: *mut window_mode_entry) -> *const libc::c_char,
            ),
            command: Some(
                window_copy_command
                    as unsafe extern "C" fn(
                        _: *mut window_mode_entry,
                        _: *mut client,
                        _: *mut session,
                        _: *mut winlink,
                        _: *mut args,
                        _: *mut mouse_event,
                    ) -> (),
            ),
            formats: Some(
                window_copy_formats
                    as unsafe extern "C" fn(
                        _: *mut window_mode_entry,
                        _: *mut crate::format::format_tree,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn window_copy_scroll_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut wme: *mut window_mode_entry = arg as *mut window_mode_entry;
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 50000 as libc::c_int as __suseconds_t,
        };
        init
    };
    event_del(&mut (*data).dragtimer);
    if (*wp).modes.tqh_first != wme {
        return;
    }
    if (*data).cy == 0 as libc::c_int as libc::c_uint {
        event_add(&mut (*data).dragtimer, &mut tv);
        window_copy_cursor_up(wme, 1 as libc::c_int);
    } else if (*data).cy
        == (*(*data).screen.grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        event_add(&mut (*data).dragtimer, &mut tv);
        window_copy_cursor_down(wme, 1 as libc::c_int);
    };
}
unsafe extern "C" fn window_copy_clone_screen(
    mut src: *mut screen,
    mut hint: *mut screen,
    mut cx: *mut u_int,
    mut cy: *mut u_int,
    mut trim: libc::c_int,
) -> *mut screen {
    let mut dst: *mut screen = 0 as *mut screen;
    let mut gl: *const grid_line = 0 as *const grid_line;
    let mut sy: u_int = 0;
    let mut wx: u_int = 0;
    let mut wy: u_int = 0;
    let mut reflow: libc::c_int = 0;
    dst = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<screen>() as libc::c_ulong,
    ) as *mut screen;
    sy = (*(*src).grid).hsize.wrapping_add((*(*src).grid).sy);
    if trim != 0 {
        while sy > (*(*src).grid).hsize {
            gl = grid_peek_line(
                (*src).grid,
                sy.wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
            if (*gl).cellused != 0 as libc::c_int as libc::c_uint {
                break;
            }
            sy = sy.wrapping_sub(1)
        }
    }
    log_debug(
        b"%s: target screen is %ux%u, source %ux%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"window_copy_clone_screen\x00"))
            .as_ptr(),
        (*(*src).grid).sx,
        sy,
        (*(*hint).grid).sx,
        (*(*src).grid).hsize.wrapping_add((*(*src).grid).sy),
    );
    screen_init(dst, (*(*src).grid).sx, sy, (*(*src).grid).hlimit);
    /*
     * Ensure history is on for the backing grid so lines are not deleted
     * during resizing.
     */
    (*(*dst).grid).flags |= 0x1 as libc::c_int;
    grid_duplicate_lines(
        (*dst).grid,
        0 as libc::c_int as u_int,
        (*src).grid,
        0 as libc::c_int as u_int,
        sy,
    );
    (*(*dst).grid).sy = sy.wrapping_sub((*(*src).grid).hsize);
    (*(*dst).grid).hsize = (*(*src).grid).hsize;
    (*(*dst).grid).hscrolled = (*(*src).grid).hscrolled;
    if (*src).cy
        > (*(*dst).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        (*dst).cx = 0 as libc::c_int as u_int;
        (*dst).cy = (*(*dst).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else {
        (*dst).cx = (*src).cx;
        (*dst).cy = (*src).cy
    }
    if !cx.is_null() && !cy.is_null() {
        *cx = (*dst).cx;
        *cy = (*(*dst).grid).hsize.wrapping_add((*dst).cy);
        reflow = ((*(*hint).grid).sx != (*(*dst).grid).sx) as libc::c_int
    } else {
        reflow = 0 as libc::c_int
    }
    if reflow != 0 {
        grid_wrap_position((*dst).grid, *cx, *cy, &mut wx, &mut wy);
    }
    screen_resize_cursor(
        dst,
        (*(*hint).grid).sx,
        (*(*hint).grid).sy,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if reflow != 0 {
        grid_unwrap_position((*dst).grid, cx, cy, wx, wy);
    }
    return dst;
}
unsafe extern "C" fn window_copy_common_init(
    mut wme: *mut window_mode_entry,
) -> *mut window_copy_mode_data {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = 0 as *mut window_copy_mode_data;
    let mut base: *mut screen = &mut (*wp).base;
    data = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<window_copy_mode_data>() as libc::c_ulong,
    ) as *mut window_copy_mode_data;
    (*wme).data = data as *mut libc::c_void;
    (*data).cursordrag = CURSORDRAG_NONE;
    (*data).lineflag = LINE_SEL_NONE;
    (*data).selflag = SEL_CHAR;
    if !(*wp).searchstr.is_null() {
        (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
        (*data).searchregex = (*wp).searchregex;
        (*data).searchstr = xstrdup((*wp).searchstr)
    } else {
        (*data).searchtype = WINDOW_COPY_OFF as libc::c_int;
        (*data).searchregex = 0 as libc::c_int;
        (*data).searchstr = 0 as *mut libc::c_char
    }
    (*data).searcho = -(1 as libc::c_int);
    (*data).searchy = (*data).searcho;
    (*data).searchx = (*data).searchy;
    (*data).jumptype = WINDOW_COPY_OFF as libc::c_int;
    (*data).jumpchar = '\u{0}' as i32 as libc::c_char;
    screen_init(
        &mut (*data).screen,
        (*(*base).grid).sx,
        (*(*base).grid).sy,
        0 as libc::c_int as u_int,
    );
    (*data).modekeys = options_get_number(
        (*(*wp).window).options,
        b"mode-keys\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    event_set(
        &mut (*data).dragtimer,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            window_copy_scroll_timer
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        wme as *mut libc::c_void,
    );
    return data;
}
unsafe extern "C" fn window_copy_init(
    mut wme: *mut window_mode_entry,
    mut _fs: *mut cmd_find_state,
    mut args: *mut args,
) -> *mut screen {
    let mut wp: *mut window_pane = (*wme).swp;
    let mut data: *mut window_copy_mode_data = 0 as *mut window_copy_mode_data;
    let mut base: *mut screen = &mut (*wp).base;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut i: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    data = window_copy_common_init(wme);
    (*data).backing = window_copy_clone_screen(
        base,
        &mut (*data).screen,
        &mut cx,
        &mut cy,
        ((*wme).swp != (*wme).wp) as libc::c_int,
    );
    (*data).cx = cx;
    if cy < (*(*(*data).backing).grid).hsize {
        (*data).cy = 0 as libc::c_int as u_int;
        (*data).oy = (*(*(*data).backing).grid).hsize.wrapping_sub(cy)
    } else {
        (*data).cy = cy.wrapping_sub((*(*(*data).backing).grid).hsize);
        (*data).oy = 0 as libc::c_int as u_int
    }
    (*data).scroll_exit = args_has(args, 'e' as i32 as u_char);
    (*data).hide_position = args_has(args, 'H' as i32 as u_char);
    (*data).screen.cx = (*data).cx;
    (*data).screen.cy = (*data).cy;
    (*data).mx = (*data).cx;
    (*data).my = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).showmark = 0 as libc::c_int;
    screen_write_start(&mut ctx, &mut (*data).screen);
    i = 0 as libc::c_int as u_int;
    while i < (*(*data).screen.grid).sy {
        window_copy_write_line(wme, &mut ctx, i);
        i = i.wrapping_add(1)
    }
    screen_write_cursormove(
        &mut ctx,
        (*data).cx as libc::c_int,
        (*data).cy as libc::c_int,
        0 as libc::c_int,
    );
    screen_write_stop(&mut ctx);
    return &mut (*data).screen;
}
unsafe extern "C" fn window_copy_view_init(
    mut wme: *mut window_mode_entry,
    mut _fs: *mut cmd_find_state,
    mut _args: *mut args,
) -> *mut screen {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = 0 as *mut window_copy_mode_data;
    let mut base: *mut screen = &mut (*wp).base;
    let mut s: *mut screen = 0 as *mut screen;
    data = window_copy_common_init(wme);
    (*data).viewmode = 1 as libc::c_int;
    s = xmalloc(::std::mem::size_of::<screen>() as libc::c_ulong) as *mut screen;
    (*data).backing = s;
    screen_init(
        s,
        (*(*base).grid).sx,
        (*(*base).grid).sy,
        (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint),
    );
    (*data).mx = (*data).cx;
    (*data).my = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).showmark = 0 as libc::c_int;
    return &mut (*data).screen;
}
unsafe extern "C" fn window_copy_free(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    event_del(&mut (*data).dragtimer);
    free((*data).searchmark as *mut libc::c_void);
    free((*data).searchstr as *mut libc::c_void);
    screen_free((*data).backing);
    free((*data).backing as *mut libc::c_void);
    screen_free(&mut (*data).screen);
    free(data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_add(
    mut wp: *mut window_pane,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    window_copy_vadd(wp, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_vadd(
    mut wp: *mut window_pane,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut wme: *mut window_mode_entry = (*wp).modes.tqh_first;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut backing: *mut screen = (*data).backing;
    let mut back_ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
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
    let mut old_hsize: u_int = 0;
    let mut old_cy: u_int = 0;
    memcpy(
        &mut gc as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    old_hsize = (*(*(*data).backing).grid).hsize;
    screen_write_start(&mut back_ctx, backing);
    if (*data).backing_written != 0 {
        /*
         * On the second or later line, do a CRLF before writing
         * (so it's on a new line).
         */
        screen_write_carriagereturn(&mut back_ctx);
        screen_write_linefeed(&mut back_ctx, 0 as libc::c_int, 8 as libc::c_int as u_int);
    } else {
        (*data).backing_written = 1 as libc::c_int
    }
    old_cy = (*backing).cy;
    screen_write_vnputs(
        &mut back_ctx,
        0 as libc::c_int as ssize_t,
        &mut gc,
        fmt,
        ap.as_va_list(),
    );
    screen_write_stop(&mut back_ctx);
    (*data).oy = ((*data).oy as libc::c_uint)
        .wrapping_add((*(*(*data).backing).grid).hsize.wrapping_sub(old_hsize))
        as u_int as u_int;
    screen_write_start_pane(&mut ctx, wp, &mut (*data).screen);
    /*
     * If the history has changed, draw the top line.
     * (If there's any history at all, it has changed.)
     */
    if (*(*(*data).backing).grid).hsize != 0 {
        window_copy_redraw_lines(wme, 0 as libc::c_int as u_int, 1 as libc::c_int as u_int);
    }
    /* Write the new lines. */
    window_copy_redraw_lines(
        wme,
        old_cy,
        (*backing)
            .cy
            .wrapping_sub(old_cy)
            .wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    screen_write_stop(&mut ctx);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_pageup(mut wp: *mut window_pane, mut half_page: libc::c_int) {
    window_copy_pageup1((*wp).modes.tqh_first, half_page);
}
unsafe extern "C" fn window_copy_pageup1(
    mut wme: *mut window_mode_entry,
    mut half_page: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut n: u_int = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    ox = window_copy_find_length(wme, oy);
    if (*data).cx != ox {
        (*data).lastcx = (*data).cx;
        (*data).lastsx = ox
    }
    (*data).cx = (*data).lastcx;
    n = 1 as libc::c_int as u_int;
    if (*(*s).grid).sy > 2 as libc::c_int as libc::c_uint {
        if half_page != 0 {
            n = (*(*s).grid)
                .sy
                .wrapping_div(2 as libc::c_int as libc::c_uint)
        } else {
            n = (*(*s).grid)
                .sy
                .wrapping_sub(2 as libc::c_int as libc::c_uint)
        }
    }
    if (*data).oy.wrapping_add(n) > (*(*(*data).backing).grid).hsize {
        (*data).oy = (*(*(*data).backing).grid).hsize;
        if (*data).cy < n {
            (*data).cy = 0 as libc::c_int as u_int
        } else {
            (*data).cy = ((*data).cy as libc::c_uint).wrapping_sub(n) as u_int as u_int
        }
    } else {
        (*data).oy = ((*data).oy as libc::c_uint).wrapping_add(n) as u_int as u_int
    }
    if (*data).screen.sel.is_null() || (*data).rectflag == 0 {
        py = (*(*(*data).backing).grid)
            .hsize
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        px = window_copy_find_length(wme, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px {
            window_copy_cursor_end_of_line(wme);
        }
    }
    if !(*data).searchmark.is_null() && (*data).timeout == 0 {
        window_copy_search_marks(wme, 0 as *mut screen, (*data).searchregex, 1 as libc::c_int);
    }
    window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
    window_copy_redraw_screen(wme);
}
unsafe extern "C" fn window_copy_pagedown(
    mut wme: *mut window_mode_entry,
    mut half_page: libc::c_int,
    mut scroll_exit: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut n: u_int = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    ox = window_copy_find_length(wme, oy);
    if (*data).cx != ox {
        (*data).lastcx = (*data).cx;
        (*data).lastsx = ox
    }
    (*data).cx = (*data).lastcx;
    n = 1 as libc::c_int as u_int;
    if (*(*s).grid).sy > 2 as libc::c_int as libc::c_uint {
        if half_page != 0 {
            n = (*(*s).grid)
                .sy
                .wrapping_div(2 as libc::c_int as libc::c_uint)
        } else {
            n = (*(*s).grid)
                .sy
                .wrapping_sub(2 as libc::c_int as libc::c_uint)
        }
    }
    if (*data).oy < n {
        (*data).oy = 0 as libc::c_int as u_int;
        if (*data).cy.wrapping_add(n.wrapping_sub((*data).oy)) >= (*(*(*data).backing).grid).sy {
            (*data).cy = (*(*(*data).backing).grid)
                .sy
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        } else {
            (*data).cy = ((*data).cy as libc::c_uint).wrapping_add(n.wrapping_sub((*data).oy))
                as u_int as u_int
        }
    } else {
        (*data).oy = ((*data).oy as libc::c_uint).wrapping_sub(n) as u_int as u_int
    }
    if (*data).screen.sel.is_null() || (*data).rectflag == 0 {
        py = (*(*(*data).backing).grid)
            .hsize
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        px = window_copy_find_length(wme, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px {
            window_copy_cursor_end_of_line(wme);
        }
    }
    if scroll_exit != 0 && (*data).oy == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    if !(*data).searchmark.is_null() && (*data).timeout == 0 {
        window_copy_search_marks(wme, 0 as *mut screen, (*data).searchregex, 1 as libc::c_int);
    }
    window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
    window_copy_redraw_screen(wme);
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_copy_previous_paragraph(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut oy: u_int = 0;
    oy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    while oy > 0 as libc::c_int as libc::c_uint
        && window_copy_find_length(wme, oy) == 0 as libc::c_int as libc::c_uint
    {
        oy = oy.wrapping_sub(1)
    }
    while oy > 0 as libc::c_int as libc::c_uint
        && window_copy_find_length(wme, oy) > 0 as libc::c_int as libc::c_uint
    {
        oy = oy.wrapping_sub(1)
    }
    window_copy_scroll_to(wme, 0 as libc::c_int as u_int, oy, 0 as libc::c_int);
}
unsafe extern "C" fn window_copy_next_paragraph(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut maxy: u_int = 0;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    oy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    maxy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*(*s).grid).sy)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    while oy < maxy && window_copy_find_length(wme, oy) == 0 as libc::c_int as libc::c_uint {
        oy = oy.wrapping_add(1)
    }
    while oy < maxy && window_copy_find_length(wme, oy) > 0 as libc::c_int as libc::c_uint {
        oy = oy.wrapping_add(1)
    }
    ox = window_copy_find_length(wme, oy);
    window_copy_scroll_to(wme, ox, oy, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_get_word(
    mut wp: *mut window_pane,
    mut x: u_int,
    mut y: u_int,
) -> *mut libc::c_char {
    let mut wme: *mut window_mode_entry = (*wp).modes.tqh_first;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut gd: *mut grid = (*data).screen.grid;
    return format_grid_word(gd, x, (*gd).hsize.wrapping_add(y));
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_get_line(
    mut wp: *mut window_pane,
    mut y: u_int,
) -> *mut libc::c_char {
    let mut wme: *mut window_mode_entry = (*wp).modes.tqh_first;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut gd: *mut grid = (*data).screen.grid;
    return format_grid_line(gd, (*gd).hsize.wrapping_add(y));
}
unsafe extern "C" fn window_copy_cursor_word_cb(
    mut ft: *mut crate::format::format_tree,
) -> *mut libc::c_char {
    let mut wp: *mut window_pane = format_get_pane(ft);
    let mut wme: *mut window_mode_entry = (*wp).modes.tqh_first;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    return window_copy_get_word(wp, (*data).cx, (*data).cy);
}
unsafe extern "C" fn window_copy_cursor_line_cb(
    mut ft: *mut crate::format::format_tree,
) -> *mut libc::c_char {
    let mut wp: *mut window_pane = format_get_pane(ft);
    let mut wme: *mut window_mode_entry = (*wp).modes.tqh_first;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    return window_copy_get_line(wp, (*data).cy);
}
unsafe extern "C" fn window_copy_search_match_cb(
    mut ft: *mut crate::format::format_tree,
) -> *mut libc::c_char {
    let mut wp: *mut window_pane = format_get_pane(ft);
    let mut wme: *mut window_mode_entry = (*wp).modes.tqh_first;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    return window_copy_match_at_cursor(data);
}
unsafe extern "C" fn window_copy_formats(
    mut wme: *mut window_mode_entry,
    mut ft: *mut crate::format::format_tree,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    format_add(
        ft,
        b"scroll_position\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (*data).oy,
    );
    format_add(
        ft,
        b"rectangle_toggle\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (*data).rectflag,
    );
    format_add(
        ft,
        b"copy_cursor_x\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (*data).cx,
    );
    format_add(
        ft,
        b"copy_cursor_y\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        (*data).cy,
    );
    format_add(
        ft,
        b"selection_present\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*data).screen.sel != 0 as *mut libc::c_void as *mut crate::screen::screen_sel)
            as libc::c_int,
    );
    if !(*data).screen.sel.is_null() {
        format_add(
            ft,
            b"selection_start_x\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*data).selx,
        );
        format_add(
            ft,
            b"selection_start_y\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*data).sely,
        );
        format_add(
            ft,
            b"selection_end_x\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*data).endselx,
        );
        format_add(
            ft,
            b"selection_end_y\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*data).endsely,
        );
        format_add(
            ft,
            b"selection_active\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            ((*data).cursordrag as libc::c_uint != CURSORDRAG_NONE as libc::c_int as libc::c_uint)
                as libc::c_int,
        );
    } else {
        format_add(
            ft,
            b"selection_active\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    format_add(
        ft,
        b"search_present\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        ((*data).searchmark != 0 as *mut libc::c_void as *mut u_char) as libc::c_int,
    );
    format_add_cb(
        ft,
        b"search_match\x00" as *const u8 as *const libc::c_char,
        Some(
            window_copy_search_match_cb
                as unsafe extern "C" fn(_: *mut crate::format::format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"copy_cursor_word\x00" as *const u8 as *const libc::c_char,
        Some(
            window_copy_cursor_word_cb
                as unsafe extern "C" fn(_: *mut crate::format::format_tree) -> *mut libc::c_char,
        ),
    );
    format_add_cb(
        ft,
        b"copy_cursor_line\x00" as *const u8 as *const libc::c_char,
        Some(
            window_copy_cursor_line_cb
                as unsafe extern "C" fn(_: *mut crate::format::format_tree) -> *mut libc::c_char,
        ),
    );
}
unsafe extern "C" fn window_copy_size_changed(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut search: libc::c_int =
        ((*data).searchmark != 0 as *mut libc::c_void as *mut u_char) as libc::c_int;
    window_copy_clear_selection(wme);
    window_copy_clear_marks(wme);
    screen_write_start(&mut ctx, s);
    window_copy_write_lines(wme, &mut ctx, 0 as libc::c_int as u_int, (*(*s).grid).sy);
    screen_write_stop(&mut ctx);
    if search != 0 && (*data).timeout == 0 {
        window_copy_search_marks(wme, 0 as *mut screen, (*data).searchregex, 0 as libc::c_int);
    }
    (*data).searchx = (*data).cx as libc::c_int;
    (*data).searchy = (*data).cy as libc::c_int;
    (*data).searcho = (*data).oy as libc::c_int;
}
unsafe extern "C" fn window_copy_resize(
    mut wme: *mut window_mode_entry,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut gd: *mut grid = (*(*data).backing).grid;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
    let mut wx: u_int = 0;
    let mut wy: u_int = 0;
    let mut reflow: libc::c_int = 0;
    screen_resize(s, sx, sy, 0 as libc::c_int);
    cx = (*data).cx;
    cy = (*gd)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    reflow = ((*gd).sx != sx) as libc::c_int;
    if reflow != 0 {
        grid_wrap_position(gd, cx, cy, &mut wx, &mut wy);
    }
    screen_resize_cursor(
        (*data).backing,
        sx,
        sy,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if reflow != 0 {
        grid_unwrap_position(gd, &mut cx, &mut cy, wx, wy);
    }
    (*data).cx = cx;
    if cy < (*gd).hsize {
        (*data).cy = 0 as libc::c_int as u_int;
        (*data).oy = (*gd).hsize.wrapping_sub(cy)
    } else {
        (*data).cy = cy.wrapping_sub((*gd).hsize);
        (*data).oy = 0 as libc::c_int as u_int
    }
    window_copy_size_changed(wme);
    window_copy_redraw_screen(wme);
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
unsafe extern "C" fn window_copy_key_table(mut wme: *mut window_mode_entry) -> *const libc::c_char {
    let mut wp: *mut window_pane = (*wme).wp;
    if options_get_number(
        (*(*wp).window).options,
        b"mode-keys\x00" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int as libc::c_longlong
    {
        return b"copy-mode-vi\x00" as *const u8 as *const libc::c_char;
    }
    return b"copy-mode\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn window_copy_expand_search_string(
    mut cs: *mut window_copy_cmd_state,
) -> libc::c_int {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut argument: *const libc::c_char = 0 as *const libc::c_char;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*cs).args).argc == 2 as libc::c_int {
        argument = *(*(*cs).args).argv.offset(1 as libc::c_int as isize);
        if *argument as libc::c_int != '\u{0}' as i32 {
            if args_has((*cs).args, 'F' as i32 as u_char) != 0 {
                expanded = format_single(
                    0 as *mut crate::cmd_queue::cmdq_item,
                    argument,
                    0 as *mut client,
                    0 as *mut session,
                    0 as *mut winlink,
                    (*wme).wp,
                );
                if *expanded as libc::c_int == '\u{0}' as i32 {
                    free(expanded as *mut libc::c_void);
                    return 0 as libc::c_int;
                }
                free((*data).searchstr as *mut libc::c_void);
                (*data).searchstr = expanded
            } else {
                free((*data).searchstr as *mut libc::c_void);
                (*data).searchstr = xstrdup(argument)
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn window_copy_cmd_append_selection(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut s: *mut session = (*cs).s;
    if !s.is_null() {
        window_copy_append_selection(wme);
    }
    window_copy_clear_selection(wme);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_append_selection_and_cancel(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut s: *mut session = (*cs).s;
    if !s.is_null() {
        window_copy_append_selection(wme);
    }
    window_copy_clear_selection(wme);
    return WINDOW_COPY_CMD_CANCEL;
}
unsafe extern "C" fn window_copy_cmd_back_to_indentation(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    window_copy_cursor_back_to_indentation(wme);
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_begin_selection(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut c: *mut client = (*cs).c;
    let mut m: *mut mouse_event = (*cs).m;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    if !m.is_null() {
        window_copy_start_drag(c, m);
        return WINDOW_COPY_CMD_NOTHING;
    }
    (*data).lineflag = LINE_SEL_NONE;
    (*data).selflag = SEL_CHAR;
    window_copy_start_selection(wme);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_stop_selection(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    (*data).cursordrag = CURSORDRAG_NONE;
    (*data).lineflag = LINE_SEL_NONE;
    (*data).selflag = SEL_CHAR;
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_bottom_line(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    (*data).cx = 0 as libc::c_int as u_int;
    (*data).cy = (*(*data).screen.grid)
        .sy
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_cancel(
    mut _cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    return WINDOW_COPY_CMD_CANCEL;
}
unsafe extern "C" fn window_copy_cmd_clear_selection(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    window_copy_clear_selection(wme);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_copy_end_of_line(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut c: *mut client = (*cs).c;
    let mut s: *mut session = (*cs).s;
    let mut wl: *mut winlink = (*cs).wl;
    let mut wp: *mut window_pane = (*wme).wp;
    let mut np: u_int = (*wme).prefix;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*cs).args).argc == 2 as libc::c_int {
        prefix = format_single(
            0 as *mut crate::cmd_queue::cmdq_item,
            *(*(*cs).args).argv.offset(1 as libc::c_int as isize),
            c,
            s,
            wl,
            wp,
        )
    }
    window_copy_start_selection(wme);
    while np > 1 as libc::c_int as libc::c_uint {
        window_copy_cursor_down(wme, 0 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    window_copy_cursor_end_of_line(wme);
    if !s.is_null() {
        window_copy_copy_selection(wme, prefix);
        free(prefix as *mut libc::c_void);
        return WINDOW_COPY_CMD_CANCEL;
    }
    free(prefix as *mut libc::c_void);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_copy_line(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut c: *mut client = (*cs).c;
    let mut s: *mut session = (*cs).s;
    let mut wl: *mut winlink = (*cs).wl;
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*cs).args).argc == 2 as libc::c_int {
        prefix = format_single(
            0 as *mut crate::cmd_queue::cmdq_item,
            *(*(*cs).args).argv.offset(1 as libc::c_int as isize),
            c,
            s,
            wl,
            wp,
        )
    }
    (*data).selflag = SEL_CHAR;
    window_copy_cursor_start_of_line(wme);
    window_copy_start_selection(wme);
    while np > 1 as libc::c_int as libc::c_uint {
        window_copy_cursor_down(wme, 0 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    window_copy_cursor_end_of_line(wme);
    if !s.is_null() {
        window_copy_copy_selection(wme, prefix);
        free(prefix as *mut libc::c_void);
        return WINDOW_COPY_CMD_CANCEL;
    }
    free(prefix as *mut libc::c_void);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_copy_selection_no_clear(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut c: *mut client = (*cs).c;
    let mut s: *mut session = (*cs).s;
    let mut wl: *mut winlink = (*cs).wl;
    let mut wp: *mut window_pane = (*wme).wp;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*cs).args).argc == 2 as libc::c_int {
        prefix = format_single(
            0 as *mut crate::cmd_queue::cmdq_item,
            *(*(*cs).args).argv.offset(1 as libc::c_int as isize),
            c,
            s,
            wl,
            wp,
        )
    }
    if !s.is_null() {
        window_copy_copy_selection(wme, prefix);
    }
    free(prefix as *mut libc::c_void);
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_copy_selection(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    window_copy_cmd_copy_selection_no_clear(cs);
    window_copy_clear_selection(wme);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_copy_selection_and_cancel(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    window_copy_cmd_copy_selection_no_clear(cs);
    window_copy_clear_selection(wme);
    return WINDOW_COPY_CMD_CANCEL;
}
unsafe extern "C" fn window_copy_cmd_cursor_down(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_down(wme, 0 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_cursor_down_and_cancel(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    let mut cy: u_int = 0;
    cy = (*data).cy;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_down(wme, 0 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    if cy == (*data).cy && (*data).oy == 0 as libc::c_int as libc::c_uint {
        return WINDOW_COPY_CMD_CANCEL;
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_cursor_left(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_left(wme);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_cursor_right(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_right(wme, 0 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_cursor_up(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_up(wme, 0 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_end_of_line(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    window_copy_cursor_end_of_line(wme);
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_halfpage_down(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        if window_copy_pagedown(wme, 1 as libc::c_int, (*data).scroll_exit) != 0 {
            return WINDOW_COPY_CMD_CANCEL;
        }
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_halfpage_down_and_cancel(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        if window_copy_pagedown(wme, 1 as libc::c_int, 1 as libc::c_int) != 0 {
            return WINDOW_COPY_CMD_CANCEL;
        }
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_halfpage_up(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_pageup1(wme, 1 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_history_bottom(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
    let mut oy: u_int = 0;
    oy = (*(*s).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    if (*data).lineflag as libc::c_uint == LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint
        && oy == (*data).endsely
    {
        window_copy_other_end(wme);
    }
    (*data).cy = (*(*data).screen.grid)
        .sy
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    (*data).cx = window_copy_find_length(wme, (*(*s).grid).hsize.wrapping_add((*data).cy));
    (*data).oy = 0 as libc::c_int as u_int;
    if !(*data).searchmark.is_null() && (*data).timeout == 0 {
        window_copy_search_marks(wme, 0 as *mut screen, (*data).searchregex, 1 as libc::c_int);
    }
    window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_history_top(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut oy: u_int = 0;
    oy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    if (*data).lineflag as libc::c_uint == LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint
        && oy == (*data).sely
    {
        window_copy_other_end(wme);
    }
    (*data).cy = 0 as libc::c_int as u_int;
    (*data).cx = 0 as libc::c_int as u_int;
    (*data).oy = (*(*(*data).backing).grid).hsize;
    if !(*data).searchmark.is_null() && (*data).timeout == 0 {
        window_copy_search_marks(wme, 0 as *mut screen, (*data).searchregex, 1 as libc::c_int);
    }
    window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_jump_again(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    match (*data).jumptype {
        3 => {
            while np != 0 as libc::c_int as libc::c_uint {
                window_copy_cursor_jump(wme);
                np = np.wrapping_sub(1)
            }
        }
        4 => {
            while np != 0 as libc::c_int as libc::c_uint {
                window_copy_cursor_jump_back(wme);
                np = np.wrapping_sub(1)
            }
        }
        5 => {
            while np != 0 as libc::c_int as libc::c_uint {
                window_copy_cursor_jump_to(wme);
                np = np.wrapping_sub(1)
            }
        }
        6 => {
            while np != 0 as libc::c_int as libc::c_uint {
                window_copy_cursor_jump_to_back(wme);
                np = np.wrapping_sub(1)
            }
        }
        _ => {}
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_jump_reverse(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    match (*data).jumptype {
        3 => {
            while np != 0 as libc::c_int as libc::c_uint {
                window_copy_cursor_jump_back(wme);
                np = np.wrapping_sub(1)
            }
        }
        4 => {
            while np != 0 as libc::c_int as libc::c_uint {
                window_copy_cursor_jump(wme);
                np = np.wrapping_sub(1)
            }
        }
        5 => {
            while np != 0 as libc::c_int as libc::c_uint {
                window_copy_cursor_jump_to_back(wme);
                np = np.wrapping_sub(1)
            }
        }
        6 => {
            while np != 0 as libc::c_int as libc::c_uint {
                window_copy_cursor_jump_to(wme);
                np = np.wrapping_sub(1)
            }
        }
        _ => {}
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_middle_line(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    (*data).cx = 0 as libc::c_int as u_int;
    (*data).cy = (*(*data).screen.grid)
        .sy
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint);
    window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_previous_matching_bracket(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
    let mut open: [libc::c_char; 4] =
        *::std::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"{[(\x00");
    let mut close: [libc::c_char; 4] =
        *::std::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"}])\x00");
    let mut tried: libc::c_char = 0;
    let mut found: libc::c_char = 0;
    let mut start: libc::c_char = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    let mut n: u_int = 0;
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
    let mut failed: libc::c_int = 0;
    while np != 0 as libc::c_int as libc::c_uint {
        /* Get cursor position and line length. */
        px = (*data).cx;
        py = (*(*s).grid)
            .hsize
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        xx = window_copy_find_length(wme, py);
        if xx == 0 as libc::c_int as libc::c_uint {
            break;
        }
        /*
         * Get the current character. If not on a bracket, try the
         * previous. If still not, then behave like previous-word.
         */
        tried = 0 as libc::c_int as libc::c_char;
        loop {
            grid_get_cell((*s).grid, px, py, &mut gc);
            if gc.data.size as libc::c_int != 1 as libc::c_int
                || gc.flags as libc::c_int & 0x4 as libc::c_int != 0
            {
                cp = 0 as *mut libc::c_char
            } else {
                found = *gc.data.data.as_mut_ptr() as libc::c_char;
                cp = strchr(close.as_mut_ptr(), found as libc::c_int)
            }
            if cp.is_null() {
                if !((*data).modekeys == 0 as libc::c_int) {
                    break;
                }
                if tried == 0 && px > 0 as libc::c_int as libc::c_uint {
                    px = px.wrapping_sub(1);
                    tried = 1 as libc::c_int as libc::c_char
                } else {
                    window_copy_cursor_previous_word(
                        wme,
                        b"}]) \x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    );
                    break;
                }
            } else {
                start = open[cp.wrapping_offset_from(close.as_mut_ptr()) as libc::c_long as usize];
                /* Walk backward until the matching bracket is reached. */
                n = 1 as libc::c_int as u_int;
                failed = 0 as libc::c_int;
                loop {
                    if px == 0 as libc::c_int as libc::c_uint {
                        if py == 0 as libc::c_int as libc::c_uint {
                            failed = 1 as libc::c_int;
                            break;
                        } else {
                            loop {
                                py = py.wrapping_sub(1);
                                xx = window_copy_find_length(wme, py);
                                if !(xx == 0 as libc::c_int as libc::c_uint
                                    && py > 0 as libc::c_int as libc::c_uint)
                                {
                                    break;
                                }
                            }
                            if xx == 0 as libc::c_int as libc::c_uint
                                && py == 0 as libc::c_int as libc::c_uint
                            {
                                failed = 1 as libc::c_int;
                                break;
                            } else {
                                px = xx.wrapping_sub(1 as libc::c_int as libc::c_uint)
                            }
                        }
                    } else {
                        px = px.wrapping_sub(1)
                    }
                    grid_get_cell((*s).grid, px, py, &mut gc);
                    if gc.data.size as libc::c_int == 1 as libc::c_int
                        && !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0
                    {
                        if *gc.data.data.as_mut_ptr() as libc::c_int == found as libc::c_int {
                            n = n.wrapping_add(1)
                        } else if *gc.data.data.as_mut_ptr() as libc::c_int == start as libc::c_int
                        {
                            n = n.wrapping_sub(1)
                        }
                    }
                    if !(n != 0 as libc::c_int as libc::c_uint) {
                        break;
                    }
                }
                /* Move the cursor to the found location if any. */
                if failed == 0 {
                    window_copy_scroll_to(wme, px, py, 0 as libc::c_int);
                }
                break;
            }
        }
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_next_matching_bracket(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
    let mut open: [libc::c_char; 4] =
        *::std::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"{[(\x00");
    let mut close: [libc::c_char; 4] =
        *::std::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"}])\x00");
    let mut tried: libc::c_char = 0;
    let mut found: libc::c_char = 0;
    let mut end: libc::c_char = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut n: u_int = 0;
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
    let mut failed: libc::c_int = 0;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    's_33: while np != 0 as libc::c_int as libc::c_uint {
        /* Get cursor position and line length. */
        px = (*data).cx;
        py = (*(*s).grid)
            .hsize
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        xx = window_copy_find_length(wme, py);
        yy = (*(*s).grid)
            .hsize
            .wrapping_add((*(*s).grid).sy)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
        if xx == 0 as libc::c_int as libc::c_uint {
            break;
        }
        /*
         * Get the current character. If not on a bracket, try the
         * next. If still not, then behave like next-word.
         */
        tried = 0 as libc::c_int as libc::c_char;
        loop {
            grid_get_cell((*s).grid, px, py, &mut gc);
            if gc.data.size as libc::c_int != 1 as libc::c_int
                || gc.flags as libc::c_int & 0x4 as libc::c_int != 0
            {
                cp = 0 as *mut libc::c_char
            } else {
                found = *gc.data.data.as_mut_ptr() as libc::c_char;
                /*
                 * In vi mode, attempt to move to previous bracket if a
                 * closing bracket is found first. If this fails,
                 * return to the original cursor position.
                 */
                cp = strchr(close.as_mut_ptr(), found as libc::c_int);
                if !cp.is_null() && (*data).modekeys == 1 as libc::c_int {
                    sx = (*data).cx;
                    sy = (*(*s).grid)
                        .hsize
                        .wrapping_add((*data).cy)
                        .wrapping_sub((*data).oy);
                    window_copy_scroll_to(wme, px, py, 0 as libc::c_int);
                    window_copy_cmd_previous_matching_bracket(cs);
                    px = (*data).cx;
                    py = (*(*s).grid)
                        .hsize
                        .wrapping_add((*data).cy)
                        .wrapping_sub((*data).oy);
                    grid_get_cell((*s).grid, px, py, &mut gc);
                    if gc.data.size as libc::c_int == 1 as libc::c_int
                        && !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0
                        && !strchr(
                            close.as_mut_ptr(),
                            *gc.data.data.as_mut_ptr() as libc::c_int,
                        )
                        .is_null()
                    {
                        window_copy_scroll_to(wme, sx, sy, 0 as libc::c_int);
                    }
                    break 's_33;
                } else {
                    cp = strchr(open.as_mut_ptr(), found as libc::c_int)
                }
            }
            if cp.is_null() {
                if (*data).modekeys == 0 as libc::c_int {
                    if tried == 0 && px <= xx {
                        px = px.wrapping_add(1);
                        tried = 1 as libc::c_int as libc::c_char
                    } else {
                        window_copy_cursor_next_word_end(
                            wme,
                            b"{[( \x00" as *const u8 as *const libc::c_char,
                            0 as libc::c_int,
                        );
                        break;
                    }
                } else if px > xx {
                    if py == yy {
                        break;
                    }
                    gl = grid_get_line((*s).grid, py);
                    if !(*gl).flags & 0x1 as libc::c_int != 0 {
                        break;
                    }
                    if (*gl).cellsize > (*(*s).grid).sx {
                        break;
                    }
                    px = 0 as libc::c_int as u_int;
                    py = py.wrapping_add(1);
                    xx = window_copy_find_length(wme, py)
                } else {
                    px = px.wrapping_add(1)
                }
            } else {
                end = close[cp.wrapping_offset_from(open.as_mut_ptr()) as libc::c_long as usize];
                /* For vi, continue searching for bracket until EOL. */
                /* Walk forward until the matching bracket is reached. */
                n = 1 as libc::c_int as u_int;
                failed = 0 as libc::c_int;
                loop {
                    if px > xx {
                        if py == yy {
                            failed = 1 as libc::c_int;
                            break;
                        } else {
                            px = 0 as libc::c_int as u_int;
                            py = py.wrapping_add(1);
                            xx = window_copy_find_length(wme, py)
                        }
                    } else {
                        px = px.wrapping_add(1)
                    }
                    grid_get_cell((*s).grid, px, py, &mut gc);
                    if gc.data.size as libc::c_int == 1 as libc::c_int
                        && !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0
                    {
                        if *gc.data.data.as_mut_ptr() as libc::c_int == found as libc::c_int {
                            n = n.wrapping_add(1)
                        } else if *gc.data.data.as_mut_ptr() as libc::c_int == end as libc::c_int {
                            n = n.wrapping_sub(1)
                        }
                    }
                    if !(n != 0 as libc::c_int as libc::c_uint) {
                        break;
                    }
                }
                /* Move the cursor to the found location if any. */
                if failed == 0 {
                    window_copy_scroll_to(wme, px, py, 0 as libc::c_int);
                }
                break;
            }
        }
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_next_paragraph(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_next_paragraph(wme);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_next_space(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_next_word(wme, b" \x00" as *const u8 as *const libc::c_char);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_next_space_end(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_next_word_end(
            wme,
            b" \x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_next_word(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut s: *mut session = (*cs).s;
    let mut np: u_int = (*wme).prefix;
    let mut ws: *const libc::c_char = 0 as *const libc::c_char;
    ws = options_get_string(
        (*s).options,
        b"word-separators\x00" as *const u8 as *const libc::c_char,
    );
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_next_word(wme, ws);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_next_word_end(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut s: *mut session = (*cs).s;
    let mut np: u_int = (*wme).prefix;
    let mut ws: *const libc::c_char = 0 as *const libc::c_char;
    ws = options_get_string(
        (*s).options,
        b"word-separators\x00" as *const u8 as *const libc::c_char,
    );
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_next_word_end(wme, ws, 0 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_other_end(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    (*data).selflag = SEL_CHAR;
    if np.wrapping_rem(2 as libc::c_int as libc::c_uint) != 0 as libc::c_int as libc::c_uint {
        window_copy_other_end(wme);
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_page_down(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        if window_copy_pagedown(wme, 0 as libc::c_int, (*data).scroll_exit) != 0 {
            return WINDOW_COPY_CMD_CANCEL;
        }
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_page_down_and_cancel(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        if window_copy_pagedown(wme, 0 as libc::c_int, 1 as libc::c_int) != 0 {
            return WINDOW_COPY_CMD_CANCEL;
        }
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_page_up(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_pageup1(wme, 0 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_previous_paragraph(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_previous_paragraph(wme);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_previous_space(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_previous_word(
            wme,
            b" \x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_previous_word(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut s: *mut session = (*cs).s;
    let mut np: u_int = (*wme).prefix;
    let mut ws: *const libc::c_char = 0 as *const libc::c_char;
    ws = options_get_string(
        (*s).options,
        b"word-separators\x00" as *const u8 as *const libc::c_char,
    );
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_previous_word(wme, ws, 1 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_rectangle_toggle(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    (*data).lineflag = LINE_SEL_NONE;
    window_copy_rectangle_toggle(wme);
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_scroll_down(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_down(wme, 1 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    if (*data).scroll_exit != 0 && (*data).oy == 0 as libc::c_int as libc::c_uint {
        return WINDOW_COPY_CMD_CANCEL;
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_scroll_down_and_cancel(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_down(wme, 1 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    if (*data).oy == 0 as libc::c_int as libc::c_uint {
        return WINDOW_COPY_CMD_CANCEL;
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_scroll_up(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut np: u_int = (*wme).prefix;
    while np != 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_up(wme, 1 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_search_again(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    if (*data).searchtype == WINDOW_COPY_SEARCHUP as libc::c_int {
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_search_up(wme, (*data).searchregex, 1 as libc::c_int);
            np = np.wrapping_sub(1)
        }
    } else if (*data).searchtype == WINDOW_COPY_SEARCHDOWN as libc::c_int {
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_search_down(wme, (*data).searchregex, 1 as libc::c_int);
            np = np.wrapping_sub(1)
        }
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_search_reverse(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    if (*data).searchtype == WINDOW_COPY_SEARCHUP as libc::c_int {
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_search_down(wme, (*data).searchregex, 1 as libc::c_int);
            np = np.wrapping_sub(1)
        }
    } else if (*data).searchtype == WINDOW_COPY_SEARCHDOWN as libc::c_int {
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_search_up(wme, (*data).searchregex, 1 as libc::c_int);
            np = np.wrapping_sub(1)
        }
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_select_line(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    (*data).lineflag = LINE_SEL_LEFT_RIGHT;
    (*data).rectflag = 0 as libc::c_int;
    (*data).selflag = SEL_LINE;
    (*data).dx = (*data).cx;
    (*data).dy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    window_copy_cursor_start_of_line(wme);
    (*data).selrx = (*data).cx;
    (*data).selry = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).endselrx = window_copy_find_length(wme, (*data).selry);
    (*data).endselry = (*data).selry;
    window_copy_start_selection(wme);
    while np > 1 as libc::c_int as libc::c_uint {
        window_copy_cursor_down(wme, 0 as libc::c_int);
        np = np.wrapping_sub(1)
    }
    window_copy_cursor_end_of_line(wme);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_select_word(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut s: *mut session = (*cs).s;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    (*data).lineflag = LINE_SEL_LEFT_RIGHT;
    (*data).rectflag = 0 as libc::c_int;
    (*data).selflag = SEL_WORD;
    (*data).dx = (*data).cx;
    (*data).dy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).ws = options_get_string(
        (*s).options,
        b"word-separators\x00" as *const u8 as *const libc::c_char,
    );
    window_copy_cursor_previous_word(wme, (*data).ws, 0 as libc::c_int);
    px = (*data).cx;
    py = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).selrx = px;
    (*data).selry = py;
    window_copy_start_selection(wme);
    if px >= window_copy_find_length(wme, py)
        || window_copy_in_set(
            wme,
            px.wrapping_add(1 as libc::c_int as libc::c_uint),
            py,
            (*data).ws,
        ) == 0
    {
        window_copy_cursor_next_word_end(wme, (*data).ws, 1 as libc::c_int);
    } else {
        window_copy_update_cursor(wme, px, (*data).cy);
        if window_copy_update_selection(wme, 1 as libc::c_int, 1 as libc::c_int) != 0 {
            window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
        }
    }
    (*data).endselrx = (*data).cx;
    (*data).endselry = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    if (*data).dx > (*data).endselrx {
        (*data).dx = (*data).endselrx
    }
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_set_mark(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut data: *mut window_copy_mode_data = (*(*cs).wme).data as *mut window_copy_mode_data;
    (*data).mx = (*data).cx;
    (*data).my = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).showmark = 1 as libc::c_int;
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_start_of_line(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    window_copy_cursor_start_of_line(wme);
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_top_line(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    (*data).cx = 0 as libc::c_int as u_int;
    (*data).cy = 0 as libc::c_int as u_int;
    window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_copy_pipe_no_clear(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut c: *mut client = (*cs).c;
    let mut s: *mut session = (*cs).s;
    let mut wl: *mut winlink = (*cs).wl;
    let mut wp: *mut window_pane = (*wme).wp;
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*cs).args).argc == 3 as libc::c_int {
        prefix = format_single(
            0 as *mut crate::cmd_queue::cmdq_item,
            *(*(*cs).args).argv.offset(2 as libc::c_int as isize),
            c,
            s,
            wl,
            wp,
        )
    }
    if !s.is_null()
        && (*(*cs).args).argc > 1 as libc::c_int
        && **(*(*cs).args).argv.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
    {
        command = format_single(
            0 as *mut crate::cmd_queue::cmdq_item,
            *(*(*cs).args).argv.offset(1 as libc::c_int as isize),
            c,
            s,
            wl,
            wp,
        )
    }
    window_copy_copy_pipe(wme, s, prefix, command);
    free(command as *mut libc::c_void);
    free(prefix as *mut libc::c_void);
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_copy_pipe(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    window_copy_cmd_copy_pipe_no_clear(cs);
    window_copy_clear_selection(wme);
    return WINDOW_COPY_CMD_REDRAW;
}
unsafe extern "C" fn window_copy_cmd_copy_pipe_and_cancel(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    window_copy_cmd_copy_pipe_no_clear(cs);
    window_copy_clear_selection(wme);
    return WINDOW_COPY_CMD_CANCEL;
}
unsafe extern "C" fn window_copy_cmd_goto_line(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut argument: *const libc::c_char = *(*(*cs).args).argv.offset(1 as libc::c_int as isize);
    if *argument as libc::c_int != '\u{0}' as i32 {
        window_copy_goto_line(wme, argument);
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_jump_backward(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    let mut argument: *const libc::c_char = *(*(*cs).args).argv.offset(1 as libc::c_int as isize);
    if *argument as libc::c_int != '\u{0}' as i32 {
        (*data).jumptype = WINDOW_COPY_JUMPBACKWARD as libc::c_int;
        (*data).jumpchar = *argument;
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_cursor_jump_back(wme);
            np = np.wrapping_sub(1)
        }
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_jump_forward(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    let mut argument: *const libc::c_char = *(*(*cs).args).argv.offset(1 as libc::c_int as isize);
    if *argument as libc::c_int != '\u{0}' as i32 {
        (*data).jumptype = WINDOW_COPY_JUMPFORWARD as libc::c_int;
        (*data).jumpchar = *argument;
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_cursor_jump(wme);
            np = np.wrapping_sub(1)
        }
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_jump_to_backward(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    let mut argument: *const libc::c_char = *(*(*cs).args).argv.offset(1 as libc::c_int as isize);
    if *argument as libc::c_int != '\u{0}' as i32 {
        (*data).jumptype = WINDOW_COPY_JUMPTOBACKWARD as libc::c_int;
        (*data).jumpchar = *argument;
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_cursor_jump_to_back(wme);
            np = np.wrapping_sub(1)
        }
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_jump_to_forward(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    let mut argument: *const libc::c_char = *(*(*cs).args).argv.offset(1 as libc::c_int as isize);
    if *argument as libc::c_int != '\u{0}' as i32 {
        (*data).jumptype = WINDOW_COPY_JUMPTOFORWARD as libc::c_int;
        (*data).jumpchar = *argument;
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_cursor_jump_to(wme);
            np = np.wrapping_sub(1)
        }
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_jump_to_mark(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    window_copy_jump_to_mark(wme);
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_search_backward(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    if window_copy_expand_search_string(cs) == 0 {
        return WINDOW_COPY_CMD_NOTHING;
    }
    if !(*data).searchstr.is_null() {
        (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
        (*data).searchregex = 1 as libc::c_int;
        (*data).timeout = 0 as libc::c_int;
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_search_up(wme, 1 as libc::c_int, 0 as libc::c_int);
            np = np.wrapping_sub(1)
        }
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_search_backward_text(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    if window_copy_expand_search_string(cs) == 0 {
        return WINDOW_COPY_CMD_NOTHING;
    }
    if !(*data).searchstr.is_null() {
        (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
        (*data).searchregex = 0 as libc::c_int;
        (*data).timeout = 0 as libc::c_int;
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_search_up(wme, 0 as libc::c_int, 0 as libc::c_int);
            np = np.wrapping_sub(1)
        }
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_search_forward(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    if window_copy_expand_search_string(cs) == 0 {
        return WINDOW_COPY_CMD_NOTHING;
    }
    if !(*data).searchstr.is_null() {
        (*data).searchtype = WINDOW_COPY_SEARCHDOWN as libc::c_int;
        (*data).searchregex = 1 as libc::c_int;
        (*data).timeout = 0 as libc::c_int;
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_search_down(wme, 1 as libc::c_int, 0 as libc::c_int);
            np = np.wrapping_sub(1)
        }
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_search_forward_text(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut np: u_int = (*wme).prefix;
    if window_copy_expand_search_string(cs) == 0 {
        return WINDOW_COPY_CMD_NOTHING;
    }
    if !(*data).searchstr.is_null() {
        (*data).searchtype = WINDOW_COPY_SEARCHDOWN as libc::c_int;
        (*data).searchregex = 0 as libc::c_int;
        (*data).timeout = 0 as libc::c_int;
        while np != 0 as libc::c_int as libc::c_uint {
            window_copy_search_down(wme, 0 as libc::c_int, 0 as libc::c_int);
            np = np.wrapping_sub(1)
        }
    }
    return WINDOW_COPY_CMD_NOTHING;
}
unsafe extern "C" fn window_copy_cmd_search_backward_incremental(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut argument: *const libc::c_char = *(*(*cs).args).argv.offset(1 as libc::c_int as isize);
    let mut ss: *const libc::c_char = (*data).searchstr;
    let mut prefix: libc::c_char = 0;
    let mut action: window_copy_cmd_action = WINDOW_COPY_CMD_NOTHING;
    (*data).timeout = 0 as libc::c_int;
    let fresh0 = argument;
    argument = argument.offset(1);
    prefix = *fresh0;
    if (*data).searchx == -(1 as libc::c_int) || (*data).searchy == -(1 as libc::c_int) {
        (*data).searchx = (*data).cx as libc::c_int;
        (*data).searchy = (*data).cy as libc::c_int;
        (*data).searcho = (*data).oy as libc::c_int
    } else if !ss.is_null() && strcmp(argument, ss) != 0 as libc::c_int {
        (*data).cx = (*data).searchx as u_int;
        (*data).cy = (*data).searchy as u_int;
        (*data).oy = (*data).searcho as u_int;
        action = WINDOW_COPY_CMD_REDRAW
    }
    if *argument as libc::c_int == '\u{0}' as i32 {
        window_copy_clear_marks(wme);
        return WINDOW_COPY_CMD_REDRAW;
    }
    match prefix as libc::c_int {
        61 | 45 => {
            (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
            (*data).searchregex = 0 as libc::c_int;
            free((*data).searchstr as *mut libc::c_void);
            (*data).searchstr = xstrdup(argument);
            if window_copy_search_up(wme, 0 as libc::c_int, 1 as libc::c_int) == 0 {
                window_copy_clear_marks(wme);
                return WINDOW_COPY_CMD_REDRAW;
            }
        }
        43 => {
            (*data).searchtype = WINDOW_COPY_SEARCHDOWN as libc::c_int;
            (*data).searchregex = 0 as libc::c_int;
            free((*data).searchstr as *mut libc::c_void);
            (*data).searchstr = xstrdup(argument);
            if window_copy_search_down(wme, 0 as libc::c_int, 0 as libc::c_int) == 0 {
                window_copy_clear_marks(wme);
                return WINDOW_COPY_CMD_REDRAW;
            }
        }
        _ => {}
    }
    return action;
}
unsafe extern "C" fn window_copy_cmd_search_forward_incremental(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut argument: *const libc::c_char = *(*(*cs).args).argv.offset(1 as libc::c_int as isize);
    let mut ss: *const libc::c_char = (*data).searchstr;
    let mut prefix: libc::c_char = 0;
    let mut action: window_copy_cmd_action = WINDOW_COPY_CMD_NOTHING;
    (*data).timeout = 0 as libc::c_int;
    let fresh1 = argument;
    argument = argument.offset(1);
    prefix = *fresh1;
    if (*data).searchx == -(1 as libc::c_int) || (*data).searchy == -(1 as libc::c_int) {
        (*data).searchx = (*data).cx as libc::c_int;
        (*data).searchy = (*data).cy as libc::c_int;
        (*data).searcho = (*data).oy as libc::c_int
    } else if !ss.is_null() && strcmp(argument, ss) != 0 as libc::c_int {
        (*data).cx = (*data).searchx as u_int;
        (*data).cy = (*data).searchy as u_int;
        (*data).oy = (*data).searcho as u_int;
        action = WINDOW_COPY_CMD_REDRAW
    }
    if *argument as libc::c_int == '\u{0}' as i32 {
        window_copy_clear_marks(wme);
        return WINDOW_COPY_CMD_REDRAW;
    }
    match prefix as libc::c_int {
        61 | 43 => {
            (*data).searchtype = WINDOW_COPY_SEARCHDOWN as libc::c_int;
            (*data).searchregex = 0 as libc::c_int;
            free((*data).searchstr as *mut libc::c_void);
            (*data).searchstr = xstrdup(argument);
            if window_copy_search_down(wme, 0 as libc::c_int, 1 as libc::c_int) == 0 {
                window_copy_clear_marks(wme);
                return WINDOW_COPY_CMD_REDRAW;
            }
        }
        45 => {
            (*data).searchtype = WINDOW_COPY_SEARCHUP as libc::c_int;
            (*data).searchregex = 0 as libc::c_int;
            free((*data).searchstr as *mut libc::c_void);
            (*data).searchstr = xstrdup(argument);
            if window_copy_search_up(wme, 0 as libc::c_int, 1 as libc::c_int) == 0 {
                window_copy_clear_marks(wme);
                return WINDOW_COPY_CMD_REDRAW;
            }
        }
        _ => {}
    }
    return action;
}
unsafe extern "C" fn window_copy_cmd_refresh_from_pane(
    mut cs: *mut window_copy_cmd_state,
) -> window_copy_cmd_action {
    let mut wme: *mut window_mode_entry = (*cs).wme;
    let mut wp: *mut window_pane = (*wme).swp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    if (*data).viewmode != 0 {
        return WINDOW_COPY_CMD_NOTHING;
    }
    screen_free((*data).backing);
    free((*data).backing as *mut libc::c_void);
    (*data).backing = window_copy_clone_screen(
        &mut (*wp).base,
        &mut (*data).screen,
        0 as *mut u_int,
        0 as *mut u_int,
        ((*wme).swp != (*wme).wp) as libc::c_int,
    );
    window_copy_size_changed(wme);
    return WINDOW_COPY_CMD_REDRAW;
}
static mut window_copy_cmd_table: [C2RustUnnamed_35; 68] = {
    [
        {
            let mut init = C2RustUnnamed_35 {
                command: b"append-selection\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_append_selection
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"append-selection-and-cancel\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_append_selection_and_cancel
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"back-to-indentation\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_back_to_indentation
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"begin-selection\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_begin_selection
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"bottom-line\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_bottom_line
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"cancel\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_cancel
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"clear-selection\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_clear_selection
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"copy-end-of-line\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_copy_end_of_line
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"copy-line\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_copy_line
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"copy-pipe-no-clear\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 2 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_NEVER,
                f: Some(
                    window_copy_cmd_copy_pipe_no_clear
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"copy-pipe\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 2 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_copy_pipe
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"copy-pipe-and-cancel\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 2 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_copy_pipe_and_cancel
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"copy-selection-no-clear\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_NEVER,
                f: Some(
                    window_copy_cmd_copy_selection_no_clear
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"copy-selection\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_copy_selection
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"copy-selection-and-cancel\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_copy_selection_and_cancel
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"cursor-down\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_cursor_down
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"cursor-down-and-cancel\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_cursor_down_and_cancel
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"cursor-left\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_cursor_left
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"cursor-right\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_cursor_right
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"cursor-up\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_cursor_up
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"end-of-line\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_end_of_line
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"goto-line\x00" as *const u8 as *const libc::c_char,
                minargs: 1 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_goto_line
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"halfpage-down\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_halfpage_down
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"halfpage-down-and-cancel\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_halfpage_down_and_cancel
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"halfpage-up\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_halfpage_up
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"history-bottom\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_history_bottom
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"history-top\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_history_top
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"jump-again\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_jump_again
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"jump-backward\x00" as *const u8 as *const libc::c_char,
                minargs: 1 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_jump_backward
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"jump-forward\x00" as *const u8 as *const libc::c_char,
                minargs: 1 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_jump_forward
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"jump-reverse\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_jump_reverse
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"jump-to-backward\x00" as *const u8 as *const libc::c_char,
                minargs: 1 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_jump_to_backward
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"jump-to-forward\x00" as *const u8 as *const libc::c_char,
                minargs: 1 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_jump_to_forward
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"jump-to-mark\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_jump_to_mark
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"middle-line\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_middle_line
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"next-matching-bracket\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_next_matching_bracket
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"next-paragraph\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_next_paragraph
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"next-space\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_next_space
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"next-space-end\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_next_space_end
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"next-word\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_next_word
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"next-word-end\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_next_word_end
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"other-end\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_other_end
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"page-down\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_page_down
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"page-down-and-cancel\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_page_down_and_cancel
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"page-up\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_page_up
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"previous-matching-bracket\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_previous_matching_bracket
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"previous-paragraph\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_previous_paragraph
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"previous-space\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_previous_space
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"previous-word\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_previous_word
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"rectangle-toggle\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_rectangle_toggle
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"refresh-from-pane\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_refresh_from_pane
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"scroll-down\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_scroll_down
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"scroll-down-and-cancel\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_scroll_down_and_cancel
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"scroll-up\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_scroll_up
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"search-again\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_search_again
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"search-backward\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_search_backward
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"search-backward-text\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_search_backward_text
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"search-backward-incremental\x00" as *const u8 as *const libc::c_char,
                minargs: 1 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_search_backward_incremental
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"search-forward\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_search_forward
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"search-forward-text\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_search_forward_text
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"search-forward-incremental\x00" as *const u8 as *const libc::c_char,
                minargs: 1 as libc::c_int,
                maxargs: 1 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_search_forward_incremental
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"search-reverse\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_search_reverse
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"select-line\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_select_line
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"select-word\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_select_word
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"set-mark\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_set_mark
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"start-of-line\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_start_of_line
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"stop-selection\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_ALWAYS,
                f: Some(
                    window_copy_cmd_stop_selection
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_35 {
                command: b"top-line\x00" as *const u8 as *const libc::c_char,
                minargs: 0 as libc::c_int,
                maxargs: 0 as libc::c_int,
                clear: WINDOW_COPY_CMD_CLEAR_EMACS_ONLY,
                f: Some(
                    window_copy_cmd_top_line
                        as unsafe extern "C" fn(
                            _: *mut window_copy_cmd_state,
                        ) -> window_copy_cmd_action,
                ),
            };
            init
        },
    ]
};
unsafe extern "C" fn window_copy_command(
    mut wme: *mut window_mode_entry,
    mut c: *mut client,
    mut s: *mut session,
    mut wl: *mut winlink,
    mut args: *mut args,
    mut m: *mut mouse_event,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut cs: window_copy_cmd_state = window_copy_cmd_state {
        wme: 0 as *mut window_mode_entry,
        args: 0 as *mut args,
        m: 0 as *mut mouse_event,
        c: 0 as *mut client,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
    };
    let mut action: window_copy_cmd_action = WINDOW_COPY_CMD_NOTHING;
    let mut clear: window_copy_cmd_clear = WINDOW_COPY_CMD_CLEAR_NEVER;
    let mut command: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: u_int = 0;
    let mut keys: libc::c_int = 0;
    if (*args).argc == 0 as libc::c_int {
        return;
    }
    command = *(*args).argv.offset(0 as libc::c_int as isize);
    if !m.is_null() && (*m).valid != 0 && (*m).b & 64 as libc::c_int as libc::c_uint == 0 {
        window_copy_move_mouse(m);
    }
    cs.wme = wme;
    cs.args = args;
    cs.m = m;
    cs.c = c;
    cs.s = s;
    cs.wl = wl;
    action = WINDOW_COPY_CMD_NOTHING;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[C2RustUnnamed_35; 68]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_35>() as libc::c_ulong)
    {
        if strcmp(window_copy_cmd_table[i as usize].command, command) == 0 as libc::c_int {
            if ((*args).argc - 1 as libc::c_int) < window_copy_cmd_table[i as usize].minargs
                || (*args).argc - 1 as libc::c_int > window_copy_cmd_table[i as usize].maxargs
            {
                break;
            }
            clear = window_copy_cmd_table[i as usize].clear;
            action = window_copy_cmd_table[i as usize]
                .f
                .expect("non-null function pointer")(&mut cs);
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    if strncmp(
        command,
        b"search-\x00" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
        && !(*data).searchmark.is_null()
    {
        keys = options_get_number(
            (*(*(*wme).wp).window).options,
            b"mode-keys\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        if clear as libc::c_uint == WINDOW_COPY_CMD_CLEAR_EMACS_ONLY as libc::c_int as libc::c_uint
            && keys == 1 as libc::c_int
        {
            clear = WINDOW_COPY_CMD_CLEAR_NEVER
        }
        if clear as libc::c_uint != WINDOW_COPY_CMD_CLEAR_NEVER as libc::c_int as libc::c_uint {
            window_copy_clear_marks(wme);
            (*data).searchy = -(1 as libc::c_int);
            (*data).searchx = (*data).searchy
        } else if (*data).searchthis != -(1 as libc::c_int) {
            (*data).searchthis = -(1 as libc::c_int);
            action = WINDOW_COPY_CMD_REDRAW
        }
        if action as libc::c_uint == WINDOW_COPY_CMD_NOTHING as libc::c_int as libc::c_uint {
            action = WINDOW_COPY_CMD_REDRAW
        }
    }
    (*wme).prefix = 1 as libc::c_int as u_int;
    if action as libc::c_uint == WINDOW_COPY_CMD_CANCEL as libc::c_int as libc::c_uint {
        window_pane_reset_mode((*wme).wp);
    } else if action as libc::c_uint == WINDOW_COPY_CMD_REDRAW as libc::c_int as libc::c_uint {
        window_copy_redraw_screen(wme);
    };
}
unsafe extern "C" fn window_copy_scroll_to(
    mut wme: *mut window_mode_entry,
    mut px: u_int,
    mut py: u_int,
    mut no_redraw: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut gd: *mut grid = (*(*data).backing).grid;
    let mut offset: u_int = 0;
    let mut gap: u_int = 0;
    (*data).cx = px;
    if py >= (*gd).hsize.wrapping_sub((*data).oy)
        && py < (*gd).hsize.wrapping_sub((*data).oy).wrapping_add((*gd).sy)
    {
        (*data).cy = py.wrapping_sub((*gd).hsize.wrapping_sub((*data).oy))
    } else {
        gap = (*gd).sy.wrapping_div(4 as libc::c_int as libc::c_uint);
        if py < (*gd).sy {
            offset = 0 as libc::c_int as u_int;
            (*data).cy = py
        } else if py > (*gd).hsize.wrapping_add((*gd).sy).wrapping_sub(gap) {
            offset = (*gd).hsize;
            (*data).cy = py.wrapping_sub((*gd).hsize)
        } else {
            offset = py.wrapping_add(gap).wrapping_sub((*gd).sy);
            (*data).cy = py.wrapping_sub(offset)
        }
        (*data).oy = (*gd).hsize.wrapping_sub(offset)
    }
    if no_redraw == 0 && !(*data).searchmark.is_null() && (*data).timeout == 0 {
        window_copy_search_marks(wme, 0 as *mut screen, (*data).searchregex, 1 as libc::c_int);
    }
    window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
    if no_redraw == 0 {
        window_copy_redraw_screen(wme);
    };
}
unsafe extern "C" fn window_copy_search_compare(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut sgd: *mut grid,
    mut spx: u_int,
    mut cis: libc::c_int,
) -> libc::c_int {
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
    let mut sgc: GridCell = GridCell {
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
    let mut ud: *const Utf8Data = 0 as *const Utf8Data;
    let mut sud: *const Utf8Data = 0 as *const Utf8Data;
    grid_get_cell(gd, px, py, &mut gc);
    ud = &mut gc.data;
    grid_get_cell(sgd, spx, 0 as libc::c_int as u_int, &mut sgc);
    sud = &mut sgc.data;
    if (*ud).size as libc::c_int != (*sud).size as libc::c_int
        || (*ud).width as libc::c_int != (*sud).width as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if cis != 0 && (*ud).size as libc::c_int == 1 as libc::c_int {
        return (({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<u_char>() as libc::c_ulong > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = (*ud).data[0 as libc::c_int as usize] as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower((*ud).data[0 as libc::c_int as usize] as libc::c_int)
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset((*ud).data[0 as libc::c_int as usize] as libc::c_int as isize)
            }
            __res
        }) == (*sud).data[0 as libc::c_int as usize] as libc::c_int) as libc::c_int;
    }
    return (memcmp(
        (*ud).data.as_ptr() as *const libc::c_void,
        (*sud).data.as_ptr() as *const libc::c_void,
        (*ud).size as libc::c_ulong,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn window_copy_search_lr(
    mut gd: *mut grid,
    mut sgd: *mut grid,
    mut ppx: *mut u_int,
    mut py: u_int,
    mut first: u_int,
    mut last: u_int,
    mut cis: libc::c_int,
) -> libc::c_int {
    let mut ax: u_int = 0;
    let mut bx: u_int = 0;
    let mut px: u_int = 0;
    let mut pywrap: u_int = 0;
    let mut endline: u_int = 0;
    let mut matched: libc::c_int = 0;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    endline = (*gd)
        .hsize
        .wrapping_add((*gd).sy)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    ax = first;
    while ax < last {
        bx = 0 as libc::c_int as u_int;
        while bx < (*sgd).sx {
            px = ax.wrapping_add(bx);
            pywrap = py;
            /* Wrap line. */
            while px >= (*gd).sx && pywrap < endline {
                gl = grid_get_line(gd, pywrap);
                if !(*gl).flags & 0x1 as libc::c_int != 0 {
                    break;
                }
                px = (px as libc::c_uint).wrapping_sub((*gd).sx) as u_int as u_int;
                pywrap = pywrap.wrapping_add(1)
            }
            /* We have run off the end of the grid. */
            if px >= (*gd).sx {
                break;
            }
            matched = window_copy_search_compare(gd, px, pywrap, sgd, bx, cis);
            if matched == 0 {
                break;
            }
            bx = bx.wrapping_add(1)
        }
        if bx == (*sgd).sx {
            *ppx = ax;
            return 1 as libc::c_int;
        }
        ax = ax.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_copy_search_rl(
    mut gd: *mut grid,
    mut sgd: *mut grid,
    mut ppx: *mut u_int,
    mut py: u_int,
    mut first: u_int,
    mut last: u_int,
    mut cis: libc::c_int,
) -> libc::c_int {
    let mut ax: u_int = 0;
    let mut bx: u_int = 0;
    let mut px: u_int = 0;
    let mut pywrap: u_int = 0;
    let mut endline: u_int = 0;
    let mut matched: libc::c_int = 0;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    endline = (*gd)
        .hsize
        .wrapping_add((*gd).sy)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    ax = last;
    while ax > first {
        bx = 0 as libc::c_int as u_int;
        while bx < (*sgd).sx {
            px = ax
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_add(bx);
            pywrap = py;
            /* Wrap line. */
            while px >= (*gd).sx && pywrap < endline {
                gl = grid_get_line(gd, pywrap);
                if !(*gl).flags & 0x1 as libc::c_int != 0 {
                    break;
                }
                px = (px as libc::c_uint).wrapping_sub((*gd).sx) as u_int as u_int;
                pywrap = pywrap.wrapping_add(1)
            }
            /* We have run off the end of the grid. */
            if px >= (*gd).sx {
                break;
            }
            matched = window_copy_search_compare(gd, px, pywrap, sgd, bx, cis);
            if matched == 0 {
                break;
            }
            bx = bx.wrapping_add(1)
        }
        if bx == (*sgd).sx {
            *ppx = ax.wrapping_sub(1 as libc::c_int as libc::c_uint);
            return 1 as libc::c_int;
        }
        ax = ax.wrapping_sub(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_copy_search_lr_regex(
    mut gd: *mut grid,
    mut ppx: *mut u_int,
    mut psx: *mut u_int,
    mut py: u_int,
    mut first: u_int,
    mut last: u_int,
    mut reg: *mut regex_t,
) -> libc::c_int {
    let mut eflags: libc::c_int = 0 as libc::c_int;
    let mut endline: u_int = 0;
    let mut foundx: u_int = 0;
    let mut foundy: u_int = 0;
    let mut len: u_int = 0;
    let mut pywrap: u_int = 0;
    let mut size: u_int = 1 as libc::c_int as u_int;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut regmatch: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    /*
     * This can happen during search if the last match was the last
     * character on a line.
     */
    if first >= last {
        return 0 as libc::c_int;
    }
    /* Set flags for regex search. */
    if first != 0 as libc::c_int as libc::c_uint {
        eflags |= 1 as libc::c_int
    }
    /* Need to look at the entire string. */
    buf = xmalloc(size as size_t) as *mut libc::c_char;
    *buf.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    buf = window_copy_stringify(gd, py, first, (*gd).sx, buf, &mut size);
    len = (*gd).sx.wrapping_sub(first);
    endline = (*gd)
        .hsize
        .wrapping_add((*gd).sy)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    pywrap = py;
    while !buf.is_null() && pywrap <= endline {
        gl = grid_get_line(gd, pywrap);
        if !(*gl).flags & 0x1 as libc::c_int != 0 {
            break;
        }
        pywrap = pywrap.wrapping_add(1);
        buf = window_copy_stringify(
            gd,
            pywrap,
            0 as libc::c_int as u_int,
            (*gd).sx,
            buf,
            &mut size,
        );
        len = (len as libc::c_uint).wrapping_add((*gd).sx) as u_int as u_int
    }
    if regexec(reg, buf, 1 as libc::c_int as size_t, &mut regmatch, eflags) == 0 as libc::c_int
        && regmatch.rm_so != regmatch.rm_eo
    {
        foundx = first;
        foundy = py;
        window_copy_cstrtocellpos(
            gd,
            len,
            &mut foundx,
            &mut foundy,
            buf.offset(regmatch.rm_so as isize),
        );
        if foundy == py && foundx < last {
            *ppx = foundx;
            len = (len as libc::c_uint).wrapping_sub(foundx.wrapping_sub(first)) as u_int as u_int;
            window_copy_cstrtocellpos(
                gd,
                len,
                &mut foundx,
                &mut foundy,
                buf.offset(regmatch.rm_eo as isize),
            );
            *psx = foundx;
            while foundy > py {
                *psx = (*psx as libc::c_uint).wrapping_add((*gd).sx) as u_int as u_int;
                foundy = foundy.wrapping_sub(1)
            }
            *psx = (*psx as libc::c_uint).wrapping_sub(*ppx) as u_int as u_int;
            free(buf as *mut libc::c_void);
            return 1 as libc::c_int;
        }
    }
    free(buf as *mut libc::c_void);
    *ppx = 0 as libc::c_int as u_int;
    *psx = 0 as libc::c_int as u_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_copy_search_rl_regex(
    mut gd: *mut grid,
    mut ppx: *mut u_int,
    mut psx: *mut u_int,
    mut py: u_int,
    mut first: u_int,
    mut last: u_int,
    mut reg: *mut regex_t,
) -> libc::c_int {
    let mut eflags: libc::c_int = 0 as libc::c_int;
    let mut endline: u_int = 0;
    let mut len: u_int = 0;
    let mut pywrap: u_int = 0;
    let mut size: u_int = 1 as libc::c_int as u_int;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    /* Set flags for regex search. */
    if first != 0 as libc::c_int as libc::c_uint {
        eflags |= 1 as libc::c_int
    }
    /* Need to look at the entire string. */
    buf = xmalloc(size as size_t) as *mut libc::c_char;
    *buf.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    buf = window_copy_stringify(gd, py, first, (*gd).sx, buf, &mut size);
    len = (*gd).sx.wrapping_sub(first);
    endline = (*gd)
        .hsize
        .wrapping_add((*gd).sy)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    pywrap = py;
    while !buf.is_null() && pywrap <= endline {
        gl = grid_get_line(gd, pywrap);
        if !(*gl).flags & 0x1 as libc::c_int != 0 {
            break;
        }
        pywrap = pywrap.wrapping_add(1);
        buf = window_copy_stringify(
            gd,
            pywrap,
            0 as libc::c_int as u_int,
            (*gd).sx,
            buf,
            &mut size,
        );
        len = (len as libc::c_uint).wrapping_add((*gd).sx) as u_int as u_int
    }
    if window_copy_last_regex(gd, py, first, last, len, ppx, psx, buf, reg, eflags) != 0 {
        free(buf as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    free(buf as *mut libc::c_void);
    *ppx = 0 as libc::c_int as u_int;
    *psx = 0 as libc::c_int as u_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_copy_cellstring(
    mut gl: *const grid_line,
    mut px: u_int,
    mut size: *mut size_t,
    mut allocated: *mut libc::c_int,
) -> *const libc::c_char {
    static mut ud: Utf8Data = Utf8Data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut gce: *mut grid_cell_entry = 0 as *mut grid_cell_entry;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    if px >= (*gl).cellsize {
        *size = 1 as libc::c_int as size_t;
        *allocated = 0 as libc::c_int;
        return b" \x00" as *const u8 as *const libc::c_char;
    }
    gce = &mut *(*gl).celldata.offset(px as isize) as *mut grid_cell_entry;
    if (*gce).flags as libc::c_int & 0x4 as libc::c_int != 0 {
        *size = 0 as libc::c_int as size_t;
        *allocated = 0 as libc::c_int;
        return 0 as *const libc::c_char;
    }
    if !((*gce).flags as libc::c_int) & 0x8 as libc::c_int != 0 {
        *size = 1 as libc::c_int as size_t;
        *allocated = 0 as libc::c_int;
        return &mut (*gce).c2rust_unnamed.data.data as *mut u_char as *const libc::c_char;
    }
    utf8_to_data(
        (*(*gl).extddata.offset((*gce).c2rust_unnamed.offset as isize)).data,
        &mut ud,
    );
    *size = ud.size as size_t;
    *allocated = 1 as libc::c_int;
    copy = xmalloc(ud.size as size_t) as *mut libc::c_char;
    memcpy(
        copy as *mut libc::c_void,
        ud.data.as_mut_ptr() as *const libc::c_void,
        ud.size as libc::c_ulong,
    );
    return copy;
}
/* Find last match in given range. */
unsafe extern "C" fn window_copy_last_regex(
    mut gd: *mut grid,
    mut py: u_int,
    mut first: u_int,
    mut last: u_int,
    mut len: u_int,
    mut ppx: *mut u_int,
    mut psx: *mut u_int,
    mut buf: *const libc::c_char,
    mut preg: *const regex_t,
    mut eflags: libc::c_int,
) -> libc::c_int {
    let mut foundx: u_int = 0;
    let mut foundy: u_int = 0;
    let mut oldx: u_int = 0;
    let mut px: u_int = 0 as libc::c_int as u_int;
    let mut savepx: u_int = 0;
    let mut savesx: u_int = 0 as libc::c_int as u_int;
    let mut regmatch: regmatch_t = regmatch_t { rm_so: 0, rm_eo: 0 };
    foundx = first;
    foundy = py;
    oldx = first;
    while regexec(
        preg,
        buf.offset(px as isize),
        1 as libc::c_int as size_t,
        &mut regmatch,
        eflags,
    ) == 0 as libc::c_int
    {
        if regmatch.rm_so == regmatch.rm_eo {
            break;
        }
        window_copy_cstrtocellpos(
            gd,
            len,
            &mut foundx,
            &mut foundy,
            buf.offset(px as isize).offset(regmatch.rm_so as isize),
        );
        if foundy > py || foundx >= last {
            break;
        }
        len = (len as libc::c_uint).wrapping_sub(foundx.wrapping_sub(oldx)) as u_int as u_int;
        savepx = foundx;
        window_copy_cstrtocellpos(
            gd,
            len,
            &mut foundx,
            &mut foundy,
            buf.offset(px as isize).offset(regmatch.rm_eo as isize),
        );
        if foundy > py || foundx >= last {
            *ppx = savepx;
            *psx = foundx;
            while foundy > py {
                *psx = (*psx as libc::c_uint).wrapping_add((*gd).sx) as u_int as u_int;
                foundy = foundy.wrapping_sub(1)
            }
            *psx = (*psx as libc::c_uint).wrapping_sub(*ppx) as u_int as u_int;
            return 1 as libc::c_int;
        } else {
            savesx = foundx.wrapping_sub(savepx);
            len = (len as libc::c_uint).wrapping_sub(savesx) as u_int as u_int;
            oldx = foundx
        }
        px = (px as libc::c_uint).wrapping_add(regmatch.rm_eo as libc::c_uint) as u_int as u_int
    }
    if savesx > 0 as libc::c_int as libc::c_uint {
        *ppx = savepx;
        *psx = savesx;
        return 1 as libc::c_int;
    } else {
        *ppx = 0 as libc::c_int as u_int;
        *psx = 0 as libc::c_int as u_int;
        return 0 as libc::c_int;
    };
}
/* Stringify line and append to input buffer. Caller frees. */
unsafe extern "C" fn window_copy_stringify(
    mut gd: *mut grid,
    mut py: u_int,
    mut first: u_int,
    mut last: u_int,
    mut buf: *mut libc::c_char,
    mut size: *mut u_int,
) -> *mut libc::c_char {
    let mut ax: u_int = 0;
    let mut bx: u_int = 0;
    let mut newsize: u_int = *size;
    let mut gl: *const grid_line = 0 as *const grid_line;
    let mut d: *const libc::c_char = 0 as *const libc::c_char;
    let mut bufsize: size_t = 1024 as libc::c_int as size_t;
    let mut dlen: size_t = 0;
    let mut allocated: libc::c_int = 0;
    while bufsize < newsize as libc::c_ulong {
        bufsize = (bufsize as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t
    }
    buf = xrealloc(buf as *mut libc::c_void, bufsize) as *mut libc::c_char;
    gl = grid_peek_line(gd, py);
    bx = (*size).wrapping_sub(1 as libc::c_int as libc::c_uint);
    ax = first;
    while ax < last {
        d = window_copy_cellstring(gl, ax, &mut dlen, &mut allocated);
        newsize = (newsize as libc::c_ulong).wrapping_add(dlen) as u_int as u_int;
        while bufsize < newsize as libc::c_ulong {
            bufsize = (bufsize as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            buf = xrealloc(buf as *mut libc::c_void, bufsize) as *mut libc::c_char
        }
        if dlen == 1 as libc::c_int as libc::c_ulong {
            let fresh2 = bx;
            bx = bx.wrapping_add(1);
            *buf.offset(fresh2 as isize) = *d
        } else {
            memcpy(
                buf.offset(bx as isize) as *mut libc::c_void,
                d as *const libc::c_void,
                dlen,
            );
            bx = (bx as libc::c_ulong).wrapping_add(dlen) as u_int as u_int
        }
        if allocated != 0 {
            free(d as *mut libc::c_void);
        }
        ax = ax.wrapping_add(1)
    }
    *buf.offset(newsize.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize) =
        '\u{0}' as i32 as libc::c_char;
    *size = newsize;
    return buf;
}
/* Map start of C string containing UTF-8 data to grid cell position. */
unsafe extern "C" fn window_copy_cstrtocellpos(
    mut gd: *mut grid,
    mut ncells: u_int,
    mut ppx: *mut u_int,
    mut ppy: *mut u_int,
    mut str: *const libc::c_char,
) {
    let mut cell: u_int = 0;
    let mut ccell: u_int = 0;
    let mut px: u_int = 0;
    let mut pywrap: u_int = 0;
    let mut pos: u_int = 0;
    let mut len: u_int = 0;
    let mut match_0: libc::c_int = 0;
    let mut gl: *const grid_line = 0 as *const grid_line;
    let mut d: *const libc::c_char = 0 as *const libc::c_char;
    let mut dlen: size_t = 0;
    let mut cells: *mut C2RustUnnamed_36 = 0 as *mut C2RustUnnamed_36;
    /* Populate the array of cell data. */
    cells = xreallocarray(
        0 as *mut libc::c_void,
        ncells as size_t,
        ::std::mem::size_of::<C2RustUnnamed_36>() as libc::c_ulong,
    ) as *mut C2RustUnnamed_36;
    cell = 0 as libc::c_int as u_int;
    px = *ppx;
    pywrap = *ppy;
    gl = grid_peek_line(gd, pywrap);
    while cell < ncells {
        let ref mut fresh3 = (*cells.offset(cell as isize)).d;
        *fresh3 = window_copy_cellstring(
            gl,
            px,
            &mut (*cells.offset(cell as isize)).dlen,
            &mut (*cells.offset(cell as isize)).allocated,
        );
        cell = cell.wrapping_add(1);
        px = px.wrapping_add(1);
        if px == (*gd).sx {
            px = 0 as libc::c_int as u_int;
            pywrap = pywrap.wrapping_add(1);
            gl = grid_peek_line(gd, pywrap)
        }
    }
    /* Locate starting cell. */
    cell = 0 as libc::c_int as u_int;
    len = strlen(str) as u_int;
    while cell < ncells {
        ccell = cell;
        pos = 0 as libc::c_int as u_int;
        match_0 = 1 as libc::c_int;
        while ccell < ncells {
            if *str.offset(pos as isize) as libc::c_int == '\u{0}' as i32 {
                match_0 = 0 as libc::c_int;
                break;
            } else {
                d = (*cells.offset(ccell as isize)).d;
                dlen = (*cells.offset(ccell as isize)).dlen;
                if dlen == 1 as libc::c_int as libc::c_ulong {
                    if *str.offset(pos as isize) as libc::c_int != *d as libc::c_int {
                        match_0 = 0 as libc::c_int;
                        break;
                    } else {
                        pos = pos.wrapping_add(1)
                    }
                } else {
                    if dlen > len.wrapping_sub(pos) as libc::c_ulong {
                        dlen = len.wrapping_sub(pos) as size_t
                    }
                    if memcmp(
                        str.offset(pos as isize) as *const libc::c_void,
                        d as *const libc::c_void,
                        dlen,
                    ) != 0 as libc::c_int
                    {
                        match_0 = 0 as libc::c_int;
                        break;
                    } else {
                        pos = (pos as libc::c_ulong).wrapping_add(dlen) as u_int as u_int
                    }
                }
                ccell = ccell.wrapping_add(1)
            }
        }
        if match_0 != 0 {
            break;
        }
        cell = cell.wrapping_add(1)
    }
    /* If not found this will be one past the end. */
    px = (*ppx).wrapping_add(cell);
    pywrap = *ppy;
    while px >= (*gd).sx {
        px = (px as libc::c_uint).wrapping_sub((*gd).sx) as u_int as u_int;
        pywrap = pywrap.wrapping_add(1)
    }
    *ppx = px;
    *ppy = pywrap;
    /* Free cell data. */
    cell = 0 as libc::c_int as u_int;
    while cell < ncells {
        if (*cells.offset(cell as isize)).allocated != 0 {
            free((*cells.offset(cell as isize)).d as *mut libc::c_void);
        }
        cell = cell.wrapping_add(1)
    }
    free(cells as *mut libc::c_void);
}
unsafe extern "C" fn window_copy_move_left(
    mut s: *mut screen,
    mut fx: *mut u_int,
    mut fy: *mut u_int,
    mut wrapflag: libc::c_int,
) {
    if *fx == 0 as libc::c_int as libc::c_uint {
        /* left */
        if *fy == 0 as libc::c_int as libc::c_uint {
            /* top */
            if wrapflag != 0 {
                *fx = (*(*s).grid)
                    .sx
                    .wrapping_sub(1 as libc::c_int as libc::c_uint);
                *fy = (*(*s).grid)
                    .hsize
                    .wrapping_add((*(*s).grid).sy)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
            }
            return;
        }
        *fx = (*(*s).grid)
            .sx
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
        *fy = (*fy).wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else {
        *fx = (*fx).wrapping_sub(1 as libc::c_int as libc::c_uint)
    };
}
unsafe extern "C" fn window_copy_is_lowercase(mut ptr: *const libc::c_char) -> libc::c_int {
    while *ptr as libc::c_int != '\u{0}' as i32 {
        if *ptr as libc::c_int
            != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<u_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *ptr as u_char as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(*ptr as u_char as libc::c_int)
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(*ptr as u_char as libc::c_int as isize)
                }
                __res
            })
        {
            return 0 as libc::c_int;
        }
        ptr = ptr.offset(1)
    }
    return 1 as libc::c_int;
}
/*
 * Search for text stored in sgd starting from position fx,fy up to endline. If
 * found, jump to it. If cis then ignore case. The direction is 0 for searching
 * up, down otherwise. If wrap then go to begin/end of grid and try again if
 * not found.
 */
unsafe extern "C" fn window_copy_search_jump(
    mut wme: *mut window_mode_entry,
    mut gd: *mut grid,
    mut sgd: *mut grid,
    mut fx: u_int,
    mut fy: u_int,
    mut endline: u_int,
    mut cis: libc::c_int,
    mut wrap: libc::c_int,
    mut direction: libc::c_int,
    mut regex: libc::c_int,
    mut foundlen: *mut u_int,
) -> libc::c_int {
    let mut i: u_int = 0;
    let mut px: u_int = 0;
    let mut sx: u_int = 0;
    let mut ssize: u_int = 1 as libc::c_int as u_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut cflags: libc::c_int = 1 as libc::c_int;
    let mut sbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reg: regex_t = regex_t {
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
    if regex != 0 {
        sbuf = xmalloc(ssize as size_t) as *mut libc::c_char;
        *sbuf.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        sbuf = window_copy_stringify(
            sgd,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
            (*sgd).sx,
            sbuf,
            &mut ssize,
        );
        if cis != 0 {
            cflags |= (1 as libc::c_int) << 1 as libc::c_int
        }
        if regcomp(&mut reg, sbuf, cflags) != 0 as libc::c_int {
            free(sbuf as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        free(sbuf as *mut libc::c_void);
    }
    if direction != 0 {
        i = fy;
        while i <= endline {
            if regex != 0 {
                found =
                    window_copy_search_lr_regex(gd, &mut px, &mut sx, i, fx, (*gd).sx, &mut reg);
                if found != 0 {
                    *foundlen = sx
                }
            } else {
                found = window_copy_search_lr(gd, sgd, &mut px, i, fx, (*gd).sx, cis);
                if found != 0 {
                    *foundlen = (*sgd).sx
                }
            }
            if found != 0 {
                break;
            }
            fx = 0 as libc::c_int as u_int;
            i = i.wrapping_add(1)
        }
    } else {
        *foundlen = 0 as libc::c_int as u_int;
        i = fy.wrapping_add(1 as libc::c_int as libc::c_uint);
        while endline < i {
            if regex != 0 {
                found = window_copy_search_rl_regex(
                    gd,
                    &mut px,
                    &mut sx,
                    i.wrapping_sub(1 as libc::c_int as libc::c_uint),
                    0 as libc::c_int as u_int,
                    fx.wrapping_add(1 as libc::c_int as libc::c_uint),
                    &mut reg,
                )
            } else {
                found = window_copy_search_rl(
                    gd,
                    sgd,
                    &mut px,
                    i.wrapping_sub(1 as libc::c_int as libc::c_uint),
                    0 as libc::c_int as u_int,
                    fx.wrapping_add(1 as libc::c_int as libc::c_uint),
                    cis,
                )
            }
            if found != 0 {
                i = i.wrapping_sub(1);
                break;
            } else {
                fx = (*gd).sx.wrapping_sub(1 as libc::c_int as libc::c_uint);
                i = i.wrapping_sub(1)
            }
        }
    }
    if regex != 0 {
        regfree(&mut reg);
    }
    if found != 0 {
        window_copy_scroll_to(wme, px, i, 1 as libc::c_int);
        return 1 as libc::c_int;
    }
    if wrap != 0 {
        return window_copy_search_jump(
            wme,
            gd,
            sgd,
            if direction != 0 {
                0 as libc::c_int as libc::c_uint
            } else {
                (*gd).sx.wrapping_sub(1 as libc::c_int as libc::c_uint)
            },
            if direction != 0 {
                0 as libc::c_int as libc::c_uint
            } else {
                (*gd)
                    .hsize
                    .wrapping_add((*gd).sy)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
            },
            fy,
            cis,
            0 as libc::c_int,
            direction,
            regex,
            foundlen,
        );
    }
    return 0 as libc::c_int;
}
/*
 * Search in for text searchstr. If direction is 0 then search up, otherwise
 * down.
 */
unsafe extern "C" fn window_copy_search(
    mut wme: *mut window_mode_entry,
    mut direction: libc::c_int,
    mut regex: libc::c_int,
    mut again: libc::c_int,
) -> libc::c_int {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
    let mut ss: screen = screen {
        title: 0 as *mut libc::c_char,
        path: 0 as *mut libc::c_char,
        titles: 0 as *mut crate::screen::screen_titles,
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
        saved_cell: GridCell {
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
        },
        saved_flags: 0,
        tabs: 0 as *mut bitstr_t,
        sel: 0 as *mut crate::screen::screen_sel,
        write_list: 0 as *mut crate::screen_write::screen_write_collect_line,
    };
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut gd: *mut grid = (*s).grid;
    let mut str: *const libc::c_char = (*data).searchstr;
    let mut fx: u_int = 0;
    let mut fy: u_int = 0;
    let mut endline: u_int = 0;
    let mut i: u_int = 0;
    let mut foundlen: u_int = 0;
    let mut wrapflag: libc::c_int = 0;
    let mut cis: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut visible_only: libc::c_int = 0;
    if regex != 0
        && *str
            .offset(strcspn(str, b"^$*+()?[].\\\x00" as *const u8 as *const libc::c_char) as isize)
            as libc::c_int
            == '\u{0}' as i32
    {
        regex = 0 as libc::c_int
    }
    if (*data).timeout != 0 {
        return 0 as libc::c_int;
    }
    if (*wp).searchstr.is_null() || (*wp).searchregex != regex {
        visible_only = 0 as libc::c_int
    } else {
        visible_only = (strcmp((*wp).searchstr, str) == 0 as libc::c_int) as libc::c_int
    }
    free((*wp).searchstr as *mut libc::c_void);
    (*wp).searchstr = xstrdup(str);
    (*wp).searchregex = regex;
    fx = (*data).cx;
    fy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_sub((*data).oy)
        .wrapping_add((*data).cy);
    screen_init(
        &mut ss,
        screen_write_strlen(b"%s\x00" as *const u8 as *const libc::c_char, str) as u_int,
        1 as libc::c_int as u_int,
        0 as libc::c_int as u_int,
    );
    screen_write_start(&mut ctx, &mut ss);
    screen_write_nputs(
        &mut ctx as *mut screen_write_ctx,
        -(1 as libc::c_int) as ssize_t,
        &grid_default_cell as *const GridCell,
        b"%s\x00" as *const u8 as *const libc::c_char,
        str,
    );
    screen_write_stop(&mut ctx);
    wrapflag = options_get_number(
        (*(*wp).window).options,
        b"wrap-search\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    cis = window_copy_is_lowercase(str);
    if direction != 0 {
        endline = (*gd)
            .hsize
            .wrapping_add((*gd).sy)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else {
        if again != 0 {
            window_copy_move_left(s, &mut fx, &mut fy, wrapflag);
        }
        endline = 0 as libc::c_int as u_int
    }
    found = window_copy_search_jump(
        wme,
        gd,
        ss.grid,
        fx,
        fy,
        endline,
        cis,
        wrapflag,
        direction,
        regex,
        &mut foundlen,
    );
    if found != 0 {
        window_copy_search_marks(wme, &mut ss, regex, visible_only);
        if foundlen != 0 as libc::c_int as libc::c_uint {
            i = 0 as libc::c_int as u_int;
            while i < foundlen {
                window_copy_cursor_right(wme, 1 as libc::c_int);
                i = i.wrapping_add(1)
            }
        }
    }
    window_copy_redraw_screen(wme);
    screen_free(&mut ss);
    return found;
}
unsafe extern "C" fn window_copy_visible_lines(
    mut data: *mut window_copy_mode_data,
    mut start: *mut u_int,
    mut end: *mut u_int,
) {
    let mut gd: *mut grid = (*(*data).backing).grid;
    let mut gl: *const grid_line = 0 as *const grid_line;
    *start = (*gd).hsize.wrapping_sub((*data).oy);
    while *start > 0 as libc::c_int as libc::c_uint {
        gl = grid_peek_line(gd, (*start).wrapping_sub(1 as libc::c_int as libc::c_uint));
        if !(*gl).flags & 0x1 as libc::c_int != 0 {
            break;
        }
        *start = (*start).wrapping_sub(1)
    }
    *end = (*gd).hsize.wrapping_sub((*data).oy).wrapping_add((*gd).sy);
}
unsafe extern "C" fn window_copy_search_mark_at(
    mut data: *mut window_copy_mode_data,
    mut px: u_int,
    mut py: u_int,
    mut at: *mut u_int,
) -> libc::c_int {
    let mut s: *mut screen = (*data).backing;
    let mut gd: *mut grid = (*s).grid;
    if py < (*gd).hsize.wrapping_sub((*data).oy) {
        return -(1 as libc::c_int);
    }
    if py
        > (*gd)
            .hsize
            .wrapping_sub((*data).oy)
            .wrapping_add((*gd).sy)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return -(1 as libc::c_int);
    }
    *at = py
        .wrapping_sub((*gd).hsize.wrapping_sub((*data).oy))
        .wrapping_mul((*gd).sx)
        .wrapping_add(px);
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_copy_search_marks(
    mut wme: *mut window_mode_entry,
    mut ssp: *mut screen,
    mut regex: libc::c_int,
    mut visible_only: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = (*data).backing;
    let mut ss: screen = screen {
        title: 0 as *mut libc::c_char,
        path: 0 as *mut libc::c_char,
        titles: 0 as *mut crate::screen::screen_titles,
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
        saved_cell: GridCell {
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
        },
        saved_flags: 0,
        tabs: 0 as *mut bitstr_t,
        sel: 0 as *mut crate::screen::screen_sel,
        write_list: 0 as *mut crate::screen_write::screen_write_collect_line,
    };
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut gd: *mut grid = (*s).grid;
    let mut found: libc::c_int = 0;
    let mut cis: libc::c_int = 0;
    let mut which: libc::c_int = -(1 as libc::c_int);
    let mut stopped: libc::c_int = 0 as libc::c_int;
    let mut cflags: libc::c_int = 1 as libc::c_int;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut i: u_int = 0;
    let mut b: u_int = 0;
    let mut nfound: u_int = 0 as libc::c_int as u_int;
    let mut width: u_int = 0;
    let mut ssize: u_int = 1 as libc::c_int as u_int;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    let mut sbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reg: regex_t = regex_t {
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
    let mut stop: uint64_t = 0 as libc::c_int as uint64_t;
    let mut tstart: uint64_t = 0;
    let mut t: uint64_t = 0;
    if ssp.is_null() {
        width = screen_write_strlen(
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*data).searchstr,
        ) as u_int;
        screen_init(
            &mut ss,
            width,
            1 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
        );
        screen_write_start(&mut ctx, &mut ss);
        screen_write_nputs(
            &mut ctx as *mut screen_write_ctx,
            -(1 as libc::c_int) as ssize_t,
            &grid_default_cell as *const GridCell,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*data).searchstr,
        );
        screen_write_stop(&mut ctx);
        ssp = &mut ss
    } else {
        width = (*(*ssp).grid).sx
    }
    cis = window_copy_is_lowercase((*data).searchstr);
    if regex != 0 {
        sbuf = xmalloc(ssize as size_t) as *mut libc::c_char;
        *sbuf.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        sbuf = window_copy_stringify(
            (*ssp).grid,
            0 as libc::c_int as u_int,
            0 as libc::c_int as u_int,
            (*(*ssp).grid).sx,
            sbuf,
            &mut ssize,
        );
        if cis != 0 {
            cflags |= (1 as libc::c_int) << 1 as libc::c_int
        }
        if regcomp(&mut reg, sbuf, cflags) != 0 as libc::c_int {
            free(sbuf as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        free(sbuf as *mut libc::c_void);
    }
    tstart = get_timer();
    if visible_only != 0 {
        window_copy_visible_lines(data, &mut start, &mut end);
    } else {
        start = 0 as libc::c_int as u_int;
        end = (*gd).hsize.wrapping_add((*gd).sy);
        stop = get_timer().wrapping_add(200 as libc::c_int as libc::c_ulong)
    }
    loop {
        free((*data).searchmark as *mut libc::c_void);
        (*data).searchmark = xcalloc((*gd).sx as size_t, (*gd).sy as size_t) as *mut u_char;
        (*data).searchgen = 1 as libc::c_int as u_char;
        py = start;
        while py < end {
            px = 0 as libc::c_int as u_int;
            loop {
                if regex != 0 {
                    found = window_copy_search_lr_regex(
                        gd,
                        &mut px,
                        &mut width,
                        py,
                        px,
                        (*gd).sx,
                        &mut reg,
                    );
                    if found == 0 {
                        break;
                    }
                } else {
                    found = window_copy_search_lr(gd, (*ssp).grid, &mut px, py, px, (*gd).sx, cis);
                    if found == 0 {
                        break;
                    }
                }
                nfound = nfound.wrapping_add(1);
                if px == (*data).cx
                    && py
                        == (*gd)
                            .hsize
                            .wrapping_add((*data).cy)
                            .wrapping_sub((*data).oy)
                {
                    which = nfound as libc::c_int
                }
                if window_copy_search_mark_at(data, px, py, &mut b) == 0 as libc::c_int {
                    if b.wrapping_add(width) > (*gd).sx.wrapping_mul((*gd).sy) {
                        width = (*gd).sx.wrapping_mul((*gd).sy).wrapping_sub(b)
                    }
                    i = b;
                    while i < b.wrapping_add(width) {
                        *(*data).searchmark.offset(i as isize) = (*data).searchgen;
                        i = i.wrapping_add(1)
                    }
                    if (*data).searchgen as libc::c_int
                        == 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    {
                        (*data).searchgen = 1 as libc::c_int as u_char
                    } else {
                        (*data).searchgen = (*data).searchgen.wrapping_add(1)
                    }
                }
                px = (px as libc::c_uint).wrapping_add(width) as u_int as u_int
            }
            t = get_timer();
            if t.wrapping_sub(tstart) > 10000 as libc::c_int as libc::c_ulong {
                (*data).timeout = 1 as libc::c_int;
                break;
            } else if stop != 0 as libc::c_int as libc::c_ulong && t > stop {
                stopped = 1 as libc::c_int;
                break;
            } else {
                py = py.wrapping_add(1)
            }
        }
        if (*data).timeout != 0 {
            window_copy_clear_marks(wme);
            break;
        } else if stopped != 0 && stop != 0 as libc::c_int as libc::c_ulong {
            /* Try again but just the visible context. */
            window_copy_visible_lines(data, &mut start, &mut end);
            stop = 0 as libc::c_int as uint64_t
        } else {
            if visible_only == 0 {
                if stopped != 0 {
                    (*data).searchthis = -(1 as libc::c_int);
                    if nfound > 1000 as libc::c_int as libc::c_uint {
                        (*data).searchcount = 1000 as libc::c_int
                    } else if nfound > 100 as libc::c_int as libc::c_uint {
                        (*data).searchcount = 100 as libc::c_int
                    } else if nfound > 10 as libc::c_int as libc::c_uint {
                        (*data).searchcount = 10 as libc::c_int
                    } else {
                        (*data).searchcount = -(1 as libc::c_int)
                    }
                    (*data).searchmore = 1 as libc::c_int
                } else {
                    if which != -(1 as libc::c_int) {
                        (*data).searchthis = (1 as libc::c_int as libc::c_uint)
                            .wrapping_add(nfound)
                            .wrapping_sub(which as libc::c_uint)
                            as libc::c_int
                    } else {
                        (*data).searchthis = -(1 as libc::c_int)
                    }
                    (*data).searchcount = nfound as libc::c_int;
                    (*data).searchmore = 0 as libc::c_int
                }
            }
            break;
        }
    }
    if ssp == &mut ss as *mut screen {
        screen_free(&mut ss);
    }
    if regex != 0 {
        regfree(&mut reg);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn window_copy_clear_marks(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    free((*data).searchmark as *mut libc::c_void);
    (*data).searchmark = 0 as *mut u_char;
}
unsafe extern "C" fn window_copy_search_up(
    mut wme: *mut window_mode_entry,
    mut regex: libc::c_int,
    mut again: libc::c_int,
) -> libc::c_int {
    return window_copy_search(wme, 0 as libc::c_int, regex, again);
}
unsafe extern "C" fn window_copy_search_down(
    mut wme: *mut window_mode_entry,
    mut regex: libc::c_int,
    mut again: libc::c_int,
) -> libc::c_int {
    return window_copy_search(wme, 1 as libc::c_int, regex, again);
}
unsafe extern "C" fn window_copy_goto_line(
    mut wme: *mut window_mode_entry,
    mut linestr: *const libc::c_char,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut lineno: libc::c_int = 0;
    lineno = strtonum(
        linestr,
        -(1 as libc::c_int) as libc::c_longlong,
        2147483647 as libc::c_int as libc::c_longlong,
        &mut errstr,
    ) as libc::c_int;
    if !errstr.is_null() {
        return;
    }
    if lineno < 0 as libc::c_int || lineno as u_int > (*(*(*data).backing).grid).hsize {
        lineno = (*(*(*data).backing).grid).hsize as libc::c_int
    }
    (*data).oy = lineno as u_int;
    window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
    window_copy_redraw_screen(wme);
}
unsafe extern "C" fn window_copy_match_start_end(
    mut data: *mut window_copy_mode_data,
    mut at: u_int,
    mut start: *mut u_int,
    mut end: *mut u_int,
) {
    let mut gd: *mut grid = (*(*data).backing).grid;
    let mut last: u_int = (*gd)
        .sy
        .wrapping_mul((*gd).sx)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut mark: u_char = *(*data).searchmark.offset(at as isize);
    *end = at;
    *start = *end;
    while *start != 0 as libc::c_int as libc::c_uint
        && *(*data).searchmark.offset(*start as isize) as libc::c_int == mark as libc::c_int
    {
        *start = (*start).wrapping_sub(1)
    }
    if *(*data).searchmark.offset(*start as isize) as libc::c_int != mark as libc::c_int {
        *start = (*start).wrapping_add(1)
    }
    while *end != last
        && *(*data).searchmark.offset(*end as isize) as libc::c_int == mark as libc::c_int
    {
        *end = (*end).wrapping_add(1)
    }
    if *(*data).searchmark.offset(*end as isize) as libc::c_int != mark as libc::c_int {
        *end = (*end).wrapping_sub(1)
    };
}
unsafe extern "C" fn window_copy_match_at_cursor(
    mut data: *mut window_copy_mode_data,
) -> *mut libc::c_char {
    let mut gd: *mut grid = (*(*data).backing).grid;
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
    let mut at: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    let mut cy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut sx: u_int = (*(*(*data).backing).grid).sx;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0 as libc::c_int as size_t;
    if (*data).searchmark.is_null() {
        return 0 as *mut libc::c_char;
    }
    cy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_sub((*data).oy)
        .wrapping_add((*data).cy);
    if window_copy_search_mark_at(data, (*data).cx, cy, &mut at) != 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if *(*data).searchmark.offset(at as isize) as libc::c_int == 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    window_copy_match_start_end(data, at, &mut start, &mut end);
    /*
     * Cells will not be set in the marked array unless they are valid text
     * and wrapping will be taken care of, so we can just copy.
     */
    at = start;
    while at <= end {
        py = at.wrapping_div(sx);
        px = at.wrapping_sub(py.wrapping_mul(sx));
        grid_get_cell(
            gd,
            px,
            (*gd).hsize.wrapping_add(py).wrapping_sub((*data).oy),
            &mut gc,
        );
        buf = xrealloc(
            buf as *mut libc::c_void,
            len.wrapping_add(gc.data.size as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(
            buf.offset(len as isize) as *mut libc::c_void,
            gc.data.data.as_mut_ptr() as *const libc::c_void,
            gc.data.size as libc::c_ulong,
        );
        len =
            (len as libc::c_ulong).wrapping_add(gc.data.size as libc::c_ulong) as size_t as size_t;
        at = at.wrapping_add(1)
    }
    if len != 0 as libc::c_int as libc::c_ulong {
        *buf.offset(len as isize) = '\u{0}' as i32 as libc::c_char
    }
    return buf;
}
unsafe extern "C" fn window_copy_update_style(
    mut wme: *mut window_mode_entry,
    mut fx: u_int,
    mut fy: u_int,
    mut gc: *mut GridCell,
    mut mgc: *const GridCell,
    mut cgc: *const GridCell,
    mut mkgc: *const GridCell,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut mark: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    let mut cy: u_int = 0;
    let mut cursor: u_int = 0;
    let mut current: u_int = 0;
    let mut inv: libc::c_int = 0 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    if (*data).showmark != 0 && fy == (*data).my {
        (*gc).attr = (*mkgc).attr;
        if fx == (*data).mx {
            inv = 1 as libc::c_int
        }
        if inv != 0 {
            (*gc).fg = (*mkgc).bg;
            (*gc).bg = (*mkgc).fg
        } else {
            (*gc).fg = (*mkgc).fg;
            (*gc).bg = (*mkgc).bg
        }
    }
    if (*data).searchmark.is_null() {
        return;
    }
    if window_copy_search_mark_at(data, fx, fy, &mut current) != 0 as libc::c_int {
        return;
    }
    mark = *(*data).searchmark.offset(current as isize) as u_int;
    if mark == 0 as libc::c_int as libc::c_uint {
        return;
    }
    cy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_sub((*data).oy)
        .wrapping_add((*data).cy);
    if window_copy_search_mark_at(data, (*data).cx, cy, &mut cursor) == 0 as libc::c_int {
        if *(*data).searchmark.offset(cursor as isize) as libc::c_uint == mark {
            found = 1 as libc::c_int
        } else if cursor != 0 as libc::c_int as libc::c_uint {
            cursor = cursor.wrapping_sub(1);
            if *(*data).searchmark.offset(cursor as isize) as libc::c_uint == mark {
                found = 1 as libc::c_int
            }
        }
        if found != 0 {
            window_copy_match_start_end(data, cursor, &mut start, &mut end);
            if current >= start && current <= end {
                (*gc).attr = (*cgc).attr;
                if inv != 0 {
                    (*gc).fg = (*cgc).bg;
                    (*gc).bg = (*cgc).fg
                } else {
                    (*gc).fg = (*cgc).fg;
                    (*gc).bg = (*cgc).bg
                }
                return;
            }
        }
    }
    (*gc).attr = (*mgc).attr;
    if inv != 0 {
        (*gc).fg = (*mgc).bg;
        (*gc).bg = (*mgc).fg
    } else {
        (*gc).fg = (*mgc).fg;
        (*gc).bg = (*mgc).bg
    };
}
unsafe extern "C" fn window_copy_write_one(
    mut wme: *mut window_mode_entry,
    mut ctx: *mut screen_write_ctx,
    mut py: u_int,
    mut fy: u_int,
    mut nx: u_int,
    mut mgc: *const GridCell,
    mut cgc: *const GridCell,
    mut mkgc: *const GridCell,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut gd: *mut grid = (*(*data).backing).grid;
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
    let mut fx: u_int = 0;
    screen_write_cursormove(ctx, 0 as libc::c_int, py as libc::c_int, 0 as libc::c_int);
    fx = 0 as libc::c_int as u_int;
    while fx < nx {
        grid_get_cell(gd, fx, fy, &mut gc);
        if fx.wrapping_add(gc.data.width as libc::c_uint) <= nx {
            window_copy_update_style(wme, fx, fy, &mut gc, mgc, cgc, mkgc);
            screen_write_cell(ctx, &mut gc);
        }
        fx = fx.wrapping_add(1)
    }
}
unsafe extern "C" fn window_copy_write_line(
    mut wme: *mut window_mode_entry,
    mut ctx: *mut screen_write_ctx,
    mut py: u_int,
) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut oo: *mut crate::options::options = (*(*wp).window).options;
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
    let mut mgc: GridCell = GridCell {
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
    let mut cgc: GridCell = GridCell {
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
    let mut mkgc: GridCell = GridCell {
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
    let mut hdr: [libc::c_char; 512] = [0; 512];
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut hsize: u_int = (*(*(*data).backing).grid).hsize;
    style_apply(
        &mut gc,
        oo,
        b"mode-style\x00" as *const u8 as *const libc::c_char,
        0 as *mut crate::format::format_tree,
    );
    gc.flags = (gc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
    style_apply(
        &mut mgc,
        oo,
        b"copy-mode-match-style\x00" as *const u8 as *const libc::c_char,
        0 as *mut crate::format::format_tree,
    );
    mgc.flags = (mgc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
    style_apply(
        &mut cgc,
        oo,
        b"copy-mode-current-match-style\x00" as *const u8 as *const libc::c_char,
        0 as *mut crate::format::format_tree,
    );
    cgc.flags = (cgc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
    style_apply(
        &mut mkgc,
        oo,
        b"copy-mode-mark-style\x00" as *const u8 as *const libc::c_char,
        0 as *mut crate::format::format_tree,
    );
    mkgc.flags = (mkgc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
    if py == 0 as libc::c_int as libc::c_uint
        && (*s).rupper < (*s).rlower
        && (*data).hide_position == 0
    {
        if (*data).searchmark.is_null() {
            if (*data).timeout != 0 {
                size = xsnprintf(
                    hdr.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                    b"(timed out) [%u/%u]\x00" as *const u8 as *const libc::c_char,
                    (*data).oy,
                    hsize,
                ) as size_t
            } else {
                size = xsnprintf(
                    hdr.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                    b"[%u/%u]\x00" as *const u8 as *const libc::c_char,
                    (*data).oy,
                    hsize,
                ) as size_t
            }
        } else if (*data).searchcount == -(1 as libc::c_int) {
            size = xsnprintf(
                hdr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"[%u/%u]\x00" as *const u8 as *const libc::c_char,
                (*data).oy,
                hsize,
            ) as size_t
        } else if (*data).searchthis == -(1 as libc::c_int) {
            size = xsnprintf(
                hdr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"(%d%s results) [%u/%u]\x00" as *const u8 as *const libc::c_char,
                (*data).searchcount,
                if (*data).searchmore != 0 {
                    b"+\x00" as *const u8 as *const libc::c_char
                } else {
                    b"\x00" as *const u8 as *const libc::c_char
                },
                (*data).oy,
                hsize,
            ) as size_t
        } else {
            size = xsnprintf(
                hdr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
                b"(%d/%d results) [%u/%u]\x00" as *const u8 as *const libc::c_char,
                (*data).searchthis,
                (*data).searchcount,
                (*data).oy,
                hsize,
            ) as size_t
        }
        if size > (*(*s).grid).sx as libc::c_ulong {
            size = (*(*s).grid).sx as size_t
        }
        screen_write_cursormove(
            ctx,
            ((*(*s).grid).sx as libc::c_ulong).wrapping_sub(size) as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        screen_write_puts(
            ctx,
            &mut gc as *mut GridCell,
            b"%s\x00" as *const u8 as *const libc::c_char,
            hdr.as_mut_ptr(),
        );
    } else {
        size = 0 as libc::c_int as size_t
    }
    if size < (*(*s).grid).sx as libc::c_ulong {
        window_copy_write_one(
            wme,
            ctx,
            py,
            hsize.wrapping_sub((*data).oy).wrapping_add(py),
            ((*(*s).grid).sx as libc::c_ulong).wrapping_sub(size) as u_int,
            &mut mgc,
            &mut cgc,
            &mut mkgc,
        );
    }
    if py == (*data).cy && (*data).cx == (*(*s).grid).sx {
        screen_write_cursormove(
            ctx,
            (*(*s).grid)
                .sx
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
            py as libc::c_int,
            0 as libc::c_int,
        );
        screen_write_putc(ctx, &grid_default_cell, '$' as i32 as u_char);
    };
}
unsafe extern "C" fn window_copy_write_lines(
    mut wme: *mut window_mode_entry,
    mut ctx: *mut screen_write_ctx,
    mut py: u_int,
    mut ny: u_int,
) {
    let mut yy: u_int = 0;
    yy = py;
    while yy < py.wrapping_add(ny) {
        window_copy_write_line(wme, ctx, py);
        yy = yy.wrapping_add(1)
    }
}
unsafe extern "C" fn window_copy_redraw_selection(
    mut wme: *mut window_mode_entry,
    mut old_y: u_int,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut gd: *mut grid = (*(*data).backing).grid;
    let mut new_y: u_int = 0;
    let mut start: u_int = 0;
    let mut end: u_int = 0;
    new_y = (*data).cy;
    if old_y <= new_y {
        start = old_y;
        end = new_y
    } else {
        start = new_y;
        end = old_y
    }
    /*
     * In word selection mode the first word on the line below the cursor
     * might be selected, so add this line to the redraw area.
     */
    if (*data).selflag as libc::c_uint == SEL_WORD as libc::c_int as libc::c_uint {
        /* Last grid line in data coordinates. */
        if end
            < (*gd)
                .sy
                .wrapping_add((*data).oy)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            end = end.wrapping_add(1)
        }
    }
    window_copy_redraw_lines(
        wme,
        start,
        end.wrapping_sub(start)
            .wrapping_add(1 as libc::c_int as libc::c_uint),
    );
}
unsafe extern "C" fn window_copy_redraw_lines(
    mut wme: *mut window_mode_entry,
    mut py: u_int,
    mut ny: u_int,
) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut i: u_int = 0;
    screen_write_start_pane(&mut ctx, wp, 0 as *mut screen);
    i = py;
    while i < py.wrapping_add(ny) {
        window_copy_write_line(wme, &mut ctx, i);
        i = i.wrapping_add(1)
    }
    screen_write_cursormove(
        &mut ctx,
        (*data).cx as libc::c_int,
        (*data).cy as libc::c_int,
        0 as libc::c_int,
    );
    screen_write_stop(&mut ctx);
}
unsafe extern "C" fn window_copy_redraw_screen(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    window_copy_redraw_lines(wme, 0 as libc::c_int as u_int, (*(*data).screen.grid).sy);
}
unsafe extern "C" fn window_copy_synchronize_cursor_end(
    mut wme: *mut window_mode_entry,
    mut begin: libc::c_int,
    mut no_reset: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    yy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    match (*data).selflag as libc::c_uint {
        1 => {
            xx = (*data).cx;
            if !(no_reset != 0) {
                begin = 0 as libc::c_int;
                if (*data).dy > yy || (*data).dy == yy && (*data).dx > xx {
                    /* Right to left selection. */
                    window_copy_cursor_previous_word_pos(
                        wme,
                        (*data).ws,
                        0 as libc::c_int,
                        &mut xx,
                        &mut yy,
                    );
                    begin = 1 as libc::c_int;
                    /* Reset the end. */
                    (*data).endselx = (*data).endselrx;
                    (*data).endsely = (*data).endselry
                } else {
                    /* Left to right selection. */
                    if xx >= window_copy_find_length(wme, yy)
                        || window_copy_in_set(
                            wme,
                            xx.wrapping_add(1 as libc::c_int as libc::c_uint),
                            yy,
                            (*data).ws,
                        ) == 0
                    {
                        window_copy_cursor_next_word_end_pos(wme, (*data).ws, &mut xx, &mut yy);
                    }
                    /* Reset the start. */
                    (*data).selx = (*data).selrx;
                    (*data).sely = (*data).selry
                }
            }
        }
        2 => {
            if no_reset != 0 {
                xx = (*data).cx
            } else {
                begin = 0 as libc::c_int;
                if (*data).dy > yy {
                    /* Right to left selection. */
                    xx = 0 as libc::c_int as u_int;
                    begin = 1 as libc::c_int;
                    /* Reset the end. */
                    (*data).endselx = (*data).endselrx;
                    (*data).endsely = (*data).endselry
                } else {
                    /* Left to right selection. */
                    xx = window_copy_find_length(wme, yy);
                    /* Reset the start. */
                    (*data).selx = (*data).selrx;
                    (*data).sely = (*data).selry
                }
            }
        }
        0 => xx = (*data).cx,
        _ => {}
    }
    if begin != 0 {
        (*data).selx = xx;
        (*data).sely = yy
    } else {
        (*data).endselx = xx;
        (*data).endsely = yy
    };
}
unsafe extern "C" fn window_copy_synchronize_cursor(
    mut wme: *mut window_mode_entry,
    mut no_reset: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    match (*data).cursordrag as libc::c_uint {
        1 => {
            window_copy_synchronize_cursor_end(wme, 0 as libc::c_int, no_reset);
        }
        2 => {
            window_copy_synchronize_cursor_end(wme, 1 as libc::c_int, no_reset);
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn window_copy_update_cursor(
    mut wme: *mut window_mode_entry,
    mut cx: u_int,
    mut cy: u_int,
) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    let mut old_cx: u_int = 0;
    let mut old_cy: u_int = 0;
    old_cx = (*data).cx;
    old_cy = (*data).cy;
    (*data).cx = cx;
    (*data).cy = cy;
    if old_cx == (*(*s).grid).sx {
        window_copy_redraw_lines(wme, old_cy, 1 as libc::c_int as u_int);
    }
    if (*data).cx == (*(*s).grid).sx {
        window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
    } else {
        screen_write_start_pane(&mut ctx, wp, 0 as *mut screen);
        screen_write_cursormove(
            &mut ctx,
            (*data).cx as libc::c_int,
            (*data).cy as libc::c_int,
            0 as libc::c_int,
        );
        screen_write_stop(&mut ctx);
    };
}
unsafe extern "C" fn window_copy_start_selection(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    (*data).selx = (*data).cx;
    (*data).sely = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).endselx = (*data).selx;
    (*data).endsely = (*data).sely;
    (*data).cursordrag = CURSORDRAG_ENDSEL;
    window_copy_set_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
}
unsafe extern "C" fn window_copy_adjust_selection(
    mut wme: *mut window_mode_entry,
    mut selx: *mut u_int,
    mut sely: *mut u_int,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut ty: u_int = 0;
    let mut relpos: libc::c_int = 0;
    sx = *selx;
    sy = *sely;
    ty = (*(*(*data).backing).grid).hsize.wrapping_sub((*data).oy);
    if sy < ty {
        relpos = WINDOW_COPY_REL_POS_ABOVE as libc::c_int;
        if (*data).rectflag == 0 {
            sx = 0 as libc::c_int as u_int
        }
        sy = 0 as libc::c_int as u_int
    } else if sy
        > ty.wrapping_add((*(*s).grid).sy)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        relpos = WINDOW_COPY_REL_POS_BELOW as libc::c_int;
        if (*data).rectflag == 0 {
            sx = (*(*s).grid)
                .sx
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        }
        sy = (*(*s).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else {
        relpos = WINDOW_COPY_REL_POS_ON_SCREEN as libc::c_int;
        sy = (sy as libc::c_uint).wrapping_sub(ty) as u_int as u_int
    }
    *selx = sx;
    *sely = sy;
    return relpos;
}
unsafe extern "C" fn window_copy_update_selection(
    mut wme: *mut window_mode_entry,
    mut may_redraw: libc::c_int,
    mut no_reset: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    if (*s).sel.is_null()
        && (*data).lineflag as libc::c_uint == LINE_SEL_NONE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    return window_copy_set_selection(wme, may_redraw, no_reset);
}
unsafe extern "C" fn window_copy_set_selection(
    mut wme: *mut window_mode_entry,
    mut may_redraw: libc::c_int,
    mut no_reset: libc::c_int,
) -> libc::c_int {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut oo: *mut crate::options::options = (*(*wp).window).options;
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
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut cy: u_int = 0;
    let mut endsx: u_int = 0;
    let mut endsy: u_int = 0;
    let mut startrelpos: libc::c_int = 0;
    let mut endrelpos: libc::c_int = 0;
    window_copy_synchronize_cursor(wme, no_reset);
    /* Adjust the selection. */
    sx = (*data).selx;
    sy = (*data).sely;
    startrelpos = window_copy_adjust_selection(wme, &mut sx, &mut sy);
    /* Adjust the end of selection. */
    endsx = (*data).endselx;
    endsy = (*data).endsely;
    endrelpos = window_copy_adjust_selection(wme, &mut endsx, &mut endsy);
    /* Selection is outside of the current screen */
    if startrelpos == endrelpos && startrelpos != WINDOW_COPY_REL_POS_ON_SCREEN as libc::c_int {
        screen_hide_selection(s);
        return 0 as libc::c_int;
    }
    /* Set colours and selection. */
    style_apply(
        &mut gc,
        oo,
        b"mode-style\x00" as *const u8 as *const libc::c_char,
        0 as *mut crate::format::format_tree,
    );
    gc.flags = (gc.flags as libc::c_int | 0x20 as libc::c_int) as u_char;
    screen_set_selection(
        s,
        sx,
        sy,
        endsx,
        endsy,
        (*data).rectflag as u_int,
        (*data).modekeys,
        &mut gc,
    );
    if (*data).rectflag != 0 && may_redraw != 0 {
        /*
         * Can't rely on the caller to redraw the right lines for
         * rectangle selection - find the highest line and the number
         * of lines, and redraw just past that in both directions
         */
        cy = (*data).cy;
        if (*data).cursordrag as libc::c_uint == CURSORDRAG_ENDSEL as libc::c_int as libc::c_uint {
            if sy < cy {
                window_copy_redraw_lines(
                    wme,
                    sy,
                    cy.wrapping_sub(sy)
                        .wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            } else {
                window_copy_redraw_lines(
                    wme,
                    cy,
                    sy.wrapping_sub(cy)
                        .wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            }
        } else if endsy < cy {
            window_copy_redraw_lines(
                wme,
                endsy,
                cy.wrapping_sub(endsy)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
            );
        } else {
            window_copy_redraw_lines(
                wme,
                cy,
                endsy
                    .wrapping_sub(cy)
                    .wrapping_add(1 as libc::c_int as libc::c_uint),
            );
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn window_copy_get_selection(
    mut wme: *mut window_mode_entry,
    mut len: *mut size_t,
) -> *mut libc::c_void {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut off: size_t = 0;
    let mut i: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut ex: u_int = 0;
    let mut ey: u_int = 0;
    let mut ey_last: u_int = 0;
    let mut firstsx: u_int = 0;
    let mut lastex: u_int = 0;
    let mut restex: u_int = 0;
    let mut restsx: u_int = 0;
    let mut selx: u_int = 0;
    let mut keys: libc::c_int = 0;
    if (*data).screen.sel.is_null()
        && (*data).lineflag as libc::c_uint == LINE_SEL_NONE as libc::c_int as libc::c_uint
    {
        buf = window_copy_match_at_cursor(data);
        if !buf.is_null() {
            *len = strlen(buf)
        } else {
            *len = 0 as libc::c_int as size_t
        }
        return buf as *mut libc::c_void;
    }
    buf = xmalloc(1 as libc::c_int as size_t) as *mut libc::c_char;
    off = 0 as libc::c_int as size_t;
    *buf = '\u{0}' as i32 as libc::c_char;
    /*
     * The selection extends from selx,sely to (adjusted) cx,cy on
     * the base screen.
     */
    /* Find start and end. */
    xx = (*data).endselx;
    yy = (*data).endsely;
    if yy < (*data).sely || yy == (*data).sely && xx < (*data).selx {
        sx = xx;
        sy = yy;
        ex = (*data).selx;
        ey = (*data).sely
    } else {
        sx = (*data).selx;
        sy = (*data).sely;
        ex = xx;
        ey = yy
    }
    /* Trim ex to end of line. */
    ey_last = window_copy_find_length(wme, ey);
    if ex > ey_last {
        ex = ey_last
    }
    /*
     * Deal with rectangle-copy if necessary; four situations: start of
     * first line (firstsx), end of last line (lastex), start (restsx) and
     * end (restex) of all other lines.
     */
    xx = (*(*s).grid).sx;
    /*
     * Behave according to mode-keys. If it is emacs, copy like emacs,
     * keeping the top-left-most character, and dropping the
     * bottom-right-most, regardless of copy direction. If it is vi, also
     * keep bottom-right-most character.
     */
    keys = options_get_number(
        (*(*wp).window).options,
        b"mode-keys\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if (*data).rectflag != 0 {
        /*
         * Need to ignore the column with the cursor in it, which for
         * rectangular copy means knowing which side the cursor is on.
         */
        if (*data).cursordrag as libc::c_uint == CURSORDRAG_ENDSEL as libc::c_int as libc::c_uint {
            selx = (*data).selx
        } else {
            selx = (*data).endselx
        }
        if selx < (*data).cx {
            /* Selection start is on the left. */
            if keys == 0 as libc::c_int {
                lastex = (*data).cx;
                restex = (*data).cx
            } else {
                lastex = (*data).cx.wrapping_add(1 as libc::c_int as libc::c_uint);
                restex = (*data).cx.wrapping_add(1 as libc::c_int as libc::c_uint)
            }
            firstsx = selx;
            restsx = selx
        } else {
            /* Cursor is on the left. */
            lastex = selx.wrapping_add(1 as libc::c_int as libc::c_uint);
            restex = selx.wrapping_add(1 as libc::c_int as libc::c_uint);
            firstsx = (*data).cx;
            restsx = (*data).cx
        }
    } else {
        if keys == 0 as libc::c_int {
            lastex = ex
        } else {
            lastex = ex.wrapping_add(1 as libc::c_int as libc::c_uint)
        }
        restex = xx;
        firstsx = sx;
        restsx = 0 as libc::c_int as u_int
    }
    /* Copy the lines. */
    i = sy;
    while i <= ey {
        window_copy_copy_line(
            wme,
            &mut buf,
            &mut off,
            i,
            if i == sy { firstsx } else { restsx },
            if i == ey { lastex } else { restex },
        );
        i = i.wrapping_add(1)
    }
    /* Don't bother if no data. */
    if off == 0 as libc::c_int as libc::c_ulong {
        free(buf as *mut libc::c_void); /* remove final \n (unless at end in vi mode) */
        *len = 0 as libc::c_int as size_t;
        return 0 as *mut libc::c_void;
    }
    if keys == 0 as libc::c_int || lastex <= ey_last {
        off = (off as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    }
    *len = off;
    return buf as *mut libc::c_void;
}
unsafe extern "C" fn window_copy_copy_buffer(
    mut wme: *mut window_mode_entry,
    mut prefix: *const libc::c_char,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    if options_get_number(
        global_options,
        b"set-clipboard\x00" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int as libc::c_longlong
    {
        screen_write_start_pane(&mut ctx, wp, 0 as *mut screen);
        screen_write_setselection(&mut ctx, buf as *mut u_char, len as u_int);
        screen_write_stop(&mut ctx);
        notify_pane(
            b"pane-set-clipboard\x00" as *const u8 as *const libc::c_char,
            wp,
        );
    }
    paste_add(prefix, buf as *mut libc::c_char, len);
}
unsafe extern "C" fn window_copy_copy_pipe(
    mut wme: *mut window_mode_entry,
    mut s: *mut session,
    mut prefix: *const libc::c_char,
    mut cmd: *const libc::c_char,
) {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    let mut job: *mut crate::job::job = 0 as *mut crate::job::job;
    buf = window_copy_get_selection(wme, &mut len);
    if cmd.is_null() || *cmd as libc::c_int == '\u{0}' as i32 {
        cmd = options_get_string(
            global_options,
            b"copy-command\x00" as *const u8 as *const libc::c_char,
        )
    }
    if !cmd.is_null() && *cmd as libc::c_int != '\u{0}' as i32 {
        job = job_run(
            cmd,
            s,
            0 as *const libc::c_char,
            None,
            None,
            None,
            0 as *mut libc::c_void,
            0x1 as libc::c_int,
            -(1 as libc::c_int),
            -(1 as libc::c_int),
        );
        bufferevent_write(job_get_event(job), buf, len);
    }
    if !buf.is_null() {
        window_copy_copy_buffer(wme, prefix, buf, len);
    };
}
unsafe extern "C" fn window_copy_copy_selection(
    mut wme: *mut window_mode_entry,
    mut prefix: *const libc::c_char,
) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    buf = window_copy_get_selection(wme, &mut len) as *mut libc::c_char;
    if !buf.is_null() {
        window_copy_copy_buffer(wme, prefix, buf as *mut libc::c_void, len);
    };
}
unsafe extern "C" fn window_copy_append_selection(mut wme: *mut window_mode_entry) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pb: *mut crate::paste::paste_buffer = 0 as *mut crate::paste::paste_buffer;
    let mut bufdata: *const libc::c_char = 0 as *const libc::c_char;
    let mut bufname: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut bufsize: size_t = 0;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    buf = window_copy_get_selection(wme, &mut len) as *mut libc::c_char;
    if buf.is_null() {
        return;
    }
    if options_get_number(
        global_options,
        b"set-clipboard\x00" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int as libc::c_longlong
    {
        screen_write_start_pane(&mut ctx, wp, 0 as *mut screen);
        screen_write_setselection(&mut ctx, buf as *mut u_char, len as u_int);
        screen_write_stop(&mut ctx);
        notify_pane(
            b"pane-set-clipboard\x00" as *const u8 as *const libc::c_char,
            wp,
        );
    }
    pb = paste_get_top(&mut bufname);
    if !pb.is_null() {
        bufdata = paste_buffer_data(pb, &mut bufsize);
        buf = xrealloc(buf as *mut libc::c_void, len.wrapping_add(bufsize)) as *mut libc::c_char;
        memmove(
            buf.offset(bufsize as isize) as *mut libc::c_void,
            buf as *const libc::c_void,
            len,
        );
        memcpy(
            buf as *mut libc::c_void,
            bufdata as *const libc::c_void,
            bufsize,
        );
        len = (len as libc::c_ulong).wrapping_add(bufsize) as size_t as size_t
    }
    if paste_set(buf, len, bufname, 0 as *mut *mut libc::c_char) != 0 as libc::c_int {
        free(buf as *mut libc::c_void);
    };
}
unsafe extern "C" fn window_copy_copy_line(
    mut wme: *mut window_mode_entry,
    mut buf: *mut *mut libc::c_char,
    mut off: *mut size_t,
    mut sy: u_int,
    mut sx: u_int,
    mut ex: u_int,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut gd: *mut grid = (*(*data).backing).grid;
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
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut ud: Utf8Data = Utf8Data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut i: u_int = 0;
    let mut xx: u_int = 0;
    let mut wrapped: u_int = 0 as libc::c_int as u_int;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if sx > ex {
        return;
    }
    /*
     * Work out if the line was wrapped at the screen edge and all of it is
     * on screen.
     */
    gl = grid_get_line(gd, sy);
    if (*gl).flags & 0x1 as libc::c_int != 0 && (*gl).cellsize <= (*gd).sx {
        wrapped = 1 as libc::c_int as u_int
    }
    /* If the line was wrapped, don't strip spaces (use the full length). */
    if wrapped != 0 {
        xx = (*gl).cellsize
    } else {
        xx = window_copy_find_length(wme, sy)
    }
    if ex > xx {
        ex = xx
    }
    if sx > xx {
        sx = xx
    }
    if sx < ex {
        i = sx;
        while i < ex {
            grid_get_cell(gd, i, sy, &mut gc);
            if !(gc.flags as libc::c_int & 0x4 as libc::c_int != 0) {
                utf8_copy(&mut ud, &mut gc.data);
                if ud.size as libc::c_int == 1 as libc::c_int
                    && gc.attr as libc::c_int & 0x80 as libc::c_int != 0
                {
                    s = tty_acs_get(0 as *mut tty, ud.data[0 as libc::c_int as usize]);
                    if !s.is_null()
                        && strlen(s) <= ::std::mem::size_of::<[u_char; 21]>() as libc::c_ulong
                    {
                        ud.size = strlen(s) as u_char;
                        memcpy(
                            ud.data.as_mut_ptr() as *mut libc::c_void,
                            s as *const libc::c_void,
                            ud.size as libc::c_ulong,
                        );
                    }
                }
                *buf = xrealloc(
                    *buf as *mut libc::c_void,
                    (*off).wrapping_add(ud.size as libc::c_ulong),
                ) as *mut libc::c_char;
                memcpy(
                    (*buf).offset(*off as isize) as *mut libc::c_void,
                    ud.data.as_mut_ptr() as *const libc::c_void,
                    ud.size as libc::c_ulong,
                );
                *off = (*off as libc::c_ulong).wrapping_add(ud.size as libc::c_ulong) as size_t
                    as size_t
            }
            i = i.wrapping_add(1)
        }
    }
    /* Only add a newline if the line wasn't wrapped. */
    if wrapped == 0 || ex != xx {
        *buf = xrealloc(
            *buf as *mut libc::c_void,
            (*off).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        let fresh4 = *off;
        *off = (*off).wrapping_add(1);
        *(*buf).offset(fresh4 as isize) = '\n' as i32 as libc::c_char
    };
}
unsafe extern "C" fn window_copy_clear_selection(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    screen_clear_selection(&mut (*data).screen);
    (*data).cursordrag = CURSORDRAG_NONE;
    (*data).lineflag = LINE_SEL_NONE;
    (*data).selflag = SEL_CHAR;
    py = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    px = window_copy_find_length(wme, py);
    if (*data).cx > px {
        window_copy_update_cursor(wme, px, (*data).cy);
    };
}
unsafe extern "C" fn window_copy_in_set(
    mut wme: *mut window_mode_entry,
    mut px: u_int,
    mut py: u_int,
    mut set: *const libc::c_char,
) -> libc::c_int {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
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
    grid_get_cell((*(*data).backing).grid, px, py, &mut gc);
    if gc.flags as libc::c_int & 0x4 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    return utf8_cstrhas(set, &mut gc.data);
}
unsafe extern "C" fn window_copy_find_length(
    mut wme: *mut window_mode_entry,
    mut py: u_int,
) -> u_int {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    return grid_line_length((*(*data).backing).grid, py);
}
unsafe extern "C" fn window_copy_cursor_start_of_line(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut gd: *mut grid = (*back_s).grid;
    let mut py: u_int = 0;
    if (*data).cx == 0 as libc::c_int as libc::c_uint
        && (*data).lineflag as libc::c_uint == LINE_SEL_NONE as libc::c_int as libc::c_uint
    {
        py = (*(*back_s).grid)
            .hsize
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        while py > 0 as libc::c_int as libc::c_uint
            && (*grid_get_line(gd, py.wrapping_sub(1 as libc::c_int as libc::c_uint))).flags
                & 0x1 as libc::c_int
                != 0
        {
            window_copy_cursor_up(wme, 0 as libc::c_int);
            py = (*(*back_s).grid)
                .hsize
                .wrapping_add((*data).cy)
                .wrapping_sub((*data).oy)
        }
    }
    window_copy_update_cursor(wme, 0 as libc::c_int as u_int, (*data).cy);
    if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
        window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
    };
}
unsafe extern "C" fn window_copy_cursor_back_to_indentation(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
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
    px = 0 as libc::c_int as u_int;
    py = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    xx = window_copy_find_length(wme, py);
    while px < xx {
        grid_get_cell((*(*data).backing).grid, px, py, &mut gc);
        if gc.data.size as libc::c_int != 1 as libc::c_int
            || *gc.data.data.as_mut_ptr() as libc::c_int != ' ' as i32
        {
            break;
        }
        px = px.wrapping_add(1)
    }
    window_copy_update_cursor(wme, px, (*data).cy);
    if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
        window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
    };
}
unsafe extern "C" fn window_copy_cursor_end_of_line(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut gd: *mut grid = (*back_s).grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    py = (*(*back_s).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    px = window_copy_find_length(wme, py);
    if (*data).cx == px
        && (*data).lineflag as libc::c_uint == LINE_SEL_NONE as libc::c_int as libc::c_uint
    {
        if !(*data).screen.sel.is_null() && (*data).rectflag != 0 {
            px = (*(*back_s).grid).sx
        }
        gl = grid_get_line(gd, py);
        if (*gl).flags & 0x1 as libc::c_int != 0 {
            while py < (*gd).sy.wrapping_add((*gd).hsize) {
                gl = grid_get_line(gd, py);
                if !(*gl).flags & 0x1 as libc::c_int != 0 {
                    break;
                }
                window_copy_cursor_down(wme, 0 as libc::c_int);
                py = (*(*back_s).grid)
                    .hsize
                    .wrapping_add((*data).cy)
                    .wrapping_sub((*data).oy)
            }
            px = window_copy_find_length(wme, py)
        }
    }
    window_copy_update_cursor(wme, px, (*data).cy);
    if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
        window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
    };
}
unsafe extern "C" fn window_copy_other_end(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut selx: u_int = 0;
    let mut sely: u_int = 0;
    let mut cy: u_int = 0;
    let mut yy: u_int = 0;
    let mut hsize: u_int = 0;
    if (*s).sel.is_null()
        && (*data).lineflag as libc::c_uint == LINE_SEL_NONE as libc::c_int as libc::c_uint
    {
        return;
    }
    if (*data).lineflag as libc::c_uint == LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint {
        (*data).lineflag = LINE_SEL_RIGHT_LEFT
    } else if (*data).lineflag as libc::c_uint == LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint
    {
        (*data).lineflag = LINE_SEL_LEFT_RIGHT
    }
    match (*data).cursordrag as libc::c_uint {
        0 | 2 => (*data).cursordrag = CURSORDRAG_ENDSEL,
        1 => (*data).cursordrag = CURSORDRAG_SEL,
        _ => {}
    }
    selx = (*data).endselx;
    sely = (*data).endsely;
    if (*data).cursordrag as libc::c_uint == CURSORDRAG_SEL as libc::c_int as libc::c_uint {
        selx = (*data).selx;
        sely = (*data).sely
    }
    cy = (*data).cy;
    yy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).cx = selx;
    hsize = (*(*(*data).backing).grid).hsize;
    if sely < hsize.wrapping_sub((*data).oy) {
        /* above */
        (*data).oy = hsize.wrapping_sub(sely);
        (*data).cy = 0 as libc::c_int as u_int
    } else if sely > hsize.wrapping_sub((*data).oy).wrapping_add((*(*s).grid).sy) {
        /* below */
        (*data).oy = hsize
            .wrapping_sub(sely)
            .wrapping_add((*(*s).grid).sy)
            .wrapping_sub(1 as libc::c_int as libc::c_uint);
        (*data).cy = (*(*s).grid)
            .sy
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else {
        (*data).cy = cy.wrapping_add(sely).wrapping_sub(yy)
    }
    window_copy_update_selection(wme, 1 as libc::c_int, 1 as libc::c_int);
    window_copy_redraw_screen(wme);
}
unsafe extern "C" fn window_copy_cursor_left(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut py: u_int = 0;
    let mut cx: u_int = 0;
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
    py = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    cx = (*data).cx;
    while cx > 0 as libc::c_int as libc::c_uint {
        grid_get_cell((*(*data).backing).grid, cx, py, &mut gc);
        if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
            break;
        }
        cx = cx.wrapping_sub(1)
    }
    if cx == 0 as libc::c_int as libc::c_uint && py > 0 as libc::c_int as libc::c_uint {
        window_copy_cursor_up(wme, 0 as libc::c_int);
        window_copy_cursor_end_of_line(wme);
    } else if cx > 0 as libc::c_int as libc::c_uint {
        window_copy_update_cursor(
            wme,
            cx.wrapping_sub(1 as libc::c_int as libc::c_uint),
            (*data).cy,
        );
        if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
            window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
        }
    };
}
unsafe extern "C" fn window_copy_cursor_right(
    mut wme: *mut window_mode_entry,
    mut all: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut yy: u_int = 0;
    let mut cx: u_int = 0;
    let mut cy: u_int = 0;
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
    py = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    yy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*(*(*data).backing).grid).sy)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    if all != 0 || !(*data).screen.sel.is_null() && (*data).rectflag != 0 {
        px = (*(*data).screen.grid).sx
    } else {
        px = window_copy_find_length(wme, py)
    }
    if (*data).cx >= px && py < yy {
        window_copy_cursor_start_of_line(wme);
        window_copy_cursor_down(wme, 0 as libc::c_int);
    } else if (*data).cx < px {
        cx = (*data).cx.wrapping_add(1 as libc::c_int as libc::c_uint);
        cy = (*(*(*data).backing).grid)
            .hsize
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        while cx < px {
            grid_get_cell((*(*data).backing).grid, cx, cy, &mut gc);
            if !(gc.flags as libc::c_int) & 0x4 as libc::c_int != 0 {
                break;
            }
            cx = cx.wrapping_add(1)
        }
        window_copy_update_cursor(wme, cx, (*data).cy);
        if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
            window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
        }
    };
}
unsafe extern "C" fn window_copy_cursor_up(
    mut wme: *mut window_mode_entry,
    mut scroll_only: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    ox = window_copy_find_length(wme, oy);
    if (*data).cx != ox {
        (*data).lastcx = (*data).cx;
        (*data).lastsx = ox
    }
    if (*data).lineflag as libc::c_uint == LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint
        && oy == (*data).sely
    {
        window_copy_other_end(wme);
    }
    if scroll_only != 0 || (*data).cy == 0 as libc::c_int as libc::c_uint {
        (*data).cx = (*data).lastcx;
        window_copy_scroll_down(wme, 1 as libc::c_int as u_int);
        if scroll_only != 0 {
            if (*data).cy
                == (*(*s).grid)
                    .sy
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
            {
                window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
            } else {
                window_copy_redraw_lines(wme, (*data).cy, 2 as libc::c_int as u_int);
            }
        }
    } else {
        window_copy_update_cursor(
            wme,
            (*data).lastcx,
            (*data).cy.wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
            if (*data).cy
                == (*(*s).grid)
                    .sy
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
            {
                window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
            } else {
                window_copy_redraw_lines(wme, (*data).cy, 2 as libc::c_int as u_int);
            }
        }
    }
    if (*data).screen.sel.is_null() || (*data).rectflag == 0 {
        py = (*(*(*data).backing).grid)
            .hsize
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        px = window_copy_find_length(wme, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px {
            window_copy_cursor_end_of_line(wme);
        }
    }
    if (*data).lineflag as libc::c_uint == LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint {
        window_copy_cursor_end_of_line(wme);
    } else if (*data).lineflag as libc::c_uint == LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint
    {
        window_copy_cursor_start_of_line(wme);
    };
}
unsafe extern "C" fn window_copy_cursor_down(
    mut wme: *mut window_mode_entry,
    mut scroll_only: libc::c_int,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    oy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    ox = window_copy_find_length(wme, oy);
    if (*data).cx != ox {
        (*data).lastcx = (*data).cx;
        (*data).lastsx = ox
    }
    if (*data).lineflag as libc::c_uint == LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint
        && oy == (*data).endsely
    {
        window_copy_other_end(wme);
    }
    if scroll_only != 0
        || (*data).cy
            == (*(*s).grid)
                .sy
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        (*data).cx = (*data).lastcx;
        window_copy_scroll_up(wme, 1 as libc::c_int as u_int);
        if scroll_only != 0 && (*data).cy > 0 as libc::c_int as libc::c_uint {
            window_copy_redraw_lines(
                wme,
                (*data).cy.wrapping_sub(1 as libc::c_int as libc::c_uint),
                2 as libc::c_int as u_int,
            );
        }
    } else {
        window_copy_update_cursor(
            wme,
            (*data).lastcx,
            (*data).cy.wrapping_add(1 as libc::c_int as libc::c_uint),
        );
        if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
            window_copy_redraw_lines(
                wme,
                (*data).cy.wrapping_sub(1 as libc::c_int as libc::c_uint),
                2 as libc::c_int as u_int,
            );
        }
    }
    if (*data).screen.sel.is_null() || (*data).rectflag == 0 {
        py = (*(*(*data).backing).grid)
            .hsize
            .wrapping_add((*data).cy)
            .wrapping_sub((*data).oy);
        px = window_copy_find_length(wme, py);
        if (*data).cx >= (*data).lastsx && (*data).cx != px || (*data).cx > px {
            window_copy_cursor_end_of_line(wme);
        }
    }
    if (*data).lineflag as libc::c_uint == LINE_SEL_LEFT_RIGHT as libc::c_int as libc::c_uint {
        window_copy_cursor_end_of_line(wme);
    } else if (*data).lineflag as libc::c_uint == LINE_SEL_RIGHT_LEFT as libc::c_int as libc::c_uint
    {
        window_copy_cursor_start_of_line(wme);
    };
}
unsafe extern "C" fn window_copy_cursor_jump(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
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
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    px = (*data).cx.wrapping_add(1 as libc::c_int as libc::c_uint);
    py = (*(*back_s).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    xx = window_copy_find_length(wme, py);
    while px < xx {
        grid_get_cell((*back_s).grid, px, py, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int == 0
            && gc.data.size as libc::c_int == 1 as libc::c_int
            && *gc.data.data.as_mut_ptr() as libc::c_int == (*data).jumpchar as libc::c_int
        {
            window_copy_update_cursor(wme, px, (*data).cy);
            if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
                window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
            }
            return;
        }
        px = px.wrapping_add(1)
    }
}
unsafe extern "C" fn window_copy_cursor_jump_back(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
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
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    px = (*data).cx;
    py = (*(*back_s).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    if px > 0 as libc::c_int as libc::c_uint {
        px = px.wrapping_sub(1)
    }
    loop {
        grid_get_cell((*back_s).grid, px, py, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int == 0
            && gc.data.size as libc::c_int == 1 as libc::c_int
            && *gc.data.data.as_mut_ptr() as libc::c_int == (*data).jumpchar as libc::c_int
        {
            window_copy_update_cursor(wme, px, (*data).cy);
            if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
                window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
            }
            return;
        }
        if px == 0 as libc::c_int as libc::c_uint {
            break;
        }
        px = px.wrapping_sub(1)
    }
}
unsafe extern "C" fn window_copy_cursor_jump_to(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
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
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    px = (*data).cx.wrapping_add(2 as libc::c_int as libc::c_uint);
    py = (*(*back_s).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    xx = window_copy_find_length(wme, py);
    while px < xx {
        grid_get_cell((*back_s).grid, px, py, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int == 0
            && gc.data.size as libc::c_int == 1 as libc::c_int
            && *gc.data.data.as_mut_ptr() as libc::c_int == (*data).jumpchar as libc::c_int
        {
            window_copy_update_cursor(
                wme,
                px.wrapping_sub(1 as libc::c_int as libc::c_uint),
                (*data).cy,
            );
            if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
                window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
            }
            return;
        }
        px = px.wrapping_add(1)
    }
}
unsafe extern "C" fn window_copy_cursor_jump_to_back(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
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
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    px = (*data).cx;
    py = (*(*back_s).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    if px > 0 as libc::c_int as libc::c_uint {
        px = px.wrapping_sub(1)
    }
    if px > 0 as libc::c_int as libc::c_uint {
        px = px.wrapping_sub(1)
    }
    loop {
        grid_get_cell((*back_s).grid, px, py, &mut gc);
        if gc.flags as libc::c_int & 0x4 as libc::c_int == 0
            && gc.data.size as libc::c_int == 1 as libc::c_int
            && *gc.data.data.as_mut_ptr() as libc::c_int == (*data).jumpchar as libc::c_int
        {
            window_copy_update_cursor(
                wme,
                px.wrapping_add(1 as libc::c_int as libc::c_uint),
                (*data).cy,
            );
            if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
                window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
            }
            return;
        }
        if px == 0 as libc::c_int as libc::c_uint {
            break;
        }
        px = px.wrapping_sub(1)
    }
}
unsafe extern "C" fn window_copy_cursor_next_word(
    mut wme: *mut window_mode_entry,
    mut separators: *const libc::c_char,
) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut back_s: *mut screen = (*data).backing;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut expected: libc::c_int = 0 as libc::c_int;
    px = (*data).cx;
    py = (*(*back_s).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    xx = window_copy_find_length(wme, py);
    yy = (*(*back_s).grid)
        .hsize
        .wrapping_add((*(*back_s).grid).sy)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    loop
    /*
     * First skip past any nonword characters and then any word characters.
     *
     * expected is initially set to 0 for the former and then 1 for the
     * latter.
     */
    {
        while px > xx || window_copy_in_set(wme, px, py, separators) == expected {
            /* Move down if we're past the end of the line. */
            if px > xx {
                if py == yy {
                    return;
                }
                window_copy_cursor_down(wme, 0 as libc::c_int);
                px = 0 as libc::c_int as u_int;
                py = (*(*back_s).grid)
                    .hsize
                    .wrapping_add((*data).cy)
                    .wrapping_sub((*data).oy);
                xx = window_copy_find_length(wme, py)
            } else {
                px = px.wrapping_add(1)
            }
        }
        expected = (expected == 0) as libc::c_int;
        if !(expected == 1 as libc::c_int) {
            break;
        }
    }
    window_copy_update_cursor(wme, px, (*data).cy);
    if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
        window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
    };
}
unsafe extern "C" fn window_copy_cursor_next_word_end_pos(
    mut wme: *mut window_mode_entry,
    mut separators: *const libc::c_char,
    mut ppx: *mut u_int,
    mut ppy: *mut u_int,
) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut oo: *mut crate::options::options = (*(*wp).window).options;
    let mut back_s: *mut screen = (*data).backing;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut keys: libc::c_int = 0;
    let mut expected: libc::c_int = 1 as libc::c_int;
    px = (*data).cx;
    py = (*(*back_s).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    xx = window_copy_find_length(wme, py);
    yy = (*(*back_s).grid)
        .hsize
        .wrapping_add((*(*back_s).grid).sy)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    keys =
        options_get_number(oo, b"mode-keys\x00" as *const u8 as *const libc::c_char) as libc::c_int;
    if keys == 1 as libc::c_int && window_copy_in_set(wme, px, py, separators) == 0 {
        px = px.wrapping_add(1)
    }
    loop
    /*
     * First skip past any word characters, then any non-word characters.
     *
     * expected is initially set to 1 for the former and then 0 for the
     * latter.
     */
    {
        while px > xx || window_copy_in_set(wme, px, py, separators) == expected {
            /* Move down if we're past the end of the line. */
            if px > xx {
                if py == yy {
                    return;
                }
                py = py.wrapping_add(1);
                px = 0 as libc::c_int as u_int;
                xx = window_copy_find_length(wme, py)
            } else {
                px = px.wrapping_add(1)
            }
        }
        expected = (expected == 0) as libc::c_int;
        if !(expected == 0 as libc::c_int) {
            break;
        }
    }
    if keys == 1 as libc::c_int && px != 0 as libc::c_int as libc::c_uint {
        px = px.wrapping_sub(1)
    }
    *ppx = px;
    *ppy = py;
}
unsafe extern "C" fn window_copy_cursor_next_word_end(
    mut wme: *mut window_mode_entry,
    mut separators: *const libc::c_char,
    mut no_reset: libc::c_int,
) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut oo: *mut crate::options::options = (*(*wp).window).options;
    let mut back_s: *mut screen = (*data).backing;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    let mut keys: libc::c_int = 0;
    let mut expected: libc::c_int = 1 as libc::c_int;
    px = (*data).cx;
    py = (*(*back_s).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    xx = window_copy_find_length(wme, py);
    yy = (*(*back_s).grid)
        .hsize
        .wrapping_add((*(*back_s).grid).sy)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    keys =
        options_get_number(oo, b"mode-keys\x00" as *const u8 as *const libc::c_char) as libc::c_int;
    if keys == 1 as libc::c_int && window_copy_in_set(wme, px, py, separators) == 0 {
        px = px.wrapping_add(1)
    }
    loop
    /*
     * First skip past any word characters, then any nonword characters.
     *
     * expected is initially set to 1 for the former and then 0 for the
     * latter.
     */
    {
        while px > xx || window_copy_in_set(wme, px, py, separators) == expected {
            /* Move down if we're past the end of the line. */
            if px > xx {
                if py == yy {
                    return;
                }
                window_copy_cursor_down(wme, 0 as libc::c_int);
                px = 0 as libc::c_int as u_int;
                py = (*(*back_s).grid)
                    .hsize
                    .wrapping_add((*data).cy)
                    .wrapping_sub((*data).oy);
                xx = window_copy_find_length(wme, py)
            } else {
                px = px.wrapping_add(1)
            }
        }
        expected = (expected == 0) as libc::c_int;
        if !(expected == 0 as libc::c_int) {
            break;
        }
    }
    if keys == 1 as libc::c_int && px != 0 as libc::c_int as libc::c_uint {
        px = px.wrapping_sub(1)
    }
    window_copy_update_cursor(wme, px, (*data).cy);
    if window_copy_update_selection(wme, 1 as libc::c_int, no_reset) != 0 {
        window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
    };
}
/* Compute the previous place where a word begins. */
unsafe extern "C" fn window_copy_cursor_previous_word_pos(
    mut wme: *mut window_mode_entry,
    mut separators: *const libc::c_char,
    mut already: libc::c_int,
    mut ppx: *mut u_int,
    mut ppy: *mut u_int,
) {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    px = (*data).cx;
    py = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    /* Move back to the previous word character. */
    if already != 0 || window_copy_in_set(wme, px, py, separators) != 0 {
        current_block = 7502529970979898288;
    } else {
        current_block = 8236137900636309791;
    }
    loop {
        match current_block {
            8236137900636309791 => {
                /* Move back to the beginning of this word. */
                while px > 0 as libc::c_int as libc::c_uint
                    && window_copy_in_set(
                        wme,
                        px.wrapping_sub(1 as libc::c_int as libc::c_uint),
                        py,
                        separators,
                    ) == 0
                {
                    px = px.wrapping_sub(1)
                }
                break;
            }
            _ => {
                if px > 0 as libc::c_int as libc::c_uint {
                    px = px.wrapping_sub(1);
                    if window_copy_in_set(wme, px, py, separators) == 0 {
                        current_block = 8236137900636309791;
                    } else {
                        current_block = 7502529970979898288;
                    }
                } else {
                    if py == 0 as libc::c_int as libc::c_uint
                        || (*data).cy == 0 as libc::c_int as libc::c_uint
                            && ((*(*(*data).backing).grid).hsize
                                == 0 as libc::c_int as libc::c_uint
                                || (*data).oy
                                    >= (*(*(*data).backing).grid)
                                        .hsize
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                    {
                        break;
                    }
                    py = py.wrapping_sub(1);
                    px = window_copy_find_length(wme, py);
                    /* Stop if separator at EOL. */
                    if px > 0 as libc::c_int as libc::c_uint
                        && window_copy_in_set(
                            wme,
                            px.wrapping_sub(1 as libc::c_int as libc::c_uint),
                            py,
                            separators,
                        ) != 0
                    {
                        current_block = 8236137900636309791;
                    } else {
                        current_block = 7502529970979898288;
                    }
                }
            }
        }
    }
    *ppx = px;
    *ppy = py;
}
/* Move to the previous place where a word begins. */
unsafe extern "C" fn window_copy_cursor_previous_word(
    mut wme: *mut window_mode_entry,
    mut separators: *const libc::c_char,
    mut already: libc::c_int,
) {
    let mut current_block: u64;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    px = (*data).cx;
    py = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    /* Move back to the previous word character. */
    if already != 0 || window_copy_in_set(wme, px, py, separators) != 0 {
        current_block = 7502529970979898288;
    } else {
        current_block = 17407779659766490442;
    }
    loop {
        match current_block {
            17407779659766490442 => {
                /* Move back to the beginning of this word. */
                while px > 0 as libc::c_int as libc::c_uint
                    && window_copy_in_set(
                        wme,
                        px.wrapping_sub(1 as libc::c_int as libc::c_uint),
                        py,
                        separators,
                    ) == 0
                {
                    px = px.wrapping_sub(1)
                }
                break;
            }
            _ => {
                if px > 0 as libc::c_int as libc::c_uint {
                    px = px.wrapping_sub(1);
                    if window_copy_in_set(wme, px, py, separators) == 0 {
                        current_block = 17407779659766490442;
                    } else {
                        current_block = 7502529970979898288;
                    }
                } else {
                    if (*data).cy == 0 as libc::c_int as libc::c_uint
                        && ((*(*(*data).backing).grid).hsize == 0 as libc::c_int as libc::c_uint
                            || (*data).oy
                                >= (*(*(*data).backing).grid)
                                    .hsize
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                    {
                        break;
                    }
                    window_copy_cursor_up(wme, 0 as libc::c_int);
                    py = (*(*(*data).backing).grid)
                        .hsize
                        .wrapping_add((*data).cy)
                        .wrapping_sub((*data).oy);
                    px = window_copy_find_length(wme, py);
                    /* Stop if separator at EOL. */
                    if px > 0 as libc::c_int as libc::c_uint
                        && window_copy_in_set(
                            wme,
                            px.wrapping_sub(1 as libc::c_int as libc::c_uint),
                            py,
                            separators,
                        ) != 0
                    {
                        current_block = 17407779659766490442;
                    } else {
                        current_block = 7502529970979898288;
                    }
                }
            }
        }
    }
    window_copy_update_cursor(wme, px, (*data).cy);
    if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
        window_copy_redraw_lines(wme, (*data).cy, 1 as libc::c_int as u_int);
    };
}
unsafe extern "C" fn window_copy_scroll_up(mut wme: *mut window_mode_entry, mut ny: u_int) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    if (*data).oy < ny {
        ny = (*data).oy
    }
    if ny == 0 as libc::c_int as libc::c_uint {
        return;
    }
    (*data).oy = ((*data).oy as libc::c_uint).wrapping_sub(ny) as u_int as u_int;
    if !(*data).searchmark.is_null() && (*data).timeout == 0 {
        window_copy_search_marks(wme, 0 as *mut screen, (*data).searchregex, 1 as libc::c_int);
    }
    window_copy_update_selection(wme, 0 as libc::c_int, 0 as libc::c_int);
    screen_write_start_pane(&mut ctx, wp, 0 as *mut screen);
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    screen_write_deleteline(&mut ctx, ny, 8 as libc::c_int as u_int);
    window_copy_write_lines(wme, &mut ctx, (*(*s).grid).sy.wrapping_sub(ny), ny);
    window_copy_write_line(wme, &mut ctx, 0 as libc::c_int as u_int);
    if (*(*s).grid).sy > 1 as libc::c_int as libc::c_uint {
        window_copy_write_line(wme, &mut ctx, 1 as libc::c_int as u_int);
    }
    if (*(*s).grid).sy > 3 as libc::c_int as libc::c_uint {
        window_copy_write_line(
            wme,
            &mut ctx,
            (*(*s).grid)
                .sy
                .wrapping_sub(2 as libc::c_int as libc::c_uint),
        );
    }
    if !(*s).sel.is_null() && (*(*s).grid).sy > ny {
        window_copy_write_line(
            wme,
            &mut ctx,
            (*(*s).grid)
                .sy
                .wrapping_sub(ny)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    screen_write_cursormove(
        &mut ctx,
        (*data).cx as libc::c_int,
        (*data).cy as libc::c_int,
        0 as libc::c_int,
    );
    screen_write_stop(&mut ctx);
}
unsafe extern "C" fn window_copy_scroll_down(mut wme: *mut window_mode_entry, mut ny: u_int) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut s: *mut screen = &mut (*data).screen;
    let mut ctx: screen_write_ctx = screen_write_ctx {
        wp: 0 as *mut window_pane,
        s: 0 as *mut screen,
        flags: 0,
        init_ctx_cb: None,
        arg: 0 as *mut libc::c_void,
        item: 0 as *mut crate::screen_write::screen_write_collect_item,
        scrolled: 0,
        bg: 0,
        cells: 0,
        written: 0,
        skipped: 0,
    };
    if ny > (*(*(*data).backing).grid).hsize {
        return;
    }
    if (*data).oy > (*(*(*data).backing).grid).hsize.wrapping_sub(ny) {
        ny = (*(*(*data).backing).grid).hsize.wrapping_sub((*data).oy)
    }
    if ny == 0 as libc::c_int as libc::c_uint {
        return;
    }
    (*data).oy = ((*data).oy as libc::c_uint).wrapping_add(ny) as u_int as u_int;
    if !(*data).searchmark.is_null() && (*data).timeout == 0 {
        window_copy_search_marks(wme, 0 as *mut screen, (*data).searchregex, 1 as libc::c_int);
    }
    window_copy_update_selection(wme, 0 as libc::c_int, 0 as libc::c_int);
    screen_write_start_pane(&mut ctx, wp, 0 as *mut screen);
    screen_write_cursormove(
        &mut ctx,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    screen_write_insertline(&mut ctx, ny, 8 as libc::c_int as u_int);
    window_copy_write_lines(wme, &mut ctx, 0 as libc::c_int as u_int, ny);
    if !(*s).sel.is_null() && (*(*s).grid).sy > ny {
        window_copy_write_line(wme, &mut ctx, ny);
    } else if ny == 1 as libc::c_int as libc::c_uint {
        /* nuke position */
        window_copy_write_line(wme, &mut ctx, 1 as libc::c_int as u_int);
    }
    screen_write_cursormove(
        &mut ctx,
        (*data).cx as libc::c_int,
        (*data).cy as libc::c_int,
        0 as libc::c_int,
    );
    screen_write_stop(&mut ctx);
}
unsafe extern "C" fn window_copy_rectangle_toggle(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    (*data).rectflag = ((*data).rectflag == 0) as libc::c_int;
    py = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    px = window_copy_find_length(wme, py);
    if (*data).cx > px {
        window_copy_update_cursor(wme, px, (*data).cy);
    }
    window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int);
    window_copy_redraw_screen(wme);
}
unsafe extern "C" fn window_copy_move_mouse(mut m: *mut mouse_event) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    wp = cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp.is_null() {
        return;
    }
    wme = (*wp).modes.tqh_first;
    if wme.is_null() {
        return;
    }
    if (*wme).mode != &window_copy_mode as *const window_mode
        && (*wme).mode != &window_view_mode as *const window_mode
    {
        return;
    }
    if cmd_mouse_at(wp, m, &mut x, &mut y, 0 as libc::c_int) != 0 as libc::c_int {
        return;
    }
    window_copy_update_cursor(wme, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn window_copy_start_drag(mut c: *mut client, mut m: *mut mouse_event) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    let mut data: *mut window_copy_mode_data = 0 as *mut window_copy_mode_data;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut yg: u_int = 0;
    if c.is_null() {
        return;
    }
    wp = cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp.is_null() {
        return;
    }
    wme = (*wp).modes.tqh_first;
    if wme.is_null() {
        return;
    }
    if (*wme).mode != &window_copy_mode as *const window_mode
        && (*wme).mode != &window_view_mode as *const window_mode
    {
        return;
    }
    if cmd_mouse_at(wp, m, &mut x, &mut y, 1 as libc::c_int) != 0 as libc::c_int {
        return;
    }
    (*c).tty.mouse_drag_update = Some(
        window_copy_drag_update as unsafe extern "C" fn(_: *mut client, _: *mut mouse_event) -> (),
    );
    (*c).tty.mouse_drag_release = Some(
        window_copy_drag_release as unsafe extern "C" fn(_: *mut client, _: *mut mouse_event) -> (),
    );
    data = (*wme).data as *mut window_copy_mode_data;
    yg = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add(y)
        .wrapping_sub((*data).oy);
    if x < (*data).selrx || x > (*data).endselrx || yg != (*data).selry {
        (*data).selflag = SEL_CHAR
    }
    match (*data).selflag as libc::c_uint {
        1 => {
            if !(*data).ws.is_null() {
                window_copy_update_cursor(wme, x, y);
                window_copy_cursor_previous_word_pos(
                    wme,
                    (*data).ws,
                    0 as libc::c_int,
                    &mut x,
                    &mut y,
                );
                y = (y as libc::c_uint)
                    .wrapping_sub((*(*(*data).backing).grid).hsize.wrapping_sub((*data).oy))
                    as u_int as u_int
            }
            window_copy_update_cursor(wme, x, y);
        }
        2 => {
            window_copy_update_cursor(wme, 0 as libc::c_int as u_int, y);
        }
        0 => {
            window_copy_update_cursor(wme, x, y);
            window_copy_start_selection(wme);
        }
        _ => {}
    }
    window_copy_redraw_screen(wme);
    window_copy_drag_update(c, m);
}
unsafe extern "C" fn window_copy_drag_update(mut c: *mut client, mut m: *mut mouse_event) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    let mut data: *mut window_copy_mode_data = 0 as *mut window_copy_mode_data;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut old_cx: u_int = 0;
    let mut old_cy: u_int = 0;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 50000 as libc::c_int as __suseconds_t,
        };
        init
    };
    if c.is_null() {
        return;
    }
    wp = cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp.is_null() {
        return;
    }
    wme = (*wp).modes.tqh_first;
    if wme.is_null() {
        return;
    }
    if (*wme).mode != &window_copy_mode as *const window_mode
        && (*wme).mode != &window_view_mode as *const window_mode
    {
        return;
    }
    data = (*wme).data as *mut window_copy_mode_data;
    event_del(&mut (*data).dragtimer);
    if cmd_mouse_at(wp, m, &mut x, &mut y, 0 as libc::c_int) != 0 as libc::c_int {
        return;
    }
    old_cx = (*data).cx;
    old_cy = (*data).cy;
    window_copy_update_cursor(wme, x, y);
    if window_copy_update_selection(wme, 1 as libc::c_int, 0 as libc::c_int) != 0 {
        window_copy_redraw_selection(wme, old_cy);
    }
    if old_cy != (*data).cy || old_cx == (*data).cx {
        if y == 0 as libc::c_int as libc::c_uint {
            event_add(&mut (*data).dragtimer, &mut tv);
            window_copy_cursor_up(wme, 1 as libc::c_int);
        } else if y
            == (*(*data).screen.grid)
                .sy
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            event_add(&mut (*data).dragtimer, &mut tv);
            window_copy_cursor_down(wme, 1 as libc::c_int);
        }
    };
}
unsafe extern "C" fn window_copy_drag_release(mut c: *mut client, mut m: *mut mouse_event) {
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    let mut data: *mut window_copy_mode_data = 0 as *mut window_copy_mode_data;
    if c.is_null() {
        return;
    }
    wp = cmd_mouse_pane(m, 0 as *mut *mut session, 0 as *mut *mut winlink);
    if wp.is_null() {
        return;
    }
    wme = (*wp).modes.tqh_first;
    if wme.is_null() {
        return;
    }
    if (*wme).mode != &window_copy_mode as *const window_mode
        && (*wme).mode != &window_view_mode as *const window_mode
    {
        return;
    }
    data = (*wme).data as *mut window_copy_mode_data;
    event_del(&mut (*data).dragtimer);
}
unsafe extern "C" fn window_copy_jump_to_mark(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_copy_mode_data = (*wme).data as *mut window_copy_mode_data;
    let mut tmx: u_int = 0;
    let mut tmy: u_int = 0;
    tmx = (*data).cx;
    tmy = (*(*(*data).backing).grid)
        .hsize
        .wrapping_add((*data).cy)
        .wrapping_sub((*data).oy);
    (*data).cx = (*data).mx;
    if (*data).my < (*(*(*data).backing).grid).hsize {
        (*data).cy = 0 as libc::c_int as u_int;
        (*data).oy = (*(*(*data).backing).grid).hsize.wrapping_sub((*data).my)
    } else {
        (*data).cy = (*data).my.wrapping_sub((*(*(*data).backing).grid).hsize);
        (*data).oy = 0 as libc::c_int as u_int
    }
    (*data).mx = tmx;
    (*data).my = tmy;
    (*data).showmark = 1 as libc::c_int;
    window_copy_update_selection(wme, 0 as libc::c_int, 0 as libc::c_int);
    window_copy_redraw_screen(wme);
}
