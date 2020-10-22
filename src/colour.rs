use std::{borrow::Cow, error::Error, fmt};

// TODO: Use enum to represent color types instead of flags
const FLAG_256: u32 = 0x0100_0000;
const FLAG_RGB: u32 = 0x0200_0000;

fn dist_1d(v1: u8, v2: u8) -> i32 {
    let v1 = v1 as i32;
    let v2 = v2 as i32;
    (v1 - v2) * (v1 - v2)
}

fn dist_sq((r1, g1, b1): (u8, u8, u8), (r2, g2, b2): (u8, u8, u8)) -> i32 {
    dist_1d(r1, r2) + dist_1d(g1, g2) + dist_1d(b1, b2)
}

fn to_6cube(v: u8) -> u8 {
    if v < 48 {
        0
    } else if v < 114 {
        1
    } else {
        (v - 35) / 40
    }
}

/// Convert an RGB triplet to the xterm(1) 256 colour palette.
///
/// xterm provides a 6x6x6 colour cube (16 - 231) and 24 greys (232 - 255). We
/// map our RGB colour to the closest in the cube, also work out the closest
/// grey, and use the nearest of the two.
///
/// Note that the xterm has much lower resolution for darker colours (they are
/// not evenly spread out), so our 6 levels are not evenly spread: 0x0, 0x5f
/// (95), 0x87 (135), 0xaf (175), 0xd7 (215) and 0xff (255). Greys are more
/// evenly spread (8, 18, 28 ... 238).
pub(crate) fn find_rgb((r, g, b): (u8, u8, u8)) -> u32 {
    static Q2C: [u8; 6] = [0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

    // Map RGB to 6x6x6 cube.
    let qr = to_6cube(r);
    let cr = Q2C[qr as usize];
    let qg = to_6cube(g);
    let cg = Q2C[qg as usize];
    let qb = to_6cube(b);
    let cb = Q2C[qb as usize];

    // If we have hit the colour exactly, return early.
    if (cr, cg, cb) == (r, g, b) {
        return (16 + (36 * qr) + (6 * qg) + qb) as u32 | FLAG_256;
    }

    // Work out the closest grey (average of RGB).
    let grey_avg = (r + g + b) / 3;
    let grey_idx = if grey_avg > 238 {
        23
    } else {
        (grey_avg - 3) / 10
    };
    let grey = 8 + (10 * grey_idx);

    // Is grey or 6x6x6 colour closest?
    let d = dist_sq((cr, cg, cb), (r, g, b));
    let idx = if dist_sq((grey, grey, grey), (r, g, b)) < d {
        232 + grey_idx
    } else {
        16 + (36 * qr) + (6 * qg) + qb
    };
    idx as u32 | FLAG_256
}

/// Join RGB into a colour.
pub(crate) fn join_rgb((r, g, b): (u8, u8, u8)) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32) | FLAG_RGB
}

/// Split colour into RGB.
pub(crate) fn split_rgb(c: u32) -> (u8, u8, u8) {
    (
        ((c >> 16) & 0xff) as u8,
        ((c >> 8) & 0xff) as u8,
        (c & 0xff) as u8,
    )
}

/// Convert colour to a string.
pub(crate) fn to_str(c: u32) -> Cow<'static, str> {
    if (c & FLAG_RGB) != 0 {
        let (r, g, b) = split_rgb(c);
        return format!("#{:02x}{:02x}{:02x}", r, g, b).into();
    }
    if (c & FLAG_256) != 0 {
        return format!("colour{:}", c & 0xff).into();
    }
    match c {
        0 => "black",
        1 => "red",
        2 => "green",
        3 => "yellow",
        4 => "blue",
        5 => "magenta",
        6 => "cyan",
        7 => "white",
        8 => "default",
        9 => "terminal",
        90 => "blightblack",
        91 => "brightred",
        92 => "brightgreen",
        93 => "brightyellow",
        94 => "brightblue",
        95 => "brightmagenta",
        96 => "brightcyan",
        97 => "brightwhite",
        _ => "invalid",
    }
    .into()
}

