use ::libc;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    pub type args_entry;
    pub type screen_write_collect_line;
    pub type screen_sel;
    pub type screen_titles;
    pub type options;
    pub type format_tree;
    pub type input_ctx;
    pub type cmds;
    pub type tty_code;
    pub type format_job_tree;
    pub type control_state;
    pub type cmdq_list;
    pub type tmuxpeer;
    pub type options_array_item;
    pub type options_entry;
    #[no_mangle]
    fn fnmatch(__pattern: *const libc::c_char, __name: *const libc::c_char,
               __flags: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn setenv(__name: *const libc::c_char, __value: *const libc::c_char,
              __replace: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn vasprintf(__ptr: *mut *mut libc::c_char, __f: *const libc::c_char,
                 __arg: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xvasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char,
                  _: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut options;
    #[no_mangle]
    static mut global_environ: *mut environ;
    #[no_mangle]
    static mut socket_path: *const libc::c_char;
    #[no_mangle]
    fn getversion() -> *const libc::c_char;
    #[no_mangle]
    fn options_get(_: *mut options, _: *const libc::c_char)
     -> *mut options_entry;
    #[no_mangle]
    fn options_array_first(_: *mut options_entry) -> *mut options_array_item;
    #[no_mangle]
    fn options_array_next(_: *mut options_array_item)
     -> *mut options_array_item;
    #[no_mangle]
    fn options_array_item_value(_: *mut options_array_item)
     -> *mut options_value;
    #[no_mangle]
    fn options_get_string(_: *mut options, _: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
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
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type va_list = __builtin_va_list;
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
    pub evcb_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_short,
                                                   _: *mut libc::c_void)
                                  -> ()>,
    pub evcb_selfcb: Option<unsafe extern "C" fn(_: *mut event_callback,
                                                 _: *mut libc::c_void) -> ()>,
    pub evcb_evfinalize: Option<unsafe extern "C" fn(_: *mut event,
                                                     _: *mut libc::c_void)
                                    -> ()>,
    pub evcb_cbfinalize: Option<unsafe extern "C" fn(_: *mut event_callback,
                                                     _: *mut libc::c_void)
                                    -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
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
pub type bufferevent_event_cb
    =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: libc::c_short,
                                _: *mut libc::c_void) -> ()>;
pub type bufferevent_data_cb
    =
    Option<unsafe extern "C" fn(_: *mut bufferevent, _: *mut libc::c_void)
               -> ()>;
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
    pub exit_type: C2RustUnnamed_29,
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
    pub prompt_mode: C2RustUnnamed_26,
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
pub type client_file_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: *const libc::c_char,
                                _: libc::c_int, _: libc::c_int,
                                _: *mut evbuffer, _: *mut libc::c_void)
               -> ()>;
pub type overlay_free_cb = Option<unsafe extern "C" fn(_: *mut client) -> ()>;
pub type overlay_key_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut key_event)
               -> libc::c_int>;
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
pub type overlay_draw_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut screen_redraw_ctx)
               -> ()>;
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
pub type overlay_mode_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut u_int, _: *mut u_int)
               -> *mut screen>;
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
pub type overlay_check_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: u_int, _: u_int)
               -> libc::c_int>;
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
/* $OpenBSD$ */
/*
 * Copyright (c) 2009 Nicholas Marriott <nicholas.marriott@gmail.com>
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
 * Environment - manipulate a set of environment variables.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environ {
    pub rbh_root: *mut environ_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environ_entry {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub flags: libc::c_int,
    pub entry: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub rbe_left: *mut environ_entry,
    pub rbe_right: *mut environ_entry,
    pub rbe_parent: *mut environ_entry,
    pub rbe_color: libc::c_int,
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
    pub entry: C2RustUnnamed_17,
    pub wentry: C2RustUnnamed_16,
    pub sentry: C2RustUnnamed_15,
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
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
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
    pub alerts_entry: C2RustUnnamed_20,
    pub options: *mut options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_19,
    pub entry: C2RustUnnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
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
    pub entry: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
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
    pub modes: C2RustUnnamed_24,
    pub searchstr: *mut libc::c_char,
    pub searchregex: libc::c_int,
    pub written: size_t,
    pub skipped: size_t,
    pub border_gc_set: libc::c_int,
    pub border_gc: grid_cell,
    pub entry: C2RustUnnamed_23,
    pub tree_entry: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
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
    pub entry: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub tqe_next: *mut window_mode_entry,
    pub tqe_prev: *mut *mut window_mode_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_mode {
    pub name: *const libc::c_char,
    pub default_format: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(_: *mut window_mode_entry,
                                          _: *mut cmd_find_state,
                                          _: *mut args) -> *mut screen>,
    pub free: Option<unsafe extern "C" fn(_: *mut window_mode_entry) -> ()>,
    pub resize: Option<unsafe extern "C" fn(_: *mut window_mode_entry,
                                            _: u_int, _: u_int) -> ()>,
    pub key: Option<unsafe extern "C" fn(_: *mut window_mode_entry,
                                         _: *mut client, _: *mut session,
                                         _: *mut winlink, _: key_code,
                                         _: *mut mouse_event) -> ()>,
    pub key_table: Option<unsafe extern "C" fn(_: *mut window_mode_entry)
                              -> *const libc::c_char>,
    pub command: Option<unsafe extern "C" fn(_: *mut window_mode_entry,
                                             _: *mut client, _: *mut session,
                                             _: *mut winlink, _: *mut args,
                                             _: *mut mouse_event) -> ()>,
    pub formats: Option<unsafe extern "C" fn(_: *mut window_mode_entry,
                                             _: *mut format_tree) -> ()>,
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
pub type C2RustUnnamed_26 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_26 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_26 = 0;
pub type prompt_free_cb
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type prompt_input_cb
    =
    Option<unsafe extern "C" fn(_: *mut client, _: *mut libc::c_void,
                                _: *const libc::c_char, _: libc::c_int)
               -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_table {
    pub name: *const libc::c_char,
    pub key_bindings: key_bindings,
    pub default_key_bindings: key_bindings,
    pub references: u_int,
    pub entry: C2RustUnnamed_27,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
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
    pub entry: C2RustUnnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
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
pub type C2RustUnnamed_29 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_29 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_29 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_29 = 0;
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
    pub entry: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
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
    pub mouse_drag_update: Option<unsafe extern "C" fn(_: *mut client,
                                                       _: *mut mouse_event)
                                      -> ()>,
    pub mouse_drag_release: Option<unsafe extern "C" fn(_: *mut client,
                                                        _: *mut mouse_event)
                                       -> ()>,
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
    pub entry: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
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
    pub entry: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub rbe_left: *mut client_window,
    pub rbe_right: *mut client_window,
    pub rbe_parent: *mut client_window,
    pub rbe_color: libc::c_int,
}
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
unsafe extern "C" fn environ_RB_FIND(mut head: *mut environ,
                                     mut elm: *mut environ_entry)
 -> *mut environ_entry {
    let mut tmp: *mut environ_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = environ_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    return 0 as *mut environ_entry;
}
unsafe extern "C" fn environ_RB_MINMAX(mut head: *mut environ,
                                       mut val: libc::c_int)
 -> *mut environ_entry {
    let mut tmp: *mut environ_entry = (*head).rbh_root;
    let mut parent: *mut environ_entry = 0 as *mut environ_entry;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else { tmp = (*tmp).entry.rbe_right }
    }
    return parent;
}
unsafe extern "C" fn environ_RB_NEXT(mut elm: *mut environ_entry)
 -> *mut environ_entry {
    if !(*elm).entry.rbe_right.is_null() {
        elm = (*elm).entry.rbe_right;
        while !(*elm).entry.rbe_left.is_null() { elm = (*elm).entry.rbe_left }
    } else if !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_left {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() &&
                  elm == (*(*elm).entry.rbe_parent).entry.rbe_right {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
unsafe extern "C" fn environ_RB_REMOVE_COLOR(mut head: *mut environ,
                                             mut parent: *mut environ_entry,
                                             mut elm: *mut environ_entry) {
    let mut tmp: *mut environ_entry = 0 as *mut environ_entry;
    while (elm.is_null() || (*elm).entry.rbe_color == 0 as libc::c_int) &&
              elm != (*head).rbh_root {
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
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left
                       {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else { (*head).rbh_root = tmp }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                tmp = (*parent).entry.rbe_right
            }
            if ((*tmp).entry.rbe_left.is_null() ||
                    (*(*tmp).entry.rbe_left).entry.rbe_color ==
                        0 as libc::c_int) &&
                   ((*tmp).entry.rbe_right.is_null() ||
                        (*(*tmp).entry.rbe_right).entry.rbe_color ==
                            0 as libc::c_int) {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_right.is_null() ||
                       (*(*tmp).entry.rbe_right).entry.rbe_color ==
                           0 as libc::c_int {
                    let mut oleft: *mut environ_entry =
                        0 as *mut environ_entry;
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
                    } else { (*head).rbh_root = oleft }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    !(*oleft).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_right
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !(*tmp).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_color =
                        0 as libc::c_int
                }
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !(*parent).entry.rbe_right.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left
                       {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else { (*head).rbh_root = tmp }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                elm = (*head).rbh_root;
                break ;
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
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left
                       {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else { (*head).rbh_root = tmp }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                tmp = (*parent).entry.rbe_left
            }
            if ((*tmp).entry.rbe_left.is_null() ||
                    (*(*tmp).entry.rbe_left).entry.rbe_color ==
                        0 as libc::c_int) &&
                   ((*tmp).entry.rbe_right.is_null() ||
                        (*(*tmp).entry.rbe_right).entry.rbe_color ==
                            0 as libc::c_int) {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent
            } else {
                if (*tmp).entry.rbe_left.is_null() ||
                       (*(*tmp).entry.rbe_left).entry.rbe_color ==
                           0 as libc::c_int {
                    let mut oright: *mut environ_entry =
                        0 as *mut environ_entry;
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
                            (*(*tmp).entry.rbe_parent).entry.rbe_right =
                                oright
                        }
                    } else { (*head).rbh_root = oright }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    !(*oright).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_left
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !(*tmp).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_color =
                        0 as libc::c_int
                }
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !(*parent).entry.rbe_left.is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !(*tmp).entry.rbe_parent.is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left
                       {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else { (*head).rbh_root = tmp }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
                elm = (*head).rbh_root;
                break ;
            }
        }
    }
    if !elm.is_null() { (*elm).entry.rbe_color = 0 as libc::c_int };
}
unsafe extern "C" fn environ_RB_REMOVE(mut head: *mut environ,
                                       mut elm: *mut environ_entry)
 -> *mut environ_entry {
    let mut current_block: u64;
    let mut child: *mut environ_entry = 0 as *mut environ_entry;
    let mut parent: *mut environ_entry = 0 as *mut environ_entry;
    let mut old: *mut environ_entry = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut environ_entry = 0 as *mut environ_entry;
        elm = (*elm).entry.rbe_right;
        loop  {
            left = (*elm).entry.rbe_left;
            if left.is_null() { break ; }
            elm = left
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() { (*child).entry.rbe_parent = parent }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child
            } else { (*parent).entry.rbe_right = child }
        } else { (*head).rbh_root = child }
        if (*elm).entry.rbe_parent == old { parent = elm }
        (*elm).entry = (*old).entry;
        if !(*old).entry.rbe_parent.is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm
            } else { (*(*old).entry.rbe_parent).entry.rbe_right = elm }
        } else { (*head).rbh_root = elm }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !(*old).entry.rbe_right.is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm
        }
        if !parent.is_null() {
            left = parent;
            loop  {
                left = (*left).entry.rbe_parent;
                if left.is_null() { break ; }
            }
        }
        current_block = 7381876040995060082;
    }
    match current_block {
        7226443171521532240 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() { (*child).entry.rbe_parent = parent }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child
                } else { (*parent).entry.rbe_right = child }
            } else { (*head).rbh_root = child }
        }
        _ => { }
    }
    if color == 0 as libc::c_int {
        environ_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn environ_RB_INSERT_COLOR(mut head: *mut environ,
                                             mut elm: *mut environ_entry) {
    let mut parent: *mut environ_entry = 0 as *mut environ_entry;
    let mut gparent: *mut environ_entry = 0 as *mut environ_entry;
    let mut tmp: *mut environ_entry = 0 as *mut environ_entry;
    loop  {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() &&
                 (*parent).entry.rbe_color == 1 as libc::c_int) {
            break ;
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
                        if parent ==
                               (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right =
                                tmp
                        }
                    } else { (*head).rbh_root = tmp }
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
                    if gparent ==
                           (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else { (*head).rbh_root = tmp }
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
                        if parent ==
                               (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right =
                                tmp
                        }
                    } else { (*head).rbh_root = tmp }
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
                    if gparent ==
                           (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp
                    }
                } else { (*head).rbh_root = tmp }
                (*tmp).entry.rbe_left = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !(*tmp).entry.rbe_parent.is_null();
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn environ_RB_INSERT(mut head: *mut environ,
                                       mut elm: *mut environ_entry)
 -> *mut environ_entry {
    let mut tmp: *mut environ_entry = 0 as *mut environ_entry;
    let mut parent: *mut environ_entry = 0 as *mut environ_entry;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = environ_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else { return tmp }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut environ_entry;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).entry.rbe_left = elm
        } else { (*parent).entry.rbe_right = elm }
    } else { (*head).rbh_root = elm }
    environ_RB_INSERT_COLOR(head, elm);
    return 0 as *mut environ_entry;
}
unsafe extern "C" fn environ_cmp(mut envent1: *mut environ_entry,
                                 mut envent2: *mut environ_entry)
 -> libc::c_int {
    return strcmp((*envent1).name, (*envent2).name);
}
/* Initialise the environment. */
#[no_mangle]
pub unsafe extern "C" fn environ_create() -> *mut environ {
    let mut env: *mut environ = 0 as *mut environ;
    env =
        xcalloc(1 as libc::c_int as size_t,
                ::std::mem::size_of::<environ>() as libc::c_ulong) as
            *mut environ;
    (*env).rbh_root = 0 as *mut environ_entry;
    return env;
}
/* Free an environment. */
#[no_mangle]
pub unsafe extern "C" fn environ_free(mut env: *mut environ) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut envent1: *mut environ_entry = 0 as *mut environ_entry;
    envent = environ_RB_MINMAX(env, -(1 as libc::c_int));
    while !envent.is_null() &&
              { envent1 = environ_RB_NEXT(envent); (1 as libc::c_int) != 0 } {
        environ_RB_REMOVE(env, envent);
        free((*envent).name as *mut libc::c_void);
        free((*envent).value as *mut libc::c_void);
        free(envent as *mut libc::c_void);
        envent = envent1
    }
    free(env as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn environ_first(mut env: *mut environ)
 -> *mut environ_entry {
    return environ_RB_MINMAX(env, -(1 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn environ_next(mut envent: *mut environ_entry)
 -> *mut environ_entry {
    return environ_RB_NEXT(envent);
}
/* Copy one environment into another. */
#[no_mangle]
pub unsafe extern "C" fn environ_copy(mut srcenv: *mut environ,
                                      mut dstenv: *mut environ) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    envent = environ_RB_MINMAX(srcenv, -(1 as libc::c_int));
    while !envent.is_null() {
        if (*envent).value.is_null() {
            environ_clear(dstenv, (*envent).name);
        } else {
            environ_set(dstenv, (*envent).name, (*envent).flags,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        (*envent).value);
        }
        envent = environ_RB_NEXT(envent)
    };
}
/* Find an environment variable. */
#[no_mangle]
pub unsafe extern "C" fn environ_find(mut env: *mut environ,
                                      mut name: *const libc::c_char)
 -> *mut environ_entry {
    let mut envent: environ_entry =
        environ_entry{name: 0 as *mut libc::c_char,
                      value: 0 as *mut libc::c_char,
                      flags: 0,
                      entry:
                          C2RustUnnamed_14{rbe_left: 0 as *mut environ_entry,
                                           rbe_right: 0 as *mut environ_entry,
                                           rbe_parent:
                                               0 as *mut environ_entry,
                                           rbe_color: 0,},};
    envent.name = name as *mut libc::c_char;
    return environ_RB_FIND(env, &mut envent);
}
/* Set an environment variable. */
#[no_mangle]
pub unsafe extern "C" fn environ_set(mut env: *mut environ,
                                     mut name: *const libc::c_char,
                                     mut flags: libc::c_int,
                                     mut fmt: *const libc::c_char,
                                     mut args: ...) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    envent = environ_find(env, name);
    if !envent.is_null() {
        (*envent).flags = flags;
        free((*envent).value as *mut libc::c_void);
        xvasprintf(&mut (*envent).value, fmt, ap.as_va_list());
    } else {
        envent =
            xmalloc(::std::mem::size_of::<environ_entry>() as libc::c_ulong)
                as *mut environ_entry;
        (*envent).name = xstrdup(name);
        (*envent).flags = flags;
        xvasprintf(&mut (*envent).value, fmt, ap.as_va_list());
        environ_RB_INSERT(env, envent);
    };
}
/* Clear an environment variable. */
#[no_mangle]
pub unsafe extern "C" fn environ_clear(mut env: *mut environ,
                                       mut name: *const libc::c_char) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    envent = environ_find(env, name);
    if !envent.is_null() {
        free((*envent).value as *mut libc::c_void);
        (*envent).value = 0 as *mut libc::c_char
    } else {
        envent =
            xmalloc(::std::mem::size_of::<environ_entry>() as libc::c_ulong)
                as *mut environ_entry;
        (*envent).name = xstrdup(name);
        (*envent).flags = 0 as libc::c_int;
        (*envent).value = 0 as *mut libc::c_char;
        environ_RB_INSERT(env, envent);
    };
}
/* Set an environment variable from a NAME=VALUE string. */
#[no_mangle]
pub unsafe extern "C" fn environ_put(mut env: *mut environ,
                                     mut var: *const libc::c_char,
                                     mut flags: libc::c_int) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    value = strchr(var, '=' as i32);
    if value.is_null() { return }
    value = value.offset(1);
    name = xstrdup(var);
    *name.offset(strcspn(name, b"=\x00" as *const u8 as *const libc::c_char)
                     as isize) = '\u{0}' as i32 as libc::c_char;
    environ_set(env, name, flags,
                b"%s\x00" as *const u8 as *const libc::c_char, value);
    free(name as *mut libc::c_void);
}
/* Unset an environment variable. */
#[no_mangle]
pub unsafe extern "C" fn environ_unset(mut env: *mut environ,
                                       mut name: *const libc::c_char) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    envent = environ_find(env, name);
    if envent.is_null() { return }
    environ_RB_REMOVE(env, envent);
    free((*envent).name as *mut libc::c_void);
    free((*envent).value as *mut libc::c_void);
    free(envent as *mut libc::c_void);
}
/* Copy variables from a destination into a source environment. */
#[no_mangle]
pub unsafe extern "C" fn environ_update(mut oo: *mut options,
                                        mut src: *mut environ,
                                        mut dst: *mut environ) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    let mut ov: *mut options_value = 0 as *mut options_value;
    o =
        options_get(oo,
                    b"update-environment\x00" as *const u8 as
                        *const libc::c_char);
    if o.is_null() { return }
    a = options_array_first(o);
    while !a.is_null() {
        ov = options_array_item_value(a);
        envent = environ_RB_MINMAX(src, -(1 as libc::c_int));
        while !envent.is_null() {
            if fnmatch((*ov).string, (*envent).name, 0 as libc::c_int) ==
                   0 as libc::c_int {
                break ;
            }
            envent = environ_RB_NEXT(envent)
        }
        if envent.is_null() {
            environ_clear(dst, (*ov).string);
        } else {
            environ_set(dst, (*envent).name, 0 as libc::c_int,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        (*envent).value);
        }
        a = options_array_next(a)
    };
}
/* Push environment into the real environment - use after fork(). */
#[no_mangle]
pub unsafe extern "C" fn environ_push(mut env: *mut environ) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    environ =
        xcalloc(1 as libc::c_int as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as *mut *mut libc::c_char;
    envent = environ_RB_MINMAX(env, -(1 as libc::c_int));
    while !envent.is_null() {
        if !(*envent).value.is_null() &&
               *(*envent).name as libc::c_int != '\u{0}' as i32 &&
               !(*envent).flags & 0x1 as libc::c_int != 0 {
            setenv((*envent).name, (*envent).value, 1 as libc::c_int);
        }
        envent = environ_RB_NEXT(envent)
    };
}
/* Log the environment. */
#[no_mangle]
pub unsafe extern "C" fn environ_log(mut env: *mut environ,
                                     mut fmt: *const libc::c_char,
                                     mut args: ...) {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut ap: ::std::ffi::VaListImpl;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    vasprintf(&mut prefix, fmt, ap.as_va_list());
    envent = environ_RB_MINMAX(env, -(1 as libc::c_int));
    while !envent.is_null() {
        if !(*envent).value.is_null() &&
               *(*envent).name as libc::c_int != '\u{0}' as i32 {
            log_debug(b"%s%s=%s\x00" as *const u8 as *const libc::c_char,
                      prefix, (*envent).name, (*envent).value);
        }
        envent = environ_RB_NEXT(envent)
    }
    free(prefix as *mut libc::c_void);
}
/* Create initial environment for new child. */
#[no_mangle]
pub unsafe extern "C" fn environ_for_session(mut s: *mut session,
                                             mut no_TERM: libc::c_int)
 -> *mut environ {
    let mut env: *mut environ = 0 as *mut environ;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0;
    env = environ_create();
    environ_copy(global_environ, env);
    if !s.is_null() { environ_copy((*s).environ, env); }
    if no_TERM == 0 {
        value =
            options_get_string(global_options,
                               b"default-terminal\x00" as *const u8 as
                                   *const libc::c_char);
        environ_set(env, b"TERM\x00" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"%s\x00" as *const u8 as *const libc::c_char, value);
        environ_set(env,
                    b"TERM_PROGRAM\x00" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tmux\x00" as *const u8 as *const libc::c_char);
        environ_set(env,
                    b"TERM_PROGRAM_VERSION\x00" as *const u8 as
                        *const libc::c_char, 0 as libc::c_int,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    getversion());
    }
    if !s.is_null() {
        idx = (*s).id as libc::c_int
    } else { idx = -(1 as libc::c_int) }
    environ_set(env, b"TMUX\x00" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"%s,%ld,%d\x00" as *const u8 as *const libc::c_char,
                socket_path, getpid() as libc::c_long, idx);
    return env;
}
