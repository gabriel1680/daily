use chrono::{DateTime, Days, Utc};

use crate::{
    application::{
        get_tasks_error::GetTaskErr, get_tasks_input::GetTasksInput, tasks_gateway::TasksGateway,
    },
    domain::task::Task,
};

pub fn get_tasks_usecase(
    gateway: TasksGateway,
) -> impl Fn(GetTasksInput) -> Result<Vec<Task>, GetTaskErr> {
    let get_tasks = move |input: GetTasksInput| {
        let (now_date, day_before_date) = get_days_range(input.days_before_today);
        let result = gateway(day_before_date, now_date);
        result
    };
    get_tasks
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
