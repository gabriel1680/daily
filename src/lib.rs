pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;

pub fn run() {
    infrastructure::run()
}