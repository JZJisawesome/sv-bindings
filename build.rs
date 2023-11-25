/*
 * File:    build.rs
 * Brief:   TODO
 *
 * Copyright (C) TODO John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO longer description
 *
 * Partly based on bindgen example
 *
*/

/*!
 * TODO rustdoc for this file here
*/

/* ------------------------------------------------------------------------------------------------
 * Submodules
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "mod ..." and "pub mod ...")

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "use ..." and "extern crate ...")

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

//TODO (also pub(crate) use the_macro statements here too)

/* ------------------------------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------------------------- */

const HEADERS_TO_BINDINGS: &[(&str, &str)] = &[
    #[cfg(feature = "svdpi")]
    ("include/svdpi.h", "svdpi.rs"),
    #[cfg(feature = "vpi_user")]
    ("include/vpi_user.h", "vpi_user.rs"),
    #[cfg(feature = "vpi_compatibility")]
    ("include/vpi_compatibility.h", "vpi_compatibility.rs"),
    #[cfg(feature = "sv_vpi_user")]
    ("include/sv_vpi_user.h", "sv_vpi_user.rs"),
];

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

//TODO includes "type"-defs, structs, enums, unions, etc

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

fn main() {
    for (header, binding) in HEADERS_TO_BINDINGS.iter() {
        setup_bindings(header, binding);
    }
}

fn setup_bindings(c_header: &str, rust_output: &str) {
    //Tell cargo to invalidate the built crate whenever the header changes
    println!("cargo:rerun-if-changed={}", c_header);

    //Generate the bindings from the header
    let bindings = bindgen::Builder::default()
        .header(c_header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    //Write the bindings to the $OUT_DIR
    let out_dir: std::path::PathBuf = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join(rust_output))
        .expect("Couldn't write bindings!");
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO
