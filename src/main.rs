mod user_credentials;
mod utils;
mod request;
mod task;

use crate::user_credentials::make_credentials;
use crate::request::make_request;
use crate::task::{make_task_map_from, print_tasks};

#[tokio::main]
async fn main() {
    let response_result = make_request(make_credentials()).send().await;
    let Ok(response) = response_result else {
        panic!("Reqwest failed: {:?}", response_result);
    };

    let json_result = response.json::<serde_json::Value>().await;
    let Ok(json) = json_result else {
        panic!("Parse json failed: {:?}", json_result);
    };

    let items = match json {
        serde_json::Value::Array(items) => items,
        _ => panic!("Invalid JSON format: {:?}", json),
    };

    print_tasks(make_task_map_from(items));
}

