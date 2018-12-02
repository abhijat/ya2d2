pub fn init_db() -> Result<sled::Tree, String> {
    let mut path_to_db = dirs::home_dir()
        .ok_or("failed to access home".to_string())?;
    path_to_db.push(".ya2d2");

    let tree = sled::Tree::start_default(path_to_db)
        .map_err(|e| e.to_string())?;
    Ok(tree)
}
