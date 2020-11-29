use crate::{
    grid::Cell as GridCell,
    style::{
        align as style_align, default_type as style_default_type, list as style_list,
        range_type as style_range_type, Style,
    },
    utf8::Utf8Data,
};
use ::libc;

extern "C" {
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
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
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
    static mut global_options: *mut options;
    #[no_mangle]
    static mut global_s_options: *mut options;
    #[no_mangle]
    static mut global_w_options: *mut options;
    #[no_mangle]
    fn checkshell(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn format_expand(
        _: *mut crate::format::format_tree,
        _: *const libc::c_char,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list);
    #[no_mangle]
    static options_other_names: [options_name_map; 0];
    #[no_mangle]
    fn cmd_parse_from_string(
        _: *const libc::c_char,
        _: *mut cmd_parse_input,
    ) -> *mut cmd_parse_result;
    #[no_mangle]
    fn colour_tostring(_: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_list_print(_: *mut cmd_list, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    static grid_default_cell: crate::grid::Cell;
    #[no_mangle]
    fn colour_fromstring(s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn key_string_lookup_string(_: *const libc::c_char) -> key_code;
    #[no_mangle]
    fn server_redraw_client(_: *mut client);
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    fn recalculate_sizes();
    #[no_mangle]
    fn status_update_cache(_: *mut session);
    #[no_mangle]
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    #[no_mangle]
    fn window_pane_tree_RB_NEXT(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn window_pane_tree_RB_MINMAX(_: *mut window_pane_tree, _: libc::c_int) -> *mut window_pane;
    #[no_mangle]
    fn alerts_reset_all();
    #[no_mangle]
    fn status_timer_start_all();
    #[no_mangle]
    fn tty_keys_build(_: *mut tty);
    #[no_mangle]
    fn server_client_set_key_table(_: *mut client, _: *const libc::c_char);
    #[no_mangle]
    fn layout_fix_panes(_: *mut window);
    #[no_mangle]
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn style_parse(
        _: *mut crate::style::Style,
        _: *const crate::grid::Cell,
        _: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn style_set(_: *mut crate::style::Style, _: *const crate::grid::Cell);
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
    pub grid: *mut crate::grid::Grid,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub rupper: u_int,
    pub rlower: u_int,
    pub mode: libc::c_int,
    pub saved_cx: u_int,
    pub saved_cy: u_int,
    pub saved_grid: *mut crate::grid::Grid,
    pub saved_cell: crate::grid::Cell,
    pub saved_flags: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: *mut crate::screen::screen_sel,
    pub write_list: *mut crate::screen_write::screen_write_collect_line,
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
    pub options: *mut options,
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
pub struct options {
    pub tree: options_tree,
    pub parent: *mut options,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct options_tree {
    pub rbh_root: *mut options_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct options_entry {
    pub owner: *mut options,
    pub name: *const libc::c_char,
    pub tableentry: *const options_table_entry,
    pub value: options_value,
    pub cached: libc::c_int,
    pub style: crate::style::Style,
    pub entry: C2RustUnnamed_15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_15 {
    pub rbe_left: *mut options_entry,
    pub rbe_right: *mut options_entry,
    pub rbe_parent: *mut options_entry,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union options_value {
    pub string: *mut libc::c_char,
    pub number: libc::c_longlong,
    pub style: crate::style::Style,
    pub array: options_array,
    pub cmdlist: *mut cmd_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_list {
    pub references: libc::c_int,
    pub group: u_int,
    pub list: *mut crate::cmd::cmds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct options_array {
    pub rbh_root: *mut options_array_item,
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2008 Nicholas Marriott <nicholas.marriott@gmail.com>
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
 * Option handling; each option has a name, type and value and is stored in
 * a red-black tree.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct options_array_item {
    pub index: u_int,
    pub value: options_value,
    pub entry: C2RustUnnamed_16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_16 {
    pub rbe_left: *mut options_array_item,
    pub rbe_right: *mut options_array_item,
    pub rbe_parent: *mut options_array_item,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
pub type options_table_type = libc::c_uint;
pub const OPTIONS_TABLE_COMMAND: options_table_type = 6;
pub const OPTIONS_TABLE_CHOICE: options_table_type = 5;
pub const OPTIONS_TABLE_FLAG: options_table_type = 4;
pub const OPTIONS_TABLE_COLOUR: options_table_type = 3;
pub const OPTIONS_TABLE_KEY: options_table_type = 2;
pub const OPTIONS_TABLE_NUMBER: options_table_type = 1;
pub const OPTIONS_TABLE_STRING: options_table_type = 0;

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
    pub entry: C2RustUnnamed_19,
    pub wentry: C2RustUnnamed_18,
    pub sentry: C2RustUnnamed_17,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_17 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_18 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_19 {
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
    pub alerts_entry: C2RustUnnamed_22,
    pub options: *mut options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_21,
    pub entry: C2RustUnnamed_20,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_20 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_21 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_22 {
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
    pub entry: C2RustUnnamed_23,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_23 {
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
    pub modes: C2RustUnnamed_26,
    pub searchstr: *mut libc::c_char,
    pub searchregex: libc::c_int,
    pub written: size_t,
    pub skipped: size_t,
    pub border_gc_set: libc::c_int,
    pub border_gc: crate::grid::Cell,
    pub entry: C2RustUnnamed_25,
    pub tree_entry: C2RustUnnamed_24,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_24 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_25 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_26 {
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
    pub entry: C2RustUnnamed_27,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_27 {
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
    pub ranges: crate::style::Ranges,
}

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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct window_pane_tree {
    pub rbh_root: *mut window_pane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct windows {
    pub rbh_root: *mut window,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sessions {
    pub rbh_root: *mut session,
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
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct options_name_map {
    pub from: *const libc::c_char,
    pub to: *const libc::c_char,
}
unsafe extern "C" fn options_array_cmp(
    mut a1: *mut options_array_item,
    mut a2: *mut options_array_item,
) -> libc::c_int {
    if (*a1).index < (*a2).index {
        return -(1i32);
    }
    if (*a1).index > (*a2).index {
        return 1i32;
    }
    return 0i32;
}
unsafe extern "C" fn options_array_RB_MINMAX(
    mut head: *mut options_array,
    mut val: libc::c_int,
) -> *mut options_array_item {
    let mut tmp: *mut options_array_item = (*head).rbh_root;
    let mut parent: *mut options_array_item = 0 as *mut options_array_item;
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
unsafe extern "C" fn options_array_RB_NEXT(
    mut elm: *mut options_array_item,
) -> *mut options_array_item {
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
unsafe extern "C" fn options_array_RB_REMOVE_COLOR(
    mut head: *mut options_array,
    mut parent: *mut options_array_item,
    mut elm: *mut options_array_item,
) {
    let mut tmp: *mut options_array_item = 0 as *mut options_array_item;
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
                    let mut oleft: *mut options_array_item = 0 as *mut options_array_item;
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
                    let mut oright: *mut options_array_item = 0 as *mut options_array_item;
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
unsafe extern "C" fn options_array_RB_REMOVE(
    mut head: *mut options_array,
    mut elm: *mut options_array_item,
) -> *mut options_array_item {
    let mut current_block: u64;
    let mut child: *mut options_array_item = 0 as *mut options_array_item;
    let mut parent: *mut options_array_item = 0 as *mut options_array_item;
    let mut old: *mut options_array_item = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut options_array_item = 0 as *mut options_array_item;
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
        current_block = 7114279629446095448;
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
        options_array_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn options_array_RB_INSERT(
    mut head: *mut options_array,
    mut elm: *mut options_array_item,
) -> *mut options_array_item {
    let mut tmp: *mut options_array_item = 0 as *mut options_array_item;
    let mut parent: *mut options_array_item = 0 as *mut options_array_item;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = options_array_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut options_array_item;
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
    options_array_RB_INSERT_COLOR(head, elm);
    return 0 as *mut options_array_item;
}
unsafe extern "C" fn options_array_RB_FIND(
    mut head: *mut options_array,
    mut elm: *mut options_array_item,
) -> *mut options_array_item {
    let mut tmp: *mut options_array_item = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = options_array_cmp(elm, tmp);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut options_array_item;
}
unsafe extern "C" fn options_array_RB_INSERT_COLOR(
    mut head: *mut options_array,
    mut elm: *mut options_array_item,
) {
    let mut parent: *mut options_array_item = 0 as *mut options_array_item;
    let mut gparent: *mut options_array_item = 0 as *mut options_array_item;
    let mut tmp: *mut options_array_item = 0 as *mut options_array_item;
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
unsafe extern "C" fn options_tree_RB_MINMAX(
    mut head: *mut options_tree,
    mut val: libc::c_int,
) -> *mut options_entry {
    let mut tmp: *mut options_entry = (*head).rbh_root;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
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
unsafe extern "C" fn options_tree_RB_FIND(
    mut head: *mut options_tree,
    mut elm: *mut options_entry,
) -> *mut options_entry {
    let mut tmp: *mut options_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = options_cmp(elm, tmp);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut options_entry;
}
unsafe extern "C" fn options_tree_RB_INSERT_COLOR(
    mut head: *mut options_tree,
    mut elm: *mut options_entry,
) {
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut gparent: *mut options_entry = 0 as *mut options_entry;
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
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
unsafe extern "C" fn options_tree_RB_INSERT(
    mut head: *mut options_tree,
    mut elm: *mut options_entry,
) -> *mut options_entry {
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut comp: libc::c_int = 0i32;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = options_cmp(elm, parent);
        if comp < 0i32 {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0i32 {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut options_entry;
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
    options_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut options_entry;
}
unsafe extern "C" fn options_tree_RB_REMOVE(
    mut head: *mut options_tree,
    mut elm: *mut options_entry,
) -> *mut options_entry {
    let mut current_block: u64;
    let mut child: *mut options_entry = 0 as *mut options_entry;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut old: *mut options_entry = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut options_entry = 0 as *mut options_entry;
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
        current_block = 9245580858887543803;
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
        options_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn options_tree_RB_REMOVE_COLOR(
    mut head: *mut options_tree,
    mut parent: *mut options_entry,
    mut elm: *mut options_entry,
) {
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
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
                    let mut oleft: *mut options_entry = 0 as *mut options_entry;
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
                    let mut oright: *mut options_entry = 0 as *mut options_entry;
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
unsafe extern "C" fn options_tree_RB_NEXT(mut elm: *mut options_entry) -> *mut options_entry {
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
unsafe extern "C" fn options_cmp(
    mut lhs: *mut options_entry,
    mut rhs: *mut options_entry,
) -> libc::c_int {
    return strcmp((*lhs).name, (*rhs).name);
}
unsafe extern "C" fn options_map_name(mut name: *const libc::c_char) -> *const libc::c_char {
    let mut map: *const options_name_map = 0 as *const options_name_map;
    map = options_other_names.as_ptr();
    while !(*map).from.is_null() {
        if strcmp((*map).from, name) == 0i32 {
            return (*map).to;
        }
        map = map.offset(1)
    }
    return name;
}
unsafe extern "C" fn options_parent_table_entry(
    mut oo: *mut options,
    mut s: *const libc::c_char,
) -> *const options_table_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    if (*oo).parent.is_null() {
        fatalx(
            b"no parent options for %s\x00" as *const u8 as *const libc::c_char,
            s,
        );
    }
    o = options_get((*oo).parent, s);
    if o.is_null() {
        fatalx(
            b"%s not in parent options\x00" as *const u8 as *const libc::c_char,
            s,
        );
    }
    return (*o).tableentry;
}
unsafe extern "C" fn options_value_free(mut o: *mut options_entry, mut ov: *mut options_value) {
    if (*o).tableentry.is_null() || (*(*o).tableentry).type_0 == OPTIONS_TABLE_STRING {
        free((*ov).string as *mut libc::c_void);
    }
    if !(*o).tableentry.is_null()
        && (*(*o).tableentry).type_0 == OPTIONS_TABLE_COMMAND
        && !(*ov).cmdlist.is_null()
    {
        cmd_list_free((*ov).cmdlist);
    };
}
unsafe extern "C" fn options_value_to_string(
    mut o: *mut options_entry,
    mut ov: *mut options_value,
    mut numeric: libc::c_int,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*o).tableentry.is_null() && (*(*o).tableentry).type_0 == OPTIONS_TABLE_COMMAND {
        return cmd_list_print((*ov).cmdlist, 0i32);
    }
    if !(*o).tableentry.is_null()
        && ((*(*o).tableentry).type_0 == OPTIONS_TABLE_NUMBER
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_KEY
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_COLOUR
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_FLAG
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_CHOICE)
    {
        match (*(*o).tableentry).type_0 {
            1 => {
                xasprintf(
                    &mut s as *mut *mut libc::c_char,
                    b"%lld\x00" as *const u8 as *const libc::c_char,
                    (*ov).number,
                );
            }
            2 => s = xstrdup(key_string_lookup_key((*ov).number as key_code, 0i32)),
            3 => s = xstrdup(colour_tostring((*ov).number as libc::c_int)),
            4 => {
                if numeric != 0 {
                    xasprintf(
                        &mut s as *mut *mut libc::c_char,
                        b"%lld\x00" as *const u8 as *const libc::c_char,
                        (*ov).number,
                    );
                } else {
                    s = xstrdup(if (*ov).number != 0 {
                        b"on\x00" as *const u8 as *const libc::c_char
                    } else {
                        b"off\x00" as *const u8 as *const libc::c_char
                    })
                }
            }
            5 => s = xstrdup(*(*(*o).tableentry).choices.offset((*ov).number as isize)),
            0 | 6 => {
                fatalx(b"not a number option type\x00" as *const u8 as *const libc::c_char);
            }
            _ => {}
        }
        return s;
    }
    if (*o).tableentry.is_null() || (*(*o).tableentry).type_0 == OPTIONS_TABLE_STRING {
        return xstrdup((*ov).string);
    }
    return xstrdup(b"\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn options_create(mut parent: *mut options) -> *mut options {
    let mut oo: *mut options = 0 as *mut options;
    oo = xcalloc(1u64, ::std::mem::size_of::<options>() as libc::c_ulong) as *mut options;
    (*oo).tree.rbh_root = 0 as *mut options_entry;
    (*oo).parent = parent;
    return oo;
}
#[no_mangle]
pub unsafe extern "C" fn options_free(mut oo: *mut options) {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut tmp: *mut options_entry = 0 as *mut options_entry;
    o = options_tree_RB_MINMAX(&mut (*oo).tree, -(1i32));
    while !o.is_null() && {
        tmp = options_tree_RB_NEXT(o);
        (1i32) != 0
    } {
        options_remove(o);
        o = tmp
    }
    free(oo as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn options_get_parent(mut oo: *mut options) -> *mut options {
    return (*oo).parent;
}
#[no_mangle]
pub unsafe extern "C" fn options_set_parent(mut oo: *mut options, mut parent: *mut options) {
    (*oo).parent = parent;
}
#[no_mangle]
pub unsafe extern "C" fn options_first(mut oo: *mut options) -> *mut options_entry {
    return options_tree_RB_MINMAX(&mut (*oo).tree, -(1i32));
}
#[no_mangle]
pub unsafe extern "C" fn options_next(mut o: *mut options_entry) -> *mut options_entry {
    return options_tree_RB_NEXT(o);
}
#[no_mangle]
pub unsafe extern "C" fn options_get_only(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> *mut options_entry {
    let mut o: options_entry = {
        let mut init = options_entry {
            owner: 0 as *mut options,
            name: name,
            tableentry: 0 as *const options_table_entry,
            value: options_value {
                string: 0 as *mut libc::c_char,
            },
            cached: 0,
            style: Style {
                gc: GridCell {
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
                ignore: 0,
                fill: 0,
                align: style_align::DEFAULT,
                list: style_list::OFF,
                range_type: style_range_type::NONE,
                range_argument: 0,
                default_type: style_default_type::BASE,
            },
            entry: C2RustUnnamed_15 {
                rbe_left: 0 as *mut options_entry,
                rbe_right: 0 as *mut options_entry,
                rbe_parent: 0 as *mut options_entry,
                rbe_color: 0,
            },
        };
        init
    };
    let mut found: *mut options_entry = 0 as *mut options_entry;
    found = options_tree_RB_FIND(&mut (*oo).tree, &mut o);
    if found.is_null() {
        o.name = options_map_name(name);
        return options_tree_RB_FIND(&mut (*oo).tree, &mut o);
    }
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn options_get(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get_only(oo, name);
    while o.is_null() {
        oo = (*oo).parent;
        if oo.is_null() {
            break;
        }
        o = options_get_only(oo, name)
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_empty(
    mut oo: *mut options,
    mut oe: *const options_table_entry,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_add(oo, (*oe).name);
    (*o).tableentry = oe;
    if (*oe).flags & 0x1i32 != 0 {
        (*o).value.array.rbh_root = 0 as *mut options_array_item
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_default(
    mut oo: *mut options,
    mut oe: *const options_table_entry,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut ov: *mut options_value = 0 as *mut options_value;
    let mut i: u_int = 0;
    o = options_empty(oo, oe);
    ov = &mut (*o).value;
    if (*oe).flags & 0x1i32 != 0 {
        if (*oe).default_arr.is_null() {
            options_array_assign(o, (*oe).default_str, 0 as *mut *mut libc::c_char);
            return o;
        }
        i = 0u32;
        while !(*(*oe).default_arr.offset(i as isize)).is_null() {
            options_array_set(
                o,
                i,
                *(*oe).default_arr.offset(i as isize),
                0i32,
                0 as *mut *mut libc::c_char,
            );
            i = i.wrapping_add(1)
        }
        return o;
    }
    match (*oe).type_0 {
        0 => (*ov).string = xstrdup((*oe).default_str),
        _ => (*ov).number = (*oe).default_num,
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_default_to_string(
    mut oe: *const options_table_entry,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    match (*oe).type_0 {
        0 | 6 => s = xstrdup((*oe).default_str),
        1 => {
            xasprintf(
                &mut s as *mut *mut libc::c_char,
                b"%lld\x00" as *const u8 as *const libc::c_char,
                (*oe).default_num,
            );
        }
        2 => s = xstrdup(key_string_lookup_key((*oe).default_num as key_code, 0i32)),
        3 => s = xstrdup(colour_tostring((*oe).default_num as libc::c_int)),
        4 => {
            s = xstrdup(if (*oe).default_num != 0 {
                b"on\x00" as *const u8 as *const libc::c_char
            } else {
                b"off\x00" as *const u8 as *const libc::c_char
            })
        }
        5 => s = xstrdup(*(*oe).choices.offset((*oe).default_num as isize)),
        _ => {}
    }
    return s;
}
unsafe extern "C" fn options_add(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get_only(oo, name);
    if !o.is_null() {
        options_remove(o);
    }
    o = xcalloc(
        1u64,
        ::std::mem::size_of::<options_entry>() as libc::c_ulong,
    ) as *mut options_entry;
    (*o).owner = oo;
    (*o).name = xstrdup(name);
    options_tree_RB_INSERT(&mut (*oo).tree, o);
    return o;
}
unsafe extern "C" fn options_remove(mut o: *mut options_entry) {
    let mut oo: *mut options = (*o).owner;
    if !(*o).tableentry.is_null() && (*(*o).tableentry).flags & 0x1i32 != 0 {
        options_array_clear(o);
    } else {
        options_value_free(o, &mut (*o).value);
    }
    options_tree_RB_REMOVE(&mut (*oo).tree, o);
    free((*o).name as *mut libc::c_void);
    free(o as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn options_name(mut o: *mut options_entry) -> *const libc::c_char {
    return (*o).name;
}
#[no_mangle]
pub unsafe extern "C" fn options_owner(mut o: *mut options_entry) -> *mut options {
    return (*o).owner;
}
#[no_mangle]
pub unsafe extern "C" fn options_table_entry(
    mut o: *mut options_entry,
) -> *const options_table_entry {
    return (*o).tableentry;
}
unsafe extern "C" fn options_array_item(
    mut o: *mut options_entry,
    mut idx: u_int,
) -> *mut options_array_item {
    let mut a: options_array_item = options_array_item {
        index: 0,
        value: options_value {
            string: 0 as *mut libc::c_char,
        },
        entry: C2RustUnnamed_16 {
            rbe_left: 0 as *mut options_array_item,
            rbe_right: 0 as *mut options_array_item,
            rbe_parent: 0 as *mut options_array_item,
            rbe_color: 0,
        },
    };
    a.index = idx;
    return options_array_RB_FIND(&mut (*o).value.array, &mut a);
}
unsafe extern "C" fn options_array_new(
    mut o: *mut options_entry,
    mut idx: u_int,
) -> *mut options_array_item {
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    a = xcalloc(
        1u64,
        ::std::mem::size_of::<options_array_item>() as libc::c_ulong,
    ) as *mut options_array_item;
    (*a).index = idx;
    options_array_RB_INSERT(&mut (*o).value.array, a);
    return a;
}
unsafe extern "C" fn options_array_free(mut o: *mut options_entry, mut a: *mut options_array_item) {
    options_value_free(o, &mut (*a).value);
    options_array_RB_REMOVE(&mut (*o).value.array, a);
    free(a as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn options_array_clear(mut o: *mut options_entry) {
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    let mut a1: *mut options_array_item = 0 as *mut options_array_item;
    if !(!(*o).tableentry.is_null() && (*(*o).tableentry).flags & 0x1i32 != 0) {
        return;
    }
    a = options_array_RB_MINMAX(&mut (*o).value.array, -(1i32));
    while !a.is_null() && {
        a1 = options_array_RB_NEXT(a);
        (1i32) != 0
    } {
        options_array_free(o, a);
        a = a1
    }
}
#[no_mangle]
pub unsafe extern "C" fn options_array_get(
    mut o: *mut options_entry,
    mut idx: u_int,
) -> *mut options_value {
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    if !(!(*o).tableentry.is_null() && (*(*o).tableentry).flags & 0x1i32 != 0) {
        return 0 as *mut options_value;
    }
    a = options_array_item(o, idx);
    if a.is_null() {
        return 0 as *mut options_value;
    }
    return &mut (*a).value;
}
#[no_mangle]
pub unsafe extern "C" fn options_array_set(
    mut o: *mut options_entry,
    mut idx: u_int,
    mut value: *const libc::c_char,
    mut append: libc::c_int,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pr: *mut cmd_parse_result = 0 as *mut cmd_parse_result;
    if !(!(*o).tableentry.is_null() && (*(*o).tableentry).flags & 0x1i32 != 0) {
        if !cause.is_null() {
            *cause = xstrdup(b"not an array\x00" as *const u8 as *const libc::c_char)
        }
        return -(1i32);
    }
    if value.is_null() {
        a = options_array_item(o, idx);
        if !a.is_null() {
            options_array_free(o, a);
        }
        return 0i32;
    }
    if !(*o).tableentry.is_null() && (*(*o).tableentry).type_0 == OPTIONS_TABLE_COMMAND {
        pr = cmd_parse_from_string(value, 0 as *mut cmd_parse_input);
        match (*pr).status {
            0 => {
                if !cause.is_null() {
                    *cause = xstrdup(b"empty command\x00" as *const u8 as *const libc::c_char)
                }
                return -(1i32);
            }
            1 => {
                if !cause.is_null() {
                    *cause = (*pr).error
                } else {
                    free((*pr).error as *mut libc::c_void);
                }
                return -(1i32);
            }
            2 | _ => {}
        }
        a = options_array_item(o, idx);
        if a.is_null() {
            a = options_array_new(o, idx)
        } else {
            options_value_free(o, &mut (*a).value);
        }
        (*a).value.cmdlist = (*pr).cmdlist;
        return 0i32;
    }
    if (*o).tableentry.is_null() || (*(*o).tableentry).type_0 == OPTIONS_TABLE_STRING {
        a = options_array_item(o, idx);
        if !a.is_null() && append != 0 {
            xasprintf(
                &mut new as *mut *mut libc::c_char,
                b"%s%s\x00" as *const u8 as *const libc::c_char,
                (*a).value.string,
                value,
            );
        } else {
            new = xstrdup(value)
        }
        if a.is_null() {
            a = options_array_new(o, idx)
        } else {
            options_value_free(o, &mut (*a).value);
        }
        (*a).value.string = new;
        return 0i32;
    }
    if !cause.is_null() {
        *cause = xstrdup(b"wrong array type\x00" as *const u8 as *const libc::c_char)
    }
    return -(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn options_array_assign(
    mut o: *mut options_entry,
    mut s: *const libc::c_char,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut separator: *const libc::c_char = 0 as *const libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: u_int = 0;
    separator = (*(*o).tableentry).separator;
    if separator.is_null() {
        separator = b" ,\x00" as *const u8 as *const libc::c_char
    }
    if *separator as libc::c_int == '\u{0}' as i32 {
        if *s as libc::c_int == '\u{0}' as i32 {
            return 0i32;
        }
        i = 0u32;
        while i < (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32) {
            if options_array_item(o, i).is_null() {
                break;
            }
            i = i.wrapping_add(1)
        }
        return options_array_set(o, i, s, 0i32, cause);
    }
    if *s as libc::c_int == '\u{0}' as i32 {
        return 0i32;
    }
    string = xstrdup(s);
    copy = string;
    loop {
        next = strsep(&mut string, separator);
        if next.is_null() {
            break;
        }
        if *next as libc::c_int == '\u{0}' as i32 {
            continue;
        }
        i = 0u32;
        while i < (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32) {
            if options_array_item(o, i).is_null() {
                break;
            }
            i = i.wrapping_add(1)
        }
        if i == (2147483647u32).wrapping_mul(2u32).wrapping_add(1u32) {
            break;
        }
        if options_array_set(o, i, next, 0i32, cause) != 0i32 {
            free(copy as *mut libc::c_void);
            return -(1i32);
        }
    }
    free(copy as *mut libc::c_void);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn options_array_first(mut o: *mut options_entry) -> *mut options_array_item {
    if !(!(*o).tableentry.is_null() && (*(*o).tableentry).flags & 0x1i32 != 0) {
        return 0 as *mut options_array_item;
    }
    return options_array_RB_MINMAX(&mut (*o).value.array, -(1i32));
}
#[no_mangle]
pub unsafe extern "C" fn options_array_next(
    mut a: *mut options_array_item,
) -> *mut options_array_item {
    return options_array_RB_NEXT(a);
}
#[no_mangle]
pub unsafe extern "C" fn options_array_item_index(mut a: *mut options_array_item) -> u_int {
    return (*a).index;
}
#[no_mangle]
pub unsafe extern "C" fn options_array_item_value(
    mut a: *mut options_array_item,
) -> *mut options_value {
    return &mut (*a).value;
}
#[no_mangle]
pub unsafe extern "C" fn options_is_array(mut o: *mut options_entry) -> libc::c_int {
    return (!(*o).tableentry.is_null() && (*(*o).tableentry).flags & 0x1i32 != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn options_is_string(mut o: *mut options_entry) -> libc::c_int {
    return ((*o).tableentry.is_null() || (*(*o).tableentry).type_0 == OPTIONS_TABLE_STRING)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn options_to_string(
    mut o: *mut options_entry,
    mut idx: libc::c_int,
    mut numeric: libc::c_int,
) -> *mut libc::c_char {
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    if !(*o).tableentry.is_null() && (*(*o).tableentry).flags & 0x1i32 != 0 {
        if idx == -(1i32) {
            return xstrdup(b"\x00" as *const u8 as *const libc::c_char);
        }
        a = options_array_item(o, idx as u_int);
        if a.is_null() {
            return xstrdup(b"\x00" as *const u8 as *const libc::c_char);
        }
        return options_value_to_string(o, &mut (*a).value, numeric);
    }
    return options_value_to_string(o, &mut (*o).value, numeric);
}
#[no_mangle]
pub unsafe extern "C" fn options_parse(
    mut name: *const libc::c_char,
    mut idx: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if *name as libc::c_int == '\u{0}' as i32 {
        return 0 as *mut libc::c_char;
    }
    copy = xstrdup(name);
    cp = strchr(copy, '[' as i32);
    if cp.is_null() {
        *idx = -(1i32);
        return copy;
    }
    end = strchr(cp.offset(1isize), ']' as i32);
    if end.is_null()
        || *end.offset(1isize) as libc::c_int != '\u{0}' as i32
        || *(*__ctype_b_loc()).offset(*end.offset(-1isize) as u_char as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_ushort as libc::c_int
            == 0
    {
        free(copy as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    if sscanf(cp, b"[%d]\x00" as *const u8 as *const libc::c_char, idx) != 1i32 || *idx < 0i32 {
        free(copy as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    *cp = '\u{0}' as libc::c_char;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn options_parse_get(
    mut oo: *mut options,
    mut s: *const libc::c_char,
    mut idx: *mut libc::c_int,
    mut only: libc::c_int,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = options_parse(s, idx);
    if name.is_null() {
        return 0 as *mut options_entry;
    }
    if only != 0 {
        o = options_get_only(oo, name)
    } else {
        o = options_get(oo, name)
    }
    free(name as *mut libc::c_void);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_match(
    mut s: *const libc::c_char,
    mut idx: *mut libc::c_int,
    mut ambiguous: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    let mut found: *const options_table_entry = 0 as *const options_table_entry;
    let mut parsed: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut namelen: size_t = 0;
    parsed = options_parse(s, idx);
    if parsed.is_null() {
        return 0 as *mut libc::c_char;
    }
    if *parsed as libc::c_int == '@' as i32 {
        *ambiguous = 0i32;
        return parsed;
    }
    name = options_map_name(parsed);
    namelen = strlen(name);
    found = 0 as *const options_table_entry;
    oe = options_table.as_ptr();
    while !(*oe).name.is_null() {
        if strcmp((*oe).name, name) == 0i32 {
            found = oe;
            break;
        } else {
            if strncmp((*oe).name, name, namelen) == 0i32 {
                if !found.is_null() {
                    *ambiguous = 1i32;
                    free(parsed as *mut libc::c_void);
                    return 0 as *mut libc::c_char;
                }
                found = oe
            }
            oe = oe.offset(1)
        }
    }
    free(parsed as *mut libc::c_void);
    if found.is_null() {
        *ambiguous = 0i32;
        return 0 as *mut libc::c_char;
    }
    return xstrdup((*found).name);
}
#[no_mangle]
pub unsafe extern "C" fn options_match_get(
    mut oo: *mut options,
    mut s: *const libc::c_char,
    mut idx: *mut libc::c_int,
    mut only: libc::c_int,
    mut ambiguous: *mut libc::c_int,
) -> *mut options_entry {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    name = options_match(s, idx, ambiguous);
    if name.is_null() {
        return 0 as *mut options_entry;
    }
    *ambiguous = 0i32;
    if only != 0 {
        o = options_get_only(oo, name)
    } else {
        o = options_get(oo, name)
    }
    free(name as *mut libc::c_void);
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_get_string(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> *const libc::c_char {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get(oo, name);
    if o.is_null() {
        fatalx(
            b"missing option %s\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    if !((*o).tableentry.is_null() || (*(*o).tableentry).type_0 == OPTIONS_TABLE_STRING) {
        fatalx(
            b"option %s is not a string\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    return (*o).value.string;
}
#[no_mangle]
pub unsafe extern "C" fn options_get_number(
    mut oo: *mut options,
    mut name: *const libc::c_char,
) -> libc::c_longlong {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    o = options_get(oo, name);
    if o.is_null() {
        fatalx(
            b"missing option %s\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    if !(!(*o).tableentry.is_null()
        && ((*(*o).tableentry).type_0 == OPTIONS_TABLE_NUMBER
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_KEY
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_COLOUR
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_FLAG
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_CHOICE))
    {
        fatalx(
            b"option %s is not a number\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    return (*o).value.number;
}
#[no_mangle]
pub unsafe extern "C" fn options_set_string(
    mut oo: *mut options,
    mut name: *const libc::c_char,
    mut append: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut ap: ::std::ffi::VaListImpl;
    let mut separator: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    xvasprintf(&mut s, fmt, ap.as_va_list());
    o = options_get_only(oo, name);
    if !o.is_null()
        && append != 0
        && ((*o).tableentry.is_null() || (*(*o).tableentry).type_0 == OPTIONS_TABLE_STRING)
    {
        if *name as libc::c_int != '@' as i32 {
            separator = (*(*o).tableentry).separator;
            if separator.is_null() {
                separator = b"\x00" as *const u8 as *const libc::c_char
            }
        }
        xasprintf(
            &mut value as *mut *mut libc::c_char,
            b"%s%s%s\x00" as *const u8 as *const libc::c_char,
            (*o).value.string,
            separator,
            s,
        );
        free(s as *mut libc::c_void);
    } else {
        value = s
    }
    if o.is_null() && *name as libc::c_int == '@' as i32 {
        o = options_add(oo, name)
    } else if o.is_null() {
        o = options_default(oo, options_parent_table_entry(oo, name));
        if o.is_null() {
            return 0 as *mut options_entry;
        }
    }
    if !((*o).tableentry.is_null() || (*(*o).tableentry).type_0 == OPTIONS_TABLE_STRING) {
        fatalx(
            b"option %s is not a string\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    free((*o).value.string as *mut libc::c_void);
    (*o).value.string = value;
    (*o).cached = 0i32;
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_set_number(
    mut oo: *mut options,
    mut name: *const libc::c_char,
    mut value: libc::c_longlong,
) -> *mut options_entry {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    if *name as libc::c_int == '@' as i32 {
        fatalx(
            b"user option %s must be a string\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    o = options_get_only(oo, name);
    if o.is_null() {
        o = options_default(oo, options_parent_table_entry(oo, name));
        if o.is_null() {
            return 0 as *mut options_entry;
        }
    }
    if !(!(*o).tableentry.is_null()
        && ((*(*o).tableentry).type_0 == OPTIONS_TABLE_NUMBER
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_KEY
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_COLOUR
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_FLAG
            || (*(*o).tableentry).type_0 == OPTIONS_TABLE_CHOICE))
    {
        fatalx(
            b"option %s is not a number\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    (*o).value.number = value;
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn options_scope_from_name(
    mut args: *mut args,
    mut window: libc::c_int,
    mut name: *const libc::c_char,
    mut fs: *mut cmd_find_state,
    mut oo: *mut *mut options,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut s: *mut session = (*fs).s;
    let mut wl: *mut winlink = (*fs).wl;
    let mut wp: *mut window_pane = (*fs).wp;
    let mut target: *const libc::c_char = args_get(args, 't' as u_char);
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    let mut scope: libc::c_int = 0i32;
    if *name as libc::c_int == '@' as i32 {
        return options_scope_from_flags(args, window, fs, oo, cause);
    }
    oe = options_table.as_ptr();
    while !(*oe).name.is_null() {
        if strcmp((*oe).name, name) == 0i32 {
            break;
        }
        oe = oe.offset(1)
    }
    if (*oe).name.is_null() {
        xasprintf(
            cause,
            b"unknown option: %s\x00" as *const u8 as *const libc::c_char,
            name,
        );
        return 0i32;
    }
    let mut current_block_38: u64;
    match (*oe).scope {
        1 => {
            *oo = global_options;
            scope = 0x1i32;
            current_block_38 = 6717214610478484138;
        }
        2 => {
            if args_has(args, 'g' as u_char) != 0 {
                *oo = global_s_options;
                scope = 0x2i32
            } else if s.is_null() && !target.is_null() {
                xasprintf(
                    cause,
                    b"no such session: %s\x00" as *const u8 as *const libc::c_char,
                    target,
                );
            } else if s.is_null() {
                xasprintf(
                    cause,
                    b"no current session\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                *oo = (*s).options;
                scope = 0x2i32
            }
            current_block_38 = 6717214610478484138;
        }
        12 => {
            if args_has(args, 'p' as u_char) != 0 {
                if wp.is_null() && !target.is_null() {
                    xasprintf(
                        cause,
                        b"no such pane: %s\x00" as *const u8 as *const libc::c_char,
                        target,
                    );
                } else if wp.is_null() {
                    xasprintf(
                        cause,
                        b"no current pane\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                    *oo = (*wp).options;
                    scope = 0x8i32
                }
                current_block_38 = 6717214610478484138;
            } else {
                current_block_38 = 15958660329489593307;
            }
        }
        4 => {
            current_block_38 = 15958660329489593307;
        }
        _ => {
            current_block_38 = 6717214610478484138;
        }
    }
    match current_block_38 {
        15958660329489593307 =>
        /* FALLTHROUGH */
        {
            if args_has(args, 'g' as u_char) != 0 {
                *oo = global_w_options;
                scope = 0x4i32
            } else if wl.is_null() && !target.is_null() {
                xasprintf(
                    cause,
                    b"no such window: %s\x00" as *const u8 as *const libc::c_char,
                    target,
                );
            } else if wl.is_null() {
                xasprintf(
                    cause,
                    b"no current window\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                *oo = (*(*wl).window).options;
                scope = 0x4i32
            }
        }
        _ => {}
    }
    return scope;
}
#[no_mangle]
pub unsafe extern "C" fn options_scope_from_flags(
    mut args: *mut args,
    mut window: libc::c_int,
    mut fs: *mut cmd_find_state,
    mut oo: *mut *mut options,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut s: *mut session = (*fs).s;
    let mut wl: *mut winlink = (*fs).wl;
    let mut wp: *mut window_pane = (*fs).wp;
    let mut target: *const libc::c_char = args_get(args, 't' as u_char);
    if args_has(args, 's' as u_char) != 0 {
        *oo = global_options;
        return 0x1i32;
    }
    if args_has(args, 'p' as u_char) != 0 {
        if wp.is_null() {
            if !target.is_null() {
                xasprintf(
                    cause,
                    b"no such pane: %s\x00" as *const u8 as *const libc::c_char,
                    target,
                );
            } else {
                xasprintf(
                    cause,
                    b"no current pane\x00" as *const u8 as *const libc::c_char,
                );
            }
            return 0i32;
        }
        *oo = (*wp).options;
        return 0x8i32;
    } else if window != 0 || args_has(args, 'w' as u_char) != 0 {
        if args_has(args, 'g' as u_char) != 0 {
            *oo = global_w_options;
            return 0x4i32;
        }
        if wl.is_null() {
            if !target.is_null() {
                xasprintf(
                    cause,
                    b"no such window: %s\x00" as *const u8 as *const libc::c_char,
                    target,
                );
            } else {
                xasprintf(
                    cause,
                    b"no current window\x00" as *const u8 as *const libc::c_char,
                );
            }
            return 0i32;
        }
        *oo = (*(*wl).window).options;
        return 0x4i32;
    } else {
        if args_has(args, 'g' as u_char) != 0 {
            *oo = global_s_options;
            return 0x2i32;
        }
        if s.is_null() {
            if !target.is_null() {
                xasprintf(
                    cause,
                    b"no such session: %s\x00" as *const u8 as *const libc::c_char,
                    target,
                );
            } else {
                xasprintf(
                    cause,
                    b"no current session\x00" as *const u8 as *const libc::c_char,
                );
            }
            return 0i32;
        }
        *oo = (*s).options;
        return 0x2i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn options_string_to_style(
    mut oo: *mut options,
    mut name: *const libc::c_char,
    mut ft: *mut crate::format::format_tree,
) -> *mut Style {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    o = options_get(oo, name);
    if o.is_null()
        || !((*o).tableentry.is_null() || (*(*o).tableentry).type_0 == OPTIONS_TABLE_STRING)
    {
        return 0 as *mut Style;
    }
    if (*o).cached != 0 {
        return &mut (*o).style;
    }
    s = (*o).value.string;
    log_debug(
        b"%s: %s is \'%s\'\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"options_string_to_style\x00"))
            .as_ptr(),
        name,
        s,
    );
    style_set(&mut (*o).style, &grid_default_cell);
    (*o).cached = (strstr(s, b"#{\x00" as *const u8 as *const libc::c_char)
        == 0 as *mut libc::c_char) as libc::c_int;
    if !ft.is_null() && (*o).cached == 0 {
        expanded = format_expand(ft, s);
        if style_parse(&mut (*o).style, &grid_default_cell, expanded) != 0i32 {
            free(expanded as *mut libc::c_void);
            return 0 as *mut Style;
        }
        free(expanded as *mut libc::c_void);
    } else if style_parse(&mut (*o).style, &grid_default_cell, s) != 0i32 {
        return 0 as *mut Style;
    }
    return &mut (*o).style;
}
unsafe extern "C" fn options_from_string_check(
    mut oe: *const options_table_entry,
    mut value: *const libc::c_char,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut sy: Style = Style {
        gc: GridCell {
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
        ignore: 0,
        fill: 0,
        align: style_align::DEFAULT,
        list: style_list::OFF,
        range_type: style_range_type::NONE,
        range_argument: 0,
        default_type: style_default_type::BASE,
    };
    if oe.is_null() {
        return 0i32;
    }
    if strcmp(
        (*oe).name,
        b"default-shell\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
        && checkshell(value) == 0
    {
        xasprintf(
            cause,
            b"not a suitable shell: %s\x00" as *const u8 as *const libc::c_char,
            value,
        );
        return -(1i32);
    }
    if !(*oe).pattern.is_null() && fnmatch((*oe).pattern, value, 0i32) != 0i32 {
        xasprintf(
            cause,
            b"value is invalid: %s\x00" as *const u8 as *const libc::c_char,
            value,
        );
        return -(1i32);
    }
    if (*oe).flags & 0x4i32 != 0
        && strstr(value, b"#{\x00" as *const u8 as *const libc::c_char).is_null()
        && style_parse(&mut sy, &grid_default_cell, value) != 0i32
    {
        xasprintf(
            cause,
            b"invalid style: %s\x00" as *const u8 as *const libc::c_char,
            value,
        );
        return -(1i32);
    }
    return 0i32;
}
unsafe extern "C" fn options_from_string_flag(
    mut oo: *mut options,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut flag: libc::c_int = 0;
    if value.is_null() || *value as libc::c_int == '\u{0}' as i32 {
        flag = (options_get_number(oo, name) == 0) as libc::c_int
    } else if strcmp(value, b"1\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcasecmp(value, b"on\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcasecmp(value, b"yes\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        flag = 1i32
    } else if strcmp(value, b"0\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcasecmp(value, b"off\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcasecmp(value, b"no\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        flag = 0i32
    } else {
        xasprintf(
            cause,
            b"bad value: %s\x00" as *const u8 as *const libc::c_char,
            value,
        );
        return -(1i32);
    }
    options_set_number(oo, name, flag as libc::c_longlong);
    return 0i32;
}
unsafe extern "C" fn options_from_string_choice(
    mut oe: *const options_table_entry,
    mut oo: *mut options,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut cp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut choice: libc::c_int = -(1i32);
    if value.is_null() {
        choice = options_get_number(oo, name) as libc::c_int;
        if choice < 2i32 {
            choice = (choice == 0) as libc::c_int
        }
    } else {
        n = 0i32;
        cp = (*oe).choices;
        while !(*cp).is_null() {
            if strcmp(*cp, value) == 0i32 {
                choice = n
            }
            n += 1;
            cp = cp.offset(1)
        }
        if choice == -(1i32) {
            xasprintf(
                cause,
                b"unknown value: %s\x00" as *const u8 as *const libc::c_char,
                value,
            );
            return -(1i32);
        }
    }
    options_set_number(oo, name, choice as libc::c_longlong);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn options_from_string(
    mut oo: *mut options,
    mut oe: *const options_table_entry,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut append: libc::c_int,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut type_0: options_table_type = OPTIONS_TABLE_STRING;
    let mut number: libc::c_longlong = 0;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut new: *const libc::c_char = 0 as *const libc::c_char;
    let mut old: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: key_code = 0;
    if !oe.is_null() {
        if value.is_null()
            && (*oe).type_0 != OPTIONS_TABLE_FLAG
            && (*oe).type_0 != OPTIONS_TABLE_CHOICE
        {
            xasprintf(
                cause,
                b"empty value\x00" as *const u8 as *const libc::c_char,
            );
            return -(1i32);
        }
        type_0 = (*oe).type_0
    } else {
        if *name as libc::c_int != '@' as i32 {
            xasprintf(
                cause,
                b"bad option name\x00" as *const u8 as *const libc::c_char,
            );
            return -(1i32);
        }
        type_0 = OPTIONS_TABLE_STRING
    }
    match type_0 {
        0 => {
            old = xstrdup(options_get_string(oo, name));
            options_set_string(
                oo,
                name,
                append,
                b"%s\x00" as *const u8 as *const libc::c_char,
                value,
            );
            new = options_get_string(oo, name);
            if options_from_string_check(oe, new, cause) != 0i32 {
                options_set_string(
                    oo,
                    name,
                    0i32,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    old,
                );
                free(old as *mut libc::c_void);
                return -(1i32);
            }
            free(old as *mut libc::c_void);
            return 0i32;
        }
        1 => {
            number = strtonum(
                value,
                (*oe).minimum as libc::c_longlong,
                (*oe).maximum as libc::c_longlong,
                &mut errstr,
            );
            if !errstr.is_null() {
                xasprintf(
                    cause,
                    b"value is %s: %s\x00" as *const u8 as *const libc::c_char,
                    errstr,
                    value,
                );
                return -(1i32);
            }
            options_set_number(oo, name, number);
            return 0i32;
        }
        2 => {
            key = key_string_lookup_string(value);
            if key == 0xfe000000000u64 {
                xasprintf(
                    cause,
                    b"bad key: %s\x00" as *const u8 as *const libc::c_char,
                    value,
                );
                return -(1i32);
            }
            options_set_number(oo, name, key as libc::c_longlong);
            return 0i32;
        }
        3 => {
            number = colour_fromstring(value) as libc::c_longlong;
            if number == -1i64 {
                xasprintf(
                    cause,
                    b"bad colour: %s\x00" as *const u8 as *const libc::c_char,
                    value,
                );
                return -(1i32);
            }
            options_set_number(oo, name, number);
            return 0i32;
        }
        4 => return options_from_string_flag(oo, name, value, cause),
        5 => return options_from_string_choice(oe, oo, name, value, cause),
        6 | _ => {}
    }
    return -(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn options_push_changes(mut name: *const libc::c_char) {
    let mut loop_0: *mut client = 0 as *mut client;
    let mut s: *mut session = 0 as *mut session;
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if strcmp(
        name,
        b"automatic-rename\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        w = windows_RB_MINMAX(&mut windows, -(1i32));
        while !w.is_null() {
            if !(*w).active.is_null() {
                if options_get_number(
                    (*w).options,
                    b"automatic-rename\x00" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    (*(*w).active).flags |= 0x80i32
                }
            }
            w = windows_RB_NEXT(w)
        }
    }
    if strcmp(name, b"key-table\x00" as *const u8 as *const libc::c_char) == 0i32 {
        loop_0 = clients.tqh_first;
        while !loop_0.is_null() {
            server_client_set_key_table(loop_0, 0 as *const libc::c_char);
            loop_0 = (*loop_0).entry.tqe_next
        }
    }
    if strcmp(name, b"user-keys\x00" as *const u8 as *const libc::c_char) == 0i32 {
        loop_0 = clients.tqh_first;
        while !loop_0.is_null() {
            if (*loop_0).tty.flags & 0x20i32 != 0 {
                tty_keys_build(&mut (*loop_0).tty);
            }
            loop_0 = (*loop_0).entry.tqe_next
        }
    }
    if strcmp(name, b"status\x00" as *const u8 as *const libc::c_char) == 0i32
        || strcmp(
            name,
            b"status-interval\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
    {
        status_timer_start_all();
    }
    if strcmp(
        name,
        b"monitor-silence\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        alerts_reset_all();
    }
    if strcmp(
        name,
        b"window-style\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
        || strcmp(
            name,
            b"window-active-style\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
    {
        wp = window_pane_tree_RB_MINMAX(&mut all_window_panes, -(1i32));
        while !wp.is_null() {
            (*wp).flags |= 0x1000i32;
            wp = window_pane_tree_RB_NEXT(wp)
        }
    }
    if strcmp(
        name,
        b"pane-border-status\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        w = windows_RB_MINMAX(&mut windows, -(1i32));
        while !w.is_null() {
            layout_fix_panes(w);
            w = windows_RB_NEXT(w)
        }
    }
    s = sessions_RB_MINMAX(&mut sessions, -(1i32));
    while !s.is_null() {
        status_update_cache(s);
        s = sessions_RB_NEXT(s)
    }
    recalculate_sizes();
    loop_0 = clients.tqh_first;
    while !loop_0.is_null() {
        if !(*loop_0).session.is_null() {
            server_redraw_client(loop_0);
        }
        loop_0 = (*loop_0).entry.tqe_next
    }
}
#[no_mangle]
pub unsafe extern "C" fn options_remove_or_default(
    mut o: *mut options_entry,
    mut idx: libc::c_int,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut oo: *mut options = (*o).owner;
    if idx == -(1i32) {
        if !(*o).tableentry.is_null()
            && (oo == global_options || oo == global_s_options || oo == global_w_options)
        {
            options_default(oo, (*o).tableentry);
        } else {
            options_remove(o);
        }
    } else if options_array_set(o, idx as u_int, 0 as *const libc::c_char, 0i32, cause) != 0i32 {
        return -(1i32);
    }
    return 0i32;
}
