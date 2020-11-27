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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type evbuffer;
    pub type bufferevent_ops;
    #[no_mangle]
    fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> libc::c_int;
    #[no_mangle]
    fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    #[no_mangle]
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    #[no_mangle]
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    #[no_mangle]
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn evbuffer_pullup(buf: *mut evbuffer, size: ssize_t) -> *mut libc::c_uchar;
    #[no_mangle]
    fn bufferevent_free(bufev: *mut bufferevent);
    #[no_mangle]
    fn bufferevent_write(
        bufev: *mut bufferevent,
        data: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_enable(bufev: *mut bufferevent, event: libc::c_short) -> libc::c_int;
    #[no_mangle]
    fn bufferevent_new(
        fd: libc::c_int,
        readcb: bufferevent_data_cb,
        writecb: bufferevent_data_cb,
        errorcb: bufferevent_event_cb,
        cbarg: *mut libc::c_void,
    ) -> *mut bufferevent;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn system(__command: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn dup(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    static mut environ: *mut *mut libc::c_char;
    #[no_mangle]
    fn execl(__path: *const libc::c_char, __arg: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn getppid() -> __pid_t;
    #[no_mangle]
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut crate::options::options;
    #[no_mangle]
    static mut global_s_options: *mut crate::options::options;
    #[no_mangle]
    static mut global_w_options: *mut crate::options::options;
    #[no_mangle]
    static mut global_environ: *mut crate::environ::environ;
    #[no_mangle]
    static mut socket_path: *const libc::c_char;
    #[no_mangle]
    static mut shell_command: *const libc::c_char;
    #[no_mangle]
    static mut ptm_fd: libc::c_int;
    #[no_mangle]
    fn setblocking(_: libc::c_int, _: libc::c_int);
    #[no_mangle]
    fn find_cwd() -> *const libc::c_char;
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
    fn proc_start(_: *const libc::c_char) -> *mut crate::proc::tmuxproc;
    #[no_mangle]
    fn proc_loop(_: *mut crate::proc::tmuxproc, _: Option<unsafe extern "C" fn() -> libc::c_int>);
    #[no_mangle]
    fn proc_exit(_: *mut crate::proc::tmuxproc);
    #[no_mangle]
    fn proc_set_signals(
        _: *mut crate::proc::tmuxproc,
        _: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    );
    #[no_mangle]
    fn proc_clear_signals(_: *mut crate::proc::tmuxproc, _: libc::c_int);
    #[no_mangle]
    fn proc_add_peer(
        _: *mut crate::proc::tmuxproc,
        _: libc::c_int,
        _: Option<unsafe extern "C" fn(_: *mut imsg, _: *mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> *mut crate::proc::tmuxpeer;
    #[no_mangle]
    fn options_free(_: *mut crate::options::options);
    #[no_mangle]
    fn environ_free(_: *mut crate::environ::environ);
    #[no_mangle]
    fn cmd_pack_argv(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut libc::c_char,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list);
    #[no_mangle]
    fn cmd_list_any_have(_: *mut cmd_list, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn cmd_parse_from_arguments(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
        _: *mut cmd_parse_input,
    ) -> *mut cmd_parse_result;
    #[no_mangle]
    fn client_files_RB_NEXT(_: *mut client_file) -> *mut client_file;
    #[no_mangle]
    fn client_files_RB_INSERT(_: *mut client_files, _: *mut client_file) -> *mut client_file;
    #[no_mangle]
    fn client_files_RB_MINMAX(_: *mut client_files, _: libc::c_int) -> *mut client_file;
    #[no_mangle]
    fn client_files_RB_FIND(_: *mut client_files, _: *mut client_file) -> *mut client_file;
    #[no_mangle]
    fn client_files_RB_REMOVE(_: *mut client_files, _: *mut client_file) -> *mut client_file;
    #[no_mangle]
    fn file_create(
        _: *mut client,
        _: libc::c_int,
        _: client_file_cb,
        _: *mut libc::c_void,
    ) -> *mut client_file;
    #[no_mangle]
    fn file_free(_: *mut client_file);
    #[no_mangle]
    fn server_start(
        _: *mut crate::proc::tmuxproc,
        _: libc::c_int,
        _: *mut event_base,
        _: libc::c_int,
        _: *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn log_debug(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn cfmakeraw(__termios_p: *mut termios);
    #[no_mangle]
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn cfsetispeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    #[no_mangle]
    fn cfsetospeed(__termios_p: *mut termios, __speed: speed_t) -> libc::c_int;
    #[no_mangle]
    fn cfgetispeed(__termios_p: *const termios) -> speed_t;
    #[no_mangle]
    fn cfgetospeed(__termios_p: *const termios) -> speed_t;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn closefrom(_: libc::c_int);
    #[no_mangle]
    fn strlcpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_0 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction:
        Option<unsafe extern "C" fn(_: libc::c_int, _: *mut siginfo_t, _: *mut libc::c_void) -> ()>,
}
pub type uint64_t = __uint64_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_16,
    pub ev_fd: libc::c_int,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_11,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_11 {
    pub ev_io: C2RustUnnamed_14,
    pub ev_signal: C2RustUnnamed_12,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_12 {
    pub ev_signal_next: C2RustUnnamed_13,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_13 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_14 {
    pub ev_io_next: C2RustUnnamed_15,
    pub ev_timeout: timeval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_15 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_16 {
    pub ev_next_with_common_timeout: C2RustUnnamed_17,
    pub min_heap_idx: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_17 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_19,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_18,
    pub evcb_arg: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_18 {
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
pub struct C2RustUnnamed_19 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
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
    pub exit_type: C2RustUnnamed_40,
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
    pub prompt_mode: C2RustUnnamed_37,
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
    pub entry: C2RustUnnamed_20,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_20 {
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
    pub entry: C2RustUnnamed_21,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_21 {
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
    pub c2rust_unnamed: C2RustUnnamed_22,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_22 {
    pub offset: u_int,
    pub data: C2RustUnnamed_23,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_23 {
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
    pub gentry: C2RustUnnamed_25,
    pub entry: C2RustUnnamed_24,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_24 {
    pub rbe_left: *mut session,
    pub rbe_right: *mut session,
    pub rbe_parent: *mut session,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_25 {
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
    pub entry: C2RustUnnamed_28,
    pub wentry: C2RustUnnamed_27,
    pub sentry: C2RustUnnamed_26,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_26 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_27 {
    pub tqe_next: *mut winlink,
    pub tqe_prev: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_28 {
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
    pub alerts_entry: C2RustUnnamed_31,
    pub options: *mut crate::options::options,
    pub references: u_int,
    pub winlinks: C2RustUnnamed_30,
    pub entry: C2RustUnnamed_29,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_29 {
    pub rbe_left: *mut window,
    pub rbe_right: *mut window,
    pub rbe_parent: *mut window,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_30 {
    pub tqh_first: *mut winlink,
    pub tqh_last: *mut *mut winlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_31 {
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
    pub entry: C2RustUnnamed_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_32 {
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
    pub modes: C2RustUnnamed_35,
    pub searchstr: *mut libc::c_char,
    pub searchregex: libc::c_int,
    pub written: size_t,
    pub skipped: size_t,
    pub border_gc_set: libc::c_int,
    pub border_gc: grid_cell,
    pub entry: C2RustUnnamed_34,
    pub tree_entry: C2RustUnnamed_33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_33 {
    pub rbe_left: *mut window_pane,
    pub rbe_right: *mut window_pane,
    pub rbe_parent: *mut window_pane,
    pub rbe_color: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_34 {
    pub tqe_next: *mut window_pane,
    pub tqe_prev: *mut *mut window_pane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_35 {
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
    pub entry: C2RustUnnamed_36,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_36 {
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
pub type C2RustUnnamed_37 = libc::c_uint;
pub const PROMPT_COMMAND: C2RustUnnamed_37 = 1;
pub const PROMPT_ENTRY: C2RustUnnamed_37 = 0;
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
    pub entry: C2RustUnnamed_38,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_38 {
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
    pub entry: C2RustUnnamed_39,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_39 {
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
pub type C2RustUnnamed_40 = libc::c_uint;
pub const CLIENT_EXIT_DETACH: C2RustUnnamed_40 = 2;
pub const CLIENT_EXIT_SHUTDOWN: C2RustUnnamed_40 = 1;
pub const CLIENT_EXIT_RETURN: C2RustUnnamed_40 = 0;

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
    pub entry: C2RustUnnamed_41,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_41 {
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
    pub entry: C2RustUnnamed_42,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_42 {
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
    pub entry: C2RustUnnamed_43,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_43 {
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
pub struct msg_read_open {
    pub stream: libc::c_int,
    pub fd: libc::c_int,
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
pub struct msg_write_open {
    pub stream: libc::c_int,
    pub fd: libc::c_int,
    pub flags: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_write_data {
    pub stream: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_write_ready {
    pub stream: libc::c_int,
    pub error: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_write_close {
    pub stream: libc::c_int,
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
pub const CLIENT_EXIT_MESSAGE_PROVIDED: C2RustUnnamed_44 = 8;
pub const CLIENT_EXIT_SERVER_EXITED: C2RustUnnamed_44 = 7;
pub const CLIENT_EXIT_EXITED: C2RustUnnamed_44 = 6;
pub const CLIENT_EXIT_LOST_SERVER: C2RustUnnamed_44 = 5;
pub const CLIENT_EXIT_TERMINATED: C2RustUnnamed_44 = 4;
pub const CLIENT_EXIT_LOST_TTY: C2RustUnnamed_44 = 3;
pub const CLIENT_EXIT_DETACHED_HUP: C2RustUnnamed_44 = 2;
pub const CLIENT_EXIT_DETACHED: C2RustUnnamed_44 = 1;
pub const CLIENT_EXIT_NONE: C2RustUnnamed_44 = 0;
pub type C2RustUnnamed_44 = libc::c_uint;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
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
static mut client_proc: *mut crate::proc::tmuxproc =
    0 as *const crate::proc::tmuxproc as *mut crate::proc::tmuxproc;
static mut client_peer: *mut crate::proc::tmuxpeer =
    0 as *const crate::proc::tmuxpeer as *mut crate::proc::tmuxpeer;
static mut client_flags: uint64_t = 0;
static mut client_suspended: libc::c_int = 0;
static mut client_exitreason: C2RustUnnamed_44 = CLIENT_EXIT_NONE;
static mut client_exitflag: libc::c_int = 0;
static mut client_exitval: libc::c_int = 0;
static mut client_exittype: msgtype = 0 as msgtype;
static mut client_exitsession: *const libc::c_char = 0 as *const libc::c_char;
static mut client_exitmessage: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut client_execshell: *const libc::c_char = 0 as *const libc::c_char;
static mut client_execcmd: *const libc::c_char = 0 as *const libc::c_char;
static mut client_attached: libc::c_int = 0;
static mut client_files: client_files = {
    let mut init = client_files {
        rbh_root: 0 as *const client_file as *mut client_file,
    };
    init
};
/*
 * Get server create lock. If already held then server start is happening in
 * another client, so block until the lock is released and return -2 to
 * retry. Return -1 on failure to continue and start the server anyway.
 */
unsafe extern "C" fn client_get_lock(mut lockfile: *mut libc::c_char) -> libc::c_int {
    let mut lockfd: libc::c_int = 0;
    log_debug(
        b"lock file is %s\x00" as *const u8 as *const libc::c_char,
        lockfile,
    );
    lockfd = open(
        lockfile,
        0o1 as libc::c_int | 0o100 as libc::c_int,
        0o600 as libc::c_int,
    );
    if lockfd == -(1 as libc::c_int) {
        log_debug(
            b"open failed: %s\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if flock(lockfd, 2 as libc::c_int | 4 as libc::c_int) == -(1 as libc::c_int) {
        log_debug(
            b"flock failed: %s\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if *__errno_location() != 11 as libc::c_int {
            return lockfd;
        }
        /* nothing */
        while flock(lockfd, 2 as libc::c_int) == -(1 as libc::c_int)
            && *__errno_location() == 4 as libc::c_int
        {}
        close(lockfd);
        return -(2 as libc::c_int);
    }
    log_debug(b"flock succeeded\x00" as *const u8 as *const libc::c_char);
    return lockfd;
}
/* Connect client to server. */
unsafe extern "C" fn client_connect(
    mut base: *mut event_base,
    mut path: *const libc::c_char,
    mut flags: uint64_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut size: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut lockfd: libc::c_int = -(1 as libc::c_int);
    let mut locked: libc::c_int = 0 as libc::c_int;
    let mut lockfile: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(
        &mut sa as *mut sockaddr_un as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
    );
    sa.sun_family = 1 as libc::c_int as sa_family_t;
    size = strlcpy(
        sa.sun_path.as_mut_ptr(),
        path,
        ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
    );
    if size >= ::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int);
    }
    log_debug(
        b"socket is %s\x00" as *const u8 as *const libc::c_char,
        path,
    );
    loop {
        fd = socket(
            1 as libc::c_int,
            SOCK_STREAM as libc::c_int,
            0 as libc::c_int,
        );
        if fd == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        log_debug(b"trying connect\x00" as *const u8 as *const libc::c_char);
        if !(connect(
            fd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &mut sa as *mut sockaddr_un as *mut sockaddr,
            },
            ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
        ) == -(1 as libc::c_int))
        {
            current_block = 2569451025026770673;
            break;
        }
        log_debug(
            b"connect failed: %s\x00" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        if *__errno_location() != 111 as libc::c_int && *__errno_location() != 2 as libc::c_int {
            current_block = 12517108148661954305;
            break;
        }
        if !flags & 0x10000000 as libc::c_int as libc::c_ulong != 0 {
            current_block = 12517108148661954305;
            break;
        }
        close(fd);
        if locked == 0 {
            xasprintf(
                &mut lockfile as *mut *mut libc::c_char,
                b"%s.lock\x00" as *const u8 as *const libc::c_char,
                path,
            );
            lockfd = client_get_lock(lockfile);
            if lockfd < 0 as libc::c_int {
                log_debug(
                    b"didn\'t get lock (%d)\x00" as *const u8 as *const libc::c_char,
                    lockfd,
                );
                free(lockfile as *mut libc::c_void);
                lockfile = 0 as *mut libc::c_char;
                if lockfd == -(2 as libc::c_int) {
                    continue;
                }
            }
            log_debug(
                b"got lock (%d)\x00" as *const u8 as *const libc::c_char,
                lockfd,
            );
            /*
             * Always retry at least once, even if we got the lock,
             * because another client could have taken the lock,
             * started the server and released the lock between our
             * connect() and flock().
             */
            locked = 1 as libc::c_int
        } else {
            if lockfd >= 0 as libc::c_int
                && unlink(path) != 0 as libc::c_int
                && *__errno_location() != 2 as libc::c_int
            {
                free(lockfile as *mut libc::c_void);
                close(lockfd);
                return -(1 as libc::c_int);
            }
            fd = server_start(client_proc, flags as libc::c_int, base, lockfd, lockfile);
            current_block = 2569451025026770673;
            break;
        }
    }
    match current_block {
        12517108148661954305 => {
            if locked != 0 {
                free(lockfile as *mut libc::c_void);
                close(lockfd);
            }
            close(fd);
            return -(1 as libc::c_int);
        }
        _ => {
            if locked != 0 && lockfd >= 0 as libc::c_int {
                free(lockfile as *mut libc::c_void);
                close(lockfd);
            }
            setblocking(fd, 0 as libc::c_int);
            return fd;
        }
    };
}
/* Get exit string from reason number. */
unsafe extern "C" fn client_exit_message() -> *const libc::c_char {
    static mut msg: [libc::c_char; 256] = [0; 256];
    match client_exitreason as libc::c_uint {
        1 => {
            if !client_exitsession.is_null() {
                xsnprintf(
                    msg.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"detached (from session %s)\x00" as *const u8 as *const libc::c_char,
                    client_exitsession,
                );
                return msg.as_mut_ptr();
            }
            return b"detached\x00" as *const u8 as *const libc::c_char;
        }
        2 => {
            if !client_exitsession.is_null() {
                xsnprintf(
                    msg.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"detached and SIGHUP (from session %s)\x00" as *const u8
                        as *const libc::c_char,
                    client_exitsession,
                );
                return msg.as_mut_ptr();
            }
            return b"detached and SIGHUP\x00" as *const u8 as *const libc::c_char;
        }
        3 => return b"lost tty\x00" as *const u8 as *const libc::c_char,
        4 => return b"terminated\x00" as *const u8 as *const libc::c_char,
        5 => return b"server exited unexpectedly\x00" as *const u8 as *const libc::c_char,
        6 => return b"exited\x00" as *const u8 as *const libc::c_char,
        7 => return b"server exited\x00" as *const u8 as *const libc::c_char,
        8 => return client_exitmessage,
        0 | _ => {}
    }
    return b"unknown reason\x00" as *const u8 as *const libc::c_char;
}
/* Exit if all streams flushed. */
unsafe extern "C" fn client_exit() {
    let mut cf: *mut client_file = 0 as *mut client_file;
    let mut left: size_t = 0;
    let mut waiting: libc::c_int = 0 as libc::c_int;
    cf = client_files_RB_MINMAX(&mut client_files, -(1 as libc::c_int));
    while !cf.is_null() {
        if !(*cf).event.is_null() {
            left = evbuffer_get_length((*(*cf).event).output);
            if left != 0 as libc::c_int as libc::c_ulong {
                waiting += 1;
                log_debug(
                    b"file %u %zu bytes left\x00" as *const u8 as *const libc::c_char,
                    (*cf).stream,
                    left,
                );
            }
        }
        cf = client_files_RB_NEXT(cf)
    }
    if waiting == 0 as libc::c_int {
        proc_exit(client_proc);
    };
}
/* Client main loop. */
#[no_mangle]
pub unsafe extern "C" fn client_main(
    mut base: *mut event_base,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut flags: uint64_t,
    mut feat: libc::c_int,
) -> libc::c_int {
    let mut pr: *mut cmd_parse_result = 0 as *mut cmd_parse_result;
    let mut data: *mut msg_command = 0 as *mut msg_command;
    let mut fd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ttynam: *const libc::c_char = 0 as *const libc::c_char;
    let mut cwd: *const libc::c_char = 0 as *const libc::c_char;
    let mut ppid: pid_t = 0;
    let mut msg: msgtype = 0 as msgtype;
    let mut tio: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut saved_tio: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut size: size_t = 0;
    let mut linesize: size_t = 0 as libc::c_int as size_t;
    let mut linelen: ssize_t = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Ignore SIGCHLD now or daemon() in the server will leave a zombie. */
    signal(
        17 as libc::c_int,
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(1 as libc::c_int as libc::intptr_t),
    );
    /* Set up the initial command. */
    if !shell_command.is_null() {
        msg = MSG_SHELL;
        flags |= 0x10000000 as libc::c_int as libc::c_ulong
    } else if argc == 0 as libc::c_int {
        msg = MSG_COMMAND;
        flags |= 0x10000000 as libc::c_int as libc::c_ulong
    } else {
        msg = MSG_COMMAND;
        /*
         * It sucks parsing the command string twice (in client and
         * later in server) but it is necessary to get the start server
         * flag.
         */
        pr = cmd_parse_from_arguments(argc, argv, 0 as *mut cmd_parse_input);
        if (*pr).status as libc::c_uint == CMD_PARSE_SUCCESS as libc::c_int as libc::c_uint {
            if cmd_list_any_have((*pr).cmdlist, 0x1 as libc::c_int) != 0 {
                flags |= 0x10000000 as libc::c_int as libc::c_ulong
            }
            cmd_list_free((*pr).cmdlist);
        } else {
            free((*pr).error as *mut libc::c_void);
        }
    }
    /* Create client process structure (starts logging). */
    client_proc = proc_start(b"client\x00" as *const u8 as *const libc::c_char);
    proc_set_signals(
        client_proc,
        Some(client_signal as unsafe extern "C" fn(_: libc::c_int) -> ()),
    );
    /* Save the flags. */
    client_flags = flags;
    log_debug(
        b"flags are %#llx\x00" as *const u8 as *const libc::c_char,
        client_flags as libc::c_ulonglong,
    );
    /* Initialize the client socket and start the server. */
    fd = client_connect(base, socket_path, client_flags);
    if fd == -(1 as libc::c_int) {
        if *__errno_location() == 111 as libc::c_int {
            fprintf(
                stderr,
                b"no server running on %s\n\x00" as *const u8 as *const libc::c_char,
                socket_path,
            );
        } else {
            fprintf(
                stderr,
                b"error connecting to %s (%s)\n\x00" as *const u8 as *const libc::c_char,
                socket_path,
                strerror(*__errno_location()),
            );
        }
        return 1 as libc::c_int;
    }
    client_peer = proc_add_peer(
        client_proc,
        fd,
        Some(client_dispatch as unsafe extern "C" fn(_: *mut imsg, _: *mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
    /* Save these before pledge(). */
    cwd = find_cwd();
    if cwd.is_null() && {
        cwd = find_home();
        cwd.is_null()
    } {
        cwd = b"/\x00" as *const u8 as *const libc::c_char
    }
    ttynam = ttyname(0 as libc::c_int);
    if ttynam.is_null() {
        ttynam = b"\x00" as *const u8 as *const libc::c_char
    }
    /*
     * Drop privileges for client. "proc exec" is needed for -c and for
     * locking (which uses system(3)).
     *
     * "tty" is needed to restore termios(4) and also for some reason -CC
     * does not work properly without it (input is not recognised).
     *
     * "sendfd" is dropped later in client_dispatch_wait().
     */
    if 0 as libc::c_int != 0 as libc::c_int {
        fatal(b"pledge failed\x00" as *const u8 as *const libc::c_char);
    }
    /* Free stuff that is not used in the client. */
    if ptm_fd != -(1 as libc::c_int) {
        close(ptm_fd);
    }
    options_free(global_options);
    options_free(global_s_options);
    options_free(global_w_options);
    environ_free(global_environ);
    /* Set up control mode. */
    if client_flags & 0x4000 as libc::c_int as libc::c_ulong != 0 {
        if tcgetattr(0 as libc::c_int, &mut saved_tio) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"tcgetattr failed: %s\n\x00" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return 1 as libc::c_int;
        }
        cfmakeraw(&mut tio);
        tio.c_iflag = (0o400 as libc::c_int | 0o4000 as libc::c_int) as tcflag_t;
        tio.c_oflag = (0o1 as libc::c_int | 0o4 as libc::c_int) as tcflag_t;
        tio.c_cflag =
            (0o200 as libc::c_int | 0o60 as libc::c_int | 0o2000 as libc::c_int) as tcflag_t;
        tio.c_cc[6 as libc::c_int as usize] = 1 as libc::c_int as cc_t;
        tio.c_cc[5 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
        cfsetispeed(&mut tio, cfgetispeed(&mut saved_tio));
        cfsetospeed(&mut tio, cfgetospeed(&mut saved_tio));
        tcsetattr(0 as libc::c_int, 0 as libc::c_int, &mut tio);
    }
    /* Send identify messages. */
    client_send_identify(ttynam, cwd, feat);
    /* Send first command. */
    if msg as libc::c_uint == MSG_COMMAND as libc::c_int as libc::c_uint {
        /* How big is the command? */
        size = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int;
        while i < argc {
            size = (size as libc::c_ulong).wrapping_add(
                strlen(*argv.offset(i as isize)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
            i += 1
        }
        if size
            > (16384 as libc::c_int as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<msg_command>() as libc::c_ulong)
        {
            fprintf(
                stderr,
                b"command too long\n\x00" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        data = xmalloc((::std::mem::size_of::<msg_command>() as libc::c_ulong).wrapping_add(size))
            as *mut msg_command;
        /* Prepare command for server. */
        (*data).argc = argc;
        if cmd_pack_argv(
            argc,
            argv,
            data.offset(1 as libc::c_int as isize) as *mut libc::c_char,
            size,
        ) != 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"command too long\n\x00" as *const u8 as *const libc::c_char,
            );
            free(data as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        size = (size as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<msg_command>() as libc::c_ulong)
            as size_t as size_t;
        /* Send the command. */
        if proc_send(
            client_peer,
            msg,
            -(1 as libc::c_int),
            data as *const libc::c_void,
            size,
        ) != 0 as libc::c_int
        {
            fprintf(
                stderr,
                b"failed to send command\n\x00" as *const u8 as *const libc::c_char,
            );
            free(data as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        free(data as *mut libc::c_void);
    } else if msg as libc::c_uint == MSG_SHELL as libc::c_int as libc::c_uint {
        proc_send(
            client_peer,
            msg,
            -(1 as libc::c_int),
            0 as *const libc::c_void,
            0 as libc::c_int as size_t,
        );
    }
    /* Start main loop. */
    proc_loop(client_proc, None);
    /* Run command if user requested exec, instead of exiting. */
    if client_exittype as libc::c_uint == MSG_EXEC as libc::c_int as libc::c_uint {
        if client_flags & 0x4000 as libc::c_int as libc::c_ulong != 0 {
            tcsetattr(1 as libc::c_int, 2 as libc::c_int, &mut saved_tio);
        }
        client_exec(client_execshell, client_execcmd);
    }
    /* Restore streams to blocking. */
    setblocking(0 as libc::c_int, 1 as libc::c_int);
    setblocking(1 as libc::c_int, 1 as libc::c_int);
    setblocking(2 as libc::c_int, 1 as libc::c_int);
    /* Print the exit message, if any, and exit. */
    if client_attached != 0 {
        if client_exitreason as libc::c_uint != CLIENT_EXIT_NONE as libc::c_int as libc::c_uint {
            printf(
                b"[%s]\n\x00" as *const u8 as *const libc::c_char,
                client_exit_message(),
            );
        }
        ppid = getppid();
        if client_exittype as libc::c_uint == MSG_DETACHKILL as libc::c_int as libc::c_uint
            && ppid > 1 as libc::c_int
        {
            kill(ppid, 1 as libc::c_int);
        }
    } else if client_flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
        if client_exitreason as libc::c_uint != CLIENT_EXIT_NONE as libc::c_int as libc::c_uint {
            printf(
                b"%%exit %s\n\x00" as *const u8 as *const libc::c_char,
                client_exit_message(),
            );
        } else {
            printf(b"%%exit\n\x00" as *const u8 as *const libc::c_char);
        }
        fflush(stdout);
        if client_flags as libc::c_ulonglong & 0x200000000 as libc::c_ulonglong != 0 {
            setvbuf(
                stdin,
                0 as *mut libc::c_char,
                1 as libc::c_int,
                0 as libc::c_int as size_t,
            );
            loop {
                linelen = getline(&mut line, &mut linesize, stdin);
                if linelen <= 1 as libc::c_int as libc::c_long {
                    break;
                }
            }
            free(line as *mut libc::c_void);
        }
        if client_flags & 0x4000 as libc::c_int as libc::c_ulong != 0 {
            printf(b"\x1b\\\x00" as *const u8 as *const libc::c_char);
            fflush(stdout);
            tcsetattr(1 as libc::c_int, 2 as libc::c_int, &mut saved_tio);
        }
    } else if client_exitreason as libc::c_uint != CLIENT_EXIT_NONE as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            client_exit_message(),
        );
    }
    return client_exitval;
}
/* Send identify messages to server. */
unsafe extern "C" fn client_send_identify(
    mut ttynam: *const libc::c_char,
    mut cwd: *const libc::c_char,
    mut feat: libc::c_int,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ss: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut sslen: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut flags: libc::c_int = client_flags as libc::c_int;
    let mut pid: pid_t = 0;
    proc_send(
        client_peer,
        MSG_IDENTIFY_FLAGS,
        -(1 as libc::c_int),
        &mut flags as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    proc_send(
        client_peer,
        MSG_IDENTIFY_LONGFLAGS,
        -(1 as libc::c_int),
        &mut client_flags as *mut uint64_t as *const libc::c_void,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    );
    s = getenv(b"TERM\x00" as *const u8 as *const libc::c_char);
    if s.is_null() {
        s = b"\x00" as *const u8 as *const libc::c_char
    }
    proc_send(
        client_peer,
        MSG_IDENTIFY_TERM,
        -(1 as libc::c_int),
        s as *const libc::c_void,
        strlen(s).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    proc_send(
        client_peer,
        MSG_IDENTIFY_FEATURES,
        -(1 as libc::c_int),
        &mut feat as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    proc_send(
        client_peer,
        MSG_IDENTIFY_TTYNAME,
        -(1 as libc::c_int),
        ttynam as *const libc::c_void,
        strlen(ttynam).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    proc_send(
        client_peer,
        MSG_IDENTIFY_CWD,
        -(1 as libc::c_int),
        cwd as *const libc::c_void,
        strlen(cwd).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    fd = dup(0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        fatal(b"dup failed\x00" as *const u8 as *const libc::c_char);
    }
    proc_send(
        client_peer,
        MSG_IDENTIFY_STDIN,
        fd,
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
    fd = dup(1 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        fatal(b"dup failed\x00" as *const u8 as *const libc::c_char);
    }
    proc_send(
        client_peer,
        MSG_IDENTIFY_STDOUT,
        fd,
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
    pid = getpid();
    proc_send(
        client_peer,
        MSG_IDENTIFY_CLIENTPID,
        -(1 as libc::c_int),
        &mut pid as *mut pid_t as *const libc::c_void,
        ::std::mem::size_of::<pid_t>() as libc::c_ulong,
    );
    ss = environ;
    while !(*ss).is_null() {
        sslen = strlen(*ss).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if !(sslen
            > (16384 as libc::c_int as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong))
        {
            proc_send(
                client_peer,
                MSG_IDENTIFY_ENVIRON,
                -(1 as libc::c_int),
                *ss as *const libc::c_void,
                sslen,
            );
        }
        ss = ss.offset(1)
    }
    proc_send(
        client_peer,
        MSG_IDENTIFY_DONE,
        -(1 as libc::c_int),
        0 as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
}
/* File write error callback. */
unsafe extern "C" fn client_write_error_callback(
    mut _bev: *mut bufferevent,
    mut _what: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut cf: *mut client_file = arg as *mut client_file;
    log_debug(
        b"write error file %d\x00" as *const u8 as *const libc::c_char,
        (*cf).stream,
    );
    bufferevent_free((*cf).event);
    (*cf).event = 0 as *mut bufferevent;
    close((*cf).fd);
    (*cf).fd = -(1 as libc::c_int);
    if client_exitflag != 0 {
        client_exit();
    };
}
/* File write callback. */
unsafe extern "C" fn client_write_callback(mut _bev: *mut bufferevent, mut arg: *mut libc::c_void) {
    let mut cf: *mut client_file = arg as *mut client_file;
    if (*cf).closed != 0
        && evbuffer_get_length((*(*cf).event).output) == 0 as libc::c_int as libc::c_ulong
    {
        bufferevent_free((*cf).event);
        close((*cf).fd);
        client_files_RB_REMOVE(&mut client_files, cf);
        file_free(cf);
    }
    if client_exitflag != 0 {
        client_exit();
    };
}
/* Open write file. */
unsafe extern "C" fn client_write_open(mut data: *mut libc::c_void, mut datalen: size_t) {
    let mut msg: *mut msg_write_open = data as *mut msg_write_open;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut reply: msg_write_ready = msg_write_ready {
        stream: 0,
        error: 0,
    };
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
        entry: C2RustUnnamed_21 {
            rbe_left: 0 as *mut client_file,
            rbe_right: 0 as *mut client_file,
            rbe_parent: 0 as *mut client_file,
            rbe_color: 0,
        },
    };
    let mut cf: *mut client_file = 0 as *mut client_file;
    let flags: libc::c_int = 0o4000 as libc::c_int | 0o1 as libc::c_int | 0o100 as libc::c_int;
    let mut error: libc::c_int = 0 as libc::c_int;
    if datalen < ::std::mem::size_of::<msg_write_open>() as libc::c_ulong {
        fatalx(b"bad MSG_WRITE_OPEN size\x00" as *const u8 as *const libc::c_char);
    }
    if datalen == ::std::mem::size_of::<msg_write_open>() as libc::c_ulong {
        path = b"-\x00" as *const u8 as *const libc::c_char
    } else {
        path = msg.offset(1 as libc::c_int as isize) as *const libc::c_char
    }
    log_debug(
        b"open write file %d %s\x00" as *const u8 as *const libc::c_char,
        (*msg).stream,
        path,
    );
    find.stream = (*msg).stream;
    cf = client_files_RB_FIND(&mut client_files, &mut find);
    if cf.is_null() {
        cf = file_create(
            0 as *mut client,
            (*msg).stream,
            None,
            0 as *mut libc::c_void,
        );
        client_files_RB_INSERT(&mut client_files, cf);
        if (*cf).closed != 0 {
            error = 9 as libc::c_int
        } else {
            (*cf).fd = -(1 as libc::c_int);
            if (*msg).fd == -(1 as libc::c_int) {
                (*cf).fd = open(path, (*msg).flags | flags, 0o644 as libc::c_int)
            } else if (*msg).fd != 1 as libc::c_int && (*msg).fd != 2 as libc::c_int {
                *__errno_location() = 9 as libc::c_int
            } else {
                (*cf).fd = dup((*msg).fd);
                if !client_flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
                    close((*msg).fd);
                }
                /* can only be used once */
            }
            if (*cf).fd == -(1 as libc::c_int) {
                error = *__errno_location()
            } else {
                (*cf).event = bufferevent_new(
                    (*cf).fd,
                    None,
                    Some(
                        client_write_callback
                            as unsafe extern "C" fn(
                                _: *mut bufferevent,
                                _: *mut libc::c_void,
                            ) -> (),
                    ),
                    Some(
                        client_write_error_callback
                            as unsafe extern "C" fn(
                                _: *mut bufferevent,
                                _: libc::c_short,
                                _: *mut libc::c_void,
                            ) -> (),
                    ),
                    cf as *mut libc::c_void,
                );
                bufferevent_enable((*cf).event, 0x4 as libc::c_int as libc::c_short);
            }
        }
    } else {
        error = 9 as libc::c_int
    }
    reply.stream = (*msg).stream;
    reply.error = error;
    proc_send(
        client_peer,
        MSG_WRITE_READY,
        -(1 as libc::c_int),
        &mut reply as *mut msg_write_ready as *const libc::c_void,
        ::std::mem::size_of::<msg_write_ready>() as libc::c_ulong,
    );
}
/* Write to client file. */
unsafe extern "C" fn client_write_data(mut data: *mut libc::c_void, mut datalen: size_t) {
    let mut msg: *mut msg_write_data = data as *mut msg_write_data;
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
        entry: C2RustUnnamed_21 {
            rbe_left: 0 as *mut client_file,
            rbe_right: 0 as *mut client_file,
            rbe_parent: 0 as *mut client_file,
            rbe_color: 0,
        },
    };
    let mut cf: *mut client_file = 0 as *mut client_file;
    let mut size: size_t =
        datalen.wrapping_sub(::std::mem::size_of::<msg_write_data>() as libc::c_ulong);
    if datalen < ::std::mem::size_of::<msg_write_data>() as libc::c_ulong {
        fatalx(b"bad MSG_WRITE size\x00" as *const u8 as *const libc::c_char);
    }
    find.stream = (*msg).stream;
    cf = client_files_RB_FIND(&mut client_files, &mut find);
    if cf.is_null() {
        fatalx(b"unknown stream number\x00" as *const u8 as *const libc::c_char);
    }
    log_debug(
        b"write %zu to file %d\x00" as *const u8 as *const libc::c_char,
        size,
        (*cf).stream,
    );
    if !(*cf).event.is_null() {
        bufferevent_write(
            (*cf).event,
            msg.offset(1 as libc::c_int as isize) as *const libc::c_void,
            size,
        );
    };
}
/* Close client file. */
unsafe extern "C" fn client_write_close(mut data: *mut libc::c_void, mut datalen: size_t) {
    let mut msg: *mut msg_write_close = data as *mut msg_write_close;
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
        entry: C2RustUnnamed_21 {
            rbe_left: 0 as *mut client_file,
            rbe_right: 0 as *mut client_file,
            rbe_parent: 0 as *mut client_file,
            rbe_color: 0,
        },
    };
    let mut cf: *mut client_file = 0 as *mut client_file;
    if datalen != ::std::mem::size_of::<msg_write_close>() as libc::c_ulong {
        fatalx(b"bad MSG_WRITE_CLOSE size\x00" as *const u8 as *const libc::c_char);
    }
    find.stream = (*msg).stream;
    cf = client_files_RB_FIND(&mut client_files, &mut find);
    if cf.is_null() {
        fatalx(b"unknown stream number\x00" as *const u8 as *const libc::c_char);
    }
    log_debug(
        b"close file %d\x00" as *const u8 as *const libc::c_char,
        (*cf).stream,
    );
    if (*cf).event.is_null()
        || evbuffer_get_length((*(*cf).event).output) == 0 as libc::c_int as libc::c_ulong
    {
        if !(*cf).event.is_null() {
            bufferevent_free((*cf).event);
        }
        if (*cf).fd != -(1 as libc::c_int) {
            close((*cf).fd);
        }
        client_files_RB_REMOVE(&mut client_files, cf);
        file_free(cf);
    };
}
/* File read callback. */
unsafe extern "C" fn client_read_callback(mut _bev: *mut bufferevent, mut arg: *mut libc::c_void) {
    let mut cf: *mut client_file = arg as *mut client_file;
    let mut bdata: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bsize: size_t = 0;
    let mut msg: *mut msg_read_data = 0 as *mut msg_read_data;
    let mut msglen: size_t = 0;
    msg = xmalloc(::std::mem::size_of::<msg_read_data>() as libc::c_ulong) as *mut msg_read_data;
    loop {
        bdata = evbuffer_pullup((*(*cf).event).input, -(1 as libc::c_int) as ssize_t)
            as *mut libc::c_void;
        bsize = evbuffer_get_length((*(*cf).event).input);
        if bsize == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        if bsize
            > (16384 as libc::c_int as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<msg_read_data>() as libc::c_ulong)
        {
            bsize = (16384 as libc::c_int as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<msg_read_data>() as libc::c_ulong)
        }
        log_debug(
            b"read %zu from file %d\x00" as *const u8 as *const libc::c_char,
            bsize,
            (*cf).stream,
        );
        msglen = (::std::mem::size_of::<msg_read_data>() as libc::c_ulong).wrapping_add(bsize);
        msg = xrealloc(msg as *mut libc::c_void, msglen) as *mut msg_read_data;
        (*msg).stream = (*cf).stream;
        memcpy(
            msg.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            bdata,
            bsize,
        );
        proc_send(
            client_peer,
            MSG_READ,
            -(1 as libc::c_int),
            msg as *const libc::c_void,
            msglen,
        );
        evbuffer_drain((*(*cf).event).input, bsize);
    }
    free(msg as *mut libc::c_void);
}
/* File read error callback. */
unsafe extern "C" fn client_read_error_callback(
    mut _bev: *mut bufferevent,
    mut _what: libc::c_short,
    mut arg: *mut libc::c_void,
) {
    let mut cf: *mut client_file = arg as *mut client_file;
    let mut msg: msg_read_done = msg_read_done {
        stream: 0,
        error: 0,
    };
    log_debug(
        b"read error file %d\x00" as *const u8 as *const libc::c_char,
        (*cf).stream,
    );
    msg.stream = (*cf).stream;
    msg.error = 0 as libc::c_int;
    proc_send(
        client_peer,
        MSG_READ_DONE,
        -(1 as libc::c_int),
        &mut msg as *mut msg_read_done as *const libc::c_void,
        ::std::mem::size_of::<msg_read_done>() as libc::c_ulong,
    );
    bufferevent_free((*cf).event);
    close((*cf).fd);
    client_files_RB_REMOVE(&mut client_files, cf);
    file_free(cf);
}
/* Open read file. */
unsafe extern "C" fn client_read_open(mut data: *mut libc::c_void, mut datalen: size_t) {
    let mut msg: *mut msg_read_open = data as *mut msg_read_open;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut reply: msg_read_done = msg_read_done {
        stream: 0,
        error: 0,
    };
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
        entry: C2RustUnnamed_21 {
            rbe_left: 0 as *mut client_file,
            rbe_right: 0 as *mut client_file,
            rbe_parent: 0 as *mut client_file,
            rbe_color: 0,
        },
    };
    let mut cf: *mut client_file = 0 as *mut client_file;
    let flags: libc::c_int = 0o4000 as libc::c_int | 0 as libc::c_int;
    let mut error: libc::c_int = 0;
    if datalen < ::std::mem::size_of::<msg_read_open>() as libc::c_ulong {
        fatalx(b"bad MSG_READ_OPEN size\x00" as *const u8 as *const libc::c_char);
    }
    if datalen == ::std::mem::size_of::<msg_read_open>() as libc::c_ulong {
        path = b"-\x00" as *const u8 as *const libc::c_char
    } else {
        path = msg.offset(1 as libc::c_int as isize) as *const libc::c_char
    }
    log_debug(
        b"open read file %d %s\x00" as *const u8 as *const libc::c_char,
        (*msg).stream,
        path,
    );
    find.stream = (*msg).stream;
    cf = client_files_RB_FIND(&mut client_files, &mut find);
    if cf.is_null() {
        cf = file_create(
            0 as *mut client,
            (*msg).stream,
            None,
            0 as *mut libc::c_void,
        );
        client_files_RB_INSERT(&mut client_files, cf);
        if (*cf).closed != 0 {
            error = 9 as libc::c_int
        } else {
            (*cf).fd = -(1 as libc::c_int);
            if (*msg).fd == -(1 as libc::c_int) {
                (*cf).fd = open(path, flags)
            } else if (*msg).fd != 0 as libc::c_int {
                *__errno_location() = 9 as libc::c_int
            } else {
                (*cf).fd = dup((*msg).fd);
                if !client_flags & 0x2000 as libc::c_int as libc::c_ulong != 0 {
                    close((*msg).fd);
                }
                /* can only be used once */
            }
            if (*cf).fd == -(1 as libc::c_int) {
                error = *__errno_location()
            } else {
                (*cf).event = bufferevent_new(
                    (*cf).fd,
                    Some(
                        client_read_callback
                            as unsafe extern "C" fn(
                                _: *mut bufferevent,
                                _: *mut libc::c_void,
                            ) -> (),
                    ),
                    None,
                    Some(
                        client_read_error_callback
                            as unsafe extern "C" fn(
                                _: *mut bufferevent,
                                _: libc::c_short,
                                _: *mut libc::c_void,
                            ) -> (),
                    ),
                    cf as *mut libc::c_void,
                );
                bufferevent_enable((*cf).event, 0x2 as libc::c_int as libc::c_short);
                return;
            }
        }
    } else {
        error = 9 as libc::c_int
    }
    reply.stream = (*msg).stream;
    reply.error = error;
    proc_send(
        client_peer,
        MSG_READ_DONE,
        -(1 as libc::c_int),
        &mut reply as *mut msg_read_done as *const libc::c_void,
        ::std::mem::size_of::<msg_read_done>() as libc::c_ulong,
    );
}
/* Run command in shell; used for -c. */
unsafe extern "C" fn client_exec(
    mut shell: *const libc::c_char,
    mut shellcmd: *const libc::c_char,
) -> ! {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut argv0: *mut libc::c_char = 0 as *mut libc::c_char;
    log_debug(
        b"shell %s, command %s\x00" as *const u8 as *const libc::c_char,
        shell,
        shellcmd,
    );
    ptr = strrchr(shell, '/' as i32);
    if !ptr.is_null() && *ptr.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
        name = ptr.offset(1 as libc::c_int as isize)
    } else {
        name = shell
    }
    if client_flags & 0x2 as libc::c_int as libc::c_ulong != 0 {
        xasprintf(
            &mut argv0 as *mut *mut libc::c_char,
            b"-%s\x00" as *const u8 as *const libc::c_char,
            name,
        );
    } else {
        xasprintf(
            &mut argv0 as *mut *mut libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    setenv(
        b"SHELL\x00" as *const u8 as *const libc::c_char,
        shell,
        1 as libc::c_int,
    );
    proc_clear_signals(client_proc, 1 as libc::c_int);
    setblocking(0 as libc::c_int, 1 as libc::c_int);
    setblocking(1 as libc::c_int, 1 as libc::c_int);
    setblocking(2 as libc::c_int, 1 as libc::c_int);
    closefrom(2 as libc::c_int + 1 as libc::c_int);
    execl(
        shell,
        argv0,
        b"-c\x00" as *const u8 as *const libc::c_char,
        shellcmd,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    fatal(b"execl failed\x00" as *const u8 as *const libc::c_char);
}
/* Callback to handle signals in the client. */
unsafe extern "C" fn client_signal(mut sig: libc::c_int) {
    let mut sigact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut status: libc::c_int = 0;
    log_debug(
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"client_signal\x00")).as_ptr(),
        strsignal(sig),
    );
    if sig == 17 as libc::c_int {
        waitpid(-(1 as libc::c_int), &mut status, 1 as libc::c_int);
    } else if client_attached == 0 {
        if sig == 15 as libc::c_int {
            proc_exit(client_proc);
        }
    } else {
        match sig {
            1 => {
                client_exitreason = CLIENT_EXIT_LOST_TTY;
                client_exitval = 1 as libc::c_int;
                proc_send(
                    client_peer,
                    MSG_EXITING,
                    -(1 as libc::c_int),
                    0 as *const libc::c_void,
                    0 as libc::c_int as size_t,
                );
            }
            15 => {
                if client_suspended == 0 {
                    client_exitreason = CLIENT_EXIT_TERMINATED
                }
                client_exitval = 1 as libc::c_int;
                proc_send(
                    client_peer,
                    MSG_EXITING,
                    -(1 as libc::c_int),
                    0 as *const libc::c_void,
                    0 as libc::c_int as size_t,
                );
            }
            28 => {
                proc_send(
                    client_peer,
                    MSG_RESIZE,
                    -(1 as libc::c_int),
                    0 as *const libc::c_void,
                    0 as libc::c_int as size_t,
                );
            }
            18 => {
                memset(
                    &mut sigact as *mut sigaction as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<sigaction>() as libc::c_ulong,
                );
                sigemptyset(&mut sigact.sa_mask);
                sigact.sa_flags = 0x10000000 as libc::c_int;
                sigact.__sigaction_handler.sa_handler =
                    ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(
                        1 as libc::c_int as libc::intptr_t,
                    );
                if sigaction(20 as libc::c_int, &mut sigact, 0 as *mut sigaction)
                    != 0 as libc::c_int
                {
                    fatal(b"sigaction failed\x00" as *const u8 as *const libc::c_char);
                }
                proc_send(
                    client_peer,
                    MSG_WAKEUP,
                    -(1 as libc::c_int),
                    0 as *const libc::c_void,
                    0 as libc::c_int as size_t,
                );
                client_suspended = 0 as libc::c_int
            }
            _ => {}
        }
    };
}
/* Callback for client read events. */
unsafe extern "C" fn client_dispatch(mut imsg: *mut imsg, mut _arg: *mut libc::c_void) {
    if imsg.is_null() {
        client_exitreason = CLIENT_EXIT_LOST_SERVER;
        client_exitval = 1 as libc::c_int;
        proc_exit(client_proc);
        return;
    }
    if client_attached != 0 {
        client_dispatch_attached(imsg);
    } else {
        client_dispatch_wait(imsg);
    };
}
/* Process an exit message. */
unsafe extern "C" fn client_dispatch_exit_message(
    mut data: *mut libc::c_char,
    mut datalen: size_t,
) {
    let mut retval: libc::c_int = 0;
    if datalen < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        && datalen != 0 as libc::c_int as libc::c_ulong
    {
        fatalx(b"bad MSG_EXIT size\x00" as *const u8 as *const libc::c_char);
    }
    if datalen >= ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
        memcpy(
            &mut retval as *mut libc::c_int as *mut libc::c_void,
            data as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        );
        client_exitval = retval
    }
    if datalen > ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
        datalen = (datalen as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as size_t as size_t;
        data = data.offset(::std::mem::size_of::<libc::c_int>() as libc::c_ulong as isize);
        client_exitmessage = xmalloc(datalen) as *mut libc::c_char;
        memcpy(
            client_exitmessage as *mut libc::c_void,
            data as *const libc::c_void,
            datalen,
        );
        *client_exitmessage
            .offset(datalen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
            '\u{0}' as i32 as libc::c_char;
        client_exitreason = CLIENT_EXIT_MESSAGE_PROVIDED
    };
}
/* Dispatch imsgs when in wait state (before MSG_READY). */
unsafe extern "C" fn client_dispatch_wait(mut imsg: *mut imsg) {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut datalen: ssize_t = 0;
    static mut pledge_applied: libc::c_int = 0;
    /*
     * "sendfd" is no longer required once all of the identify messages
     * have been sent. We know the server won't send us anything until that
     * point (because we don't ask it to), so we can drop "sendfd" once we
     * get the first message from the server.
     */
    if pledge_applied == 0 {
        if 0 as libc::c_int != 0 as libc::c_int {
            fatal(b"pledge failed\x00" as *const u8 as *const libc::c_char);
        }
        pledge_applied = 1 as libc::c_int
    }
    data = (*imsg).data as *mut libc::c_char;
    datalen = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong) as ssize_t;
    match (*imsg).hdr.type_0 {
        203 | 210 => {
            client_dispatch_exit_message(data, datalen as size_t);
            client_exitflag = 1 as libc::c_int;
            client_exit();
        }
        207 => {
            if datalen != 0 as libc::c_int as libc::c_long {
                fatalx(b"bad MSG_READY size\x00" as *const u8 as *const libc::c_char);
            }
            client_attached = 1 as libc::c_int;
            proc_send(
                client_peer,
                MSG_RESIZE,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        12 => {
            if datalen != 0 as libc::c_int as libc::c_long {
                fatalx(b"bad MSG_VERSION size\x00" as *const u8 as *const libc::c_char);
            }
            fprintf(
                stderr,
                b"protocol version mismatch (client %d, server %u)\n\x00" as *const u8
                    as *const libc::c_char,
                8 as libc::c_int,
                (*imsg).hdr.peerid & 0xff as libc::c_int as libc::c_uint,
            );
            client_exitval = 1 as libc::c_int;
            proc_exit(client_proc);
        }
        218 => {
            if datalen as libc::c_ulong != ::std::mem::size_of::<uint64_t>() as libc::c_ulong {
                fatalx(b"bad MSG_FLAGS string\x00" as *const u8 as *const libc::c_char);
            }
            memcpy(
                &mut client_flags as *mut uint64_t as *mut libc::c_void,
                data as *const libc::c_void,
                ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
            );
            log_debug(
                b"new flags are %#llx\x00" as *const u8 as *const libc::c_char,
                client_flags as libc::c_ulonglong,
            );
        }
        209 => {
            if datalen == 0 as libc::c_int as libc::c_long
                || *data.offset((datalen - 1 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int
                    != '\u{0}' as i32
            {
                fatalx(b"bad MSG_SHELL string\x00" as *const u8 as *const libc::c_char);
            }
            client_exec(data, shell_command);
        }
        201 | 202 => {
            /* NOTREACHED */
            proc_send(
                client_peer,
                MSG_EXITING,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        204 => {
            proc_exit(client_proc);
        }
        300 => {
            client_read_open(data as *mut libc::c_void, datalen as size_t);
        }
        303 => {
            client_write_open(data as *mut libc::c_void, datalen as size_t);
        }
        304 => {
            client_write_data(data as *mut libc::c_void, datalen as size_t);
        }
        306 => {
            client_write_close(data as *mut libc::c_void, datalen as size_t);
        }
        211 | 212 | 213 => {
            fprintf(
                stderr,
                b"server version is too old for client\n\x00" as *const u8 as *const libc::c_char,
            );
            proc_exit(client_proc);
        }
        _ => {}
    };
}
/* Dispatch imsgs in attached state (after MSG_READY). */
unsafe extern "C" fn client_dispatch_attached(mut imsg: *mut imsg) {
    let mut sigact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut datalen: ssize_t = 0;
    data = (*imsg).data as *mut libc::c_char;
    datalen = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<imsg_hdr>() as libc::c_ulong) as ssize_t;
    match (*imsg).hdr.type_0 {
        218 => {
            if datalen as libc::c_ulong != ::std::mem::size_of::<uint64_t>() as libc::c_ulong {
                fatalx(b"bad MSG_FLAGS string\x00" as *const u8 as *const libc::c_char);
            }
            memcpy(
                &mut client_flags as *mut uint64_t as *mut libc::c_void,
                data as *const libc::c_void,
                ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
            );
            log_debug(
                b"new flags are %#llx\x00" as *const u8 as *const libc::c_char,
                client_flags as libc::c_ulonglong,
            );
        }
        201 | 202 => {
            if datalen == 0 as libc::c_int as libc::c_long
                || *data.offset((datalen - 1 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int
                    != '\u{0}' as i32
            {
                fatalx(b"bad MSG_DETACH string\x00" as *const u8 as *const libc::c_char);
            }
            client_exitsession = xstrdup(data);
            client_exittype = (*imsg).hdr.type_0 as msgtype;
            if (*imsg).hdr.type_0 == MSG_DETACHKILL as libc::c_int as libc::c_uint {
                client_exitreason = CLIENT_EXIT_DETACHED_HUP
            } else {
                client_exitreason = CLIENT_EXIT_DETACHED
            }
            proc_send(
                client_peer,
                MSG_EXITING,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        217 => {
            if datalen == 0 as libc::c_int as libc::c_long
                || *data.offset((datalen - 1 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int
                    != '\u{0}' as i32
                || strlen(data).wrapping_add(1 as libc::c_int as libc::c_ulong) == datalen as size_t
            {
                fatalx(b"bad MSG_EXEC string\x00" as *const u8 as *const libc::c_char);
            }
            client_execcmd = xstrdup(data);
            client_execshell = xstrdup(
                data.offset(strlen(data) as isize)
                    .offset(1 as libc::c_int as isize),
            );
            client_exittype = (*imsg).hdr.type_0 as msgtype;
            proc_send(
                client_peer,
                MSG_EXITING,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        203 => {
            client_dispatch_exit_message(data, datalen as size_t);
            if client_exitreason as libc::c_uint == CLIENT_EXIT_NONE as libc::c_int as libc::c_uint
            {
                client_exitreason = CLIENT_EXIT_EXITED
            }
            proc_send(
                client_peer,
                MSG_EXITING,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        204 => {
            if datalen != 0 as libc::c_int as libc::c_long {
                fatalx(b"bad MSG_EXITED size\x00" as *const u8 as *const libc::c_char);
            }
            proc_exit(client_proc);
        }
        210 => {
            if datalen != 0 as libc::c_int as libc::c_long {
                fatalx(b"bad MSG_SHUTDOWN size\x00" as *const u8 as *const libc::c_char);
            }
            proc_send(
                client_peer,
                MSG_EXITING,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
            client_exitreason = CLIENT_EXIT_SERVER_EXITED;
            client_exitval = 1 as libc::c_int
        }
        214 => {
            if datalen != 0 as libc::c_int as libc::c_long {
                fatalx(b"bad MSG_SUSPEND size\x00" as *const u8 as *const libc::c_char);
            }
            memset(
                &mut sigact as *mut sigaction as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sigaction>() as libc::c_ulong,
            );
            sigemptyset(&mut sigact.sa_mask);
            sigact.sa_flags = 0x10000000 as libc::c_int;
            sigact.__sigaction_handler.sa_handler = None;
            if sigaction(20 as libc::c_int, &mut sigact, 0 as *mut sigaction) != 0 as libc::c_int {
                fatal(b"sigaction failed\x00" as *const u8 as *const libc::c_char);
            }
            client_suspended = 1 as libc::c_int;
            kill(getpid(), 20 as libc::c_int);
        }
        206 => {
            if datalen == 0 as libc::c_int as libc::c_long
                || *data.offset((datalen - 1 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int
                    != '\u{0}' as i32
            {
                fatalx(b"bad MSG_LOCK string\x00" as *const u8 as *const libc::c_char);
            }
            system(data);
            proc_send(
                client_peer,
                MSG_UNLOCK,
                -(1 as libc::c_int),
                0 as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
        _ => {}
    };
}
