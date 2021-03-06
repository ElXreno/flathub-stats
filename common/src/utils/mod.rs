/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use chrono::{DateTime, NaiveDate, Utc};
use std::path::Path;

pub fn create_dir(path: &Path) {
    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(()) => debug!("{} dir created successfully!", &path.display()),
            Err(e) => panic!("Error {}", e),
        }
    } else if !path.is_dir() {
        panic!(
            "{} already exists but is not a directory, exiting...",
            &path.display()
        );
    }
}

pub fn parse_datetime_from_string(date: String, format: &str) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDate::parse_from_str(&date, format)
            .unwrap()
            .and_hms(0, 0, 0),
        Utc,
    )
}
