use std::fmt;

pub struct Task {
    id: String,
    description: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.id, self.description)
    }
}

impl Task {
    pub fn new(description: String) -> Self {
        let digest = md5::compute(&description);
        let id = format!("{:x}", digest)
            .chars()
            .take(6)
            .collect::<String>();
        Task { id, description }
    }
}
