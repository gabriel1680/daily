#[derive(Debug)]
pub struct Task {
    pub description: String,
    pub total_duration: u32,
    pub tags: Vec<String>,
    pub quantity: u64,
}
