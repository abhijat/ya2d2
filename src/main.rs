extern crate linefeed;

use std::io;
use std::sync::Arc;

use cmd::{
    CommandCompleter,
    commands,
    ParseResponse,
    process_command,
};

mod cmd;

fn main() -> io::Result<()> {
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
            ParseResponse::ChangePrompt(new_prompt) =>
                reader.set_prompt(&new_prompt)?,

            ParseResponse::DisplayString(response) =>
                println!("{}", response),
        }
    }

    println!("good-bye!");
    Ok(())
}
