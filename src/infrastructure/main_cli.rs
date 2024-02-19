use crate::adapters::{
    cli_presenter::tasks_cli_presenter, task_http_gateway_adapters::make_http_tasks_gateway,
};
use crate::application::{
    get_tasks_input::GetTasksInput,
    get_tasks_usecase::{make_get_tasks_service, GetTaskErr, TasksGateway},
};
use crate::domain::task::Task;

use std::env;

pub fn run_cli() {
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
