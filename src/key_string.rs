use crate::{
    key_code::code as key_code_code,
    utf8::{utf8_state, Utf8Char, Utf8Data, Utf8State},
};
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
    fn utf8_from_data(_: *const Utf8Data, _: *mut Utf8Char) -> crate::utf8::Utf8State;
    #[no_mangle]
    fn utf8_append(_: *mut Utf8Data, _: u_char) -> crate::utf8::Utf8State;
    #[no_mangle]
    fn utf8_open(_: *mut Utf8Data, _: u_char) -> crate::utf8::Utf8State;
    #[no_mangle]
    fn utf8_to_data(_: Utf8Char, _: *mut Utf8Data);
    #[no_mangle]
    fn utf8_fromcstr(_: *const libc::c_char) -> *mut Utf8Data;
}
pub type __u_char = libc::c_uchar;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type key_code = libc::c_ulonglong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub string: *const libc::c_char,
    pub key: key_code,
}
static mut key_string_table: [C2RustUnnamed_0; 188] = [
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F1\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F1 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F2\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F2 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F3\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F3 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F4\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F4 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F5\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F5 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F6\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F6 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F7\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F7 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F8\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F8 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F9\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F9 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F10\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F10 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F11\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F11 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"F12\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::F12 as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"IC\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::IC as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Insert\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::IC as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DC\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DC as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Delete\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DC as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Home\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::HOME as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"End\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::END as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"NPage\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::NPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PageDown\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::NPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PgDn\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::NPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PPage\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::PPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PageUp\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::PPAGE as libc::c_ulong as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"PgUp\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::PPAGE as libc::c_ulong as libc::c_ulonglong
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
            key: key_code_code::BTAB as libc::c_ulong as key_code,
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
            key: key_code_code::BSPACE as libc::c_ulong as key_code,
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
            key: key_code_code::UP as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Down\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOWN as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Left\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::LEFT as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"Right\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::RIGHT as libc::c_ulong as libc::c_ulonglong
                | 0x4000000000000 as libc::c_ulonglong
                | 0x8000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP/\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_SLASH as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP*\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_STAR as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP-\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_MINUS as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP7\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_SEVEN as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP8\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_EIGHT as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP9\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_NINE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP+\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_PLUS as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP4\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_FOUR as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP5\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_FIVE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP6\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_SIX as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP1\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_ONE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP2\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_TWO as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP3\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_THREE as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KPEnter\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_ENTER as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP0\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_ZERO as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"KP.\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::KP_PERIOD as libc::c_ulong as libc::c_ulonglong
                | 0x2000000000000 as libc::c_ulonglong,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown1Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown2Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDown3Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDOWN3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp1Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp2Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseUp3Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEUP3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag1Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag2Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDrag3Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAG3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd1Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd2Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"MouseDragEnd3Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::MOUSEDRAGEND3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpPane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELUP_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpStatus\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELUP_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpStatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELUP_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpStatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELUP_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpStatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELUP_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelUpBorder\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELUP_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownPane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELDOWN_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownStatus\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELDOWN_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownStatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELDOWN_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownStatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELDOWN_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownStatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELDOWN_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"WheelDownBorder\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::WHEELDOWN_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick1Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick2Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"SecondClick3Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::SECONDCLICK3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick1Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick2Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"DoubleClick3Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::DOUBLECLICK3_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK1_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK1_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK1_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK1_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK1_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick1Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK1_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK2_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK2_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK2_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK2_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK2_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick2Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK2_BORDER as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3Pane\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK3_PANE as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3Status\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK3_STATUS as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3StatusLeft\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK3_STATUS_LEFT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3StatusRight\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK3_STATUS_RIGHT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3StatusDefault\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK3_STATUS_DEFAULT as libc::c_ulong as key_code,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            string: b"TripleClick3Border\x00" as *const u8 as *const libc::c_char,
            key: key_code_code::TRIPLECLICK3_BORDER as libc::c_ulong as key_code,
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
    let mut ud: Utf8Data = Utf8Data {
        data: [0; 21],
        have: 0,
        size: 0,
        width: 0,
    };
    let mut udp: *mut Utf8Data = 0 as *mut Utf8Data;
    let mut more: Utf8State = utf8_state::MORE;
    let mut uc: Utf8Char = 0;
    let mut m: [libc::c_char; 17] = [0; 17];
    let mut mlen: libc::c_int = 0;
    /* Is this no key or any key? */
    if strcasecmp(string, b"None\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return 0xff000000000 as libc::c_ulonglong;
    }
    if strcasecmp(string, b"Any\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return key_code_code::ANY as libc::c_ulong as key_code;
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
                != utf8_state::DONE as libc::c_int as libc::c_uint
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
        if more as libc::c_uint == utf8_state::MORE as libc::c_int as libc::c_uint {
            if strlen(string) != ud.size as libc::c_ulong {
                return 0xfe000000000 as libc::c_ulonglong;
            }
            i = 1 as libc::c_int as u_int;
            while i < ud.size as libc::c_uint {
                more = utf8_append(&mut ud, *string.offset(i as isize) as u_char);
                i = i.wrapping_add(1)
            }
            if more as libc::c_uint != utf8_state::DONE as libc::c_int as libc::c_uint {
                return 0xfe000000000 as libc::c_ulonglong;
            }
            if utf8_from_data(&mut ud, &mut uc) as libc::c_uint
                != utf8_state::DONE as libc::c_int as libc::c_uint
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
    let mut ud: Utf8Data = Utf8Data {
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
        } else if key == key_code_code::ANY as libc::c_ulong as libc::c_ulonglong {
            s = b"Any\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::FOCUS_IN as libc::c_ulong as libc::c_ulonglong {
            s = b"FocusIn\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::FOCUS_OUT as libc::c_ulong as libc::c_ulonglong {
            s = b"FocusOut\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::PASTE_START as libc::c_ulong as libc::c_ulonglong {
            s = b"PasteStart\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::PASTE_END as libc::c_ulong as libc::c_ulonglong {
            s = b"PasteEnd\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::MOUSE as libc::c_ulong as libc::c_ulonglong {
            s = b"Mouse\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::DRAGGING as libc::c_ulong as libc::c_ulonglong {
            s = b"Dragging\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::MOUSEMOVE_PANE as libc::c_ulong as libc::c_ulonglong {
            s = b"MouseMovePane\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::MOUSEMOVE_STATUS as libc::c_ulong as libc::c_ulonglong {
            s = b"MouseMoveStatus\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::MOUSEMOVE_STATUS_LEFT as libc::c_ulong as libc::c_ulonglong
        {
            s = b"MouseMoveStatusLeft\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::MOUSEMOVE_STATUS_RIGHT as libc::c_ulong as libc::c_ulonglong
        {
            s = b"MouseMoveStatusRight\x00" as *const u8 as *const libc::c_char;
            current_block = 12826695638702783628;
        } else if key == key_code_code::MOUSEMOVE_BORDER as libc::c_ulong as libc::c_ulonglong {
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
                    utf8_to_data(key as Utf8Char, &mut ud);
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
