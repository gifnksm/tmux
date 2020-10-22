#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

macro_rules! c_try {
    ($e:expr, $ret:expr) => {
        match $e {
            Ok(s) => s,
            Err(_) => return $ret,
        }
    };
}
