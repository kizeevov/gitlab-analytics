use chrono::{TimeZone, Utc};
use clap::{App, Arg, ArgMatches};
use config::{Config, File};
use gitlab::Gitlab;
use gitlab::{
    api::{self, projects, projects::merge_requests::*, Query},
    MergeRequest, Project,
};
use std::collections::HashMap;

mod settings;

use settings::Settings;

extern crate fstrings;

fn main() {
    let matches = get_args_matches();
    let config_path = matches.value_of("config").unwrap_or("config.json");

    let settings = Settings::new(config_path).unwrap();
    println!("{:?}", settings.gitlab);

    // let mut settings = Config::new();
    // settings.merge(File::with_name("config.json")).unwrap();

    // println!("{}", settings.get::<String>("gitlab-host").unwrap());
    // println!("{}", settings.get_str("gitlab-host").unwrap());

    // println!(
    //     "\n{:?} \n\n-----------",
    //     settings.try_into::<HashMap<String, String>>().unwrap()
    // );

    // let client = Gitlab::new_insecure(GITLAB_HOST_URL, GITLAB_APIKEY).unwrap();
    // let endpoint = projects::Project::builder()
    //     .project(GITLAB_PROJECT_NAME)
    //     .build()
    //     .unwrap();
    // let project: Project = endpoint.query(&client).unwrap();

    // println_f!("{project.name}");

    // let merge_requests_endpoint = MergeRequests::builder()
    //     .project(project.id.value())
    //     .order_by(MergeRequestOrderBy::CreatedAt)
    //     .state(MergeRequestState::Merged)
    //     .created_after(
    //         Utc.datetime_from_str("2021-04-01 12:00:09", "%Y-%m-%d %H:%M:%S")
    //             .unwrap(),
    //     )
    //     .build()
    //     .unwrap();

    // let mr_vector: Vec<MergeRequest> = api::paged(merge_requests_endpoint, api::Pagination::All)
    //     .query(&client)
    //     .unwrap();
    // for mr in mr_vector.iter() {
    //     println!("{:?}", mr.title);
    // }
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
