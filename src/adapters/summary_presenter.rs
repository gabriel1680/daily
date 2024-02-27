use std::collections::HashMap;

use crate::domain::task::Task;

#[derive(Debug, Clone)]
pub struct TaskSummary {
    pub description: String,
    pub total_duration: f32,
    pub tags: Vec<String>,
    pub quantity: u32,
}

pub type TaskMap = HashMap<String, TaskSummary>;

const EMPTY_STR: &str = "\u{0020}";

/// Map a Vec of Task entity to a HashMap of TaskSummary
pub fn tasks_summary_presenter(tasks: &Vec<Task>) -> TaskMap {
    let mut task_map: TaskMap = HashMap::new();
    for task in tasks {
        task_map
            .entry(task.description.to_string())
            .and_modify(|summary| {
                summary.quantity += 1;
                summary.total_duration += sec_to_min(task.duration);
                let fallback = String::from(EMPTY_STR);
                let tag = task.tags.get(0).unwrap_or(&fallback);
                if !summary.tags.contains(tag) {
                    summary.tags.push(tag.clone().to_owned());
                }
            })
            .or_insert(to_output(&task));
    }
    task_map
}

fn to_output(task: &Task) -> TaskSummary {
    TaskSummary {
        description: task.description.clone(),
        total_duration: sec_to_min(task.duration),
        tags: if task.tags.len() == 0 {
            vec![EMPTY_STR.to_owned()]
        } else {
            task.tags.clone()
        },
        quantity: 1,
    }
}

fn sec_to_min(sec: u32) -> f32 {
    ((sec as f32) / 60.0).round()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_task_summary_of(key: &str) -> TaskSummary {
        let tasks = make_dummy_tasks();
        let map = tasks_summary_presenter(&tasks);
        map.get(key).unwrap().clone()
    }

    #[test]
    fn tasks_accumulation() {
        let task_summary = get_task_summary_of("Task 1");
        assert_eq!(task_summary.quantity, 2);
        assert_eq!(task_summary.total_duration, 80.0);
    }

    #[test]
    fn one_task_only_accumulation() {
        let task_summary = get_task_summary_of("Task 3");
        assert_eq!(task_summary.quantity, 1);
        assert_eq!(task_summary.total_duration, 1.0);
    }

    #[test]
    fn empty_tags_vec() {
        let task_summary = get_task_summary_of("Task 3");
        assert_eq!(task_summary.tags, vec!["\u{0020}"]);
    }

    #[test]
    fn tags_concatenation() {
        let task_summary = get_task_summary_of("Task 1");
        assert_eq!(task_summary.tags, vec!["tag 1", "tag 2"]);
    }

    fn make_dummy_tasks() -> Vec<Task> {
        vec![
            Task {
                id: 1,
                description: String::from("Task 1"),
                duration: 3600,
                tags: vec![String::from("tag 1")],
            },
            Task {
                id: 2,
                description: String::from("Task 1"),
                duration: 1200,
                tags: vec![String::from("tag 2")],
            },
            Task {
                id: 3,
                description: String::from("Task 3"),
                duration: 60,
                tags: vec![],
            },
        ]
    }
}
