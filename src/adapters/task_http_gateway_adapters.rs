use futures::executor::block_on;
use reqwest::{self, header::CONTENT_TYPE, Client, Method};
use serde::{Deserialize, Serialize};
use std::env;

use crate::{
    application::{get_tasks_error::GetTaskErr, tasks_gateway::TasksGateway},
    domain::task::Task,
};

/// HttpTasksGateway factory and sync wrapper
pub fn make_http_tasks_gateway() -> TasksGateway<'static> {
    &|start_date: String, end_date: String| {
        block_on(async { http_tasks_gateway(start_date, end_date).await })
    }
}

/// HttpTasksGateway implementation - retrieves tasks from toggle api endpoint
///
/// # Errors
///
/// This function will return an error if the toggle api call fails for any reason.
async fn http_tasks_gateway(start_date: String, end_date: String) -> Result<Vec<Task>, GetTaskErr> {
    let tasks_response_list = make_request(make_url(start_date, end_date))
        .send()
        .await?
        .json::<Vec<HttpTaskResponse>>()
        .await?;
    Ok(tasks_response_list.iter().map(to_task).collect())
}

fn make_url(start_date: String, end_date: String) -> String {
    let base_url = "https://api.track.toggl.com/api/v9/me/time_entries";
    format!("{base_url}?start_date={start_date}&end_date={end_date}")
}

fn make_request(url: String) -> reqwest::RequestBuilder {
    let (username, password) = load_credentials_from_env();
    Client::new()
        .request(Method::GET, url)
        .basic_auth(username, Some(password))
        .header(CONTENT_TYPE, "application/json")
}

fn load_credentials_from_env() -> (String, String) {
    let username = env::var("API_USERNAME").expect("username env var is not set");
    let password = env::var("API_PASSWORD").expect("password env var is not set");
    (username, password)
}

fn to_task(htr: &HttpTaskResponse) -> Task {
    Task {
        id: htr.id,
        description: htr.description.clone(),
        duration: htr.duration,
        tags: htr.tags.clone(),
    }
}

/// HttpTaskResponse is a list of tasks that toggle api would responde on success
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
