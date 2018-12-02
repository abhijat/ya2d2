use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub description: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tasks_have_id_of_correct_size() {
        let payload = "foo bar baz";
        let task = Task::new(payload.to_string());
        assert_eq!(task.id.len(), 6);
    }
}
