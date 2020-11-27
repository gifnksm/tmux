use crate::key_code::code as key_code_code;
use ::libc;

extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn event_add(ev: *mut event, timeout: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn event_del(_: *mut event) -> libc::c_int;
    #[no_mangle]
    fn event_pending(ev: *const event, events: libc::c_short, tv: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn event_initialized(ev: *const event) -> libc::c_int;
    #[no_mangle]
    fn event_once(
        _: libc::c_int,
        _: libc::c_short,
        _: Option<
            unsafe extern "C" fn(_: libc::c_int, _: libc::c_short, _: *mut libc::c_void) -> (),
        >,
        _: *mut libc::c_void,
        _: *const timeval,
    ) -> libc::c_int;
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
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_enable(bufev: *mut bufferevent, event: libc::c_short) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_disable(bufev: *mut bufferevent, event: libc::c_short) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_add(buf: *mut evbuffer, data: *const libc::c_void, datlen: size_t) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strsep(__stringp: *mut *mut libc::c_char, __delim: *const libc::c_char)
        -> *mut libc::c_char;
    #[no_mangle]
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut crate::options::options;
    #[no_mangle]
    static mut global_s_options: *mut crate::options::options;
    #[no_mangle]
    fn checkshell(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn setblocking(_: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn find_home() -> *const libc::c_char;
    #[no_mangle]
    fn proc_send(
        _: *mut crate::proc::tmuxpeer,
        _: msgtype,
        _: libc::c_int,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn proc_add_peer(
        _: *mut crate::proc::tmuxproc,
        _: libc::c_int,
        _: Option<unsafe extern "C" fn(_: *mut imsg, _: *mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> *mut crate::proc::tmuxpeer;
    #[no_mangle]
    fn proc_remove_peer(_: *mut crate::proc::tmuxpeer);
    #[no_mangle]
    fn proc_kill_peer(_: *mut crate::proc::tmuxpeer);
    #[no_mangle]
    static mut cfg_finished: libc::c_int;
    #[no_mangle]
    static mut cfg_client: *mut client;
    #[no_mangle]
    fn start_cfg();
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
    fn format_expand_time(
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
    fn format_lost_client(_: *mut client);
    #[no_mangle]
    fn notify_client(_: *const libc::c_char, _: *mut client);
    #[no_mangle]
    fn notify_pane(_: *const libc::c_char, _: *mut window_pane);
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
    fn environ_create() -> *mut crate::environ::environ;
    #[no_mangle]
    fn environ_free(_: *mut crate::environ::environ);
    #[no_mangle]
    fn environ_find(_: *mut crate::environ::environ, _: *const libc::c_char) -> *mut environ_entry;
    #[no_mangle]
    fn environ_put(_: *mut crate::environ::environ, _: *const libc::c_char, _: libc::c_int);
    #[no_mangle]
    fn tty_window_offset(
        _: *mut tty,
        _: *mut u_int,
        _: *mut u_int,
        _: *mut u_int,
        _: *mut u_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn tty_reset(_: *mut tty);
    #[no_mangle]
    fn tty_region_off(_: *mut tty);
    #[no_mangle]
    fn tty_margin_off(_: *mut tty);
    #[no_mangle]
    fn tty_cursor(_: *mut tty, _: u_int, _: u_int);
    #[no_mangle]
    fn tty_init(_: *mut tty, _: *mut client) -> libc::c_int;
    #[no_mangle]
    fn tty_resize(_: *mut tty);
    #[no_mangle]
    fn tty_start_tty(_: *mut tty);
    #[no_mangle]
    fn tty_send_requests(_: *mut tty);
    #[no_mangle]
    fn tty_stop_tty(_: *mut tty);
    #[no_mangle]
    fn tty_set_title(_: *mut tty, _: *const libc::c_char);
    #[no_mangle]
    fn tty_update_mode(_: *mut tty, _: libc::c_int, _: *mut screen);
    #[no_mangle]
    fn tty_sync_end(_: *mut tty);
    #[no_mangle]
    fn tty_open(_: *mut tty, _: *mut *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tty_close(_: *mut tty);
    #[no_mangle]
    fn tty_free(_: *mut tty);
    #[no_mangle]
    fn tty_get_features(_: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_find_from_client(_: *mut cmd_find_state, _: *mut client, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_from_mouse(
        _: *mut cmd_find_state,
        _: *mut mouse_event,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn cmd_unpack_argv(
        _: *mut libc::c_char,
        _: size_t,
        _: libc::c_int,
        _: *mut *mut *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char);
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list);
    #[no_mangle]
    fn cmd_parse_from_arguments(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut cmd_parse_input,
    ) -> *mut cmd_parse_result;
    #[no_mangle]
    fn cmdq_new() -> *mut crate::cmd_queue::cmdq_list;
    #[no_mangle]
    fn cmdq_free(_: *mut crate::cmd_queue::cmdq_list);
    #[no_mangle]
    fn cmdq_get_client(_: *mut crate::cmd_queue::cmdq_item) -> *mut client;
    #[no_mangle]
    fn cmdq_get_command(
        _: *mut cmd_list,
        _: *mut crate::cmd_queue::cmdq_state,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_get_callback1(
        _: *const libc::c_char,
        _: cmdq_cb,
        _: *mut libc::c_void,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_get_error(_: *const libc::c_char) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_append(
        _: *mut client,
        _: *mut crate::cmd_queue::cmdq_item,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn key_bindings_get_table(_: *const libc::c_char, _: libc::c_int) -> *mut key_table;
    #[no_mangle]
    fn key_bindings_unref_table(_: *mut key_table);
    #[no_mangle]
    fn key_bindings_get(_: *mut key_table, _: key_code) -> *mut key_binding;
    #[no_mangle]
    fn key_bindings_dispatch(
        _: *mut key_binding,
        _: *mut crate::cmd_queue::cmdq_item,
        _: *mut client,
        _: *mut key_event,
        _: *mut cmd_find_state,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn server_status_client(_: *mut client);
    #[no_mangle]
    fn status_get_range(_: *mut client, _: u_int, _: u_int) -> *mut style_range;
    #[no_mangle]
    fn status_line_size(_: *mut client) -> u_int;
    #[no_mangle]
    fn status_at_line(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn recalculate_size(_: *mut window, _: libc::c_int);
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    fn status_prompt_key(_: *mut client, _: key_code) -> libc::c_int;
    #[no_mangle]
    fn status_message_clear(_: *mut client);
    #[no_mangle]
    fn status_init(_: *mut client);
    #[no_mangle]
    fn server_redraw_client(_: *mut client);
    #[no_mangle]
    fn recalculate_sizes();
    #[no_mangle]
    fn server_check_unattached();
    #[no_mangle]
    fn status_free(_: *mut client);
    #[no_mangle]
    fn status_prompt_clear(_: *mut client);
    #[no_mangle]
    fn server_add_accept(_: libc::c_int);
    #[no_mangle]
    fn server_update_socket();
    #[no_mangle]
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    #[no_mangle]
    static mut clients: clients;
    #[no_mangle]
    static mut server_proc: *mut crate::proc::tmuxproc;
    #[no_mangle]
    fn file_push(_: *mut client_file);
    #[no_mangle]
    fn file_fire_read(_: *mut client_file);
    #[no_mangle]
    fn file_fire_done(_: *mut client_file);
    #[no_mangle]
    fn client_files_RB_MINMAX(_: *mut client_files, _: libc::c_int) -> *mut client_file;
    #[no_mangle]
    fn screen_redraw_screen(_: *mut client);
    #[no_mangle]
    fn client_files_RB_NEXT(_: *mut client_file) -> *mut client_file;
    #[no_mangle]
    fn screen_redraw_pane(_: *mut client, _: *mut window_pane);
    #[no_mangle]
    fn client_files_RB_FIND(_: *mut client_files, _: *mut client_file) -> *mut client_file;
    #[no_mangle]
    fn resize_window(_: *mut window, _: u_int, _: u_int, _: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn window_get_active_at(_: *mut window, _: u_int, _: u_int) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_key(
        _: *mut window_pane,
        _: *mut client,
        _: *mut session,
        _: *mut winlink,
        _: key_code,
        _: *mut mouse_event,
    ) -> libc::c_int;
    #[no_mangle]
    fn winlink_find_by_index(_: *mut winlinks, _: libc::c_int) -> *mut winlink;
    #[no_mangle]
    fn window_pane_tree_RB_MINMAX(_: *mut window_pane_tree, _: libc::c_int) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_tree_RB_NEXT(_: *mut window_pane) -> *mut window_pane;
    #[no_mangle]
    fn window_pane_send_resize(_: *mut window_pane, _: libc::c_int);
    #[no_mangle]
    fn window_pane_get_new_data(
        _: *mut window_pane,
        _: *mut window_pane_offset,
        _: *mut size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn control_reset_offsets(_: *mut client);
    #[no_mangle]
    fn session_update_activity(_: *mut session, _: *mut timeval);
    #[no_mangle]
    fn control_start(_: *mut client);
    #[no_mangle]
    fn control_stop(_: *mut client);
    #[no_mangle]
    fn check_window_name(_: *mut window);
    #[no_mangle]
    fn control_pane_offset(
        _: *mut client,
        _: *mut window_pane,
        _: *mut libc::c_int,
    ) -> *mut window_pane_offset;
    #[no_mangle]
    fn control_all_done(_: *mut client) -> libc::c_int;
    #[no_mangle]
    fn control_discard(_: *mut client);
    #[no_mangle]
    fn log_get_level() -> libc::c_int;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

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
pub struct imsg_hdr {
    pub type_0: uint32_t,
    pub len: uint16_t,
    pub flags: uint16_t,
    pub peerid: uint32_t,
    pub pid: uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imsg {
    pub hdr: imsg_hdr,
    pub fd: libc::c_int,
    pub data: *mut libc::c_void,
}

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
    pub data: utf8_data,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct utf8_data {
    pub data: [u_char; 21],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
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
    pub data: utf8_char,
    pub attr: u_short,
    pub flags: u_char,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub us: libc::c_int,
}
pub type utf8_char = u_int;

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
pub struct msg_command {
    pub argc: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_read_data {
    pub stream: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_read_done {
    pub stream: libc::c_int,
    pub error: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_write_ready {
    pub stream: libc::c_int,
    pub error: libc::c_int,
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
pub type cmdq_cb = Option<
    unsafe extern "C" fn(_: *mut crate::cmd_queue::cmdq_item, _: *mut libc::c_void) -> cmd_retval,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
pub const BORDER: C2RustUnnamed_34 = 6;
pub type C2RustUnnamed_34 = libc::c_uint;
pub const STATUS_DEFAULT: C2RustUnnamed_34 = 5;
pub const STATUS_RIGHT: C2RustUnnamed_34 = 4;
pub const STATUS_LEFT: C2RustUnnamed_34 = 3;
pub const STATUS: C2RustUnnamed_34 = 2;
pub const PANE: C2RustUnnamed_34 = 1;
pub const NOWHERE: C2RustUnnamed_34 = 0;
pub const TRIPLE: C2RustUnnamed_35 = 8;
pub const DOUBLE: C2RustUnnamed_35 = 7;
pub const SECOND: C2RustUnnamed_35 = 6;
pub const DOWN: C2RustUnnamed_35 = 2;
pub const UP: C2RustUnnamed_35 = 3;
pub const WHEEL: C2RustUnnamed_35 = 5;
pub const DRAG: C2RustUnnamed_35 = 4;
pub const MOVE: C2RustUnnamed_35 = 1;
pub const NOTYPE: C2RustUnnamed_35 = 0;
pub type C2RustUnnamed_35 = libc::c_uint;
/* Compare client windows. */
unsafe extern "C" fn server_client_window_cmp(
    mut cw1: *mut client_window,
    mut cw2: *mut client_window,
) -> libc::c_int {
    if (*cw1).window < (*cw2).window {
        return -(1 as libc::c_int);
    }
    if (*cw1).window > (*cw2).window {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn client_windows_RB_NFIND(
    mut head: *mut client_windows,
    mut elm: *mut client_window,
) -> *mut client_window {
    let mut tmp: *mut client_window = (*head).rbh_root;
    let mut res: *mut client_window = 0 as *mut client_window;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = server_client_window_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            res = tmp;
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn client_windows_RB_INSERT(
    mut head: *mut client_windows,
    mut elm: *mut client_window,
) -> *mut client_window {
    let mut tmp: *mut client_window = 0 as *mut client_window;
    let mut parent: *mut client_window = 0 as *mut client_window;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = server_client_window_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut client_window;
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
    client_windows_RB_INSERT_COLOR(head, elm);
    return 0 as *mut client_window;
}
#[no_mangle]
pub unsafe extern "C" fn client_windows_RB_INSERT_COLOR(
    mut head: *mut client_windows,
    mut elm: *mut client_window,
) {
    let mut parent: *mut client_window = 0 as *mut client_window;
    let mut gparent: *mut client_window = 0 as *mut client_window;
    let mut tmp: *mut client_window = 0 as *mut client_window;
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
#[no_mangle]
pub unsafe extern "C" fn client_windows_RB_FIND(
    mut head: *mut client_windows,
    mut elm: *mut client_window,
) -> *mut client_window {
    let mut tmp: *mut client_window = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = server_client_window_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right
        } else {
            return tmp;
        }
    }
    return 0 as *mut client_window;
}
#[no_mangle]
pub unsafe extern "C" fn client_windows_RB_MINMAX(
    mut head: *mut client_windows,
    mut val: libc::c_int,
) -> *mut client_window {
    let mut tmp: *mut client_window = (*head).rbh_root;
    let mut parent: *mut client_window = 0 as *mut client_window;
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
#[no_mangle]
pub unsafe extern "C" fn client_windows_RB_PREV(mut elm: *mut client_window) -> *mut client_window {
    if !(*elm).entry.rbe_left.is_null() {
        elm = (*elm).entry.rbe_left;
        while !(*elm).entry.rbe_right.is_null() {
            elm = (*elm).entry.rbe_right
        }
    } else if !(*elm).entry.rbe_parent.is_null()
        && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
    {
        elm = (*elm).entry.rbe_parent
    } else {
        while !(*elm).entry.rbe_parent.is_null() && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
        {
            elm = (*elm).entry.rbe_parent
        }
        elm = (*elm).entry.rbe_parent
    }
    return elm;
}
#[no_mangle]
pub unsafe extern "C" fn client_windows_RB_NEXT(mut elm: *mut client_window) -> *mut client_window {
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
#[no_mangle]
pub unsafe extern "C" fn client_windows_RB_REMOVE(
    mut head: *mut client_windows,
    mut elm: *mut client_window,
) -> *mut client_window {
    let mut current_block: u64;
    let mut child: *mut client_window = 0 as *mut client_window;
    let mut parent: *mut client_window = 0 as *mut client_window;
    let mut old: *mut client_window = elm;
    let mut color: libc::c_int = 0;
    if (*elm).entry.rbe_left.is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7226443171521532240;
    } else if (*elm).entry.rbe_right.is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7226443171521532240;
    } else {
        let mut left: *mut client_window = 0 as *mut client_window;
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
        current_block = 1839618019415382039;
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
        client_windows_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn client_windows_RB_REMOVE_COLOR(
    mut head: *mut client_windows,
    mut parent: *mut client_window,
    mut elm: *mut client_window,
) {
    let mut tmp: *mut client_window = 0 as *mut client_window;
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
                    let mut oleft: *mut client_window = 0 as *mut client_window;
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
                    let mut oright: *mut client_window = 0 as *mut client_window;
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
/* Number of attached clients. */
#[no_mangle]
pub unsafe extern "C" fn server_client_how_many() -> u_int {
    let mut c: *mut client = 0 as *mut client;
    let mut n: u_int = 0;
    n = 0 as libc::c_int as u_int;
    c = clients.tqh_first;
    while !c.is_null() {
        if !(*c).session.is_null()
            && !(*c).flags
                & (0x200 as libc::c_int | 0x40 as libc::c_int | 0x4 as libc::c_int) as libc::c_ulong
                != 0
        {
            n = n.wrapping_add(1)
        }
        c = (*c).entry.tqe_next
    }
    return n;
}
/* Overlay timer callback. */
unsafe extern "C" fn server_client_overlay_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    server_client_clear_overlay(data as *mut client);
}
/* Set an overlay on client. */
#[no_mangle]
pub unsafe extern "C" fn server_client_set_overlay(
    mut c: *mut client,
    mut delay: u_int,
    mut checkcb: overlay_check_cb,
    mut modecb: overlay_mode_cb,
    mut drawcb: overlay_draw_cb,
    mut keycb: overlay_key_cb,
    mut freecb: overlay_free_cb,
    mut data: *mut libc::c_void,
) {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if (*c).overlay_draw.is_some() {
        server_client_clear_overlay(c);
    }
    tv.tv_sec = delay.wrapping_div(1000 as libc::c_int as libc::c_uint) as __time_t;
    tv.tv_usec = delay.wrapping_rem(1000 as libc::c_int as libc::c_uint) as libc::c_long
        * 1000 as libc::c_long;
    if event_initialized(&mut (*c).overlay_timer) != 0 {
        event_del(&mut (*c).overlay_timer);
    }
    event_set(
        &mut (*c).overlay_timer,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            server_client_overlay_timer
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        c as *mut libc::c_void,
    );
    if delay != 0 as libc::c_int as libc::c_uint {
        event_add(&mut (*c).overlay_timer, &mut tv);
    }
    (*c).overlay_check = checkcb;
    (*c).overlay_mode = modecb;
    (*c).overlay_draw = drawcb;
    (*c).overlay_key = keycb;
    (*c).overlay_free = freecb;
    (*c).overlay_data = data;
    (*c).tty.flags |= 0x2 as libc::c_int;
    if (*c).overlay_mode.is_none() {
        (*c).tty.flags |= 0x1 as libc::c_int
    }
    server_redraw_client(c);
}
/* Clear overlay mode on client. */
#[no_mangle]
pub unsafe extern "C" fn server_client_clear_overlay(mut c: *mut client) {
    if (*c).overlay_draw.is_none() {
        return;
    }
    if event_initialized(&mut (*c).overlay_timer) != 0 {
        event_del(&mut (*c).overlay_timer);
    }
    if (*c).overlay_free.is_some() {
        (*c).overlay_free.expect("non-null function pointer")(c);
    }
    (*c).overlay_check = None;
    (*c).overlay_mode = None;
    (*c).overlay_draw = None;
    (*c).overlay_key = None;
    (*c).overlay_free = None;
    (*c).overlay_data = 0 as *mut libc::c_void;
    (*c).tty.flags &= !(0x2 as libc::c_int | 0x1 as libc::c_int);
    server_redraw_client(c);
}
/* Check if this client is inside this server. */
#[no_mangle]
pub unsafe extern "C" fn server_client_check_nested(mut c: *mut client) -> libc::c_int {
    let mut envent: *mut environ_entry = 0 as *mut environ_entry;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    envent = environ_find(
        (*c).environ,
        b"TMUX\x00" as *const u8 as *const libc::c_char,
    );
    if envent.is_null() || *(*envent).value as libc::c_int == '\u{0}' as i32 {
        return 0 as libc::c_int;
    }
    wp = window_pane_tree_RB_MINMAX(&mut all_window_panes, -(1 as libc::c_int));
    while !wp.is_null() {
        if strcmp((*wp).tty.as_mut_ptr(), (*c).ttyname) == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        wp = window_pane_tree_RB_NEXT(wp)
    }
    return 0 as libc::c_int;
}
/* Set client key table. */
#[no_mangle]
pub unsafe extern "C" fn server_client_set_key_table(
    mut c: *mut client,
    mut name: *const libc::c_char,
) {
    if name.is_null() {
        name = server_client_get_key_table(c)
    }
    key_bindings_unref_table((*c).keytable);
    (*c).keytable = key_bindings_get_table(name, 1 as libc::c_int);
    (*(*c).keytable).references = (*(*c).keytable).references.wrapping_add(1);
}
/* Get default key table. */
#[no_mangle]
pub unsafe extern "C" fn server_client_get_key_table(mut c: *mut client) -> *const libc::c_char {
    let mut s: *mut session = (*c).session;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if s.is_null() {
        return b"root\x00" as *const u8 as *const libc::c_char;
    }
    name = options_get_string(
        (*s).options,
        b"key-table\x00" as *const u8 as *const libc::c_char,
    );
    if *name as libc::c_int == '\u{0}' as i32 {
        return b"root\x00" as *const u8 as *const libc::c_char;
    }
    return name;
}
/* Is this table the default key table? */
unsafe extern "C" fn server_client_is_default_key_table(
    mut c: *mut client,
    mut table: *mut key_table,
) -> libc::c_int {
    return (strcmp((*table).name, server_client_get_key_table(c)) == 0 as libc::c_int)
        as libc::c_int;
}
/* Create a new client. */
#[no_mangle]
pub unsafe extern "C" fn server_client_create(mut fd: libc::c_int) -> *mut client {
    let mut c: *mut client = 0 as *mut client;
    setblocking(fd, 0 as libc::c_int);
    c = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<client>() as libc::c_ulong,
    ) as *mut client;
    (*c).references = 1 as libc::c_int;
    (*c).peer = proc_add_peer(
        server_proc,
        fd,
        Some(
            server_client_dispatch
                as unsafe extern "C" fn(_: *mut imsg, _: *mut libc::c_void) -> (),
        ),
        c as *mut libc::c_void,
    );
    if gettimeofday(&mut (*c).creation_time, 0 as *mut libc::c_void) != 0 as libc::c_int {
        fatal(b"gettimeofday failed\x00" as *const u8 as *const libc::c_char);
    }
    memcpy(
        &mut (*c).activity_time as *mut timeval as *mut libc::c_void,
        &mut (*c).creation_time as *mut timeval as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong,
    );
    (*c).environ = environ_create();
    (*c).fd = -(1 as libc::c_int);
    (*c).out_fd = -(1 as libc::c_int);
    (*c).queue = cmdq_new();
    (*c).windows.rbh_root = 0 as *mut client_window;
    (*c).files.rbh_root = 0 as *mut client_file;
    (*c).tty.sx = 80 as libc::c_int as u_int;
    (*c).tty.sy = 24 as libc::c_int as u_int;
    status_init(c);
    (*c).flags |= 0x8000 as libc::c_int as libc::c_ulong;
    (*c).keytable = key_bindings_get_table(
        b"root\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    (*(*c).keytable).references = (*(*c).keytable).references.wrapping_add(1);
    event_set(
        &mut (*c).repeat_timer,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            server_client_repeat_timer
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        c as *mut libc::c_void,
    );
    event_set(
        &mut (*c).click_timer,
        -(1 as libc::c_int),
        0 as libc::c_int as libc::c_short,
        Some(
            server_client_click_timer
                as unsafe extern "C" fn(
                    _: libc::c_int,
                    _: libc::c_short,
                    _: *mut libc::c_void,
                ) -> (),
        ),
        c as *mut libc::c_void,
    );
    (*c).entry.tqe_next = 0 as *mut client;
    (*c).entry.tqe_prev = clients.tqh_last;
    *clients.tqh_last = c;
    clients.tqh_last = &mut (*c).entry.tqe_next;
    log_debug(b"new client %p\x00" as *const u8 as *const libc::c_char, c);
    return c;
}
/* Open client terminal if needed. */
#[no_mangle]
pub unsafe extern "C" fn server_client_open(
    mut c: *mut client,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ttynam: *const libc::c_char = b"/dev/tty\x00" as *const u8 as *const libc::c_char;
    if (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
        return 0 as libc::c_int;
    }
    if strcmp((*c).ttyname, ttynam) == 0 as libc::c_int
        || (isatty(0 as libc::c_int) != 0
            && {
                ttynam = ttyname(0 as libc::c_int);
                !ttynam.is_null()
            }
            && strcmp((*c).ttyname, ttynam) == 0 as libc::c_int
            || isatty(1 as libc::c_int) != 0
                && {
                    ttynam = ttyname(1 as libc::c_int);
                    !ttynam.is_null()
                }
                && strcmp((*c).ttyname, ttynam) == 0 as libc::c_int
            || isatty(2 as libc::c_int) != 0
                && {
                    ttynam = ttyname(2 as libc::c_int);
                    !ttynam.is_null()
                }
                && strcmp((*c).ttyname, ttynam) == 0 as libc::c_int)
    {
        xasprintf(
            cause,
            b"can\'t use %s\x00" as *const u8 as *const libc::c_char,
            (*c).ttyname,
        );
        return -(1 as libc::c_int);
    }
    if (*c).flags & 0x1 as libc::c_int as libc::c_ulong == 0 {
        *cause = xstrdup(b"not a terminal\x00" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if tty_open(&mut (*c).tty, cause) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/* Lost a client. */
#[no_mangle]
pub unsafe extern "C" fn server_client_lost(mut c: *mut client) {
    let mut cf: *mut client_file = 0 as *mut client_file; /* may be more file descriptors now */
    let mut cf1: *mut client_file = 0 as *mut client_file;
    let mut cw: *mut client_window = 0 as *mut client_window;
    let mut cw1: *mut client_window = 0 as *mut client_window;
    (*c).flags |= 0x200 as libc::c_int as libc::c_ulong;
    server_client_clear_overlay(c);
    status_prompt_clear(c);
    status_message_clear(c);
    cf = client_files_RB_MINMAX(&mut (*c).files, -(1 as libc::c_int));
    while !cf.is_null() && {
        cf1 = client_files_RB_NEXT(cf);
        (1 as libc::c_int) != 0
    } {
        (*cf).error = 4 as libc::c_int;
        file_fire_done(cf);
        cf = cf1
    }
    cw = client_windows_RB_MINMAX(&mut (*c).windows, -(1 as libc::c_int));
    while !cw.is_null() && {
        cw1 = client_windows_RB_NEXT(cw);
        (1 as libc::c_int) != 0
    } {
        client_windows_RB_REMOVE(&mut (*c).windows, cw);
        free(cw as *mut libc::c_void);
        cw = cw1
    }
    if !(*c).entry.tqe_next.is_null() {
        (*(*c).entry.tqe_next).entry.tqe_prev = (*c).entry.tqe_prev
    } else {
        clients.tqh_last = (*c).entry.tqe_prev
    }
    *(*c).entry.tqe_prev = (*c).entry.tqe_next;
    log_debug(b"lost client %p\x00" as *const u8 as *const libc::c_char, c);
    if (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
        control_stop(c);
    }
    if (*c).flags & 0x1 as libc::c_int as libc::c_ulong != 0 {
        tty_free(&mut (*c).tty);
    }
    free((*c).ttyname as *mut libc::c_void);
    free((*c).term_name as *mut libc::c_void);
    free((*c).term_type as *mut libc::c_void);
    status_free(c);
    free((*c).title as *mut libc::c_void);
    free((*c).cwd as *mut libc::c_void);
    event_del(&mut (*c).repeat_timer);
    event_del(&mut (*c).click_timer);
    key_bindings_unref_table((*c).keytable);
    free((*c).message_string as *mut libc::c_void);
    if event_initialized(&mut (*c).message_timer) != 0 {
        event_del(&mut (*c).message_timer);
    }
    free((*c).prompt_saved as *mut libc::c_void);
    free((*c).prompt_string as *mut libc::c_void);
    free((*c).prompt_buffer as *mut libc::c_void);
    format_lost_client(c);
    environ_free((*c).environ);
    proc_remove_peer((*c).peer);
    (*c).peer = 0 as *mut crate::proc::tmuxpeer;
    if (*c).out_fd != -(1 as libc::c_int) {
        close((*c).out_fd);
    }
    if (*c).fd != -(1 as libc::c_int) {
        close((*c).fd);
        (*c).fd = -(1 as libc::c_int)
    }
    server_client_unref(c);
    server_add_accept(0 as libc::c_int);
    recalculate_sizes();
    server_check_unattached();
    server_update_socket();
}
/* Remove reference from a client. */
#[no_mangle]
pub unsafe extern "C" fn server_client_unref(mut c: *mut client) {
    log_debug(
        b"unref client %p (%d references)\x00" as *const u8 as *const libc::c_char,
        c,
        (*c).references,
    );
    (*c).references -= 1;
    if (*c).references == 0 as libc::c_int {
        event_once(
            -(1 as libc::c_int),
            0x1 as libc::c_int as libc::c_short,
            Some(
                server_client_free
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            c as *mut libc::c_void,
            0 as *const timeval,
        );
    };
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
/* Free dead client. */
unsafe extern "C" fn server_client_free(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut c: *mut client = arg as *mut client;
    log_debug(
        b"free client %p (%d references)\x00" as *const u8 as *const libc::c_char,
        c,
        (*c).references,
    );
    cmdq_free((*c).queue);
    if (*c).references == 0 as libc::c_int {
        free((*c).name as *mut libc::c_void);
        free(c as *mut libc::c_void);
    };
}
/* Suspend a client. */
#[no_mangle]
pub unsafe extern "C" fn server_client_suspend(mut c: *mut client) {
    let mut s: *mut session = (*c).session;
    if s.is_null()
        || (*c).flags
            & (0x200 as libc::c_int | 0x40 as libc::c_int | 0x4 as libc::c_int) as libc::c_ulong
            != 0
    {
        return;
    }
    tty_stop_tty(&mut (*c).tty);
    (*c).flags |= 0x40 as libc::c_int as libc::c_ulong;
    proc_send(
        (*c).peer,
        MSG_SUSPEND,
        -(1 as libc::c_int),
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
}
/* Detach a client. */
#[no_mangle]
pub unsafe extern "C" fn server_client_detach(mut c: *mut client, mut msgtype: msgtype) {
    let mut s: *mut session = (*c).session;
    if s.is_null()
        || (*c).flags
            & (0x200 as libc::c_int | 0x40 as libc::c_int | 0x4 as libc::c_int) as libc::c_ulong
            != 0
    {
        return;
    }
    (*c).flags |= 0x4 as libc::c_int as libc::c_ulong;
    (*c).exit_type = CLIENT_EXIT_DETACH;
    (*c).exit_msgtype = msgtype;
    (*c).exit_session = xstrdup((*s).name);
}
/* Execute command to replace a client. */
#[no_mangle]
pub unsafe extern "C" fn server_client_exec(mut c: *mut client, mut cmd: *const libc::c_char) {
    let mut s: *mut session = (*c).session;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmdsize: size_t = 0;
    let mut shellsize: size_t = 0;
    if *cmd as libc::c_int == '\u{0}' as i32 {
        return;
    }
    cmdsize = strlen(cmd).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if !s.is_null() {
        shell = options_get_string(
            (*s).options,
            b"default-shell\x00" as *const u8 as *const libc::c_char,
        )
    } else {
        shell = options_get_string(
            global_s_options,
            b"default-shell\x00" as *const u8 as *const libc::c_char,
        )
    }
    if checkshell(shell) == 0 {
        shell = b"/bin/sh\x00" as *const u8 as *const libc::c_char
    }
    shellsize = strlen(shell).wrapping_add(1 as libc::c_int as libc::c_ulong);
    msg = xmalloc(cmdsize.wrapping_add(shellsize)) as *mut libc::c_char;
    memcpy(
        msg as *mut libc::c_void,
        cmd as *const libc::c_void,
        cmdsize,
    );
    memcpy(
        msg.offset(cmdsize as isize) as *mut libc::c_void,
        shell as *const libc::c_void,
        shellsize,
    );
    proc_send(
        (*c).peer,
        MSG_EXEC,
        -(1 as libc::c_int),
        msg as *const libc::c_void,
        cmdsize.wrapping_add(shellsize),
    );
    free(msg as *mut libc::c_void);
}
/* Check for mouse keys. */
unsafe extern "C" fn server_client_check_mouse(
    mut c: *mut client,
    mut event: *mut key_event,
) -> key_code {
    let mut current_block: u64;
    let mut m: *mut mouse_event = &mut (*event).m;
    let mut s: *mut session = (*c).session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut x: u_int = 0;
    let mut y: u_int = 0;
    let mut b: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    let mut px: u_int = 0;
    let mut py: u_int = 0;
    let mut ignore: libc::c_int = 0 as libc::c_int;
    let mut key: key_code = 0;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut sr: *mut style_range = 0 as *mut style_range;
    let mut type_0: C2RustUnnamed_35 = NOTYPE;
    let mut where_0: C2RustUnnamed_34 = NOWHERE;
    log_debug(
        b"%s mouse %02x at %u,%u (last %u,%u) (%d)\x00" as *const u8 as *const libc::c_char,
        (*c).name,
        (*m).b,
        (*m).x,
        (*m).y,
        (*m).lx,
        (*m).ly,
        (*c).tty.mouse_drag_flag,
    );
    /* What type of event is this? */
    if (*event).key == key_code_code::DOUBLECLICK as libc::c_ulong as libc::c_ulonglong {
        type_0 = DOUBLE;
        x = (*m).x;
        y = (*m).y;
        b = (*m).b;
        ignore = 1 as libc::c_int;
        log_debug(
            b"double-click at %u,%u\x00" as *const u8 as *const libc::c_char,
            x,
            y,
        );
    } else if (*m).sgr_type != ' ' as i32 as libc::c_uint
        && (*m).sgr_b & 32 as libc::c_int as libc::c_uint != 0
        && (*m).sgr_b & 3 as libc::c_int as libc::c_uint == 3 as libc::c_int as libc::c_uint
        || (*m).sgr_type == ' ' as i32 as libc::c_uint
            && (*m).b & 32 as libc::c_int as libc::c_uint != 0
            && (*m).b & 3 as libc::c_int as libc::c_uint == 3 as libc::c_int as libc::c_uint
            && (*m).lb & 3 as libc::c_int as libc::c_uint == 3 as libc::c_int as libc::c_uint
    {
        type_0 = MOVE;
        x = (*m).x;
        y = (*m).y;
        b = 0 as libc::c_int as u_int;
        log_debug(
            b"move at %u,%u\x00" as *const u8 as *const libc::c_char,
            x,
            y,
        );
    } else if (*m).b & 32 as libc::c_int as libc::c_uint != 0 {
        type_0 = DRAG;
        if (*c).tty.mouse_drag_flag != 0 {
            x = (*m).x;
            y = (*m).y;
            b = (*m).b;
            if x == (*m).lx && y == (*m).ly {
                return 0xfe000000000 as libc::c_ulonglong;
            }
            log_debug(
                b"drag update at %u,%u\x00" as *const u8 as *const libc::c_char,
                x,
                y,
            );
        } else {
            x = (*m).lx;
            y = (*m).ly;
            b = (*m).lb;
            log_debug(
                b"drag start at %u,%u\x00" as *const u8 as *const libc::c_char,
                x,
                y,
            );
        }
    } else if (*m).b & 64 as libc::c_int as libc::c_uint != 0 {
        type_0 = WHEEL;
        x = (*m).x;
        y = (*m).y;
        b = (*m).b;
        log_debug(
            b"wheel at %u,%u\x00" as *const u8 as *const libc::c_char,
            x,
            y,
        );
    } else if (*m).b & 3 as libc::c_int as libc::c_uint == 3 as libc::c_int as libc::c_uint {
        type_0 = UP;
        x = (*m).x;
        y = (*m).y;
        b = (*m).lb;
        log_debug(b"up at %u,%u\x00" as *const u8 as *const libc::c_char, x, y);
    } else {
        if (*c).flags & 0x100000 as libc::c_int as libc::c_ulong != 0 {
            event_del(&mut (*c).click_timer);
            (*c).flags &= !(0x100000 as libc::c_int) as libc::c_ulong;
            if (*m).b == (*c).click_button {
                type_0 = SECOND;
                x = (*m).x;
                y = (*m).y;
                b = (*m).b;
                log_debug(
                    b"second-click at %u,%u\x00" as *const u8 as *const libc::c_char,
                    x,
                    y,
                );
                (*c).flags |= 0x200000 as libc::c_int as libc::c_ulong
            }
            current_block = 1724319918354933278;
        } else if (*c).flags & 0x200000 as libc::c_int as libc::c_ulong != 0 {
            event_del(&mut (*c).click_timer);
            (*c).flags &= !(0x200000 as libc::c_int) as libc::c_ulong;
            if (*m).b == (*c).click_button {
                type_0 = TRIPLE;
                x = (*m).x;
                y = (*m).y;
                b = (*m).b;
                log_debug(
                    b"triple-click at %u,%u\x00" as *const u8 as *const libc::c_char,
                    x,
                    y,
                );
                current_block = 1862170126770142589;
            } else {
                current_block = 1724319918354933278;
            }
        } else {
            type_0 = DOWN;
            x = (*m).x;
            y = (*m).y;
            b = (*m).b;
            log_debug(
                b"down at %u,%u\x00" as *const u8 as *const libc::c_char,
                x,
                y,
            );
            (*c).flags |= 0x100000 as libc::c_int as libc::c_ulong;
            current_block = 1724319918354933278;
        }
        match current_block {
            1862170126770142589 => {}
            _ => {
                if 300 as libc::c_int != 0 as libc::c_int {
                    memcpy(
                        &mut (*c).click_event as *mut mouse_event as *mut libc::c_void,
                        m as *const libc::c_void,
                        ::std::mem::size_of::<mouse_event>() as libc::c_ulong,
                    );
                    (*c).click_button = (*m).b;
                    log_debug(b"click timer started\x00" as *const u8 as *const libc::c_char);
                    tv.tv_sec = (300 as libc::c_int / 1000 as libc::c_int) as __time_t;
                    tv.tv_usec = (300 as libc::c_int % 1000 as libc::c_int) as libc::c_long
                        * 1000 as libc::c_long;
                    event_del(&mut (*c).click_timer);
                    event_add(&mut (*c).click_timer, &mut tv);
                }
            }
        }
    }
    if type_0 as libc::c_uint == NOTYPE as libc::c_int as libc::c_uint {
        return 0xfe000000000 as libc::c_ulonglong;
    }
    /* Save the session. */
    (*m).s = (*s).id as libc::c_int;
    (*m).w = -(1 as libc::c_int);
    (*m).ignore = ignore;
    /* Is this on the status line? */
    (*m).statusat = status_at_line(c);
    (*m).statuslines = status_line_size(c);
    if (*m).statusat != -(1 as libc::c_int)
        && y >= (*m).statusat as u_int
        && y < ((*m).statusat as libc::c_uint).wrapping_add((*m).statuslines)
    {
        sr = status_get_range(c, x, y.wrapping_sub((*m).statusat as libc::c_uint));
        if sr.is_null() {
            where_0 = STATUS_DEFAULT
        } else {
            match (*sr).type_0 as libc::c_uint {
                0 => return 0xfe000000000 as libc::c_ulonglong,
                1 => where_0 = STATUS_LEFT,
                2 => where_0 = STATUS_RIGHT,
                3 => {
                    wl = winlink_find_by_index(&mut (*s).windows, (*sr).argument as libc::c_int);
                    if wl.is_null() {
                        return 0xfe000000000 as libc::c_ulonglong;
                    }
                    (*m).w = (*(*wl).window).id as libc::c_int;
                    where_0 = STATUS
                }
                _ => {}
            }
        }
    }
    /* Not on status line. Adjust position and check for border or pane. */
    if where_0 as libc::c_uint == NOWHERE as libc::c_int as libc::c_uint {
        px = x;
        if (*m).statusat == 0 as libc::c_int && y >= (*m).statuslines {
            py = y.wrapping_sub((*m).statuslines)
        } else if (*m).statusat > 0 as libc::c_int && y >= (*m).statusat as u_int {
            py = ((*m).statusat - 1 as libc::c_int) as u_int
        } else {
            py = y
        }
        tty_window_offset(&mut (*c).tty, &mut (*m).ox, &mut (*m).oy, &mut sx, &mut sy);
        log_debug(
            b"mouse window @%u at %u,%u (%ux%u)\x00" as *const u8 as *const libc::c_char,
            (*(*(*s).curw).window).id,
            (*m).ox,
            (*m).oy,
            sx,
            sy,
        );
        if px > sx || py > sy {
            return 0xfe000000000 as libc::c_ulonglong;
        }
        px = px.wrapping_add((*m).ox);
        py = py.wrapping_add((*m).oy);
        /* Try the pane borders if not zoomed. */
        if !(*(*(*s).curw).window).flags & 0x8 as libc::c_int != 0 {
            wp = (*(*(*s).curw).window).panes.tqh_first;
            while !wp.is_null() {
                if (*wp).xoff.wrapping_add((*wp).sx) == px
                    && (*wp).yoff <= (1 as libc::c_int as libc::c_uint).wrapping_add(py)
                    && (*wp).yoff.wrapping_add((*wp).sy) >= py
                    || (*wp).yoff.wrapping_add((*wp).sy) == py
                        && (*wp).xoff <= (1 as libc::c_int as libc::c_uint).wrapping_add(px)
                        && (*wp).xoff.wrapping_add((*wp).sx) >= px
                {
                    break;
                }
                wp = (*wp).entry.tqe_next
            }
            if !wp.is_null() {
                where_0 = BORDER
            }
        }
        /* Otherwise try inside the pane. */
        if where_0 as libc::c_uint == NOWHERE as libc::c_int as libc::c_uint {
            wp = window_get_active_at((*(*s).curw).window, px, py);
            if !wp.is_null() {
                where_0 = PANE
            } else {
                return 0xfe000000000 as libc::c_ulonglong;
            }
        }
        if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
            log_debug(
                b"mouse %u,%u on pane %%%u\x00" as *const u8 as *const libc::c_char,
                x,
                y,
                (*wp).id,
            );
        } else if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
            log_debug(
                b"mouse on pane %%%u border\x00" as *const u8 as *const libc::c_char,
                (*wp).id,
            );
        }
        (*m).wp = (*wp).id as libc::c_int;
        (*m).w = (*(*wp).window).id as libc::c_int
    } else {
        (*m).wp = -(1 as libc::c_int)
    }
    /* Stop dragging if needed. */
    if type_0 as libc::c_uint != DRAG as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint != WHEEL as libc::c_int as libc::c_uint
        && (*c).tty.mouse_drag_flag != 0
    {
        if (*c).tty.mouse_drag_release.is_some() {
            (*c).tty
                .mouse_drag_release
                .expect("non-null function pointer")(c, m);
        }
        (*c).tty.mouse_drag_update = None;
        (*c).tty.mouse_drag_release = None;
        /*
         * End a mouse drag by passing a MouseDragEnd key corresponding
         * to the button that started the drag.
         */
        match (*c).tty.mouse_drag_flag {
            1 => {
                if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND1_PANE as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND1_STATUS as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND1_STATUS_LEFT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND1_STATUS_RIGHT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND1_STATUS_DEFAULT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND1_BORDER as libc::c_ulong as key_code
                }
            }
            2 => {
                if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND2_PANE as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND2_STATUS as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND2_STATUS_LEFT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND2_STATUS_RIGHT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND2_STATUS_DEFAULT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND2_BORDER as libc::c_ulong as key_code
                }
            }
            3 => {
                if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND3_PANE as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND3_STATUS as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND3_STATUS_LEFT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND3_STATUS_RIGHT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND3_STATUS_DEFAULT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEDRAGEND3_BORDER as libc::c_ulong as key_code
                }
            }
            _ => key = key_code_code::MOUSE as libc::c_ulong as key_code,
        }
        (*c).tty.mouse_drag_flag = 0 as libc::c_int
    } else {
        /* Convert to a key binding. */
        key = 0xfe000000000 as libc::c_ulonglong;
        match type_0 as libc::c_uint {
            1 => {
                if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEMOVE_PANE as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEMOVE_STATUS as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEMOVE_STATUS_LEFT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEMOVE_STATUS_RIGHT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEMOVE_STATUS_DEFAULT as libc::c_ulong as key_code
                }
                if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                    key = key_code_code::MOUSEMOVE_BORDER as libc::c_ulong as key_code
                }
            }
            4 => {
                if (*c).tty.mouse_drag_update.is_some() {
                    key = key_code_code::DRAGGING as libc::c_ulong as key_code
                } else {
                    match b & 3 as libc::c_int as libc::c_uint {
                        0 => {
                            if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                                key = key_code_code::MOUSEDRAG1_PANE as libc::c_ulong as key_code
                            }
                            if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                                key = key_code_code::MOUSEDRAG1_STATUS as libc::c_ulong as key_code
                            }
                            if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint
                            {
                                key = key_code_code::MOUSEDRAG1_STATUS_LEFT as libc::c_ulong
                                    as key_code
                            }
                            if where_0 as libc::c_uint
                                == STATUS_RIGHT as libc::c_int as libc::c_uint
                            {
                                key = key_code_code::MOUSEDRAG1_STATUS_RIGHT as libc::c_ulong
                                    as key_code
                            }
                            if where_0 as libc::c_uint
                                == STATUS_DEFAULT as libc::c_int as libc::c_uint
                            {
                                key = key_code_code::MOUSEDRAG1_STATUS_DEFAULT as libc::c_ulong
                                    as key_code
                            }
                            if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                                key = key_code_code::MOUSEDRAG1_BORDER as libc::c_ulong as key_code
                            }
                        }
                        1 => {
                            if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                                key = key_code_code::MOUSEDRAG2_PANE as libc::c_ulong as key_code
                            }
                            if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                                key = key_code_code::MOUSEDRAG2_STATUS as libc::c_ulong as key_code
                            }
                            if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint
                            {
                                key = key_code_code::MOUSEDRAG2_STATUS_LEFT as libc::c_ulong
                                    as key_code
                            }
                            if where_0 as libc::c_uint
                                == STATUS_RIGHT as libc::c_int as libc::c_uint
                            {
                                key = key_code_code::MOUSEDRAG2_STATUS_RIGHT as libc::c_ulong
                                    as key_code
                            }
                            if where_0 as libc::c_uint
                                == STATUS_DEFAULT as libc::c_int as libc::c_uint
                            {
                                key = key_code_code::MOUSEDRAG2_STATUS_DEFAULT as libc::c_ulong
                                    as key_code
                            }
                            if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                                key = key_code_code::MOUSEDRAG2_BORDER as libc::c_ulong as key_code
                            }
                        }
                        2 => {
                            if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                                key = key_code_code::MOUSEDRAG3_PANE as libc::c_ulong as key_code
                            }
                            if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                                key = key_code_code::MOUSEDRAG3_STATUS as libc::c_ulong as key_code
                            }
                            if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint
                            {
                                key = key_code_code::MOUSEDRAG3_STATUS_LEFT as libc::c_ulong
                                    as key_code
                            }
                            if where_0 as libc::c_uint
                                == STATUS_RIGHT as libc::c_int as libc::c_uint
                            {
                                key = key_code_code::MOUSEDRAG3_STATUS_RIGHT as libc::c_ulong
                                    as key_code
                            }
                            if where_0 as libc::c_uint
                                == STATUS_DEFAULT as libc::c_int as libc::c_uint
                            {
                                key = key_code_code::MOUSEDRAG3_STATUS_DEFAULT as libc::c_ulong
                                    as key_code
                            }
                            if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                                key = key_code_code::MOUSEDRAG3_BORDER as libc::c_ulong as key_code
                            }
                        }
                        _ => {}
                    }
                }
                /*
                 * Begin a drag by setting the flag to a non-zero value that
                 * corresponds to the mouse button in use.
                 */
                (*c).tty.mouse_drag_flag = (b & 3 as libc::c_int as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    as libc::c_int
            }
            5 => {
                if b & 3 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELUP_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELUP_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELUP_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELUP_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELUP_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELUP_BORDER as libc::c_ulong as key_code
                    }
                } else {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELDOWN_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELDOWN_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELDOWN_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELDOWN_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELDOWN_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::WHEELDOWN_BORDER as libc::c_ulong as key_code
                    }
                }
            }
            3 => match b & 3 as libc::c_int as libc::c_uint {
                0 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP1_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP1_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP1_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP1_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP1_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP1_BORDER as libc::c_ulong as key_code
                    }
                }
                1 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP2_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP2_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP2_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP2_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP2_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP2_BORDER as libc::c_ulong as key_code
                    }
                }
                2 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP3_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP3_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP3_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP3_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP3_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEUP3_BORDER as libc::c_ulong as key_code
                    }
                }
                _ => {}
            },
            2 => match b & 3 as libc::c_int as libc::c_uint {
                0 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN1_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN1_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN1_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN1_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN1_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN1_BORDER as libc::c_ulong as key_code
                    }
                }
                1 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN2_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN2_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN2_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN2_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN2_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN2_BORDER as libc::c_ulong as key_code
                    }
                }
                2 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN3_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN3_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN3_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN3_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN3_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::MOUSEDOWN3_BORDER as libc::c_ulong as key_code
                    }
                }
                _ => {}
            },
            6 => match b & 3 as libc::c_int as libc::c_uint {
                0 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK1_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK1_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK1_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK1_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key =
                            key_code_code::SECONDCLICK1_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK1_BORDER as libc::c_ulong as key_code
                    }
                }
                1 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK2_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK2_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK2_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK2_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key =
                            key_code_code::SECONDCLICK2_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK2_BORDER as libc::c_ulong as key_code
                    }
                }
                2 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK3_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK3_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK3_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK3_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key =
                            key_code_code::SECONDCLICK3_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::SECONDCLICK3_BORDER as libc::c_ulong as key_code
                    }
                }
                _ => {}
            },
            7 => match b & 3 as libc::c_int as libc::c_uint {
                0 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK1_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK1_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK1_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK1_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key =
                            key_code_code::DOUBLECLICK1_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK1_BORDER as libc::c_ulong as key_code
                    }
                }
                1 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK2_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK2_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK2_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK2_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key =
                            key_code_code::DOUBLECLICK2_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK2_BORDER as libc::c_ulong as key_code
                    }
                }
                2 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK3_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK3_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK3_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK3_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key =
                            key_code_code::DOUBLECLICK3_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::DOUBLECLICK3_BORDER as libc::c_ulong as key_code
                    }
                }
                _ => {}
            },
            8 => match b & 3 as libc::c_int as libc::c_uint {
                0 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK1_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK1_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK1_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK1_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key =
                            key_code_code::TRIPLECLICK1_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK1_BORDER as libc::c_ulong as key_code
                    }
                }
                1 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK2_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK2_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK2_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK2_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key =
                            key_code_code::TRIPLECLICK2_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK2_BORDER as libc::c_ulong as key_code
                    }
                }
                2 => {
                    if where_0 as libc::c_uint == PANE as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK3_PANE as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK3_STATUS as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_LEFT as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK3_STATUS_LEFT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_RIGHT as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK3_STATUS_RIGHT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == STATUS_DEFAULT as libc::c_int as libc::c_uint {
                        key =
                            key_code_code::TRIPLECLICK3_STATUS_DEFAULT as libc::c_ulong as key_code
                    }
                    if where_0 as libc::c_uint == BORDER as libc::c_int as libc::c_uint {
                        key = key_code_code::TRIPLECLICK3_BORDER as libc::c_ulong as key_code
                    }
                }
                _ => {}
            },
            0 | _ => {}
        }
        if key == 0xfe000000000 as libc::c_ulonglong {
            return 0xfe000000000 as libc::c_ulonglong;
        }
    }
    /* Apply modifiers if any. */
    if b & 8 as libc::c_int as libc::c_uint != 0 {
        key |= 0x100000000000 as libc::c_ulonglong
    }
    if b & 16 as libc::c_int as libc::c_uint != 0 {
        key |= 0x200000000000 as libc::c_ulonglong
    }
    if b & 4 as libc::c_int as libc::c_uint != 0 {
        key |= 0x400000000000 as libc::c_ulonglong
    }
    if log_get_level() != 0 as libc::c_int {
        log_debug(
            b"mouse key is %s\x00" as *const u8 as *const libc::c_char,
            key_string_lookup_key(key, 1 as libc::c_int),
        );
    }
    return key;
}
/* Is this fast enough to probably be a paste? */
unsafe extern "C" fn server_client_assume_paste(mut s: *mut session) -> libc::c_int {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut t: libc::c_int = 0;
    t = options_get_number(
        (*s).options,
        b"assume-paste-time\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if t == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    tv.tv_sec = (*s).activity_time.tv_sec - (*s).last_activity_time.tv_sec;
    tv.tv_usec = (*s).activity_time.tv_usec - (*s).last_activity_time.tv_usec;
    if tv.tv_usec < 0 as libc::c_int as libc::c_long {
        tv.tv_sec -= 1;
        tv.tv_usec += 1000000 as libc::c_int as libc::c_long
    }
    if tv.tv_sec == 0 as libc::c_int as libc::c_long
        && tv.tv_usec < (t * 1000 as libc::c_int) as libc::c_long
    {
        log_debug(
            b"session %s pasting (flag %d)\x00" as *const u8 as *const libc::c_char,
            (*s).name,
            ((*s).flags & 0x1 as libc::c_int != 0) as libc::c_int,
        );
        if (*s).flags & 0x1 as libc::c_int != 0 {
            return 1 as libc::c_int;
        }
        (*s).flags |= 0x1 as libc::c_int;
        return 0 as libc::c_int;
    }
    log_debug(
        b"session %s not pasting\x00" as *const u8 as *const libc::c_char,
        (*s).name,
    );
    (*s).flags &= !(0x1 as libc::c_int);
    return 0 as libc::c_int;
}
/* Has the latest client changed? */
unsafe extern "C" fn server_client_update_latest(mut c: *mut client) {
    let mut w: *mut window = 0 as *mut window;
    if (*c).session.is_null() {
        return;
    }
    w = (*(*(*c).session).curw).window;
    if (*w).latest == c as *mut libc::c_void {
        return;
    }
    (*w).latest = c as *mut libc::c_void;
    if options_get_number(
        (*w).options,
        b"window-size\x00" as *const u8 as *const libc::c_char,
    ) == 3 as libc::c_int as libc::c_longlong
    {
        recalculate_size(w, 0 as libc::c_int);
    };
}
/*
 * Handle data key input from client. This owns and can modify the key event it
 * is given and is responsible for freeing it.
 */
unsafe extern "C" fn server_client_key_callback(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut data: *mut libc::c_void,
) -> cmd_retval {
    let mut current_block: u64;
    let mut c: *mut client = cmdq_get_client(item);
    let mut event: *mut key_event = data as *mut key_event;
    let mut key: key_code = (*event).key;
    let mut m: *mut mouse_event = &mut (*event).m;
    let mut s: *mut session = (*c).session;
    let mut wl: *mut winlink = 0 as *mut winlink;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut table: *mut key_table = 0 as *mut key_table;
    let mut first: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut xtimeout: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    let mut key0: key_code = 0;
    /* Check the client is good to accept input. */
    if !(s.is_null()
        || (*c).flags
            & (0x200 as libc::c_int | 0x40 as libc::c_int | 0x4 as libc::c_int) as libc::c_ulong
            != 0)
    {
        wl = (*s).curw;
        /* Update the activity timer. */
        if gettimeofday(&mut (*c).activity_time, 0 as *mut libc::c_void) != 0 as libc::c_int {
            fatal(b"gettimeofday failed\x00" as *const u8 as *const libc::c_char);
        }
        session_update_activity(s, &mut (*c).activity_time);
        /* Check for mouse keys. */
        (*m).valid = 0 as libc::c_int;
        if key == key_code_code::MOUSE as libc::c_ulong as libc::c_ulonglong
            || key == key_code_code::DOUBLECLICK as libc::c_ulong as libc::c_ulonglong
        {
            if (*c).flags & 0x800 as libc::c_int as libc::c_ulong != 0 {
                current_block = 4278348705573827347;
            } else {
                key = server_client_check_mouse(c, event);
                if key == 0xfe000000000 as libc::c_ulonglong {
                    current_block = 4278348705573827347;
                } else {
                    (*m).valid = 1 as libc::c_int;
                    (*m).key = key;
                    /*
                     * Mouse drag is in progress, so fire the callback (now that
                     * the mouse event is valid).
                     */
                    if key & 0xfffffffffff as libc::c_ulonglong
                        == key_code_code::DRAGGING as libc::c_ulong as libc::c_ulonglong
                    {
                        (*c).tty
                            .mouse_drag_update
                            .expect("non-null function pointer")(c, m);
                        current_block = 4278348705573827347;
                    } else {
                        (*event).key = key;
                        current_block = 5634871135123216486;
                    }
                }
            }
        } else {
            current_block = 5634871135123216486;
        }
        match current_block {
            4278348705573827347 => {}
            _ => {
                /* Find affected pane. */
                if !(key & 0xfffffffffff as libc::c_ulonglong
                    >= key_code_code::MOUSE as libc::c_ulong as libc::c_ulonglong
                    && (key & 0xfffffffffff as libc::c_ulonglong)
                        < key_code_code::BSPACE as libc::c_ulong as libc::c_ulonglong)
                    || cmd_find_from_mouse(&mut fs, m, 0 as libc::c_int) != 0 as libc::c_int
                {
                    cmd_find_from_client(&mut fs, c, 0 as libc::c_int);
                }
                wp = fs.wp;
                /* Forward mouse keys if disabled. */
                if key & 0xfffffffffff as libc::c_ulonglong
                    >= key_code_code::MOUSE as libc::c_ulong as libc::c_ulonglong
                    && (key & 0xfffffffffff as libc::c_ulonglong)
                        < key_code_code::BSPACE as libc::c_ulong as libc::c_ulonglong
                    && options_get_number(
                        (*s).options,
                        b"mouse\x00" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    current_block = 9475926807518885972;
                } else if !(key & 0xfffffffffff as libc::c_ulonglong
                    >= key_code_code::MOUSE as libc::c_ulong as libc::c_ulonglong
                    && (key & 0xfffffffffff as libc::c_ulonglong)
                        < key_code_code::BSPACE as libc::c_ulong as libc::c_ulonglong)
                    && server_client_assume_paste(s) != 0
                {
                    current_block = 9475926807518885972;
                } else {
                    /* Treat everything as a regular key when pasting is detected. */
                    /*
                     * Work out the current key table. If the pane is in a mode, use
                     * the mode table instead of the default key table.
                     */
                    if server_client_is_default_key_table(c, (*c).keytable) != 0
                        && !wp.is_null()
                        && {
                            wme = (*wp).modes.tqh_first;
                            !wme.is_null()
                        }
                        && (*(*wme).mode).key_table.is_some()
                    {
                        table = key_bindings_get_table(
                            (*(*wme).mode).key_table.expect("non-null function pointer")(wme),
                            1 as libc::c_int,
                        )
                    } else {
                        table = (*c).keytable
                    }
                    first = table;
                    loop {
                        /*
                         * The prefix always takes precedence and forces a switch to the prefix
                         * table, unless we are already there.
                         */
                        key0 = key
                            & (0xfffffffffff as libc::c_ulonglong
                                | 0xf00000000000 as libc::c_ulonglong);
                        if (key0
                            == options_get_number(
                                (*s).options,
                                b"prefix\x00" as *const u8 as *const libc::c_char,
                            ) as key_code
                            || key0
                                == options_get_number(
                                    (*s).options,
                                    b"prefix2\x00" as *const u8 as *const libc::c_char,
                                ) as key_code)
                            && strcmp(
                                (*table).name,
                                b"prefix\x00" as *const u8 as *const libc::c_char,
                            ) != 0 as libc::c_int
                        {
                            server_client_set_key_table(
                                c,
                                b"prefix\x00" as *const u8 as *const libc::c_char,
                            );
                            server_status_client(c);
                            current_block = 4278348705573827347;
                            break;
                        } else {
                            flags = (*c).flags as libc::c_int;
                            loop {
                                /* Log key table. */
                                if wp.is_null() {
                                    log_debug(
                                        b"key table %s (no pane)\x00" as *const u8
                                            as *const libc::c_char,
                                        (*table).name,
                                    );
                                } else {
                                    log_debug(
                                        b"key table %s (pane %%%u)\x00" as *const u8
                                            as *const libc::c_char,
                                        (*table).name,
                                        (*wp).id,
                                    );
                                }
                                if (*c).flags & 0x20 as libc::c_int as libc::c_ulong != 0 {
                                    log_debug(
                                        b"currently repeating\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                /* Try to see if there is a key binding in the current table. */
                                bd = key_bindings_get(table, key0);
                                if !bd.is_null() {
                                    /*
                                     * Key was matched in this table. If currently repeating but a
                                     * non-repeating binding was found, stop repeating and try
                                     * again in the root table.
                                     */
                                    if (*c).flags & 0x20 as libc::c_int as libc::c_ulong != 0
                                        && !(*bd).flags & 0x1 as libc::c_int != 0
                                    {
                                        current_block = 3938820862080741272;
                                        break;
                                    } else {
                                        current_block = 13460095289871124136;
                                        break;
                                    }
                                } else if key0
                                    != key_code_code::ANY as libc::c_ulong as libc::c_ulonglong
                                {
                                    key0 = key_code_code::ANY as libc::c_ulong as key_code
                                } else {
                                    /*
                                     * No match, try the ANY key.
                                     */
                                    /*
                                     * No match in this table. If not in the root table or if repeating,
                                     * switch the client back to the root table and try again.
                                     */
                                    log_debug(
                                        b"not found in key table %s\x00" as *const u8
                                            as *const libc::c_char,
                                        (*table).name,
                                    );
                                    if server_client_is_default_key_table(c, table) == 0
                                        || (*c).flags & 0x20 as libc::c_int as libc::c_ulong != 0
                                    {
                                        current_block = 5028470053297453708;
                                        break;
                                    } else {
                                        current_block = 5873035170358615968;
                                        break;
                                    }
                                }
                            }
                            match current_block {
                                5873035170358615968 =>
                                /*
                                 * No match in the root table either. If this wasn't the first table
                                 * tried, don't pass the key to the pane.
                                 */
                                {
                                    if first != table && !flags & 0x20 as libc::c_int != 0 {
                                        current_block = 3736434875406665187;
                                        break;
                                    } else {
                                        current_block = 9475926807518885972;
                                        break;
                                    }
                                }
                                3938820862080741272 => {
                                    log_debug(
                                        b"found in key table %s (not repeating)\x00" as *const u8
                                            as *const libc::c_char,
                                        (*table).name,
                                    );
                                    server_client_set_key_table(c, 0 as *const libc::c_char);
                                    table = (*c).keytable;
                                    first = table;
                                    (*c).flags &= !(0x20 as libc::c_int) as libc::c_ulong;
                                    server_status_client(c);
                                }
                                5028470053297453708 => {
                                    log_debug(
                                        b"trying in root table\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                    server_client_set_key_table(c, 0 as *const libc::c_char);
                                    table = (*c).keytable;
                                    if (*c).flags & 0x20 as libc::c_int as libc::c_ulong != 0 {
                                        first = table
                                    }
                                    (*c).flags &= !(0x20 as libc::c_int) as libc::c_ulong;
                                    server_status_client(c);
                                }
                                _ => {
                                    log_debug(
                                        b"found in key table %s\x00" as *const u8
                                            as *const libc::c_char,
                                        (*table).name,
                                    );
                                    /*
                                     * Take a reference to this table to make sure the key binding
                                     * doesn't disappear.
                                     */
                                    (*table).references = (*table).references.wrapping_add(1);
                                    /*
                                     * If this is a repeating key, start the timer. Otherwise reset
                                     * the client back to the root table.
                                     */
                                    xtimeout = options_get_number(
                                        (*s).options,
                                        b"repeat-time\x00" as *const u8 as *const libc::c_char,
                                    ) as libc::c_int;
                                    if xtimeout != 0 as libc::c_int
                                        && (*bd).flags & 0x1 as libc::c_int != 0
                                    {
                                        (*c).flags |= 0x20 as libc::c_int as libc::c_ulong;
                                        tv.tv_sec = (xtimeout / 1000 as libc::c_int) as __time_t;
                                        tv.tv_usec = (xtimeout % 1000 as libc::c_int)
                                            as libc::c_long
                                            * 1000 as libc::c_long;
                                        event_del(&mut (*c).repeat_timer);
                                        event_add(&mut (*c).repeat_timer, &mut tv);
                                    } else {
                                        (*c).flags &= !(0x20 as libc::c_int) as libc::c_ulong;
                                        server_client_set_key_table(c, 0 as *const libc::c_char);
                                    }
                                    server_status_client(c);
                                    /* Execute the key binding. */
                                    key_bindings_dispatch(bd, item, c, event, &mut fs);
                                    key_bindings_unref_table(table);
                                    current_block = 4278348705573827347;
                                    break;
                                }
                            }
                        }
                    }
                    match current_block {
                        4278348705573827347 => {}
                        9475926807518885972 => {}
                        _ => {
                            server_client_set_key_table(c, 0 as *const libc::c_char);
                            server_status_client(c);
                            current_block = 4278348705573827347;
                        }
                    }
                }
                match current_block {
                    4278348705573827347 => {}
                    _ => {
                        if !((*c).flags & 0x800 as libc::c_int as libc::c_ulong != 0) {
                            if !wp.is_null() {
                                window_pane_key(wp, c, s, wl, key, m);
                            }
                        }
                    }
                }
            }
        }
    }
    if !s.is_null() && key != key_code_code::FOCUS_OUT as libc::c_ulong as libc::c_ulonglong {
        server_client_update_latest(c);
    }
    free(event as *mut libc::c_void);
    return CMD_RETURN_NORMAL;
}
/* Handle a key event. */
#[no_mangle]
pub unsafe extern "C" fn server_client_handle_key(
    mut c: *mut client,
    mut event: *mut key_event,
) -> libc::c_int {
    let mut s: *mut session = (*c).session;
    let mut item: *mut crate::cmd_queue::cmdq_item = 0 as *mut crate::cmd_queue::cmdq_item;
    /* Check the client is good to accept input. */
    if s.is_null()
        || (*c).flags
            & (0x200 as libc::c_int | 0x40 as libc::c_int | 0x4 as libc::c_int) as libc::c_ulong
            != 0
    {
        return 0 as libc::c_int;
    }
    /*
     * Key presses in overlay mode and the command prompt are a special
     * case. The queue might be blocked so they need to be processed
     * immediately rather than queued.
     */
    if !(*c).flags & 0x800 as libc::c_int as libc::c_ulong != 0 {
        status_message_clear(c);
        if (*c).overlay_key.is_some() {
            match (*c).overlay_key.expect("non-null function pointer")(c, event) {
                0 => return 0 as libc::c_int,
                1 => {
                    server_client_clear_overlay(c);
                    return 0 as libc::c_int;
                }
                _ => {}
            }
        }
        server_client_clear_overlay(c);
        if !(*c).prompt_string.is_null() {
            if status_prompt_key(c, (*event).key) == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
        }
    }
    /*
     * Add the key to the queue so it happens after any commands queued by
     * previous keys.
     */
    item = cmdq_get_callback1(
        b"server_client_key_callback\x00" as *const u8 as *const libc::c_char,
        Some(
            server_client_key_callback
                as unsafe extern "C" fn(
                    _: *mut crate::cmd_queue::cmdq_item,
                    _: *mut libc::c_void,
                ) -> cmd_retval,
        ),
        event as *mut libc::c_void,
    );
    cmdq_append(c, item);
    return 1 as libc::c_int;
}
/* Client functions that need to happen every loop. */
#[no_mangle]
pub unsafe extern "C" fn server_client_loop() {
    let mut c: *mut client = 0 as *mut client;
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut focus: libc::c_int = 0;
    /* Check for window resize. This is done before redrawing. */
    w = windows_RB_MINMAX(&mut windows, -(1 as libc::c_int));
    while !w.is_null() {
        server_client_check_window_resize(w);
        w = windows_RB_NEXT(w)
    }
    /* Check clients. */
    c = clients.tqh_first;
    while !c.is_null() {
        server_client_check_exit(c);
        if !(*c).session.is_null() {
            server_client_check_redraw(c);
            server_client_reset_state(c);
        }
        c = (*c).entry.tqe_next
    }
    /*
     * Any windows will have been redrawn as part of clients, so clear
     * their flags now. Also check pane focus and resize.
     */
    focus = options_get_number(
        global_options,
        b"focus-events\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    w = windows_RB_MINMAX(&mut windows, -(1 as libc::c_int));
    while !w.is_null() {
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if (*wp).fd != -(1 as libc::c_int) {
                if focus != 0 {
                    server_client_check_pane_focus(wp);
                }
                server_client_check_pane_resize(wp);
                server_client_check_pane_buffer(wp);
            }
            (*wp).flags &= !(0x1 as libc::c_int);
            wp = (*wp).entry.tqe_next
        }
        check_window_name(w);
        w = windows_RB_NEXT(w)
    }
}
/* Check if window needs to be resized. */
unsafe extern "C" fn server_client_check_window_resize(mut w: *mut window) {
    let mut wl: *mut winlink = 0 as *mut winlink;
    if !(*w).flags & 0x20 as libc::c_int != 0 {
        return;
    }
    wl = (*w).winlinks.tqh_first;
    while !wl.is_null() {
        if (*(*wl).session).attached != 0 as libc::c_int as libc::c_uint
            && (*(*wl).session).curw == wl
        {
            break;
        }
        wl = (*wl).wentry.tqe_next
    }
    if wl.is_null() {
        return;
    }
    log_debug(
        b"%s: resizing window @%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
            b"server_client_check_window_resize\x00",
        ))
        .as_ptr(),
        (*w).id,
    );
    resize_window(
        w,
        (*w).new_sx,
        (*w).new_sy,
        (*w).new_xpixel as libc::c_int,
        (*w).new_ypixel as libc::c_int,
    );
}
/* Resize timer event. */
unsafe extern "C" fn server_client_resize_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut wp: *mut window_pane = data as *mut window_pane;
    log_debug(
        b"%s: %%%u resize timer expired\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
            b"server_client_resize_timer\x00",
        ))
        .as_ptr(),
        (*wp).id,
    );
    event_del(&mut (*wp).resize_timer);
}
/* Start the resize timer. */
unsafe extern "C" fn server_client_start_resize_timer(mut wp: *mut window_pane) {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 250000 as libc::c_int as __suseconds_t,
        };
        init
    };
    log_debug(
        b"%s: %%%u resize timer started\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
            b"server_client_start_resize_timer\x00",
        ))
        .as_ptr(),
        (*wp).id,
    );
    event_add(&mut (*wp).resize_timer, &mut tv);
}
/* Force timer event. */
unsafe extern "C" fn server_client_force_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut wp: *mut window_pane = data as *mut window_pane;
    log_debug(
        b"%s: %%%u force timer expired\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
            b"server_client_force_timer\x00",
        ))
        .as_ptr(),
        (*wp).id,
    );
    event_del(&mut (*wp).force_timer);
    (*wp).flags |= 0x2000 as libc::c_int;
}
/* Start the force timer. */
unsafe extern "C" fn server_client_start_force_timer(mut wp: *mut window_pane) {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 10000 as libc::c_int as __suseconds_t,
        };
        init
    };
    log_debug(
        b"%s: %%%u force timer started\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
            b"server_client_start_force_timer\x00",
        ))
        .as_ptr(),
        (*wp).id,
    );
    event_add(&mut (*wp).force_timer, &mut tv);
}
/* Check if pane should be resized. */
unsafe extern "C" fn server_client_check_pane_resize(mut wp: *mut window_pane) {
    if event_initialized(&mut (*wp).resize_timer) == 0 {
        event_set(
            &mut (*wp).resize_timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                server_client_resize_timer
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            wp as *mut libc::c_void,
        );
    }
    if event_initialized(&mut (*wp).force_timer) == 0 {
        event_set(
            &mut (*wp).force_timer,
            -(1 as libc::c_int),
            0 as libc::c_int as libc::c_short,
            Some(
                server_client_force_timer
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            wp as *mut libc::c_void,
        );
    }
    if !(*wp).flags & 0x8 as libc::c_int != 0 {
        return;
    }
    log_debug(
        b"%s: %%%u needs to be resized\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
            b"server_client_check_pane_resize\x00",
        ))
        .as_ptr(),
        (*wp).id,
    );
    if event_pending(
        &mut (*wp).resize_timer,
        0x1 as libc::c_int as libc::c_short,
        0 as *mut timeval,
    ) != 0
    {
        log_debug(
            b"%s: %%%u resize timer is running\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"server_client_check_pane_resize\x00",
            ))
            .as_ptr(),
            (*wp).id,
        );
        return;
    }
    server_client_start_resize_timer(wp);
    if !(*wp).flags & 0x10 as libc::c_int != 0 {
        /*
         * The timer is not running and we don't need to force a
         * resize, so just resize immediately.
         */
        log_debug(
            b"%s: resizing %%%u now\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"server_client_check_pane_resize\x00",
            ))
            .as_ptr(),
            (*wp).id,
        );
        window_pane_send_resize(wp, 0 as libc::c_int);
        (*wp).flags &= !(0x8 as libc::c_int)
    } else if (*wp).flags & 0x2000 as libc::c_int != 0 {
        log_debug(
            b"%s: resizing %%%u after forced resize\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"server_client_check_pane_resize\x00",
            ))
            .as_ptr(),
            (*wp).id,
        );
        window_pane_send_resize(wp, 0 as libc::c_int);
        (*wp).flags &= !(0x8 as libc::c_int | 0x10 as libc::c_int | 0x2000 as libc::c_int)
    } else if event_pending(
        &mut (*wp).force_timer,
        0x1 as libc::c_int as libc::c_short,
        0 as *mut timeval,
    ) == 0
    {
        log_debug(
            b"%s: forcing resize of %%%u\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"server_client_check_pane_resize\x00",
            ))
            .as_ptr(),
            (*wp).id,
        );
        window_pane_send_resize(wp, 1 as libc::c_int);
        server_client_start_force_timer(wp);
    };
}
/*
 * The timer is not running, but we need to force a resize. If
 * the force timer has expired, resize to the real size now.
 * Otherwise resize to the force size and start the timer.
 */
