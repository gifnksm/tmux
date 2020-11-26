use ::libc;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn wctomb(__s: *mut libc::c_char, __wchar: wchar_t) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strlcat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_ulong;
    #[no_mangle]
    fn xsnprintf(_: *mut libc::c_char, _: size_t, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn utf8_from_data(_: *const utf8_data, _: *mut utf8_char) -> utf8_state;
    #[no_mangle]
    fn utf8_append(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_open(_: *mut utf8_data, _: u_char) -> utf8_state;
    #[no_mangle]
    fn utf8_to_data(_: utf8_char, _: *mut utf8_data);
    #[no_mangle]
    fn utf8_fromcstr(_: *const libc::c_char) -> *mut utf8_data;
}
pub type __u_char = libc::c_uchar;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type key_code = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utf8_data {
    pub data: [u_char; 21],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
pub type utf8_char = u_int;
pub type C2RustUnnamed = libc::c_ulong;
pub const KEYC_KP_PERIOD: C2RustUnnamed = 68719476927;
pub const KEYC_KP_ZERO: C2RustUnnamed = 68719476926;
pub const KEYC_KP_ENTER: C2RustUnnamed = 68719476925;
pub const KEYC_KP_THREE: C2RustUnnamed = 68719476924;
pub const KEYC_KP_TWO: C2RustUnnamed = 68719476923;
pub const KEYC_KP_ONE: C2RustUnnamed = 68719476922;
pub const KEYC_KP_SIX: C2RustUnnamed = 68719476921;
pub const KEYC_KP_FIVE: C2RustUnnamed = 68719476920;
pub const KEYC_KP_FOUR: C2RustUnnamed = 68719476919;
pub const KEYC_KP_PLUS: C2RustUnnamed = 68719476918;
pub const KEYC_KP_NINE: C2RustUnnamed = 68719476917;
pub const KEYC_KP_EIGHT: C2RustUnnamed = 68719476916;
pub const KEYC_KP_SEVEN: C2RustUnnamed = 68719476915;
pub const KEYC_KP_MINUS: C2RustUnnamed = 68719476914;
pub const KEYC_KP_STAR: C2RustUnnamed = 68719476913;
pub const KEYC_KP_SLASH: C2RustUnnamed = 68719476912;
pub const KEYC_RIGHT: C2RustUnnamed = 68719476911;
pub const KEYC_LEFT: C2RustUnnamed = 68719476910;
pub const KEYC_DOWN: C2RustUnnamed = 68719476909;
pub const KEYC_UP: C2RustUnnamed = 68719476908;
pub const KEYC_BTAB: C2RustUnnamed = 68719476907;
pub const KEYC_PPAGE: C2RustUnnamed = 68719476906;
pub const KEYC_NPAGE: C2RustUnnamed = 68719476905;
pub const KEYC_END: C2RustUnnamed = 68719476904;
pub const KEYC_HOME: C2RustUnnamed = 68719476903;
pub const KEYC_DC: C2RustUnnamed = 68719476902;
pub const KEYC_IC: C2RustUnnamed = 68719476901;
pub const KEYC_F12: C2RustUnnamed = 68719476900;
pub const KEYC_F11: C2RustUnnamed = 68719476899;
pub const KEYC_F10: C2RustUnnamed = 68719476898;
pub const KEYC_F9: C2RustUnnamed = 68719476897;
pub const KEYC_F8: C2RustUnnamed = 68719476896;
pub const KEYC_F7: C2RustUnnamed = 68719476895;
pub const KEYC_F6: C2RustUnnamed = 68719476894;
pub const KEYC_F5: C2RustUnnamed = 68719476893;
pub const KEYC_F4: C2RustUnnamed = 68719476892;
pub const KEYC_F3: C2RustUnnamed = 68719476891;
pub const KEYC_F2: C2RustUnnamed = 68719476890;
pub const KEYC_F1: C2RustUnnamed = 68719476889;
pub const KEYC_BSPACE: C2RustUnnamed = 68719476888;
pub const KEYC_TRIPLECLICK3_BORDER: C2RustUnnamed = 68719476887;
pub const KEYC_TRIPLECLICK3_STATUS_DEFAULT: C2RustUnnamed = 68719476886;
pub const KEYC_TRIPLECLICK3_STATUS_RIGHT: C2RustUnnamed = 68719476885;
pub const KEYC_TRIPLECLICK3_STATUS_LEFT: C2RustUnnamed = 68719476884;
pub const KEYC_TRIPLECLICK3_STATUS: C2RustUnnamed = 68719476883;
pub const KEYC_TRIPLECLICK3_PANE: C2RustUnnamed = 68719476882;
pub const KEYC_TRIPLECLICK2_BORDER: C2RustUnnamed = 68719476881;
pub const KEYC_TRIPLECLICK2_STATUS_DEFAULT: C2RustUnnamed = 68719476880;
pub const KEYC_TRIPLECLICK2_STATUS_RIGHT: C2RustUnnamed = 68719476879;
pub const KEYC_TRIPLECLICK2_STATUS_LEFT: C2RustUnnamed = 68719476878;
pub const KEYC_TRIPLECLICK2_STATUS: C2RustUnnamed = 68719476877;
pub const KEYC_TRIPLECLICK2_PANE: C2RustUnnamed = 68719476876;
pub const KEYC_TRIPLECLICK1_BORDER: C2RustUnnamed = 68719476875;
pub const KEYC_TRIPLECLICK1_STATUS_DEFAULT: C2RustUnnamed = 68719476874;
pub const KEYC_TRIPLECLICK1_STATUS_RIGHT: C2RustUnnamed = 68719476873;
pub const KEYC_TRIPLECLICK1_STATUS_LEFT: C2RustUnnamed = 68719476872;
pub const KEYC_TRIPLECLICK1_STATUS: C2RustUnnamed = 68719476871;
pub const KEYC_TRIPLECLICK1_PANE: C2RustUnnamed = 68719476870;
pub const KEYC_DOUBLECLICK3_BORDER: C2RustUnnamed = 68719476869;
pub const KEYC_DOUBLECLICK3_STATUS_DEFAULT: C2RustUnnamed = 68719476868;
pub const KEYC_DOUBLECLICK3_STATUS_RIGHT: C2RustUnnamed = 68719476867;
pub const KEYC_DOUBLECLICK3_STATUS_LEFT: C2RustUnnamed = 68719476866;
pub const KEYC_DOUBLECLICK3_STATUS: C2RustUnnamed = 68719476865;
pub const KEYC_DOUBLECLICK3_PANE: C2RustUnnamed = 68719476864;
pub const KEYC_DOUBLECLICK2_BORDER: C2RustUnnamed = 68719476863;
pub const KEYC_DOUBLECLICK2_STATUS_DEFAULT: C2RustUnnamed = 68719476862;
pub const KEYC_DOUBLECLICK2_STATUS_RIGHT: C2RustUnnamed = 68719476861;
pub const KEYC_DOUBLECLICK2_STATUS_LEFT: C2RustUnnamed = 68719476860;
pub const KEYC_DOUBLECLICK2_STATUS: C2RustUnnamed = 68719476859;
pub const KEYC_DOUBLECLICK2_PANE: C2RustUnnamed = 68719476858;
pub const KEYC_DOUBLECLICK1_BORDER: C2RustUnnamed = 68719476857;
pub const KEYC_DOUBLECLICK1_STATUS_DEFAULT: C2RustUnnamed = 68719476856;
pub const KEYC_DOUBLECLICK1_STATUS_RIGHT: C2RustUnnamed = 68719476855;
pub const KEYC_DOUBLECLICK1_STATUS_LEFT: C2RustUnnamed = 68719476854;
pub const KEYC_DOUBLECLICK1_STATUS: C2RustUnnamed = 68719476853;
pub const KEYC_DOUBLECLICK1_PANE: C2RustUnnamed = 68719476852;
pub const KEYC_SECONDCLICK3_BORDER: C2RustUnnamed = 68719476851;
pub const KEYC_SECONDCLICK3_STATUS_DEFAULT: C2RustUnnamed = 68719476850;
pub const KEYC_SECONDCLICK3_STATUS_RIGHT: C2RustUnnamed = 68719476849;
pub const KEYC_SECONDCLICK3_STATUS_LEFT: C2RustUnnamed = 68719476848;
pub const KEYC_SECONDCLICK3_STATUS: C2RustUnnamed = 68719476847;
pub const KEYC_SECONDCLICK3_PANE: C2RustUnnamed = 68719476846;
pub const KEYC_SECONDCLICK2_BORDER: C2RustUnnamed = 68719476845;
pub const KEYC_SECONDCLICK2_STATUS_DEFAULT: C2RustUnnamed = 68719476844;
pub const KEYC_SECONDCLICK2_STATUS_RIGHT: C2RustUnnamed = 68719476843;
pub const KEYC_SECONDCLICK2_STATUS_LEFT: C2RustUnnamed = 68719476842;
pub const KEYC_SECONDCLICK2_STATUS: C2RustUnnamed = 68719476841;
pub const KEYC_SECONDCLICK2_PANE: C2RustUnnamed = 68719476840;
pub const KEYC_SECONDCLICK1_BORDER: C2RustUnnamed = 68719476839;
pub const KEYC_SECONDCLICK1_STATUS_DEFAULT: C2RustUnnamed = 68719476838;
pub const KEYC_SECONDCLICK1_STATUS_RIGHT: C2RustUnnamed = 68719476837;
pub const KEYC_SECONDCLICK1_STATUS_LEFT: C2RustUnnamed = 68719476836;
pub const KEYC_SECONDCLICK1_STATUS: C2RustUnnamed = 68719476835;
pub const KEYC_SECONDCLICK1_PANE: C2RustUnnamed = 68719476834;
pub const KEYC_WHEELDOWN_BORDER: C2RustUnnamed = 68719476833;
pub const KEYC_WHEELDOWN_STATUS_DEFAULT: C2RustUnnamed = 68719476832;
pub const KEYC_WHEELDOWN_STATUS_RIGHT: C2RustUnnamed = 68719476831;
pub const KEYC_WHEELDOWN_STATUS_LEFT: C2RustUnnamed = 68719476830;
pub const KEYC_WHEELDOWN_STATUS: C2RustUnnamed = 68719476829;
pub const KEYC_WHEELDOWN_PANE: C2RustUnnamed = 68719476828;
pub const KEYC_WHEELUP_BORDER: C2RustUnnamed = 68719476827;
pub const KEYC_WHEELUP_STATUS_DEFAULT: C2RustUnnamed = 68719476826;
pub const KEYC_WHEELUP_STATUS_RIGHT: C2RustUnnamed = 68719476825;
pub const KEYC_WHEELUP_STATUS_LEFT: C2RustUnnamed = 68719476824;
pub const KEYC_WHEELUP_STATUS: C2RustUnnamed = 68719476823;
pub const KEYC_WHEELUP_PANE: C2RustUnnamed = 68719476822;
pub const KEYC_MOUSEDRAGEND3_BORDER: C2RustUnnamed = 68719476821;
pub const KEYC_MOUSEDRAGEND3_STATUS_DEFAULT: C2RustUnnamed = 68719476820;
pub const KEYC_MOUSEDRAGEND3_STATUS_RIGHT: C2RustUnnamed = 68719476819;
pub const KEYC_MOUSEDRAGEND3_STATUS_LEFT: C2RustUnnamed = 68719476818;
pub const KEYC_MOUSEDRAGEND3_STATUS: C2RustUnnamed = 68719476817;
pub const KEYC_MOUSEDRAGEND3_PANE: C2RustUnnamed = 68719476816;
pub const KEYC_MOUSEDRAGEND2_BORDER: C2RustUnnamed = 68719476815;
pub const KEYC_MOUSEDRAGEND2_STATUS_DEFAULT: C2RustUnnamed = 68719476814;
pub const KEYC_MOUSEDRAGEND2_STATUS_RIGHT: C2RustUnnamed = 68719476813;
pub const KEYC_MOUSEDRAGEND2_STATUS_LEFT: C2RustUnnamed = 68719476812;
pub const KEYC_MOUSEDRAGEND2_STATUS: C2RustUnnamed = 68719476811;
pub const KEYC_MOUSEDRAGEND2_PANE: C2RustUnnamed = 68719476810;
pub const KEYC_MOUSEDRAGEND1_BORDER: C2RustUnnamed = 68719476809;
pub const KEYC_MOUSEDRAGEND1_STATUS_DEFAULT: C2RustUnnamed = 68719476808;
pub const KEYC_MOUSEDRAGEND1_STATUS_RIGHT: C2RustUnnamed = 68719476807;
pub const KEYC_MOUSEDRAGEND1_STATUS_LEFT: C2RustUnnamed = 68719476806;
pub const KEYC_MOUSEDRAGEND1_STATUS: C2RustUnnamed = 68719476805;
pub const KEYC_MOUSEDRAGEND1_PANE: C2RustUnnamed = 68719476804;
pub const KEYC_MOUSEDRAG3_BORDER: C2RustUnnamed = 68719476803;
pub const KEYC_MOUSEDRAG3_STATUS_DEFAULT: C2RustUnnamed = 68719476802;
pub const KEYC_MOUSEDRAG3_STATUS_RIGHT: C2RustUnnamed = 68719476801;
pub const KEYC_MOUSEDRAG3_STATUS_LEFT: C2RustUnnamed = 68719476800;
pub const KEYC_MOUSEDRAG3_STATUS: C2RustUnnamed = 68719476799;
pub const KEYC_MOUSEDRAG3_PANE: C2RustUnnamed = 68719476798;
pub const KEYC_MOUSEDRAG2_BORDER: C2RustUnnamed = 68719476797;
pub const KEYC_MOUSEDRAG2_STATUS_DEFAULT: C2RustUnnamed = 68719476796;
pub const KEYC_MOUSEDRAG2_STATUS_RIGHT: C2RustUnnamed = 68719476795;
pub const KEYC_MOUSEDRAG2_STATUS_LEFT: C2RustUnnamed = 68719476794;
pub const KEYC_MOUSEDRAG2_STATUS: C2RustUnnamed = 68719476793;
pub const KEYC_MOUSEDRAG2_PANE: C2RustUnnamed = 68719476792;
pub const KEYC_MOUSEDRAG1_BORDER: C2RustUnnamed = 68719476791;
pub const KEYC_MOUSEDRAG1_STATUS_DEFAULT: C2RustUnnamed = 68719476790;
pub const KEYC_MOUSEDRAG1_STATUS_RIGHT: C2RustUnnamed = 68719476789;
pub const KEYC_MOUSEDRAG1_STATUS_LEFT: C2RustUnnamed = 68719476788;
pub const KEYC_MOUSEDRAG1_STATUS: C2RustUnnamed = 68719476787;
pub const KEYC_MOUSEDRAG1_PANE: C2RustUnnamed = 68719476786;
pub const KEYC_MOUSEUP3_BORDER: C2RustUnnamed = 68719476785;
pub const KEYC_MOUSEUP3_STATUS_DEFAULT: C2RustUnnamed = 68719476784;
pub const KEYC_MOUSEUP3_STATUS_RIGHT: C2RustUnnamed = 68719476783;
pub const KEYC_MOUSEUP3_STATUS_LEFT: C2RustUnnamed = 68719476782;
pub const KEYC_MOUSEUP3_STATUS: C2RustUnnamed = 68719476781;
pub const KEYC_MOUSEUP3_PANE: C2RustUnnamed = 68719476780;
pub const KEYC_MOUSEUP2_BORDER: C2RustUnnamed = 68719476779;
pub const KEYC_MOUSEUP2_STATUS_DEFAULT: C2RustUnnamed = 68719476778;
pub const KEYC_MOUSEUP2_STATUS_RIGHT: C2RustUnnamed = 68719476777;
pub const KEYC_MOUSEUP2_STATUS_LEFT: C2RustUnnamed = 68719476776;
pub const KEYC_MOUSEUP2_STATUS: C2RustUnnamed = 68719476775;
pub const KEYC_MOUSEUP2_PANE: C2RustUnnamed = 68719476774;
pub const KEYC_MOUSEUP1_BORDER: C2RustUnnamed = 68719476773;
pub const KEYC_MOUSEUP1_STATUS_DEFAULT: C2RustUnnamed = 68719476772;
pub const KEYC_MOUSEUP1_STATUS_RIGHT: C2RustUnnamed = 68719476771;
pub const KEYC_MOUSEUP1_STATUS_LEFT: C2RustUnnamed = 68719476770;
pub const KEYC_MOUSEUP1_STATUS: C2RustUnnamed = 68719476769;
pub const KEYC_MOUSEUP1_PANE: C2RustUnnamed = 68719476768;
pub const KEYC_MOUSEDOWN3_BORDER: C2RustUnnamed = 68719476767;
pub const KEYC_MOUSEDOWN3_STATUS_DEFAULT: C2RustUnnamed = 68719476766;
pub const KEYC_MOUSEDOWN3_STATUS_RIGHT: C2RustUnnamed = 68719476765;
pub const KEYC_MOUSEDOWN3_STATUS_LEFT: C2RustUnnamed = 68719476764;
pub const KEYC_MOUSEDOWN3_STATUS: C2RustUnnamed = 68719476763;
pub const KEYC_MOUSEDOWN3_PANE: C2RustUnnamed = 68719476762;
pub const KEYC_MOUSEDOWN2_BORDER: C2RustUnnamed = 68719476761;
pub const KEYC_MOUSEDOWN2_STATUS_DEFAULT: C2RustUnnamed = 68719476760;
pub const KEYC_MOUSEDOWN2_STATUS_RIGHT: C2RustUnnamed = 68719476759;
pub const KEYC_MOUSEDOWN2_STATUS_LEFT: C2RustUnnamed = 68719476758;
pub const KEYC_MOUSEDOWN2_STATUS: C2RustUnnamed = 68719476757;
pub const KEYC_MOUSEDOWN2_PANE: C2RustUnnamed = 68719476756;
pub const KEYC_MOUSEDOWN1_BORDER: C2RustUnnamed = 68719476755;
pub const KEYC_MOUSEDOWN1_STATUS_DEFAULT: C2RustUnnamed = 68719476754;
pub const KEYC_MOUSEDOWN1_STATUS_RIGHT: C2RustUnnamed = 68719476753;
pub const KEYC_MOUSEDOWN1_STATUS_LEFT: C2RustUnnamed = 68719476752;
pub const KEYC_MOUSEDOWN1_STATUS: C2RustUnnamed = 68719476751;
pub const KEYC_MOUSEDOWN1_PANE: C2RustUnnamed = 68719476750;
pub const KEYC_MOUSEMOVE_BORDER: C2RustUnnamed = 68719476749;
pub const KEYC_MOUSEMOVE_STATUS_DEFAULT: C2RustUnnamed = 68719476748;
pub const KEYC_MOUSEMOVE_STATUS_RIGHT: C2RustUnnamed = 68719476747;
pub const KEYC_MOUSEMOVE_STATUS_LEFT: C2RustUnnamed = 68719476746;
pub const KEYC_MOUSEMOVE_STATUS: C2RustUnnamed = 68719476745;
pub const KEYC_MOUSEMOVE_PANE: C2RustUnnamed = 68719476744;
pub const KEYC_DOUBLECLICK: C2RustUnnamed = 68719476743;
pub const KEYC_DRAGGING: C2RustUnnamed = 68719476742;
pub const KEYC_MOUSE: C2RustUnnamed = 68719476741;
pub const KEYC_PASTE_END: C2RustUnnamed = 68719476740;
pub const KEYC_PASTE_START: C2RustUnnamed = 68719476739;
pub const KEYC_ANY: C2RustUnnamed = 68719476738;
pub const KEYC_FOCUS_OUT: C2RustUnnamed = 68719476737;
pub const KEYC_FOCUS_IN: C2RustUnnamed = 68719476736;
pub type utf8_state = libc::c_uint;
pub const UTF8_ERROR: utf8_state = 2;
pub const UTF8_DONE: utf8_state = 1;
pub const UTF8_MORE: utf8_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub string: *const libc::c_char,
    pub key: key_code,
}
static mut key_string_table: [C2RustUnnamed_0; 188] = [
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F1\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F1 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F2\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F2 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F3\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F3 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F4\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F4 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F5\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F5 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F6\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F6 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F7\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F7 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F8\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F8 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F9\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F9 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F10\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F10 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F11\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F11 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F12\x00" as *const u8 as *const libc::c_char,
            key: KEYC_F12 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"IC\x00" as *const u8 as *const libc::c_char,
            key: KEYC_IC as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Insert\x00" as *const u8 as *const libc::c_char,
            key: KEYC_IC as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DC\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DC as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Delete\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DC as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Home\x00" as *const u8 as *const libc::c_char,
            key: KEYC_HOME as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"End\x00" as *const u8 as *const libc::c_char,
            key: KEYC_END as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"NPage\x00" as *const u8 as *const libc::c_char,
            key: KEYC_NPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PageDown\x00" as *const u8 as *const libc::c_char,
            key: KEYC_NPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PgDn\x00" as *const u8 as *const libc::c_char,
            key: KEYC_NPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PPage\x00" as *const u8 as *const libc::c_char,
            key: KEYC_PPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PageUp\x00" as *const u8 as *const libc::c_char,
            key: KEYC_PPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PgUp\x00" as *const u8 as *const libc::c_char,
            key: KEYC_PPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Tab\x00" as *const u8 as *const libc::c_char,
            key: '\t' as i32 as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"BTab\x00" as *const u8 as *const libc::c_char,
            key: KEYC_BTAB as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Space\x00" as *const u8 as *const libc::c_char,
            key: ' ' as i32 as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"BSpace\x00" as *const u8 as *const libc::c_char,
            key: KEYC_BSPACE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Enter\x00" as *const u8 as *const libc::c_char,
            key: '\r' as i32 as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Escape\x00" as *const u8 as *const libc::c_char,
            key: '\u{1b}' as i32 as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Up\x00" as *const u8 as *const libc::c_char,
            key: KEYC_UP as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Down\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOWN as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Left\x00" as *const u8 as *const libc::c_char,
            key: KEYC_LEFT as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Right\x00" as *const u8 as *const libc::c_char,
            key: KEYC_RIGHT as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP/\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_SLASH as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP*\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_STAR as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP-\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_MINUS as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP7\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_SEVEN as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP8\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_EIGHT as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP9\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_NINE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP+\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_PLUS as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP4\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_FOUR as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP5\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_FIVE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP6\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_SIX as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP1\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_ONE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP2\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_TWO as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP3\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_THREE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KPEnter\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_ENTER as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP0\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_ZERO as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP.\x00" as *const u8 as *const libc::c_char,
            key: KEYC_KP_PERIOD as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDOWN3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEUP3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAG3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_MOUSEDRAGEND3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpPane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELUP_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpStatus\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELUP_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpStatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELUP_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpStatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELUP_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpStatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELUP_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpBorder\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELUP_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownPane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELDOWN_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownStatus\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELDOWN_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownStatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELDOWN_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownStatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELDOWN_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownStatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELDOWN_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownBorder\x00" as *const u8 as *const libc::c_char,
            key: KEYC_WHEELDOWN_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_SECONDCLICK3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_DOUBLECLICK3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3Pane\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3Status\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3Border\x00" as *const u8 as *const libc::c_char,
            key: KEYC_TRIPLECLICK3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
];
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
/* Find key string in table. */
unsafe extern "C" fn key_string_search_table(mut string: *const libc::c_char) -> key_code {
    let mut i: u_int = 0;
    let mut user: u_int = 0;
    i = 0 as libc::c_int as u_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[C2RustUnnamed_0; 188]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        if strcasecmp(string, key_string_table[i as usize].string) == 0 as libc::c_int {
            return key_string_table[i as usize].key;
        }
        i = i.wrapping_add(1)
    }
    if sscanf(
        string,
        b"User%u\x00" as *const u8 as *const libc::c_char,
        &mut user as *mut u_int,
    ) == 1 as libc::c_int
        && user < 1000 as libc::c_int as libc::c_uint
    {
        return (0x2000000000 as libc::c_ulonglong).wrapping_add(user as libc::c_ulonglong);
    }
    return 0xfe000000000 as libc::c_ulonglong;
}
/* Find modifiers. */
unsafe extern "C" fn key_string_get_modifiers(mut string: *mut *const libc::c_char) -> key_code {
    let mut modifiers: key_code = 0;
    modifiers = 0 as libc::c_int as key_code;
    while *(*string).offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
        && *(*string).offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        match *(*string).offset(0 as libc::c_int as isize) as libc::c_int {
            67 | 99 => modifiers |= 0x200000000000 as libc::c_ulonglong,
            77 | 109 => modifiers |= 0x100000000000 as libc::c_ulonglong,
            83 | 115 => modifiers |= 0x400000000000 as libc::c_ulonglong,
            _ => {
                *string = 0 as *const libc::c_char;
                return 0 as libc::c_int as key_code;
            }
        }
        *string = (*string).offset(2 as libc::c_int as isize)
    }
    return modifiers;
}
/* Lookup a string and convert to a key value. */
#[no_mangle]
pub unsafe extern "C" fn key_string_lookup_string(mut string: *const libc::c_char) -> key_code {
    static mut other: *const libc::c_char =
        b"!#()+,-.0123456789:;<=>\'\r\t\x00" as *const u8 as *const libc::c_char;
    let mut key: key_code = 0;
    let mut modifiers: key_code = 0;
    let mut u: u_int = 0;
    let mut i: u_int = 0;
    let mut ud: utf8_data = utf8_data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut udp: *mut utf8_data = 0 as *mut utf8_data;
    let mut more: utf8_state = UTF8_MORE;
    let mut uc: utf8_char = 0;
    let mut m: [libc::c_char; 17] = [0; 17];
    let mut mlen: libc::c_int = 0;
    /* Is this no key or any key? */
    if strcasecmp(string, b"None\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return 0xff000000000 as libc::c_ulonglong;
    }
    if strcasecmp(string, b"Any\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return KEYC_ANY as libc::c_ulong as key_code;
    }
    /* Is this a hexadecimal value? */
    if *string.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && *string.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
    {
        if sscanf(
            string.offset(2 as libc::c_int as isize),
            b"%x\x00" as *const u8 as *const libc::c_char,
            &mut u as *mut u_int,
        ) != 1 as libc::c_int
        {
            return 0xfe000000000 as libc::c_ulonglong;
        }
        mlen = wctomb(m.as_mut_ptr(), u as wchar_t);
        if mlen <= 0 as libc::c_int || mlen > 16 as libc::c_int {
            return 0xfe000000000 as libc::c_ulonglong;
        }
        m[mlen as usize] = '\u{0}' as i32 as libc::c_char;
        udp = utf8_fromcstr(m.as_mut_ptr());
        if udp.is_null()
            || (*udp.offset(0 as libc::c_int as isize)).size as libc::c_int == 0 as libc::c_int
            || (*udp.offset(1 as libc::c_int as isize)).size as libc::c_int != 0 as libc::c_int
            || utf8_from_data(&mut *udp.offset(0 as libc::c_int as isize), &mut uc) as libc::c_uint
                != UTF8_DONE as libc::c_int as libc::c_uint
        {
            free(udp as *mut libc::c_void);
            return 0xfe000000000 as libc::c_ulonglong;
        }
        free(udp as *mut libc::c_void);
        return uc as key_code;
    }
    /* Check for modifiers. */
    modifiers = 0 as libc::c_int as key_code;
    if *string.offset(0 as libc::c_int as isize) as libc::c_int == '^' as i32
        && *string.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
    {
        modifiers |= 0x200000000000 as libc::c_ulonglong;
        string = string.offset(1)
    }
    modifiers |= key_string_get_modifiers(&mut string);
    if string.is_null()
        || *string.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    {
        return 0xfe000000000 as libc::c_ulonglong;
    }
    /* Is this a standard ASCII key? */
    if *string.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        && *string.offset(0 as libc::c_int as isize) as u_char as libc::c_int <= 127 as libc::c_int
    {
        key = *string.offset(0 as libc::c_int as isize) as u_char as key_code;
        if key < 32 as libc::c_int as libc::c_ulonglong {
            return 0xfe000000000 as libc::c_ulonglong;
        }
    } else {
        /* Try as a UTF-8 key. */
        more = utf8_open(&mut ud, *string as u_char);
        if more as libc::c_uint == UTF8_MORE as libc::c_int as libc::c_uint {
            if strlen(string) != ud.size as libc::c_ulong {
                return 0xfe000000000 as libc::c_ulonglong;
            }
            i = 1 as libc::c_int as u_int;
            while i < ud.size as libc::c_uint {
                more = utf8_append(&mut ud, *string.offset(i as isize) as u_char);
                i = i.wrapping_add(1)
            }
            if more as libc::c_uint != UTF8_DONE as libc::c_int as libc::c_uint {
                return 0xfe000000000 as libc::c_ulonglong;
            }
            if utf8_from_data(&mut ud, &mut uc) as libc::c_uint
                != UTF8_DONE as libc::c_int as libc::c_uint
            {
                return 0xfe000000000 as libc::c_ulonglong;
            }
            return uc as libc::c_ulonglong | modifiers;
        }
        /* Otherwise look the key up in the table. */
        key = key_string_search_table(string);
        if key == 0xfe000000000 as libc::c_ulonglong {
            return 0xfe000000000 as libc::c_ulonglong;
        }
        if !modifiers & 0x100000000000 as libc::c_ulonglong != 0 {
            key &= !(0x8000000000000 as libc::c_ulonglong)
        }
    }
    /* Convert the standard control keys. */
    if key < 0x1000000000 as libc::c_ulonglong
        && modifiers & 0x200000000000 as libc::c_ulonglong != 0
        && strchr(other, key as libc::c_int).is_null()
    {
        if key >= 97 as libc::c_int as libc::c_ulonglong
            && key <= 122 as libc::c_int as libc::c_ulonglong
        {
            key = (key as libc::c_ulonglong).wrapping_sub(96 as libc::c_int as libc::c_ulonglong)
                as key_code as key_code
        } else if key >= 64 as libc::c_int as libc::c_ulonglong
            && key <= 95 as libc::c_int as libc::c_ulonglong
        {
            key = (key as libc::c_ulonglong).wrapping_sub(64 as libc::c_int as libc::c_ulonglong)
                as key_code as key_code
        } else if key == 32 as libc::c_int as libc::c_ulonglong {
            key = 0 as libc::c_int as key_code
        } else if key == 63 as libc::c_int as libc::c_ulonglong {
            key = 127 as libc::c_int as key_code
        } else {
            return 0xfe000000000 as libc::c_ulonglong;
        }
        modifiers &= !(0x200000000000 as libc::c_ulonglong)
    }
    return key | modifiers;
}
/* Convert a key code into string format, with prefix if necessary. */
#[no_mangle]
pub unsafe extern "C" fn key_string_lookup_key(
    mut key: key_code,
    mut with_flags: libc::c_int,
) -> *const libc::c_char {
    let mut current_block: u64;
    let mut saved: key_code = key;
    static mut out: [libc::c_char; 64] = [0; 64];
    let mut tmp: [libc::c_char; 8] = [0; 8];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: u_int = 0;
    let mut ud: utf8_data = utf8_data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut off: size_t = 0;
    *out.as_mut_ptr() = '\u{0}' as i32 as libc::c_char;
    /* Literal keys are themselves. */
    if key & 0x1000000000000 as libc::c_ulonglong != 0 {
        snprintf(
            out.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%c\x00" as *const u8 as *const libc::c_char,
            (key & 0xff as libc::c_int as libc::c_ulonglong) as libc::c_int,
        );
    } else {
        /* Display C-@ as C-Space. */
        if key & (0xfffffffffff as libc::c_ulonglong | 0xf00000000000 as libc::c_ulonglong)
            == 0 as libc::c_int as libc::c_ulonglong
        {
            key = ' ' as i32 as libc::c_ulonglong | 0x200000000000 as libc::c_ulonglong
        }
        /* Fill in the modifiers. */
        if key & 0x200000000000 as libc::c_ulonglong != 0 {
            strlcat(
                out.as_mut_ptr(),
                b"C-\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            );
        }
        if key & 0x100000000000 as libc::c_ulonglong != 0 {
            strlcat(
                out.as_mut_ptr(),
                b"M-\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            );
        }
        if key & 0x400000000000 as libc::c_ulonglong != 0 {
            strlcat(
                out.as_mut_ptr(),
                b"S-\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            );
        }
        key &= 0xfffffffffff as libc::c_ulonglong;
        /* Handle no key. */
        if key == 0xff000000000 as libc::c_ulonglong {
            s = b"None\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == 0xfe000000000 as libc::c_ulonglong {
            s = b"Unknown\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_ANY as libc::c_ulong as libc::c_ulonglong {
            s = b"Any\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_FOCUS_IN as libc::c_ulong as libc::c_ulonglong {
            s = b"FocusIn\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_FOCUS_OUT as libc::c_ulong as libc::c_ulonglong {
            s = b"FocusOut\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_PASTE_START as libc::c_ulong as libc::c_ulonglong {
            s = b"PasteStart\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_PASTE_END as libc::c_ulong as libc::c_ulonglong {
            s = b"PasteEnd\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_MOUSE as libc::c_ulong as libc::c_ulonglong {
            s = b"Mouse\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_DRAGGING as libc::c_ulong as libc::c_ulonglong {
            s = b"Dragging\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_MOUSEMOVE_PANE as libc::c_ulong as libc::c_ulonglong {
            s = b"MouseMovePane\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_MOUSEMOVE_STATUS as libc::c_ulong as libc::c_ulonglong {
            s = b"MouseMoveStatus\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_MOUSEMOVE_STATUS_LEFT as libc::c_ulong as libc::c_ulonglong {
            s = b"MouseMoveStatusLeft\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_MOUSEMOVE_STATUS_RIGHT as libc::c_ulong as libc::c_ulonglong {
            s = b"MouseMoveStatusRight\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == KEYC_MOUSEMOVE_BORDER as libc::c_ulong as libc::c_ulonglong {
            s = b"MouseMoveBorder\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else {
            if key >= 0x2000000000 as libc::c_ulonglong
                && key
                    < (0x2000000000 as libc::c_ulonglong)
                        .wrapping_add(1000 as libc::c_int as libc::c_ulonglong)
            {
                snprintf(
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                    b"User%u\x00" as *const u8 as *const libc::c_char,
                    key.wrapping_sub(0x2000000000 as libc::c_ulonglong) as u_int,
                );
                strlcat(
                    out.as_mut_ptr(),
                    tmp.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                );
            } else {
                /* Handle special keys. */
                /* Try the key against the string table. */
                i = 0 as libc::c_int as u_int;
                while (i as libc::c_ulong)
                    < (::std::mem::size_of::<[C2RustUnnamed_0; 188]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
                {
                    if key == key_string_table[i as usize].key & 0xfffffffffff as libc::c_ulonglong
                    {
                        break;
                    }
                    i = i.wrapping_add(1)
                }
                if i as libc::c_ulong
                    != (::std::mem::size_of::<[C2RustUnnamed_0; 188]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
                {
                    strlcat(
                        out.as_mut_ptr(),
                        key_string_table[i as usize].string,
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    );
                } else if key > 127 as libc::c_int as libc::c_ulonglong
                    && key < 0x1000000000 as libc::c_ulonglong
                {
                    utf8_to_data(key as utf8_char, &mut ud);
                    off = strlen(out.as_mut_ptr());
                    memcpy(
                        out.as_mut_ptr().offset(off as isize) as *mut libc::c_void,
                        ud.data.as_mut_ptr() as *const libc::c_void,
                        ud.size as libc::c_ulong,
                    );
                    out[off.wrapping_add(ud.size as libc::c_ulong) as usize] =
                        '\u{0}' as i32 as libc::c_char
                } else if key > 255 as libc::c_int as libc::c_ulonglong {
                    snprintf(
                        out.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                        b"Invalid#%llx\x00" as *const u8 as *const libc::c_char,
                        saved,
                    );
                } else {
                    /* Is this a UTF-8 key? */
                    /* Invalid keys are errors. */
                    /* Check for standard or control key. */
                    if key <= 32 as libc::c_int as libc::c_ulonglong {
                        if key == 0 as libc::c_int as libc::c_ulonglong
                            || key > 26 as libc::c_int as libc::c_ulonglong
                        {
                            xsnprintf(
                                tmp.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                                b"C-%c\x00" as *const u8 as *const libc::c_char,
                                (64 as libc::c_int as libc::c_ulonglong).wrapping_add(key)
                                    as libc::c_int,
                            );
                        } else {
                            xsnprintf(
                                tmp.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                                b"C-%c\x00" as *const u8 as *const libc::c_char,
                                (96 as libc::c_int as libc::c_ulonglong).wrapping_add(key)
                                    as libc::c_int,
                            );
                        }
                    } else if key >= 32 as libc::c_int as libc::c_ulonglong
                        && key <= 126 as libc::c_int as libc::c_ulonglong
                    {
                        tmp[0 as libc::c_int as usize] = key as libc::c_char;
                        tmp[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
                    } else if key == 127 as libc::c_int as libc::c_ulonglong {
                        xsnprintf(
                            tmp.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                            b"C-?\x00" as *const u8 as *const libc::c_char,
                        );
                    } else if key >= 128 as libc::c_int as libc::c_ulonglong {
                        xsnprintf(
                            tmp.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                            b"\\%llo\x00" as *const u8 as *const libc::c_char,
                            key,
                        );
                    }
                    strlcat(
                        out.as_mut_ptr(),
                        tmp.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    );
                }
            }
            current_block = 13298589467384833406;
        }
        match current_block {
            13298589467384833406 => {}
            _ => {
                strlcat(
                    out.as_mut_ptr(),
                    s,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                );
            }
        }
    }
    if with_flags != 0
        && saved & 0xff000000000000 as libc::c_ulonglong != 0 as libc::c_int as libc::c_ulonglong
    {
        strlcat(
            out.as_mut_ptr(),
            b"[\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        );
        if saved & 0x1000000000000 as libc::c_ulonglong != 0 {
            strlcat(
                out.as_mut_ptr(),
                b"L\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            );
        }
        if saved & 0x2000000000000 as libc::c_ulonglong != 0 {
            strlcat(
                out.as_mut_ptr(),
                b"K\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            );
        }
        if saved & 0x4000000000000 as libc::c_ulonglong != 0 {
            strlcat(
                out.as_mut_ptr(),
                b"C\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            );
        }
        if saved & 0x8000000000000 as libc::c_ulonglong != 0 {
            strlcat(
                out.as_mut_ptr(),
                b"I\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            );
        }
        if saved & 0x10000000000000 as libc::c_ulonglong != 0 {
            strlcat(
                out.as_mut_ptr(),
                b"B\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            );
        }
        strlcat(
            out.as_mut_ptr(),
            b"]\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        );
    }
    return out.as_mut_ptr();
}
