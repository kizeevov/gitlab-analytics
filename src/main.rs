use clap::{App, Arg, ArgMatches};
use gitlab::Gitlab;

mod configuration;
mod reports;
mod tools;

use configuration::Config;

use crate::reports::report::{self};

fn main() {
    let matches = get_args_matches();
    let config_path = matches.value_of("config").unwrap_or("config.json");

    let settings = Config::new(config_path).unwrap();

    let client = Gitlab::new_insecure(settings.gitlab.host, settings.gitlab.apikey).unwrap();

    for report_param in settings.reports {
        report::make(report_param, &client);
    }
}

fn get_args_matches<'a>() -> ArgMatches<'a> {
    App::new("GitLab Analytics Program")
        .version("1.0")
        .about("A program for generating reports for collecting metrics")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches()
}
