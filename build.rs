/*
 * File:    build.rs
 * Brief:   Build script for the sv-bindings crate
 *
 * Copyright (C) 2023 John Jekel
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
    //("include/svdpi.h", "svdpi.rs"),
    //("include/vpi_user.h", "vpi_user.rs"),
    //("include/vpi_compatibility.h", "vpi_compatibility.rs"),
    //("include/sv_vpi_user.h", "sv_vpi_user.rs"),
    ("include/wrapper.h", "wrapper.rs")//Switching to a wrapper to make life easier
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
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
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
