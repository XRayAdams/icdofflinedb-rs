/*
 * Copyright (c) 2025 Konstantin Adamov
 *  SPDX-License-Identifier: MIT
 *
 *  For full license text, see the LICENSE file in the repo root.
 */

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Rerun this script if assets change
    println!("cargo:rerun-if-changed=assets");

    let out_dir = env::var("OUT_DIR").unwrap();
    // Navigate up to the target directory (target/debug or target/release)
    // Structure is usually: target/<profile>/build/<package>-<hash>/out
    // So we go up 3 levels to get to target/<profile>
    let dest_path = Path::new(&out_dir)
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap();
        
    let src = Path::new("assets");
    let dest = dest_path.join("assets");

    if src.exists() {
        copy_dir_all(src, &dest).expect("Failed to copy assets");
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
