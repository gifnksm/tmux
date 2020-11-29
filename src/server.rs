use ::libc;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type event_base;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn socketpair(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
        __fds: *mut libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t)
        -> libc::c_int;
    #[no_mangle]
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn umask(__mask: __mode_t) -> __mode_t;
    #[no_mangle]
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn killpg(__pgrp: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn event_reinit(base: *mut event_base) -> libc::c_int;
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
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fork() -> __pid_t;
    #[no_mangle]
    static mut global_options: *mut crate::options::options;
    #[no_mangle]
    static mut start_time: timeval;
    #[no_mangle]
    static mut socket_path: *const libc::c_char;
    #[no_mangle]
    fn setblocking(_: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn proc_start(_: *const libc::c_char) -> *mut crate::proc::tmuxproc;
    #[no_mangle]
    fn proc_loop(_: *mut crate::proc::tmuxproc, _: Option<unsafe extern "C" fn() -> libc::c_int>);
    #[no_mangle]
    fn proc_set_signals(
        _: *mut crate::proc::tmuxproc,
        _: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    );
    #[no_mangle]
    fn proc_clear_signals(_: *mut crate::proc::tmuxproc, _: libc::c_int);
    #[no_mangle]
    fn proc_toggle_log(_: *mut crate::proc::tmuxproc);
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
    fn job_check_died(_: pid_t, _: libc::c_int);
    #[no_mangle]
    fn job_kill_all();
    #[no_mangle]
    fn job_still_running() -> libc::c_int;
    #[no_mangle]
    fn tty_create_log();
    #[no_mangle]
    fn cmd_find_clear_state(_: *mut cmd_find_state, _: libc::c_int);
    #[no_mangle]
    fn cmd_find_valid_state(_: *mut cmd_find_state) -> libc::c_int;
    #[no_mangle]
    fn cmdq_get_error(_: *const libc::c_char) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_append(
        _: *mut client,
        _: *mut crate::cmd_queue::cmdq_item,
    ) -> *mut crate::cmd_queue::cmdq_item;
    #[no_mangle]
    fn cmdq_next(_: *mut client) -> u_int;
    #[no_mangle]
    fn cmd_wait_for_flush();
    #[no_mangle]
    fn key_bindings_init();
    #[no_mangle]
    fn server_client_create(_: libc::c_int) -> *mut client;
    #[no_mangle]
    fn server_client_lost(_: *mut client);
    #[no_mangle]
    fn server_client_loop();
    #[no_mangle]
    fn server_destroy_pane(_: *mut window_pane, _: libc::c_int);
    #[no_mangle]
    fn status_prompt_save_history();
    #[no_mangle]
    fn input_key_build();
    #[no_mangle]
    fn xvasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut sessions: sessions;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn sessions_RB_NEXT(_: *mut session) -> *mut session;
    #[no_mangle]
    fn sessions_RB_MINMAX(_: *mut sessions, _: libc::c_int) -> *mut session;
    #[no_mangle]
    static mut all_window_panes: window_pane_tree;
    #[no_mangle]
    static mut windows: windows;
    #[no_mangle]
    fn log_get_level() -> libc::c_int;
    #[no_mangle]
    fn window_pane_destroy_ready(_: *mut window_pane) -> libc::c_int;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn windows_RB_NEXT(_: *mut window) -> *mut window;
    #[no_mangle]
    fn windows_RB_MINMAX(_: *mut windows, _: libc::c_int) -> *mut window;
    #[no_mangle]
    fn session_destroy(_: *mut session, _: libc::c_int, _: *const libc::c_char);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
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
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type uint64_t = __uint64_t;
pub type va_list = __builtin_va_list;

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
    pub celldata: *mut crate::grid::CellEntry,
    pub extdsize: u_int,
    pub extddata: *mut crate::grid::ExtdEntry,
    pub flags: libc::c_int,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct message_entry {
    pub msg: *mut libc::c_char,
    pub msg_num: u_int,
    pub msg_time: timeval,
    pub entry: C2RustUnnamed_33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub tqe_next: *mut message_entry,
    pub tqe_prev: *mut *mut message_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct message_list {
    pub tqh_first: *mut message_entry,
    pub tqh_last: *mut *mut message_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clients {
    pub tqh_first: *mut client,
    pub tqh_last: *mut *mut client,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1i32, __path, __statbuf);
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
/*
 * Main server functions.
 */
#[no_mangle]
pub static mut clients: clients = clients {
    tqh_first: 0 as *mut client,
    tqh_last: 0 as *mut *mut client,
};
#[no_mangle]
pub static mut server_proc: *mut crate::proc::tmuxproc = 0 as *mut crate::proc::tmuxproc;
static mut server_fd: libc::c_int = -(1i32);
static mut server_client_flags: uint64_t = 0;
static mut server_exit: libc::c_int = 0;
static mut server_ev_accept: event = event {
    ev_evcallback: event_callback {
        evcb_active_next: C2RustUnnamed_8 {
            tqe_next: 0 as *mut event_callback,
            tqe_prev: 0 as *mut *mut event_callback,
        },
        evcb_flags: 0,
        evcb_pri: 0,
        evcb_closure: 0,
        evcb_cb_union: C2RustUnnamed_7 {
            evcb_callback: None,
        },
        evcb_arg: 0 as *mut libc::c_void,
    },
    ev_timeout_pos: C2RustUnnamed_5 {
        ev_next_with_common_timeout: C2RustUnnamed_6 {
            tqe_next: 0 as *mut event,
            tqe_prev: 0 as *mut *mut event,
        },
    },
    ev_fd: 0,
    ev_base: 0 as *mut event_base,
    ev_: C2RustUnnamed_0 {
        ev_io: C2RustUnnamed_3 {
            ev_io_next: C2RustUnnamed_4 {
                le_next: 0 as *mut event,
                le_prev: 0 as *mut *mut event,
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
#[no_mangle]
pub static mut marked_pane: cmd_find_state = cmd_find_state {
    flags: 0,
    current: 0 as *mut cmd_find_state,
    s: 0 as *mut session,
    wl: 0 as *mut winlink,
    w: 0 as *mut window,
    wp: 0 as *mut window_pane,
    idx: 0,
};
static mut message_next: u_int = 0;
#[no_mangle]
pub static mut message_log: message_list = message_list {
    tqh_first: 0 as *mut message_entry,
    tqh_last: 0 as *mut *mut message_entry,
};
/* Set marked pane. */
#[no_mangle]
pub unsafe extern "C" fn server_set_marked(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
) {
    cmd_find_clear_state(&mut marked_pane, 0i32);
    marked_pane.s = s;
    marked_pane.wl = wl;
    marked_pane.w = (*wl).window;
    marked_pane.wp = wp;
}
/* Clear marked pane. */
#[no_mangle]
pub unsafe extern "C" fn server_clear_marked() {
    cmd_find_clear_state(&mut marked_pane, 0i32);
}
/* Is this the marked pane? */
#[no_mangle]
pub unsafe extern "C" fn server_is_marked(
    mut s: *mut session,
    mut wl: *mut winlink,
    mut wp: *mut window_pane,
) -> libc::c_int {
    if s.is_null() || wl.is_null() || wp.is_null() {
        return 0i32;
    }
    if marked_pane.s != s || marked_pane.wl != wl {
        return 0i32;
    }
    if marked_pane.wp != wp {
        return 0i32;
    }
    return server_check_marked();
}
/* Check if the marked pane is still valid. */
#[no_mangle]
pub unsafe extern "C" fn server_check_marked() -> libc::c_int {
    return cmd_find_valid_state(&mut marked_pane);
}
/* Create server socket. */
unsafe extern "C" fn server_create_socket(
    mut flags: libc::c_int,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut size: size_t = 0;
    let mut mask: mode_t = 0;
    let mut fd: libc::c_int = 0;
    let mut saved_errno: libc::c_int = 0;
    memset(
        &mut sa as *mut sockaddr_un as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
    );
    sa.sun_family = 1u16;
    size = strlcpy(
        sa.sun_path.as_mut_ptr(),
        socket_path,
        ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    if size >= ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong {
        *__errno_location() = 36i32
    } else {
        unlink(sa.sun_path.as_mut_ptr());
        fd = socket(1i32, SOCK_STREAM as libc::c_int, 0i32);
        if !(fd == -(1i32)) {
            if flags & 0x8000000i32 != 0 {
                mask = umask(
                    (0o100i32 | 0o100i32 >> 3i32 | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32 >> 3i32)
                        as __mode_t,
                )
            } else {
                mask = umask(
                    (0o100i32
                        | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32
                        | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32 >> 3i32)
                        as __mode_t,
                )
            }
            if bind(
                fd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &mut sa as *mut sockaddr_un as *mut sockaddr,
                },
                ::std::mem::size_of::<sockaddr_un>() as socklen_t,
            ) == -(1i32)
            {
                saved_errno = *__errno_location();
                close(fd);
                *__errno_location() = saved_errno
            } else {
                umask(mask);
                if listen(fd, 128i32) == -(1i32) {
                    saved_errno = *__errno_location();
                    close(fd);
                    *__errno_location() = saved_errno
                } else {
                    setblocking(fd, 0i32);
                    return fd;
                }
            }
        }
    }
    if !cause.is_null() {
        xasprintf(
            cause,
            b"error creating %s (%s)\x00" as *const u8 as *const libc::c_char,
            socket_path,
            strerror(*__errno_location()),
        );
    }
    return -(1i32);
}
/* Fork new server. */
#[no_mangle]
pub unsafe extern "C" fn server_start(
    mut client: *mut crate::proc::tmuxproc,
    mut flags: libc::c_int,
    mut base: *mut event_base,
    mut lockfd: libc::c_int,
    mut lockfile: *mut libc::c_char,
) -> libc::c_int {
    let mut pair: [libc::c_int; 2] = [0; 2];
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    let mut c: *mut client = 0 as *mut client;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    sigfillset(&mut set);
    sigprocmask(0i32, &mut set, &mut oldset);
    if !flags & 0x40000000i32 != 0 {
        if socketpair(1i32, SOCK_STREAM as libc::c_int, 0i32, pair.as_mut_ptr()) != 0i32 {
            fatal(b"socketpair failed\x00" as *const u8 as *const libc::c_char);
        }
        match fork() {
            -1 => {
                fatal(b"fork failed\x00" as *const u8 as *const libc::c_char);
            }
            0 => {}
            _ => {
                sigprocmask(2i32, &mut oldset, 0 as *mut sigset_t);
                close(pair[1usize]);
                return pair[0usize];
            }
        }
        close(pair[0usize]);
        if daemon(1i32, 0i32) != 0i32 {
            fatal(b"daemon failed\x00" as *const u8 as *const libc::c_char);
        }
    }
    server_client_flags = flags as uint64_t;
    proc_clear_signals(client, 0i32);
    if event_reinit(base) != 0i32 {
        fatalx(b"event_reinit failed\x00" as *const u8 as *const libc::c_char);
    }
    server_proc = proc_start(b"server\x00" as *const u8 as *const libc::c_char);
    proc_set_signals(
        server_proc,
        Some(server_signal as unsafe extern "C" fn(_: libc::c_int) -> ()),
    );
    sigprocmask(2i32, &mut oldset, 0 as *mut sigset_t);
    if log_get_level() > 1i32 {
        tty_create_log();
    }
    if 0i32 != 0i32 {
        fatal(b"pledge failed\x00" as *const u8 as *const libc::c_char);
    }
    input_key_build();
    windows.rbh_root = 0 as *mut window;
    all_window_panes.rbh_root = 0 as *mut window_pane;
    clients.tqh_first = 0 as *mut client;
    clients.tqh_last = &mut clients.tqh_first;
    sessions.rbh_root = 0 as *mut session;
    key_bindings_init();
    message_log.tqh_first = 0 as *mut message_entry;
    message_log.tqh_last = &mut message_log.tqh_first;
    gettimeofday(&mut start_time, 0 as *mut libc::c_void);
    server_fd = server_create_socket(flags, &mut cause);
    if server_fd != -(1i32) {
        server_update_socket();
    }
    if !flags & 0x40000000i32 != 0 {
        c = server_client_create(pair[1usize])
    } else {
        options_set_number(
            global_options,
            b"exit-empty\x00" as *const u8 as *const libc::c_char,
            0i64,
        );
    }
    if lockfd >= 0i32 {
        unlink(lockfile);
        free(lockfile as *mut libc::c_void);
        close(lockfd);
    }
    if !cause.is_null() {
        if !c.is_null() {
            cmdq_append(c, cmdq_get_error(cause));
            (*c).flags |= 0x4u64
        }
        free(cause as *mut libc::c_void);
    }
    server_add_accept(0i32);
    proc_loop(
        server_proc,
        Some(server_loop as unsafe extern "C" fn() -> libc::c_int),
    );
    job_kill_all();
    status_prompt_save_history();
    exit(0i32);
}
/* Server loop callback. */
unsafe extern "C" fn server_loop() -> libc::c_int {
    let mut c: *mut client = 0 as *mut client;
    let mut items: u_int = 0;
    loop {
        items = cmdq_next(0 as *mut client);
        c = clients.tqh_first;
        while !c.is_null() {
            if (*c).flags & 0x40000u64 != 0 {
                items = (items).wrapping_add(cmdq_next(c))
            }
            c = (*c).entry.tqe_next
        }
        if !(items != 0u32) {
            break;
        }
    }
    server_client_loop();
    if options_get_number(
        global_options,
        b"exit-empty\x00" as *const u8 as *const libc::c_char,
    ) == 0
        && server_exit == 0
    {
        return 0i32;
    }
    if options_get_number(
        global_options,
        b"exit-unattached\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if !sessions.rbh_root.is_null() {
            return 0i32;
        }
    }
    c = clients.tqh_first;
    while !c.is_null() {
        if !(*c).session.is_null() {
            return 0i32;
        }
        c = (*c).entry.tqe_next
    }
    /*
     * No attached clients therefore want to exit - flush any waiting
     * clients but don't actually exit until they've gone.
     */
    cmd_wait_for_flush();
    if !clients.tqh_first.is_null() {
        return 0i32;
    }
    if job_still_running() != 0 {
        return 0i32;
    }
    return 1i32;
}
/* Exit the server by killing all clients and windows. */
unsafe extern "C" fn server_send_exit() {
    let mut c: *mut client = 0 as *mut client;
    let mut c1: *mut client = 0 as *mut client;
    let mut s: *mut session = 0 as *mut session;
    let mut s1: *mut session = 0 as *mut session;
    cmd_wait_for_flush();
    c = clients.tqh_first;
    while !c.is_null() && {
        c1 = (*c).entry.tqe_next;
        (1i32) != 0
    } {
        if (*c).flags & 0x40u64 != 0 {
            server_client_lost(c);
        } else {
            (*c).flags |= 0x4u64;
            (*c).exit_type = CLIENT_EXIT_SHUTDOWN
        }
        (*c).session = 0 as *mut session;
        c = c1
    }
    s = sessions_RB_MINMAX(&mut sessions, -(1i32));
    while !s.is_null() && {
        s1 = sessions_RB_NEXT(s);
        (1i32) != 0
    } {
        session_destroy(
            s,
            1i32,
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"server_send_exit\x00"))
                .as_ptr(),
        );
        s = s1
    }
}
/* Update socket execute permissions based on whether sessions are attached. */
#[no_mangle]
pub unsafe extern "C" fn server_update_socket() {
    let mut s: *mut session = 0 as *mut session;
    static mut last: libc::c_int = -(1i32);
    let mut n: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    n = 0i32;
    s = sessions_RB_MINMAX(&mut sessions, -(1i32));
    while !s.is_null() {
        if (*s).attached != 0u32 {
            n += 1;
            break;
        } else {
            s = sessions_RB_NEXT(s)
        }
    }
    if n != last {
        last = n;
        if stat(socket_path, &mut sb) != 0i32 {
            return;
        }
        mode = (sb.st_mode
            & (0o400i32
                | 0o200i32
                | 0o100i32
                | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32
                | (0o400i32 | 0o200i32 | 0o100i32) >> 3i32 >> 3i32) as libc::c_uint)
            as libc::c_int;
        if n != 0i32 {
            if mode & 0o400i32 != 0 {
                mode |= 0o100i32
            }
            if mode & 0o400i32 >> 3i32 != 0 {
                mode |= 0o100i32 >> 3i32
            }
            if mode & 0o400i32 >> 3i32 >> 3i32 != 0 {
                mode |= 0o100i32 >> 3i32 >> 3i32
            }
        } else {
            mode &= !(0o100i32 | 0o100i32 >> 3i32 | 0o100i32 >> 3i32 >> 3i32)
        }
        chmod(socket_path, mode as __mode_t);
    };
}
/* Callback for server socket. */
unsafe extern "C" fn server_accept(
    mut fd: libc::c_int,
    mut events: libc::c_short,
    mut _data: *mut libc::c_void,
) {
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut slen: socklen_t = ::std::mem::size_of::<sockaddr_storage>() as socklen_t;
    let mut newfd: libc::c_int = 0;
    server_add_accept(0i32);
    if events as libc::c_int & 0x2i32 == 0 {
        return;
    }
    newfd = accept(
        fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut sa as *mut sockaddr_storage as *mut sockaddr,
        },
        &mut slen,
    );
    if newfd == -(1i32) {
        if *__errno_location() == 11i32
            || *__errno_location() == 4i32
            || *__errno_location() == 103i32
        {
            return;
        }
        if *__errno_location() == 23i32 || *__errno_location() == 24i32 {
            /* Delete and don't try again for 1 second. */
            server_add_accept(1i32);
            return;
        }
        fatal(b"accept failed\x00" as *const u8 as *const libc::c_char);
    }
    if server_exit != 0 {
        close(newfd);
        return;
    }
    server_client_create(newfd);
}
/*
 * Add accept event. If timeout is nonzero, add as a timeout instead of a read
 * event - used to backoff when running out of file descriptors.
 */
#[no_mangle]
pub unsafe extern "C" fn server_add_accept(mut timeout: libc::c_int) {
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: timeout as __time_t,
            tv_usec: 0i64,
        };
        init
    };
    if server_fd == -(1i32) {
        return;
    }
    if event_initialized(&mut server_ev_accept) != 0 {
        event_del(&mut server_ev_accept);
    }
    if timeout == 0i32 {
        event_set(
            &mut server_ev_accept,
            server_fd,
            0x2i16,
            Some(
                server_accept
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        event_add(&mut server_ev_accept, 0 as *const timeval);
    } else {
        event_set(
            &mut server_ev_accept,
            server_fd,
            0x1i16,
            Some(
                server_accept
                    as unsafe extern "C" fn(
                        _: libc::c_int,
                        _: libc::c_short,
                        _: *mut libc::c_void,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        );
        event_add(&mut server_ev_accept, &mut tv);
    };
}
/* Signal handler. */
unsafe extern "C" fn server_signal(mut sig: libc::c_int) {
    let mut fd: libc::c_int = 0;
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"server_signal\x00")).as_ptr(),
        strsignal(sig),
    );
    match sig {
        2 | 15 => {
            server_exit = 1i32;
            server_send_exit();
        }
        17 => {
            server_child_signal();
        }
        10 => {
            event_del(&mut server_ev_accept);
            fd = server_create_socket(
                server_client_flags as libc::c_int,
                0 as *mut *mut libc::c_char,
            );
            if fd != -(1i32) {
                close(server_fd);
                server_fd = fd;
                server_update_socket();
            }
            server_add_accept(0i32);
        }
        12 => {
            proc_toggle_log(server_proc);
        }
        _ => {}
    };
}
/* Handle SIGCHLD. */
unsafe extern "C" fn server_child_signal() {
    let mut status: libc::c_int = 0;
    let mut pid: pid_t = 0;
    loop {
        pid = waitpid(-(1i32), &mut status, 1i32 | 2i32);
        match pid {
            -1 => {
                if *__errno_location() == 10i32 {
                    return;
                }
                fatal(b"waitpid failed\x00" as *const u8 as *const libc::c_char);
            }
            0 => return,
            _ => {}
        }
        if status & 0xffi32 == 0x7fi32 {
            server_child_stopped(pid, status);
        } else if status & 0x7fi32 == 0i32
            || ((status & 0x7fi32) + 1i32) as libc::c_schar as libc::c_int >> 1i32 > 0i32
        {
            server_child_exited(pid, status);
        }
    }
}
/* Handle exited children. */
unsafe extern "C" fn server_child_exited(mut pid: pid_t, mut status: libc::c_int) {
    let mut w: *mut window = 0 as *mut window;
    let mut w1: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    w = windows_RB_MINMAX(&mut windows, -(1i32));
    while !w.is_null() && {
        w1 = windows_RB_NEXT(w);
        (1i32) != 0
    } {
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if (*wp).pid == pid {
                (*wp).status = status;
                (*wp).flags |= 0x200i32;
                log_debug(
                    b"%%%u exited\x00" as *const u8 as *const libc::c_char,
                    (*wp).id,
                );
                (*wp).flags |= 0x100i32;
                if window_pane_destroy_ready(wp) != 0 {
                    server_destroy_pane(wp, 1i32);
                }
                break;
            } else {
                wp = (*wp).entry.tqe_next
            }
        }
        w = w1
    }
    job_check_died(pid, status);
}
/* Handle stopped children. */
unsafe extern "C" fn server_child_stopped(mut pid: pid_t, mut status: libc::c_int) {
    let mut w: *mut window = 0 as *mut window;
    let mut wp: *mut window_pane = 0 as *mut window_pane;
    if (status & 0xff00i32) >> 8i32 == 21i32 || (status & 0xff00i32) >> 8i32 == 22i32 {
        return;
    }
    w = windows_RB_MINMAX(&mut windows, -(1i32));
    while !w.is_null() {
        wp = (*w).panes.tqh_first;
        while !wp.is_null() {
            if (*wp).pid == pid {
                if killpg(pid, 18i32) != 0i32 {
                    kill(pid, 18i32);
                }
            }
            wp = (*wp).entry.tqe_next
        }
        w = windows_RB_NEXT(w)
    }
    job_check_died(pid, status);
}
/* Add to message log. */
#[no_mangle]
pub unsafe extern "C" fn server_add_message(mut fmt: *const libc::c_char, mut args: ...) {
    let mut msg: *mut message_entry = 0 as *mut message_entry;
    let mut msg1: *mut message_entry = 0 as *mut message_entry;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    let mut limit: u_int = 0;
    ap = args.clone();
    xvasprintf(&mut s, fmt, ap.as_va_list());
    log_debug(b"message: %s\x00" as *const u8 as *const libc::c_char, s);
    msg = xcalloc(
        1u64,
        ::std::mem::size_of::<message_entry>() as libc::c_ulong,
    ) as *mut message_entry;
    gettimeofday(&mut (*msg).msg_time, 0 as *mut libc::c_void);
    let fresh0 = message_next;
    message_next = message_next.wrapping_add(1);
    (*msg).msg_num = fresh0;
    (*msg).msg = s;
    (*msg).entry.tqe_next = 0 as *mut message_entry;
    (*msg).entry.tqe_prev = message_log.tqh_last;
    *message_log.tqh_last = msg;
    message_log.tqh_last = &mut (*msg).entry.tqe_next;
    limit = options_get_number(
        global_options,
        b"message-limit\x00" as *const u8 as *const libc::c_char,
    ) as u_int;
    msg = message_log.tqh_first;
    while !msg.is_null() && {
        msg1 = (*msg).entry.tqe_next;
        (1i32) != 0
    } {
        if (*msg).msg_num.wrapping_add(limit) >= message_next {
            break;
        }
        free((*msg).msg as *mut libc::c_void);
        if !(*msg).entry.tqe_next.is_null() {
            (*(*msg).entry.tqe_next).entry.tqe_prev = (*msg).entry.tqe_prev
        } else {
            message_log.tqh_last = (*msg).entry.tqe_prev
        }
        *(*msg).entry.tqe_prev = (*msg).entry.tqe_next;
        free(msg as *mut libc::c_void);
        msg = msg1
    }
}
