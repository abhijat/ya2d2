use std::error::Error;
use std::sync::Arc;

use linefeed::{
    DefaultTerminal,
    Interface,
};

use cmd::{
    CommandCompleter,
    commands,
};

pub fn database() -> Result<sled::Tree, String> {
    let mut path_to_db = dirs::home_dir()
        .ok_or("failed to access home".to_string())?;
    path_to_db.push(".ya2d2");

    let tree = sled::Tree::start_default(path_to_db)
        .map_err(|e| e.to_string())?;
    Ok(tree)
}

pub fn reader() -> Result<Interface<DefaultTerminal>, Box<Error>> {
    let reader = linefeed::Interface::new("application")?;

    let completer = CommandCompleter::new(commands());

    reader.set_prompt("Ya2d2> ")?;
    reader.set_completer(Arc::new(completer));

    Ok(reader)
}
