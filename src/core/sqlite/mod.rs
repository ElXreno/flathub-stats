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

    let conn = Connection::open(&db_path)
        .expect(format!("Can't connect to {}", db_path.display()).as_str());
    conn.pragma_update(None, "journal_mode", &"MEMORY").unwrap();
    conn.pragma_update(None, "synchronous", &"OFF").unwrap();
    conn.pragma_update(None, "cache_size", &100000).unwrap();
    conn
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

pub fn db_is_empty() -> bool {
    let conn = get_connection();

    let mut prep = conn
        .prepare("select * from dates limit 1")
        .unwrap();

    let mut result = prep.query(params![]).unwrap();

    if let Some(_row) = result.next().unwrap() {
        return false;
    }

    true
}

pub fn save_stats(stats: Vec<(Vec<AppId>, bool)>) {
    let mut conn = get_connection();

    let transaction = conn.transaction().unwrap();

    for (app_ids, is_full) in stats {
        let mut app_id_prep = transaction
            .prepare_cached("insert or replace into refs values (?1, ?2, ?3, ?4, ?5);")
            .unwrap();
        let mut date_prep = transaction
            .prepare_cached("insert or replace into dates values (?1, ?2);")
            .unwrap();

        for app_id in app_ids {
            let date = app_id
                .date
                .format(Config::default().sqlite_date_format)
                .to_string();

            app_id_prep
                .execute(params![
                    app_id.app_id,
                    date,
                    app_id.downloads,
                    app_id.updates,
                    app_id.new_downloads
                ])
                .unwrap();

            date_prep.execute(params![date, is_full]).unwrap();
        }
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
                        Config::default().sqlite_date_format,
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
