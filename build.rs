use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=tmux.h");

    let bindings = bindgen::Builder::default()
        .header("tmux.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function("^colour_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
