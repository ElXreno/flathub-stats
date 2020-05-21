use chrono::prelude::*;
use chrono::Duration;
use futures::StreamExt;
use reqwest::Client;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::prelude::*;

pub mod config;
pub mod structs;

pub async fn find_stats(appid: &str) {
    let file = include_str!("/home/elxreno/.cache/flathub-stats/stats/2020/05/20.json");

    let json: serde_json::Value = serde_json::from_str(file).expect("Can't parse json file!");

    println!("{:#?}", json.get("refs").unwrap().get(appid));
}

pub async fn refresh_cache(config: config::Config) {
    let tmp = config::project_dirs::get_cache_dir();
    println!("Path: {}", tmp.display());

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
}

async fn download_stats(
    client: &Client,
    date: DateTime<Utc>,
    date_format: &str,
    force_refresh: bool,
) {
    debug!("Checking for date: {}...", date.format(date_format));

    let file_path = format!("{}.json", date.format(date_format));
    let destination_file = config::project_dirs::get_cache_dir()
        .join("stats")
        .join(&file_path);

    create_dir(&destination_file.parent().unwrap().to_path_buf());

    if !force_refresh && destination_file.exists() {
        debug!("{}: already downloaded!", date);
        return;
    }

    let mut response = client
        .get(format!("{}/{}", config::FLATHUB_STATS_BASE_URL, file_path).as_str())
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        println!(
            "{}: failed to download! Status: {}",
            date,
            response.status()
        );
        return;
    }

    let mut file = File::create(&destination_file).await.unwrap();

    while let Some(chunk) = response.chunk().await.unwrap() {
        file.write(&chunk).await.unwrap();
    }

    file.flush().await.unwrap();

    println!("{}: downloaded! Status: {}", date, response.status());
}

fn create_dir(path: &PathBuf) {
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
