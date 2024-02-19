mod adapters;
mod get_tasks_input;
mod get_tasks_service;
mod presenters;
mod task;

use std::env;

use get_tasks_service::{make_get_tasks_service, TasksGateway};
use presenters::tasks_cli_presenter;
use task::Task;

use crate::adapters::make_http_tasks_gateway;
use crate::get_tasks_input::GetTasksInput;
use crate::get_tasks_service::GetTaskErr;

#[tokio::main]
async fn main() {
    let input = get_input_args();
    let gateway: TasksGateway = make_http_tasks_gateway();
    let tasks_result: Result<Vec<Task>, GetTaskErr> = make_get_tasks_service(gateway)(input);
    let Ok(tasks) = tasks_result else {
        panic!("Failed to get tasks: {:?}", tasks_result);
    };
    tasks_cli_presenter(tasks);
}

fn get_input_args() -> GetTasksInput {
    let days_before_today: u64 = env::args()
        .nth(1)
        .expect("no sub days specified")
        .parse::<u64>()
        .expect("days must be a integer number");
    GetTasksInput::new(days_before_today)
}