/* Check pane buffer size. */
unsafe extern "C" fn server_client_check_pane_buffer(mut wp: *mut window_pane) {
    let mut evb: *mut evbuffer = (*(*wp).event).input;
    let mut minimum: size_t = 0;
    let mut c: *mut client = 0 as *mut client;
    let mut wpo: *mut window_pane_offset = 0 as *mut window_pane_offset;
    let mut off: libc::c_int = 1 as libc::c_int;
    let mut flag: libc::c_int = 0;
    let mut attached_clients: u_int = 0 as libc::c_int as u_int;
    let mut new_size: size_t = 0;
    /*
     * Work out the minimum used size. This is the most that can be removed
     * from the buffer.
     */
    minimum = (*wp).offset.used;
    if (*wp).pipe_fd != -(1 as libc::c_int) && (*wp).pipe_offset.used < minimum {
        minimum = (*wp).pipe_offset.used
    }
    c = clients.tqh_first;
    while !c.is_null() {
        if !(*c).session.is_null() {
            attached_clients = attached_clients.wrapping_add(1);
            if !(*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
                off = 0 as libc::c_int
            } else {
                wpo = control_pane_offset(c, wp, &mut flag);
                if wpo.is_null() {
                    off = 0 as libc::c_int
                } else {
                    if flag == 0 {
                        off = 0 as libc::c_int
                    }
                    window_pane_get_new_data(wp, wpo, &mut new_size);
                    log_debug(
                        b"%s: %s has %zu bytes used and %zu left for %%%u\x00" as *const u8
                            as *const libc::c_char,
                        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"server_client_check_pane_buffer\x00",
                        ))
                        .as_ptr(),
                        (*c).name,
                        (*wpo).used.wrapping_sub((*wp).base_offset),
                        new_size,
                        (*wp).id,
                    );
                    if (*wpo).used < minimum {
                        minimum = (*wpo).used
                    }
                }
            }
        }
        c = (*c).entry.tqe_next
    }
    if attached_clients == 0 as libc::c_int as libc::c_uint {
        off = 0 as libc::c_int
    }
    minimum = (minimum as libc::c_ulong).wrapping_sub((*wp).base_offset) as size_t as size_t;
    if !(minimum == 0 as libc::c_int as libc::c_ulong) {
        /* Drain the buffer. */
        log_debug(
            b"%s: %%%u has %zu minimum (of %zu) bytes used\x00" as *const u8 as *const libc::c_char,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"server_client_check_pane_buffer\x00",
            ))
            .as_ptr(),
            (*wp).id,
            minimum,
            evbuffer_get_length(evb),
        );
        evbuffer_drain(evb, minimum);
        /*
         * Adjust the base offset. If it would roll over, all the offsets into
         * the buffer need to be adjusted.
         */
        if (*wp).base_offset > (18446744073709551615 as libc::c_ulong).wrapping_sub(minimum) {
            log_debug(
                b"%s: %%%u base offset has wrapped\x00" as *const u8 as *const libc::c_char,
                (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"server_client_check_pane_buffer\x00",
                ))
                .as_ptr(),
                (*wp).id,
            );
            (*wp).offset.used = ((*wp).offset.used as libc::c_ulong).wrapping_sub((*wp).base_offset)
                as size_t as size_t;
            if (*wp).pipe_fd != -(1 as libc::c_int) {
                (*wp).pipe_offset.used = ((*wp).pipe_offset.used as libc::c_ulong)
                    .wrapping_sub((*wp).base_offset)
                    as size_t as size_t
            }
            c = clients.tqh_first;
            while !c.is_null() {
                if !((*c).session.is_null()
                    || !(*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0)
                {
                    wpo = control_pane_offset(c, wp, &mut flag);
                    if !wpo.is_null() && flag == 0 {
                        (*wpo).used = ((*wpo).used as libc::c_ulong).wrapping_sub((*wp).base_offset)
                            as size_t as size_t
                    }
                }
                c = (*c).entry.tqe_next
            }
            (*wp).base_offset = minimum
        } else {
            (*wp).base_offset =
                ((*wp).base_offset as libc::c_ulong).wrapping_add(minimum) as size_t as size_t
        }
    }
    /*
     * If there is data remaining, and there are no clients able to consume
     * it, do not read any more. This is true when there are attached
     * clients, all of which are control clients which are not able to
     * accept any more data.
     */
    log_debug(
        b"%s: pane %%%u is %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
            b"server_client_check_pane_buffer\x00",
        ))
        .as_ptr(),
        (*wp).id,
        if off != 0 {
            b"off\x00" as *const u8 as *const libc::c_char
        } else {
            b"on\x00" as *const u8 as *const libc::c_char
        },
    );
    if off != 0 {
        bufferevent_disable((*wp).event, 0x2 as libc::c_int as libc::c_short);
    } else {
        bufferevent_enable((*wp).event, 0x2 as libc::c_int as libc::c_short);
    };
}
/* Check whether pane should be focused. */
unsafe extern "C" fn server_client_check_pane_focus(mut wp: *mut window_pane) {
    let mut current_block: u64;
    let mut c: *mut client = 0 as *mut client;
    let mut push: libc::c_int = 0;
    /* Do we need to push the focus state? */
    push = (*wp).flags & 0x20 as libc::c_int;
    (*wp).flags &= !(0x20 as libc::c_int);
    /* If we're not the active pane in our window, we're not focused. */
    if !((*(*wp).window).active != wp) {
        /*
         * If our window is the current window in any focused clients with an
         * attached session, we're focused.
         */
        c = clients.tqh_first;
        loop {
            if c.is_null() {
                current_block = 12717618590137386175;
                break;
            }
            if !((*c).session.is_null() || (*c).flags & 0x8000 as libc::c_int as libc::c_ulong == 0)
            {
                if !((*(*c).session).attached == 0 as libc::c_int as libc::c_uint) {
                    if (*(*(*c).session).curw).window == (*wp).window {
                        current_block = 16836582972682985622;
                        break;
                    }
                }
            }
            c = (*c).entry.tqe_next
        }
        match current_block {
            12717618590137386175 => {}
            _ => {
                if push != 0 || (*wp).flags & 0x4 as libc::c_int == 0 {
                    if (*wp).base.mode & 0x800 as libc::c_int != 0 {
                        bufferevent_write(
                            (*wp).event,
                            b"\x1b[I\x00" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            3 as libc::c_int as size_t,
                        );
                    }
                    notify_pane(b"pane-focus-in\x00" as *const u8 as *const libc::c_char, wp);
                    session_update_activity((*c).session, 0 as *mut timeval);
                }
                (*wp).flags |= 0x4 as libc::c_int;
                return;
            }
        }
    }
    if push != 0 || (*wp).flags & 0x4 as libc::c_int != 0 {
        if (*wp).base.mode & 0x800 as libc::c_int != 0 {
            bufferevent_write(
                (*wp).event,
                b"\x1b[O\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as size_t,
            );
        }
        notify_pane(
            b"pane-focus-out\x00" as *const u8 as *const libc::c_char,
            wp,
        );
    }
    (*wp).flags &= !(0x4 as libc::c_int);
}
/*
 * Update cursor position and mode settings. The scroll region and attributes
 * are cleared when idle (waiting for an event) as this is the most likely time
 * a user may interrupt tmux, for example with ~^Z in ssh(1). This is a
 * compromise between excessive resets and likelihood of an interrupt.
 *
 * tty_region/tty_reset/tty_update_mode already take care of not resetting
 * things that are already in their default state.
 */