#[derive(Debug)]
pub(crate) struct FromStrError {
    kind: FromStrErrorKind,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
enum FromStrErrorKind {
    InvalidRgbColourCode,
    InvalidColourNumber,
    InvalidColourName,
}

impl fmt::Display for FromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use FromStrErrorKind::*;
        let s = match &self.kind {
            InvalidRgbColourCode => "invalid RGB colour code",
            InvalidColourNumber => "invalid colour number",
            InvalidColourName => "invalid colour name",
        };
        fmt::Display::fmt(s, f)
    }
}

impl Error for FromStrError {}

impl From<FromStrErrorKind> for FromStrError {
    fn from(kind: FromStrErrorKind) -> Self {
        Self { kind }
    }
}

/// Convert colour from string.
pub(crate) fn from_str(s: &str) -> Result<u32, FromStrError> {
    use FromStrErrorKind::*;

    if let Some(s) = s.strip_prefix('#') {
        if s.len() == 6 {
            if s.chars().any(|ch| !ch.is_ascii_hexdigit()) {
                return Err(InvalidRgbColourCode.into());
            }
            let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| InvalidRgbColourCode)?;
            let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| InvalidRgbColourCode)?;
            let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| InvalidRgbColourCode)?;
            return Ok(join_rgb((r, g, b)));
        }
    }

    // For simplicity, convert input string to lowercase
    let s = s.to_ascii_lowercase();

    if let Some(s) = s.strip_prefix("colour") {
        let n = u8::from_str_radix(s.trim(), 10).map_err(|_| InvalidColourNumber)?;
        return Ok(n as u32 | FLAG_256);
    }

    let n = match &*s {
        "default" => 8,
        "terminal" => 9,
        "black" | "0" => 0,
        "red" | "1" => 1,
        "green" | "2" => 2,
        "yellow" | "3" => 3,
        "blue" | "4" => 4,
        "magenta" | "5" => 5,
        "cyan" | "6" => 6,
        "white" | "7" => 7,
        "brightblack" | "90" => 90,
        "brightred" | "91" => 91,
        "brightgreen" | "92" => 92,
        "brightyellow" | "93" => 93,
        "brightblue" | "94" => 94,
        "brightmagenta" | "95" => 95,
        "brightcyan" | "96" => 96,
        "brightwhite" | "97" => 97,
        _ => return Err(InvalidColourName.into()),
    };

    Ok(n)
}

