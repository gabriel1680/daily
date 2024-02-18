mod get_tasks_input;
mod request;
mod task;

use std::env;

use crate::get_tasks_input::GetTasksInput;
use crate::request::make_request;
use crate::task::{make_task_map_from, print_tasks, HttpTaskResponse};

#[tokio::main]
async fn main() {
    let input = get_input_args();
    let response_result = make_request(input).send().await;
    let Ok(response) = response_result else {
        panic!("Reqwest failed: {:?}", response_result);
    };

    let tasks_result = response.json::<Vec<HttpTaskResponse>>().await;
    let Ok(tasks) = tasks_result else {
        panic!("Parse json failed: {:?}", tasks_result);
    };

    print_tasks(make_task_map_from(tasks));
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
