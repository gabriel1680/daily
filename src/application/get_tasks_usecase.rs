use std::fmt;

use chrono::{DateTime, Days, Utc};

use crate::{application::get_tasks_input::GetTasksInput, domain::task::Task};

#[derive(Debug, Clone)]
pub struct GetTaskErr {
    pub message: String,
}

impl fmt::Display for GetTaskErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error on retrieving tasks: {}", self.message)
    }
}

pub type TasksGateway = Box<dyn Fn(String, String) -> Result<Vec<Task>, GetTaskErr>>;

pub type GetTasks = Box<dyn Fn(GetTasksInput) -> Result<Vec<Task>, GetTaskErr>>;

pub fn get_tasks_usecase(gateway: TasksGateway) -> GetTasks {
    let get_tasks = move |input: GetTasksInput| {
        let (now_date, day_before_date) = get_days_range(input.days_before_today);
        let result = gateway(day_before_date, now_date);
        result
    };
    Box::new(get_tasks)
}

fn get_days_range(days: u64) -> (String, String) {
    let now = Utc::now();
    let day_before_date = now.checked_sub_days(Days::new(days)).unwrap_or(now);
    (format_date(now), format_date(day_before_date))
}

fn format_date(date: DateTime<Utc>) -> String {
    let fmt = "%Y-%m-%d";
    date.format(fmt).to_string()
}
