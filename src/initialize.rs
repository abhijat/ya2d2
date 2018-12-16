use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use linefeed::{
    DefaultTerminal,
    Interface,
};

use cmd::{
    CommandCompleter,
    commands,
};

const HISTORY_FILE_PATH: &str = ".ya2d2_hist";
const DB_DIR: &str = ".ya2d2";

pub fn database() -> Result<sled::Tree, String> {
    let path_to_db = dirs::home_dir()
        .ok_or("failed to access home".to_string())?
        .join(DB_DIR);

    let tree = sled::Tree::start_default(path_to_db)
        .map_err(|e| e.to_string())?;
    Ok(tree)
}

pub fn reader(db: Arc<sled::Tree>) -> Result<Interface<DefaultTerminal>, Box<Error>> {
    let reader = linefeed::Interface::new("application")?;

    let completer = CommandCompleter::new(commands(), db);

    let histpath = dirs::home_dir()
        .ok_or("failed to access home".to_string())?
        .join(HISTORY_FILE_PATH);

    if histpath.exists() {
        reader.load_history(histpath)?;
    }

    reader.set_prompt(" [[ Ya2D2 ]] ")?;
    reader.set_completer(Arc::new(completer));

    Ok(reader)
}

pub fn hist_path() -> Result<PathBuf, String> {
    let p = dirs::home_dir()
        .ok_or("failed to access home dir".to_string())?
        .join(HISTORY_FILE_PATH);
    Ok(p)
}
