/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate rusqlite;

use crate::core::config::project_dirs;
use crate::core::config::Config;
use crate::core::structs::AppId;
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{named_params, params, Connection};

fn get_connection() -> Connection {
    let config_dir = project_dirs::get_config_dir();
    let db_path = config_dir.join("stats.db");

    Connection::open(&db_path).expect(format!("Can't connect to {}", db_path.display()).as_str())
}

pub fn initialize_db() {
    let mut conn = get_connection();
    let transaction = conn.transaction().unwrap();

    transaction.execute(
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

    transaction.execute(
        "create table if not exists date_cache (
            date varchar(10) primary key
        );",
        params![]
    ).unwrap();

    transaction.commit().unwrap();
}

pub fn save_stats(app_ids: Vec<AppId>) {
    let mut conn = get_connection();
    let transaction = conn.transaction().unwrap();

    for app_id in app_ids {
        transaction
            .execute(
                "insert or replace into refs values (?1, ?2, ?3, ?4, ?5);",
                params![
                    app_id.app_id,
                    app_id
                        .date
                        .format(Config::default().date_format)
                        .to_string(),
                    app_id.downloads,
                    app_id.updates,
                    app_id.new_downloads
                ],
            )
            .unwrap();
    }

    transaction.commit().unwrap();
}

pub fn get_stats_for_app_id(app_id: String) -> Vec<AppId> {
    let conn = get_connection();

    let mut stats_by_app_id_prep = conn
        .prepare("select * from refs where appid = :appid")
        .unwrap();

    let mapped_rows_result = stats_by_app_id_prep
        .query_map_named(named_params! { ":appid": app_id }, |row| {
            Ok(AppId {
                app_id: row.get(0).unwrap(),
                date: DateTime::<Utc>::from_utc(
                    NaiveDate::parse_from_str(
                        row.get::<usize, String>(1).unwrap().to_string().as_str(),
                        Config::default().date_format,
                    )
                    .unwrap()
                    .and_hms(0, 0, 0),
                    Utc,
                ),
                downloads: row.get(2).unwrap(),
                updates: row.get(3).unwrap(),
                new_downloads: row.get(4).unwrap(),
            })
        })
        .unwrap();

    let mut result: Vec<AppId> = vec![];
    for mapped_row_result in mapped_rows_result {
        result.push(mapped_row_result.unwrap());
    }

    result
}

pub fn is_stats_exists_by_date(date: &str) -> bool {
    let conn = get_connection();
    let mut prep = conn
        .prepare("select * from date_cache where date = :date limit 1")
        .unwrap();

    let mut result = prep.query_named(named_params! { ":date": date })
        .unwrap();

    if let Some(_row) = result.next().unwrap() {
        return true;
    }

    false
}

pub fn update_date_cache_table() {
    let conn = get_connection();
    let mut prep = conn
        .prepare("select distinct date from refs order by date asc;")
        .unwrap();

    let mapped_rows_result = prep.query_map(params![], |row| {
        Ok(Some(row.get::<usize, String>(0).unwrap()))
    }).unwrap();

    let mut conn = get_connection();
    let transaction = conn.transaction().unwrap();

    for mapped_row_result in mapped_rows_result {
        transaction.execute(
            "insert or ignore into date_cache values (?1);",
            params![mapped_row_result.unwrap()]
        ).unwrap();
    }

    transaction.commit().unwrap();
}