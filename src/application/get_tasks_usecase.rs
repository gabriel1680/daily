use chrono::{DateTime, Days, Utc};

use crate::{
    application::{
        get_tasks_error::GetTaskErr, get_tasks_input::GetTasksInput, tasks_gateway::TasksGateway,
    },
    domain::task::Task,
};

/// GetTasksUseCase
///
/// # Errors
///
/// This function will return an error if the gateway returns it.
pub fn get_tasks_usecase(
    gateway: TasksGateway,
) -> impl Fn(GetTasksInput) -> Result<Vec<Task>, GetTaskErr> + '_ {
    let get_tasks = |input: GetTasksInput| {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dummy_tasks_result() -> Result<Vec<Task>, GetTaskErr> {
        Ok(vec![Task {
            id: 1,
            description: String::from("some description"),
            duration: 12,
            tags: vec![String::from("some tag")],
        }])
    }

    #[test]
    fn get_tasks_usecase_test() {
        let mocked_response = make_dummy_tasks_result();
        let gateway =
            |_: String, _: String| -> Result<Vec<Task>, GetTaskErr> { make_dummy_tasks_result() };
        let input = GetTasksInput::new(12);
        let sut = get_tasks_usecase(&gateway);
        assert_eq!(sut(input), mocked_response);
    }
}
