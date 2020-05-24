/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct AppId {
    pub app_id: String,
    pub date: DateTime<Utc>,
    pub downloads: i64,
    pub updates: i64,
    pub new_downloads: i64,
}
