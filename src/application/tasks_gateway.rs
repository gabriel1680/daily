use crate::domain::task::Task;

use super::get_tasks_error::GetTaskErr;

pub type TasksGateway = Box<dyn Fn(String, String) -> Result<Vec<Task>, GetTaskErr>>;
