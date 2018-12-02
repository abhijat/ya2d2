use self::ParseResponse::*;

const PUSH_USAGE: &str = "usage: push <description of the task>";
const CHANGE_PROMPT_USAGE: &str = "usage: change-prompt <new-prompt>";
const POP_USAGE: &str = "usage: pop <key of entry to pop>";
const UNKNOWN_COMMAND: &str = "unknown command :-(";

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
        _ => DisplayStringCommand(UNKNOWN_COMMAND.to_string()),
    }
}

fn parse_change_prompt_command(command: &str) -> ParseResponse {
    // TODO we want to fail when the prompt is multi word?
    let new_prompt = command.split_whitespace()
        .skip(1)
        .nth(0);
    match new_prompt {
        Some(s) => {
            let prompt_string = format!("{} ", s);
            ChangePromptCommand(prompt_string)
        }
        None => DisplayStringCommand(CHANGE_PROMPT_USAGE.to_string()),
    }
}

fn parse_pop_command(command: &str) -> ParseResponse {
    let pop_args = command.split_whitespace().collect::<Vec<&str>>();
    if pop_args.len() != 2 {
        DisplayStringCommand(POP_USAGE.to_string())
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
        DisplayStringCommand(PUSH_USAGE.to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    const INVALID_RESPONSE: &str = "Invalid response variant";

    #[test]
    fn parse_push_command_without_args_shows_help() {
        match parse_push_command("push   ") {
            DisplayStringCommand(ref s) => assert_eq!(s, PUSH_USAGE),
            _ => panic!(INVALID_RESPONSE),
        }
    }

    #[test]
    fn pop_command_without_args_shows_help() {
        match parse_pop_command("pop     ") {
            DisplayStringCommand(ref s) => assert_eq!(s, POP_USAGE),
            _ => panic!(INVALID_RESPONSE),
        }
    }

    #[test]
    fn pop_command_with_too_many_args_shows_help() {
        match parse_pop_command("pop one two three four") {
            DisplayStringCommand(ref s) => assert_eq!(s, POP_USAGE),
            _ => panic!(INVALID_RESPONSE)
        }
    }

    #[test]
    fn change_prompt_command_without_args_shows_help() {
        match parse_change_prompt_command("change-prompt     ") {
            DisplayStringCommand(ref s) => assert_eq!(CHANGE_PROMPT_USAGE, s),
            _ => panic!(INVALID_RESPONSE),
        }
    }

    #[test]
    fn unknown_command_response() {
        match process_command("foo bar".to_string()) {
            DisplayStringCommand(ref s) => assert_eq!(s, UNKNOWN_COMMAND),
            _ => panic!(INVALID_RESPONSE),
        }
    }

    #[test]
    fn list_command_response() {
        match process_command("ls".to_string()) {
            ListCommand(n) => assert_eq!(n, 0),
            _ => panic!(INVALID_RESPONSE),
        }
    }

    #[test]
    fn push_command_response_for_valid_input() {
        let task = "gone with the wind";
        match process_command(format!("push {}", task)) {
            PushCommand(ref s) => assert_eq!(s, task),
            _ => panic!(INVALID_RESPONSE),
        }
    }

    #[test]
    fn pop_command_response_for_valid_input() {
        let key = "abcdef";
        match process_command(format!("pop {}", key)) {
            PopCommand(s) => assert_eq!(s, key),
            _ => panic!(INVALID_RESPONSE),
        }
    }

    #[test]
    fn change_prompt_response_for_valid_input() {
        let new_prompt = "x123";
        match process_command(format!("change-prompt {}", new_prompt)) {
            ChangePromptCommand(ref s) => assert_eq!(s, &format!("{} ", new_prompt)),
            _ => panic!(INVALID_RESPONSE),
        }
    }
}
