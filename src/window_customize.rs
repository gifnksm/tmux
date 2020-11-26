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
    pub type cmds;
    pub type tty_code;
    pub type format_job_tree;
    pub type control_state;
    pub type cmdq_list;
    pub type tmuxpeer;
    pub type cmdq_item;
    pub type mode_tree_data;
    pub type options_array_item;
    pub type options_entry;
    pub type screen_write_collect_item;
    pub type mode_tree_item;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xasprintf(_: *mut *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut global_options: *mut options;
    #[no_mangle]
    static mut global_s_options: *mut options;
    #[no_mangle]
    static mut global_w_options: *mut options;
    #[no_mangle]
    fn format_true(_: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn format_free(_: *mut format_tree);
    #[no_mangle]
    fn format_add(_: *mut format_tree, _: *const libc::c_char, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn format_expand(_: *mut format_tree, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn format_create_from_state(
        _: *mut cmdq_item,
        _: *mut client,
        _: *mut cmd_find_state,
    ) -> *mut format_tree;
    #[no_mangle]
    fn options_get_parent(_: *mut options) -> *mut options;
    #[no_mangle]
    fn options_first(_: *mut options) -> *mut options_entry;
    #[no_mangle]
    fn options_next(_: *mut options_entry) -> *mut options_entry;
    #[no_mangle]
    fn options_default_to_string(_: *const options_table_entry) -> *mut libc::c_char;
    #[no_mangle]
    fn options_name(_: *mut options_entry) -> *const libc::c_char;
    #[no_mangle]
    fn options_owner(_: *mut options_entry) -> *mut options;
    #[no_mangle]
    fn options_table_entry(_: *mut options_entry) -> *const options_table_entry;
    #[no_mangle]
    fn options_get_only(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    #[no_mangle]
    fn options_get(_: *mut options, _: *const libc::c_char) -> *mut options_entry;
    #[no_mangle]
    fn options_array_get(_: *mut options_entry, _: u_int) -> *mut options_value;
    #[no_mangle]
    fn options_array_set(
        _: *mut options_entry,
        _: u_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn options_array_first(_: *mut options_entry) -> *mut options_array_item;
    #[no_mangle]
    fn options_array_next(_: *mut options_array_item) -> *mut options_array_item;
    #[no_mangle]
    fn options_array_item_index(_: *mut options_array_item) -> u_int;
    #[no_mangle]
    fn options_to_string(
        _: *mut options_entry,
        _: libc::c_int,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    #[no_mangle]
    fn options_set_number(
        _: *mut options,
        _: *const libc::c_char,
        _: libc::c_longlong,
    ) -> *mut options_entry;
    #[no_mangle]
    fn options_from_string(
        _: *mut options,
        _: *const options_table_entry,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn options_push_changes(_: *const libc::c_char);
    #[no_mangle]
    fn options_remove_or_default(
        _: *mut options_entry,
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    static options_table: [options_table_entry; 0];
    #[no_mangle]
    fn args_has(_: *mut args, _: u_char) -> libc::c_int;
    #[no_mangle]
    fn args_get(_: *mut args, _: u_char) -> *const libc::c_char;
    #[no_mangle]
    fn cmd_find_valid_state(_: *mut cmd_find_state) -> libc::c_int;
    #[no_mangle]
    fn cmd_find_copy_state(_: *mut cmd_find_state, _: *mut cmd_find_state);
    #[no_mangle]
    fn cmd_find_from_pane(
        _: *mut cmd_find_state,
        _: *mut window_pane,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn cmd_list_free(_: *mut cmd_list);
    #[no_mangle]
    fn cmd_list_print(_: *mut cmd_list, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn cmd_parse_from_string(
        _: *const libc::c_char,
        _: *mut cmd_parse_input,
    ) -> *mut cmd_parse_result;
    #[no_mangle]
    fn key_bindings_get_table(_: *const libc::c_char, _: libc::c_int) -> *mut key_table;
    #[no_mangle]
    fn key_bindings_first_table() -> *mut key_table;
    #[no_mangle]
    fn key_bindings_next_table(_: *mut key_table) -> *mut key_table;
    #[no_mangle]
    fn key_bindings_get(_: *mut key_table, _: key_code) -> *mut key_binding;
    #[no_mangle]
    fn key_bindings_get_default(_: *mut key_table, _: key_code) -> *mut key_binding;
    #[no_mangle]
    fn key_bindings_first(_: *mut key_table) -> *mut key_binding;
    #[no_mangle]
    fn key_bindings_next(_: *mut key_table, _: *mut key_binding) -> *mut key_binding;
    #[no_mangle]
    fn key_bindings_remove(_: *const libc::c_char, _: key_code);
    #[no_mangle]
    fn key_bindings_reset(_: *const libc::c_char, _: key_code);
    #[no_mangle]
    fn key_string_lookup_key(_: key_code, _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn status_message_set(
        _: *mut client,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn status_prompt_set(
        _: *mut client,
        _: *mut cmd_find_state,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: prompt_input_cb,
        _: prompt_free_cb,
        _: *mut libc::c_void,
        _: libc::c_int,
    );
    #[no_mangle]
    static grid_default_cell: grid_cell;
    #[no_mangle]
    fn screen_write_text(
        _: *mut screen_write_ctx,
        _: u_int,
        _: u_int,
        _: u_int,
        _: libc::c_int,
        _: *const grid_cell,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn screen_write_cursormove(
        _: *mut screen_write_ctx,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
    );
    #[no_mangle]
    fn window_pane_index(_: *mut window_pane, _: *mut u_int) -> libc::c_int;
    #[no_mangle]
    fn window_pane_reset_mode(_: *mut window_pane);
    #[no_mangle]
    fn mode_tree_each_tagged(
        _: *mut mode_tree_data,
        _: mode_tree_each_cb,
        _: *mut client,
        _: key_code,
        _: libc::c_int,
    );
    #[no_mangle]
    fn mode_tree_up(_: *mut mode_tree_data, _: libc::c_int);
    #[no_mangle]
    fn mode_tree_start(
        _: *mut window_pane,
        _: *mut args,
        _: mode_tree_build_cb,
        _: mode_tree_draw_cb,
        _: mode_tree_search_cb,
        _: mode_tree_menu_cb,
        _: mode_tree_height_cb,
        _: *mut libc::c_void,
        _: *const menu_item,
        _: *mut *const libc::c_char,
        _: u_int,
        _: *mut *mut screen,
    ) -> *mut mode_tree_data;
    #[no_mangle]
    fn mode_tree_zoom(_: *mut mode_tree_data, _: *mut args);
    #[no_mangle]
    fn mode_tree_build(_: *mut mode_tree_data);
    #[no_mangle]
    fn mode_tree_free(_: *mut mode_tree_data);
    #[no_mangle]
    fn mode_tree_resize(_: *mut mode_tree_data, _: u_int, _: u_int);
    #[no_mangle]
    fn mode_tree_add(
        _: *mut mode_tree_data,
        _: *mut mode_tree_item,
        _: *mut libc::c_void,
        _: uint64_t,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut mode_tree_item;
    #[no_mangle]
    fn mode_tree_draw_as_parent(_: *mut mode_tree_item);
    #[no_mangle]
    fn mode_tree_no_tag(_: *mut mode_tree_item);
    #[no_mangle]
    fn mode_tree_draw(_: *mut mode_tree_data);
    #[no_mangle]
    fn mode_tree_key(
        _: *mut mode_tree_data,
        _: *mut client,
        _: *mut key_code,
        _: *mut mouse_event,
        _: *mut u_int,
        _: *mut u_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn mode_tree_collapse_current(_: *mut mode_tree_data);
    #[no_mangle]
    fn mode_tree_get_current_name(_: *mut mode_tree_data) -> *const libc::c_char;
    #[no_mangle]
    fn mode_tree_get_current(_: *mut mode_tree_data) -> *mut libc::c_void;
    #[no_mangle]
    fn mode_tree_count_tagged(_: *mut mode_tree_data) -> u_int;
    #[no_mangle]
    fn style_apply(_: *mut grid_cell, _: *mut options, _: *const libc::c_char, _: *mut format_tree);
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
pub type cmd_parse_status = libc::c_uint;
pub const CMD_PARSE_SUCCESS: cmd_parse_status = 2;
pub const CMD_PARSE_ERROR: cmd_parse_status = 1;
pub const CMD_PARSE_EMPTY: cmd_parse_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_parse_result {
    pub status: cmd_parse_status,
    pub cmdlist: *mut cmd_list,
    pub error: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_parse_input {
    pub flags: libc::c_int,
    pub file: *const libc::c_char,
    pub line: u_int,
    pub item: *mut cmdq_item,
    pub c: *mut client,
    pub fs: cmd_find_state,
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
pub struct mode_tree_sort_criteria {
    pub field: u_int,
    pub reversed: libc::c_int,
}
pub type mode_tree_build_cb = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut mode_tree_sort_criteria,
        _: *mut uint64_t,
        _: *const libc::c_char,
    ) -> (),
>;
pub type mode_tree_draw_cb = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut screen_write_ctx,
        _: u_int,
        _: u_int,
    ) -> (),
>;
pub type mode_tree_search_cb = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *const libc::c_char,
    ) -> libc::c_int,
>;
pub type mode_tree_menu_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut client, _: key_code) -> ()>;
pub type mode_tree_height_cb =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: u_int) -> u_int>;
pub type mode_tree_each_cb = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut libc::c_void,
        _: *mut client,
        _: key_code,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_customize_modedata {
    pub wp: *mut window_pane,
    pub dead: libc::c_int,
    pub references: libc::c_int,
    pub data: *mut mode_tree_data,
    pub format: *mut libc::c_char,
    pub hide_global: libc::c_int,
    pub item_list: *mut *mut window_customize_itemdata,
    pub item_size: u_int,
    pub fs: cmd_find_state,
    pub change: window_customize_change,
}
pub type window_customize_change = libc::c_uint;
pub const WINDOW_CUSTOMIZE_RESET: window_customize_change = 1;
pub const WINDOW_CUSTOMIZE_UNSET: window_customize_change = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window_customize_itemdata {
    pub data: *mut window_customize_modedata,
    pub scope: window_customize_scope,
    pub table: *mut libc::c_char,
    pub key: key_code,
    pub oo: *mut options,
    pub name: *mut libc::c_char,
    pub idx: libc::c_int,
}
pub type window_customize_scope = libc::c_uint;
pub const WINDOW_CUSTOMIZE_PANE: window_customize_scope = 7;
pub const WINDOW_CUSTOMIZE_WINDOW: window_customize_scope = 6;
pub const WINDOW_CUSTOMIZE_GLOBAL_WINDOW: window_customize_scope = 5;
pub const WINDOW_CUSTOMIZE_SESSION: window_customize_scope = 4;
pub const WINDOW_CUSTOMIZE_GLOBAL_SESSION: window_customize_scope = 3;
pub const WINDOW_CUSTOMIZE_SERVER: window_customize_scope = 2;
pub const WINDOW_CUSTOMIZE_KEY: window_customize_scope = 1;
pub const WINDOW_CUSTOMIZE_NONE: window_customize_scope = 0;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut window_customize_menu_items: [menu_item; 9] = [
    {
        let mut init = menu_item {
            name: b"Select\x00" as *const u8 as *const libc::c_char,
            key: '\r' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: b"Expand\x00" as *const u8 as *const libc::c_char,
            key: KEYC_RIGHT as libc::c_ulong as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: b"\x00" as *const u8 as *const libc::c_char,
            key: 0xff000000000 as libc::c_ulonglong,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: b"Tag\x00" as *const u8 as *const libc::c_char,
            key: 't' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: b"Tag All\x00" as *const u8 as *const libc::c_char,
            key: '\u{14}' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: b"Tag None\x00" as *const u8 as *const libc::c_char,
            key: 'T' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: b"\x00" as *const u8 as *const libc::c_char,
            key: 0xff000000000 as libc::c_ulonglong,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: b"Cancel\x00" as *const u8 as *const libc::c_char,
            key: 'q' as i32 as key_code,
            command: 0 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = menu_item {
            name: 0 as *const libc::c_char,
            key: 0xff000000000 as libc::c_ulonglong,
            command: 0 as *const libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub static mut window_customize_mode: window_mode = unsafe {
    {
        let mut init =
                window_mode{name:
                                b"options-mode\x00" as *const u8 as
                                    *const libc::c_char,
                            default_format:
                                b"#{?is_option,#{?option_is_global,,#[reverse](#{option_scope})#[default] }#[ignore]#{option_value}#{?option_unit, #{option_unit},},#{key}}\x00"
                                    as *const u8 as *const libc::c_char,
                            init:
                                Some(window_customize_init as
                                         unsafe extern "C" fn(_:
                                                                  *mut window_mode_entry,
                                                              _:
                                                                  *mut cmd_find_state,
                                                              _: *mut args)
                                             -> *mut screen),
                            free:
                                Some(window_customize_free as
                                         unsafe extern "C" fn(_:
                                                                  *mut window_mode_entry)
                                             -> ()),
                            resize:
                                Some(window_customize_resize as
                                         unsafe extern "C" fn(_:
                                                                  *mut window_mode_entry,
                                                              _: u_int,
                                                              _: u_int)
                                             -> ()),
                            key:
                                Some(window_customize_key as
                                         unsafe extern "C" fn(_:
                                                                  *mut window_mode_entry,
                                                              _: *mut client,
                                                              _: *mut session,
                                                              _: *mut winlink,
                                                              _: key_code,
                                                              _:
                                                                  *mut mouse_event)
                                             -> ()),
                            key_table: None,
                            command: None,
                            formats: None,};
        init
    }
};
unsafe extern "C" fn window_customize_get_tag(
    mut o: *mut options_entry,
    mut idx: libc::c_int,
    mut oe: *const options_table_entry,
) -> uint64_t {
    let mut offset: uint64_t = 0;
    if oe.is_null() {
        return o as uint64_t;
    }
    offset = ((oe as *mut libc::c_char)
        .wrapping_offset_from(options_table.as_ptr() as *mut libc::c_char)
        as libc::c_long as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<options_table_entry>() as libc::c_ulong);
    return ((2 as libc::c_ulonglong) << 62 as libc::c_int
        | (offset << 32 as libc::c_int) as libc::c_ulonglong
        | ((idx + 1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulonglong
        | 1 as libc::c_int as libc::c_ulonglong) as uint64_t;
}
unsafe extern "C" fn window_customize_get_tree(
    mut scope: window_customize_scope,
    mut fs: *mut cmd_find_state,
) -> *mut options {
    match scope as libc::c_uint {
        0 | 1 => return 0 as *mut options,
        2 => return global_options,
        3 => return global_s_options,
        4 => return (*(*fs).s).options,
        5 => return global_w_options,
        6 => return (*(*fs).w).options,
        7 => return (*(*fs).wp).options,
        _ => {}
    }
    return 0 as *mut options;
}
unsafe extern "C" fn window_customize_check_item(
    mut data: *mut window_customize_modedata,
    mut item: *mut window_customize_itemdata,
    mut fsp: *mut cmd_find_state,
) -> libc::c_int {
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    if fsp.is_null() {
        fsp = &mut fs
    }
    if cmd_find_valid_state(&mut (*data).fs) != 0 {
        cmd_find_copy_state(fsp, &mut (*data).fs);
    } else {
        cmd_find_from_pane(fsp, (*data).wp, 0 as libc::c_int);
    }
    return ((*item).oo == window_customize_get_tree((*item).scope, fsp)) as libc::c_int;
}
unsafe extern "C" fn window_customize_get_key(
    mut item: *mut window_customize_itemdata,
    mut ktp: *mut *mut key_table,
    mut bdp: *mut *mut key_binding,
) -> libc::c_int {
    let mut kt: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    kt = key_bindings_get_table((*item).table, 0 as libc::c_int);
    if kt.is_null() {
        return 0 as libc::c_int;
    }
    bd = key_bindings_get(kt, (*item).key);
    if bd.is_null() {
        return 0 as libc::c_int;
    }
    if !ktp.is_null() {
        *ktp = kt
    }
    if !bdp.is_null() {
        *bdp = bd
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn window_customize_scope_text(
    mut scope: window_customize_scope,
    mut fs: *mut cmd_find_state,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: u_int = 0;
    match scope as libc::c_uint {
        0 | 1 | 2 | 3 | 5 => s = xstrdup(b"\x00" as *const u8 as *const libc::c_char),
        7 => {
            window_pane_index((*fs).wp, &mut idx);
            xasprintf(
                &mut s as *mut *mut libc::c_char,
                b"pane %u\x00" as *const u8 as *const libc::c_char,
                idx,
            );
        }
        4 => {
            xasprintf(
                &mut s as *mut *mut libc::c_char,
                b"session %s\x00" as *const u8 as *const libc::c_char,
                (*(*fs).s).name,
            );
        }
        6 => {
            xasprintf(
                &mut s as *mut *mut libc::c_char,
                b"window %u\x00" as *const u8 as *const libc::c_char,
                (*(*fs).wl).idx,
            );
        }
        _ => {}
    }
    return s;
}
unsafe extern "C" fn window_customize_add_item(
    mut data: *mut window_customize_modedata,
) -> *mut window_customize_itemdata {
    let mut item: *mut window_customize_itemdata = 0 as *mut window_customize_itemdata;
    (*data).item_list = xreallocarray(
        (*data).item_list as *mut libc::c_void,
        (*data)
            .item_size
            .wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
        ::std::mem::size_of::<*mut window_customize_itemdata>() as libc::c_ulong,
    ) as *mut *mut window_customize_itemdata;
    let fresh0 = (*data).item_size;
    (*data).item_size = (*data).item_size.wrapping_add(1);
    let ref mut fresh1 = *(*data).item_list.offset(fresh0 as isize);
    *fresh1 = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<window_customize_itemdata>() as libc::c_ulong,
    ) as *mut window_customize_itemdata;
    item = *fresh1;
    return item;
}
unsafe extern "C" fn window_customize_free_item(mut item: *mut window_customize_itemdata) {
    free((*item).table as *mut libc::c_void);
    free((*item).name as *mut libc::c_void);
    free(item as *mut libc::c_void);
}
unsafe extern "C" fn window_customize_build_array(
    mut data: *mut window_customize_modedata,
    mut top: *mut mode_tree_item,
    mut scope: window_customize_scope,
    mut o: *mut options_entry,
    mut ft: *mut format_tree,
) {
    let mut oe: *const options_table_entry = options_table_entry(o);
    let mut oo: *mut options = options_owner(o);
    let mut item: *mut window_customize_itemdata = 0 as *mut window_customize_itemdata;
    let mut ai: *mut options_array_item = 0 as *mut options_array_item;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: u_int = 0;
    let mut tag: uint64_t = 0;
    ai = options_array_first(o);
    while !ai.is_null() {
        idx = options_array_item_index(ai);
        xasprintf(
            &mut name as *mut *mut libc::c_char,
            b"%s[%u]\x00" as *const u8 as *const libc::c_char,
            options_name(o),
            idx,
        );
        format_add(
            ft,
            b"option_name\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            name,
        );
        value = options_to_string(o, idx as libc::c_int, 0 as libc::c_int);
        format_add(
            ft,
            b"option_value\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            value,
        );
        item = window_customize_add_item(data);
        (*item).scope = scope;
        (*item).oo = oo;
        (*item).name = xstrdup(options_name(o));
        (*item).idx = idx as libc::c_int;
        text = format_expand(ft, (*data).format);
        tag = window_customize_get_tag(o, idx as libc::c_int, oe);
        mode_tree_add(
            (*data).data,
            top,
            item as *mut libc::c_void,
            tag,
            name,
            text,
            -(1 as libc::c_int),
        );
        free(text as *mut libc::c_void);
        free(name as *mut libc::c_void);
        free(value as *mut libc::c_void);
        ai = options_array_next(ai)
    }
}
unsafe extern "C" fn window_customize_build_option(
    mut data: *mut window_customize_modedata,
    mut top: *mut mode_tree_item,
    mut scope: window_customize_scope,
    mut o: *mut options_entry,
    mut ft: *mut format_tree,
    mut filter: *const libc::c_char,
    mut fs: *mut cmd_find_state,
) {
    let mut oe: *const options_table_entry = options_table_entry(o);
    let mut oo: *mut options = options_owner(o);
    let mut name: *const libc::c_char = options_name(o);
    let mut item: *mut window_customize_itemdata = 0 as *mut window_customize_itemdata;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut global: libc::c_int = 0 as libc::c_int;
    let mut array: libc::c_int = 0 as libc::c_int;
    let mut tag: uint64_t = 0;
    if !oe.is_null() && (*oe).flags & 0x2 as libc::c_int != 0 {
        return;
    }
    if !oe.is_null() && (*oe).flags & 0x1 as libc::c_int != 0 {
        array = 1 as libc::c_int
    }
    if scope as libc::c_uint == WINDOW_CUSTOMIZE_SERVER as libc::c_int as libc::c_uint
        || scope as libc::c_uint == WINDOW_CUSTOMIZE_GLOBAL_SESSION as libc::c_int as libc::c_uint
        || scope as libc::c_uint == WINDOW_CUSTOMIZE_GLOBAL_WINDOW as libc::c_int as libc::c_uint
    {
        global = 1 as libc::c_int
    }
    if (*data).hide_global != 0 && global != 0 {
        return;
    }
    format_add(
        ft,
        b"option_name\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        name,
    );
    format_add(
        ft,
        b"option_is_global\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        global,
    );
    format_add(
        ft,
        b"option_is_array\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        array,
    );
    text = window_customize_scope_text(scope, fs);
    format_add(
        ft,
        b"option_scope\x00" as *const u8 as *const libc::c_char,
        b"%s\x00" as *const u8 as *const libc::c_char,
        text,
    );
    free(text as *mut libc::c_void);
    if !oe.is_null() && !(*oe).unit.is_null() {
        format_add(
            ft,
            b"option_unit\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*oe).unit,
        );
    } else {
        format_add(
            ft,
            b"option_unit\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    if array == 0 {
        value = options_to_string(o, -(1 as libc::c_int), 0 as libc::c_int);
        format_add(
            ft,
            b"option_value\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            value,
        );
        free(value as *mut libc::c_void);
    }
    if !filter.is_null() {
        expanded = format_expand(ft, filter);
        if format_true(expanded) == 0 {
            free(expanded as *mut libc::c_void);
            return;
        }
        free(expanded as *mut libc::c_void);
    }
    item = window_customize_add_item(data);
    (*item).oo = oo;
    (*item).scope = scope;
    (*item).name = xstrdup(name);
    (*item).idx = -(1 as libc::c_int);
    if array != 0 {
        text = 0 as *mut libc::c_char
    } else {
        text = format_expand(ft, (*data).format)
    }
    tag = window_customize_get_tag(o, -(1 as libc::c_int), oe);
    top = mode_tree_add(
        (*data).data,
        top,
        item as *mut libc::c_void,
        tag,
        name,
        text,
        0 as libc::c_int,
    );
    free(text as *mut libc::c_void);
    if array != 0 {
        window_customize_build_array(data, top, scope, o, ft);
    };
}
unsafe extern "C" fn window_customize_find_user_options(
    mut oo: *mut options,
    mut list: *mut *mut *const libc::c_char,
    mut size: *mut u_int,
) {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: u_int = 0;
    o = options_first(oo);
    while !o.is_null() {
        name = options_name(o);
        if *name as libc::c_int != '@' as i32 {
            o = options_next(o)
        } else {
            i = 0 as libc::c_int as u_int;
            while i < *size {
                if strcmp(*(*list).offset(i as isize), name) == 0 as libc::c_int {
                    break;
                }
                i = i.wrapping_add(1)
            }
            if i != *size {
                o = options_next(o)
            } else {
                *list = xreallocarray(
                    *list as *mut libc::c_void,
                    (*size).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
                    ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ) as *mut *const libc::c_char;
                let fresh2 = *size;
                *size = (*size).wrapping_add(1);
                let ref mut fresh3 = *(*list).offset(fresh2 as isize);
                *fresh3 = name;
                o = options_next(o)
            }
        }
    }
}
unsafe extern "C" fn window_customize_build_options(
    mut data: *mut window_customize_modedata,
    mut title: *const libc::c_char,
    mut tag: uint64_t,
    mut scope0: window_customize_scope,
    mut oo0: *mut options,
    mut scope1: window_customize_scope,
    mut oo1: *mut options,
    mut scope2: window_customize_scope,
    mut oo2: *mut options,
    mut ft: *mut format_tree,
    mut filter: *const libc::c_char,
    mut fs: *mut cmd_find_state,
) {
    let mut top: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut loop_0: *mut options_entry = 0 as *mut options_entry;
    let mut list: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: u_int = 0 as libc::c_int as u_int;
    let mut i: u_int = 0;
    let mut scope: window_customize_scope = WINDOW_CUSTOMIZE_NONE;
    top = mode_tree_add(
        (*data).data,
        0 as *mut mode_tree_item,
        0 as *mut libc::c_void,
        tag,
        title,
        0 as *const libc::c_char,
        0 as libc::c_int,
    );
    mode_tree_no_tag(top);
    /*
     * We get the options from the first tree, but build it using the
     * values from the other two. Any tree can have user options so we need
     * to build a separate list of them.
     */
    window_customize_find_user_options(oo0, &mut list, &mut size); /* skip line */
    if !oo1.is_null() {
        window_customize_find_user_options(oo1, &mut list, &mut size); /* skip line */
    } /* skip line */
    if !oo2.is_null() {
        window_customize_find_user_options(oo2, &mut list, &mut size); /* skip line */
    } /* skip line */
    i = 0 as libc::c_int as u_int;
    while i < size {
        if !oo2.is_null() {
            o = options_get(oo0, *list.offset(i as isize))
        }
        if o.is_null() && !oo1.is_null() {
            o = options_get(oo1, *list.offset(i as isize))
        }
        if o.is_null() {
            o = options_get(oo2, *list.offset(i as isize))
        }
        if options_owner(o) == oo2 {
            scope = scope2
        } else if options_owner(o) == oo1 {
            scope = scope1
        } else {
            scope = scope0
        }
        window_customize_build_option(data, top, scope, o, ft, filter, fs);
        i = i.wrapping_add(1)
    }
    free(list as *mut libc::c_void);
    loop_0 = options_first(oo0);
    while !loop_0.is_null() {
        name = options_name(loop_0);
        if *name as libc::c_int == '@' as i32 {
            loop_0 = options_next(loop_0)
        } else {
            if !oo2.is_null() {
                o = options_get(oo2, name)
            } else if !oo1.is_null() {
                o = options_get(oo1, name)
            } else {
                o = loop_0
            }
            if options_owner(o) == oo2 {
                scope = scope2
            } else if options_owner(o) == oo1 {
                scope = scope1
            } else {
                scope = scope0
            }
            window_customize_build_option(data, top, scope, o, ft, filter, fs);
            loop_0 = options_next(loop_0)
        }
    }
}
unsafe extern "C" fn window_customize_build_keys(
    mut data: *mut window_customize_modedata,
    mut kt: *mut key_table,
    mut ft: *mut format_tree,
    mut filter: *const libc::c_char,
    mut fs: *mut cmd_find_state,
    mut number: u_int,
) {
    let mut top: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut child: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut mti: *mut mode_tree_item = 0 as *mut mode_tree_item;
    let mut item: *mut window_customize_itemdata = 0 as *mut window_customize_itemdata;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut title: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flag: *const libc::c_char = 0 as *const libc::c_char;
    let mut tag: uint64_t = 0;
    tag = ((1 as libc::c_ulonglong) << 62 as libc::c_int
        | ((number as uint64_t) << 54 as libc::c_int) as libc::c_ulonglong
        | 1 as libc::c_int as libc::c_ulonglong) as uint64_t;
    xasprintf(
        &mut title as *mut *mut libc::c_char,
        b"Key Table - %s\x00" as *const u8 as *const libc::c_char,
        (*kt).name,
    );
    top = mode_tree_add(
        (*data).data,
        0 as *mut mode_tree_item,
        0 as *mut libc::c_void,
        tag,
        title,
        0 as *const libc::c_char,
        0 as libc::c_int,
    );
    mode_tree_no_tag(top);
    free(title as *mut libc::c_void);
    ft = format_create_from_state(0 as *mut cmdq_item, 0 as *mut client, fs);
    format_add(
        ft,
        b"is_option\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    format_add(
        ft,
        b"is_key\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
    bd = key_bindings_first(kt);
    while !bd.is_null() {
        format_add(
            ft,
            b"key\x00" as *const u8 as *const libc::c_char,
            b"%s\x00" as *const u8 as *const libc::c_char,
            key_string_lookup_key((*bd).key, 0 as libc::c_int),
        );
        if !(*bd).note.is_null() {
            format_add(
                ft,
                b"key_note\x00" as *const u8 as *const libc::c_char,
                b"%s\x00" as *const u8 as *const libc::c_char,
                (*bd).note,
            );
        }
        if !filter.is_null() {
            expanded = format_expand(ft, filter);
            if format_true(expanded) == 0 {
                free(expanded as *mut libc::c_void);
                continue;
            } else {
                free(expanded as *mut libc::c_void);
            }
        }
        item = window_customize_add_item(data);
        (*item).scope = WINDOW_CUSTOMIZE_KEY;
        (*item).table = xstrdup((*kt).name);
        (*item).key = (*bd).key;
        (*item).name = xstrdup(key_string_lookup_key((*item).key, 0 as libc::c_int));
        (*item).idx = -(1 as libc::c_int);
        expanded = format_expand(ft, (*data).format);
        child = mode_tree_add(
            (*data).data,
            top,
            item as *mut libc::c_void,
            bd as uint64_t,
            expanded,
            0 as *const libc::c_char,
            0 as libc::c_int,
        );
        free(expanded as *mut libc::c_void);
        tmp = cmd_list_print((*bd).cmdlist, 0 as libc::c_int);
        xasprintf(
            &mut text as *mut *mut libc::c_char,
            b"#[ignore]%s\x00" as *const u8 as *const libc::c_char,
            tmp,
        );
        free(tmp as *mut libc::c_void);
        mti = mode_tree_add(
            (*data).data,
            child,
            item as *mut libc::c_void,
            (tag as libc::c_ulonglong
                | (*bd).key << 3 as libc::c_int
                | ((0 as libc::c_int) << 1 as libc::c_int) as libc::c_ulonglong
                | 1 as libc::c_int as libc::c_ulonglong) as uint64_t,
            b"Command\x00" as *const u8 as *const libc::c_char,
            text,
            -(1 as libc::c_int),
        );
        mode_tree_draw_as_parent(mti);
        mode_tree_no_tag(mti);
        free(text as *mut libc::c_void);
        if !(*bd).note.is_null() {
            xasprintf(
                &mut text as *mut *mut libc::c_char,
                b"#[ignore]%s\x00" as *const u8 as *const libc::c_char,
                (*bd).note,
            );
        } else {
            text = xstrdup(b"\x00" as *const u8 as *const libc::c_char)
        }
        mti = mode_tree_add(
            (*data).data,
            child,
            item as *mut libc::c_void,
            (tag as libc::c_ulonglong
                | (*bd).key << 3 as libc::c_int
                | ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulonglong
                | 1 as libc::c_int as libc::c_ulonglong) as uint64_t,
            b"Note\x00" as *const u8 as *const libc::c_char,
            text,
            -(1 as libc::c_int),
        );
        mode_tree_draw_as_parent(mti);
        mode_tree_no_tag(mti);
        free(text as *mut libc::c_void);
        if (*bd).flags & 0x1 as libc::c_int != 0 {
            flag = b"on\x00" as *const u8 as *const libc::c_char
        } else {
            flag = b"off\x00" as *const u8 as *const libc::c_char
        }
        mti = mode_tree_add(
            (*data).data,
            child,
            item as *mut libc::c_void,
            (tag as libc::c_ulonglong
                | (*bd).key << 3 as libc::c_int
                | ((2 as libc::c_int) << 1 as libc::c_int) as libc::c_ulonglong
                | 1 as libc::c_int as libc::c_ulonglong) as uint64_t,
            b"Repeat\x00" as *const u8 as *const libc::c_char,
            flag,
            -(1 as libc::c_int),
        );
        mode_tree_draw_as_parent(mti);
        mode_tree_no_tag(mti);
        bd = key_bindings_next(kt, bd)
    }
    format_free(ft);
}
unsafe extern "C" fn window_customize_build(
    mut modedata: *mut libc::c_void,
    mut _sort_crit: *mut mode_tree_sort_criteria,
    mut _tag: *mut uint64_t,
    mut filter: *const libc::c_char,
) {
    let mut data: *mut window_customize_modedata = modedata as *mut window_customize_modedata;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    let mut i: u_int = 0;
    let mut kt: *mut key_table = 0 as *mut key_table;
    i = 0 as libc::c_int as u_int;
    while i < (*data).item_size {
        window_customize_free_item(*(*data).item_list.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*data).item_list as *mut libc::c_void);
    (*data).item_list = 0 as *mut *mut window_customize_itemdata;
    (*data).item_size = 0 as libc::c_int as u_int;
    if cmd_find_valid_state(&mut (*data).fs) != 0 {
        cmd_find_copy_state(&mut fs, &mut (*data).fs);
    } else {
        cmd_find_from_pane(&mut fs, (*data).wp, 0 as libc::c_int);
    }
    ft = format_create_from_state(0 as *mut cmdq_item, 0 as *mut client, &mut fs);
    format_add(
        ft,
        b"is_option\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
    format_add(
        ft,
        b"is_key\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    window_customize_build_options(
        data,
        b"Server Options\x00" as *const u8 as *const libc::c_char,
        ((3 as libc::c_ulonglong) << 62 as libc::c_int
            | ((0x1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulonglong
            | 1 as libc::c_int as libc::c_ulonglong) as uint64_t,
        WINDOW_CUSTOMIZE_SERVER,
        global_options,
        WINDOW_CUSTOMIZE_NONE,
        0 as *mut options,
        WINDOW_CUSTOMIZE_NONE,
        0 as *mut options,
        ft,
        filter,
        &mut fs,
    );
    window_customize_build_options(
        data,
        b"Session Options\x00" as *const u8 as *const libc::c_char,
        ((3 as libc::c_ulonglong) << 62 as libc::c_int
            | ((0x2 as libc::c_int) << 1 as libc::c_int) as libc::c_ulonglong
            | 1 as libc::c_int as libc::c_ulonglong) as uint64_t,
        WINDOW_CUSTOMIZE_GLOBAL_SESSION,
        global_s_options,
        WINDOW_CUSTOMIZE_SESSION,
        (*fs.s).options,
        WINDOW_CUSTOMIZE_NONE,
        0 as *mut options,
        ft,
        filter,
        &mut fs,
    );
    window_customize_build_options(
        data,
        b"Window & Pane Options\x00" as *const u8 as *const libc::c_char,
        ((3 as libc::c_ulonglong) << 62 as libc::c_int
            | ((0x4 as libc::c_int) << 1 as libc::c_int) as libc::c_ulonglong
            | 1 as libc::c_int as libc::c_ulonglong) as uint64_t,
        WINDOW_CUSTOMIZE_GLOBAL_WINDOW,
        global_w_options,
        WINDOW_CUSTOMIZE_WINDOW,
        (*fs.w).options,
        WINDOW_CUSTOMIZE_PANE,
        (*fs.wp).options,
        ft,
        filter,
        &mut fs,
    );
    format_free(ft);
    ft = format_create_from_state(0 as *mut cmdq_item, 0 as *mut client, &mut fs);
    i = 0 as libc::c_int as u_int;
    kt = key_bindings_first_table();
    while !kt.is_null() {
        if !(*kt).key_bindings.rbh_root.is_null() {
            window_customize_build_keys(data, kt, ft, filter, &mut fs, i);
            i = i.wrapping_add(1);
            if i == 256 as libc::c_int as libc::c_uint {
                break;
            }
        }
        kt = key_bindings_next_table(kt)
    }
    format_free(ft);
}
unsafe extern "C" fn window_customize_draw_key(
    mut _data: *mut window_customize_modedata,
    mut item: *mut window_customize_itemdata,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut s: *mut screen = (*ctx).s;
    let mut cx: u_int = (*s).cx;
    let mut cy: u_int = (*s).cy;
    let mut kt: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut default_bd: *mut key_binding = 0 as *mut key_binding;
    let mut note: *const libc::c_char = 0 as *const libc::c_char;
    let mut period: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut default_cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    if item.is_null() || window_customize_get_key(item, &mut kt, &mut bd) == 0 {
        return;
    }
    note = (*bd).note;
    if note.is_null() {
        note = b"There is no note for this key.\x00" as *const u8 as *const libc::c_char
    }
    if *note as libc::c_int != '\u{0}' as i32
        && *note.offset(strlen(note).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            != '.' as i32
    {
        period = b".\x00" as *const u8 as *const libc::c_char
    }
    if screen_write_text(
        ctx,
        cx,
        sx,
        sy,
        0 as libc::c_int,
        &grid_default_cell as *const grid_cell,
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        note,
        period,
    ) == 0
    {
        return;
    }
    screen_write_cursormove(
        ctx,
        cx as libc::c_int,
        (*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
        0 as libc::c_int,
    );
    if (*s).cy
        >= cy
            .wrapping_add(sy)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return;
    }
    if screen_write_text(
        ctx,
        cx,
        sx,
        sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
        0 as libc::c_int,
        &grid_default_cell as *const grid_cell,
        b"This key is in the %s table.\x00" as *const u8 as *const libc::c_char,
        (*kt).name,
    ) == 0
    {
        return;
    }
    if screen_write_text(
        ctx,
        cx,
        sx,
        sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
        0 as libc::c_int,
        &grid_default_cell as *const grid_cell,
        b"This key %s repeat.\x00" as *const u8 as *const libc::c_char,
        if (*bd).flags & 0x1 as libc::c_int != 0 {
            b"does\x00" as *const u8 as *const libc::c_char
        } else {
            b"does not\x00" as *const u8 as *const libc::c_char
        },
    ) == 0
    {
        return;
    }
    screen_write_cursormove(
        ctx,
        cx as libc::c_int,
        (*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
        0 as libc::c_int,
    );
    if (*s).cy
        >= cy
            .wrapping_add(sy)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        return;
    }
    cmd = cmd_list_print((*bd).cmdlist, 0 as libc::c_int);
    if screen_write_text(
        ctx,
        cx,
        sx,
        sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
        0 as libc::c_int,
        &grid_default_cell as *const grid_cell,
        b"Command: %s\x00" as *const u8 as *const libc::c_char,
        cmd,
    ) == 0
    {
        free(cmd as *mut libc::c_void);
        return;
    }
    default_bd = key_bindings_get_default(kt, (*bd).key);
    if !default_bd.is_null() {
        default_cmd = cmd_list_print((*default_bd).cmdlist, 0 as libc::c_int);
        if strcmp(cmd, default_cmd) != 0 as libc::c_int
            && screen_write_text(
                ctx,
                cx,
                sx,
                sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                0 as libc::c_int,
                &grid_default_cell as *const grid_cell,
                b"The default is: %s\x00" as *const u8 as *const libc::c_char,
                default_cmd,
            ) == 0
        {
            free(default_cmd as *mut libc::c_void);
            free(cmd as *mut libc::c_void);
            return;
        }
        free(default_cmd as *mut libc::c_void);
    }
    free(cmd as *mut libc::c_void);
}
unsafe extern "C" fn window_customize_draw_option(
    mut data: *mut window_customize_modedata,
    mut item: *mut window_customize_itemdata,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut current_block: u64;
    let mut s: *mut screen = (*ctx).s;
    let mut cx: u_int = (*s).cx;
    let mut cy: u_int = (*s).cy;
    let mut idx: libc::c_int = 0;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut parent: *mut options_entry = 0 as *mut options_entry;
    let mut go: *mut options = 0 as *mut options;
    let mut wo: *mut options = 0 as *mut options;
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
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
    let mut choice: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut text: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut space: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    let mut unit: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expanded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut default_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut choices: [libc::c_char; 256] =
        *::std::mem::transmute::<&[u8; 256],
                                 &mut [libc::c_char; 256]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    let mut ft: *mut format_tree = 0 as *mut format_tree;
    if window_customize_check_item(data, item, &mut fs) == 0 {
        return;
    }
    name = (*item).name;
    idx = (*item).idx;
    o = options_get((*item).oo, name);
    if o.is_null() {
        return;
    }
    oe = options_table_entry(o);
    if !oe.is_null() && !(*oe).unit.is_null() {
        space = b" \x00" as *const u8 as *const libc::c_char;
        unit = (*oe).unit
    }
    ft = format_create_from_state(0 as *mut cmdq_item, 0 as *mut client, &mut fs);
    if oe.is_null() {
        text = b"This is a user option.\x00" as *const u8 as *const libc::c_char
    } else if (*oe).text.is_null() {
        text = b"This option doesn\'t have a description.\x00" as *const u8 as *const libc::c_char
    } else {
        text = (*oe).text
    }
    if !(screen_write_text(
        ctx,
        cx,
        sx,
        sy,
        0 as libc::c_int,
        &grid_default_cell as *const grid_cell,
        b"%s\x00" as *const u8 as *const libc::c_char,
        text,
    ) == 0)
    {
        screen_write_cursormove(
            ctx,
            cx as libc::c_int,
            (*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
            0 as libc::c_int,
        );
        if !((*s).cy
            >= cy
                .wrapping_add(sy)
                .wrapping_sub(1 as libc::c_int as libc::c_uint))
        {
            if oe.is_null() {
                text = b"user\x00" as *const u8 as *const libc::c_char
            } else if (*oe).scope & (0x4 as libc::c_int | 0x8 as libc::c_int)
                == 0x4 as libc::c_int | 0x8 as libc::c_int
            {
                text = b"window and pane\x00" as *const u8 as *const libc::c_char
            } else if (*oe).scope & 0x4 as libc::c_int != 0 {
                text = b"window\x00" as *const u8 as *const libc::c_char
            } else if (*oe).scope & 0x2 as libc::c_int != 0 {
                text = b"session\x00" as *const u8 as *const libc::c_char
            } else {
                text = b"server\x00" as *const u8 as *const libc::c_char
            }
            if !(screen_write_text(
                ctx,
                cx,
                sx,
                sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                0 as libc::c_int,
                &grid_default_cell as *const grid_cell,
                b"This is a %s option.\x00" as *const u8 as *const libc::c_char,
                text,
            ) == 0)
            {
                if !oe.is_null() && (*oe).flags & 0x1 as libc::c_int != 0 {
                    if idx != -(1 as libc::c_int) {
                        if screen_write_text(
                            ctx,
                            cx,
                            sx,
                            sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                            0 as libc::c_int,
                            &grid_default_cell as *const grid_cell,
                            b"This is an array option, index %u.\x00" as *const u8
                                as *const libc::c_char,
                            idx,
                        ) == 0
                        {
                            current_block = 13498252046719513127;
                        } else {
                            current_block = 17784502470059252271;
                        }
                    } else if screen_write_text(
                        ctx,
                        cx,
                        sx,
                        sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                        0 as libc::c_int,
                        &grid_default_cell as *const grid_cell,
                        b"This is an array option.\x00" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        current_block = 13498252046719513127;
                    } else {
                        current_block = 17784502470059252271;
                    }
                    match current_block {
                        13498252046719513127 => {}
                        _ => {
                            if idx == -(1 as libc::c_int) {
                                current_block = 13498252046719513127;
                            } else {
                                current_block = 12997042908615822766;
                            }
                        }
                    }
                } else {
                    current_block = 12997042908615822766;
                }
                match current_block {
                    13498252046719513127 => {}
                    _ => {
                        screen_write_cursormove(
                            ctx,
                            cx as libc::c_int,
                            (*s).cy.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
                            0 as libc::c_int,
                        );
                        if !((*s).cy
                            >= cy
                                .wrapping_add(sy)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint))
                        {
                            value = options_to_string(o, idx, 0 as libc::c_int);
                            if !oe.is_null() && idx == -(1 as libc::c_int) {
                                default_value = options_default_to_string(oe);
                                if strcmp(default_value, value) == 0 as libc::c_int {
                                    free(default_value as *mut libc::c_void);
                                    default_value = 0 as *mut libc::c_char
                                }
                            }
                            if !(screen_write_text(
                                ctx,
                                cx,
                                sx,
                                sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                                0 as libc::c_int,
                                &grid_default_cell as *const grid_cell,
                                b"Option value: %s%s%s\x00" as *const u8 as *const libc::c_char,
                                value,
                                space,
                                unit,
                            ) == 0)
                            {
                                if oe.is_null()
                                    || (*oe).type_0 as libc::c_uint
                                        == OPTIONS_TABLE_STRING as libc::c_int as libc::c_uint
                                {
                                    expanded = format_expand(ft, value);
                                    if strcmp(expanded, value) != 0 as libc::c_int {
                                        if screen_write_text(
                                            ctx,
                                            cx,
                                            sx,
                                            sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                                            0 as libc::c_int,
                                            &grid_default_cell as *const grid_cell,
                                            b"This expands to: %s\x00" as *const u8
                                                as *const libc::c_char,
                                            expanded,
                                        ) == 0
                                        {
                                            current_block = 13498252046719513127;
                                        } else {
                                            current_block = 9241535491006583629;
                                        }
                                    } else {
                                        current_block = 9241535491006583629;
                                    }
                                    match current_block {
                                        13498252046719513127 => {}
                                        _ => {
                                            free(expanded as *mut libc::c_void);
                                            current_block = 11441799814184323368;
                                        }
                                    }
                                } else {
                                    current_block = 11441799814184323368;
                                }
                                match current_block {
                                    13498252046719513127 => {}
                                    _ => {
                                        if !oe.is_null()
                                            && (*oe).type_0 as libc::c_uint
                                                == OPTIONS_TABLE_CHOICE as libc::c_int
                                                    as libc::c_uint
                                        {
                                            choice = (*oe).choices;
                                            while !(*choice).is_null() {
                                                strlcat(
                                                    choices.as_mut_ptr(),
                                                    *choice,
                                                    ::std::mem::size_of::<[libc::c_char; 256]>()
                                                        as libc::c_ulong,
                                                );
                                                strlcat(
                                                    choices.as_mut_ptr(),
                                                    b", \x00" as *const u8 as *const libc::c_char,
                                                    ::std::mem::size_of::<[libc::c_char; 256]>()
                                                        as libc::c_ulong,
                                                );
                                                choice = choice.offset(1)
                                            }
                                            choices[strlen(choices.as_mut_ptr())
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                as usize] = '\u{0}' as i32 as libc::c_char;
                                            if screen_write_text(
                                                ctx,
                                                cx,
                                                sx,
                                                sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                                                0 as libc::c_int,
                                                &grid_default_cell as *const grid_cell,
                                                b"Available values are: %s\x00" as *const u8
                                                    as *const libc::c_char,
                                                choices.as_mut_ptr(),
                                            ) == 0
                                            {
                                                current_block = 13498252046719513127;
                                            } else {
                                                current_block = 10067844863897285902;
                                            }
                                        } else {
                                            current_block = 10067844863897285902;
                                        }
                                        match current_block {
                                            13498252046719513127 => {}
                                            _ => {
                                                if !oe.is_null()
                                                    && (*oe).type_0 as libc::c_uint
                                                        == OPTIONS_TABLE_COLOUR as libc::c_int
                                                            as libc::c_uint
                                                {
                                                    if screen_write_text(
                                                        ctx,
                                                        cx,
                                                        sx,
                                                        sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                                                        1 as libc::c_int,
                                                        &grid_default_cell as *const grid_cell,
                                                        b"This is a colour option: \x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                    ) == 0
                                                    {
                                                        current_block = 13498252046719513127;
                                                    } else {
                                                        memcpy(
                                                            &mut gc as *mut grid_cell
                                                                as *mut libc::c_void,
                                                            &grid_default_cell as *const grid_cell
                                                                as *const libc::c_void,
                                                            ::std::mem::size_of::<grid_cell>()
                                                                as libc::c_ulong,
                                                        );
                                                        gc.fg = options_get_number((*item).oo, name)
                                                            as libc::c_int;
                                                        if screen_write_text(
                                                            ctx,
                                                            cx,
                                                            sx,
                                                            sy.wrapping_sub(
                                                                (*s).cy.wrapping_sub(cy),
                                                            ),
                                                            0 as libc::c_int,
                                                            &mut gc as *mut grid_cell,
                                                            b"EXAMPLE\x00" as *const u8
                                                                as *const libc::c_char,
                                                        ) == 0
                                                        {
                                                            current_block = 13498252046719513127;
                                                        } else {
                                                            current_block = 9437375157805982253;
                                                        }
                                                    }
                                                } else {
                                                    current_block = 9437375157805982253;
                                                }
                                                match current_block {
                                                    13498252046719513127 => {}
                                                    _ => {
                                                        if !oe.is_null()
                                                            && (*oe).flags & 0x4 as libc::c_int != 0
                                                        {
                                                            if screen_write_text(
                                                                ctx,
                                                                cx,
                                                                sx,
                                                                sy.wrapping_sub(
                                                                    (*s).cy.wrapping_sub(cy),
                                                                ),
                                                                1 as libc::c_int,
                                                                &grid_default_cell
                                                                    as *const grid_cell,
                                                                b"This is a style option: \x00"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                            ) == 0
                                                            {
                                                                current_block =
                                                                    13498252046719513127;
                                                            } else {
                                                                style_apply(
                                                                    &mut gc,
                                                                    (*item).oo,
                                                                    name,
                                                                    ft,
                                                                );
                                                                if screen_write_text(
                                                                    ctx,
                                                                    cx,
                                                                    sx,
                                                                    sy.wrapping_sub(
                                                                        (*s).cy.wrapping_sub(cy),
                                                                    ),
                                                                    0 as libc::c_int,
                                                                    &mut gc as *mut grid_cell,
                                                                    b"EXAMPLE\x00" as *const u8
                                                                        as *const libc::c_char,
                                                                ) == 0
                                                                {
                                                                    current_block =
                                                                        13498252046719513127;
                                                                } else {
                                                                    current_block =
                                                                        7318352876044315808;
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 7318352876044315808;
                                                        }
                                                        match current_block {
                                                            13498252046719513127 => {}
                                                            _ => {
                                                                if !default_value.is_null() {
                                                                    if screen_write_text(ctx,
                                                                                         cx,
                                                                                         sx,
                                                                                         sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                                                                                         0
                                                                                             as
                                                                                             libc::c_int,
                                                                                         &grid_default_cell
                                                                                             as
                                                                                             *const grid_cell,
                                                                                         b"The default is: %s%s%s\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         default_value,
                                                                                         space,
                                                                                         unit)
                                                                           ==
                                                                           0 {
                                                                        current_block
                                                                            =
                                                                            13498252046719513127;
                                                                    } else {
                                                                        current_block
                                                                            =
                                                                            2472048668343472511;
                                                                    }
                                                                } else {
                                                                    current_block =
                                                                        2472048668343472511;
                                                                }
                                                                match current_block {
                                                                    13498252046719513127 => {}
                                                                    _ => {
                                                                        screen_write_cursormove(
                                                                            ctx,
                                                                            cx as libc::c_int,
                                                                            (*s).cy.wrapping_add(
                                                                                1 as libc::c_int
                                                                                    as libc::c_uint,
                                                                            )
                                                                                as libc::c_int,
                                                                            0 as libc::c_int,
                                                                        );
                                                                        if !((*s).cy
                                                                            > cy.wrapping_add(sy)
                                                                                .wrapping_sub(
                                                                                1 as libc::c_int
                                                                                    as libc::c_uint,
                                                                            ))
                                                                        {
                                                                            if !oe.is_null()
                                                                                   &&
                                                                                   (*oe).flags
                                                                                       &
                                                                                       0x1
                                                                                           as
                                                                                           libc::c_int
                                                                                       !=
                                                                                       0
                                                                               {
                                                                                wo
                                                                                    =
                                                                                    0
                                                                                        as
                                                                                        *mut options;
                                                                                go
                                                                                    =
                                                                                    0
                                                                                        as
                                                                                        *mut options
                                                                            } else {
                                                                                match (*item).scope
                                                                                          as
                                                                                          libc::c_uint
                                                                                    {
                                                                                    7
                                                                                    =>
                                                                                    {
                                                                                        wo
                                                                                            =
                                                                                            options_get_parent((*item).oo);
                                                                                        go
                                                                                            =
                                                                                            options_get_parent(wo)
                                                                                    }
                                                                                    6
                                                                                    |
                                                                                    4
                                                                                    =>
                                                                                    {
                                                                                        wo
                                                                                            =
                                                                                            0
                                                                                                as
                                                                                                *mut options;
                                                                                        go
                                                                                            =
                                                                                            options_get_parent((*item).oo)
                                                                                    }
                                                                                    _
                                                                                    =>
                                                                                    {
                                                                                        wo
                                                                                            =
                                                                                            0
                                                                                                as
                                                                                                *mut options;
                                                                                        go
                                                                                            =
                                                                                            0
                                                                                                as
                                                                                                *mut options
                                                                                    }
                                                                                }
                                                                            }
                                                                            if !wo.is_null()
                                                                                && options_owner(o)
                                                                                    != wo
                                                                            {
                                                                                parent
                                                                                    =
                                                                                    options_get_only(wo,
                                                                                                     name);
                                                                                if !parent.is_null()
                                                                                {
                                                                                    value
                                                                                        =
                                                                                        options_to_string(parent,
                                                                                                          -(1
                                                                                                                as
                                                                                                                libc::c_int),
                                                                                                          0
                                                                                                              as
                                                                                                              libc::c_int);
                                                                                    if screen_write_text(ctx,
                                                                                                         (*s).cx,
                                                                                                         sx,
                                                                                                         sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                                                                                                         0
                                                                                                             as
                                                                                                             libc::c_int,
                                                                                                         &grid_default_cell
                                                                                                             as
                                                                                                             *const grid_cell,
                                                                                                         b"Window value (from window %u): %s%s%s\x00"
                                                                                                             as
                                                                                                             *const u8
                                                                                                             as
                                                                                                             *const libc::c_char,
                                                                                                         (*fs.wl).idx,
                                                                                                         value,
                                                                                                         space,
                                                                                                         unit)
                                                                                           ==
                                                                                           0
                                                                                       {
                                                                                        current_block
                                                                                            =
                                                                                            13498252046719513127;
                                                                                    } else {
                                                                                        current_block
                                                                                            =
                                                                                            15447629348493591490;
                                                                                    }
                                                                                } else {
                                                                                    current_block
                                                                                        =
                                                                                        15447629348493591490;
                                                                                }
                                                                            } else {
                                                                                current_block
                                                                                    =
                                                                                    15447629348493591490;
                                                                            }
                                                                            match current_block
                                                                                {
                                                                                13498252046719513127
                                                                                =>
                                                                                {
                                                                                }
                                                                                _
                                                                                =>
                                                                                {
                                                                                    if !go.is_null()
                                                                                           &&
                                                                                           options_owner(o)
                                                                                               !=
                                                                                               go
                                                                                       {
                                                                                        parent
                                                                                            =
                                                                                            options_get_only(go,
                                                                                                             name);
                                                                                        if !parent.is_null()
                                                                                           {
                                                                                            value
                                                                                                =
                                                                                                options_to_string(parent,
                                                                                                                  -(1
                                                                                                                        as
                                                                                                                        libc::c_int),
                                                                                                                  0
                                                                                                                      as
                                                                                                                      libc::c_int);
                                                                                            (screen_write_text(ctx,
                                                                                                               (*s).cx,
                                                                                                               sx,
                                                                                                               sy.wrapping_sub((*s).cy.wrapping_sub(cy)),
                                                                                                               0
                                                                                                                   as
                                                                                                                   libc::c_int,
                                                                                                               &grid_default_cell
                                                                                                                   as
                                                                                                                   *const grid_cell,
                                                                                                               b"Global value: %s%s%s\x00"
                                                                                                                   as
                                                                                                                   *const u8
                                                                                                                   as
                                                                                                                   *const libc::c_char,
                                                                                                               value,
                                                                                                               space,
                                                                                                               unit))
                                                                                                ==
                                                                                                0;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(value as *mut libc::c_void);
    free(default_value as *mut libc::c_void);
    format_free(ft);
}
unsafe extern "C" fn window_customize_draw(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut ctx: *mut screen_write_ctx,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut data: *mut window_customize_modedata = modedata as *mut window_customize_modedata;
    let mut item: *mut window_customize_itemdata = itemdata as *mut window_customize_itemdata;
    if item.is_null() {
        return;
    }
    if (*item).scope as libc::c_uint == WINDOW_CUSTOMIZE_KEY as libc::c_int as libc::c_uint {
        window_customize_draw_key(data, item, ctx, sx, sy);
    } else {
        window_customize_draw_option(data, item, ctx, sx, sy);
    };
}
unsafe extern "C" fn window_customize_menu(
    mut modedata: *mut libc::c_void,
    mut c: *mut client,
    mut key: key_code,
) {
    let mut data: *mut window_customize_modedata = modedata as *mut window_customize_modedata;
    let mut wp: *mut window_pane = (*data).wp;
    let mut wme: *mut window_mode_entry = 0 as *mut window_mode_entry;
    wme = (*wp).modes.tqh_first;
    if wme.is_null() || (*wme).data != modedata {
        return;
    }
    window_customize_key(
        wme,
        c,
        0 as *mut session,
        0 as *mut winlink,
        key,
        0 as *mut mouse_event,
    );
}
unsafe extern "C" fn window_customize_height(
    mut _modedata: *mut libc::c_void,
    mut _height: u_int,
) -> u_int {
    return 12 as libc::c_int as u_int;
}
/* $OpenBSD$ */
/*
 * Copyright (c) 2020 Nicholas Marriott <nicholas.marriott@gmail.com>
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
unsafe extern "C" fn window_customize_init(
    mut wme: *mut window_mode_entry,
    mut fs: *mut cmd_find_state,
    mut args: *mut args,
) -> *mut screen {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_customize_modedata = 0 as *mut window_customize_modedata;
    let mut s: *mut screen = 0 as *mut screen;
    data = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<window_customize_modedata>() as libc::c_ulong,
    ) as *mut window_customize_modedata;
    (*wme).data = data as *mut libc::c_void;
    (*data).wp = wp;
    (*data).references = 1 as libc::c_int;
    memcpy(
        &mut (*data).fs as *mut cmd_find_state as *mut libc::c_void,
        fs as *const libc::c_void,
        ::std::mem::size_of::<cmd_find_state>() as libc::c_ulong,
    );
    if args.is_null() || args_has(args, 'F' as i32 as u_char) == 0 {
        (*data).format =
            xstrdup(b"#{?is_option,#{?option_is_global,,#[reverse](#{option_scope})#[default] }#[ignore]#{option_value}#{?option_unit, #{option_unit},},#{key}}\x00"
                        as *const u8 as *const libc::c_char)
    } else {
        (*data).format = xstrdup(args_get(args, 'F' as i32 as u_char))
    }
    (*data).data = mode_tree_start(
        wp,
        args,
        Some(
            window_customize_build
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut mode_tree_sort_criteria,
                    _: *mut uint64_t,
                    _: *const libc::c_char,
                ) -> (),
        ),
        Some(
            window_customize_draw
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                    _: *mut screen_write_ctx,
                    _: u_int,
                    _: u_int,
                ) -> (),
        ),
        None,
        Some(
            window_customize_menu
                as unsafe extern "C" fn(_: *mut libc::c_void, _: *mut client, _: key_code) -> (),
        ),
        Some(
            window_customize_height
                as unsafe extern "C" fn(_: *mut libc::c_void, _: u_int) -> u_int,
        ),
        data as *mut libc::c_void,
        window_customize_menu_items.as_ptr(),
        0 as *mut *const libc::c_char,
        0 as libc::c_int as u_int,
        &mut s,
    );
    mode_tree_zoom((*data).data, args);
    mode_tree_build((*data).data);
    mode_tree_draw((*data).data);
    return s;
}
unsafe extern "C" fn window_customize_destroy(mut data: *mut window_customize_modedata) {
    let mut i: u_int = 0;
    (*data).references -= 1;
    if (*data).references != 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int as u_int;
    while i < (*data).item_size {
        window_customize_free_item(*(*data).item_list.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*data).item_list as *mut libc::c_void);
    free((*data).format as *mut libc::c_void);
    free(data as *mut libc::c_void);
}
unsafe extern "C" fn window_customize_free(mut wme: *mut window_mode_entry) {
    let mut data: *mut window_customize_modedata = (*wme).data as *mut window_customize_modedata;
    if data.is_null() {
        return;
    }
    (*data).dead = 1 as libc::c_int;
    mode_tree_free((*data).data);
    window_customize_destroy(data);
}
unsafe extern "C" fn window_customize_resize(
    mut wme: *mut window_mode_entry,
    mut sx: u_int,
    mut sy: u_int,
) {
    let mut data: *mut window_customize_modedata = (*wme).data as *mut window_customize_modedata;
    mode_tree_resize((*data).data, sx, sy);
}
unsafe extern "C" fn window_customize_free_callback(mut modedata: *mut libc::c_void) {
    window_customize_destroy(modedata as *mut window_customize_modedata);
}
unsafe extern "C" fn window_customize_free_item_callback(mut itemdata: *mut libc::c_void) {
    let mut item: *mut window_customize_itemdata = itemdata as *mut window_customize_itemdata;
    let mut data: *mut window_customize_modedata = (*item).data;
    window_customize_free_item(item);
    window_customize_destroy(data);
}
unsafe extern "C" fn window_customize_set_option_callback(
    mut c: *mut client,
    mut itemdata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut _done: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut item: *mut window_customize_itemdata = itemdata as *mut window_customize_itemdata;
    let mut data: *mut window_customize_modedata = (*item).data;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    let mut oo: *mut options = (*item).oo;
    let mut name: *const libc::c_char = (*item).name;
    let mut cause: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_int = (*item).idx;
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 || (*data).dead != 0 {
        return 0 as libc::c_int;
    }
    if item.is_null() || window_customize_check_item(data, item, 0 as *mut cmd_find_state) == 0 {
        return 0 as libc::c_int;
    }
    o = options_get(oo, name);
    if o.is_null() {
        return 0 as libc::c_int;
    }
    oe = options_table_entry(o);
    if !oe.is_null() && (*oe).flags & 0x1 as libc::c_int != 0 {
        if idx == -(1 as libc::c_int) {
            idx = 0 as libc::c_int;
            while idx < 2147483647 as libc::c_int {
                if options_array_get(o, idx as u_int).is_null() {
                    break;
                }
                idx += 1
            }
        }
        if options_array_set(o, idx as u_int, s, 0 as libc::c_int, &mut cause) != 0 as libc::c_int {
            current_block = 229924417310053542;
        } else {
            current_block = 2668756484064249700;
        }
    } else if options_from_string(oo, oe, name, s, 0 as libc::c_int, &mut cause) != 0 as libc::c_int
    {
        current_block = 229924417310053542;
    } else {
        current_block = 2668756484064249700;
    }
    match current_block {
        229924417310053542 => {
            *cause = ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<u_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *cause as u_char as libc::c_int;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = toupper(*cause as u_char as libc::c_int)
                    }
                } else {
                    __res =
                        *(*__ctype_toupper_loc()).offset(*cause as u_char as libc::c_int as isize)
                }
                __res
            }) as libc::c_char;
            status_message_set(
                c,
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char,
                cause,
            );
            free(cause as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        _ => {
            options_push_changes((*item).name);
            mode_tree_build((*data).data);
            mode_tree_draw((*data).data);
            (*(*data).wp).flags |= 0x1 as libc::c_int;
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn window_customize_set_option(
    mut c: *mut client,
    mut data: *mut window_customize_modedata,
    mut item: *mut window_customize_itemdata,
    mut global: libc::c_int,
    mut pane: libc::c_int,
) {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    let mut oe: *const options_table_entry = 0 as *const options_table_entry;
    let mut oo: *mut options = 0 as *mut options;
    let mut new_item: *mut window_customize_itemdata = 0 as *mut window_customize_itemdata;
    let mut flag: libc::c_int = 0;
    let mut idx: libc::c_int = (*item).idx;
    let mut scope: window_customize_scope = WINDOW_CUSTOMIZE_NONE;
    let mut choice: u_int = 0;
    let mut name: *const libc::c_char = (*item).name;
    let mut space: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    let mut prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fs: cmd_find_state = cmd_find_state {
        flags: 0,
        current: 0 as *mut cmd_find_state,
        s: 0 as *mut session,
        wl: 0 as *mut winlink,
        w: 0 as *mut window,
        wp: 0 as *mut window_pane,
        idx: 0,
    };
    if item.is_null() || window_customize_check_item(data, item, &mut fs) == 0 {
        return;
    }
    o = options_get((*item).oo, name);
    if o.is_null() {
        return;
    }
    oe = options_table_entry(o);
    if !oe.is_null() && !(*oe).scope & 0x8 as libc::c_int != 0 {
        pane = 0 as libc::c_int
    }
    if !oe.is_null() && (*oe).flags & 0x1 as libc::c_int != 0 {
        scope = (*item).scope;
        oo = (*item).oo
    } else {
        if global != 0 {
            match (*item).scope as libc::c_uint {
                0 | 1 | 2 | 3 | 5 => scope = (*item).scope,
                4 => scope = WINDOW_CUSTOMIZE_GLOBAL_SESSION,
                6 | 7 => scope = WINDOW_CUSTOMIZE_GLOBAL_WINDOW,
                _ => {}
            }
        } else {
            match (*item).scope as libc::c_uint {
                0 | 1 | 2 | 4 => scope = (*item).scope,
                6 | 7 => {
                    if pane != 0 {
                        scope = WINDOW_CUSTOMIZE_PANE
                    } else {
                        scope = WINDOW_CUSTOMIZE_WINDOW
                    }
                }
                3 => scope = WINDOW_CUSTOMIZE_SESSION,
                5 => {
                    if pane != 0 {
                        scope = WINDOW_CUSTOMIZE_PANE
                    } else {
                        scope = WINDOW_CUSTOMIZE_WINDOW
                    }
                }
                _ => {}
            }
        }
        if scope as libc::c_uint == (*item).scope as libc::c_uint {
            oo = (*item).oo
        } else {
            oo = window_customize_get_tree(scope, &mut fs)
        }
    }
    if !oe.is_null()
        && (*oe).type_0 as libc::c_uint == OPTIONS_TABLE_FLAG as libc::c_int as libc::c_uint
    {
        flag = options_get_number(oo, name) as libc::c_int;
        options_set_number(oo, name, (flag == 0) as libc::c_int as libc::c_longlong);
    } else if !oe.is_null()
        && (*oe).type_0 as libc::c_uint == OPTIONS_TABLE_CHOICE as libc::c_int as libc::c_uint
    {
        choice = options_get_number(oo, name) as u_int;
        if (*(*oe)
            .choices
            .offset(choice.wrapping_add(1 as libc::c_int as libc::c_uint) as isize))
        .is_null()
        {
            choice = 0 as libc::c_int as u_int
        } else {
            choice = choice.wrapping_add(1)
        }
        options_set_number(oo, name, choice as libc::c_longlong);
    } else {
        text = window_customize_scope_text(scope, &mut fs);
        if *text as libc::c_int != '\u{0}' as i32 {
            space = b", for \x00" as *const u8 as *const libc::c_char
        } else if scope as libc::c_uint != WINDOW_CUSTOMIZE_SERVER as libc::c_int as libc::c_uint {
            space = b", global\x00" as *const u8 as *const libc::c_char
        }
        if !oe.is_null() && (*oe).flags & 0x1 as libc::c_int != 0 {
            if idx == -(1 as libc::c_int) {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"(%s[+]%s%s) \x00" as *const u8 as *const libc::c_char,
                    name,
                    space,
                    text,
                );
            } else {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"(%s[%d]%s%s) \x00" as *const u8 as *const libc::c_char,
                    name,
                    idx,
                    space,
                    text,
                );
            }
        } else {
            xasprintf(
                &mut prompt as *mut *mut libc::c_char,
                b"(%s%s%s) \x00" as *const u8 as *const libc::c_char,
                name,
                space,
                text,
            );
        }
        free(text as *mut libc::c_void);
        value = options_to_string(o, idx, 0 as libc::c_int);
        new_item = xcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<window_customize_itemdata>() as libc::c_ulong,
        ) as *mut window_customize_itemdata;
        (*new_item).data = data;
        (*new_item).scope = scope;
        (*new_item).oo = oo;
        (*new_item).name = xstrdup(name);
        (*new_item).idx = idx;
        (*data).references += 1;
        status_prompt_set(
            c,
            0 as *mut cmd_find_state,
            prompt,
            value,
            Some(
                window_customize_set_option_callback
                    as unsafe extern "C" fn(
                        _: *mut client,
                        _: *mut libc::c_void,
                        _: *const libc::c_char,
                        _: libc::c_int,
                    ) -> libc::c_int,
            ),
            Some(
                window_customize_free_item_callback
                    as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
            ),
            new_item as *mut libc::c_void,
            0x8 as libc::c_int,
        );
        free(prompt as *mut libc::c_void);
        free(value as *mut libc::c_void);
    };
}
unsafe extern "C" fn window_customize_unset_option(
    mut data: *mut window_customize_modedata,
    mut item: *mut window_customize_itemdata,
) {
    let mut o: *mut options_entry = 0 as *mut options_entry;
    if item.is_null() || window_customize_check_item(data, item, 0 as *mut cmd_find_state) == 0 {
        return;
    }
    o = options_get((*item).oo, (*item).name);
    if o.is_null() {
        return;
    }
    if (*item).idx != -(1 as libc::c_int)
        && item == mode_tree_get_current((*data).data) as *mut window_customize_itemdata
    {
        mode_tree_up((*data).data, 0 as libc::c_int);
    }
    options_remove_or_default(o, (*item).idx, 0 as *mut *mut libc::c_char);
}
unsafe extern "C" fn window_customize_reset_option(
    mut data: *mut window_customize_modedata,
    mut item: *mut window_customize_itemdata,
) {
    let mut oo: *mut options = 0 as *mut options;
    let mut o: *mut options_entry = 0 as *mut options_entry;
    if item.is_null() || window_customize_check_item(data, item, 0 as *mut cmd_find_state) == 0 {
        return;
    }
    if (*item).idx != -(1 as libc::c_int) {
        return;
    }
    oo = (*item).oo;
    while !oo.is_null() {
        o = options_get_only((*item).oo, (*item).name);
        if !o.is_null() {
            options_remove_or_default(o, -(1 as libc::c_int), 0 as *mut *mut libc::c_char);
        }
        oo = options_get_parent(oo)
    }
}
unsafe extern "C" fn window_customize_set_command_callback(
    mut c: *mut client,
    mut itemdata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut _done: libc::c_int,
) -> libc::c_int {
    let mut item: *mut window_customize_itemdata = itemdata as *mut window_customize_itemdata;
    let mut data: *mut window_customize_modedata = (*item).data;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut pr: *mut cmd_parse_result = 0 as *mut cmd_parse_result;
    let mut error: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 || (*data).dead != 0 {
        return 0 as libc::c_int;
    }
    if item.is_null() || window_customize_get_key(item, 0 as *mut *mut key_table, &mut bd) == 0 {
        return 0 as libc::c_int;
    }
    pr = cmd_parse_from_string(s, 0 as *mut cmd_parse_input);
    match (*pr).status as libc::c_uint {
        0 => error = xstrdup(b"empty command\x00" as *const u8 as *const libc::c_char),
        1 => error = (*pr).error,
        2 | _ => {
            cmd_list_free((*bd).cmdlist);
            (*bd).cmdlist = (*pr).cmdlist;
            mode_tree_build((*data).data);
            mode_tree_draw((*data).data);
            (*(*data).wp).flags |= 0x1 as libc::c_int;
            return 0 as libc::c_int;
        }
    }
    *error = ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<u_char>() as libc::c_ulong > 1 as libc::c_int as libc::c_ulong {
            if 0 != 0 {
                let mut __c: libc::c_int = *error as u_char as libc::c_int;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_toupper_loc()).offset(__c as isize)
                }
            } else {
                __res = toupper(*error as u_char as libc::c_int)
            }
        } else {
            __res = *(*__ctype_toupper_loc()).offset(*error as u_char as libc::c_int as isize)
        }
        __res
    }) as libc::c_char;
    status_message_set(
        c,
        -(1 as libc::c_int),
        1 as libc::c_int,
        b"%s\x00" as *const u8 as *const libc::c_char,
        error,
    );
    free(error as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_customize_set_note_callback(
    mut _c: *mut client,
    mut itemdata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut _done: libc::c_int,
) -> libc::c_int {
    let mut item: *mut window_customize_itemdata = itemdata as *mut window_customize_itemdata;
    let mut data: *mut window_customize_modedata = (*item).data;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 || (*data).dead != 0 {
        return 0 as libc::c_int;
    }
    if item.is_null() || window_customize_get_key(item, 0 as *mut *mut key_table, &mut bd) == 0 {
        return 0 as libc::c_int;
    }
    free((*bd).note as *mut libc::c_void);
    (*bd).note = xstrdup(s);
    mode_tree_build((*data).data);
    mode_tree_draw((*data).data);
    (*(*data).wp).flags |= 0x1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_customize_set_key(
    mut c: *mut client,
    mut data: *mut window_customize_modedata,
    mut item: *mut window_customize_itemdata,
) {
    let mut key: key_code = (*item).key;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_item: *mut window_customize_itemdata = 0 as *mut window_customize_itemdata;
    if item.is_null() || window_customize_get_key(item, 0 as *mut *mut key_table, &mut bd) == 0 {
        return;
    }
    s = mode_tree_get_current_name((*data).data);
    if strcmp(s, b"Repeat\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*bd).flags ^= 0x1 as libc::c_int
    } else if strcmp(s, b"Command\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        xasprintf(
            &mut prompt as *mut *mut libc::c_char,
            b"(%s) \x00" as *const u8 as *const libc::c_char,
            key_string_lookup_key(key, 0 as libc::c_int),
        );
        value = cmd_list_print((*bd).cmdlist, 0 as libc::c_int);
        new_item = xcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<window_customize_itemdata>() as libc::c_ulong,
        ) as *mut window_customize_itemdata;
        (*new_item).data = data;
        (*new_item).scope = (*item).scope;
        (*new_item).table = xstrdup((*item).table);
        (*new_item).key = key;
        (*data).references += 1;
        status_prompt_set(
            c,
            0 as *mut cmd_find_state,
            prompt,
            value,
            Some(
                window_customize_set_command_callback
                    as unsafe extern "C" fn(
                        _: *mut client,
                        _: *mut libc::c_void,
                        _: *const libc::c_char,
                        _: libc::c_int,
                    ) -> libc::c_int,
            ),
            Some(
                window_customize_free_item_callback
                    as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
            ),
            new_item as *mut libc::c_void,
            0x8 as libc::c_int,
        );
        free(prompt as *mut libc::c_void);
        free(value as *mut libc::c_void);
    } else if strcmp(s, b"Note\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        xasprintf(
            &mut prompt as *mut *mut libc::c_char,
            b"(%s) \x00" as *const u8 as *const libc::c_char,
            key_string_lookup_key(key, 0 as libc::c_int),
        );
        new_item = xcalloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<window_customize_itemdata>() as libc::c_ulong,
        ) as *mut window_customize_itemdata;
        (*new_item).data = data;
        (*new_item).scope = (*item).scope;
        (*new_item).table = xstrdup((*item).table);
        (*new_item).key = key;
        (*data).references += 1;
        status_prompt_set(
            c,
            0 as *mut cmd_find_state,
            prompt,
            if (*bd).note.is_null() {
                b"\x00" as *const u8 as *const libc::c_char
            } else {
                (*bd).note
            },
            Some(
                window_customize_set_note_callback
                    as unsafe extern "C" fn(
                        _: *mut client,
                        _: *mut libc::c_void,
                        _: *const libc::c_char,
                        _: libc::c_int,
                    ) -> libc::c_int,
            ),
            Some(
                window_customize_free_item_callback
                    as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
            ),
            new_item as *mut libc::c_void,
            0x8 as libc::c_int,
        );
        free(prompt as *mut libc::c_void);
    };
}
unsafe extern "C" fn window_customize_unset_key(
    mut data: *mut window_customize_modedata,
    mut item: *mut window_customize_itemdata,
) {
    let mut kt: *mut key_table = 0 as *mut key_table;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    if item.is_null() || window_customize_get_key(item, &mut kt, &mut bd) == 0 {
        return;
    }
    if item == mode_tree_get_current((*data).data) as *mut window_customize_itemdata {
        mode_tree_collapse_current((*data).data);
        mode_tree_up((*data).data, 0 as libc::c_int);
    }
    key_bindings_remove((*kt).name, (*bd).key);
}
unsafe extern "C" fn window_customize_reset_key(
    mut data: *mut window_customize_modedata,
    mut item: *mut window_customize_itemdata,
) {
    let mut kt: *mut key_table = 0 as *mut key_table;
    let mut dd: *mut key_binding = 0 as *mut key_binding;
    let mut bd: *mut key_binding = 0 as *mut key_binding;
    if item.is_null() || window_customize_get_key(item, &mut kt, &mut bd) == 0 {
        return;
    }
    dd = key_bindings_get_default(kt, (*bd).key);
    if !dd.is_null() && (*bd).cmdlist == (*dd).cmdlist {
        return;
    }
    if dd.is_null() && item == mode_tree_get_current((*data).data) as *mut window_customize_itemdata
    {
        mode_tree_collapse_current((*data).data);
        mode_tree_up((*data).data, 0 as libc::c_int);
    }
    key_bindings_reset((*kt).name, (*bd).key);
}
unsafe extern "C" fn window_customize_change_each(
    mut modedata: *mut libc::c_void,
    mut itemdata: *mut libc::c_void,
    mut _c: *mut client,
    mut _key: key_code,
) {
    let mut data: *mut window_customize_modedata = modedata as *mut window_customize_modedata;
    let mut item: *mut window_customize_itemdata = itemdata as *mut window_customize_itemdata;
    match (*data).change as libc::c_uint {
        0 => {
            if (*item).scope as libc::c_uint == WINDOW_CUSTOMIZE_KEY as libc::c_int as libc::c_uint
            {
                window_customize_unset_key(data, item);
            } else {
                window_customize_unset_option(data, item);
            }
        }
        1 => {
            if (*item).scope as libc::c_uint == WINDOW_CUSTOMIZE_KEY as libc::c_int as libc::c_uint
            {
                window_customize_reset_key(data, item);
            } else {
                window_customize_reset_option(data, item);
            }
        }
        _ => {}
    }
    if (*item).scope as libc::c_uint != WINDOW_CUSTOMIZE_KEY as libc::c_int as libc::c_uint {
        options_push_changes((*item).name);
    };
}
unsafe extern "C" fn window_customize_change_current_callback(
    mut _c: *mut client,
    mut modedata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut _done: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_customize_modedata = modedata as *mut window_customize_modedata;
    let mut item: *mut window_customize_itemdata = 0 as *mut window_customize_itemdata;
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 || (*data).dead != 0 {
        return 0 as libc::c_int;
    }
    if ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<u_char>() as libc::c_ulong > 1 as libc::c_int as libc::c_ulong {
            if 0 != 0 {
                let mut __c: libc::c_int =
                    *s.offset(0 as libc::c_int as isize) as u_char as libc::c_int;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                }
            } else {
                __res = tolower(*s.offset(0 as libc::c_int as isize) as u_char as libc::c_int)
            }
        } else {
            __res = *(*__ctype_tolower_loc())
                .offset(*s.offset(0 as libc::c_int as isize) as u_char as libc::c_int as isize)
        }
        __res
    }) != 'y' as i32
        || *s.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
    {
        return 0 as libc::c_int;
    }
    item = mode_tree_get_current((*data).data) as *mut window_customize_itemdata;
    match (*data).change as libc::c_uint {
        0 => {
            if (*item).scope as libc::c_uint == WINDOW_CUSTOMIZE_KEY as libc::c_int as libc::c_uint
            {
                window_customize_unset_key(data, item);
            } else {
                window_customize_unset_option(data, item);
            }
        }
        1 => {
            if (*item).scope as libc::c_uint == WINDOW_CUSTOMIZE_KEY as libc::c_int as libc::c_uint
            {
                window_customize_reset_key(data, item);
            } else {
                window_customize_reset_option(data, item);
            }
        }
        _ => {}
    }
    if (*item).scope as libc::c_uint != WINDOW_CUSTOMIZE_KEY as libc::c_int as libc::c_uint {
        options_push_changes((*item).name);
    }
    mode_tree_build((*data).data);
    mode_tree_draw((*data).data);
    (*(*data).wp).flags |= 0x1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_customize_change_tagged_callback(
    mut c: *mut client,
    mut modedata: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut _done: libc::c_int,
) -> libc::c_int {
    let mut data: *mut window_customize_modedata = modedata as *mut window_customize_modedata;
    if s.is_null() || *s as libc::c_int == '\u{0}' as i32 || (*data).dead != 0 {
        return 0 as libc::c_int;
    }
    if ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<u_char>() as libc::c_ulong > 1 as libc::c_int as libc::c_ulong {
            if 0 != 0 {
                let mut __c: libc::c_int =
                    *s.offset(0 as libc::c_int as isize) as u_char as libc::c_int;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                }
            } else {
                __res = tolower(*s.offset(0 as libc::c_int as isize) as u_char as libc::c_int)
            }
        } else {
            __res = *(*__ctype_tolower_loc())
                .offset(*s.offset(0 as libc::c_int as isize) as u_char as libc::c_int as isize)
        }
        __res
    }) != 'y' as i32
        || *s.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
    {
        return 0 as libc::c_int;
    }
    mode_tree_each_tagged(
        (*data).data,
        Some(
            window_customize_change_each
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                    _: *mut client,
                    _: key_code,
                ) -> (),
        ),
        c,
        0xff000000000 as libc::c_ulonglong,
        0 as libc::c_int,
    );
    mode_tree_build((*data).data);
    mode_tree_draw((*data).data);
    (*(*data).wp).flags |= 0x1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn window_customize_key(
    mut wme: *mut window_mode_entry,
    mut c: *mut client,
    mut _s: *mut session,
    mut _wl: *mut winlink,
    mut key: key_code,
    mut m: *mut mouse_event,
) {
    let mut wp: *mut window_pane = (*wme).wp;
    let mut data: *mut window_customize_modedata = (*wme).data as *mut window_customize_modedata;
    let mut item: *mut window_customize_itemdata = 0 as *mut window_customize_itemdata;
    let mut new_item: *mut window_customize_itemdata = 0 as *mut window_customize_itemdata;
    let mut finished: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut prompt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tagged: u_int = 0;
    item = mode_tree_get_current((*data).data) as *mut window_customize_itemdata;
    finished = mode_tree_key(
        (*data).data,
        c,
        &mut key,
        m,
        0 as *mut u_int,
        0 as *mut u_int,
    );
    new_item = mode_tree_get_current((*data).data) as *mut window_customize_itemdata;
    if item != new_item {
        item = new_item
    }
    match key {
        13 | 115 => {
            if !item.is_null() {
                if (*item).scope as libc::c_uint
                    == WINDOW_CUSTOMIZE_KEY as libc::c_int as libc::c_uint
                {
                    window_customize_set_key(c, data, item);
                } else {
                    window_customize_set_option(c, data, item, 0 as libc::c_int, 1 as libc::c_int);
                    options_push_changes((*item).name);
                }
                mode_tree_build((*data).data);
            }
        }
        119 => {
            if !(item.is_null()
                || (*item).scope as libc::c_uint
                    == WINDOW_CUSTOMIZE_KEY as libc::c_int as libc::c_uint)
            {
                window_customize_set_option(c, data, item, 0 as libc::c_int, 0 as libc::c_int);
                options_push_changes((*item).name);
                mode_tree_build((*data).data);
            }
        }
        83 | 87 => {
            if !(item.is_null()
                || (*item).scope as libc::c_uint
                    == WINDOW_CUSTOMIZE_KEY as libc::c_int as libc::c_uint)
            {
                window_customize_set_option(c, data, item, 1 as libc::c_int, 0 as libc::c_int);
                options_push_changes((*item).name);
                mode_tree_build((*data).data);
            }
        }
        100 => {
            if !(item.is_null() || (*item).idx != -(1 as libc::c_int)) {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"Reset %s to default? \x00" as *const u8 as *const libc::c_char,
                    (*item).name,
                );
                (*data).references += 1;
                (*data).change = WINDOW_CUSTOMIZE_RESET;
                status_prompt_set(
                    c,
                    0 as *mut cmd_find_state,
                    prompt,
                    b"\x00" as *const u8 as *const libc::c_char,
                    Some(
                        window_customize_change_current_callback
                            as unsafe extern "C" fn(
                                _: *mut client,
                                _: *mut libc::c_void,
                                _: *const libc::c_char,
                                _: libc::c_int,
                            ) -> libc::c_int,
                    ),
                    Some(
                        window_customize_free_callback
                            as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
                    ),
                    data as *mut libc::c_void,
                    0x1 as libc::c_int | 0x8 as libc::c_int,
                );
                free(prompt as *mut libc::c_void);
            }
        }
        68 => {
            tagged = mode_tree_count_tagged((*data).data);
            if !(tagged == 0 as libc::c_int as libc::c_uint) {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"Reset %u tagged to default? \x00" as *const u8 as *const libc::c_char,
                    tagged,
                );
                (*data).references += 1;
                (*data).change = WINDOW_CUSTOMIZE_RESET;
                status_prompt_set(
                    c,
                    0 as *mut cmd_find_state,
                    prompt,
                    b"\x00" as *const u8 as *const libc::c_char,
                    Some(
                        window_customize_change_tagged_callback
                            as unsafe extern "C" fn(
                                _: *mut client,
                                _: *mut libc::c_void,
                                _: *const libc::c_char,
                                _: libc::c_int,
                            ) -> libc::c_int,
                    ),
                    Some(
                        window_customize_free_callback
                            as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
                    ),
                    data as *mut libc::c_void,
                    0x1 as libc::c_int | 0x8 as libc::c_int,
                );
                free(prompt as *mut libc::c_void);
            }
        }
        117 => {
            if !item.is_null() {
                idx = (*item).idx;
                if idx != -(1 as libc::c_int) {
                    xasprintf(
                        &mut prompt as *mut *mut libc::c_char,
                        b"Unset %s[%d]? \x00" as *const u8 as *const libc::c_char,
                        (*item).name,
                        idx,
                    );
                } else {
                    xasprintf(
                        &mut prompt as *mut *mut libc::c_char,
                        b"Unset %s? \x00" as *const u8 as *const libc::c_char,
                        (*item).name,
                    );
                }
                (*data).references += 1;
                (*data).change = WINDOW_CUSTOMIZE_UNSET;
                status_prompt_set(
                    c,
                    0 as *mut cmd_find_state,
                    prompt,
                    b"\x00" as *const u8 as *const libc::c_char,
                    Some(
                        window_customize_change_current_callback
                            as unsafe extern "C" fn(
                                _: *mut client,
                                _: *mut libc::c_void,
                                _: *const libc::c_char,
                                _: libc::c_int,
                            ) -> libc::c_int,
                    ),
                    Some(
                        window_customize_free_callback
                            as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
                    ),
                    data as *mut libc::c_void,
                    0x1 as libc::c_int | 0x8 as libc::c_int,
                );
                free(prompt as *mut libc::c_void);
            }
        }
        85 => {
            tagged = mode_tree_count_tagged((*data).data);
            if !(tagged == 0 as libc::c_int as libc::c_uint) {
                xasprintf(
                    &mut prompt as *mut *mut libc::c_char,
                    b"Unset %u tagged? \x00" as *const u8 as *const libc::c_char,
                    tagged,
                );
                (*data).references += 1;
                (*data).change = WINDOW_CUSTOMIZE_UNSET;
                status_prompt_set(
                    c,
                    0 as *mut cmd_find_state,
                    prompt,
                    b"\x00" as *const u8 as *const libc::c_char,
                    Some(
                        window_customize_change_tagged_callback
                            as unsafe extern "C" fn(
                                _: *mut client,
                                _: *mut libc::c_void,
                                _: *const libc::c_char,
                                _: libc::c_int,
                            ) -> libc::c_int,
                    ),
                    Some(
                        window_customize_free_callback
                            as unsafe extern "C" fn(_: *mut libc::c_void) -> (),
                    ),
                    data as *mut libc::c_void,
                    0x1 as libc::c_int | 0x8 as libc::c_int,
                );
                free(prompt as *mut libc::c_void);
            }
        }
        72 => {
            (*data).hide_global = ((*data).hide_global == 0) as libc::c_int;
            mode_tree_build((*data).data);
        }
        _ => {}
    }
    if finished != 0 {
        window_pane_reset_mode(wp);
    } else {
        mode_tree_draw((*data).data);
        (*wp).flags |= 0x1 as libc::c_int
    };
}
