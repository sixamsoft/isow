/*
 * This project is licensed under the MPL 2.0 license.
 * See the LICENSE file in the project root for more information.
 */
#![allow(dead_code)]
#![allow(unused_imports)]

use chrono::{Datelike, Local, Utc};
use clap::{crate_authors, crate_description, crate_version, App, Clap};
use isocal::IsoDate;
#[cfg(feature = "updater")] use isow::patcher::Patcher;
use isow::options::{Options, Updater};
// use rbtag::{BuildDateTime, BuildInfo};
use std::error::Error;
use isow::options::Patcher::Update;
use std::convert::TryInto;

fn exit_on_error(err: Box<dyn Error>) {
    eprintln!("[ERROR] {}", err);
    ::std::process::exit(1);
}

/*#[derive(BuildDateTime, BuildInfo)]
struct BuildTag;

/// Remove "-clean" from the commit id
fn normalize_commit_id(id: &str) -> String {
    let clean_stat = "-clean";

    match id.contains(clean_stat) {
        true => {
            id.replace(clean_stat, "")
        },
        false => id.to_string(),
    }
}

fn version() -> String {

    let build_commit = BuildTag {}.get_build_commit();

    format!(
        "{}-{}",
        crate_version!(),
        normalize_commit_id(build_commit)
    )
}*/

fn iso_dt(is_utc: bool, is_day: bool, is_week: bool, is_year: bool, is_time: bool) -> String {
    let dt_utc = Utc::now();
    let dt_local = Local::now();
    let isow_tz = match is_utc {
        true => dt_utc.iso_week(),
        false => dt_local.iso_week(),
    };
    let day = match is_utc {
        true => format!("{:02}", dt_utc.day()),
        false => format!("{:02}", dt_local.day()),
    };
    let week = isow_tz.week_fancy();

    // If the time is in UTC, add a Z directly after the time without a space.
    // Z is the zone designator for the zero UTC offset.
    let time = match is_utc {
        true => format!("T{}Z", dt_utc.time()),
        false => format!("T{}", dt_local.time()),
    };
    let year = isow_tz.year();
    let iso_date = format!("{}-{}", isow_tz.date(), day);

    let output = match (is_year, is_week, is_day, is_time) {
        (true, true, true, true) => format!("{}-{}{}", iso_date, day, time),
        (_, true, true, true) => format!("--{}-{}{}", week, day, time),
        (true, true, _, true) => format!("{}-{}{}", year, week, time),
        (true, _, true, true) => format!("{}-{}{}", year, day, time),
        (true, _, _, true) => format!("{}{}", year, time),
        (_, true, _, true) => format!("{}{}", week, time),
        (_, _, true, true) => format!("{}{}", day, time),
        (true, true, true, _) => format!("{}", iso_date),
        (_, true, true, _) => format!("--{}-{}", week, day),
        (true, true, _, _) => format!("{}-", week),
        (true, _, true, _) => format!("{}-{}", year, day),
        (_, _, _, true) => format!("{}", time),
        (_, _, true, _) => format!("--{}", day),
        (_, true, _, _) => format!("-{}-", week),
        (true, _, _, _) => format!("{}--", year),
        _ => format!("{}{}", iso_date, time),
    };

    output
}

fn main() {
    let opts: Options = Options::parse();

    let is_utc = opts.utc;
    let (is_day, is_week, is_year, is_time) = (
        opts.day,
        opts.week,
        opts.year,
        opts.time,
    );
    let iso_date = iso_dt(is_utc, is_day, is_week, is_year, is_time);

    #[cfg(feature = "updater")]
    match opts.patcher {
        Update(upd) => {
            let patcher = Patcher::default();

            match upd.list {
                true => {
                    if let Err(err) = patcher.release_list() {
                        exit_on_error(err);
                    }
                }
                false => {
                    if let Err(err) = patcher.update() {
                        exit_on_error(err);
                    }
                }
            }
        }
    }

    println!("{}", iso_date);
}
