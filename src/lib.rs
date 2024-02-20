use adapters::summary_presenter::TaskMap;
use application::get_tasks_error::GetTaskErr;

pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;

pub fn run(days_before_today: u64) -> Result<TaskMap, GetTaskErr> {
    infrastructure::run(days_before_today)
}
