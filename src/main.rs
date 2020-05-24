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
                        .help("Set threads for refreshing stats")
                        .short("t")
                        .long("threads")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("force-refresh")
                        .help("Override already downloaded stats")
                        .short("f")
                        .long("force")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("appid")
                .about("Get stats for appid")
                .arg(
                    Arg::with_name("appid")
                        .help("App ID")
                        .index(1)
                        .takes_value(true),
                ),
        )
        .get_matches();

    trace!("Initializing db...");
    core::sqlite::initialize_db();

    trace!("Matching subcommand...");
    match matches.subcommand_name() {
        Some("refresh") => {
            trace!("Matched refresh subcommand");
            let mut config = core::config::Config::default();

            if let Some(ref matches) = matches.subcommand_matches("refresh") {
                if let Some(threads) = matches.value_of("threads") {
                    config.threads = threads.parse::<usize>().unwrap();
                }

                config.force_refresh = matches.is_present("force-refresh")
            }

            println!(
                "Config: threads = {}; force-refresh = {}",
                config.threads, config.force_refresh
            );
            core::refresh_cache(config).await;
        }
        Some("appid") => {
            trace!("Matched appid subcommand");

            if let Some(ref matches) = matches.subcommand_matches("appid") {
                if let Some(app_id) = matches.value_of("appid") {
                    let days = core::sqlite::get_stats_for_app_id(app_id.to_string());
                    for day in days {
                        println!("-----------------");
                        println!(
                            "Date: {}",
                            day.date.format(core::config::Config::default().date_format)
                        );
                        println!("Downloads: {}", day.downloads);
                        println!("New downloads: {}", day.new_downloads);
                        println!("Updates: {}", day.updates);
                    }
                }
            }
        }
        _ => {
            trace!("Matched None o_O");
        }
    }
}
