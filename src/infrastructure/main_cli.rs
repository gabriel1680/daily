use crate::adapters::cli_presenter::TaskMap;
use crate::adapters::{
    cli_presenter::tasks_cli_presenter, task_http_gateway_adapters::make_http_tasks_gateway,
};
use crate::application::{
    get_tasks_input::GetTasksInput,
    get_tasks_usecase::{get_tasks_usecase, GetTaskErr, TasksGateway},
};
use crate::domain::task::Task;

use std::env;

pub fn run_cli() {
    let input: GetTasksInput = get_input_args();
    let gateway: TasksGateway = make_http_tasks_gateway();
    let tasks_result: Result<Vec<Task>, GetTaskErr> = get_tasks_usecase(gateway)(input);
    let Ok(tasks) = tasks_result else {
        panic!("Failed to get tasks: {:?}", tasks_result);
    };
    let tasks_map = tasks_cli_presenter(tasks);
    print_tasks(&tasks_map);
}

fn get_input_args() -> GetTasksInput {
    let days_before_today: u64 = env::args()
        .nth(1)
        .expect("no sub days specified")
        .parse::<u64>()
        .expect("days must be a integer number");
    GetTasksInput::new(days_before_today)
}

fn print_tasks(map: &TaskMap) {
    print_table_header();
    for (_, task) in map.iter() {
        print_task(task);
    }
}

fn print_table_header() {
    println!(
        "\n{0: <40} | {1: <10} | {2: <14} | {3: <10}",
        "Description", "Quantity", "Duration (min)", "Tags"
    );
}

fn print_task(task: &Task) {
    println!(
        "{0: <40} | {1: <10} | {2: <14} | {3: <10}",
        task.description, task.quantity, task.total_duration, task.tags[0]
    );
}
