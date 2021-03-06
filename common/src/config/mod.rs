/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use chrono::{DateTime, TimeZone, Utc};

pub mod project_dirs;

pub const FLATHUB_STATS_BASE_URL: &str = "https://flathub.org/stats";

pub struct Config {
    pub date_format: &'static str,
    pub sqlite_date_format: &'static str,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub threads: usize,
    pub force_refresh: bool,
    pub ignore_404: bool,
    pub show_all: bool,
    pub show_days: i64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            date_format: "%Y/%m/%d",
            sqlite_date_format: "%Y-%m-%d",
            start_date: Utc.ymd(2018, 04, 29).and_hms(0, 0, 0),
            end_date: Utc::now(), // end_date: Utc.ymd(2019, 01, 01).and_hms(0, 0, 0),
            threads: 4,
            force_refresh: false,
            ignore_404: false,
            show_all: false,
            show_days: 180,
        }
    }
}