/// Convert 256 colour to RGB colour.
pub(crate) fn c256_to_rgb(c: u8) -> u32 {
    const TABLE: [u32; 256] = [
        0x000000, 0x800000, 0x008000, 0x808000, 0x000080, 0x800080, 0x008080, 0xc0c0c0, 0x808080,
        0xff0000, 0x00ff00, 0xffff00, 0x0000ff, 0xff00ff, 0x00ffff, 0xffffff, 0x000000, 0x00005f,
        0x000087, 0x0000af, 0x0000d7, 0x0000ff, 0x005f00, 0x005f5f, 0x005f87, 0x005faf, 0x005fd7,
        0x005fff, 0x008700, 0x00875f, 0x008787, 0x0087af, 0x0087d7, 0x0087ff, 0x00af00, 0x00af5f,
        0x00af87, 0x00afaf, 0x00afd7, 0x00afff, 0x00d700, 0x00d75f, 0x00d787, 0x00d7af, 0x00d7d7,
        0x00d7ff, 0x00ff00, 0x00ff5f, 0x00ff87, 0x00ffaf, 0x00ffd7, 0x00ffff, 0x5f0000, 0x5f005f,
        0x5f0087, 0x5f00af, 0x5f00d7, 0x5f00ff, 0x5f5f00, 0x5f5f5f, 0x5f5f87, 0x5f5faf, 0x5f5fd7,
        0x5f5fff, 0x5f8700, 0x5f875f, 0x5f8787, 0x5f87af, 0x5f87d7, 0x5f87ff, 0x5faf00, 0x5faf5f,
        0x5faf87, 0x5fafaf, 0x5fafd7, 0x5fafff, 0x5fd700, 0x5fd75f, 0x5fd787, 0x5fd7af, 0x5fd7d7,
        0x5fd7ff, 0x5fff00, 0x5fff5f, 0x5fff87, 0x5fffaf, 0x5fffd7, 0x5fffff, 0x870000, 0x87005f,
        0x870087, 0x8700af, 0x8700d7, 0x8700ff, 0x875f00, 0x875f5f, 0x875f87, 0x875faf, 0x875fd7,
        0x875fff, 0x878700, 0x87875f, 0x878787, 0x8787af, 0x8787d7, 0x8787ff, 0x87af00, 0x87af5f,
        0x87af87, 0x87afaf, 0x87afd7, 0x87afff, 0x87d700, 0x87d75f, 0x87d787, 0x87d7af, 0x87d7d7,
        0x87d7ff, 0x87ff00, 0x87ff5f, 0x87ff87, 0x87ffaf, 0x87ffd7, 0x87ffff, 0xaf0000, 0xaf005f,
        0xaf0087, 0xaf00af, 0xaf00d7, 0xaf00ff, 0xaf5f00, 0xaf5f5f, 0xaf5f87, 0xaf5faf, 0xaf5fd7,
        0xaf5fff, 0xaf8700, 0xaf875f, 0xaf8787, 0xaf87af, 0xaf87d7, 0xaf87ff, 0xafaf00, 0xafaf5f,
        0xafaf87, 0xafafaf, 0xafafd7, 0xafafff, 0xafd700, 0xafd75f, 0xafd787, 0xafd7af, 0xafd7d7,
        0xafd7ff, 0xafff00, 0xafff5f, 0xafff87, 0xafffaf, 0xafffd7, 0xafffff, 0xd70000, 0xd7005f,
        0xd70087, 0xd700af, 0xd700d7, 0xd700ff, 0xd75f00, 0xd75f5f, 0xd75f87, 0xd75faf, 0xd75fd7,
        0xd75fff, 0xd78700, 0xd7875f, 0xd78787, 0xd787af, 0xd787d7, 0xd787ff, 0xd7af00, 0xd7af5f,
        0xd7af87, 0xd7afaf, 0xd7afd7, 0xd7afff, 0xd7d700, 0xd7d75f, 0xd7d787, 0xd7d7af, 0xd7d7d7,
        0xd7d7ff, 0xd7ff00, 0xd7ff5f, 0xd7ff87, 0xd7ffaf, 0xd7ffd7, 0xd7ffff, 0xff0000, 0xff005f,
        0xff0087, 0xff00af, 0xff00d7, 0xff00ff, 0xff5f00, 0xff5f5f, 0xff5f87, 0xff5faf, 0xff5fd7,
        0xff5fff, 0xff8700, 0xff875f, 0xff8787, 0xff87af, 0xff87d7, 0xff87ff, 0xffaf00, 0xffaf5f,
        0xffaf87, 0xffafaf, 0xffafd7, 0xffafff, 0xffd700, 0xffd75f, 0xffd787, 0xffd7af, 0xffd7d7,
        0xffd7ff, 0xffff00, 0xffff5f, 0xffff87, 0xffffaf, 0xffffd7, 0xffffff, 0x080808, 0x121212,
        0x1c1c1c, 0x262626, 0x303030, 0x3a3a3a, 0x444444, 0x4e4e4e, 0x585858, 0x626262, 0x6c6c6c,
        0x767676, 0x808080, 0x8a8a8a, 0x949494, 0x9e9e9e, 0xa8a8a8, 0xb2b2b2, 0xbcbcbc, 0xc6c6c6,
        0xd0d0d0, 0xdadada, 0xe4e4e4, 0xeeeeee,
    ];

    TABLE[c as usize] | FLAG_RGB
}

