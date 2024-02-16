use serde_json::Value;
use std::collections::HashMap;

pub fn make_task_map_from(items: Vec<Value>) -> HashMap<String, i32> {
    let mut task_map = HashMap::new();
    items.iter().for_each(|item| {
        task_map
            .entry(item["description"].to_string())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });
    task_map
}

pub fn print_tasks(map: HashMap<String, i32>) {
    print_table_header("description", "quantity");
    for (task, quantity) in map.iter() {
        print_table_line(task.to_string(), *quantity);
    }
}

fn print_table_header(header_1: &str, header_2: &str) {
    println!("{0: <40} | {1: <10}", header_1, header_2);
}

fn print_table_line(key: String, value: i32) {
    println!("{0: <40} | {1: <10}", key, value);
}
