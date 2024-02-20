use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct GetTaskErr {
    pub message: String,
}

impl fmt::Display for GetTaskErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error on retrieving tasks: {}", self.message)
    }
}
