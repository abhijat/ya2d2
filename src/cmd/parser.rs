pub enum ParseResponse {
    ChangePrompt(String),
    DisplayString(String),
}

pub fn process_command(command: String) -> ParseResponse {
    use self::ParseResponse::*;

    match command.trim_right().as_ref() {
        "ls" => DisplayString("heres your list".to_string()),
        "push" => DisplayString("push away friend".to_string()),
        "pop" => DisplayString("pop away friendo".to_string()),
        s if s.starts_with("change-prompt") => parse_change_prompt_command(s),
        _ => DisplayString("unknown command, my good pal".to_string()),
    }
}

fn parse_change_prompt_command(command: &str) -> ParseResponse {
    use self::ParseResponse::*;

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

pub fn commands() -> Vec<String> {
    vec![
        "ls".to_string(),
        "push".to_string(),
        "pop".to_string(),
        "change-prompt".to_string(),
    ]
}
