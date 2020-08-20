/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::core::structs::AppId;
use chrono::prelude::*;
use chrono::Duration;
use config::Config;
use futures::StreamExt;
use reqwest::Client;

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

    let start_download_time = chrono::Local::now();

    let client = Client::new();

    let fetchers = futures::stream::iter((0..days).map(|day| {
        let date = config.start_date + Duration::days(day);
        let force_refresh = config.force_refresh;
        let client = &client;
        let ignore_404 = config.ignore_404;
        async move {
            let result = download_stats(client, date, force_refresh, ignore_404).await;
            result
        }
    }))
    .buffer_unordered(config.threads)
    .collect::<Vec<(Vec<AppId>, bool)>>();

    let result = fetchers.await;

    let end_download_time = chrono::Local::now();

    println!("Saving...");

    sqlite::save_stats(result);

    let end_save_time = chrono::Local::now();

    println!("Update complete!");

    println!("-----Debug stats-----");
    println!(
        "Download time: ~{} ms",
        end_download_time
            .signed_duration_since(start_download_time)
            .num_milliseconds()
    );
    println!(
        "Save time: ~{} ms",
        end_save_time
            .signed_duration_since(end_download_time)
            .num_milliseconds()
    );
}

async fn download_stats(
    client: &Client,
    date: DateTime<Utc>,
    force_refresh: bool,
    ignore_404: bool,
) -> (Vec<AppId>, bool) {
    let fdate = date.format(Config::default().date_format).to_string();
    debug!("Checking for existence: {}...", &fdate);

    if !force_refresh
        && sqlite::is_stats_exists_by_date(
            date.format(Config::default().sqlite_date_format)
                .to_string(),
            true,
        )
    {
        debug!("{}: already downloaded!", date);
        return (vec![], false);
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
        return (vec![], false);
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

    println!("{}: downloaded! Status: {}", date, status);
    (app_ids, is_full)
}
