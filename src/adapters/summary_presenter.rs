use std::collections::HashMap;

use crate::domain::task::Task;

#[derive(Debug)]
pub struct TaskSummary {
    pub description: String,
    pub total_duration: f32,
    pub tags: Vec<String>,
    pub quantity: u32,
}

pub type TaskMap = HashMap<String, TaskSummary>;

/// Map a Vec of Task entity to a HashMap of TaskSummary
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
        total_duration: ((task.duration as f32) / 60.0).round(),
        tags: task.tags.clone(),
        quantity: 1,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tasks_summary_presenter_test() {
        let tasks = make_dummy_tasks();
        let output = tasks_summary_presenter(&tasks);
        let task_1_summary = output.get("Task 1").unwrap();
        assert_eq!(task_1_summary.quantity, 2);
        assert_eq!(task_1_summary.total_duration, 120.0);
        let task_3_summary = output.get("Task 3").unwrap();
        assert_eq!(task_3_summary.quantity, 1);
        assert_eq!(task_3_summary.total_duration, 1.0);
    }

    fn make_dummy_tasks() -> Vec<Task> {
        vec![
            Task {
                id: 1,
                description: String::from("Task 1"),
                duration: 3600,
                tags: vec![String::from("some tag")],
            },
            Task {
                id: 2,
                description: String::from("Task 1"),
                duration: 3600,
                tags: vec![String::from("some tag")],
            },
            Task {
                id: 3,
                description: String::from("Task 3"),
                duration: 60,
                tags: vec![String::from("some tag")],
            },
        ]
    }
}
