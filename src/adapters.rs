use futures::executor::block_on;
use reqwest::{self, header::CONTENT_TYPE, Client, Method};
use serde::{Deserialize, Serialize};
use std::env;

use crate::{
    get_tasks_service::{GetTaskErr, TasksGateway},
    task::Task,
};

pub fn make_http_tasks_gateway() -> TasksGateway {
    Box::new(move |start_date: String, end_date: String| {
        block_on(async { http_tasks_gateway(start_date, end_date).await })
    })
}

async fn http_tasks_gateway(start_date: String, end_date: String) -> Result<Vec<Task>, GetTaskErr> {
    let base_url = "https://api.track.toggl.com/api/v9/me/time_entries".to_owned();
    let url = base_url + "?start_date=" + &start_date + "&end_date=" + &end_date;
    let tasks_response_list: Vec<HttpTaskResponse> = make_request(url)
        .send()
        .await?
        .json::<Vec<HttpTaskResponse>>()
        .await?;
    Ok(tasks_response_list
        .iter()
        .map(|http_response_task| to_task(http_response_task))
        .collect())
}

#[derive(Debug, Serialize, Deserialize)]
struct HttpTaskResponse {
    at: String,
    billable: bool,
    description: String,
    duration: u32,
    duronly: bool,
    id: u64,
    pid: u32,
    project_id: u32,
    server_deleted_at: Option<String>,
    start: String,
    stop: String,
    tag_ids: Vec<u64>,
    tags: Vec<String>,
    task_id: Option<u64>,
    uid: u64,
    user_id: u64,
    wid: u64,
    workspace_id: u64,
}

impl From<reqwest::Error> for GetTaskErr {
    fn from(value: reqwest::Error) -> Self {
        GetTaskErr {
            message: value.to_string(),
        }
    }
}

struct UserCredentials {
    username: String,
    password: String,
}

impl UserCredentials {
    fn load_from_env() -> UserCredentials {
        UserCredentials {
            username: env::var("API_USERNAME").expect("username env var is not set"),
            password: env::var("API_PASSWORD").expect("password env var is not set"),
        }
    }
}

fn make_request(url: String) -> reqwest::RequestBuilder {
    let credentials = UserCredentials::load_from_env();
    Client::new()
        .request(Method::GET, url)
        .basic_auth(credentials.username, Some(credentials.password))
        .header(CONTENT_TYPE, "application/json")
}

fn to_task(htr: &HttpTaskResponse) -> Task {
    Task {
        description: htr.description.clone(),
        total_duration: htr.duration / 60,
        tags: htr.tags.clone(),
        quantity: 1,
    }
}
