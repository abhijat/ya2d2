extern crate linefeed;
extern crate md5;

use std::io;
use std::sync::Arc;

use cmd::{
    CommandCompleter,
    commands,
    ParseResponse,
    process_command,
};

mod cmd;
mod task;

fn main() -> io::Result<()> {
    let reader = linefeed::Interface::new("application")?;

    let completer = CommandCompleter::new(commands());

    reader.set_prompt("Ya2d2> ")?;
    reader.set_completer(Arc::new(completer));

    let mut todo: Vec<task::Task> = Vec::new();

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
                let task = task::Task::new(description);
                todo.push(task);
            }

            ParseResponse::ListCommand(_count) =>
                todo.iter().for_each(|s| {
                    println!("{}", s)
                }),
        }
    }

    println!("good-bye!");
    Ok(())
}
