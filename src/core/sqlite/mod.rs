/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate rusqlite;

use crate::core::config::project_dirs;
use crate::core::config::Config;
use crate::core::structs::AppId;
use crate::core::utils;
use rusqlite::{named_params, params, Connection};

fn get_connection() -> Connection {
    let config_dir = project_dirs::get_config_dir();
    let db_path = config_dir.join("stats.db");

    Connection::open(&db_path).expect(format!("Can't connect to {}", db_path.display()).as_str())
}

pub fn initialize_db() {
    let mut conn = get_connection();
    let transaction = conn.transaction().unwrap();

    transaction
        .execute(
            "create table if not exists refs (
                appid varchar(128),
                date varchar(10),
                downloads int,
                updates int,
                new_downloads int,
                primary key (appid, date)
            );",
            params![],
        )
        .unwrap();

    transaction
        .execute(
            "create table if not exists dates (
                date varchar(10),
                is_full boolean,
                primary key (date, is_full)
            );",
            params![],
        )
        .unwrap();

    transaction.commit().unwrap();
}

pub fn save_stats(app_ids: Vec<AppId>, is_full: bool) {
    let mut conn = get_connection();
    let transaction = conn.transaction().unwrap();

    for app_id in app_ids {
        let date = app_id
            .date
            .format(Config::default().sqlite_date_format)
            .to_string();

        transaction
            .execute(
                "insert or replace into refs values (?1, ?2, ?3, ?4, ?5);",
                params![
                    app_id.app_id,
                    date,
                    app_id.downloads,
                    app_id.updates,
                    app_id.new_downloads
                ],
            )
            .unwrap();

        transaction
            .execute(
                "insert or replace into dates values (?1, ?2);",
                params![date, is_full],
            )
            .unwrap();
    }

    transaction.commit().unwrap();
}

pub fn get_stats_for_app_id(app_id: String, start_date: String, end_date: String) -> Vec<AppId> {
    let conn = get_connection();

    let mut stats_by_app_id_prep = conn
        .prepare("select * from refs where appid = :appid and date >= date(:start_date) and date <= date(:end_date);")
        .unwrap();

    let mapped_rows_result = stats_by_app_id_prep
        .query_map_named(
            named_params! { ":appid": app_id, ":start_date": start_date, ":end_date": end_date },
            |row| {
                Ok(AppId {
                    app_id: row.get(0).unwrap(),
                    date: utils::parse_datetime_from_string(
                        row.get::<usize, String>(1).unwrap().to_string(),
                        Config::default().sqlite_date_format
                    ),
                    downloads: row.get(2).unwrap(),
                    updates: row.get(3).unwrap(),
                    new_downloads: row.get(4).unwrap(),
                })
            },
        )
        .unwrap();

    let mut result: Vec<AppId> = vec![];
    for mapped_row_result in mapped_rows_result {
        result.push(mapped_row_result.unwrap());
    }

    result
}

pub fn is_stats_exists_by_date(date: String, full: bool) -> bool {
    let conn = get_connection();
    let mut prep = conn
        .prepare("select * from dates where date = :date and is_full = :full limit 1")
        .unwrap();

    let mut result = prep
        .query_named(named_params! { ":date": date, ":full": full })
        .unwrap();

    if let Some(_row) = result.next().unwrap() {
        return true;
    }

    false
}
