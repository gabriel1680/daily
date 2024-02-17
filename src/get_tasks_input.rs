pub struct GetTasksInput {
    pub days_before_today: u64,
    pub password: String,
    pub username: String,
}

impl GetTasksInput {
    pub fn new(days_before_today: u64, password: String, username: String) -> Self {
        Self {
            days_before_today,
            password,
            username,
        }
    }
}
