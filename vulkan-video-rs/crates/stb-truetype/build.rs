use std::path::PathBuf;

fn main() {

    let cargo_manifest_dir: PathBuf = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());

    // generate bindings
    {
        let bindings = bindgen::Builder::default()
            .clang_arg("-fno-inline-functions")
            .header("./src/stb_truetype_prepared.h")
            .generate()
            .expect("Unable to generate bindings");

        // Write the bindings to file
        let bindings_dir = cargo_manifest_dir.join("bindings");
        let bindings_file = bindings_dir.join("bindings.rs");

        if let Err(e) = std::fs::create_dir_all(&bindings_dir) {
            panic!("failed to create directory {}: {}", bindings_dir.display(), e);
        }
        bindings
            .write_to_file(bindings_file)
            .expect("Couldn't write bindings!");
    }

    cc::Build::new()
        .file(cargo_manifest_dir.join("./src/stb_truetype.h"))
        .define("STB_TRUETYPE_IMPLEMENTATION", "1")
        .compile("stb_truetype_rs");

    // // link the libraries specified by pkg-config.
    // for dir in &library.link_paths {
    //     println!("cargo:rustc-link-search=native={}", dir.to_str().unwrap());
    // }
    // for lib in &library.libs {
    //     println!("cargo:rustc-link-lib={}", lib);
    // }
}
