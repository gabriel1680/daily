pub struct GetTasksInput {
    pub days_before_today: u64,
}

impl GetTasksInput {
    pub fn new(days_before_today: u64) -> Self {
        Self { days_before_today }
    }
}
