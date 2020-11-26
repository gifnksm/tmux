fn main() {
    println!("cargo:rustc-link-lib=event_core");
    println!("cargo:rustc-link-lib=tinfo");
    println!("cargo:rustc-link-lib=resolv");
}
