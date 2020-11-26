use ::libc;
pub type __u_int = libc::c_uint;
pub type u_int = __u_int;
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
pub struct options_name_map {
    pub from: *const libc::c_char,
    pub to: *const libc::c_char,
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
 * This file has a tables with all the server, session and window
 * options. These tables are the master copy of the options with their real
 * (user-visible) types, range limits and default values. At start these are
 * copied into the runtime global options trees (which only has number and
 * string types). These tables are then used to look up the real type when the
 * user sets an option or its value needs to be shown.
 */
/* Choice option type lists. */
static mut options_table_mode_keys_list: [*const libc::c_char; 3] = [
    b"emacs\x00" as *const u8 as *const libc::c_char,
    b"vi\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_clock_mode_style_list: [*const libc::c_char; 3] = [
    b"12\x00" as *const u8 as *const libc::c_char,
    b"24\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_status_list: [*const libc::c_char; 7] = [
    b"off\x00" as *const u8 as *const libc::c_char,
    b"on\x00" as *const u8 as *const libc::c_char,
    b"2\x00" as *const u8 as *const libc::c_char,
    b"3\x00" as *const u8 as *const libc::c_char,
    b"4\x00" as *const u8 as *const libc::c_char,
    b"5\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_status_keys_list: [*const libc::c_char; 3] = [
    b"emacs\x00" as *const u8 as *const libc::c_char,
    b"vi\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_status_justify_list: [*const libc::c_char; 4] = [
    b"left\x00" as *const u8 as *const libc::c_char,
    b"centre\x00" as *const u8 as *const libc::c_char,
    b"right\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_status_position_list: [*const libc::c_char; 3] = [
    b"top\x00" as *const u8 as *const libc::c_char,
    b"bottom\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_bell_action_list: [*const libc::c_char; 5] = [
    b"none\x00" as *const u8 as *const libc::c_char,
    b"any\x00" as *const u8 as *const libc::c_char,
    b"current\x00" as *const u8 as *const libc::c_char,
    b"other\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_visual_bell_list: [*const libc::c_char; 4] = [
    b"off\x00" as *const u8 as *const libc::c_char,
    b"on\x00" as *const u8 as *const libc::c_char,
    b"both\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_pane_status_list: [*const libc::c_char; 4] = [
    b"off\x00" as *const u8 as *const libc::c_char,
    b"top\x00" as *const u8 as *const libc::c_char,
    b"bottom\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_pane_lines_list: [*const libc::c_char; 6] = [
    b"single\x00" as *const u8 as *const libc::c_char,
    b"double\x00" as *const u8 as *const libc::c_char,
    b"heavy\x00" as *const u8 as *const libc::c_char,
    b"simple\x00" as *const u8 as *const libc::c_char,
    b"number\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_set_clipboard_list: [*const libc::c_char; 4] = [
    b"off\x00" as *const u8 as *const libc::c_char,
    b"external\x00" as *const u8 as *const libc::c_char,
    b"on\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_window_size_list: [*const libc::c_char; 5] = [
    b"largest\x00" as *const u8 as *const libc::c_char,
    b"smallest\x00" as *const u8 as *const libc::c_char,
    b"manual\x00" as *const u8 as *const libc::c_char,
    b"latest\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut options_table_status_format_default: [*const libc::c_char; 3] =
    [b"#[align=left range=left #{status-left-style}]#[push-default]#{T;=/#{status-left-length}:status-left}#[pop-default]#[norange default]#[list=on align=#{status-justify}]#[list=left-marker]<#[list=right-marker]>#[list=on]#{W:#[range=window|#{window_index} #{window-status-style}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-format}#[pop-default]#[norange default]#{?window_end_flag,,#{window-status-separator}},#[range=window|#{window_index} list=focus #{?#{!=:#{window-status-current-style},default},#{window-status-current-style},#{window-status-style}}#{?#{&&:#{window_last_flag},#{!=:#{window-status-last-style},default}}, #{window-status-last-style},}#{?#{&&:#{window_bell_flag},#{!=:#{window-status-bell-style},default}}, #{window-status-bell-style},#{?#{&&:#{||:#{window_activity_flag},#{window_silence_flag}},#{!=:#{window-status-activity-style},default}}, #{window-status-activity-style},}}]#[push-default]#{T:window-status-current-format}#[pop-default]#[norange list=on default]#{?window_end_flag,,#{window-status-separator}}}#[nolist align=right range=right #{status-right-style}]#[push-default]#{T;=/#{status-right-length}:status-right}#[pop-default]#[norange default]\x00"
         as *const u8 as *const libc::c_char,
     b"#[align=centre]#{P:#{?pane_active,#[reverse],}#{pane_index}[#{pane_width}x#{pane_height}]#[default] }\x00"
         as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
/* Map of name conversions. */
#[no_mangle]
pub static mut options_other_names: [options_name_map; 4] = [
    {
        let mut init = options_name_map {
            from: b"display-panes-color\x00" as *const u8 as *const libc::c_char,
            to: b"display-panes-colour\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = options_name_map {
            from: b"display-panes-active-color\x00" as *const u8 as *const libc::c_char,
            to: b"display-panes-active-colour\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = options_name_map {
            from: b"clock-mode-color\x00" as *const u8 as *const libc::c_char,
            to: b"clock-mode-colour\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = options_name_map {
            from: 0 as *const libc::c_char,
            to: 0 as *const libc::c_char,
        };
        init
    },
];
/* Top-level options. */
#[no_mangle]
pub static mut options_table: [options_table_entry; 166] = unsafe {
    [
        {
            let mut init = options_table_entry {
                name: b"backspace\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_KEY,
                scope: 0x1 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: '\u{7f}' as i32 as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"The key to send for backspace.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"buffer-limit\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_NUMBER,
                                     scope: 0x1 as libc::c_int,
                                     flags: 0,
                                     minimum: 1 as libc::c_int as u_int,
                                     maximum:
                                         2147483647 as libc::c_int as u_int,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         50 as libc::c_int as
                                             libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"The maximum number of automatic buffers. When this is reached, the oldest buffer is deleted.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"command-alias\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x1 as libc::c_int,
                                     flags: 0x1 as libc::c_int,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str:
                                         b"split-pane=split-window,splitp=split-window,server-info=show-messages -JT,info=show-messages -JT,choose-window=choose-tree -w,choose-session=choose-tree -s\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator:
                                         b",\x00" as *const u8 as
                                             *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Array of command aliases. Each entry is an alias and a command separated by \'=\'.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"copy-command\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x1 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Shell command run when text is copied. If empty, no command is run.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"default-terminal\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x1 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"screen\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Default for the \'TERM\' environment variable.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"editor\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x1 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"/usr/bin/vi\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Editor run to edit files.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"escape-time\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x1 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 500 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Time to wait before assuming a key is Escape.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"exit-empty\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x1 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether the server should exit if there are no sessions.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"exit-unattached\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x1 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether the server should exit if there are no attached clients.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"extended-keys\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x1 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text:
                    b"Whether to request extended key sequences from terminals that support it.\x00"
                        as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"focus-events\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x1 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether to send focus events to applications.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"history-file\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x1 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str:
                                         b"\x00" as *const u8 as
                                             *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Location of the command prompt history file. Empty does not write a history file.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-limit\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x1 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1000 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Maximum number of server messages to keep.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"set-clipboard\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_CHOICE,
                                     scope: 0x1 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         options_table_set_clipboard_list.as_ptr()
                                             as *mut _,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         1 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Whether to attempt to set the system clipboard (\'on\' or \'external\') and whether to allow applications to create paste buffers with an escape sequence (\'on\' only).\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"terminal-overrides\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x1 as libc::c_int,
                flags: 0x1 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"List of terminal capabilities overrides.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"terminal-features\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x1 as libc::c_int,
                flags: 0x1 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"xterm*:clipboard:ccolour:cstyle:focus:title,screen*:title\x00"
                    as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text:
                    b"List of terminal features, used if they cannot be automatically detected.\x00"
                        as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"user-keys\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x1 as libc::c_int,
                                     flags: 0x1 as libc::c_int,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str:
                                         b"\x00" as *const u8 as
                                             *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator:
                                         b",\x00" as *const u8 as
                                             *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"User key assignments. Each sequence in the list is translated into a key: \'User0\', \'User1\' and so on.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"activity-action\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_bell_action_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 3 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Action to take on an activity alert.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"assume-paste-time\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Maximum time between input to assume it pasting rather than typing.\x00"
                    as *const u8 as *const libc::c_char,
                unit: b"milliseconds\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"base-index\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Default index of the first window in each session.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"bell-action\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_bell_action_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Action to take on a bell alert.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"default-command\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Default command to run in new panes. If empty, a shell is started.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"default-shell\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"/bin/sh\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Location of default shell.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"default-size\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"80x24\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: b"[0-9]*x[0-9]*\x00" as *const u8 as *const libc::c_char,
                text: b"Initial size of new sessions.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"destroy-unattached\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether to destroy sessions when they have no attached clients.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"detach-on-destroy\x00" as *const u8
                                             as *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_FLAG,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         1 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Whether to detach when a session is destroyed, or switch the client to another session if any exist.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"display-panes-active-colour\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Colour of the active pane for \'display-panes\'.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"display-panes-colour\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 4 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Colour of not active panes for \'display-panes\'.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"display-panes-time\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 1 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1000 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Time for which \'display-panes\' should show pane numbers.\x00" as *const u8
                    as *const libc::c_char,
                unit: b"milliseconds\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"display-time\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 750 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Time for which status line messages should appear.\x00" as *const u8
                    as *const libc::c_char,
                unit: b"milliseconds\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"history-limit\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_NUMBER,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0,
                                     minimum: 0 as libc::c_int as u_int,
                                     maximum:
                                         2147483647 as libc::c_int as u_int,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         2000 as libc::c_int as
                                             libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Maximum number of lines to keep in the history for each pane. If changed, the new value applies only to new panes.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit:
                                         b"lines\x00" as *const u8 as
                                             *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"key-table\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"root\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Default key table. Key presses are first looked up in this table.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"lock-after-time\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Time after which a client is locked if not used.\x00" as *const u8
                    as *const libc::c_char,
                unit: b"seconds\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"lock-command\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"lock -np\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Shell command to run to lock a client.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"message-command-style\x00" as
                                             *const u8 as *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0x4 as libc::c_int,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str:
                                         b"bg=black,fg=yellow\x00" as
                                             *const u8 as *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator:
                                         b",\x00" as *const u8 as
                                             *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Style of the command prompt when in command mode, if \'mode-keys\' is set to \'vi\'.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"message-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"bg=yellow,fg=black\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of the command prompt.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"mouse\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_FLAG,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         0 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Whether the mouse is recognised and mouse key bindings are executed. Applications inside panes can use the mouse even when \'off\'.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"prefix\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_KEY,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: '\u{2}' as i32 as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"The prefix key.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"prefix2\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_KEY,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0xff000000000 as libc::c_ulonglong as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"A second prefix key.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"renumber-windows\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether windows are automatically renumbered rather than leaving gaps.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"repeat-time\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_NUMBER,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0,
                                     minimum: 0 as libc::c_int as u_int,
                                     maximum: 32767 as libc::c_int as u_int,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         500 as libc::c_int as
                                             libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Time to wait for a key binding to repeat, if it is bound with the \'-r\' flag.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit:
                                         b"milliseconds\x00" as *const u8 as
                                             *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"set-titles\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether to set the terminal title, if supported.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"set-titles-string\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"#S:#I:#W - \"#T\" #{session_alerts}\x00" as *const u8
                    as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Format of the terminal title to set.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"silence-action\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_bell_action_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 3 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Action to take on a silence alert.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_status_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Number of lines in the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"status-bg\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_COLOUR,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         8 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Background colour of the status line. This option is deprecated, use \'status-style\' instead.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"status-fg\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_COLOUR,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         8 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Foreground colour of the status line. This option is deprecated, use \'status-style\' instead.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"status-format\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0x1 as libc::c_int,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         options_table_status_format_default.as_ptr()
                                             as *mut _,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Formats for the status lines. Each array member is the format for one status line. The default status line is made up of several components which may be configured individually with other option such as \'status-left\'.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-interval\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 15 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Number of seconds between status line updates.\x00" as *const u8
                    as *const libc::c_char,
                unit: b"seconds\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-justify\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_status_justify_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Position of the window list in the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-keys\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_status_keys_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Key set to use at the command prompt.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-left\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"[#{session_name}] \x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Contents of the left side of the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-left-length\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: 32767 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 10 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Maximum width of the left side of the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-left-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of the left side of the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-position\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_status_position_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Position of the status line.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"status-right\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str:
                                         b"#{?window_bigger,[#{window_offset_x}#,#{window_offset_y}] ,}\"#{=21:pane_title}\" %H:%M %d-%b-%y\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Contents of the right side of the status line.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-right-length\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: 32767 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 40 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Maximum width of the right side of the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-right-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of the right side of the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"status-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"bg=green,fg=black\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of the status line.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"update-environment\x00" as
                                             *const u8 as *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0x1 as libc::c_int,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str:
                                         b"DISPLAY KRB5CCNAME SSH_ASKPASS SSH_AUTH_SOCK SSH_AGENT_PID SSH_CONNECTION WINDOWID XAUTHORITY\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"List of environment variables to update in the session environment when a client is attached.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"visual-activity\x00" as *const u8
                                             as *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_CHOICE,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         options_table_visual_bell_list.as_ptr()
                                             as *mut _,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         0 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"How activity alerts should be shown: a message (\'on\'), a message and a bell (\'both\') or nothing (\'off\').\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"visual-bell\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_CHOICE,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         options_table_visual_bell_list.as_ptr()
                                             as *mut _,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         0 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"How bell alerts should be shown: a message (\'on\'), a message and a bell (\'both\') or nothing (\'off\').\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"visual-silence\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_CHOICE,
                                     scope: 0x2 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         options_table_visual_bell_list.as_ptr()
                                             as *mut _,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         0 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"How silence alerts should be shown: a message (\'on\'), a message and a bell (\'both\') or nothing (\'off\').\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"word-separators\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x2 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b" \x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Characters considered to separate words.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"aggressive-resize\x00" as *const u8
                                             as *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_FLAG,
                                     scope: 0x4 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         0 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"When \'window-size\' is \'smallest\', whether the maximum size of a window is the smallest attached session where it is the current window (\'on\') or the smallest session it is linked to (\'off\').\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"allow-rename\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_FLAG,
                                     scope:
                                         0x4 as libc::c_int |
                                             0x8 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         0 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Whether applications are allowed to use the escape sequence to rename windows.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"alternate-screen\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x4 as libc::c_int | 0x8 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether applications are allowed to use the alternate screen.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"automatic-rename\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether windows are automatically renamed.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"automatic-rename-format\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str:
                    b"#{?pane_in_mode,[tmux],#{pane_current_command}}#{?pane_dead,[dead],}\x00"
                        as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Format used to automatically rename windows.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"clock-mode-colour\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COLOUR,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 4 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Colour of the clock in clock mode.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"clock-mode-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_clock_mode_style_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Time format of the clock in clock mode.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"copy-mode-match-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"bg=cyan,fg=black\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of search matches in copy mode.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"copy-mode-current-match-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"bg=magenta,fg=black\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of the current search match in copy mode.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"copy-mode-mark-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"bg=red,fg=black\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of the marked line in copy mode.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"main-pane-height\x00" as *const u8
                                             as *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x4 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str:
                                         b"24\x00" as *const u8 as
                                             *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Height of the main pane in the \'main-horizontal\' layout. This may be a percentage, for example \'10%\'.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"main-pane-width\x00" as *const u8
                                             as *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x4 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str:
                                         b"80\x00" as *const u8 as
                                             *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Width of the main pane in the \'main-vertical\' layout. This may be a percentage, for example \'10%\'.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"mode-keys\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_mode_keys_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Key set used in copy mode.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"mode-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"bg=yellow,fg=black\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of indicators and highlighting in modes.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"monitor-activity\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether an alert is triggered by activity.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"monitor-bell\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether an alert is triggered by a bell.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"monitor-silence\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: 2147483647 as libc::c_int as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Time after which an alert is triggered by silence. Zero means no alert.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"other-pane-height\x00" as *const u8
                                             as *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x4 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str:
                                         b"0\x00" as *const u8 as
                                             *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Height of the other panes in the \'main-horizontal\' layout. This may be a percentage, for example \'10%\'.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"other-pane-width\x00" as *const u8
                                             as *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_STRING,
                                     scope: 0x4 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str:
                                         b"0\x00" as *const u8 as
                                             *const libc::c_char,
                                     default_num: 0,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Height of the other panes in the \'main-vertical\' layout. This may be a percentage, for example \'10%\'.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-active-border-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"#{?pane_in_mode,fg=yellow,#{?synchronize-panes,fg=red,fg=green}}\x00"
                    as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of the active pane border.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-base-index\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_NUMBER,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0 as libc::c_int as u_int,
                maximum: (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as u_int,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Index of the first pane in each window.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-border-format\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str:
                    b"#{?pane_active,#[reverse],}#{pane_index}#[default] \"#{pane_title}\"\x00"
                        as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Format of text in the pane status lines.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-border-lines\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_pane_lines_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Type of the pane type lines.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-border-status\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_CHOICE,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: options_table_pane_status_list.as_ptr() as *mut _,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Position of the pane status lines.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-border-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of the pane status lines.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"remain-on-exit\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_FLAG,
                                     scope:
                                         0x4 as libc::c_int |
                                             0x8 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         0 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Whether panes should remain (\'on\') or be automatically killed (\'off\') when the program inside exits.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"synchronize-panes\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether typing should be sent to all panes simultaneously.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-active-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int | 0x8 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Default style of the active pane.\x00" as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"window-size\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_CHOICE,
                                     scope: 0x4 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         options_table_window_size_list.as_ptr()
                                             as *mut _,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         3 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"How window size is calculated. \'latest\' uses the size of the most recently used client, \'largest\' the largest client, \'smallest\' the smallest client and \'manual\' a size set by the \'resize-window\' command.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int | 0x8 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Default style of panes that are not the active pane.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-activity-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"reverse\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of windows in the status line with an activity alert.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-bell-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"reverse\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of windows in the status line with a bell alert.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-current-format\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"#I:#W#{?window_flags,#{window_flags}, }\x00" as *const u8
                    as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Format of the current window in the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-current-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of the current window in the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-format\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"#I:#W#{?window_flags,#{window_flags}, }\x00" as *const u8
                    as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Format of windows in the status line, except the current window.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-last-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Style of the last window in the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-separator\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b" \x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Separator between windows in the status line.\x00" as *const u8
                    as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-status-style\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0x4 as libc::c_int,
                flags: 0x4 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"default\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b",\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text:
                    b"Style of windows in the status line, except the current and last windows.\x00"
                        as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"wrap-search\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_FLAG,
                scope: 0x4 as libc::c_int,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 1 as libc::c_int as libc::c_longlong,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: b"Whether searching in copy mode should wrap at the top or bottom.\x00"
                    as *const u8 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init =
                 options_table_entry{name:
                                         b"xterm-keys\x00" as *const u8 as
                                             *const libc::c_char,
                                     alternative_name:
                                         0 as *const libc::c_char,
                                     type_0: OPTIONS_TABLE_FLAG,
                                     scope: 0x4 as libc::c_int,
                                     flags: 0,
                                     minimum: 0,
                                     maximum: 0,
                                     choices:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     default_str: 0 as *const libc::c_char,
                                     default_num:
                                         1 as libc::c_int as libc::c_longlong,
                                     default_arr:
                                         0 as *const *const libc::c_char as
                                             *mut *const libc::c_char,
                                     separator: 0 as *const libc::c_char,
                                     pattern: 0 as *const libc::c_char,
                                     text:
                                         b"Whether xterm-style function key sequences should be sent. This option is no longer used.\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                     unit: 0 as *const libc::c_char,};
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-bind-key\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-capture-pane\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-copy-mode\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-display-message\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-display-panes\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-kill-pane\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-list-buffers\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-list-clients\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-list-keys\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-list-panes\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-list-sessions\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-list-windows\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-load-buffer\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-lock-server\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-new-session\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-new-window\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-paste-buffer\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-pipe-pane\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-queue\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-refresh-client\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-rename-session\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-rename-window\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-resize-pane\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-resize-window\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-save-buffer\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-select-layout\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-select-pane\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-select-window\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-send-keys\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-set-buffer\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-set-environment\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-set-hook\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-set-option\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-show-environment\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-show-messages\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-show-options\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-split-window\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"after-unbind-key\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"alert-activity\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"alert-bell\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"alert-silence\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"client-attached\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"client-detached\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"client-resized\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"client-session-changed\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-died\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int | 0x8 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-exited\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int | 0x8 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-focus-in\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int | 0x8 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-focus-out\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int | 0x8 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-mode-changed\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int | 0x8 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-set-clipboard\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int | 0x8 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"pane-title-changed\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int | 0x8 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"session-closed\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"session-created\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"session-renamed\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"session-window-changed\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x2 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-layout-changed\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-linked\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-pane-changed\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-renamed\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: b"window-unlinked\x00" as *const u8 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_COMMAND,
                scope: 0x4 as libc::c_int,
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: b"\x00" as *const u8 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: b"\x00" as *const u8 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = options_table_entry {
                name: 0 as *const libc::c_char,
                alternative_name: 0 as *const libc::c_char,
                type_0: OPTIONS_TABLE_STRING,
                scope: 0,
                flags: 0,
                minimum: 0,
                maximum: 0,
                choices: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                default_str: 0 as *const libc::c_char,
                default_num: 0,
                default_arr: 0 as *const *const libc::c_char as *mut *const libc::c_char,
                separator: 0 as *const libc::c_char,
                pattern: 0 as *const libc::c_char,
                text: 0 as *const libc::c_char,
                unit: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
