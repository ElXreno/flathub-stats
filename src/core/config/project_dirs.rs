/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use directories::ProjectDirs;
use std::path::Path;
use crate::core::utils;

lazy_static! {
    static ref PROJECT_DIRS: ProjectDirs =
        ProjectDirs::from("com", "elxreno", "flathub-stats").unwrap();
}

pub fn get_cache_dir() -> &'static Path {
    let cache_dir: &Path = PROJECT_DIRS.cache_dir();
    utils::create_dir(cache_dir);
    cache_dir
}

pub fn get_config_dir() -> &'static Path {
    let config_dir: &Path = PROJECT_DIRS.config_dir();
    utils::create_dir(config_dir);
    config_dir
}