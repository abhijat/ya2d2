use std::error::Error;

use linefeed::{
    Interface,
    Terminal,
};

use cmd::{ParseResponse, process_command};
use task::Task;

pub fn start_shell<T>(tree: &sled::Tree, reader: &Interface<T>) -> Result<(), Box<Error>>
    where T: Terminal {
    while let linefeed::ReadResult::Input(data) = reader.read_line()? {
        if data.is_empty() {
            continue;
        }

        reader.add_history_unique(data.clone());
        let response = process_command(data);
        match response {
            ParseResponse::ChangePromptCommand(new_prompt) =>
                reader.set_prompt(&new_prompt)?,
            ParseResponse::DisplayStringCommand(response) =>
                println!("{}", response),
            ParseResponse::PushCommand(description) => {
                let task = Task::new(description);
                let payload = serde_json::to_string(&task)?;
                tree.set(task.id, payload.as_bytes().to_vec())?;
                tree.flush()?;
            }
            ParseResponse::ListCommand(_count) =>
                for pair in tree.iter() {
                    let (_key, value) = pair?;
                    let task: Task = serde_json::from_slice(&value)?;
                    println!("{}", task);
                }
            ParseResponse::PopCommand(ref key) =>
                match tree.del(key)? {
                    None => println!("The key '{}' is not present in the database", key),
                    Some(value) => {
                        let task: Task = serde_json::from_slice(&value)?;
                        println!("removed: {}", task);
                    }
                }
        }
    }

    tree.flush()?;

    println!("good-bye!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use linefeed::memory::MemoryTerminal;

    use super::*;

    fn in_memory_db() -> sled::Tree {
        let config = sled::ConfigBuilder::new()
            .temporary(true)
            .build();
        sled::Tree::start(config)
            .expect("failed to build in memory db for testing")
    }

    fn in_memory_reader(terminal: MemoryTerminal) -> Interface<MemoryTerminal> {
        Interface::with_term("test", terminal)
            .expect("failed to build in memory reader for testing")
    }

    fn term_with_commands(commands: Vec<&str>) -> MemoryTerminal {
        let term = MemoryTerminal::new();
        commands.iter().for_each(|&s| {
            let command = format!("{}\n", s);
            term.push_input(&command);
        });

        term.push_input("\x04");
        term
    }

    fn nth_task_from_db(db: &sled::Tree, n: usize) -> Task {
        let (_, value) = db.iter()
            .nth(n)
            .unwrap()
            .unwrap();

        serde_json::from_slice(&value).expect(&format!(
            "failed to deserialize payload {}",
            String::from_utf8_lossy(&value)
        ))
    }

    #[test]
    fn test_adding_a_task_to_shell() {
        let db = in_memory_db();
        let reader = in_memory_reader(
            term_with_commands(vec!["push gone with the wind", ])
        );

        start_shell(&db, &reader).unwrap();
        assert_eq!(nth_task_from_db(&db, 0).description, "gone with the wind");
    }

    #[test]
    fn add_then_remove_task_from_shell_leaves_db_empty() {
        let db = in_memory_db();

        let task_string = "days of the future past";
        let task_id = Task::task_id(task_string);

        let push_command = format!("push {}", task_string);
        let pop_command = format!("pop {}", task_id);

        let commands = vec![
            push_command.as_str(),
            pop_command.as_str()
        ];

        let reader = in_memory_reader(
            term_with_commands(commands)
        );

        start_shell(&db, &reader).unwrap();
        assert_eq!(db.len(), 0);
    }
}

