use crate::{
    grid::Cell as GridCell,
    utf8::{utf8_state, Utf8Data},
};
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __b64_ntop(
        _: *const libc::c_uchar,
        _: size_t,
        _: *mut libc::c_char,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn __b64_pton(_: *const libc::c_char, _: *mut libc::c_uchar, _: size_t) -> libc::c_int;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char)
        -> *mut libc::c_char;
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
    fn evbuffer_new() -> *mut evbuffer;
    #[no_mangle]
    fn evbuffer_free(buf: *mut evbuffer);
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_add(buf: *mut evbuffer, data: *const libc::c_void, datlen: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
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
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xvasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut crate::options::options;
    #[no_mangle]
    fn getversion() -> *const libc::c_char;
    #[no_mangle]
    fn paste_buffer_data(_: *mut crate::paste::paste_buffer, _: *mut size_t)
        -> *const libc::c_char;
    #[no_mangle]
    fn paste_get_top(_: *mut *const libc::c_char) -> *mut crate::paste::paste_buffer;
    #[no_mangle]
    fn paste_add(_: *const libc::c_char, _: *mut libc::c_char, _: size_t);
    #[no_mangle]
    fn notify_pane(_: *const libc::c_char, _: *mut window_pane);
    #[no_mangle]
    fn options_get_only(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> *mut crate::options::options_entry;
    #[no_mangle]
    fn options_get_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn options_set_number(
        _: *mut crate::options::options,
        _: *const libc::c_char,
        _: libc::c_longlong,
    ) -> *mut crate::options::options_entry;
    #[no_mangle]
    fn options_remove_or_default(
        _: *mut crate::options::options_entry,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn tty_default_colours(_: *mut crate::grid::Cell, _: *mut window_pane);
    #[no_mangle]
    fn alerts_queue(_: *mut window, _: libc::c_int);
    #[no_mangle]
    fn server_redraw_window_borders(_: *mut window);
    #[no_mangle]
    fn server_status_window(_: *mut window);
    #[no_mangle]
    fn screen_write_collect_add(_: *mut screen_write_ctx, _: *const crate::grid::Cell);
    #[no_mangle]
    fn screen_write_carriagereturn(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_linefeed(_: *mut screen_write_ctx, _: libc::c_int, _: u_int);
    #[no_mangle]
    fn screen_write_backspace(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_alignmenttest(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_cursormove(
        _: *mut screen_write_ctx,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_write_mode_clear(_: *mut screen_write_ctx, _: libc::c_int);
    #[no_mangle]
    fn screen_write_mode_set(_: *mut screen_write_ctx, _: libc::c_int);
    #[no_mangle]
    fn screen_write_reverseindex(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_reset(_: *mut screen_write_ctx);
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn window_pane_reset_palette(_: *mut window_pane);
    #[no_mangle]
    fn window_set_name(_: *mut window, _: *const libc::c_char);
    #[no_mangle]
    fn screen_set_title(_: *mut screen, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn screen_set_cursor_colour(_: *mut screen, _: *const libc::c_char);
    #[no_mangle]
    fn window_pane_unset_palette(_: *mut window_pane, _: u_int);
    #[no_mangle]
    fn screen_write_stop(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_setselection(_: *mut screen_write_ctx, _: *mut u_char, _: u_int);
    #[no_mangle]
    fn screen_write_start_pane(_: *mut screen_write_ctx, _: *mut window_pane, _: *mut screen);
    #[no_mangle]
    fn colour_join_rgb(_: u_char, _: u_char, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn colour_split_rgb(_: libc::c_int, _: *mut u_char, _: *mut u_char, _: *mut u_char);
    #[no_mangle]
    fn screen_set_path(_: *mut screen, _: *const libc::c_char);
    #[no_mangle]
    fn window_pane_set_palette(_: *mut window_pane, _: u_int, _: libc::c_int);
    #[no_mangle]
    fn screen_set_cursor_style(_: *mut screen, _: u_int);
    #[no_mangle]
    fn screen_write_scrolldown(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_scrollup(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_alternateon(
        _: *mut screen_write_ctx,
        _: *mut crate::grid::Cell,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_write_clearscreen(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_alternateoff(
        _: *mut screen_write_ctx,
        _: *mut crate::grid::Cell,
        _: libc::c_int,
    );
    #[no_mangle]
    fn screen_write_insertline(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_insertcharacter(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_clearline(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_clearstartofline(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_clearendofline(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_clearhistory(_: *mut screen_write_ctx);
    #[no_mangle]
    fn screen_write_clearstartofscreen(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_clearendofscreen(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_deleteline(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_scrollregion(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_deletecharacter(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_clearcharacter(_: *mut screen_write_ctx, _: u_int, _: u_int);
    #[no_mangle]
    fn screen_write_cursorup(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_cursordown(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_pop_title(_: *mut screen);
    #[no_mangle]
    fn screen_push_title(_: *mut screen);
    #[no_mangle]
    fn screen_write_cursorright(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_cursorleft(_: *mut screen_write_ctx, _: u_int);
    #[no_mangle]
    fn screen_write_rawstring(_: *mut screen_write_ctx, _: *mut u_char, _: u_int);
    #[no_mangle]
    fn screen_write_start(_: *mut screen_write_ctx, _: *mut screen);
    #[no_mangle]
    fn window_pane_update_used_data(_: *mut window_pane, _: *mut window_pane_offset, _: size_t);
    #[no_mangle]
    fn screen_write_collect_end(_: *mut screen_write_ctx);
    #[no_mangle]
    fn window_update_activity(_: *mut window);
    #[no_mangle]
    fn window_pane_get_new_data(
        _: *mut window_pane,
        _: *mut window_pane_offset,
        _: *mut size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn screen_write_start_callback(
        _: *mut screen_write_ctx,
        _: *mut screen,
        _: screen_write_init_ctx_cb,
        _: *mut libc::c_void,
    );
    #[no_mangle]
    fn utf8_copy(_: *mut Utf8Data, _: *const Utf8Data);
    #[no_mangle]
    fn utf8_isvalid(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn utf8_set(_: *mut Utf8Data, _: u_char);
    #[no_mangle]
    fn utf8_open(_: *mut Utf8Data, _: u_char) -> crate::utf8::Utf8State;
    #[no_mangle]
    fn utf8_append(_: *mut Utf8Data, _: u_char) -> crate::utf8::Utf8State;
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
pub type va_list = __builtin_va_list;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;

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
    pub exit_type: C2RustUnnamed_31,
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
    pub prompt_mode: C2RustUnnamed_28,
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
    pub extddata: *mut crate::grid::ExtdEntry,
    pub flags: libc::c_int,
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
    pub ictx: *mut input_ctx,
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
/* Input parser context. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_ctx {
    pub wp: *mut window_pane,
    pub event: *mut bufferevent,
    pub ctx: screen_write_ctx,
    pub cell: input_cell,
    pub old_cell: input_cell,
    pub old_cx: u_int,
    pub old_cy: u_int,
    pub old_mode: libc::c_int,
    pub interm_buf: [u_char; 4],
    pub interm_len: size_t,
    pub param_buf: [u_char; 64],
    pub param_len: size_t,
    pub input_buf: *mut u_char,
    pub input_len: size_t,
    pub input_space: size_t,
    pub input_end: C2RustUnnamed_27,
    pub param_list: [input_param; 24],
    pub param_list_len: u_int,
    pub utf8data: Utf8Data,
    pub utf8started: libc::c_int,
    pub ch: libc::c_int,
    pub last: libc::c_int,
    pub flags: libc::c_int,
    pub state: *const input_state,
    pub timer: event,
    pub since_ground: *mut evbuffer,
}
/* Input state. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_state {
    pub name: *const libc::c_char,
    pub enter: Option<unsafe extern "C" fn(_: *mut input_ctx) -> ()>,
    pub exit: Option<unsafe extern "C" fn(_: *mut input_ctx) -> ()>,
    pub transitions: *const input_transition,
}
/* Input transition. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_transition {
    pub first: libc::c_int,
    pub last: libc::c_int,
    pub handler: Option<unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int>,
    pub state: *const input_state,
}
/* 1 if ACS */
/* Input parser argument. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_param {
    pub type_0: C2RustUnnamed_26,
    pub c2rust_unnamed: C2RustUnnamed_25,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_25 {
    pub num: libc::c_int,
    pub str_0: *mut libc::c_char,
}
pub type C2RustUnnamed_26 = libc::c_uint;
pub const INPUT_STRING: C2RustUnnamed_26 = 2;
pub const INPUT_NUMBER: C2RustUnnamed_26 = 1;
pub const INPUT_MISSING: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const INPUT_END_BEL: C2RustUnnamed_27 = 1;
pub const INPUT_END_ST: C2RustUnnamed_27 = 0;
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
/*
 * Based on the description by Paul Williams at:
 *
 * https://vt100.net/emu/dec_ansi_parser
 *
 * With the following changes:
 *
 * - 7-bit only.
 *
 * - Support for UTF-8.
 *
 * - OSC (but not APC) may be terminated by \007 as well as ST.
 *
 * - A state for APC similar to OSC. Some terminals appear to use this to set
 *   the title.
 *
 * - A state for the screen \033k...\033\\ sequence to rename a window. This is
 *   pretty stupid but not supporting it is more trouble than it is worth.
 *
 * - Special handling for ESC inside a DCS to allow arbitrary byte sequences to
 *   be passed to the underlying terminals.
 */
/* Input parser cell. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_cell {
    pub cell: crate::grid::Cell,
    pub set: libc::c_int,
    pub g0set: libc::c_int,
    pub g1set: libc::c_int,
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
pub type C2RustUnnamed_28 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_28 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_28 = 0;
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
    pub entry: C2RustUnnamed_29,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_29 {
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
    pub entry: C2RustUnnamed_30,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_30 {
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
pub type C2RustUnnamed_31 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_31 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_31 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_31 = 0;

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
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
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
    pub entry: C2RustUnnamed_33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
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
pub const INPUT_ESC_ST: input_esc_type = 14;
pub const INPUT_ESC_SCSG1_OFF: input_esc_type = 12;
pub const INPUT_ESC_SCSG1_ON: input_esc_type = 13;
pub const INPUT_ESC_SCSG0_OFF: input_esc_type = 10;
pub const INPUT_ESC_SCSG0_ON: input_esc_type = 11;
pub const INPUT_ESC_DECALN: input_esc_type = 0;
pub const INPUT_ESC_DECRC: input_esc_type = 3;
pub const INPUT_ESC_DECSC: input_esc_type = 4;
pub const INPUT_ESC_DECKPNM: input_esc_type = 2;
pub const INPUT_ESC_DECKPAM: input_esc_type = 1;
pub const INPUT_ESC_RI: input_esc_type = 8;
pub const INPUT_ESC_HTS: input_esc_type = 5;
pub const INPUT_ESC_NEL: input_esc_type = 7;
pub const INPUT_ESC_IND: input_esc_type = 6;
pub const INPUT_ESC_RIS: input_esc_type = 9;
/* Command table entry. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_table_entry {
    pub ch: libc::c_int,
    pub interm: *const libc::c_char,
    pub type_0: libc::c_int,
}
pub const INPUT_CSI_XDA: input_csi_type = 36;
pub const INPUT_CSI_DECSCUSR: input_csi_type = 11;
pub const INPUT_CSI_VPA: input_csi_type = 34;
pub const INPUT_CSI_TBC: input_csi_type = 33;
pub const INPUT_CSI_SD: input_csi_type = 28;
pub const INPUT_CSI_SU: input_csi_type = 32;
pub const INPUT_CSI_SM_PRIVATE: input_csi_type = 31;
pub const INPUT_CSI_SM: input_csi_type = 30;
pub const INPUT_CSI_SGR: input_csi_type = 29;
pub const INPUT_CSI_SCP: input_csi_type = 27;
pub const INPUT_CSI_RM_PRIVATE: input_csi_type = 26;
pub const INPUT_CSI_RM: input_csi_type = 25;
pub const INPUT_CSI_RCP: input_csi_type = 23;
pub const INPUT_CSI_REP: input_csi_type = 24;
pub const INPUT_CSI_IL: input_csi_type = 20;
pub const INPUT_CSI_ICH: input_csi_type = 19;
pub const INPUT_CSI_HPA: input_csi_type = 18;
pub const INPUT_CSI_EL: input_csi_type = 17;
pub const INPUT_CSI_ED: input_csi_type = 16;
pub const INPUT_CSI_DSR: input_csi_type = 14;
pub const INPUT_CSI_DL: input_csi_type = 13;
pub const INPUT_CSI_DECSTBM: input_csi_type = 12;
pub const INPUT_CSI_DCH: input_csi_type = 10;
pub const INPUT_CSI_ECH: input_csi_type = 15;
pub const INPUT_CSI_DA_TWO: input_csi_type = 9;
pub const INPUT_CSI_DA: input_csi_type = 8;
pub const INPUT_CSI_CPL: input_csi_type = 2;
pub const INPUT_CSI_CNL: input_csi_type = 1;
pub const INPUT_CSI_CUU: input_csi_type = 7;
pub const INPUT_CSI_WINOPS: input_csi_type = 35;
pub const INPUT_CSI_MODOFF: input_csi_type = 21;
pub const INPUT_CSI_MODSET: input_csi_type = 22;
pub const INPUT_CSI_CUP: input_csi_type = 6;
pub const INPUT_CSI_CUF: input_csi_type = 5;
pub const INPUT_CSI_CUD: input_csi_type = 4;
pub const INPUT_CSI_CUB: input_csi_type = 3;
pub const INPUT_CSI_CBT: input_csi_type = 0;
/* Escape commands. */
pub type input_esc_type = libc::c_uint;
/* Control (CSI) commands. */
pub type input_csi_type = libc::c_uint;
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0u64;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2u64);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *mut libc::c_void;
        __comparison = Some(__compar.expect("non-null function pointer"))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0i32 {
            __u = __idx
        } else if __comparison > 0i32 {
            __l = __idx.wrapping_add(1u64)
        } else {
            return __p as *mut libc::c_void;
        }
    }
    return 0 as *mut libc::c_void;
}
/* Escape command table. */
static mut input_esc_table: [input_table_entry; 15] = [
    {
        let mut init = input_table_entry {
            ch: '0' as i32,
            interm: b"(\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_SCSG0_ON as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '0' as i32,
            interm: b")\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_SCSG1_ON as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '7' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_DECSC as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '8' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_DECRC as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '8' as i32,
            interm: b"#\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_DECALN as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '=' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_DECKPAM as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '>' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_DECKPNM as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'B' as i32,
            interm: b"(\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_SCSG0_OFF as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'B' as i32,
            interm: b")\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_SCSG1_OFF as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'D' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_IND as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'E' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_NEL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'H' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_HTS as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'M' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_RI as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '\\' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_ST as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'c' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_ESC_RIS as libc::c_int,
        };
        init
    },
];
/* Control (CSI) command table. */
static mut input_csi_table: [input_table_entry; 39] = [
    {
        let mut init = input_table_entry {
            ch: '@' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_ICH as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'A' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUU as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'B' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUD as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'C' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUF as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'D' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUB as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'E' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CNL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'F' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CPL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'G' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_HPA as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'H' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUP as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'J' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_ED as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'K' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_EL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'L' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_IL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'M' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DL as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'P' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DCH as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'S' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SU as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'T' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SD as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'X' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_ECH as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'Z' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CBT as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: '`' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_HPA as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'b' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_REP as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'c' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DA as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'c' as i32,
            interm: b">\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DA_TWO as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'd' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_VPA as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'f' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_CUP as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'g' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_TBC as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'h' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SM as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'h' as i32,
            interm: b"?\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SM_PRIVATE as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'l' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_RM as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'l' as i32,
            interm: b"?\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_RM_PRIVATE as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'm' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SGR as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'm' as i32,
            interm: b">\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_MODSET as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'n' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DSR as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'n' as i32,
            interm: b">\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_MODOFF as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'q' as i32,
            interm: b" \x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DECSCUSR as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'q' as i32,
            interm: b">\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_XDA as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'r' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_DECSTBM as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 's' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_SCP as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 't' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_WINOPS as libc::c_int,
        };
        init
    },
    {
        let mut init = input_table_entry {
            ch: 'u' as i32,
            interm: b"\x00" as *const u8 as *const libc::c_char,
            type_0: INPUT_CSI_RCP as libc::c_int,
        };
        init
    },
];
/* State transitions available from all states. */
/* Forward declarations of state tables. */
/* ground state definition. */
static mut input_state_ground: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"ground\x00" as *const u8 as *const libc::c_char,
            enter: Some(input_ground as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            exit: None,
            transitions: input_state_ground_table.as_ptr(),
        };
        init
    }
};
/* esc_enter state definition. */
static mut input_state_esc_enter: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"esc_enter\x00" as *const u8 as *const libc::c_char,
            enter: Some(input_clear as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            exit: None,
            transitions: input_state_esc_enter_table.as_ptr(),
        };
        init
    }
};
/* esc_intermediate state definition. */
static mut input_state_esc_intermediate: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"esc_intermediate\x00" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_esc_intermediate_table.as_ptr(),
        };
        init
    }
};
/* csi_enter state definition. */
static mut input_state_csi_enter: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"csi_enter\x00" as *const u8 as *const libc::c_char,
            enter: Some(input_clear as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            exit: None,
            transitions: input_state_csi_enter_table.as_ptr(),
        };
        init
    }
};
/* csi_parameter state definition. */
static mut input_state_csi_parameter: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"csi_parameter\x00" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_csi_parameter_table.as_ptr(),
        };
        init
    }
};
/* csi_intermediate state definition. */
static mut input_state_csi_intermediate: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"csi_intermediate\x00" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_csi_intermediate_table.as_ptr(),
        };
        init
    }
};
/* csi_ignore state definition. */
static mut input_state_csi_ignore: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"csi_ignore\x00" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_csi_ignore_table.as_ptr(),
        };
        init
    }
};
/* dcs_enter state definition. */
static mut input_state_dcs_enter: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_enter\x00" as *const u8 as *const libc::c_char,
            enter: Some(input_enter_dcs as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            exit: None,
            transitions: input_state_dcs_enter_table.as_ptr(),
        };
        init
    }
};
/* dcs_parameter state definition. */
static mut input_state_dcs_parameter: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_parameter\x00" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_dcs_parameter_table.as_ptr(),
        };
        init
    }
};
/* dcs_intermediate state definition. */
static mut input_state_dcs_intermediate: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_intermediate\x00" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_dcs_intermediate_table.as_ptr(),
        };
        init
    }
};
/* dcs_handler state definition. */
static mut input_state_dcs_handler: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_handler\x00" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_dcs_handler_table.as_ptr(),
        };
        init
    }
};
/* dcs_escape state definition. */
static mut input_state_dcs_escape: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_escape\x00" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_dcs_escape_table.as_ptr(),
        };
        init
    }
};
/* dcs_ignore state definition. */
static mut input_state_dcs_ignore: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"dcs_ignore\x00" as *const u8 as *const libc::c_char,
            enter: None,
            exit: None,
            transitions: input_state_dcs_ignore_table.as_ptr(),
        };
        init
    }
};
/* osc_string state definition. */
static mut input_state_osc_string: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"osc_string\x00" as *const u8 as *const libc::c_char,
            enter: Some(input_enter_osc as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            exit: Some(input_exit_osc as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            transitions: input_state_osc_string_table.as_ptr(),
        };
        init
    }
};
/* apc_string state definition. */
static mut input_state_apc_string: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"apc_string\x00" as *const u8 as *const libc::c_char,
            enter: Some(input_enter_apc as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            exit: Some(input_exit_apc as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            transitions: input_state_apc_string_table.as_ptr(),
        };
        init
    }
};
/* rename_string state definition. */
static mut input_state_rename_string: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"rename_string\x00" as *const u8 as *const libc::c_char,
            enter: Some(input_enter_rename as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            exit: Some(input_exit_rename as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            transitions: input_state_rename_string_table.as_ptr(),
        };
        init
    }
};
/* consume_st state definition. */
static mut input_state_consume_st: input_state = unsafe {
    {
        let mut init = input_state {
            name: b"consume_st\x00" as *const u8 as *const libc::c_char,
            enter: Some(input_enter_rename as unsafe extern "C" fn(_: *mut input_ctx) -> ()),
            exit: None,
            transitions: input_state_consume_st_table.as_ptr(),
        };
        init
    }
};
/* ground state table. */
static mut input_state_ground_table: [input_transition; 10] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0x7ei32,
                handler: Some(
                    input_print as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7fi32,
                last: 0x7fi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x80i32,
                last: 0xffi32,
                handler: Some(
                    input_top_bit_set as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* esc_enter state table. */
static mut input_state_esc_enter_table: [input_transition; 23] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0x2fi32,
                handler: Some(
                    input_intermediate as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_esc_intermediate as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30i32,
                last: 0x4fi32,
                handler: Some(
                    input_esc_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x50i32,
                last: 0x50i32,
                handler: None,
                state: &input_state_dcs_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x51i32,
                last: 0x57i32,
                handler: Some(
                    input_esc_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x58i32,
                last: 0x58i32,
                handler: None,
                state: &input_state_consume_st as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x59i32,
                last: 0x59i32,
                handler: Some(
                    input_esc_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5ai32,
                last: 0x5ai32,
                handler: Some(
                    input_esc_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5bi32,
                last: 0x5bi32,
                handler: None,
                state: &input_state_csi_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5ci32,
                last: 0x5ci32,
                handler: Some(
                    input_esc_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5di32,
                last: 0x5di32,
                handler: None,
                state: &input_state_osc_string as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5ei32,
                last: 0x5ei32,
                handler: None,
                state: &input_state_consume_st as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5fi32,
                last: 0x5fi32,
                handler: None,
                state: &input_state_apc_string as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x60i32,
                last: 0x6ai32,
                handler: Some(
                    input_esc_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x6bi32,
                last: 0x6bi32,
                handler: None,
                state: &input_state_rename_string as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x6ci32,
                last: 0x7ei32,
                handler: Some(
                    input_esc_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7fi32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* esc_intermediate state table. */
static mut input_state_esc_intermediate_table: [input_transition; 10] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0x2fi32,
                handler: Some(
                    input_intermediate as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30i32,
                last: 0x7ei32,
                handler: Some(
                    input_esc_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7fi32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* csi_enter state table. */
static mut input_state_csi_enter_table: [input_transition; 14] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0x2fi32,
                handler: Some(
                    input_intermediate as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_intermediate as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30i32,
                last: 0x39i32,
                handler: Some(
                    input_parameter as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3ai32,
                last: 0x3ai32,
                handler: Some(
                    input_parameter as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3bi32,
                last: 0x3bi32,
                handler: Some(
                    input_parameter as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3ci32,
                last: 0x3fi32,
                handler: Some(
                    input_intermediate as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40i32,
                last: 0x7ei32,
                handler: Some(
                    input_csi_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7fi32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* csi_parameter state table. */
static mut input_state_csi_parameter_table: [input_transition; 14] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0x2fi32,
                handler: Some(
                    input_intermediate as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_csi_intermediate as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30i32,
                last: 0x39i32,
                handler: Some(
                    input_parameter as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3ai32,
                last: 0x3ai32,
                handler: Some(
                    input_parameter as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3bi32,
                last: 0x3bi32,
                handler: Some(
                    input_parameter as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3ci32,
                last: 0x3fi32,
                handler: None,
                state: &input_state_csi_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40i32,
                last: 0x7ei32,
                handler: Some(
                    input_csi_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7fi32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* csi_intermediate state table. */
static mut input_state_csi_intermediate_table: [input_transition; 11] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0x2fi32,
                handler: Some(
                    input_intermediate as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30i32,
                last: 0x3fi32,
                handler: None,
                state: &input_state_csi_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40i32,
                last: 0x7ei32,
                handler: Some(
                    input_csi_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7fi32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* csi_ignore state table. */
static mut input_state_csi_ignore_table: [input_transition; 10] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0x3fi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40i32,
                last: 0x7ei32,
                handler: None,
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7fi32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* dcs_enter state table. */
static mut input_state_dcs_enter_table: [input_transition; 14] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0x2fi32,
                handler: Some(
                    input_intermediate as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_intermediate as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30i32,
                last: 0x39i32,
                handler: Some(
                    input_parameter as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3ai32,
                last: 0x3ai32,
                handler: None,
                state: &input_state_dcs_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3bi32,
                last: 0x3bi32,
                handler: Some(
                    input_parameter as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3ci32,
                last: 0x3fi32,
                handler: Some(
                    input_intermediate as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_parameter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40i32,
                last: 0x7ei32,
                handler: Some(
                    input_input as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_handler as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7fi32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* dcs_parameter state table. */
static mut input_state_dcs_parameter_table: [input_transition; 14] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0x2fi32,
                handler: Some(
                    input_intermediate as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_intermediate as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30i32,
                last: 0x39i32,
                handler: Some(
                    input_parameter as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3ai32,
                last: 0x3ai32,
                handler: None,
                state: &input_state_dcs_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3bi32,
                last: 0x3bi32,
                handler: Some(
                    input_parameter as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x3ci32,
                last: 0x3fi32,
                handler: None,
                state: &input_state_dcs_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40i32,
                last: 0x7ei32,
                handler: Some(
                    input_input as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_handler as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7fi32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* dcs_intermediate state table. */
static mut input_state_dcs_intermediate_table: [input_transition; 11] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0x2fi32,
                handler: Some(
                    input_intermediate as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x30i32,
                last: 0x3fi32,
                handler: None,
                state: &input_state_dcs_ignore as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x40i32,
                last: 0x7ei32,
                handler: Some(
                    input_input as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_handler as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7fi32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* dcs_handler state table. */
static mut input_state_dcs_handler_table: [input_transition; 4] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x1ai32,
                handler: Some(
                    input_input as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_dcs_escape as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0xffi32,
                handler: Some(
                    input_input as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* dcs_escape state table. */
static mut input_state_dcs_escape_table: [input_transition; 4] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x5bi32,
                handler: Some(
                    input_input as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_handler as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5ci32,
                last: 0x5ci32,
                handler: Some(
                    input_dcs_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x5di32,
                last: 0xffi32,
                handler: Some(
                    input_input as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_dcs_handler as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* dcs_ignore state table. */
static mut input_state_dcs_ignore_table: [input_transition; 8] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* osc_string state table. */
static mut input_state_osc_string_table: [input_transition; 10] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x6i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x7i32,
                last: 0x7i32,
                handler: Some(
                    input_end_bel as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x8i32,
                last: 0x17i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0xffi32,
                handler: Some(
                    input_input as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* apc_string state table. */
static mut input_state_apc_string_table: [input_transition; 8] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0xffi32,
                handler: Some(
                    input_input as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* rename_string state table. */
static mut input_state_rename_string_table: [input_transition; 8] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0xffi32,
                handler: Some(
                    input_input as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* consume_st state table. */
static mut input_state_consume_st_table: [input_transition; 8] = unsafe {
    [
        {
            let mut init = input_transition {
                first: 0x18i32,
                last: 0x18i32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ai32,
                last: 0x1ai32,
                handler: Some(
                    input_c0_dispatch as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int,
                ),
                state: &input_state_ground as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1bi32,
                last: 0x1bi32,
                handler: None,
                state: &input_state_esc_enter as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0i32,
                last: 0x17i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x19i32,
                last: 0x19i32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x1ci32,
                last: 0x1fi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: 0x20i32,
                last: 0xffi32,
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
        {
            let mut init = input_transition {
                first: -(1i32),
                last: -(1i32),
                handler: None,
                state: 0 as *const input_state,
            };
            init
        },
    ]
};
/* Command table comparison function. */
/* Input table compare. */
unsafe extern "C" fn input_table_compare(
    mut key: *const libc::c_void,
    mut value: *const libc::c_void,
) -> libc::c_int {
    let mut ictx: *const input_ctx = key as *const input_ctx;
    let mut entry: *const input_table_entry = value as *const input_table_entry;
    if (*ictx).ch != (*entry).ch {
        return (*ictx).ch - (*entry).ch;
    }
    return strcmp(
        (*ictx).interm_buf.as_ptr() as *const libc::c_char,
        (*entry).interm,
    );
}
/*
 * Timer - if this expires then have been waiting for a terminator for too
 * long, so reset to ground.
 */
unsafe extern "C" fn input_timer_callback(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut ictx: *mut input_ctx = arg as *mut input_ctx;
    log_debug(
        b"%s: %s expired\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"input_timer_callback\x00"))
            .as_ptr(),
        (*(*ictx).state).name,
    );
    input_reset(ictx, 0i32);
}
/* Start the timer. */
unsafe extern "C" fn input_start_timer(mut ictx: *mut input_ctx) {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 5i64,
            tv_usec: 0i64,
        };
        init
    };
    event_del(&mut (*ictx).timer);
    event_add(&mut (*ictx).timer, &mut tv);
}
/* Reset cell state to default. */
unsafe extern "C" fn input_reset_cell(mut ictx: *mut input_ctx) {
    memcpy(
        &mut (*ictx).cell.cell as *mut GridCell as *mut libc::c_void,
        &grid_default_cell as *const GridCell as *const libc::c_void,
        ::std::mem::size_of::<GridCell>() as libc::c_ulong,
    );
    (*ictx).cell.set = 0i32;
    (*ictx).cell.g1set = 0i32;
    (*ictx).cell.g0set = (*ictx).cell.g1set;
    memcpy(
        &mut (*ictx).old_cell as *mut input_cell as *mut libc::c_void,
        &mut (*ictx).cell as *mut input_cell as *const libc::c_void,
        ::std::mem::size_of::<input_cell>() as libc::c_ulong,
    );
    (*ictx).old_cx = 0u32;
    (*ictx).old_cy = 0u32;
}
/* Save screen state. */
unsafe extern "C" fn input_save_state(mut ictx: *mut input_ctx) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut s: *mut screen = (*sctx).s;
    memcpy(
        &mut (*ictx).old_cell as *mut input_cell as *mut libc::c_void,
        &mut (*ictx).cell as *mut input_cell as *const libc::c_void,
        ::std::mem::size_of::<input_cell>() as libc::c_ulong,
    );
    (*ictx).old_cx = (*s).cx;
    (*ictx).old_cy = (*s).cy;
    (*ictx).old_mode = (*s).mode;
}
/* Restore screen state. */
unsafe extern "C" fn input_restore_state(mut ictx: *mut input_ctx) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    memcpy(
        &mut (*ictx).cell as *mut input_cell as *mut libc::c_void,
        &mut (*ictx).old_cell as *mut input_cell as *const libc::c_void,
        ::std::mem::size_of::<input_cell>() as libc::c_ulong,
    );
    if (*ictx).old_mode & 0x2000i32 != 0 {
        screen_write_mode_set(sctx, 0x2000i32);
    } else {
        screen_write_mode_clear(sctx, 0x2000i32);
    }
    screen_write_cursormove(
        sctx,
        (*ictx).old_cx as libc::c_int,
        (*ictx).old_cy as libc::c_int,
        0i32,
    );
}
/* Initialise input parser. */
#[no_mangle]
pub unsafe extern "C" fn input_init(
    mut wp: *mut window_pane,
    mut bev: *mut bufferevent,
) -> *mut input_ctx {
    let mut ictx: *mut input_ctx = 0 as *mut input_ctx;
    ictx = xcalloc(1u64, ::std::mem::size_of::<input_ctx>() as libc::c_ulong) as *mut input_ctx;
    (*ictx).wp = wp;
    (*ictx).event = bev;
    (*ictx).input_space = 32u64;
    (*ictx).input_buf = xmalloc(32u64) as *mut u_char;
    (*ictx).since_ground = evbuffer_new();
    if (*ictx).since_ground.is_null() {
        fatalx(b"out of memory\x00" as *const u8 as *const libc::c_char);
    }
    event_set(
        &mut (*ictx).timer,
        -(1i32),
        0i16,
        Some(
            input_timer_callback
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        ictx as *mut libc::c_void,
    );
    input_reset(ictx, 0i32);
    return ictx;
}
/* Destroy input parser. */
#[no_mangle]
pub unsafe extern "C" fn input_free(mut ictx: *mut input_ctx) {
    let mut i: u_int = 0;
    i = 0u32;
    while i < (*ictx).param_list_len {
        if (*ictx).param_list[i as usize].type_0 == INPUT_STRING {
            free((*ictx).param_list[i as usize].c2rust_unnamed.str_0 as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    event_del(&mut (*ictx).timer);
    free((*ictx).input_buf as *mut libc::c_void);
    evbuffer_free((*ictx).since_ground);
    free(ictx as *mut libc::c_void);
}
/* Reset input state and clear screen. */
#[no_mangle]
pub unsafe extern "C" fn input_reset(mut ictx: *mut input_ctx, mut clear: libc::c_int) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut wp: *mut window_pane = (*ictx).wp;
    input_reset_cell(ictx);
    if clear != 0 && !wp.is_null() {
        if (*wp).modes.tqh_first.is_null() {
            screen_write_start_pane(sctx, wp, &mut (*wp).base);
        } else {
            screen_write_start(sctx, &mut (*wp).base);
        }
        screen_write_reset(sctx);
        screen_write_stop(sctx);
    }
    input_clear(ictx);
    (*ictx).last = -(1i32);
    (*ictx).state = &input_state_ground;
    (*ictx).flags = 0i32;
}
/* Return pending data. */
#[no_mangle]
pub unsafe extern "C" fn input_pending(mut ictx: *mut input_ctx) -> *mut evbuffer {
    return (*ictx).since_ground;
}
/* Change input state. */
unsafe extern "C" fn input_set_state(mut ictx: *mut input_ctx, mut itr: *const input_transition) {
    if (*(*ictx).state).exit.is_some() {
        (*(*ictx).state).exit.expect("non-null function pointer")(ictx);
    }
    (*ictx).state = (*itr).state;
    if (*(*ictx).state).enter.is_some() {
        (*(*ictx).state).enter.expect("non-null function pointer")(ictx);
    };
}
/* Parse data. */
unsafe extern "C" fn input_parse(mut ictx: *mut input_ctx, mut buf: *mut u_char, mut len: size_t) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut state: *const input_state = 0 as *const input_state;
    let mut itr: *const input_transition = 0 as *const input_transition;
    let mut off: size_t = 0u64;
    /* Parse the input. */
    while off < len {
        let fresh0 = off;
        off = off.wrapping_add(1);
        (*ictx).ch = *buf.offset(fresh0 as isize) as libc::c_int;
        /* Find the transition. */
        if (*ictx).state != state
            || itr.is_null()
            || (*ictx).ch < (*itr).first
            || (*ictx).ch > (*itr).last
        {
            itr = (*(*ictx).state).transitions;
            while (*itr).first != -(1i32) && (*itr).last != -(1i32) {
                if (*ictx).ch >= (*itr).first && (*ictx).ch <= (*itr).last {
                    break;
                }
                itr = itr.offset(1)
            }
            if (*itr).first == -(1i32) || (*itr).last == -(1i32) {
                /* No transition? Eh? */
                fatalx(b"no transition from state\x00" as *const u8 as *const libc::c_char);
            }
        }
        state = (*ictx).state;
        /*
         * Any state except print stops the current collection. This is
         * an optimization to avoid checking if the attributes have
         * changed for every character. It will stop unnecessarily for
         * sequences that don't make a terminal change, but they should
         * be the minority.
         */
        if (*itr).handler
            != Some(input_print as unsafe extern "C" fn(_: *mut input_ctx) -> libc::c_int)
        {
            screen_write_collect_end(sctx);
        }
        /*
         * Execute the handler, if any. Don't switch state if it
         * returns non-zero.
         */
        if (*itr).handler.is_some()
            && (*itr).handler.expect("non-null function pointer")(ictx) != 0i32
        {
            continue;
        }
        /* And switch state, if necessary. */
        if !(*itr).state.is_null() {
            input_set_state(ictx, itr);
        }
        /* If not in ground state, save input. */
        if (*ictx).state != &input_state_ground as *const input_state {
            evbuffer_add(
                (*ictx).since_ground,
                &mut (*ictx).ch as *mut libc::c_int as *const libc::c_void,
                1u64,
            );
        }
    }
}
/* Parse input from pane. */
#[no_mangle]
pub unsafe extern "C" fn input_parse_pane(mut wp: *mut window_pane) {
    let mut new_data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_size: size_t = 0;
    new_data = window_pane_get_new_data(wp, &mut (*wp).offset, &mut new_size);
    input_parse_buffer(wp, new_data as *mut u_char, new_size);
    window_pane_update_used_data(wp, &mut (*wp).offset, new_size);
}
/* Parse given input. */
#[no_mangle]
pub unsafe extern "C" fn input_parse_buffer(
    mut wp: *mut window_pane,
    mut buf: *mut u_char,
    mut len: size_t,
) {
    let mut ictx: *mut input_ctx = (*wp).ictx;
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    if len == 0u64 {
        return;
    }
    window_update_activity((*wp).window);
    (*wp).flags |= 0x80i32;
    /* NULL wp if there is a mode set as don't want to update the tty. */
    if (*wp).modes.tqh_first.is_null() {
        screen_write_start_pane(sctx, wp, &mut (*wp).base);
    } else {
        screen_write_start(sctx, &mut (*wp).base);
    }
    log_debug(
        b"%s: %%%u %s, %zu bytes: %.*s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"input_parse_buffer\x00"))
            .as_ptr(),
        (*wp).id,
        (*(*ictx).state).name,
        len,
        len as libc::c_int,
        buf,
    );
    input_parse(ictx, buf, len);
    screen_write_stop(sctx);
}
/* Parse given input for screen. */
#[no_mangle]
pub unsafe extern "C" fn input_parse_screen(
    mut ictx: *mut input_ctx,
    mut s: *mut screen,
    mut cb: screen_write_init_ctx_cb,
    mut arg: *mut libc::c_void,
    mut buf: *mut u_char,
    mut len: size_t,
) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    if len == 0u64 {
        return;
    }
    screen_write_start_callback(sctx, s, cb, arg);
    input_parse(ictx, buf, len);
    screen_write_stop(sctx);
}
/* Helper functions. */
/* Split the parameter list (if any). */
unsafe extern "C" fn input_split(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip: *mut input_param = 0 as *mut input_param;
    let mut i: u_int = 0;
    i = 0u32;
    while i < (*ictx).param_list_len {
        if (*ictx).param_list[i as usize].type_0 == INPUT_STRING {
            free((*ictx).param_list[i as usize].c2rust_unnamed.str_0 as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    (*ictx).param_list_len = 0u32;
    if (*ictx).param_len == 0u64 {
        return 0i32;
    }
    ip = &mut *(*ictx).param_list.as_mut_ptr().offset(0isize) as *mut input_param;
    ptr = (*ictx).param_buf.as_mut_ptr() as *mut libc::c_char;
    loop {
        out = strsep(&mut ptr, b";\x00" as *const u8 as *const libc::c_char);
        if out.is_null() {
            break;
        }
        if *out as libc::c_int == '\u{0}' as i32 {
            (*ip).type_0 = INPUT_MISSING
        } else if !strchr(out, ':' as i32).is_null() {
            (*ip).type_0 = INPUT_STRING;
            (*ip).c2rust_unnamed.str_0 = xstrdup(out)
        } else {
            (*ip).type_0 = INPUT_NUMBER;
            (*ip).c2rust_unnamed.num =
                strtonum(out, 0i64, 2147483647i64, &mut errstr) as libc::c_int;
            if !errstr.is_null() {
                return -(1i32);
            }
        }
        (*ictx).param_list_len = (*ictx).param_list_len.wrapping_add(1);
        ip = &mut *(*ictx)
            .param_list
            .as_mut_ptr()
            .offset((*ictx).param_list_len as isize) as *mut input_param;
        if (*ictx).param_list_len as libc::c_ulong
            == (::std::mem::size_of::<[input_param; 24]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<input_param>() as libc::c_ulong)
        {
            return -(1i32);
        }
    }
    i = 0u32;
    while i < (*ictx).param_list_len {
        ip = &mut *(*ictx).param_list.as_mut_ptr().offset(i as isize) as *mut input_param;
        if (*ip).type_0 == INPUT_MISSING {
            log_debug(
                b"parameter %u: missing\x00" as *const u8 as *const libc::c_char,
                i,
            );
        } else if (*ip).type_0 == INPUT_STRING {
            log_debug(
                b"parameter %u: string %s\x00" as *const u8 as *const libc::c_char,
                i,
                (*ip).c2rust_unnamed.str_0,
            );
        } else if (*ip).type_0 == INPUT_NUMBER {
            log_debug(
                b"parameter %u: number %d\x00" as *const u8 as *const libc::c_char,
                i,
                (*ip).c2rust_unnamed.num,
            );
        }
        i = i.wrapping_add(1)
    }
    return 0i32;
}
/* Get an argument or return default value. */
unsafe extern "C" fn input_get(
    mut ictx: *mut input_ctx,
    mut validx: u_int,
    mut minval: libc::c_int,
    mut defval: libc::c_int,
) -> libc::c_int {
    let mut ip: *mut input_param = 0 as *mut input_param;
    let mut retval: libc::c_int = 0;
    if validx >= (*ictx).param_list_len {
        return defval;
    }
    ip = &mut *(*ictx).param_list.as_mut_ptr().offset(validx as isize) as *mut input_param;
    if (*ip).type_0 == INPUT_MISSING {
        return defval;
    }
    if (*ip).type_0 == INPUT_STRING {
        return -(1i32);
    }
    retval = (*ip).c2rust_unnamed.num;
    if retval < minval {
        return minval;
    }
    return retval;
}
/* Reply to terminal query. */
unsafe extern "C" fn input_reply(
    mut ictx: *mut input_ctx,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut bev: *mut bufferevent = (*ictx).event;
    let mut ap: ::std::ffi::VaListImpl;
    let mut reply: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    xvasprintf(&mut reply, fmt, ap.as_va_list());
    bufferevent_write(bev, reply as *const libc::c_void, strlen(reply));
    free(reply as *mut libc::c_void);
}
/* Transition entry/exit handlers. */
/* Clear saved state. */
unsafe extern "C" fn input_clear(mut ictx: *mut input_ctx) {
    event_del(&mut (*ictx).timer);
    *(*ictx).interm_buf.as_mut_ptr() = '\u{0}' as u_char;
    (*ictx).interm_len = 0u64;
    *(*ictx).param_buf.as_mut_ptr() = '\u{0}' as u_char;
    (*ictx).param_len = 0u64;
    *(*ictx).input_buf = '\u{0}' as u_char;
    (*ictx).input_len = 0u64;
    (*ictx).input_end = INPUT_END_ST;
    (*ictx).flags &= !(0x1i32);
}
/* Reset for ground state. */
unsafe extern "C" fn input_ground(mut ictx: *mut input_ctx) {
    event_del(&mut (*ictx).timer);
    evbuffer_drain(
        (*ictx).since_ground,
        evbuffer_get_length((*ictx).since_ground),
    );
    if (*ictx).input_space > 32u64 {
        (*ictx).input_space = 32u64;
        (*ictx).input_buf = xrealloc((*ictx).input_buf as *mut libc::c_void, 32u64) as *mut u_char
    };
}
/* Input state handlers. */
/* Output this character to the screen. */
unsafe extern "C" fn input_print(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx; /* can't be valid UTF-8 */
    let mut set: libc::c_int = 0;
    (*ictx).utf8started = 0i32;
    set = if (*ictx).cell.set == 0i32 {
        (*ictx).cell.g0set
    } else {
        (*ictx).cell.g1set
    };
    if set == 1i32 {
        (*ictx).cell.cell.attr = ((*ictx).cell.cell.attr as libc::c_int | 0x80i32) as u_short
    } else {
        (*ictx).cell.cell.attr = ((*ictx).cell.cell.attr as libc::c_int & !(0x80i32)) as u_short
    }
    utf8_set(&mut (*ictx).cell.cell.data, (*ictx).ch as u_char);
    screen_write_collect_add(sctx, &mut (*ictx).cell.cell);
    (*ictx).last = (*ictx).ch;
    (*ictx).cell.cell.attr = ((*ictx).cell.cell.attr as libc::c_int & !(0x80i32)) as u_short;
    return 0i32;
}
/* Collect intermediate string. */
unsafe extern "C" fn input_intermediate(mut ictx: *mut input_ctx) -> libc::c_int {
    if (*ictx).interm_len
        == (::std::mem::size_of::<[u_char; 4]>() as libc::c_ulong).wrapping_sub(1u64)
    {
        (*ictx).flags |= 0x1i32
    } else {
        let fresh1 = (*ictx).interm_len;
        (*ictx).interm_len = (*ictx).interm_len.wrapping_add(1);
        (*ictx).interm_buf[fresh1 as usize] = (*ictx).ch as u_char;
        (*ictx).interm_buf[(*ictx).interm_len as usize] = '\u{0}' as u_char
    }
    return 0i32;
}
/* Collect parameter string. */
unsafe extern "C" fn input_parameter(mut ictx: *mut input_ctx) -> libc::c_int {
    if (*ictx).param_len
        == (::std::mem::size_of::<[u_char; 64]>() as libc::c_ulong).wrapping_sub(1u64)
    {
        (*ictx).flags |= 0x1i32
    } else {
        let fresh2 = (*ictx).param_len;
        (*ictx).param_len = (*ictx).param_len.wrapping_add(1);
        (*ictx).param_buf[fresh2 as usize] = (*ictx).ch as u_char;
        (*ictx).param_buf[(*ictx).param_len as usize] = '\u{0}' as u_char
    }
    return 0i32;
}
/* Collect input string. */
unsafe extern "C" fn input_input(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut available: size_t = 0;
    available = (*ictx).input_space;
    while (*ictx).input_len.wrapping_add(1u64) >= available {
        available = (available).wrapping_mul(2u64);
        if available > 1048576u64 {
            (*ictx).flags |= 0x1i32;
            return 0i32;
        }
        (*ictx).input_buf =
            xrealloc((*ictx).input_buf as *mut libc::c_void, available) as *mut u_char;
        (*ictx).input_space = available
    }
    let fresh3 = (*ictx).input_len;
    (*ictx).input_len = (*ictx).input_len.wrapping_add(1);
    *(*ictx).input_buf.offset(fresh3 as isize) = (*ictx).ch as u_char;
    *(*ictx).input_buf.offset((*ictx).input_len as isize) = '\u{0}' as u_char;
    return 0i32;
}
/* Execute C0 control sequence. */
unsafe extern "C" fn input_c0_dispatch(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx; /* can't be valid UTF-8 */
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut s: *mut screen = (*sctx).s;
    (*ictx).utf8started = 0i32;
    log_debug(
        b"%s: \'%c\'\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"input_c0_dispatch\x00"))
            .as_ptr(),
        (*ictx).ch,
    );
    let mut current_block_13: u64;
    match (*ictx).ch {
        0 => {
            current_block_13 = 18386322304582297246;
        }
        7 => {
            /* BEL */
            if !wp.is_null() {
                alerts_queue((*wp).window, 0x1i32);
            }
            current_block_13 = 18386322304582297246;
        }
        8 => {
            /* BS */
            screen_write_backspace(sctx);
            current_block_13 = 18386322304582297246;
        }
        9 => {
            /* HT */
            /* Don't tab beyond the end of the line. */
            if (*s).cx >= (*(*s).grid).sx.wrapping_sub(1u32) {
                current_block_13 = 18386322304582297246;
            } else {
                loop
                /* Find the next tab point, or use the last column if none. */
                {
                    (*s).cx = (*s).cx.wrapping_add(1);
                    if *(*s).tabs.offset(((*s).cx >> 3i32) as isize) as libc::c_int
                        & (1i32) << ((*s).cx & 0x7u32)
                        != 0
                    {
                        break;
                    }
                    if !((*s).cx < (*(*s).grid).sx.wrapping_sub(1u32)) {
                        break;
                    }
                }
                current_block_13 = 18386322304582297246;
            }
        }
        11 => {
            /* VT */
            current_block_13 = 4414197365169205075;
        }
        10 | 12 => {
            current_block_13 = 4414197365169205075;
        }
        13 => {
            /* CR */
            screen_write_carriagereturn(sctx);
            current_block_13 = 18386322304582297246;
        }
        14 => {
            /* SO */
            (*ictx).cell.set = 1i32;
            current_block_13 = 18386322304582297246;
        }
        15 => {
            /* SI */
            (*ictx).cell.set = 0i32;
            current_block_13 = 18386322304582297246;
        }
        _ => {
            log_debug(
                b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                    b"input_c0_dispatch\x00",
                ))
                .as_ptr(),
                (*ictx).ch,
            );
            current_block_13 = 18386322304582297246;
        }
    }
    match current_block_13 {
        4414197365169205075 =>
        /* LF */
        /* FF */
        {
            screen_write_linefeed(sctx, 0i32, (*ictx).cell.cell.bg as u_int);
            if (*s).mode & 0x4000i32 != 0 {
                screen_write_carriagereturn(sctx);
            }
        }
        _ => {}
    }
    (*ictx).last = -(1i32);
    return 0i32;
}
/* Execute escape sequence. */
unsafe extern "C" fn input_esc_dispatch(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut s: *mut screen = (*sctx).s;
    let mut entry: *mut input_table_entry = 0 as *mut input_table_entry;
    if (*ictx).flags & 0x1i32 != 0 {
        return 0i32;
    }
    log_debug(
        b"%s: \'%c\', %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"input_esc_dispatch\x00"))
            .as_ptr(),
        (*ictx).ch,
        (*ictx).interm_buf.as_mut_ptr(),
    );
    entry = bsearch(
        ictx as *const libc::c_void,
        input_esc_table.as_ptr() as *const libc::c_void,
        (::std::mem::size_of::<[input_table_entry; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<input_table_entry>() as libc::c_ulong),
        ::std::mem::size_of::<input_table_entry>() as libc::c_ulong,
        Some(
            input_table_compare
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    ) as *mut input_table_entry;
    if entry.is_null() {
        log_debug(
            b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"input_esc_dispatch\x00"))
                .as_ptr(),
            (*ictx).ch,
        );
        return 0i32;
    }
    match (*entry).type_0 {
        9 => {
            if !wp.is_null() {
                window_pane_reset_palette(wp);
            }
            input_reset_cell(ictx);
            screen_write_reset(sctx);
        }
        6 => {
            screen_write_linefeed(sctx, 0i32, (*ictx).cell.cell.bg as u_int);
        }
        7 => {
            screen_write_carriagereturn(sctx);
            screen_write_linefeed(sctx, 0i32, (*ictx).cell.cell.bg as u_int);
        }
        5 => {
            if (*s).cx < (*(*s).grid).sx {
                let ref mut fresh4 = *(*s).tabs.offset(((*s).cx >> 3i32) as isize);
                *fresh4 = (*fresh4 as libc::c_int | (1i32) << ((*s).cx & 0x7u32)) as bitstr_t
            }
        }
        8 => {
            screen_write_reverseindex(sctx, (*ictx).cell.cell.bg as u_int);
        }
        1 => {
            screen_write_mode_set(sctx, 0x8i32);
        }
        2 => {
            screen_write_mode_clear(sctx, 0x8i32);
        }
        4 => {
            input_save_state(ictx);
        }
        3 => {
            input_restore_state(ictx);
        }
        0 => {
            screen_write_alignmenttest(sctx);
        }
        11 => (*ictx).cell.g0set = 1i32,
        10 => (*ictx).cell.g0set = 0i32,
        13 => (*ictx).cell.g1set = 1i32,
        12 => (*ictx).cell.g1set = 0i32,
        14 | _ => {}
    }
    (*ictx).last = -(1i32);
    return 0i32;
}
/* Execute control sequence. */
unsafe extern "C" fn input_csi_dispatch(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut s: *mut screen = (*sctx).s;
    let mut entry: *mut input_table_entry = 0 as *mut input_table_entry;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut cx: u_int = 0;
    let mut bg: u_int = (*ictx).cell.cell.bg as u_int;
    if (*ictx).flags & 0x1i32 != 0 {
        return 0i32;
    }
    log_debug(
        b"%s: \'%c\' \"%s\" \"%s\"\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"input_csi_dispatch\x00"))
            .as_ptr(),
        (*ictx).ch,
        (*ictx).interm_buf.as_mut_ptr(),
        (*ictx).param_buf.as_mut_ptr(),
    );
    if input_split(ictx) != 0i32 {
        return 0i32;
    }
    entry = bsearch(
        ictx as *const libc::c_void,
        input_csi_table.as_ptr() as *const libc::c_void,
        (::std::mem::size_of::<[input_table_entry; 39]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<input_table_entry>() as libc::c_ulong),
        ::std::mem::size_of::<input_table_entry>() as libc::c_ulong,
        Some(
            input_table_compare
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    ) as *mut input_table_entry;
    if entry.is_null() {
        log_debug(
            b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"input_csi_dispatch\x00"))
                .as_ptr(),
            (*ictx).ch,
        );
        return 0i32;
    }
    match (*entry).type_0 {
        0 => {
            /* Find the previous tab point, n times. */
            cx = (*s).cx;
            if cx > (*(*s).grid).sx.wrapping_sub(1u32) {
                cx = (*(*s).grid).sx.wrapping_sub(1u32)
            }
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if !(n == -(1i32)) {
                while cx > 0u32 && {
                    let fresh5 = n;
                    n = n - 1;
                    (fresh5) > 0i32
                } {
                    loop {
                        cx = cx.wrapping_sub(1);
                        if !(cx > 0u32
                            && *(*s).tabs.offset((cx >> 3i32) as isize) as libc::c_int
                                & (1i32) << (cx & 0x7u32)
                                == 0)
                        {
                            break;
                        }
                    }
                }
                (*s).cx = cx
            }
        }
        3 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_cursorleft(sctx, n as u_int);
            }
        }
        4 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_cursordown(sctx, n as u_int);
            }
        }
        5 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_cursorright(sctx, n as u_int);
            }
        }
        6 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            m = input_get(ictx, 1u32, 1i32, 1i32);
            if n != -(1i32) && m != -(1i32) {
                screen_write_cursormove(sctx, m - 1i32, n - 1i32, 1i32);
            }
        }
        22 => {
            n = input_get(ictx, 0u32, 0i32, 0i32);
            m = input_get(ictx, 1u32, 0i32, 0i32);
            if n == 0i32 || n == 4i32 && m == 0i32 {
                screen_write_mode_clear(sctx, 0x8000i32);
            } else if n == 4i32 && (m == 1i32 || m == 2i32) {
                screen_write_mode_set(sctx, 0x8000i32);
            }
        }
        21 => {
            n = input_get(ictx, 0u32, 0i32, 0i32);
            if n == 4i32 {
                screen_write_mode_clear(sctx, 0x8000i32);
            }
        }
        35 => {
            input_csi_dispatch_winops(ictx);
        }
        7 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_cursorup(sctx, n as u_int);
            }
        }
        1 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_carriagereturn(sctx);
                screen_write_cursordown(sctx, n as u_int);
            }
        }
        2 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_carriagereturn(sctx);
                screen_write_cursorup(sctx, n as u_int);
            }
        }
        8 => match input_get(ictx, 0u32, 0i32, 0i32) {
            -1 => {}
            0 => {
                input_reply(ictx, b"\x1b[?1;2c\x00" as *const u8 as *const libc::c_char);
            }
            _ => {
                log_debug(
                    b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"input_csi_dispatch\x00",
                    ))
                    .as_ptr(),
                    (*ictx).ch,
                );
            }
        },
        9 => match input_get(ictx, 0u32, 0i32, 0i32) {
            -1 => {}
            0 => {
                input_reply(
                    ictx,
                    b"\x1b[>84;0;0c\x00" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                log_debug(
                    b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"input_csi_dispatch\x00",
                    ))
                    .as_ptr(),
                    (*ictx).ch,
                );
            }
        },
        15 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_clearcharacter(sctx, n as u_int, bg);
            }
        }
        10 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_deletecharacter(sctx, n as u_int, bg);
            }
        }
        12 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            m = input_get(ictx, 1u32, 1i32, (*(*s).grid).sy as libc::c_int);
            if n != -(1i32) && m != -(1i32) {
                screen_write_scrollregion(sctx, (n - 1i32) as u_int, (m - 1i32) as u_int);
            }
        }
        13 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_deleteline(sctx, n as u_int, bg);
            }
        }
        14 => match input_get(ictx, 0u32, 0i32, 0i32) {
            -1 => {}
            5 => {
                input_reply(ictx, b"\x1b[0n\x00" as *const u8 as *const libc::c_char);
            }
            6 => {
                input_reply(
                    ictx,
                    b"\x1b[%u;%uR\x00" as *const u8 as *const libc::c_char,
                    (*s).cy.wrapping_add(1u32),
                    (*s).cx.wrapping_add(1u32),
                );
            }
            _ => {
                log_debug(
                    b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"input_csi_dispatch\x00",
                    ))
                    .as_ptr(),
                    (*ictx).ch,
                );
            }
        },
        16 => {
            match input_get(ictx, 0u32, 0i32, 0i32) {
                -1 => {}
                0 => {
                    screen_write_clearendofscreen(sctx, bg);
                }
                1 => {
                    screen_write_clearstartofscreen(sctx, bg);
                }
                2 => {
                    screen_write_clearscreen(sctx, bg);
                }
                3 => {
                    if input_get(ictx, 1u32, 0i32, 0i32) == 0i32 {
                        /*
                         * Linux console extension to clear history
                         * (for example before locking the screen).
                         */
                        screen_write_clearhistory(sctx);
                    }
                }
                _ => {
                    log_debug(
                        b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                            b"input_csi_dispatch\x00",
                        ))
                        .as_ptr(),
                        (*ictx).ch,
                    );
                }
            }
        }
        17 => match input_get(ictx, 0u32, 0i32, 0i32) {
            -1 => {}
            0 => {
                screen_write_clearendofline(sctx, bg);
            }
            1 => {
                screen_write_clearstartofline(sctx, bg);
            }
            2 => {
                screen_write_clearline(sctx, bg);
            }
            _ => {
                log_debug(
                    b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"input_csi_dispatch\x00",
                    ))
                    .as_ptr(),
                    (*ictx).ch,
                );
            }
        },
        18 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_cursormove(sctx, n - 1i32, -(1i32), 1i32);
            }
        }
        19 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_insertcharacter(sctx, n as u_int, bg);
            }
        }
        20 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_insertline(sctx, n as u_int, bg);
            }
        }
        24 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if !(n == -(1i32)) {
                m = (*(*s).grid).sx.wrapping_sub((*s).cx) as libc::c_int;
                if n > m {
                    n = m
                }
                if !((*ictx).last == -(1i32)) {
                    (*ictx).ch = (*ictx).last;
                    i = 0i32;
                    while i < n {
                        input_print(ictx);
                        i += 1
                    }
                }
            }
        }
        23 => {
            input_restore_state(ictx);
        }
        25 => {
            input_csi_dispatch_rm(ictx);
        }
        26 => {
            input_csi_dispatch_rm_private(ictx);
        }
        27 => {
            input_save_state(ictx);
        }
        29 => {
            input_csi_dispatch_sgr(ictx);
        }
        30 => {
            input_csi_dispatch_sm(ictx);
        }
        31 => {
            input_csi_dispatch_sm_private(ictx);
        }
        32 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_scrollup(sctx, n as u_int, bg);
            }
        }
        28 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_scrolldown(sctx, n as u_int, bg);
            }
        }
        33 => match input_get(ictx, 0u32, 0i32, 0i32) {
            -1 => {}
            0 => {
                if (*s).cx < (*(*s).grid).sx {
                    let ref mut fresh6 = *(*s).tabs.offset(((*s).cx >> 3i32) as isize);
                    *fresh6 = (*fresh6 as libc::c_int & !((1i32) << ((*s).cx & 0x7u32))) as bitstr_t
                }
            }
            3 => {
                let mut _name: *mut bitstr_t = (*s).tabs;
                let mut _start: libc::c_int = 0i32;
                let mut _stop: libc::c_int = (*(*s).grid).sx.wrapping_sub(1u32) as libc::c_int;
                while _start <= _stop {
                    let ref mut fresh7 = *_name.offset((_start >> 3i32) as isize);
                    *fresh7 = (*fresh7 as libc::c_int & !((1i32) << (_start & 0x7i32))) as bitstr_t;
                    _start += 1
                }
            }
            _ => {
                log_debug(
                    b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"input_csi_dispatch\x00",
                    ))
                    .as_ptr(),
                    (*ictx).ch,
                );
            }
        },
        34 => {
            n = input_get(ictx, 0u32, 1i32, 1i32);
            if n != -(1i32) {
                screen_write_cursormove(sctx, -(1i32), n - 1i32, 1i32);
            }
        }
        11 => {
            n = input_get(ictx, 0u32, 0i32, 0i32);
            if n != -(1i32) {
                screen_set_cursor_style(s, n as u_int);
            }
        }
        36 => {
            n = input_get(ictx, 0u32, 0i32, 0i32);
            if n == 0i32 {
                input_reply(
                    ictx,
                    b"\x1bP>|tmux %s\x1b\\\x00" as *const u8 as *const libc::c_char,
                    getversion(),
                );
            }
        }
        _ => {}
    }
    (*ictx).last = -(1i32);
    return 0i32;
}
/* Handle CSI RM. */
unsafe extern "C" fn input_csi_dispatch_rm(mut ictx: *mut input_ctx) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut i: u_int = 0;
    i = 0u32;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0i32, -(1i32)) {
            -1 => {}
            4 => {
                /* IRM */
                screen_write_mode_clear(sctx, 0x2i32);
            }
            34 => {
                screen_write_mode_set(sctx, 0x80i32);
            }
            _ => {
                log_debug(
                    b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"input_csi_dispatch_rm\x00",
                    ))
                    .as_ptr(),
                    (*ictx).ch,
                );
            }
        }
        i = i.wrapping_add(1)
    }
}
/* Handle CSI private RM. */
unsafe extern "C" fn input_csi_dispatch_rm_private(mut ictx: *mut input_ctx) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut gc: *mut GridCell = &mut (*ictx).cell.cell;
    let mut i: u_int = 0;
    i = 0u32;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0i32, -(1i32)) {
            -1 => {}
            1 => {
                /* DECCKM */
                screen_write_mode_clear(sctx, 0x4i32);
            }
            3 => {
                /* DECCOLM */
                screen_write_cursormove(sctx, 0i32, 0i32, 1i32);
                screen_write_clearscreen(sctx, (*gc).bg as u_int);
            }
            6 => {
                /* DECOM */
                screen_write_mode_clear(sctx, 0x2000i32);
                screen_write_cursormove(sctx, 0i32, 0i32, 1i32);
            }
            7 => {
                /* DECAWM */
                screen_write_mode_clear(sctx, 0x10i32);
            }
            12 => {
                screen_write_mode_clear(sctx, 0x80i32);
            }
            25 => {
                /* TCEM */
                screen_write_mode_clear(sctx, 0x1i32);
            }
            1000 | 1001 | 1002 | 1003 => {
                screen_write_mode_clear(sctx, 0x20i32 | 0x40i32 | 0x1000i32);
            }
            1004 => {
                screen_write_mode_clear(sctx, 0x800i32);
            }
            1005 => {
                screen_write_mode_clear(sctx, 0x100i32);
            }
            1006 => {
                screen_write_mode_clear(sctx, 0x200i32);
            }
            47 | 1047 => {
                screen_write_alternateoff(sctx, gc, 0i32);
            }
            1049 => {
                screen_write_alternateoff(sctx, gc, 1i32);
            }
            2004 => {
                screen_write_mode_clear(sctx, 0x400i32);
            }
            _ => {
                log_debug(
                    b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                        b"input_csi_dispatch_rm_private\x00",
                    ))
                    .as_ptr(),
                    (*ictx).ch,
                );
            }
        }
        i = i.wrapping_add(1)
    }
}
/* Handle CSI SM. */
unsafe extern "C" fn input_csi_dispatch_sm(mut ictx: *mut input_ctx) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut i: u_int = 0;
    i = 0u32;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0i32, -(1i32)) {
            -1 => {}
            4 => {
                /* IRM */
                screen_write_mode_set(sctx, 0x2i32);
            }
            34 => {
                screen_write_mode_clear(sctx, 0x80i32);
            }
            _ => {
                log_debug(
                    b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"input_csi_dispatch_sm\x00",
                    ))
                    .as_ptr(),
                    (*ictx).ch,
                );
            }
        }
        i = i.wrapping_add(1)
    }
}
/* Handle CSI private SM. */
unsafe extern "C" fn input_csi_dispatch_sm_private(mut ictx: *mut input_ctx) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut gc: *mut GridCell = &mut (*ictx).cell.cell;
    let mut i: u_int = 0;
    i = 0u32;
    while i < (*ictx).param_list_len {
        match input_get(ictx, i, 0i32, -(1i32)) {
            -1 => {}
            1 => {
                /* DECCKM */
                screen_write_mode_set(sctx, 0x4i32);
            }
            3 => {
                /* DECCOLM */
                screen_write_cursormove(sctx, 0i32, 0i32, 1i32);
                screen_write_clearscreen(sctx, (*ictx).cell.cell.bg as u_int);
            }
            6 => {
                /* DECOM */
                screen_write_mode_set(sctx, 0x2000i32);
                screen_write_cursormove(sctx, 0i32, 0i32, 1i32);
            }
            7 => {
                /* DECAWM */
                screen_write_mode_set(sctx, 0x10i32);
            }
            12 => {
                screen_write_mode_set(sctx, 0x80i32);
            }
            25 => {
                /* TCEM */
                screen_write_mode_set(sctx, 0x1i32); /* force update */
            }
            1000 => {
                screen_write_mode_clear(sctx, 0x20i32 | 0x40i32 | 0x1000i32);
                screen_write_mode_set(sctx, 0x20i32);
            }
            1002 => {
                screen_write_mode_clear(sctx, 0x20i32 | 0x40i32 | 0x1000i32);
                screen_write_mode_set(sctx, 0x40i32);
            }
            1003 => {
                screen_write_mode_clear(sctx, 0x20i32 | 0x40i32 | 0x1000i32);
                screen_write_mode_set(sctx, 0x1000i32);
            }
            1004 => {
                if !((*(*sctx).s).mode & 0x800i32 != 0) {
                    screen_write_mode_set(sctx, 0x800i32);
                    if !wp.is_null() {
                        (*wp).flags |= 0x20i32
                    }
                }
            }
            1005 => {
                screen_write_mode_set(sctx, 0x100i32);
            }
            1006 => {
                screen_write_mode_set(sctx, 0x200i32);
            }
            47 | 1047 => {
                screen_write_alternateon(sctx, gc, 0i32);
            }
            1049 => {
                screen_write_alternateon(sctx, gc, 1i32);
            }
            2004 => {
                screen_write_mode_set(sctx, 0x400i32);
            }
            _ => {
                log_debug(
                    b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                        b"input_csi_dispatch_sm_private\x00",
                    ))
                    .as_ptr(),
                    (*ictx).ch,
                );
            }
        }
        i = i.wrapping_add(1)
    }
}
/* Handle CSI window operations. */
unsafe extern "C" fn input_csi_dispatch_winops(mut ictx: *mut input_ctx) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut s: *mut screen = (*sctx).s;
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut x: u_int = (*(*s).grid).sx;
    let mut y: u_int = (*(*s).grid).sy;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    m = 0i32;
    loop {
        n = input_get(ictx, m as u_int, 0i32, -(1i32));
        if !(n != -(1i32)) {
            break;
        }
        let mut current_block_21: u64;
        match n {
            1 | 2 | 5 | 6 | 7 | 11 | 13 | 14 | 19 | 20 | 21 | 24 => {
                current_block_21 = 7245201122033322888;
            }
            3 | 4 | 8 => {
                m += 1;
                if input_get(ictx, m as u_int, 0i32, -(1i32)) == -(1i32) {
                    return;
                }
                current_block_21 = 5991938557911941140;
            }
            9 | 10 => {
                current_block_21 = 5991938557911941140;
            }
            22 => {
                m += 1;
                match input_get(ictx, m as u_int, 0i32, -(1i32)) {
                    -1 => return,
                    0 | 2 => {
                        screen_push_title((*sctx).s);
                    }
                    _ => {}
                }
                current_block_21 = 7245201122033322888;
            }
            23 => {
                m += 1;
                match input_get(ictx, m as u_int, 0i32, -(1i32)) {
                    -1 => return,
                    0 | 2 => {
                        screen_pop_title((*sctx).s);
                        if !wp.is_null() {
                            notify_pane(
                                b"pane-title-changed\x00" as *const u8 as *const libc::c_char,
                                wp,
                            );
                            server_redraw_window_borders((*wp).window);
                            server_status_window((*wp).window);
                        }
                    }
                    _ => {}
                }
                current_block_21 = 7245201122033322888;
            }
            18 => {
                input_reply(
                    ictx,
                    b"\x1b[8;%u;%ut\x00" as *const u8 as *const libc::c_char,
                    x,
                    y,
                );
                current_block_21 = 7245201122033322888;
            }
            _ => {
                log_debug(
                    b"%s: unknown \'%c\'\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                        b"input_csi_dispatch_winops\x00",
                    ))
                    .as_ptr(),
                    (*ictx).ch,
                );
                current_block_21 = 7245201122033322888;
            }
        }
        match current_block_21 {
            5991938557911941140 =>
            /* FALLTHROUGH */
            {
                m += 1;
                if input_get(ictx, m as u_int, 0i32, -(1i32)) == -(1i32) {
                    return;
                }
            }
            _ => {}
        }
        m += 1
    }
}
/* Helper for 256 colour SGR. */
unsafe extern "C" fn input_csi_dispatch_sgr_256_do(
    mut ictx: *mut input_ctx,
    mut fgbg: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut gc: *mut GridCell = &mut (*ictx).cell.cell;
    if c == -(1i32) || c > 255i32 {
        if fgbg == 38i32 {
            (*gc).fg = 8i32
        } else if fgbg == 48i32 {
            (*gc).bg = 8i32
        }
    } else if fgbg == 38i32 {
        (*gc).fg = c | 0x1000000i32
    } else if fgbg == 48i32 {
        (*gc).bg = c | 0x1000000i32
    } else if fgbg == 58i32 {
        (*gc).us = c | 0x1000000i32
    }
    return 1i32;
}
/* Handle CSI SGR for 256 colours. */
unsafe extern "C" fn input_csi_dispatch_sgr_256(
    mut ictx: *mut input_ctx,
    mut fgbg: libc::c_int,
    mut i: *mut u_int,
) {
    let mut c: libc::c_int = 0;
    c = input_get(ictx, (*i).wrapping_add(1u32), 0i32, -(1i32));
    if input_csi_dispatch_sgr_256_do(ictx, fgbg, c) != 0 {
        *i = (*i).wrapping_add(1)
    };
}
/* Helper for RGB colour SGR. */
unsafe extern "C" fn input_csi_dispatch_sgr_rgb_do(
    mut ictx: *mut input_ctx,
    mut fgbg: libc::c_int,
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    let mut gc: *mut GridCell = &mut (*ictx).cell.cell;
    if r == -(1i32) || r > 255i32 {
        return 0i32;
    }
    if g == -(1i32) || g > 255i32 {
        return 0i32;
    }
    if b == -(1i32) || b > 255i32 {
        return 0i32;
    }
    if fgbg == 38i32 {
        (*gc).fg = colour_join_rgb(r as u_char, g as u_char, b as u_char)
    } else if fgbg == 48i32 {
        (*gc).bg = colour_join_rgb(r as u_char, g as u_char, b as u_char)
    } else if fgbg == 58i32 {
        (*gc).us = colour_join_rgb(r as u_char, g as u_char, b as u_char)
    }
    return 1i32;
}
/* Handle CSI SGR for RGB colours. */
unsafe extern "C" fn input_csi_dispatch_sgr_rgb(
    mut ictx: *mut input_ctx,
    mut fgbg: libc::c_int,
    mut i: *mut u_int,
) {
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    r = input_get(ictx, (*i).wrapping_add(1u32), 0i32, -(1i32));
    g = input_get(ictx, (*i).wrapping_add(2u32), 0i32, -(1i32));
    b = input_get(ictx, (*i).wrapping_add(3u32), 0i32, -(1i32));
    if input_csi_dispatch_sgr_rgb_do(ictx, fgbg, r, g, b) != 0 {
        *i = (*i).wrapping_add(3u32)
    };
}
/* Handle CSI SGR with a ISO parameter. */
unsafe extern "C" fn input_csi_dispatch_sgr_colon(mut ictx: *mut input_ctx, mut i: u_int) {
    let mut gc: *mut GridCell = &mut (*ictx).cell.cell;
    let mut s: *mut libc::c_char = (*ictx).param_list[i as usize].c2rust_unnamed.str_0;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: [libc::c_int; 8] = [0; 8];
    let mut n: u_int = 0;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    n = 0u32;
    while (n as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_int; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        p[n as usize] = -(1i32);
        n = n.wrapping_add(1)
    }
    n = 0u32;
    copy = xstrdup(s);
    ptr = copy;
    loop {
        out = strsep(&mut ptr, b":\x00" as *const u8 as *const libc::c_char);
        if out.is_null() {
            break;
        }
        if *out as libc::c_int != '\u{0}' as i32 {
            let fresh8 = n;
            n = n.wrapping_add(1);
            p[fresh8 as usize] = strtonum(out, 0i64, 2147483647i64, &mut errstr) as libc::c_int;
            if !errstr.is_null()
                || n as libc::c_ulong
                    == (::std::mem::size_of::<[libc::c_int; 8]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            {
                free(copy as *mut libc::c_void);
                return;
            }
        } else {
            n = n.wrapping_add(1);
            if n as libc::c_ulong
                == (::std::mem::size_of::<[libc::c_int; 8]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            {
                free(copy as *mut libc::c_void);
                return;
            }
        }
        log_debug(
            b"%s: %u = %d\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"input_csi_dispatch_sgr_colon\x00",
            ))
            .as_ptr(),
            n.wrapping_sub(1u32),
            p[n.wrapping_sub(1u32) as usize],
        );
    }
    free(copy as *mut libc::c_void);
    if n == 0u32 {
        return;
    }
    if p[0usize] == 4i32 {
        if n != 2u32 {
            return;
        }
        match p[1usize] {
            0 => {
                (*gc).attr = ((*gc).attr as libc::c_int
                    & !(0x4i32 | 0x200i32 | 0x400i32 | 0x800i32 | 0x1000i32))
                    as u_short
            }
            1 => {
                (*gc).attr = ((*gc).attr as libc::c_int
                    & !(0x4i32 | 0x200i32 | 0x400i32 | 0x800i32 | 0x1000i32))
                    as u_short;
                (*gc).attr = ((*gc).attr as libc::c_int | 0x4i32) as u_short
            }
            2 => {
                (*gc).attr = ((*gc).attr as libc::c_int
                    & !(0x4i32 | 0x200i32 | 0x400i32 | 0x800i32 | 0x1000i32))
                    as u_short;
                (*gc).attr = ((*gc).attr as libc::c_int | 0x200i32) as u_short
            }
            3 => {
                (*gc).attr = ((*gc).attr as libc::c_int
                    & !(0x4i32 | 0x200i32 | 0x400i32 | 0x800i32 | 0x1000i32))
                    as u_short;
                (*gc).attr = ((*gc).attr as libc::c_int | 0x400i32) as u_short
            }
            4 => {
                (*gc).attr = ((*gc).attr as libc::c_int
                    & !(0x4i32 | 0x200i32 | 0x400i32 | 0x800i32 | 0x1000i32))
                    as u_short;
                (*gc).attr = ((*gc).attr as libc::c_int | 0x800i32) as u_short
            }
            5 => {
                (*gc).attr = ((*gc).attr as libc::c_int
                    & !(0x4i32 | 0x200i32 | 0x400i32 | 0x800i32 | 0x1000i32))
                    as u_short;
                (*gc).attr = ((*gc).attr as libc::c_int | 0x1000i32) as u_short
            }
            _ => {}
        }
        return;
    }
    if n < 2u32 || p[0usize] != 38i32 && p[0usize] != 48i32 && p[0usize] != 58i32 {
        return;
    }
    match p[1usize] {
        2 => {
            if !(n < 3u32) {
                if n == 5u32 {
                    i = 2u32
                } else {
                    i = 3u32
                }
                if !(n < i.wrapping_add(3u32)) {
                    input_csi_dispatch_sgr_rgb_do(
                        ictx,
                        p[0usize],
                        p[i as usize],
                        p[i.wrapping_add(1u32) as usize],
                        p[i.wrapping_add(2u32) as usize],
                    );
                }
            }
        }
        5 => {
            if !(n < 3u32) {
                input_csi_dispatch_sgr_256_do(ictx, p[0usize], p[2usize]);
            }
        }
        _ => {}
    };
}
/* Handle CSI SGR. */
unsafe extern "C" fn input_csi_dispatch_sgr(mut ictx: *mut input_ctx) {
    let mut gc: *mut GridCell = &mut (*ictx).cell.cell;
    let mut i: u_int = 0;
    let mut n: libc::c_int = 0;
    if (*ictx).param_list_len == 0u32 {
        memcpy(
            gc as *mut libc::c_void,
            &grid_default_cell as *const GridCell as *const libc::c_void,
            ::std::mem::size_of::<GridCell>() as libc::c_ulong,
        );
        return;
    }
    i = 0u32;
    while i < (*ictx).param_list_len {
        if (*ictx).param_list[i as usize].type_0 == INPUT_STRING {
            input_csi_dispatch_sgr_colon(ictx, i);
        } else {
            n = input_get(ictx, i, 0i32, 0i32);
            if !(n == -(1i32)) {
                if n == 38i32 || n == 48i32 || n == 58i32 {
                    i = i.wrapping_add(1);
                    match input_get(ictx, i, 0i32, -(1i32)) {
                        2 => {
                            input_csi_dispatch_sgr_rgb(ictx, n, &mut i);
                        }
                        5 => {
                            input_csi_dispatch_sgr_256(ictx, n, &mut i);
                        }
                        _ => {}
                    }
                } else {
                    match n {
                        0 => {
                            memcpy(
                                gc as *mut libc::c_void,
                                &grid_default_cell as *const GridCell as *const libc::c_void,
                                ::std::mem::size_of::<GridCell>() as libc::c_ulong,
                            );
                        }
                        1 => (*gc).attr = ((*gc).attr as libc::c_int | 0x1i32) as u_short,
                        2 => (*gc).attr = ((*gc).attr as libc::c_int | 0x2i32) as u_short,
                        3 => (*gc).attr = ((*gc).attr as libc::c_int | 0x40i32) as u_short,
                        4 => {
                            (*gc).attr = ((*gc).attr as libc::c_int
                                & !(0x4i32 | 0x200i32 | 0x400i32 | 0x800i32 | 0x1000i32))
                                as u_short;
                            (*gc).attr = ((*gc).attr as libc::c_int | 0x4i32) as u_short
                        }
                        5 => (*gc).attr = ((*gc).attr as libc::c_int | 0x8i32) as u_short,
                        7 => (*gc).attr = ((*gc).attr as libc::c_int | 0x10i32) as u_short,
                        8 => (*gc).attr = ((*gc).attr as libc::c_int | 0x20i32) as u_short,
                        9 => (*gc).attr = ((*gc).attr as libc::c_int | 0x100i32) as u_short,
                        22 => {
                            (*gc).attr = ((*gc).attr as libc::c_int & !(0x1i32 | 0x2i32)) as u_short
                        }
                        23 => (*gc).attr = ((*gc).attr as libc::c_int & !(0x40i32)) as u_short,
                        24 => {
                            (*gc).attr = ((*gc).attr as libc::c_int
                                & !(0x4i32 | 0x200i32 | 0x400i32 | 0x800i32 | 0x1000i32))
                                as u_short
                        }
                        25 => (*gc).attr = ((*gc).attr as libc::c_int & !(0x8i32)) as u_short,
                        27 => (*gc).attr = ((*gc).attr as libc::c_int & !(0x10i32)) as u_short,
                        28 => (*gc).attr = ((*gc).attr as libc::c_int & !(0x20i32)) as u_short,
                        29 => (*gc).attr = ((*gc).attr as libc::c_int & !(0x100i32)) as u_short,
                        30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 => (*gc).fg = n - 30i32,
                        39 => (*gc).fg = 8i32,
                        40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 => (*gc).bg = n - 40i32,
                        49 => (*gc).bg = 8i32,
                        53 => (*gc).attr = ((*gc).attr as libc::c_int | 0x2000i32) as u_short,
                        55 => (*gc).attr = ((*gc).attr as libc::c_int & !(0x2000i32)) as u_short,
                        59 => (*gc).us = 0i32,
                        90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 => (*gc).fg = n,
                        100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 => (*gc).bg = n - 10i32,
                        _ => {}
                    }
                }
            }
        }
        i = i.wrapping_add(1)
    }
}
/* End of input with BEL. */
unsafe extern "C" fn input_end_bel(mut ictx: *mut input_ctx) -> libc::c_int {
    log_debug(
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"input_end_bel\x00")).as_ptr(),
    );
    (*ictx).input_end = INPUT_END_BEL;
    return 0i32;
}
/* DCS string started. */
unsafe extern "C" fn input_enter_dcs(mut ictx: *mut input_ctx) {
    log_debug(
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"input_enter_dcs\x00")).as_ptr(),
    );
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = -(1i32);
}
/* DCS terminator (ST) received. */
unsafe extern "C" fn input_dcs_dispatch(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut buf: *mut u_char = (*ictx).input_buf;
    let mut len: size_t = (*ictx).input_len;
    let prefix: [libc::c_char; 6] =
        *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"tmux;\x00");
    let prefixlen: u_int =
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong).wrapping_sub(1u64) as u_int;
    if (*ictx).flags & 0x1i32 != 0 {
        return 0i32;
    }
    log_debug(
        b"%s: \"%s\"\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"input_dcs_dispatch\x00"))
            .as_ptr(),
        buf,
    );
    if len >= prefixlen as libc::c_ulong
        && strncmp(
            buf as *const libc::c_char,
            prefix.as_ptr(),
            prefixlen as libc::c_ulong,
        ) == 0i32
    {
        screen_write_rawstring(
            sctx,
            buf.offset(prefixlen as isize),
            len.wrapping_sub(prefixlen as libc::c_ulong) as u_int,
        );
    }
    return 0i32;
}
/* OSC string started. */
unsafe extern "C" fn input_enter_osc(mut ictx: *mut input_ctx) {
    log_debug(
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"input_enter_osc\x00")).as_ptr(),
    );
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = -(1i32);
}
/* OSC terminator (ST) received. */
unsafe extern "C" fn input_exit_osc(mut ictx: *mut input_ctx) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut p: *mut u_char = (*ictx).input_buf;
    let mut option: u_int = 0;
    if (*ictx).flags & 0x1i32 != 0 {
        return;
    }
    if (*ictx).input_len < 1u64
        || (*p as libc::c_int) < '0' as i32
        || *p as libc::c_int > '9' as i32
    {
        return;
    }
    log_debug(
        b"%s: \"%s\" (end %s)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"input_exit_osc\x00")).as_ptr(),
        p,
        if (*ictx).input_end == INPUT_END_ST {
            b"ST\x00" as *const u8 as *const libc::c_char
        } else {
            b"BEL\x00" as *const u8 as *const libc::c_char
        },
    );
    option = 0u32;
    while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
        let fresh9 = p;
        p = p.offset(1);
        option = option
            .wrapping_mul(10u32)
            .wrapping_add(*fresh9 as libc::c_uint)
            .wrapping_sub('0' as libc::c_uint)
    }
    if *p as libc::c_int == ';' as i32 {
        p = p.offset(1)
    }
    match option {
        0 | 2 => {
            if screen_set_title((*sctx).s, p as *const libc::c_char) != 0 && !wp.is_null() {
                notify_pane(
                    b"pane-title-changed\x00" as *const u8 as *const libc::c_char,
                    wp,
                );
                server_redraw_window_borders((*wp).window);
                server_status_window((*wp).window);
            }
        }
        4 => {
            input_osc_4(ictx, p as *const libc::c_char);
        }
        7 => {
            if utf8_isvalid(p as *const libc::c_char) != 0 {
                screen_set_path((*sctx).s, p as *const libc::c_char);
                if !wp.is_null() {
                    server_redraw_window_borders((*wp).window);
                    server_status_window((*wp).window);
                }
            }
        }
        10 => {
            input_osc_10(ictx, p as *const libc::c_char);
        }
        11 => {
            input_osc_11(ictx, p as *const libc::c_char);
        }
        12 => {
            if utf8_isvalid(p as *const libc::c_char) != 0 && *p as libc::c_int != '?' as i32 {
                /* ? is colour request */
                screen_set_cursor_colour((*sctx).s, p as *const libc::c_char);
            }
        }
        52 => {
            input_osc_52(ictx, p as *const libc::c_char);
        }
        104 => {
            input_osc_104(ictx, p as *const libc::c_char);
        }
        112 => {
            if *p as libc::c_int == '\u{0}' as i32 {
                /* no arguments allowed */
                screen_set_cursor_colour((*sctx).s, b"\x00" as *const u8 as *const libc::c_char);
            }
        }
        _ => {
            log_debug(
                b"%s: unknown \'%u\'\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"input_exit_osc\x00"))
                    .as_ptr(),
                option,
            );
        }
    };
}
/* APC string started. */
unsafe extern "C" fn input_enter_apc(mut ictx: *mut input_ctx) {
    log_debug(
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"input_enter_apc\x00")).as_ptr(),
    );
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = -(1i32);
}
/* APC terminator (ST) received. */
unsafe extern "C" fn input_exit_apc(mut ictx: *mut input_ctx) {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut wp: *mut window_pane = (*ictx).wp;
    if (*ictx).flags & 0x1i32 != 0 {
        return;
    }
    log_debug(
        b"%s: \"%s\"\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"input_exit_apc\x00")).as_ptr(),
        (*ictx).input_buf,
    );
    if screen_set_title((*sctx).s, (*ictx).input_buf as *const libc::c_char) != 0 && !wp.is_null() {
        notify_pane(
            b"pane-title-changed\x00" as *const u8 as *const libc::c_char,
            wp,
        );
        server_redraw_window_borders((*wp).window);
        server_status_window((*wp).window);
    };
}
/* Rename string started. */
unsafe extern "C" fn input_enter_rename(mut ictx: *mut input_ctx) {
    log_debug(
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"input_enter_rename\x00"))
            .as_ptr(),
    );
    input_clear(ictx);
    input_start_timer(ictx);
    (*ictx).last = -(1i32);
}
/* Rename terminator (ST) received. */
unsafe extern "C" fn input_exit_rename(mut ictx: *mut input_ctx) {
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut o: *mut crate::options::options_entry = 0 as *mut crate::options::options_entry;
    if wp.is_null() {
        return;
    }
    if (*ictx).flags & 0x1i32 != 0 {
        return;
    }
    if options_get_number(
        (*(*ictx).wp).options,
        b"allow-rename\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return;
    }
    log_debug(
        b"%s: \"%s\"\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"input_exit_rename\x00"))
            .as_ptr(),
        (*ictx).input_buf,
    );
    if utf8_isvalid((*ictx).input_buf as *const libc::c_char) == 0 {
        return;
    }
    if (*ictx).input_len == 0u64 {
        o = options_get_only(
            (*(*wp).window).options,
            b"automatic-rename\x00" as *const u8 as *const libc::c_char,
        );
        if !o.is_null() {
            options_remove_or_default(o, -(1i32), 0 as *mut *mut libc::c_char);
        }
        return;
    }
    window_set_name((*wp).window, (*ictx).input_buf as *const libc::c_char);
    options_set_number(
        (*(*wp).window).options,
        b"automatic-rename\x00" as *const u8 as *const libc::c_char,
        0i64,
    );
    server_redraw_window_borders((*wp).window);
    server_status_window((*wp).window);
}
/* Open UTF-8 character. */
unsafe extern "C" fn input_top_bit_set(mut ictx: *mut input_ctx) -> libc::c_int {
    let mut sctx: *mut screen_write_ctx = &mut (*ictx).ctx;
    let mut ud: *mut Utf8Data = &mut (*ictx).utf8data;
    (*ictx).last = -(1i32);
    if (*ictx).utf8started == 0 {
        if utf8_open(ud, (*ictx).ch as u_char) != utf8_state::MORE {
            return 0i32;
        }
        (*ictx).utf8started = 1i32;
        return 0i32;
    }
    match utf8_append(ud, (*ictx).ch as u_char) {
        0 => return 0i32,
        2 => {
            (*ictx).utf8started = 0i32;
            return 0i32;
        }
        1 | _ => {}
    }
    (*ictx).utf8started = 0i32;
    log_debug(
        b"%s %hhu \'%*s\' (width %hhu)\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"input_top_bit_set\x00"))
            .as_ptr(),
        (*ud).size as libc::c_int,
        (*ud).size as libc::c_int,
        (*ud).data.as_mut_ptr(),
        (*ud).width as libc::c_int,
    );
    utf8_copy(&mut (*ictx).cell.cell.data, ud);
    screen_write_collect_add(sctx, &mut (*ictx).cell.cell);
    return 0i32;
}
/* Parse colour from OSC. */
unsafe extern "C" fn input_osc_parse_colour(
    mut p: *const libc::c_char,
    mut r: *mut u_int,
    mut g: *mut u_int,
    mut b: *mut u_int,
) -> libc::c_int {
    let mut rsize: u_int = 0;
    let mut gsize: u_int = 0;
    let mut bsize: u_int = 0;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = p;
    if sscanf(
        p,
        b"rgb:%x/%x/%x\x00" as *const u8 as *const libc::c_char,
        r,
        g,
        b,
    ) != 3i32
    {
        return 0i32;
    }
    p = p.offset(4isize);
    cp = strchr(p, '/' as i32);
    rsize = cp.wrapping_offset_from(p) as u_int;
    if rsize == 1u32 {
        *r = *r | *r << 4i32
    } else if rsize == 3u32 {
        *r >>= 4i32
    } else if rsize == 4u32 {
        *r >>= 8i32
    } else if rsize != 2u32 {
        return 0i32;
    }
    p = cp.offset(1isize);
    cp = strchr(p, '/' as i32);
    gsize = cp.wrapping_offset_from(p) as u_int;
    if gsize == 1u32 {
        *g = *g | *g << 4i32
    } else if gsize == 3u32 {
        *g >>= 4i32
    } else if gsize == 4u32 {
        *g >>= 8i32
    } else if gsize != 2u32 {
        return 0i32;
    }
    bsize = strlen(cp.offset(1isize)) as u_int;
    if bsize == 1u32 {
        *b = *b | *b << 4i32
    } else if bsize == 3u32 {
        *b >>= 4i32
    } else if bsize == 4u32 {
        *b >>= 8i32
    } else if bsize != 2u32 {
        return 0i32;
    }
    log_debug(
        b"%s: %s = %02x%02x%02x\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"input_osc_parse_colour\x00"))
            .as_ptr(),
        s,
        *r,
        *g,
        *b,
    );
    return 1i32;
}
/* Reply to a colour request. */
unsafe extern "C" fn input_osc_colour_reply(
    mut ictx: *mut input_ctx,
    mut n: u_int,
    mut c: libc::c_int,
) {
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    if c == 8i32 || !c & 0x2000000i32 != 0 {
        return;
    }
    colour_split_rgb(c, &mut r, &mut g, &mut b);
    if (*ictx).input_end == INPUT_END_BEL {
        end = b"\x07\x00" as *const u8 as *const libc::c_char
    } else {
        end = b"\x1b\\\x00" as *const u8 as *const libc::c_char
    }
    input_reply(
        ictx,
        b"\x1b]%u;rgb:%02hhx/%02hhx/%02hhx%s\x00" as *const u8 as *const libc::c_char,
        n,
        r as libc::c_int,
        g as libc::c_int,
        b as libc::c_int,
        end,
    );
}
/* Handle the OSC 4 sequence for setting (multiple) palette entries. */
unsafe extern "C" fn input_osc_4(mut ictx: *mut input_ctx, mut p: *const libc::c_char) {
    let mut current_block: u64;
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_long = 0;
    let mut r: u_int = 0;
    let mut g: u_int = 0;
    let mut b: u_int = 0;
    if wp.is_null() {
        return;
    }
    s = xstrdup(p);
    copy = s;
    loop {
        if !(!s.is_null() && *s as libc::c_int != '\u{0}' as i32) {
            current_block = 10048703153582371463;
            break;
        }
        idx = strtol(s, &mut next, 10i32);
        let fresh10 = next;
        next = next.offset(1);
        if *fresh10 as libc::c_int != ';' as i32 {
            current_block = 12820755034573918988;
            break;
        }
        if idx < 0i64 || idx >= 0x100i64 {
            current_block = 12820755034573918988;
            break;
        }
        s = strsep(&mut next, b";\x00" as *const u8 as *const libc::c_char);
        if input_osc_parse_colour(s, &mut r, &mut g, &mut b) == 0 {
            s = next
        } else {
            window_pane_set_palette(
                wp,
                idx as u_int,
                colour_join_rgb(r as u_char, g as u_char, b as u_char),
            );
            s = next
        }
    }
    match current_block {
        12820755034573918988 => {
            log_debug(b"bad OSC 4: %s\x00" as *const u8 as *const libc::c_char, p);
            free(copy as *mut libc::c_void);
            return;
        }
        _ => {
            free(copy as *mut libc::c_void);
            return;
        }
    };
}
/* Handle the OSC 10 sequence for setting and querying foreground colour. */
unsafe extern "C" fn input_osc_10(mut ictx: *mut input_ctx, mut p: *const libc::c_char) {
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut defaults: GridCell = GridCell {
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
    let mut r: u_int = 0;
    let mut g: u_int = 0;
    let mut b: u_int = 0;
    if wp.is_null() {
        return;
    }
    if strcmp(p, b"?\x00" as *const u8 as *const libc::c_char) == 0i32 {
        tty_default_colours(&mut defaults, wp);
        input_osc_colour_reply(ictx, 10u32, defaults.fg);
        return;
    }
    if input_osc_parse_colour(p, &mut r, &mut g, &mut b) == 0 {
        log_debug(b"bad OSC 10: %s\x00" as *const u8 as *const libc::c_char, p);
        return;
    } else {
        (*wp).fg = colour_join_rgb(r as u_char, g as u_char, b as u_char);
        (*wp).flags |= 0x1i32 | 0x1000i32;
        return;
    };
}
/* Handle the OSC 11 sequence for setting and querying background colour. */
unsafe extern "C" fn input_osc_11(mut ictx: *mut input_ctx, mut p: *const libc::c_char) {
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut defaults: GridCell = GridCell {
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
    let mut r: u_int = 0;
    let mut g: u_int = 0;
    let mut b: u_int = 0;
    if wp.is_null() {
        return;
    }
    if strcmp(p, b"?\x00" as *const u8 as *const libc::c_char) == 0i32 {
        tty_default_colours(&mut defaults, wp);
        input_osc_colour_reply(ictx, 11u32, defaults.bg);
        return;
    }
    if input_osc_parse_colour(p, &mut r, &mut g, &mut b) == 0 {
        log_debug(b"bad OSC 11: %s\x00" as *const u8 as *const libc::c_char, p);
        return;
    } else {
        (*wp).bg = colour_join_rgb(r as u_char, g as u_char, b as u_char);
        (*wp).flags |= 0x1i32 | 0x1000i32;
        return;
    };
}
/* Handle the OSC 52 sequence for setting the clipboard. */
unsafe extern "C" fn input_osc_52(mut ictx: *mut input_ctx, mut p: *const libc::c_char) {
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut out: *mut u_char = 0 as *mut u_char;
    let mut outlen: libc::c_int = 0;
    let mut state: libc::c_int = 0;
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
    let mut pb: *mut crate::paste::paste_buffer = 0 as *mut crate::paste::paste_buffer;
    if wp.is_null() {
        return;
    }
    state = options_get_number(
        global_options,
        b"set-clipboard\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if state != 2i32 {
        return;
    }
    end = strchr(p, ';' as i32);
    if end.is_null() {
        return;
    }
    end = end.offset(1);
    if *end as libc::c_int == '\u{0}' as i32 {
        return;
    }
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"input_osc_52\x00")).as_ptr(),
        end,
    );
    if strcmp(end, b"?\x00" as *const u8 as *const libc::c_char) == 0i32 {
        pb = paste_get_top(0 as *mut *const libc::c_char);
        if !pb.is_null() {
            buf = paste_buffer_data(pb, &mut len);
            outlen = (4u64)
                .wrapping_mul(len.wrapping_add(2u64).wrapping_div(3u64))
                .wrapping_add(1u64) as libc::c_int;
            out = xmalloc(outlen as size_t) as *mut u_char;
            outlen = __b64_ntop(
                buf as *const libc::c_uchar,
                len,
                out as *mut libc::c_char,
                outlen as size_t,
            );
            if outlen == -(1i32) {
                free(out as *mut libc::c_void);
                return;
            }
        } else {
            outlen = 0i32;
            out = 0 as *mut u_char
        }
        bufferevent_write(
            (*ictx).event,
            b"\x1b]52;;\x00" as *const u8 as *const libc::c_void,
            6u64,
        );
        if outlen != 0i32 {
            bufferevent_write((*ictx).event, out as *const libc::c_void, outlen as size_t);
        }
        if (*ictx).input_end == INPUT_END_BEL {
            bufferevent_write(
                (*ictx).event,
                b"\x07\x00" as *const u8 as *const libc::c_void,
                1u64,
            );
        } else {
            bufferevent_write(
                (*ictx).event,
                b"\x1b\\\x00" as *const u8 as *const libc::c_void,
                2u64,
            );
        }
        free(out as *mut libc::c_void);
        return;
    }
    len = strlen(end).wrapping_div(4u64).wrapping_mul(3u64);
    if len == 0u64 {
        return;
    }
    out = xmalloc(len) as *mut u_char;
    outlen = __b64_pton(end, out, len);
    if outlen == -(1i32) {
        free(out as *mut libc::c_void);
        return;
    }
    screen_write_start_pane(&mut ctx, wp, 0 as *mut screen);
    screen_write_setselection(&mut ctx, out, outlen as u_int);
    screen_write_stop(&mut ctx);
    notify_pane(
        b"pane-set-clipboard\x00" as *const u8 as *const libc::c_char,
        wp,
    );
    paste_add(
        0 as *const libc::c_char,
        out as *mut libc::c_char,
        outlen as size_t,
    );
}
/* Handle the OSC 104 sequence for unsetting (multiple) palette entries. */
unsafe extern "C" fn input_osc_104(mut ictx: *mut input_ctx, mut p: *const libc::c_char) {
    let mut current_block: u64;
    let mut wp: *mut window_pane = (*ictx).wp;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_long = 0;
    if wp.is_null() {
        return;
    }
    if *p as libc::c_int == '\u{0}' as i32 {
        window_pane_reset_palette(wp);
        return;
    }
    s = xstrdup(p);
    copy = s;
    loop {
        if !(*s as libc::c_int != '\u{0}' as i32) {
            current_block = 8236137900636309791;
            break;
        }
        idx = strtol(s, &mut s, 10i32);
        if *s as libc::c_int != '\u{0}' as i32 && *s as libc::c_int != ';' as i32 {
            current_block = 13905477316042032337;
            break;
        }
        if idx < 0i64 || idx >= 0x100i64 {
            current_block = 13905477316042032337;
            break;
        }
        window_pane_unset_palette(wp, idx as u_int);
        if *s as libc::c_int == ';' as i32 {
            s = s.offset(1)
        }
    }
    match current_block {
        13905477316042032337 => {
            log_debug(
                b"bad OSC 104: %s\x00" as *const u8 as *const libc::c_char,
                p,
            );
            free(copy as *mut libc::c_void);
            return;
        }
        _ => {
            free(copy as *mut libc::c_void);
            return;
        }
    };
}