unsafe extern "C" fn server_client_reset_state(mut c: *mut client) {
    let mut tty: *mut tty = &mut (*c).tty;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = server_client_get_pane(c);
    let mut loop_0: *mut window_pane = 0 as *mut window_pane;
    let mut s: *mut screen = 0 as *mut screen;
    let mut oo: *mut crate::options::options = (*(*c).session).options;
    let mut mode: libc::c_int = 0 as libc::c_int;
    let mut cursor: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut cx: u_int = 0 as libc::c_int as u_int;
    let mut cy: u_int = 0 as libc::c_int as u_int;
    let mut ox: u_int = 0;
    let mut oy: u_int = 0;
    let mut sx: u_int = 0;
    let mut sy: u_int = 0;
    if (*c).flags & (0x2000 as libc::c_int | 0x40 as libc::c_int) as libc::c_ulong != 0 {
        return;
    }
    /* Disable the block flag. */
    flags = (*tty).flags & 0x80 as libc::c_int;
    (*tty).flags &= !(0x80 as libc::c_int);
    /* Get mode from overlay if any, else from screen. */
    if (*c).overlay_draw.is_some() {
        if (*c).overlay_mode.is_some() {
            s = (*c).overlay_mode.expect("non-null function pointer")(c, &mut cx, &mut cy)
        }
    } else {
        s = (*wp).screen
    }
    if !s.is_null() {
        mode = (*s).mode
    }
    log_debug(
        b"%s: client %s mode %x\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
            b"server_client_reset_state\x00",
        ))
        .as_ptr(),
        (*c).name,
        mode,
    );
    /* Reset region and margin. */
    tty_region_off(tty);
    tty_margin_off(tty);
    /* Move cursor to pane cursor and offset. */
    if (*c).overlay_draw.is_none() {
        cursor = 0 as libc::c_int;
        tty_window_offset(tty, &mut ox, &mut oy, &mut sx, &mut sy);
        if (*wp).xoff.wrapping_add((*s).cx) >= ox
            && (*wp).xoff.wrapping_add((*s).cx) <= ox.wrapping_add(sx)
            && (*wp).yoff.wrapping_add((*s).cy) >= oy
            && (*wp).yoff.wrapping_add((*s).cy) <= oy.wrapping_add(sy)
        {
            cursor = 1 as libc::c_int;
            cx = (*wp).xoff.wrapping_add((*s).cx).wrapping_sub(ox);
            cy = (*wp).yoff.wrapping_add((*s).cy).wrapping_sub(oy);
            if status_at_line(c) == 0 as libc::c_int {
                cy = (cy as libc::c_uint).wrapping_add(status_line_size(c)) as u_int as u_int
            }
        }
        if cursor == 0 {
            mode &= !(0x1 as libc::c_int)
        }
    }
    log_debug(
        b"%s: cursor to %u,%u\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
            b"server_client_reset_state\x00",
        ))
        .as_ptr(),
        cx,
        cy,
    );
    tty_cursor(tty, cx, cy);
    /*
     * Set mouse mode if requested. To support dragging, always use button
     * mode.
     */
    if options_get_number(oo, b"mouse\x00" as *const u8 as *const libc::c_char) != 0 {
        if (*c).overlay_draw.is_none() {
            mode &= !(0x20 as libc::c_int | 0x40 as libc::c_int | 0x1000 as libc::c_int);
            loop_0 = (*w).panes.tqh_first;
            while !loop_0.is_null() {
                if (*(*loop_0).screen).mode & 0x1000 as libc::c_int != 0 {
                    mode |= 0x1000 as libc::c_int
                }
                loop_0 = (*loop_0).entry.tqe_next
            }
        }
        if !mode & 0x1000 as libc::c_int != 0 {
            mode |= 0x40 as libc::c_int
        }
    }
    /* Clear bracketed paste mode if at the prompt. */
    if (*c).overlay_draw.is_none() && !(*c).prompt_string.is_null() {
        mode &= !(0x400 as libc::c_int)
    }
    /* Set the terminal mode and reset attributes. */
    tty_update_mode(tty, mode, s);
    tty_reset(tty);
    /* All writing must be done, send a sync end (if it was started). */
    tty_sync_end(tty);
    (*tty).flags |= flags;
}
/* Repeat time callback. */
unsafe extern "C" fn server_client_repeat_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut c: *mut client = data as *mut client;
    if (*c).flags & 0x20 as libc::c_int as libc::c_ulong != 0 {
        server_client_set_key_table(c, 0 as *const libc::c_char);
        (*c).flags &= !(0x20 as libc::c_int) as libc::c_ulong;
        server_status_client(c);
    };
}
/* Double-click callback. */
unsafe extern "C" fn server_client_click_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut c: *mut client = data as *mut client;
    let mut event: *mut key_event = 0 as *mut key_event;
    log_debug(b"click timer expired\x00" as *const u8 as *const libc::c_char);
    if (*c).flags & 0x200000 as libc::c_int as libc::c_ulong != 0 {
        /*
         * Waiting for a third click that hasn't happened, so this must
         * have been a double click.
         */
        event = xmalloc(::std::mem::size_of::<key_event>() as libc::c_ulong) as *mut key_event;
        (*event).key = key_code_code::DOUBLECLICK as libc::c_ulong as key_code;
        memcpy(
            &mut (*event).m as *mut mouse_event as *mut libc::c_void,
            &mut (*c).click_event as *mut mouse_event as *const libc::c_void,
            ::std::mem::size_of::<mouse_event>() as libc::c_ulong,
        );
        if server_client_handle_key(c, event) == 0 {
            free(event as *mut libc::c_void);
        }
    }
    (*c).flags &= !(0x100000 as libc::c_int | 0x200000 as libc::c_int) as libc::c_ulong;
}
/* Check if client should be exited. */
unsafe extern "C" fn server_client_check_exit(mut c: *mut client) {
    let mut cf: *mut client_file = 0 as *mut client_file;
    let mut name: *const libc::c_char = (*c).exit_session;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    let mut msize: size_t = 0;
    if (*c).flags & (0x200 as libc::c_int | 0x100 as libc::c_int) as libc::c_ulong != 0 {
        return;
    }
    if !(*c).flags & 0x4 as libc::c_int as libc::c_ulong != 0 {
        return;
    }
    if (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
        control_discard(c);
        if control_all_done(c) == 0 {
            return;
        }
    }
    cf = client_files_RB_MINMAX(&mut (*c).files, -(1 as libc::c_int));
    while !cf.is_null() {
        if evbuffer_get_length((*cf).buffer) != 0 as libc::c_int as libc::c_ulong {
            return;
        }
        cf = client_files_RB_NEXT(cf)
    }
    if (*c).flags & 0x80 as libc::c_int as libc::c_ulong != 0 {
        notify_client(
            b"client-detached\x00" as *const u8 as *const libc::c_char,
            c,
        );
    }
    (*c).flags |= 0x100 as libc::c_int as libc::c_ulong;
    match (*c).exit_type as libc::c_uint {
        0 => {
            if !(*c).exit_message.is_null() {
                msize = strlen((*c).exit_message).wrapping_add(1 as libc::c_int as libc::c_ulong);
                size = (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_add(msize)
            } else {
                size = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            }
            data = xmalloc(size) as *mut libc::c_char;
            memcpy(
                data as *mut libc::c_void,
                &mut (*c).retval as *mut libc::c_int as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            if !(*c).exit_message.is_null() {
                memcpy(
                    data.offset(::std::mem::size_of::<libc::c_int>() as libc::c_ulong as isize)
                        as *mut libc::c_void,
                    (*c).exit_message as *const libc::c_void,
                    msize,
                );
            }
            proc_send(
                (*c).peer,
                MSG_EXIT,
                -(1 as libc::c_int),
                data as *const libc::c_void,
                size,
            );
            free(data as *mut libc::c_void);
        }
        1 => {
            proc_send(
                (*c).peer,
                MSG_SHUTDOWN,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        2 => {
            proc_send(
                (*c).peer,
                (*c).exit_msgtype,
                -(1 as libc::c_int),
                name as *const libc::c_void,
                strlen(name).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        _ => {}
    }
    free((*c).exit_session as *mut libc::c_void);
    free((*c).exit_message as *mut libc::c_void);
}
/* Redraw timer callback. */
unsafe extern "C" fn server_client_redraw_timer(
    mut _fd: libc::c_int,
    mut _events: libc::c_short,
    mut _data: *mut libc::c_void,
) {
    log_debug(b"redraw timer fired\x00" as *const u8 as *const libc::c_char);
}
/* Check for client redraws. */
unsafe extern "C" fn server_client_check_redraw(mut c: *mut client) {
    let mut s: *mut session = (*c).session;
    let mut tty: *mut tty = &mut (*c).tty;
    let mut w: *mut window = (*(*(*c).session).curw).window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    let mut needed: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut mode: libc::c_int = (*tty).mode;
    let mut new_flags: libc::c_int = 0 as libc::c_int;
    let mut redraw: libc::c_int = 0;
    let mut bit: u_int = 0 as libc::c_int as u_int;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0,
            tv_usec: 1000 as libc::c_int as __suseconds_t,
        };
        init
    };
    static mut ev: event = event {
        ev_evcallback: event_callback {
            evcb_active_next: C2RustUnnamed_7 {
                tqe_next: 0 as *const event_callback as *mut event_callback,
                tqe_prev: 0 as *const *mut event_callback as *mut *mut event_callback,
            },
            evcb_flags: 0,
            evcb_pri: 0,
            evcb_closure: 0,
            evcb_cb_union: C2RustUnnamed_6 {
                evcb_callback: None,
            },
            evcb_arg: 0 as *const libc::c_void as *mut libc::c_void,
        },
        ev_timeout_pos: C2RustUnnamed_4 {
            ev_next_with_common_timeout: C2RustUnnamed_5 {
                tqe_next: 0 as *const event as *mut event,
                tqe_prev: 0 as *const *mut event as *mut *mut event,
            },
        },
        ev_fd: 0,
        ev_base: 0 as *const event_base as *mut event_base,
        ev_: C2RustUnnamed {
            ev_io: C2RustUnnamed_2 {
                ev_io_next: C2RustUnnamed_3 {
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
    let mut left: size_t = 0;
    if (*c).flags & (0x2000 as libc::c_int | 0x40 as libc::c_int) as libc::c_ulong != 0 {
        return;
    }
    if (*c).flags
        & (0x8 as libc::c_int
            | 0x10 as libc::c_int
            | 0x1000000 as libc::c_int
            | 0x400 as libc::c_int
            | 0x2000000 as libc::c_int
            | 0x20000000 as libc::c_int) as libc::c_ulong
        != 0
    {
        log_debug(
            b"%s: redraw%s%s%s%s%s\x00" as *const u8 as *const libc::c_char,
            (*c).name,
            if (*c).flags & 0x8 as libc::c_int as libc::c_ulong != 0 {
                b" window\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if (*c).flags & 0x10 as libc::c_int as libc::c_ulong != 0 {
                b" status\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if (*c).flags & 0x400 as libc::c_int as libc::c_ulong != 0 {
                b" borders\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if (*c).flags & 0x2000000 as libc::c_int as libc::c_ulong != 0 {
                b" overlay\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if (*c).flags & 0x20000000 as libc::c_int as libc::c_ulong != 0 {
                b" panes\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        );
    }
    /*
     * If there is outstanding data, defer the redraw until it has been
     * consumed. We can just add a timer to get out of the event loop and
     * end up back here.
     */
    needed = 0 as libc::c_int;
    if (*c).flags
        & (0x8 as libc::c_int
            | 0x10 as libc::c_int
            | 0x1000000 as libc::c_int
            | 0x400 as libc::c_int
            | 0x2000000 as libc::c_int
            | 0x20000000 as libc::c_int) as libc::c_ulong
        != 0
    {
        needed = 1 as libc::c_int
    } else {
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if (*wp).flags & 0x1 as libc::c_int != 0 {
                needed = 1 as libc::c_int;
                break;
            } else {
                wp = (*wp).entry.tqe_next
            }
        }
        if needed != 0 {
            new_flags |= 0x20000000 as libc::c_int
        }
    }
    if needed != 0 && {
        left = evbuffer_get_length((*tty).out);
        (left) != 0 as libc::c_int as libc::c_ulong
    } {
        log_debug(
            b"%s: redraw deferred (%zu left)\x00" as *const u8 as *const libc::c_char,
            (*c).name,
            left,
        );
        if event_initialized(&mut ev) == 0 {
            event_set(
                &mut ev,
                -(1 as libc::c_int),
                0 as libc::c_int as libc::c_short,
                Some(
                    server_client_redraw_timer
                        as unsafe extern "C" fn(
                            _: libc::c_int,
                            _: libc::c_short,
                            _: *mut libc::c_void,
                        ) -> (),
                ),
                0 as *mut libc::c_void,
            );
        }
        if event_pending(
            &mut ev,
            0x1 as libc::c_int as libc::c_short,
            0 as *mut timeval,
        ) == 0
        {
            log_debug(b"redraw timer started\x00" as *const u8 as *const libc::c_char);
            event_add(&mut ev, &mut tv);
        }
        if !(*c).flags & 0x8 as libc::c_int as libc::c_ulong != 0 {
            wp = (*w).panes.tqh_first;
            while !wp.is_null() {
                if (*wp).flags & 0x1 as libc::c_int != 0 {
                    log_debug(
                        b"%s: pane %%%u needs redraw\x00" as *const u8 as *const libc::c_char,
                        (*c).name,
                        (*wp).id,
                    );
                    (*c).redraw_panes |= ((1 as libc::c_int) << bit) as libc::c_ulong
                }
                bit = bit.wrapping_add(1);
                if bit == 64 as libc::c_int as libc::c_uint {
                    /*
                     * If more that 64 panes, give up and
                     * just redraw the window.
                     */
                    new_flags &= 0x20000000 as libc::c_int;
                    new_flags |= 0x8 as libc::c_int;
                    break;
                } else {
                    wp = (*wp).entry.tqe_next
                }
            }
            if (*c).redraw_panes != 0 as libc::c_int as libc::c_ulong {
                (*c).flags |= 0x20000000 as libc::c_int as libc::c_ulong
            }
        }
        (*c).flags |= new_flags as libc::c_ulong;
        return;
    } else {
        if needed != 0 {
            log_debug(
                b"%s: redraw needed\x00" as *const u8 as *const libc::c_char,
                (*c).name,
            );
        }
    }
    flags = (*tty).flags & (0x80 as libc::c_int | 0x2 as libc::c_int | 0x1 as libc::c_int);
    (*tty).flags = (*tty).flags & !(0x80 as libc::c_int | 0x2 as libc::c_int) | 0x1 as libc::c_int;
    if !(*c).flags & 0x8 as libc::c_int as libc::c_ulong != 0 {
        /*
         * If not redrawing the entire window, check whether each pane
         * needs to be redrawn.
         */
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            redraw = 0 as libc::c_int;
            if (*wp).flags & 0x1 as libc::c_int != 0 {
                redraw = 1 as libc::c_int
            } else if (*c).flags & 0x20000000 as libc::c_int as libc::c_ulong != 0 {
                redraw = ((*c).redraw_panes & ((1 as libc::c_int) << bit) as libc::c_ulong != 0)
                    as libc::c_int
            }
            bit = bit.wrapping_add(1);
            if !(redraw == 0) {
                log_debug(
                    b"%s: redrawing pane %%%u\x00" as *const u8 as *const libc::c_char,
                    (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                        b"server_client_check_redraw\x00",
                    ))
                    .as_ptr(),
                    (*wp).id,
                );
                screen_redraw_pane(c, wp);
            }
            wp = (*wp).entry.tqe_next
        }
        (*c).redraw_panes = 0 as libc::c_int as uint64_t;
        (*c).flags &= !(0x20000000 as libc::c_int) as libc::c_ulong
    }
    if (*c).flags
        & (0x8 as libc::c_int
            | 0x10 as libc::c_int
            | 0x1000000 as libc::c_int
            | 0x400 as libc::c_int
            | 0x2000000 as libc::c_int
            | 0x20000000 as libc::c_int) as libc::c_ulong
        != 0
    {
        if options_get_number(
            (*s).options,
            b"set-titles\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
            server_client_set_title(c);
        }
        screen_redraw_screen(c);
    }
    (*tty).flags = (*tty).flags & !(0x1 as libc::c_int) | flags & 0x1 as libc::c_int;
    tty_update_mode(tty, mode, 0 as *mut screen);
    (*tty).flags =
        (*tty).flags & !(0x80 as libc::c_int | 0x2 as libc::c_int | 0x1 as libc::c_int) | flags;
    (*c).flags &= !(0x8 as libc::c_int
        | 0x10 as libc::c_int
        | 0x1000000 as libc::c_int
        | 0x400 as libc::c_int
        | 0x2000000 as libc::c_int
        | 0x20000000 as libc::c_int
        | 0x80000 as libc::c_int) as libc::c_ulong;
    if needed != 0 {
        /*
         * We would have deferred the redraw unless the output buffer
         * was empty, so we can record how many bytes the redraw
         * generated.
         */
        (*c).redraw = evbuffer_get_length((*tty).out);
        log_debug(
            b"%s: redraw added %zu bytes\x00" as *const u8 as *const libc::c_char,
            (*c).name,
            (*c).redraw,
        );
    };
}
/* Set client title. */
unsafe extern "C" fn server_client_set_title(mut c: *mut client) {
    let mut s: *mut session = (*c).session;
    let mut template: *const libc::c_char = 0 as *const libc::c_char;
    let mut title: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ft: *mut crate::format::format_tree = 0 as *mut crate::format::format_tree;
    template = options_get_string(
        (*s).options,
        b"set-titles-string\x00" as *const u8 as *const libc::c_char,
    );
    ft = format_create(
        c,
        0 as *mut crate::cmd_queue::cmdq_item,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    format_defaults(
        ft,
        c,
        0 as *mut session,
        0 as *mut winlink,
        0 as *mut window_pane,
    );
    title = format_expand_time(ft, template);
    if (*c).title.is_null() || strcmp(title, (*c).title) != 0 as libc::c_int {
        free((*c).title as *mut libc::c_void);
        (*c).title = xstrdup(title);
        tty_set_title(&mut (*c).tty, (*c).title);
    }
    free(title as *mut libc::c_void);
    format_free(ft);
}
/* Dispatch message from client. */
unsafe extern "C" fn server_client_dispatch(mut imsg: *mut imsg, mut arg: *mut libc::c_void) {
    let mut c: *mut client = arg as *mut client;
    let mut datalen: ssize_t = 0;
    let mut s: *mut session = 0 as *mut session;
    if (*c).flags & 0x200 as libc::c_int as libc::c_ulong != 0 {
        return;
    }
    if imsg.is_null() {
        server_client_lost(c);
        return;
    }
    datalen = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong) as ssize_t;
    match (*imsg).hdr.type_0 {
        109 | 100 | 111 | 101 | 102 | 108 | 104 | 110 | 105 | 107 | 106 => {
            server_client_dispatch_identify(c, imsg);
        }
        200 => {
            server_client_dispatch_command(c, imsg);
        }
        208 => {
            if datalen != 0 as libc::c_int as libc::c_long {
                fatalx(b"bad MSG_RESIZE size\x00" as *const u8 as *const libc::c_char);
            }
            if !((*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0) {
                server_client_update_latest(c);
                server_client_clear_overlay(c);
                tty_resize(&mut (*c).tty);
                recalculate_sizes();
                server_redraw_client(c);
                if !(*c).session.is_null() {
                    notify_client(b"client-resized\x00" as *const u8 as *const libc::c_char, c);
                }
            }
        }
        205 => {
            if datalen != 0 as libc::c_int as libc::c_long {
                fatalx(b"bad MSG_EXITING size\x00" as *const u8 as *const libc::c_char);
            }
            (*c).session = 0 as *mut session;
            tty_close(&mut (*c).tty);
            proc_send(
                (*c).peer,
                MSG_EXITED,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        216 | 215 => {
            if datalen != 0 as libc::c_int as libc::c_long {
                fatalx(b"bad MSG_WAKEUP size\x00" as *const u8 as *const libc::c_char);
            }
            if !((*c).flags & 0x40 as libc::c_int as libc::c_ulong == 0) {
                (*c).flags &= !(0x40 as libc::c_int) as libc::c_ulong;
                if !((*c).fd == -(1 as libc::c_int) || (*c).session.is_null()) {
                    s = (*c).session;
                    if gettimeofday(&mut (*c).activity_time, 0 as *mut libc::c_void)
                        != 0 as libc::c_int
                    {
                        fatal(b"gettimeofday failed\x00" as *const u8 as *const libc::c_char);
                    }
                    tty_start_tty(&mut (*c).tty);
                    server_redraw_client(c);
                    recalculate_sizes();
                    if !s.is_null() {
                        session_update_activity(s, &mut (*c).activity_time);
                    }
                }
            }
        }
        209 => {
            if datalen != 0 as libc::c_int as libc::c_long {
                fatalx(b"bad MSG_SHELL size\x00" as *const u8 as *const libc::c_char);
            }
            server_client_dispatch_shell(c);
        }
        305 => {
            server_client_dispatch_write_ready(c, imsg);
        }
        301 => {
            server_client_dispatch_read_data(c, imsg);
        }
        302 => {
            server_client_dispatch_read_done(c, imsg);
        }
        _ => {}
    };
}
/* Callback when command is done. */
unsafe extern "C" fn server_client_command_done(
    mut item: *mut crate::cmd_queue::cmdq_item,
    mut _data: *mut libc::c_void,
) -> cmd_retval {
    let mut c: *mut client = cmdq_get_client(item);
    if !(*c).flags & 0x80 as libc::c_int as libc::c_ulong != 0 {
        (*c).flags |= 0x4 as libc::c_int as libc::c_ulong
    } else if !(*c).flags & 0x4 as libc::c_int as libc::c_ulong != 0 {
        tty_send_requests(&mut (*c).tty);
    }
    return CMD_RETURN_NORMAL;
}
/* Handle command message. */
unsafe extern "C" fn server_client_dispatch_command(mut c: *mut client, mut imsg: *mut imsg) {
    let mut data: msg_command = msg_command { argc: 0 };
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut argc: libc::c_int = 0;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pr: *mut cmd_parse_result = 0 as *mut cmd_parse_result;
    if (*c).flags & 0x4 as libc::c_int as libc::c_ulong != 0 {
        return;
    }
    if ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong)
        < ::std::mem::size_of::<msg_command>() as libc::c_ulong
    {
        fatalx(b"bad MSG_COMMAND size\x00" as *const u8 as *const libc::c_char);
    }
    memcpy(
        &mut data as *mut msg_command as *mut libc::c_void,
        (*imsg).data,
        ::std::mem::size_of::<msg_command>() as libc::c_ulong,
    );
    buf = ((*imsg).data as *mut libc::c_char)
        .offset(::std::mem::size_of::<msg_command>() as libc::c_ulong as isize);
    len = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<msg_command>() as libc::c_ulong);
    if len > 0 as libc::c_int as libc::c_ulong
        && *buf.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
            != '\u{0}' as i32
    {
        fatalx(b"bad MSG_COMMAND string\x00" as *const u8 as *const libc::c_char);
    }
    argc = data.argc;
    if cmd_unpack_argv(buf, len, argc, &mut argv) != 0 as libc::c_int {
        cause = xstrdup(b"command too long\x00" as *const u8 as *const libc::c_char)
    } else {
        if argc == 0 as libc::c_int {
            argc = 1 as libc::c_int;
            argv = xcalloc(
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ) as *mut *mut libc::c_char;
            *argv = xstrdup(b"new-session\x00" as *const u8 as *const libc::c_char)
        }
        pr = cmd_parse_from_arguments(argc, argv, 0 as *mut cmd_parse_input);
        match (*pr).status as libc::c_uint {
            0 => cause = xstrdup(b"empty command\x00" as *const u8 as *const libc::c_char),
            1 => cause = (*pr).error,
            2 | _ => {
                cmd_free_argv(argc, argv);
                cmdq_append(
                    c,
                    cmdq_get_command((*pr).cmdlist, 0 as *mut crate::cmd_queue::cmdq_state),
                );
                cmdq_append(
                    c,
                    cmdq_get_callback1(
                        b"server_client_command_done\x00" as *const u8 as *const libc::c_char,
                        Some(
                            server_client_command_done
                                as unsafe extern "C" fn(
                                    _: *mut crate::cmd_queue::cmdq_item,
                                    _: *mut libc::c_void,
                                )
                                    -> cmd_retval,
                        ),
                        0 as *mut libc::c_void,
                    ),
                );
                cmd_list_free((*pr).cmdlist);
                return;
            }
        }
    }
    cmd_free_argv(argc, argv);
    cmdq_append(c, cmdq_get_error(cause));
    free(cause as *mut libc::c_void);
    (*c).flags |= 0x4 as libc::c_int as libc::c_ulong;
}
/* Handle identify message. */
unsafe extern "C" fn server_client_dispatch_identify(mut c: *mut client, mut imsg: *mut imsg) {
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    let mut datalen: size_t = 0;
    let mut flags: libc::c_int = 0;
    let mut feat: libc::c_int = 0;
    let mut longflags: uint64_t = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*c).flags & 0x40000 as libc::c_int as libc::c_ulong != 0 {
        fatalx(b"out-of-order identify message\x00" as *const u8 as *const libc::c_char);
    }
    data = (*imsg).data as *const libc::c_char;
    datalen = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong);
    match (*imsg).hdr.type_0 {
        109 => {
            if datalen != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
                fatalx(b"bad MSG_IDENTIFY_FEATURES size\x00" as *const u8 as *const libc::c_char);
            }
            memcpy(
                &mut feat as *mut libc::c_int as *mut libc::c_void,
                data as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            (*c).term_features |= feat;
            log_debug(
                b"client %p IDENTIFY_FEATURES %s\x00" as *const u8 as *const libc::c_char,
                c,
                tty_get_features(feat),
            );
        }
        100 => {
            if datalen != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
                fatalx(b"bad MSG_IDENTIFY_FLAGS size\x00" as *const u8 as *const libc::c_char);
            }
            memcpy(
                &mut flags as *mut libc::c_int as *mut libc::c_void,
                data as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            (*c).flags |= flags as libc::c_ulong;
            log_debug(
                b"client %p IDENTIFY_FLAGS %#x\x00" as *const u8 as *const libc::c_char,
                c,
                flags,
            );
        }
        111 => {
            if datalen != ::std::mem::size_of::<uint64_t>() as libc::c_ulong {
                fatalx(b"bad MSG_IDENTIFY_LONGFLAGS size\x00" as *const u8 as *const libc::c_char);
            }
            memcpy(
                &mut longflags as *mut uint64_t as *mut libc::c_void,
                data as *const libc::c_void,
                ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
            );
            (*c).flags |= longflags;
            log_debug(
                b"client %p IDENTIFY_LONGFLAGS %#llx\x00" as *const u8 as *const libc::c_char,
                c,
                longflags as libc::c_ulonglong,
            );
        }
        101 => {
            if datalen == 0 as libc::c_int as libc::c_ulong
                || *data.offset(datalen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    != '\u{0}' as i32
            {
                fatalx(b"bad MSG_IDENTIFY_TERM string\x00" as *const u8 as *const libc::c_char);
            }
            if *data as libc::c_int == '\u{0}' as i32 {
                (*c).term_name = xstrdup(b"unknown\x00" as *const u8 as *const libc::c_char)
            } else {
                (*c).term_name = xstrdup(data)
            }
            log_debug(
                b"client %p IDENTIFY_TERM %s\x00" as *const u8 as *const libc::c_char,
                c,
                data,
            );
        }
        102 => {
            if datalen == 0 as libc::c_int as libc::c_ulong
                || *data.offset(datalen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    != '\u{0}' as i32
            {
                fatalx(b"bad MSG_IDENTIFY_TTYNAME string\x00" as *const u8 as *const libc::c_char);
            }
            (*c).ttyname = xstrdup(data);
            log_debug(
                b"client %p IDENTIFY_TTYNAME %s\x00" as *const u8 as *const libc::c_char,
                c,
                data,
            );
        }
        108 => {
            if datalen == 0 as libc::c_int as libc::c_ulong
                || *data.offset(datalen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    != '\u{0}' as i32
            {
                fatalx(b"bad MSG_IDENTIFY_CWD string\x00" as *const u8 as *const libc::c_char);
            }
            if access(data, 1 as libc::c_int) == 0 as libc::c_int {
                (*c).cwd = xstrdup(data)
            } else {
                home = find_home();
                if !home.is_null() {
                    (*c).cwd = xstrdup(home)
                } else {
                    (*c).cwd = xstrdup(b"/\x00" as *const u8 as *const libc::c_char)
                }
            }
            log_debug(
                b"client %p IDENTIFY_CWD %s\x00" as *const u8 as *const libc::c_char,
                c,
                data,
            );
        }
        104 => {
            if datalen != 0 as libc::c_int as libc::c_ulong {
                fatalx(b"bad MSG_IDENTIFY_STDIN size\x00" as *const u8 as *const libc::c_char);
            }
            (*c).fd = (*imsg).fd;
            log_debug(
                b"client %p IDENTIFY_STDIN %d\x00" as *const u8 as *const libc::c_char,
                c,
                (*imsg).fd,
            );
        }
        110 => {
            if datalen != 0 as libc::c_int as libc::c_ulong {
                fatalx(b"bad MSG_IDENTIFY_STDOUT size\x00" as *const u8 as *const libc::c_char);
            }
            (*c).out_fd = (*imsg).fd;
            log_debug(
                b"client %p IDENTIFY_STDOUT %d\x00" as *const u8 as *const libc::c_char,
                c,
                (*imsg).fd,
            );
        }
        105 => {
            if datalen == 0 as libc::c_int as libc::c_ulong
                || *data.offset(datalen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    != '\u{0}' as i32
            {
                fatalx(b"bad MSG_IDENTIFY_ENVIRON string\x00" as *const u8 as *const libc::c_char);
            }
            if !strchr(data, '=' as i32).is_null() {
                environ_put((*c).environ, data, 0 as libc::c_int);
            }
            log_debug(
                b"client %p IDENTIFY_ENVIRON %s\x00" as *const u8 as *const libc::c_char,
                c,
                data,
            );
        }
        107 => {
            if datalen != ::std::mem::size_of::<pid_t>() as libc::c_ulong {
                fatalx(b"bad MSG_IDENTIFY_CLIENTPID size\x00" as *const u8 as *const libc::c_char);
            }
            memcpy(
                &mut (*c).pid as *mut pid_t as *mut libc::c_void,
                data as *const libc::c_void,
                ::std::mem::size_of::<pid_t>() as libc::c_ulong,
            );
            log_debug(
                b"client %p IDENTIFY_CLIENTPID %ld\x00" as *const u8 as *const libc::c_char,
                c,
                (*c).pid as libc::c_long,
            );
        }
        _ => {}
    }
    if (*imsg).hdr.type_0 != MSG_IDENTIFY_DONE as libc::c_int as libc::c_uint {
        return;
    }
    (*c).flags |= 0x40000 as libc::c_int as libc::c_ulong;
    if *(*c).ttyname as libc::c_int != '\u{0}' as i32 {
        name = xstrdup((*c).ttyname)
    } else {
        xasprintf(
            &mut name as *mut *mut libc::c_char,
            b"client-%ld\x00" as *const u8 as *const libc::c_char,
            (*c).pid as libc::c_long,
        );
    }
    (*c).name = name;
    log_debug(
        b"client %p name is %s\x00" as *const u8 as *const libc::c_char,
        c,
        (*c).name,
    );
    if (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
        control_start(c);
    } else if (*c).fd != -(1 as libc::c_int) {
        if tty_init(&mut (*c).tty, c) != 0 as libc::c_int {
            close((*c).fd);
            (*c).fd = -(1 as libc::c_int)
        } else {
            tty_resize(&mut (*c).tty);
            (*c).flags |= 0x1 as libc::c_int as libc::c_ulong
        }
        close((*c).out_fd);
        (*c).out_fd = -(1 as libc::c_int)
    }
    /*
     * If this is the first client, load configuration files. Any later
     * clients are allowed to continue with their command even if the
     * config has not been loaded - they might have been run from inside it
     */
    if !(*c).flags & 0x4 as libc::c_int as libc::c_ulong != 0
        && cfg_finished == 0
        && c == clients.tqh_first
    {
        start_cfg();
    };
}
/* Handle shell message. */
unsafe extern "C" fn server_client_dispatch_shell(mut c: *mut client) {
    let mut shell: *const libc::c_char = 0 as *const libc::c_char;
    shell = options_get_string(
        global_s_options,
        b"default-shell\x00" as *const u8 as *const libc::c_char,
    );
    if checkshell(shell) == 0 {
        shell = b"/bin/sh\x00" as *const u8 as *const libc::c_char
    }
    proc_send(
        (*c).peer,
        MSG_SHELL,
        -(1 as libc::c_int),
        shell as *const libc::c_void,
        strlen(shell).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    proc_kill_peer((*c).peer);
}
/* Handle write ready message. */
unsafe extern "C" fn server_client_dispatch_write_ready(mut c: *mut client, mut imsg: *mut imsg) {
    let mut msg: *mut msg_write_ready = (*imsg).data as *mut msg_write_ready;
    let mut msglen: size_t = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong);
    let mut find: client_file = client_file {
        c: 0 as *mut client,
        references: 0,
        stream: 0,
        path: 0 as *mut libc::c_char,
        buffer: 0 as *mut evbuffer,
        event: 0 as *mut bufferevent,
        fd: 0,
        error: 0,
        closed: 0,
        cb: None,
        data: 0 as *mut libc::c_void,
        entry: C2RustUnnamed_9 {
            rbe_left: 0 as *mut client_file,
            rbe_right: 0 as *mut client_file,
            rbe_parent: 0 as *mut client_file,
            rbe_color: 0,
        },
    };
    let mut cf: *mut client_file = 0 as *mut client_file;
    if msglen != ::std::mem::size_of::<msg_write_ready>() as libc::c_ulong {
        fatalx(b"bad MSG_WRITE_READY size\x00" as *const u8 as *const libc::c_char);
    }
    find.stream = (*msg).stream;
    cf = client_files_RB_FIND(&mut (*c).files, &mut find);
    if cf.is_null() {
        return;
    }
    if (*msg).error != 0 as libc::c_int {
        (*cf).error = (*msg).error;
        file_fire_done(cf);
    } else {
        file_push(cf);
    };
}
/* Handle read data message. */
unsafe extern "C" fn server_client_dispatch_read_data(mut c: *mut client, mut imsg: *mut imsg) {
    let mut msg: *mut msg_read_data = (*imsg).data as *mut msg_read_data;
    let mut msglen: size_t = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong);
    let mut find: client_file = client_file {
        c: 0 as *mut client,
        references: 0,
        stream: 0,
        path: 0 as *mut libc::c_char,
        buffer: 0 as *mut evbuffer,
        event: 0 as *mut bufferevent,
        fd: 0,
        error: 0,
        closed: 0,
        cb: None,
        data: 0 as *mut libc::c_void,
        entry: C2RustUnnamed_9 {
            rbe_left: 0 as *mut client_file,
            rbe_right: 0 as *mut client_file,
            rbe_parent: 0 as *mut client_file,
            rbe_color: 0,
        },
    };
    let mut cf: *mut client_file = 0 as *mut client_file;
    let mut bdata: *mut libc::c_void = msg.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    let mut bsize: size_t =
        msglen.wrapping_sub(::std::mem::size_of::<msg_read_data>() as libc::c_ulong);
    if msglen < ::std::mem::size_of::<msg_read_data>() as libc::c_ulong {
        fatalx(b"bad MSG_READ_DATA size\x00" as *const u8 as *const libc::c_char);
    }
    find.stream = (*msg).stream;
    cf = client_files_RB_FIND(&mut (*c).files, &mut find);
    if cf.is_null() {
        return;
    }
    log_debug(
        b"%s: file %d read %zu bytes\x00" as *const u8 as *const libc::c_char,
        (*c).name,
        (*cf).stream,
        bsize,
    );
    if (*cf).error == 0 as libc::c_int {
        if evbuffer_add((*cf).buffer, bdata, bsize) != 0 as libc::c_int {
            (*cf).error = 12 as libc::c_int;
            file_fire_done(cf);
        } else {
            file_fire_read(cf);
        }
    };
}
/* Handle read done message. */
unsafe extern "C" fn server_client_dispatch_read_done(mut c: *mut client, mut imsg: *mut imsg) {
    let mut msg: *mut msg_read_done = (*imsg).data as *mut msg_read_done;
    let mut msglen: size_t = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong);
    let mut find: client_file = client_file {
        c: 0 as *mut client,
        references: 0,
        stream: 0,
        path: 0 as *mut libc::c_char,
        buffer: 0 as *mut evbuffer,
        event: 0 as *mut bufferevent,
        fd: 0,
        error: 0,
        closed: 0,
        cb: None,
        data: 0 as *mut libc::c_void,
        entry: C2RustUnnamed_9 {
            rbe_left: 0 as *mut client_file,
            rbe_right: 0 as *mut client_file,
            rbe_parent: 0 as *mut client_file,
            rbe_color: 0,
        },
    };
    let mut cf: *mut client_file = 0 as *mut client_file;
    if msglen != ::std::mem::size_of::<msg_read_done>() as libc::c_ulong {
        fatalx(b"bad MSG_READ_DONE size\x00" as *const u8 as *const libc::c_char);
    }
    find.stream = (*msg).stream;
    cf = client_files_RB_FIND(&mut (*c).files, &mut find);
    if cf.is_null() {
        return;
    }
    log_debug(
        b"%s: file %d read done\x00" as *const u8 as *const libc::c_char,
        (*c).name,
        (*cf).stream,
    );
    (*cf).error = (*msg).error;
    file_fire_done(cf);
}
/* Get client working directory. */
#[no_mangle]
pub unsafe extern "C" fn server_client_get_cwd(
    mut c: *mut client,
    mut s: *mut session,
) -> *const libc::c_char {
    let mut home: *const libc::c_char = 0 as *const libc::c_char;
    if cfg_finished == 0 && !cfg_client.is_null() {
        return (*cfg_client).cwd;
    }
    if !c.is_null() && (*c).session.is_null() && !(*c).cwd.is_null() {
        return (*c).cwd;
    }
    if !s.is_null() && !(*s).cwd.is_null() {
        return (*s).cwd;
    }
    if !c.is_null()
        && {
            s = (*c).session;
            !s.is_null()
        }
        && !(*s).cwd.is_null()
    {
        return (*s).cwd;
    }
    home = find_home();
    if !home.is_null() {
        return home;
    }
    return b"/\x00" as *const u8 as *const libc::c_char;
}
/* Get control client flags. */
unsafe extern "C" fn server_client_control_flags(
    mut c: *mut client,
    mut next: *const libc::c_char,
) -> uint64_t {
    if strcmp(next, b"pause-after\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*c).pause_age = 0 as libc::c_int as u_int;
        return 0x100000000 as libc::c_ulonglong as uint64_t;
    }
    if sscanf(
        next,
        b"pause-after=%u\x00" as *const u8 as *const libc::c_char,
        &mut (*c).pause_age as *mut u_int,
    ) == 1 as libc::c_int
    {
        (*c).pause_age = ((*c).pause_age as libc::c_uint)
            .wrapping_mul(1000 as libc::c_int as libc::c_uint) as u_int
            as u_int;
        return 0x100000000 as libc::c_ulonglong as uint64_t;
    }
    if strcmp(next, b"no-output\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return 0x4000000 as libc::c_int as uint64_t;
    }
    if strcmp(next, b"wait-exit\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return 0x200000000 as libc::c_ulonglong as uint64_t;
    }
    return 0 as libc::c_int as uint64_t;
}
/* Set client flags. */
#[no_mangle]
pub unsafe extern "C" fn server_client_set_flags(
    mut c: *mut client,
    mut flags: *const libc::c_char,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flag: uint64_t = 0;
    let mut not: libc::c_int = 0;
    copy = xstrdup(flags);
    s = copy;
    loop {
        next = strsep(&mut s, b",\x00" as *const u8 as *const libc::c_char);
        if next.is_null() {
            break;
        }
        not = (*next as libc::c_int == '!' as i32) as libc::c_int;
        if not != 0 {
            next = next.offset(1)
        }
        if (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
            flag = server_client_control_flags(c, next)
        } else {
            flag = 0 as libc::c_int as uint64_t
        }
        if strcmp(next, b"read-only\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            flag = 0x800 as libc::c_int as uint64_t
        } else if strcmp(next, b"ignore-size\x00" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            flag = 0x20000 as libc::c_int as uint64_t
        } else if strcmp(next, b"active-pane\x00" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            flag = 0x80000000 as libc::c_ulonglong as uint64_t
        }
        if flag == 0 as libc::c_int as libc::c_ulong {
            continue;
        }
        log_debug(
            b"client %s set flag %s\x00" as *const u8 as *const libc::c_char,
            (*c).name,
            next,
        );
        if not != 0 {
            (*c).flags &= !flag
        } else {
            (*c).flags |= flag
        }
        if flag == 0x4000000 as libc::c_int as libc::c_ulong {
            control_reset_offsets(c);
        }
    }
    free(copy as *mut libc::c_void);
    proc_send(
        (*c).peer,
        MSG_FLAGS,
        -(1 as libc::c_int),
        &mut (*c).flags as *mut uint64_t as *const libc::c_void,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    );
}
/* Get client flags. This is only flags useful to show to users. */
#[no_mangle]
pub unsafe extern "C" fn server_client_get_flags(mut c: *mut client) -> *const libc::c_char {
    static mut s: [libc::c_char; 256] = [0; 256];
    let mut tmp: [libc::c_char; 32] = [0; 32];
    *s.as_mut_ptr() = '\u{0}' as i32 as libc::c_char;
    if (*c).flags & 0x80 as libc::c_int as libc::c_ulong != 0 {
        strlcat(
            s.as_mut_ptr(),
            b"attached,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if (*c).flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
        strlcat(
            s.as_mut_ptr(),
            b"control-mode,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if (*c).flags & 0x20000 as libc::c_int as libc::c_ulong != 0 {
        strlcat(
            s.as_mut_ptr(),
            b"ignore-size,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if (*c).flags & 0x4000000 as libc::c_int as libc::c_ulong != 0 {
        strlcat(
            s.as_mut_ptr(),
            b"no-output,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if (*c).flags as libc::c_ulonglong & 0x200000000 as libc::c_ulonglong != 0 {
        strlcat(
            s.as_mut_ptr(),
            b"wait-exit,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if (*c).flags as libc::c_ulonglong & 0x100000000 as libc::c_ulonglong != 0 {
        xsnprintf(
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"pause-after=%u,\x00" as *const u8 as *const libc::c_char,
            (*c).pause_age
                .wrapping_div(1000 as libc::c_int as libc::c_uint),
        );
        strlcat(
            s.as_mut_ptr(),
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if (*c).flags & 0x800 as libc::c_int as libc::c_ulong != 0 {
        strlcat(
            s.as_mut_ptr(),
            b"read-only,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if (*c).flags as libc::c_ulonglong & 0x80000000 as libc::c_ulonglong != 0 {
        strlcat(
            s.as_mut_ptr(),
            b"active-pane,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if (*c).flags & 0x40 as libc::c_int as libc::c_ulong != 0 {
        strlcat(
            s.as_mut_ptr(),
            b"suspended,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if (*c).flags & 0x10000 as libc::c_int as libc::c_ulong != 0 {
        strlcat(
            s.as_mut_ptr(),
            b"UTF-8,\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
    }
    if *s.as_mut_ptr() as libc::c_int != '\u{0}' as i32 {
        s[strlen(s.as_mut_ptr()).wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] =
            '\u{0}' as i32 as libc::c_char
    }
    return s.as_mut_ptr();
}
/* Get client window. */
unsafe extern "C" fn server_client_get_client_window(
    mut c: *mut client,
    mut id: u_int,
) -> *mut client_window {
    let mut cw: client_window = {
        let mut init = client_window {
            window: id,
            pane: 0 as *mut window_pane,
            entry: C2RustUnnamed_31 {
                rbe_left: 0 as *mut client_window,
                rbe_right: 0 as *mut client_window,
                rbe_parent: 0 as *mut client_window,
                rbe_color: 0,
            },
        };
        init
    };
    return client_windows_RB_FIND(&mut (*c).windows, &mut cw);
}
/* Get client active pane. */
#[no_mangle]
pub unsafe extern "C" fn server_client_get_pane(mut c: *mut client) -> *mut window_pane {
    let mut s: *mut session = (*c).session;
    let mut cw: *mut client_window = 0 as *mut client_window;
    if s.is_null() {
        return 0 as *mut window_pane;
    }
    if !(*c).flags as libc::c_ulonglong & 0x80000000 as libc::c_ulonglong != 0 {
        return (*(*(*s).curw).window).active;
    }
    cw = server_client_get_client_window(c, (*(*(*s).curw).window).id);
    if cw.is_null() {
        return (*(*(*s).curw).window).active;
    }
    return (*cw).pane;
}
/* Set client active pane. */
#[no_mangle]
pub unsafe extern "C" fn server_client_set_pane(mut c: *mut client, mut wp: *mut window_pane) {
    let mut s: *mut session = (*c).session;
    let mut cw: *mut client_window = 0 as *mut client_window;
    if s.is_null() {
        return;
    }
    cw = server_client_get_client_window(c, (*(*(*s).curw).window).id);
    if cw.is_null() {
        cw = xcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<client_window>() as libc::c_ulong,
        ) as *mut client_window;
        (*cw).window = (*(*(*s).curw).window).id;
        client_windows_RB_INSERT(&mut (*c).windows, cw);
    }
    (*cw).pane = wp;
    log_debug(
        b"%s pane now %%%u\x00" as *const u8 as *const libc::c_char,
        (*c).name,
        (*wp).id,
    );
}
/* Remove pane from client lists. */
#[no_mangle]
pub unsafe extern "C" fn server_client_remove_pane(mut wp: *mut window_pane) {
    let mut c: *mut client = 0 as *mut client;
    let mut w: *mut window = (*wp).window;
    let mut cw: *mut client_window = 0 as *mut client_window;
    c = clients.tqh_first;
    while !c.is_null() {
        cw = server_client_get_client_window(c, (*w).id);
        if !cw.is_null() && (*cw).pane == wp {
            client_windows_RB_REMOVE(&mut (*c).windows, cw);
            free(cw as *mut libc::c_void);
        }
        c = (*c).entry.tqe_next
    }
}
