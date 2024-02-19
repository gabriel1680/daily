use std::collections::HashMap;

use crate::domain::task::Task;

pub fn tasks_cli_presenter(tasks: Vec<Task>) {
    let tasks_map = make_task_map_from(tasks);
    print_tasks(tasks_map);
}

type TaskMap = HashMap<String, Task>;

fn make_task_map_from(tasks: Vec<Task>) -> TaskMap {
    let mut task_map: TaskMap = HashMap::new();
    for task in tasks {
        task_map
            .entry(task.description.to_string())
            .and_modify(|task| {
                task.quantity += 1;
                task.total_duration += task.total_duration / 60;
            })
            .or_insert(task);
    }
    task_map
}

fn print_tasks(map: TaskMap) {
    print_table_header();
    for (_, task) in map.iter() {
        print_task(task);
    }
}

fn print_table_header() {
    println!(
        "\n{0: <40} | {1: <10} | {2: <12} | {3: <10}",
        "Description", "Quantity", "Duration (h)", "Tags"
    );
}

fn print_task(task: &Task) {
    println!(
        "{0: <40} | {1: <10} | {2: <12} | {3: <10}",
        task.description, task.quantity, task.total_duration, task.tags[0]
    );
}
