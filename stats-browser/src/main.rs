#![cfg(not(test))]

use clap::parser::ValuesRef;
use clap::Arg;
use clap::ArgAction;
use clap::Command;
use libtw2_stats_browser::tracker_fstd;
use libtw2_stats_browser::tracker_json;
use libtw2_stats_browser::StatsBrowser;
use libtw2_stats_browser::StatsBrowserCb;
use std::collections::HashSet;
use uuid::Uuid;

fn run_browser<T: StatsBrowserCb>(tracker: &mut T, masters: Vec<(String, bool)>) {
    let browser = if masters.is_empty() {
        StatsBrowser::new(tracker)
    } else {
        StatsBrowser::new_without_masters(tracker).map(|mut browser| {
            for (master, nobackcompat) in masters {
                browser.add_master(master, nobackcompat);
            }
            browser
        })
    };
    if let Some(mut browser) = browser {
        browser.run();
    } else {
        panic!("Failed to bind socket.");
    }
}

fn main() {
    libtw2_logger::init();

    let matches = Command::new("stats_browser")
        .version("0.0.1")
        .author("heinrich5991 <heinrich5991@gmail.com>")
        .about("Tracks changes in the Teeworlds server list")
        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .value_name("FORMAT")
            .value_parser(["fstd", "json"])
            .default_value("fstd")
            .help("Output format")
        )
        .arg(Arg::new("filename")
            .long("filename")
            .value_name("FILENAME")
            .default_value("dump.json")
            .help("Output filename (only used for json tracker)")
        )
        .arg(Arg::new("locations")
            .long("locations")
            .value_name("LOCATIONS")
            .help("IP to continent locations database filename (only used for json tracker, libloc format, can be obtained from https://location.ipfire.org/databases/1/location.db.xz)")
        )
        .arg(Arg::new("seed")
            .long("seed")
            .value_name("SEED")
            .value_parser(clap::value_parser!(Uuid))
            .help("UUID seed to use for fake secrets of the reported servers (only used for json tracker, useful if you want to merge output of multiple stats_browser instances)")
        )
        .arg(Arg::new("master")
            .long("master")
            .value_name("MASTER")
            .num_args(1)
            .action(ArgAction::Append)
            .help("Master server to use [default: master1.teeworlds.com to master4.teeworlds.com]")
        )
        .arg(Arg::new("master-nobackcompat")
            .long("master-nobackcompat")
            .value_name("MASTER")
            .num_args(1)
            .action(ArgAction::Append)
            .help("Master server to use, has to support the NOBACKCOMPAT extension to not send servers obtained from the newer HTTPS masters")
        )
        .get_matches();

    fn add_masters(
        masters: &mut Vec<(String, bool)>,
        seen: &mut HashSet<String>,
        args: Option<ValuesRef<'_, String>>,
        nobackcompat: bool,
    ) {
        if let Some(args) = args {
            for arg in args {
                if !seen.insert(arg.to_owned()) {
                    panic!("master {:?} seen twice", arg);
                }
                masters.push((arg.to_owned(), nobackcompat));
            }
        }
    }
    let mut masters = Vec::new();
    {
        let mut seen = HashSet::new();
        add_masters(&mut masters, &mut seen, matches.get_many("master"), false);
        add_masters(
            &mut masters,
            &mut seen,
            matches.get_many("master-nobackcompat"),
            true,
        );
    }

    match matches.get_one::<String>("format").map(String::as_str).unwrap() {
        "fstd" => {
            let mut tracker = tracker_fstd::Tracker::new();
            tracker.start();
            run_browser(&mut tracker, masters);
        }
        "json" => {
            let filename = matches.get_one::<String>("filename").unwrap().to_owned();
            let locations = matches
                .get_one::<String>("locations")
                .map(|location| location.to_owned());
            let seed = matches.get_one::<Uuid>("seed").cloned();
            let mut tracker = tracker_json::Tracker::new(filename, locations, seed);
            tracker.start();
            run_browser(&mut tracker, masters);
        }
        _ => unreachable!(),
    }
}
