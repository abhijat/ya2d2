use std::fmt;

const TASK_ID_LENGTH: usize = 6;

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
    pub fn task_id(description: &str) -> String {
        let digest = md5::compute(description);
        format!("{:x}", digest)
            .chars()
            .take(TASK_ID_LENGTH)
            .collect()
    }

    pub fn new(description: String) -> Self {
        let id = Task::task_id(&description);
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

        assert_eq!(task.id.len(), TASK_ID_LENGTH);
    }
}
