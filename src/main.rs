use std::{env, process};

use daily_cli::{
    adapters::summary_presenter::{TaskMap, TaskSummary},
    run,
};

#[tokio::main]
async fn main() {
    let days_before_today = env::args()
        .nth(1)
        .expect("no sub days specified")
        .parse::<u64>()
        .expect("days must be a integer number");
    match run(days_before_today) {
        Ok(map) => {
            print_tasks(&map);
            process::exit(0);
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn print_tasks(map: &TaskMap) {
    print_table_header();
    for (_, task) in map.iter() {
        print_task(task);
    }
}

fn print_table_header() {
    println!(
        "{0: <40} | {1: <10} | {2: <14} | {3: <10}",
        "Description", "Quantity", "Duration (min)", "Tags"
    );
}

fn print_task(task: &TaskSummary) {
    println!(
        "{0: <40} | {1: <10} | {2: <14} | {3: <10}",
        task.description, task.quantity, task.total_duration, task.tags[0]
    );
}
