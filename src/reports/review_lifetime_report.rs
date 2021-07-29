use chrono::{DateTime, Utc};
use gitlab::{
    api::{
        self, projects, projects::merge_requests::awards::*, projects::merge_requests::*, Query,
    },
    AwardEmoji, MergeRequest, Project,
};
use gitlab::{Gitlab, UserBasic};
use serde::Deserialize;

use super::report::ReportMaker;
use crate::tools::DateTimeExtension;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ReviewLifetimeReportParams {
    project_name: String,
    datetime_from: DateTime<Utc>,
    datetime_to: DateTime<Utc>,
    assignee: String,
}

impl ReportMaker for ReviewLifetimeReportParams {
    fn make_report(&self, client: &Gitlab) {
        println!("---------- review lifetime report ----------");

        let endpoint = projects::Project::builder()
            .project(self.project_name.as_str())
            .build()
            .unwrap();
        let project: Project = endpoint.query(client).unwrap();

        // println!("{:?}", project.name_with_namespace);

        let merge_requests = self.get_merge_requests(client, &project);
        // for mr in &merge_requests {
        //     println!("{:?}", mr.title);
        // }
        // println!("----------");
        for mr in self.filter_merge_requests(&merge_requests) {
            match mr.assignee {
                Some(user) => {
                    if self.assignee.split(",").any(|x| x.trim() == user.username) {
                        println!("{}\t{}", mr.web_url, mr.title);
                    }
                }
                _ => (),
            }
        }
    }
}

impl ReviewLifetimeReportParams {
    fn get_merge_requests(&self, client: &Gitlab, project: &Project) -> Vec<MergeRequest> {
        let merge_requests_endpoint = MergeRequests::builder()
            .project(project.id.value())
            .order_by(MergeRequestOrderBy::CreatedAt)
            .state(MergeRequestState::Merged)
            .created_after(self.datetime_from)
            .created_before(self.datetime_to)
            .build()
            .unwrap();

        api::paged(merge_requests_endpoint, api::Pagination::All)
            .query(client)
            .unwrap()
    }

    fn filter_merge_requests(&self, merge_requests: &Vec<MergeRequest>) -> Vec<MergeRequest> {
        // merge_requests.sort_by(|x, y| x.title.cmp(&y.title))
        merge_requests
            .iter()
            .filter(|x| {
                x.merged_at
                    .unwrap_or(Utc::now())
                    .duration_since_without_weekend(x.created_at)
                    .num_days()
                    > 1
            })
            .cloned()
            .collect()
    }
}

fn get_merge_requests(client: &Gitlab, project: &Project) -> Vec<MergeRequest> {
    let merge_requests_endpoint = MergeRequests::builder()
        .project(project.id.value())
        .iid(4055)
        .build()
        .unwrap();

    api::paged(merge_requests_endpoint, api::Pagination::All)
        .query(client)
        .unwrap()
}

fn get_merge_requests_awards(client: &Gitlab, project: &Project) -> Vec<AwardEmoji> {
    let merge_requests_endpoint_awards = MergeRequestAwards::builder()
        .project(project.id.value())
        .merge_request(4044)
        .build()
        .unwrap();

    api::paged(merge_requests_endpoint_awards, api::Pagination::All)
        .query(client)
        .unwrap()
}
