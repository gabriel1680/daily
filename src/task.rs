use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpTaskResponse {
    at: String,
    billable: bool,
    description: String,
    duration: u32,
    duronly: bool,
    id: u64,
    pid: u32,
    project_id: u32,
    server_deleted_at: Option<String>,
    start: String,
    stop: String,
    tag_ids: Vec<u64>,
    tags: Vec<String>,
    task_id: Option<u64>,
    uid: u64,
    user_id: u64,
    wid: u64,
    workspace_id: u64,
}

#[derive(Debug)]
pub struct Task {
    description: String,
    total_duration: u32,
    tags: Vec<String>,
    quantity: u64,
}

pub type TaskMap = HashMap<String, Task>;

pub fn make_task_map_from(tasks: Vec<HttpTaskResponse>) -> TaskMap {
    let mut task_map: TaskMap = HashMap::new();
    for task in tasks {
        task_map
            .entry(task.description.to_string())
            .and_modify(|task| {
                task.quantity += 1;
                task.total_duration += task.total_duration / 60;
            })
            .or_insert(to_task(task));
    }
    task_map
}

fn to_task(htr: HttpTaskResponse) -> Task {
    Task {
        description: htr.description.clone(),
        total_duration: htr.duration / 60,
        tags: htr.tags.clone(),
        quantity: 1,
    }
}

pub fn print_tasks(map: TaskMap) {
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
