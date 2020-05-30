/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use clap::{App, AppSettings, Arg, SubCommand};

mod core;

#[tokio::main]
async fn main() {
    trace!("Getting matches...");
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("refresh")
                .about("Refreshes current stats cache")
                .arg(
                    Arg::with_name("threads")
                        .help("Set threads for refreshing stats (currently not fully required due sqlite)")
                        .short("t")
                        .long("threads")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("force-refresh")
                        .help("Override already downloaded stats")
                        .short("f")
                        .long("force")
                        .takes_value(false)
                )
                .arg(
                    Arg::with_name("ignore-404")
                        .help("Ignore 404 status code")
                        .short("i")
                        .long("ignore-404")
                        .takes_value(false)
                )
        )
        .subcommand(
            SubCommand::with_name("appid")
                .about("Get stats for appid")
                .arg(
                    Arg::with_name("appid")
                        .help("App ID")
                        .index(1)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("refresh")
                        .help("Refreshes current stats cache")
                        .short("r")
                        .long("refresh")
                        .takes_value(false)
                )
                .arg(
                Arg::with_name("force-refresh")
                    .help("Override already downloaded stats")
                    .short("f")
                    .long("force")
                    .takes_value(false)
                )
                .arg(
                    Arg::with_name("ignore-404")
                        .help("Ignore 404 status code")
                        .short("i")
                        .long("ignore-404")
                        .takes_value(false)
                )
                .arg(
                    Arg::with_name("start-date")
                        .help("Start date")
                        .short("s")
                        .long("start-date")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("end-date")
                        .help("End date")
                        .short("e")
                        .long("end-date")
                        .takes_value(true)
                )
        )
        .get_matches();

    trace!("Initialize config...");
    let mut config = core::config::Config::default();

    trace!("Initializing db...");
    core::sqlite::initialize_db();

    trace!("Matching subcommand...");
    match matches.subcommand_name() {
        Some("refresh") => {
            trace!("Matched refresh subcommand");

            if let Some(ref matches) = matches.subcommand_matches("refresh") {
                if let Some(threads) = matches.value_of("threads") {
                    config.threads = threads.parse::<usize>().unwrap();
                }

                config.force_refresh = matches.is_present("force-refresh");
                config.ignore_404 = matches.is_present("ignore-404");
            }

            refresh(&config).await;
        }
        Some("appid") => {
            trace!("Matched appid subcommand");

            if let Some(ref matches) = matches.subcommand_matches("appid") {
                config.force_refresh = matches.is_present("force-refresh");
                config.ignore_404 = matches.is_present("ignore-404");

                if let Some(start_date) = matches.value_of("start-date") {
                    config.start_date =
                        core::utils::parse_datetime_from_string(start_date.to_string(), config.date_format);
                }

                if let Some(end_date) = matches.value_of("end-date") {
                    config.end_date = core::utils::parse_datetime_from_string(end_date.to_string(), config.date_format);
                }

                if matches.is_present("refresh") {
                    refresh(&config).await;
                }

                if let Some(app_id) = matches.value_of("appid") {
                    let days = core::sqlite::get_stats_for_app_id(
                        app_id.to_string(),
                        config.start_date.format(config.sqlite_date_format).to_string(),
                        config.end_date.format(config.sqlite_date_format).to_string(),
                    );
                    for day in &days {
                        println!("-----------------");
                        println!("Date: {}", day.date.format(config.date_format));
                        println!("Downloads: {}", day.downloads);
                        println!("New downloads: {}", day.new_downloads);
                        println!("Updates: {}", day.updates);
                    }
                    let total_downloads = days.iter().map(|x| x.downloads).fold(0, |acc, x| acc + x);
                    let total_new_downloads = days.iter().map(|x| x.new_downloads).fold(0, |acc, x| acc + x);
                    let total_updates = days.iter().map(|x| x.updates).fold(0, |acc, x| acc + x);
                    println!("-----Summary-----");
                    println!("Total downloads: {}", total_downloads);
                    println!("Total new downloads: {}", total_new_downloads);
                    println!("Total updates: {}", total_updates);
                }
            }
        }
        _ => {
            trace!("Matched None o_O");
        }
    }
}

async fn refresh(config: &core::config::Config) {
    println!(
        "Config: threads = {}; force-refresh = {}",
        config.threads, config.force_refresh
    );
    core::refresh_cache(config).await;
}
