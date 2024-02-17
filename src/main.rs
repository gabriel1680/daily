mod get_tasks_input;
mod request;
mod task;

use std::env;

use crate::get_tasks_input::GetTasksInput;
use crate::request::make_request;
use crate::task::{make_task_map_from, print_tasks};

#[tokio::main]
async fn main() {
    let input = get_input_args();
    let response_result = make_request(input).send().await;
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

fn get_input_args() -> GetTasksInput {
    let username: String = env::args().nth(1).expect("no username specified");
    let password: String = env::args().nth(2).expect("no password specified");
    let days_before_today: u64 = env::args()
        .nth(3)
        .expect("no sub days specified")
        .parse::<u64>()
        .expect("days must be a integer number");
    GetTasksInput::new(days_before_today, password, username)
}
