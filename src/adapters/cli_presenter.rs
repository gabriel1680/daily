use std::collections::HashMap;

use crate::domain::task::Task;

pub type TaskMap = HashMap<String, Task>;

pub fn tasks_cli_presenter(tasks: Vec<Task>) -> TaskMap {
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
