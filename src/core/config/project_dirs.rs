/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use directories::ProjectDirs;
use std::path::Path;

lazy_static! {
    static ref PROJECT_DIRS: ProjectDirs =
        ProjectDirs::from("com", "elxreno", "flathub-stats").unwrap();
}

pub fn get_cache_dir() -> &'static Path {
    PROJECT_DIRS.cache_dir()
}
