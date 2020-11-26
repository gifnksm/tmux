use ::libc;
extern "C" {
    pub type event_base;
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
    pub type cmdq_item;
    pub type tty_code;
    pub type format_job_tree;
    pub type control_state;
    pub type cmdq_list;
    pub type tmuxpeer;
    pub type options_array_item;
    pub type options_entry;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
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
    fn options_get_only(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    #[no_mangle]
    fn options_array_first(_: *mut options_entry) -> *mut options_array_item;
    #[no_mangle]
    fn options_array_next(_: *mut options_array_item) -> *mut options_array_item;
    #[no_mangle]
    fn options_array_item_value(_: *mut options_array_item) -> *mut options_value;
    #[no_mangle]
    fn args_parse(_: *const libc::c_char, _: libc::c_int, _: *mut *mut libc::c_char) -> *mut args;
    #[no_mangle]
    fn args_free(_: *mut args);
    #[no_mangle]
    fn args_print(_: *mut args) -> *mut libc::c_char;
    #[no_mangle]
    fn args_escape(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn window_pane_find_by_id(_: u_int) -> *mut window_pane;
    #[no_mangle]
    fn window_has_pane(_: *mut window, _: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn window_find_by_id(_: u_int) -> *mut window;
    #[no_mangle]
    fn winlink_find_by_window(_: *mut winlinks, _: *mut window) -> *mut winlink;
    #[no_mangle]
    fn session_find_by_id(_: u_int) -> *mut session;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
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
    #[no_mangle]
    static cmd_attach_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_bind_key_entry: cmd_entry;
    #[no_mangle]
    static cmd_break_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_capture_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_choose_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_choose_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_choose_tree_entry: cmd_entry;
    #[no_mangle]
    static cmd_clear_history_entry: cmd_entry;
    #[no_mangle]
    static cmd_clock_mode_entry: cmd_entry;
    #[no_mangle]
    static cmd_command_prompt_entry: cmd_entry;
    #[no_mangle]
    static cmd_confirm_before_entry: cmd_entry;
    #[no_mangle]
    static cmd_copy_mode_entry: cmd_entry;
    #[no_mangle]
    static cmd_customize_mode_entry: cmd_entry;
    #[no_mangle]
    static cmd_delete_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_detach_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_display_menu_entry: cmd_entry;
    #[no_mangle]
    static cmd_display_message_entry: cmd_entry;
    #[no_mangle]
    static cmd_display_popup_entry: cmd_entry;
    #[no_mangle]
    static cmd_display_panes_entry: cmd_entry;
    #[no_mangle]
    static cmd_find_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_has_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_if_shell_entry: cmd_entry;
    #[no_mangle]
    static cmd_join_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_kill_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_kill_server_entry: cmd_entry;
    #[no_mangle]
    static cmd_kill_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_kill_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_last_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_last_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_link_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_buffers_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_clients_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_commands_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_keys_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_panes_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_sessions_entry: cmd_entry;
    #[no_mangle]
    static cmd_list_windows_entry: cmd_entry;
    #[no_mangle]
    static cmd_load_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_lock_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_lock_server_entry: cmd_entry;
    #[no_mangle]
    static cmd_lock_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_move_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_move_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_new_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_new_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_next_layout_entry: cmd_entry;
    #[no_mangle]
    static cmd_next_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_paste_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_pipe_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_previous_layout_entry: cmd_entry;
    #[no_mangle]
    static cmd_previous_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_refresh_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_rename_session_entry: cmd_entry;
    #[no_mangle]
    static cmd_rename_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_resize_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_resize_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_respawn_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_respawn_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_rotate_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_run_shell_entry: cmd_entry;
    #[no_mangle]
    static cmd_save_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_select_layout_entry: cmd_entry;
    #[no_mangle]
    static cmd_select_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_select_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_send_keys_entry: cmd_entry;
    #[no_mangle]
    static cmd_send_prefix_entry: cmd_entry;
    #[no_mangle]
    static cmd_set_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_set_environment_entry: cmd_entry;
    #[no_mangle]
    static cmd_set_hook_entry: cmd_entry;
    #[no_mangle]
    static cmd_set_option_entry: cmd_entry;
    #[no_mangle]
    static cmd_set_window_option_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_buffer_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_environment_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_hooks_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_messages_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_options_entry: cmd_entry;
    #[no_mangle]
    static cmd_show_window_options_entry: cmd_entry;
    #[no_mangle]
    static cmd_source_file_entry: cmd_entry;
    #[no_mangle]
    static cmd_split_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_start_server_entry: cmd_entry;
    #[no_mangle]
    static cmd_suspend_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_swap_pane_entry: cmd_entry;
    #[no_mangle]
    static cmd_swap_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_switch_client_entry: cmd_entry;
    #[no_mangle]
    static cmd_unbind_key_entry: cmd_entry;
    #[no_mangle]
    static cmd_unlink_window_entry: cmd_entry;
    #[no_mangle]
    static cmd_wait_for_entry: cmd_entry;
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
    pub exit_type: C2RustUnnamed_30,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmds {
    pub tqh_first: *mut cmd,
    pub tqh_last: *mut *mut cmd,
}
/* Instance of a command. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd {
    pub entry: *const cmd_entry,
    pub args: *mut args,
    pub group: u_int,
    pub file: *mut libc::c_char,
    pub line: u_int,
    pub alias: *mut libc::c_char,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub qentry: C2RustUnnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub tqe_next: *mut cmd,
    pub tqe_prev: *mut *mut cmd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_entry {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub args: C2RustUnnamed_29,
    pub usage: *const libc::c_char,
    pub source: cmd_entry_flag,
    pub target: cmd_entry_flag,
    pub flags: libc::c_int,
    pub exec: Option<unsafe extern "C" fn(_: *mut cmd, _: *mut cmdq_item) -> cmd_retval>,
}
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
pub type cmd_find_type = libc::c_uint;
pub const CMD_FIND_SESSION: cmd_find_type = 2;
pub const CMD_FIND_WINDOW: cmd_find_type = 1;
pub const CMD_FIND_PANE: cmd_find_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub template: *const libc::c_char,
    pub lower: libc::c_int,
    pub upper: libc::c_int,
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
pub type C2RustUnnamed_30 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_30 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_30 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_30 = 0;
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
    pub entry: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
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
    pub entry: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
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
    pub entry: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
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
#[no_mangle]
pub static mut cmd_table: [*const cmd_entry; 88] = unsafe {
    [
        &cmd_attach_session_entry as *const cmd_entry,
        &cmd_bind_key_entry as *const cmd_entry,
        &cmd_break_pane_entry as *const cmd_entry,
        &cmd_capture_pane_entry as *const cmd_entry,
        &cmd_choose_buffer_entry as *const cmd_entry,
        &cmd_choose_client_entry as *const cmd_entry,
        &cmd_choose_tree_entry as *const cmd_entry,
        &cmd_clear_history_entry as *const cmd_entry,
        &cmd_clock_mode_entry as *const cmd_entry,
        &cmd_command_prompt_entry as *const cmd_entry,
        &cmd_confirm_before_entry as *const cmd_entry,
        &cmd_copy_mode_entry as *const cmd_entry,
        &cmd_customize_mode_entry as *const cmd_entry,
        &cmd_delete_buffer_entry as *const cmd_entry,
        &cmd_detach_client_entry as *const cmd_entry,
        &cmd_display_menu_entry as *const cmd_entry,
        &cmd_display_message_entry as *const cmd_entry,
        &cmd_display_popup_entry as *const cmd_entry,
        &cmd_display_panes_entry as *const cmd_entry,
        &cmd_find_window_entry as *const cmd_entry,
        &cmd_has_session_entry as *const cmd_entry,
        &cmd_if_shell_entry as *const cmd_entry,
        &cmd_join_pane_entry as *const cmd_entry,
        &cmd_kill_pane_entry as *const cmd_entry,
        &cmd_kill_server_entry as *const cmd_entry,
        &cmd_kill_session_entry as *const cmd_entry,
        &cmd_kill_window_entry as *const cmd_entry,
        &cmd_last_pane_entry as *const cmd_entry,
        &cmd_last_window_entry as *const cmd_entry,
        &cmd_link_window_entry as *const cmd_entry,
        &cmd_list_buffers_entry as *const cmd_entry,
        &cmd_list_clients_entry as *const cmd_entry,
        &cmd_list_commands_entry as *const cmd_entry,
        &cmd_list_keys_entry as *const cmd_entry,
        &cmd_list_panes_entry as *const cmd_entry,
        &cmd_list_sessions_entry as *const cmd_entry,
        &cmd_list_windows_entry as *const cmd_entry,
        &cmd_load_buffer_entry as *const cmd_entry,
        &cmd_lock_client_entry as *const cmd_entry,
        &cmd_lock_server_entry as *const cmd_entry,
        &cmd_lock_session_entry as *const cmd_entry,
        &cmd_move_pane_entry as *const cmd_entry,
        &cmd_move_window_entry as *const cmd_entry,
        &cmd_new_session_entry as *const cmd_entry,
        &cmd_new_window_entry as *const cmd_entry,
        &cmd_next_layout_entry as *const cmd_entry,
        &cmd_next_window_entry as *const cmd_entry,
        &cmd_paste_buffer_entry as *const cmd_entry,
        &cmd_pipe_pane_entry as *const cmd_entry,
        &cmd_previous_layout_entry as *const cmd_entry,
        &cmd_previous_window_entry as *const cmd_entry,
        &cmd_refresh_client_entry as *const cmd_entry,
        &cmd_rename_session_entry as *const cmd_entry,
        &cmd_rename_window_entry as *const cmd_entry,
        &cmd_resize_pane_entry as *const cmd_entry,
        &cmd_resize_window_entry as *const cmd_entry,
        &cmd_respawn_pane_entry as *const cmd_entry,
        &cmd_respawn_window_entry as *const cmd_entry,
        &cmd_rotate_window_entry as *const cmd_entry,
        &cmd_run_shell_entry as *const cmd_entry,
        &cmd_save_buffer_entry as *const cmd_entry,
        &cmd_select_layout_entry as *const cmd_entry,
        &cmd_select_pane_entry as *const cmd_entry,
        &cmd_select_window_entry as *const cmd_entry,
        &cmd_send_keys_entry as *const cmd_entry,
        &cmd_send_prefix_entry as *const cmd_entry,
        &cmd_set_buffer_entry as *const cmd_entry,
        &cmd_set_environment_entry as *const cmd_entry,
        &cmd_set_hook_entry as *const cmd_entry,
        &cmd_set_option_entry as *const cmd_entry,
        &cmd_set_window_option_entry as *const cmd_entry,
        &cmd_show_buffer_entry as *const cmd_entry,
        &cmd_show_environment_entry as *const cmd_entry,
        &cmd_show_hooks_entry as *const cmd_entry,
        &cmd_show_messages_entry as *const cmd_entry,
        &cmd_show_options_entry as *const cmd_entry,
        &cmd_show_window_options_entry as *const cmd_entry,
        &cmd_source_file_entry as *const cmd_entry,
        &cmd_split_window_entry as *const cmd_entry,
        &cmd_start_server_entry as *const cmd_entry,
        &cmd_suspend_client_entry as *const cmd_entry,
        &cmd_swap_pane_entry as *const cmd_entry,
        &cmd_swap_window_entry as *const cmd_entry,
        &cmd_switch_client_entry as *const cmd_entry,
        &cmd_unbind_key_entry as *const cmd_entry,
        &cmd_unlink_window_entry as *const cmd_entry,
        &cmd_wait_for_entry as *const cmd_entry,
        0 as *const cmd_entry,
    ]
};
/* Next group number for new command list. */
static mut cmd_list_next_group: u_int = 1 as libc::c_int as u_int;
/* Log an argument vector. */
#[no_mangle]
pub unsafe extern "C" fn cmd_log_argv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    let mut i: libc::c_int = 0;
    ap = args.clone();
    xvasprintf(&mut prefix, fmt, ap.as_va_list());
    i = 0 as libc::c_int;
    while i < argc {
        log_debug(
            b"%s: argv[%d]=%s\x00" as *const u8 as *const libc::c_char,
            prefix,
            i,
            *argv.offset(i as isize),
        );
        i += 1
    }
    free(prefix as *mut libc::c_void);
}
/* Prepend to an argument vector. */
#[no_mangle]
pub unsafe extern "C" fn cmd_prepend_argv(
    mut argc: *mut libc::c_int,
    mut argv: *mut *mut *mut libc::c_char,
    mut arg: *mut libc::c_char,
) {
    let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    new_argv = xreallocarray(
        0 as *mut libc::c_void,
        (*argc + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    let ref mut fresh0 = *new_argv.offset(0 as libc::c_int as isize);
    *fresh0 = xstrdup(arg);
    i = 0 as libc::c_int;
    while i < *argc {
        let ref mut fresh1 = *new_argv.offset((1 as libc::c_int + i) as isize);
        *fresh1 = *(*argv).offset(i as isize);
        i += 1
    }
    free(*argv as *mut libc::c_void);
    *argv = new_argv;
    *argc += 1;
}
/* Append to an argument vector. */
#[no_mangle]
pub unsafe extern "C" fn cmd_append_argv(
    mut argc: *mut libc::c_int,
    mut argv: *mut *mut *mut libc::c_char,
    mut arg: *mut libc::c_char,
) {
    *argv = xreallocarray(
        *argv as *mut libc::c_void,
        (*argc + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    let fresh2 = *argc;
    *argc = *argc + 1;
    let ref mut fresh3 = *(*argv).offset(fresh2 as isize);
    *fresh3 = xstrdup(arg);
}
/* Pack an argument vector up into a buffer. */
#[no_mangle]
pub unsafe extern "C" fn cmd_pack_argv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut arglen: size_t = 0;
    let mut i: libc::c_int = 0;
    if argc == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    cmd_log_argv(
        argc,
        argv,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"cmd_pack_argv\x00")).as_ptr(),
    );
    *buf = '\u{0}' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        if strlcpy(buf, *argv.offset(i as isize), len) >= len {
            return -(1 as libc::c_int);
        }
        arglen = strlen(*argv.offset(i as isize)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        buf = buf.offset(arglen as isize);
        len = (len as libc::c_ulong).wrapping_sub(arglen) as size_t as size_t;
        i += 1
    }
    return 0 as libc::c_int;
}
/* Unpack an argument vector from a packed buffer. */
#[no_mangle]
pub unsafe extern "C" fn cmd_unpack_argv(
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut arglen: size_t = 0;
    if argc == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    *argv = xcalloc(
        argc as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    *buf.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
        '\u{0}' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        if len == 0 as libc::c_int as libc::c_ulong {
            cmd_free_argv(argc, *argv);
            return -(1 as libc::c_int);
        }
        arglen = strlen(buf).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let ref mut fresh4 = *(*argv).offset(i as isize);
        *fresh4 = xstrdup(buf);
        buf = buf.offset(arglen as isize);
        len = (len as libc::c_ulong).wrapping_sub(arglen) as size_t as size_t;
        i += 1
    }
    cmd_log_argv(
        argc,
        *argv,
        b"%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"cmd_unpack_argv\x00")).as_ptr(),
    );
    return 0 as libc::c_int;
}
/* Copy an argument vector, ensuring it is terminated by NULL. */
#[no_mangle]
pub unsafe extern "C" fn cmd_copy_argv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if argc == 0 as libc::c_int {
        return 0 as *mut *mut libc::c_char;
    }
    new_argv = xcalloc(
        (argc + 1 as libc::c_int) as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < argc {
        if !(*argv.offset(i as isize)).is_null() {
            let ref mut fresh5 = *new_argv.offset(i as isize);
            *fresh5 = xstrdup(*argv.offset(i as isize))
        }
        i += 1
    }
    return new_argv;
}
/* Free an argument vector. */
#[no_mangle]
pub unsafe extern "C" fn cmd_free_argv(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    if argc == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < argc {
        free(*argv.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free(argv as *mut libc::c_void);
}
/* Convert argument vector to a string. */
#[no_mangle]
pub unsafe extern "C" fn cmd_stringify_argv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut i: libc::c_int = 0;
    if argc == 0 as libc::c_int {
        return xstrdup(b"\x00" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < argc {
        s = args_escape(*argv.offset(i as isize));
        log_debug(
            b"%s: %u %s = %s\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"cmd_stringify_argv\x00"))
                .as_ptr(),
            i,
            *argv.offset(i as isize),
            s,
        );
        len = (len as libc::c_ulong)
            .wrapping_add(strlen(s).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
        buf = xrealloc(buf as *mut libc::c_void, len) as *mut libc::c_char;
        if i == 0 as libc::c_int {
            *buf = '\u{0}' as i32 as libc::c_char
        } else {
            strlcat(buf, b" \x00" as *const u8 as *const libc::c_char, len);
        }
        strlcat(buf, s, len);
        free(s as *mut libc::c_void);
        i += 1
    }
    return buf;
}
/* Get entry for command. */
#[no_mangle]
pub unsafe extern "C" fn cmd_get_entry(mut cmd: *mut cmd) -> *const cmd_entry {
    return (*cmd).entry;
}
/* Get arguments for command. */
#[no_mangle]
pub unsafe extern "C" fn cmd_get_args(mut cmd: *mut cmd) -> *mut args {
    return (*cmd).args;
}
/* Get group for command. */
#[no_mangle]
pub unsafe extern "C" fn cmd_get_group(mut cmd: *mut cmd) -> u_int {
    return (*cmd).group;
}
/* Get file and line for command. */
#[no_mangle]
pub unsafe extern "C" fn cmd_get_source(
    mut cmd: *mut cmd,
    mut file: *mut *const libc::c_char,
    mut line: *mut u_int,
) {
    if !file.is_null() {
        *file = (*cmd).file
    }
    if !line.is_null() {
        *line = (*cmd).line
    };
}
/* Look for an alias for a command. */
#[no_mangle]
pub unsafe extern "C" fn cmd_get_alias(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut a: *mut options_array_item = 0 as *mut options_array_item;
    let mut ov: *mut options_value = 0 as *mut options_value;
    let mut wanted: size_t = 0;
    let mut n: size_t = 0;
    let mut equals: *const libc::c_char = 0 as *const libc::c_char;
    o = options_get_only(
        global_options,
        b"command-alias\x00" as *const u8 as *const libc::c_char,
    );
    if o.is_null() {
        return 0 as *mut libc::c_char;
    }
    wanted = strlen(name);
    a = options_array_first(o);
    while !a.is_null() {
        ov = options_array_item_value(a);
        equals = strchr((*ov).string, '=' as i32);
        if !equals.is_null() {
            n = equals.wrapping_offset_from((*ov).string) as libc::c_long as size_t;
            if n == wanted && strncmp(name, (*ov).string, n) == 0 as libc::c_int {
                return xstrdup(equals.offset(1 as libc::c_int as isize));
            }
        }
        a = options_array_next(a)
    }
    return 0 as *mut libc::c_char;
}
/* Look up a command entry by name. */
unsafe extern "C" fn cmd_find(
    mut name: *const libc::c_char,
    mut cause: *mut *mut libc::c_char,
) -> *const cmd_entry {
    let mut loop_0: *mut *const cmd_entry = 0 as *mut *const cmd_entry;
    let mut entry: *const cmd_entry = 0 as *const cmd_entry;
    let mut found: *const cmd_entry = 0 as *const cmd_entry;
    let mut ambiguous: libc::c_int = 0;
    let mut s: [libc::c_char; 8192] = [0; 8192];
    ambiguous = 0 as libc::c_int;
    loop_0 = cmd_table.as_mut_ptr();
    while !(*loop_0).is_null() {
        entry = *loop_0;
        if !(*entry).alias.is_null() && strcmp((*entry).alias, name) == 0 as libc::c_int {
            ambiguous = 0 as libc::c_int;
            found = entry;
            break;
        } else {
            if !(strncmp((*entry).name, name, strlen(name)) != 0 as libc::c_int) {
                if !found.is_null() {
                    ambiguous = 1 as libc::c_int
                }
                found = entry;
                if strcmp((*entry).name, name) == 0 as libc::c_int {
                    break;
                }
            }
            loop_0 = loop_0.offset(1)
        }
    }
    if ambiguous != 0 {
        *s.as_mut_ptr() = '\u{0}' as i32 as libc::c_char;
        loop_0 = cmd_table.as_mut_ptr();
        while !(*loop_0).is_null() {
            entry = *loop_0;
            if !(strncmp((*entry).name, name, strlen(name)) != 0 as libc::c_int) {
                if strlcat(
                    s.as_mut_ptr(),
                    (*entry).name,
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                ) >= ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                {
                    break;
                }
                if strlcat(
                    s.as_mut_ptr(),
                    b", \x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
                ) >= ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                {
                    break;
                }
            }
            loop_0 = loop_0.offset(1)
        }
        s[strlen(s.as_mut_ptr()).wrapping_sub(2 as libc::c_int as libc::c_ulong) as usize] =
            '\u{0}' as i32 as libc::c_char;
        xasprintf(
            cause,
            b"ambiguous command: %s, could be: %s\x00" as *const u8 as *const libc::c_char,
            name,
            s.as_mut_ptr(),
        );
        return 0 as *const cmd_entry;
    } else {
        if found.is_null() {
            xasprintf(
                cause,
                b"unknown command: %s\x00" as *const u8 as *const libc::c_char,
                name,
            );
            return 0 as *const cmd_entry;
        }
        return found;
    };
}
/* Parse a single command from an argument vector. */
#[no_mangle]
pub unsafe extern "C" fn cmd_parse(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut file: *const libc::c_char,
    mut line: u_int,
    mut cause: *mut *mut libc::c_char,
) -> *mut cmd {
    let mut entry: *const cmd_entry = 0 as *const cmd_entry;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut args: *mut args = 0 as *mut args;
    if argc == 0 as libc::c_int {
        xasprintf(cause, b"no command\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut cmd;
    }
    name = *argv.offset(0 as libc::c_int as isize);
    entry = cmd_find(name, cause);
    if entry.is_null() {
        return 0 as *mut cmd;
    }
    cmd_log_argv(
        argc,
        argv,
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"cmd_parse\x00")).as_ptr(),
        (*entry).name,
    );
    args = args_parse((*entry).args.template, argc, argv);
    if !args.is_null() {
        if !((*entry).args.lower != -(1 as libc::c_int) && (*args).argc < (*entry).args.lower) {
            if !((*entry).args.upper != -(1 as libc::c_int) && (*args).argc > (*entry).args.upper) {
                cmd = xcalloc(
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<cmd>() as libc::c_ulong,
                ) as *mut cmd;
                (*cmd).entry = entry;
                (*cmd).args = args;
                if !file.is_null() {
                    (*cmd).file = xstrdup(file)
                }
                (*cmd).line = line;
                (*cmd).alias = 0 as *mut libc::c_char;
                (*cmd).argc = argc;
                (*cmd).argv = cmd_copy_argv(argc, argv);
                return cmd;
            }
        }
    }
    if !args.is_null() {
        args_free(args);
    }
    xasprintf(
        cause,
        b"usage: %s %s\x00" as *const u8 as *const libc::c_char,
        (*entry).name,
        (*entry).usage,
    );
    return 0 as *mut cmd;
}
/* Free a command. */
#[no_mangle]
pub unsafe extern "C" fn cmd_free(mut cmd: *mut cmd) {
    free((*cmd).alias as *mut libc::c_void);
    cmd_free_argv((*cmd).argc, (*cmd).argv);
    free((*cmd).file as *mut libc::c_void);
    args_free((*cmd).args);
    free(cmd as *mut libc::c_void);
}
/* Get a command as a string. */
#[no_mangle]
pub unsafe extern "C" fn cmd_print(mut cmd: *mut cmd) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = args_print((*cmd).args);
    if *s as libc::c_int != '\u{0}' as i32 {
        xasprintf(
            &mut out as *mut *mut libc::c_char,
            b"%s %s\x00" as *const u8 as *const libc::c_char,
            (*(*cmd).entry).name,
            s,
        );
    } else {
        out = xstrdup((*(*cmd).entry).name)
    }
    free(s as *mut libc::c_void);
    return out;
}
/* Create a new command list. */
#[no_mangle]
pub unsafe extern "C" fn cmd_list_new() -> *mut cmd_list {
    let mut cmdlist: *mut cmd_list = 0 as *mut cmd_list;
    cmdlist = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cmd_list>() as libc::c_ulong,
    ) as *mut cmd_list;
    (*cmdlist).references = 1 as libc::c_int;
    let fresh6 = cmd_list_next_group;
    cmd_list_next_group = cmd_list_next_group.wrapping_add(1);
    (*cmdlist).group = fresh6;
    (*cmdlist).list = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cmds>() as libc::c_ulong,
    ) as *mut cmds;
    (*(*cmdlist).list).tqh_first = 0 as *mut cmd;
    (*(*cmdlist).list).tqh_last = &mut (*(*cmdlist).list).tqh_first;
    return cmdlist;
}
/* Append a command to a command list. */
#[no_mangle]
pub unsafe extern "C" fn cmd_list_append(mut cmdlist: *mut cmd_list, mut cmd: *mut cmd) {
    (*cmd).group = (*cmdlist).group;
    (*cmd).qentry.tqe_next = 0 as *mut cmd;
    (*cmd).qentry.tqe_prev = (*(*cmdlist).list).tqh_last;
    *(*(*cmdlist).list).tqh_last = cmd;
    (*(*cmdlist).list).tqh_last = &mut (*cmd).qentry.tqe_next;
}
/* Move all commands from one command list to another */
#[no_mangle]
pub unsafe extern "C" fn cmd_list_move(mut cmdlist: *mut cmd_list, mut from: *mut cmd_list) {
    if !(*(*from).list).tqh_first.is_null() {
        *(*(*cmdlist).list).tqh_last = (*(*from).list).tqh_first;
        (*(*(*from).list).tqh_first).qentry.tqe_prev = (*(*cmdlist).list).tqh_last;
        (*(*cmdlist).list).tqh_last = (*(*from).list).tqh_last;
        (*(*from).list).tqh_first = 0 as *mut cmd;
        (*(*from).list).tqh_last = &mut (*(*from).list).tqh_first
    }
    let fresh7 = cmd_list_next_group;
    cmd_list_next_group = cmd_list_next_group.wrapping_add(1);
    (*cmdlist).group = fresh7;
}
/* Free a command list. */
#[no_mangle]
pub unsafe extern "C" fn cmd_list_free(mut cmdlist: *mut cmd_list) {
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut cmd1: *mut cmd = 0 as *mut cmd;
    (*cmdlist).references -= 1;
    if (*cmdlist).references != 0 as libc::c_int {
        return;
    }
    cmd = (*(*cmdlist).list).tqh_first;
    while !cmd.is_null() && {
        cmd1 = (*cmd).qentry.tqe_next;
        (1 as libc::c_int) != 0
    } {
        if !(*cmd).qentry.tqe_next.is_null() {
            (*(*cmd).qentry.tqe_next).qentry.tqe_prev = (*cmd).qentry.tqe_prev
        } else {
            (*(*cmdlist).list).tqh_last = (*cmd).qentry.tqe_prev
        }
        *(*cmd).qentry.tqe_prev = (*cmd).qentry.tqe_next;
        cmd_free(cmd);
        cmd = cmd1
    }
    free((*cmdlist).list as *mut libc::c_void);
    free(cmdlist as *mut libc::c_void);
}
/* Get a command list as a string. */
#[no_mangle]
pub unsafe extern "C" fn cmd_list_print(
    mut cmdlist: *mut cmd_list,
    mut escaped: libc::c_int,
) -> *mut libc::c_char {
    let mut cmd: *mut cmd = 0 as *mut cmd;
    let mut next: *mut cmd = 0 as *mut cmd;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut this: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    len = 1 as libc::c_int as size_t;
    buf = xcalloc(1 as libc::c_int as size_t, len) as *mut libc::c_char;
    cmd = (*(*cmdlist).list).tqh_first;
    while !cmd.is_null() {
        this = cmd_print(cmd);
        len = (len as libc::c_ulong)
            .wrapping_add(strlen(this).wrapping_add(6 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
        buf = xrealloc(buf as *mut libc::c_void, len) as *mut libc::c_char;
        strlcat(buf, this, len);
        next = (*cmd).qentry.tqe_next;
        if !next.is_null() {
            if (*cmd).group != (*next).group {
                if escaped != 0 {
                    strlcat(
                        buf,
                        b" \\;\\; \x00" as *const u8 as *const libc::c_char,
                        len,
                    );
                } else {
                    strlcat(buf, b" ;; \x00" as *const u8 as *const libc::c_char, len);
                }
            } else if escaped != 0 {
                strlcat(buf, b" \\; \x00" as *const u8 as *const libc::c_char, len);
            } else {
                strlcat(buf, b" ; \x00" as *const u8 as *const libc::c_char, len);
            }
        }
        free(this as *mut libc::c_void);
        cmd = (*cmd).qentry.tqe_next
    }
    return buf;
}
/* Get first command in list. */
#[no_mangle]
pub unsafe extern "C" fn cmd_list_first(mut cmdlist: *mut cmd_list) -> *mut cmd {
    return (*(*cmdlist).list).tqh_first;
}
/* Get next command in list. */
#[no_mangle]
pub unsafe extern "C" fn cmd_list_next(mut cmd: *mut cmd) -> *mut cmd {
    return (*cmd).qentry.tqe_next;
}
/* Do all of the commands in this command list have this flag? */
#[no_mangle]
pub unsafe extern "C" fn cmd_list_all_have(
    mut cmdlist: *mut cmd_list,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut cmd: *mut cmd = 0 as *mut cmd;
    cmd = (*(*cmdlist).list).tqh_first;
    while !cmd.is_null() {
        if !(*(*cmd).entry).flags & flag != 0 {
            return 0 as libc::c_int;
        }
        cmd = (*cmd).qentry.tqe_next
    }
    return 1 as libc::c_int;
}
/* Do any of the commands in this command list have this flag? */
#[no_mangle]
pub unsafe extern "C" fn cmd_list_any_have(
    mut cmdlist: *mut cmd_list,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut cmd: *mut cmd = 0 as *mut cmd;
    cmd = (*(*cmdlist).list).tqh_first;
    while !cmd.is_null() {
        if (*(*cmd).entry).flags & flag != 0 {
            return 1 as libc::c_int;
        }
        cmd = (*cmd).qentry.tqe_next
    }
    return 0 as libc::c_int;
}
/* Adjust current mouse position for a pane. */
#[no_mangle]
pub unsafe extern "C" fn cmd_mouse_at(
    mut wp: *mut window_pane,
    mut m: *mut mouse_event,
    mut xp: *mut u_int,
    mut yp: *mut u_int,
    mut last: libc::c_int,
) -> libc::c_int {
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    if last != 0 {
        x = (*m).lx.wrapping_add((*m).ox);
        y = (*m).ly.wrapping_add((*m).oy)
    } else {
        x = (*m).x.wrapping_add((*m).ox);
        y = (*m).y.wrapping_add((*m).oy)
    }
    log_debug(
        b"%s: x=%u, y=%u%s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"cmd_mouse_at\x00")).as_ptr(),
        x,
        y,
        if last != 0 {
            b" (last)\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
    );
    if (*m).statusat == 0 as libc::c_int && y >= (*m).statuslines {
        y = (y as libc::c_uint).wrapping_sub((*m).statuslines) as u_int as u_int
    }
    if x < (*wp).xoff || x >= (*wp).xoff.wrapping_add((*wp).sx) {
        return -(1 as libc::c_int);
    }
    if y < (*wp).yoff || y >= (*wp).yoff.wrapping_add((*wp).sy) {
        return -(1 as libc::c_int);
    }
    if !xp.is_null() {
        *xp = x.wrapping_sub((*wp).xoff)
    }
    if !yp.is_null() {
        *yp = y.wrapping_sub((*wp).yoff)
    }
    return 0 as libc::c_int;
}
/* Get current mouse window if any. */
#[no_mangle]
pub unsafe extern "C" fn cmd_mouse_window(
    mut m: *mut mouse_event,
    mut sp: *mut *mut session,
) -> *mut winlink {
    let mut s: *mut session = 0 as *mut session;
    let mut w: *mut window = 0 as *mut window;
    let mut wl: *mut winlink = 0 as *mut winlink;
    if (*m).valid == 0 {
        return 0 as *mut winlink;
    }
    if (*m).s == -(1 as libc::c_int) || {
        s = session_find_by_id((*m).s as u_int);
        s.is_null()
    } {
        return 0 as *mut winlink;
    }
    if (*m).w == -(1 as libc::c_int) {
        wl = (*s).curw
    } else {
        w = window_find_by_id((*m).w as u_int);
        if w.is_null() {
            return 0 as *mut winlink;
        }
        wl = winlink_find_by_window(&mut (*s).windows, w)
    }
    if !sp.is_null() {
        *sp = s
    }
    return wl;
}
/* Get current mouse pane if any. */
#[no_mangle]
pub unsafe extern "C" fn cmd_mouse_pane(
    mut m: *mut mouse_event,
    mut sp: *mut *mut session,
    mut wlp: *mut *mut winlink,
) -> *mut window_pane {
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    wl = cmd_mouse_window(m, sp);
    if wl.is_null() {
        return 0 as *mut window_pane;
    }
    wp = window_pane_find_by_id((*m).wp as u_int);
    if wp.is_null() {
        return 0 as *mut window_pane;
    }
    if window_has_pane((*wl).window, wp) == 0 {
        return 0 as *mut window_pane;
    }
    if !wlp.is_null() {
        *wlp = wl
    }
    return wp;
}
/* Replace the first %% or %idx in template by s. */
#[no_mangle]
pub unsafe extern "C" fn cmd_template_replace(
    mut template: *const libc::c_char,
    mut s: *const libc::c_char,
    mut idx: libc::c_int,
) -> *mut libc::c_char {
    let mut ch: libc::c_char = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let quote: [libc::c_char; 6] =
        *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"\"\\$;~\x00");
    let mut replaced: libc::c_int = 0;
    let mut quoted: libc::c_int = 0;
    let mut len: size_t = 0;
    if strchr(template, '%' as i32).is_null() {
        return xstrdup(template);
    }
    buf = xmalloc(1 as libc::c_int as size_t) as *mut libc::c_char;
    *buf = '\u{0}' as i32 as libc::c_char;
    len = 0 as libc::c_int as size_t;
    replaced = 0 as libc::c_int;
    ptr = template;
    let mut current_block_22: u64;
    while *ptr as libc::c_int != '\u{0}' as i32 {
        let fresh8 = ptr;
        ptr = ptr.offset(1);
        ch = *fresh8;
        match ch as libc::c_int {
            37 => {
                if (*ptr as libc::c_int) < '1' as i32
                    || *ptr as libc::c_int > '9' as i32
                    || *ptr as libc::c_int - '0' as i32 != idx
                {
                    if *ptr as libc::c_int != '%' as i32 || replaced != 0 {
                        current_block_22 = 16203760046146113240;
                    } else {
                        replaced = 1 as libc::c_int;
                        current_block_22 = 10599921512955367680;
                    }
                } else {
                    current_block_22 = 10599921512955367680;
                }
                match current_block_22 {
                    16203760046146113240 => {}
                    _ => {
                        ptr = ptr.offset(1);
                        quoted = (*ptr as libc::c_int == '%' as i32) as libc::c_int;
                        if quoted != 0 {
                            ptr = ptr.offset(1)
                        }
                        buf = xrealloc(
                            buf as *mut libc::c_void,
                            len.wrapping_add(
                                strlen(s).wrapping_mul(3 as libc::c_int as libc::c_ulong),
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as *mut libc::c_char;
                        cp = s;
                        while *cp as libc::c_int != '\u{0}' as i32 {
                            if quoted != 0 && !strchr(quote.as_ptr(), *cp as libc::c_int).is_null()
                            {
                                let fresh9 = len;
                                len = len.wrapping_add(1);
                                *buf.offset(fresh9 as isize) = '\\' as i32 as libc::c_char
                            }
                            let fresh10 = len;
                            len = len.wrapping_add(1);
                            *buf.offset(fresh10 as isize) = *cp;
                            cp = cp.offset(1)
                        }
                        *buf.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
                        continue;
                    }
                }
            }
            _ => {}
        }
        buf = xrealloc(
            buf as *mut libc::c_void,
            len.wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        let fresh11 = len;
        len = len.wrapping_add(1);
        *buf.offset(fresh11 as isize) = ch;
        *buf.offset(len as isize) = '\u{0}' as i32 as libc::c_char
    }
    return buf;
}
