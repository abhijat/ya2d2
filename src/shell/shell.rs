use std::error::Error;
use std::sync::Arc;

use linefeed::{
    Interface,
    Terminal,
};

use cmd::{
    CommandObject,
    CommandParserResponse,
    parse_input,
};
use config::Configuration;
use initialize::hist_path;
use task::Task;

pub fn start_shell<T>(tree: Arc<sled::Tree>, reader: &Interface<T>, config: Option<&Configuration>) -> Result<(), Box<Error>>
    where T: Terminal {

    let display = ::shell::display::Display::new(config);

    while let linefeed::ReadResult::Input(data) = reader.read_line()? {
        let data = data.trim();

        if data.is_empty() {
            continue;
        }

        reader.add_history_unique(data.to_owned());
        let response = parse_input(data);
        match response {
//            ParseResponse::ChangePromptCommand(new_prompt) =>
//                reader.set_prompt(&new_prompt)?,

            CommandParserResponse::Error(response) =>
                display.show(&response),

            CommandParserResponse::Command(command_object) => {
                match command_object {
                    CommandObject::PushCommand { payload, tags: _ } => {
                        let task = Task::new(payload);
                        match serde_json::to_string(&task) {
                            Ok(payload) => {
                                if let Err(err) = tree.set(task.id, payload.as_bytes().to_vec()) {
                                    display.show(&format!("failed to save task in db: {}", err));
                                }
                            }
                            Err(err) => display.show(&format!("failed to serialize task before save: {}", err)),
                        }

                        if let Err(err) = tree.flush() {
                            display.show(&format!("failed to flush db: {}", err));
                        }
                    }
                    CommandObject::PopCommand { ref key } => {
                        match tree.del(key)? {
                            None => display.show(&format!("The key '{}' is not present in the database", key)),
                            Some(value) => {
                                match serde_json::from_slice::<Task>(&value) {
                                    Ok(task) => println!("removed [{}]", task),
                                    Err(err) => display.show(&format!("failed to deserialize task: {}", err)),
                                }
                            }
                        }
                    }
                    CommandObject::ListCommand { tags: _ } => {
                        for pair in tree.iter() {
                            match pair {
                                Ok((_, value)) => match serde_json::from_slice::<Task>(&value) {
                                    Ok(task) => println!("{}", task),
                                    Err(err) => display.show(&format!("failed to deserialize task: {}", err)),
                                },
                                Err(err) => display.show(&format!("failed to read entry: {}", err)),
                            }
                        }
                    }
                }
            }
        }
    }

    if let Err(err) = tree.flush() {
        display.show(&format!("failed to flush db: {}", err));
    }

    reader.save_history(hist_path()?)?;

    println!("good-bye!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use linefeed::memory::MemoryTerminal;

    use config::Configuration;
    use task::Task;

    use super::*;

    fn in_memory_db() -> Arc<sled::Tree> {
        let config = sled::ConfigBuilder::new()
            .temporary(true)
            .build();
        let tree = sled::Tree::start(config)
            .expect("failed to build in memory db for testing");
        Arc::new(tree)
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

    fn nth_task_from_db(db: Arc<sled::Tree>, n: usize) -> Task {
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

        let cfg = Configuration::default();
        start_shell(db.clone(), &reader, Some(&cfg)).unwrap();
        assert_eq!(nth_task_from_db(db.clone(), 0).description, "gone with the wind");
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

        let cfg = Configuration::new();
        start_shell(db.clone(), &reader, Some(&cfg)).unwrap();
        assert_eq!(db.len(), 0);
    }
}

