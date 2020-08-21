/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use chrono::Duration;
use clap::{App, AppSettings, Arg, SubCommand};

mod core;

#[tokio::main]
async fn main() {
    trace!("Initialize config...");
    let mut config = core::config::Config::default();

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
                        .help("Set threads for refreshing stats")
                        .short("t")
                        .long("threads")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("force-refresh")
                        .help("Override already cached stats")
                        .short("f")
                        .long("force-refresh")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("ignore-404")
                        .help("Disable 404 code ignoring")
                        .short("i")
                        .long("disable-404-ignoring")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("start-date")
                        .help("Start date (default is 2018/04/29)")
                        .short("s")
                        .long("start-date")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("end-date")
                        .help("End date")
                        .short("e")
                        .long("end-date")
                        .takes_value(true),
                ),
        )
        .arg(
            Arg::with_name("app-id")
                .value_name("APP-ID")
                .help("Get stats by application ID")
                .index(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("disable-refresh")
                .help("Don't refresh current stats cache")
                .short("d")
                .long("disable-refresh")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("force-refresh")
                .help("Override already cached stats")
                .short("f")
                .long("force-refresh")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("ignore-404")
                .help("Disable 404 code ignoring")
                .short("i")
                .long("disable-404-ignoring")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("start-date")
                .help("Start date (default is 2018/04/29 if --show-all is present)")
                .short("s")
                .long("start-date")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("end-date")
                .help("End date (default is today)")
                .short("e")
                .long("end-date")
                .takes_value(true),
        )
        .arg(Arg::with_name("show-all")
            .help(
                &format!(
                    "Show stats for all days (by default shows only for {} days)",
                    config.show_days
                )
            )
             .long_help(
                 &format!(
                     "Show stats for all days (by default shows only for {} days if --start-date or --end-date not present)",
                     config.show_days
                 )
             )
            .short("a")
            .long("show-all")
            .takes_value(false),
        )
        .get_matches();

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
                config.ignore_404 = !matches.is_present("ignore-404");

                if let Some(start_date) = matches.value_of("start-date") {
                    config.start_date = core::utils::parse_datetime_from_string(
                        start_date.to_string(),
                        config.date_format,
                    );
                }

                if let Some(end_date) = matches.value_of("end-date") {
                    config.end_date = core::utils::parse_datetime_from_string(
                        end_date.to_string(),
                        config.date_format,
                    );
                }
            }

            refresh(&config).await;
        }
        _ => {
            if let Some(app_id) = matches.value_of("app-id") {
                trace!("Matched {}", app_id);

                config.force_refresh = matches.is_present("force-refresh");
                config.ignore_404 = !matches.is_present("ignore-404");
                config.show_all = matches.is_present("show-all");

                if let Some(start_date) = matches.value_of("start-date") {
                    config.show_all = true;
                    config.start_date = core::utils::parse_datetime_from_string(
                        start_date.to_string(),
                        config.date_format,
                    );
                }

                if let Some(end_date) = matches.value_of("end-date") {
                    config.show_all = true;
                    config.end_date = core::utils::parse_datetime_from_string(
                        end_date.to_string(),
                        config.date_format,
                    );
                }

                if !config.show_all {
                    config.start_date = config.end_date - Duration::days(config.show_days);
                }

                if !matches.is_present("disable-refresh") || core::sqlite::db_is_empty() {
                    refresh(&config).await;
                }

                let days = core::sqlite::get_stats_for_app_id(
                    app_id.to_string(),
                    config
                        .start_date
                        .format(config.sqlite_date_format)
                        .to_string(),
                    config
                        .end_date
                        .format(config.sqlite_date_format)
                        .to_string(),
                );
                for day in &days {
                    println!("-----------------");
                    println!("Date: {}", day.date.format(config.date_format));
                    println!("Downloads: {}", day.downloads);
                    println!("New downloads: {}", day.new_downloads);
                    println!("Updates: {}", day.updates);
                }

                let total_downloads = days.iter().map(|x| x.downloads).fold(0, |acc, x| acc + x);
                let total_new_downloads = days
                    .iter()
                    .map(|x| x.new_downloads)
                    .fold(0, |acc, x| acc + x);
                let total_updates = days.iter().map(|x| x.updates).fold(0, |acc, x| acc + x);

                println!("-----Summary-----");
                println!("Total downloads: {}", total_downloads);
                println!("Total new downloads: {}", total_new_downloads);
                println!("Total updates: {}", total_updates);

            // TODO: Show summary for all days
            } else {
                trace!("Matched nothing");
            }
        }
    }
}

async fn refresh(config: &core::config::Config) {
    println!("Updating stats cache, please wait...");
    trace!("Threads = {}", config.threads);
    trace!("Force refresh: {}", config.force_refresh);
    core::refresh_cache(config).await;
}