/// Convert 256 colour to 16 colour.
pub(crate) fn c256_to_16(c: u8) -> u8 {
    const TABLE: [u8; 256] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 4, 4, 4, 12, 12, 2, 6, 4, 4, 12,
        12, 2, 2, 6, 4, 12, 12, 2, 2, 2, 6, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14,
        1, 5, 4, 4, 12, 12, 3, 8, 4, 4, 12, 12, 2, 2, 6, 4, 12, 12, 2, 2, 2, 6, 12, 12, 10, 10, 10,
        10, 14, 12, 10, 10, 10, 10, 10, 14, 1, 1, 5, 4, 12, 12, 1, 1, 5, 4, 12, 12, 3, 3, 8, 4, 12,
        12, 2, 2, 2, 6, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10, 10, 10, 10, 14, 1, 1, 1, 5, 12, 12,
        1, 1, 1, 5, 12, 12, 1, 1, 1, 5, 12, 12, 3, 3, 3, 7, 12, 12, 10, 10, 10, 10, 14, 12, 10, 10,
        10, 10, 10, 14, 9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13, 12, 9, 9, 9, 9, 13,
        12, 11, 11, 11, 11, 7, 12, 10, 10, 10, 10, 10, 14, 9, 9, 9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 9,
        9, 9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 9, 9, 9, 9, 9, 13, 11, 11, 11, 11, 11, 15, 0, 0, 0, 0,
        0, 0, 8, 8, 8, 8, 8, 8, 7, 7, 7, 7, 7, 7, 15, 15, 15, 15, 15, 15,
    ];

    TABLE[c as usize]
}

mod ffi {
    use std::{
        ffi::{CStr, CString},
        os::raw::{c_char, c_int, c_uchar},
    };

    #[no_mangle]
    pub extern "C" fn colour_find_rgb(r: u8, g: u8, b: u8) -> c_int {
        super::find_rgb((r, g, b)) as c_int
    }
    #[no_mangle]
    extern "C" fn colour_join_rgb(r: c_uchar, g: c_uchar, b: c_uchar) -> c_int {
        super::join_rgb((r as u8, g as u8, b as u8)) as c_int
    }
    #[no_mangle]
    extern "C" fn colour_split_rgb(c: c_int, r: *mut c_uchar, g: *mut c_uchar, b: *mut c_uchar) {
        let c = super::split_rgb(c as u32);
        unsafe {
            *r = c.0;
            *g = c.1;
            *b = c.2;
        }
    }
    #[no_mangle]
    extern "C" fn colour_tostring(c: c_int) -> *const c_char {
        static mut STR: [c_char; 32] = [0; 32];
        let s = super::to_str(c as u32).into_owned().into_bytes();
        let s = CString::new(s).unwrap();
        unsafe {
            libc::strncpy(STR.as_mut_ptr(), s.as_ptr(), STR.len());
            *STR.last_mut().unwrap() = 0;
            STR.as_ptr()
        }
    }
    #[no_mangle]
    extern "C" fn colour_fromstring(s: *const c_char) -> c_int {
        let s = unsafe { CStr::from_ptr(s) };
        let s = c_try!(s.to_str(), -1);
        c_try!(super::from_str(s), -1) as c_int
    }
    #[no_mangle]
    extern "C" fn colour_256toRGB(c: c_int) -> c_int {
        super::c256_to_rgb((c & 0xff) as u8) as c_int
    }
    #[no_mangle]
    extern "C" fn colour_256to16(c: c_int) -> c_int {
        super::c256_to_16((c & 0xff) as u8) as c_int
    }
}
