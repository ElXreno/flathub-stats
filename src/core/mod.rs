/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::core::structs::AppId;
use chrono::prelude::*;
use chrono::Duration;
use futures::StreamExt;
use reqwest::Client;
use config::Config;

pub mod config;
pub mod sqlite;
pub mod structs;
pub mod utils;

pub async fn refresh_cache(config: &config::Config) {
    let days = config
        .end_date
        .signed_duration_since(config.start_date)
        .num_days()
        + 1;

    println!("Days: {}", days);

    let client = Client::new();

    let fetchers = futures::stream::iter((0..days).map(|day| {
        let date = config.start_date + Duration::days(day);
        let force_refresh = config.force_refresh;
        let client = &client;
        let ignore_404 = config.ignore_404;
        async move {
            download_stats(client, date, force_refresh, ignore_404).await;
        }
    }))
    .buffer_unordered(config.threads)
    .collect::<Vec<()>>();
    fetchers.await;

    println!("Refreshing done!");
}

async fn download_stats(
    client: &Client,
    date: DateTime<Utc>,
    force_refresh: bool,
    ignore_404: bool,
) {
    let fdate = date.format(Config::default().date_format).to_string();
    debug!("Checking for existence: {}...", &fdate);

    if !force_refresh && sqlite::is_stats_exists_by_date(date.format(Config::default().sqlite_date_format).to_string(), true) {
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
        if response.status() != 404 || !ignore_404 {
            println!("{}: failed to download! Status: {}", date, status);
        }
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

    let is_full = fdate
        != Utc::now()
            .format(config::Config::default().date_format)
            .to_string();

    sqlite::save_stats(app_ids, is_full);

    println!("{}: downloaded! Status: {}", date, status);
}
