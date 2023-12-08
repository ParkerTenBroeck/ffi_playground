use std::env;
use std::path::PathBuf;

pub fn main(){
    // println!("cargo:rustc-link-search=native=/home/may/Documents/GitHub/cpp_rust/out/program");
   
    println!("cargo:rerun-if-changed=../cpp_side/src/extern_api/export.hpp");
    println!("cargo:rerun-if-changed=../cpp_side/src/extern_api/import.hpp");
    

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../cpp_side/src/extern_api/export.hpp")
        .header("../cpp_side/src/extern_api/import.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .clang_arg("--std=c++20").clang_arg("-xc++")
        .enable_cxx_namespaces()
        
        .allowlist_type("ffi.*")
        .allowlist_function("ffi.*")
        .override_abi(bindgen::Abi::CUnwind, ".*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");


    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // panic!("{:?}", out_path);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}