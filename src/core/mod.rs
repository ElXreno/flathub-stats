/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::core::structs::AppId;
use chrono::prelude::*;
use chrono::Duration;
use futures::StreamExt;
use reqwest::Client;

pub mod config;
pub mod sqlite;
pub mod structs;
pub mod utils;

pub async fn refresh_cache(config: config::Config) {
    let days = config
        .end_date
        .signed_duration_since(config.start_date)
        .num_days();

    println!("Days: {}", days);

    let client = Client::new();

    let fetchers = futures::stream::iter((0..days).map(|day| {
        let date = config.start_date + Duration::days(day);
        let date_format = config.date_format;
        let force_refresh = config.force_refresh;
        let client = &client;
        async move {
            download_stats(client, date, date_format, force_refresh).await;
        }
    }))
    .buffer_unordered(config.threads)
    .collect::<Vec<()>>();
    fetchers.await;

    println!("Download done! Creating cache...");

    sqlite::update_date_cache_table();

    println!("Done!");
}

async fn download_stats(
    client: &Client,
    date: DateTime<Utc>,
    date_format: &str,
    force_refresh: bool,
) {
    let fdate = date.format(date_format).to_string();
    debug!("Checking for existence: {}...", &fdate);

    // TODO: Improve checking, maybe it not fully downloaded for this date
    if !force_refresh && sqlite::is_stats_exists_by_date(fdate.as_str()) {
        debug!("{}: already downloaded!", date);
        return;
    }

    let file_path = format!("{}.json", &fdate);

    let response = client
        .get(format!("{}/{}", config::FLATHUB_STATS_BASE_URL, file_path).as_str())
        .send()
        .await
        .unwrap();

    let status = &response.status();

    if !response.status().is_success() {
        println!("{}: failed to download! Status: {}", date, status);
        return;
    }

    let body = &response.text().await.unwrap();
    let json_response: serde_json::Value = serde_json::from_str(body).unwrap();

    let mut app_ids: Vec<AppId> = vec![];
    let refs = json_response["refs"].as_object().unwrap();

    for refs in refs {
        let app_id = refs.0.clone();
        let mut downloads = 0;
        let mut updates = 0;
        for refs in refs.1.as_object().unwrap() {
            let tmp = refs.1.as_array().unwrap();
            downloads += tmp[0].as_i64().unwrap();
            updates += tmp[1].as_i64().unwrap();
        }
        let new_downloads = downloads - updates;

        app_ids.push(AppId {
            app_id,
            date,
            downloads,
            updates,
            new_downloads,
        })
    }

    sqlite::save_stats(app_ids);

    println!("{}: downloaded! Status: {}", date, status);
}
