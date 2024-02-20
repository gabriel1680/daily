use std::collections::HashMap;

use crate::domain::task::Task;

#[derive(Debug)]
pub struct TaskSummary {
    pub description: String,
    pub total_duration: u32,
    pub tags: Vec<String>,
    pub quantity: u32,
}

pub type TaskMap = HashMap<String, TaskSummary>;

pub fn tasks_summary_presenter(tasks: &Vec<Task>) -> TaskMap {
    let mut task_map: TaskMap = HashMap::new();
    for task in tasks {
        task_map
            .entry(task.description.to_string())
            .and_modify(|task| {
                task.quantity += 1;
                task.total_duration += task.total_duration;
            })
            .or_insert(to_output(&task));
    }
    task_map
}

fn to_output(task: &Task) -> TaskSummary {
    TaskSummary {
        description: task.description.clone(),
        total_duration: task.duration,
        tags: task.tags.clone(),
        quantity: 1,
    }
}
