use self::ParseResponse::*;

pub enum ParseResponse {
    ChangePromptCommand(String),
    DisplayStringCommand(String),
    PushCommand(String),
    ListCommand(i64),
    PopCommand(String),
}

pub fn process_command(command: String) -> ParseResponse {
    match command.trim_right().as_ref() {
        // TODO we want to parse this string and accept a count of items to show
        "ls" => ListCommand(0),
        s if s.starts_with("push") => parse_push_command(s),
        s if s.starts_with("pop") => parse_pop_command(s),
        s if s.starts_with("change-prompt") => parse_change_prompt_command(s),
        _ => DisplayStringCommand("unknown command, my good pal".to_string()),
    }
}

fn parse_change_prompt_command(command: &str) -> ParseResponse {
    let new_prompt = command.split_whitespace()
        .skip(1)
        .nth(0);
    match new_prompt {
        Some(s) => {
            let prompt_string = format!("{} ", s);
            ChangePromptCommand(prompt_string)
        }
        None => DisplayStringCommand("usage: change-prompt <new-prompt>".to_string()),
    }
}

fn parse_pop_command(command: &str) -> ParseResponse {
    let pop_args = command.split_whitespace().collect::<Vec<&str>>();
    if pop_args.len() != 2 {
        DisplayStringCommand("usage: pop <key of entry to pop>".to_string())
    } else {
        PopCommand(pop_args.get(1).unwrap().to_string())
    }
}

fn parse_push_command(command: &str) -> ParseResponse {
    let payload = command.split_whitespace()
        .skip(1)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    if payload.is_empty() {
        DisplayStringCommand("usage: push <description of the task>".to_string())
    } else {
        PushCommand(payload)
    }
}

pub fn commands() -> Vec<String> {
    vec![
        "ls".to_string(),
        "push".to_string(),
        "pop".to_string(),
        "change-prompt".to_string(),
    ]
}
