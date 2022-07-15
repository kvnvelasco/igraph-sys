use std::error::Error;
use std::fmt::format;
use std::path::PathBuf;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let make = cmake::Config::new("./igraph")
        .env("IGRAPH_STATIC", "1")
        .build();

    println!("cargo:rustc-link-search=native={}", make.display());
    println!("cargo:rustc-link-search=native={}/lib64", make.display());
    println!("cargo:rustc-link-lib=igraph");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=igraph-sys.h");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("igraph-sys.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("igraph_t")
        .allowlist_type("igraph_vector_t")
        .allowlist_type("igraph_real_t")
        .allowlist_type("igraph_i_directed_t")
        .allowlist_function("igraph_vector_view")
        .allowlist_function("igraph_create")
        .allowlist_function("igraph_destroy")
        .allowlist_function("igraph_vector_init")
        .allowlist_function("igraph_vector_max")
        .allowlist_function("igraph_vector_which_max")
        .allowlist_function("igraph_degree")
        .allowlist_function("igraph_closeness")
        .allowlist_function("igraph_vector_destroy")
        .allowlist_function("igraph_graph_destroy")
        .allowlist_function("igraph_vss_all")
        .rustified_enum("igraph_neimode_t")
        .rustified_enum("igraph_i_directed_t")
        .clang_arg(format!("-I{}/include/igraph", make.display()))
        // Finish the builder and generate the bindings.
        .generate()?;

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("igraph.rs"))?;
    Ok(())
}
