use self::ParseResponse::*;

pub enum ParseResponse {
    ChangePrompt(String),
    DisplayString(String),
}

pub fn process_command(command: String) -> ParseResponse {
    match command.trim_right().as_ref() {
        "ls" => DisplayString("heres your list".to_string()),
        s if s.starts_with("push") => parse_push_command(s),
        "pop" => DisplayString("pop away friendo".to_string()),
        s if s.starts_with("change-prompt") => parse_change_prompt_command(s),
        _ => DisplayString("unknown command, my good pal".to_string()),
    }
}

fn parse_change_prompt_command(command: &str) -> ParseResponse {
    let new_prompt = command.split_whitespace()
        .skip(1)
        .nth(0);
    match new_prompt {
        Some(s) => {
            let prompt_string = format!("{} ", s);
            ChangePrompt(prompt_string)
        }
        None => DisplayString("usage: change-prompt <new-prompt>".to_string()),
    }
}

fn parse_push_command(command: &str) -> ParseResponse {
    let payload = command.split_whitespace()
        .skip(1)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    if payload.is_empty() {
        DisplayString("usage: push <description of the task>".to_string())
    } else {
        DisplayString(format!("Let's add the task '{}'", payload))
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
