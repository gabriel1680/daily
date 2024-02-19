#[derive(Debug)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub duration: u32,
    pub tags: Vec<String>,
}
