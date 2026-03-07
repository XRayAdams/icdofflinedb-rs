/*
 * Copyright (c) 2025 Konstantin Adamov
 *  SPDX-License-Identifier: MIT
 *
 *  For full license text, see the LICENSE file in the repo root.
 */
use std::process::Command;

fn main() {
    // Compile GResource
    println!("cargo:rerun-if-changed=resources.gresource.xml");
    println!("cargo:rerun-if-changed=assets");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    
    // Compile the GResource file
    let status = Command::new("glib-compile-resources")
        .arg("resources.gresource.xml")
        .arg("--target")
        .arg(format!("{}/resources.gresource", out_dir))
        .arg("--sourcedir")
        .arg(".")
        .status()
        .expect("Failed to execute glib-compile-resources. Make sure it is installed.");

    if !status.success() {
        panic!("glib-compile-resources failed");
    }
}
