use std::error::Error;
use std::sync::Arc;

use cmd::{CommandCompleter, commands, ParseResponse, process_command};
use task::Task;

pub fn start_shell(tree: sled::Tree) -> Result<(), Box<Error>> {
    let reader = linefeed::Interface::new("application")?;

    let completer = CommandCompleter::new(commands());

    reader.set_prompt("Ya2d2> ")?;
    reader.set_completer(Arc::new(completer));

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

            ParseResponse::ListCommand(_count) => {
                for pair in tree.iter() {
                    let (_key, value) = pair?;
                    let task: Task = serde_json::from_slice(&value)?;
                    println!("{}", task);
                }
            }

            ParseResponse::PopCommand(ref key) => {
                let removed = tree.del(key)?;

                match removed {
                    None => println!("The key '{}' is not present in the database", key),
                    Some(value) => {
                        let task: Task = serde_json::from_slice(&value)?;
                        println!("removed: {}", task);
                    }
                }
            }
        }
    }

    tree.flush()?;

    println!("good-bye!");
    Ok(())
}
