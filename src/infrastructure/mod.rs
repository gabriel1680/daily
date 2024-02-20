use crate::adapters::summary_presenter::TaskMap;
use crate::adapters::{
    summary_presenter::tasks_summary_presenter, task_http_gateway_adapters::make_http_tasks_gateway,
};
use crate::application::get_tasks_error::GetTaskErr;
use crate::application::tasks_gateway::TasksGateway;
use crate::application::{get_tasks_input::GetTasksInput, get_tasks_usecase::get_tasks_usecase};

pub fn run(days_before_today: u64) -> Result<TaskMap, GetTaskErr> {
    let input = GetTasksInput::new(days_before_today);
    let gateway: TasksGateway = make_http_tasks_gateway();
    let tasks = get_tasks_usecase(gateway)(input)?;
    let tasks_map = tasks_summary_presenter(&tasks);
    Ok(tasks_map)
}
