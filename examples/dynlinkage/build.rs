//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   Pierre Avital, <pierre.avital@me.com>
//

fn main() {
    let Ok(dir) = std::env::var("PROFILE") else { return };
    println!(
        "cargo:rustc-link-search=native={}",
        [".", "target", &dir]
            .into_iter()
            .collect::<std::path::PathBuf>()
            .to_str()
            .unwrap()
    );
}
