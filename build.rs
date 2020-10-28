use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=rust-glue.h");

    let bindings = bindgen::Builder::default()
        .header("rust-glue.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function("^(glue|cmdq?|format|window|options|server|args|layout|session)_.*")
        .whitelist_type("^cmd_.*")
        .whitelist_var("^(CMD|WINDOW|WINLINK)_.*|^sessions$")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
