use nom::types::CompleteStr;

use cmd::command_objects::{
    CommandObject,
    CommandParserResponse,
    Tags,
};

// define the command keyword parsers
named!( push_parser<CompleteStr, CompleteStr>, tag!( "push" ) );
named!( pop_parser<CompleteStr, CompleteStr>, tag!( "pop" ) );
named!( ls_parser<CompleteStr, CompleteStr>, tag!( "ls" ) );

// define the optional tag parser, so we can tag entries
named!(
    entry_tag<CompleteStr, CompleteStr>,
    delimited!(
        tag!("[["),
        take_until!("]]"),
        tag!("]]")
    )
);

// define the parser to combine tag entries - consumes one or more entry tags
named!(
    optional_multi_tags<CompleteStr, Tags>,
    opt!(
        many1!(
            ws!( entry_tag )
        )
    )
);

named!(
    push_command_parser<CompleteStr, CommandObject>,
    do_parse!(
        push_parser >>
        tag!(" ") >>
        tags: optional_multi_tags >>
        payload: take_till1!(|ch: char| ch == '\r' || ch == '\n') >>
        eof!() >>
        (CommandObject::push_command(payload, tags))
    )
);

named!(
    pop_command_parser<CompleteStr, CommandObject>,
    do_parse!(
        pop_parser >>
        tag!(" ") >>
        key: take_till1!(|ch: char| !ch.is_alphanumeric()) >>
        eof!() >>
        (CommandObject::pop_command(key))
    )
);

named!(
    ls_command_parser<CompleteStr, CommandObject>,
    do_parse!(
        ls_parser >>
        tags: ws!( optional_multi_tags ) >>
        eof!() >>
        (CommandObject::ls_command(tags))
    )
);

pub fn parse_input(input: &str) -> CommandParserResponse {
    if input.starts_with("push") {
        analyze(push_command_parser(CompleteStr(input)), input)
    } else if input.starts_with("pop") {
        analyze(pop_command_parser(CompleteStr(input)), input)
    } else if input.starts_with("ls") {
        analyze(ls_command_parser(CompleteStr(input)), input)
    } else {
        CommandParserResponse::Error("unknown command!".to_owned())
    }
}

fn analyze(result: nom::IResult<CompleteStr, CommandObject>, command_string: &str) -> CommandParserResponse {
    match result {
        Ok(result) => {
            let (_left_over, command_object) = result;
            CommandParserResponse::Command(command_object)
        }
        Err(e) => {
            // TODO usage needs to come from here!
            CommandParserResponse::Error(format!("failed to parse command: [{}] error: [{}]", command_string, e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_valid_push_command<T>(input: &str, f: T) where T: (Fn(String, Option<Vec<String>>) -> ()) {
        let response = parse_input(input);
        if let CommandParserResponse::Command(object) = response {
            if let CommandObject::PushCommand { payload, tags } = object {
                f(payload, tags);
            } else {
                panic!("invalid object type for push command")
            }
        } else {
            panic!("invalid error response for push command")
        }
    }

    fn assert_failed_while_parsing(input: &str) {
        match parse_input(input) {
            CommandParserResponse::Error(e) => assert!(e.starts_with("failed to parse command")),
            _ => panic!("invalid response"),
        }
    }

    #[test]
    fn test_push_command_valid_data_no_tags() {
        test_valid_push_command("push foo bar", |payload, tags| {
            assert_eq!(payload, "foo bar");
            assert!(tags.is_none());
        });
    }

    #[test]
    fn test_push_command_valid_data_single_tag() {
        test_valid_push_command("push [[foobar]] birds are up in the air", |payload, tags| {
            assert_eq!(payload, "birds are up in the air");
            assert!(tags.is_some());

            let tags = tags.unwrap();
            assert_eq!(tags.get(0).unwrap(), "foobar");
        });
    }

    #[test]
    fn test_push_command_valid_data_multiple_tags() {
        test_valid_push_command("push [[foobar]] [[birds]] are up in the air", |payload, tags| {
            assert_eq!(payload, "are up in the air");
            assert!(tags.is_some());

            let tags = tags.unwrap();
            assert_eq!(tags.get(0).unwrap(), "foobar");
            assert_eq!(tags.get(1).unwrap(), "birds");
        });
    }

    #[test]
    fn test_push_command_with_no_other_inputs_fails() {
        assert_failed_while_parsing("push");
    }

    #[test]
    fn test_push_command_with_tags_and_no_other_inputs_fails() {
        assert_failed_while_parsing("push [[fff]]");
        assert_failed_while_parsing("push [[fff]] [[xxxxx]]");
    }

    fn test_valid_pop_command<T>(input: &str, f: T) where T: (Fn(String) -> ()) {
        if let CommandParserResponse::Command(object) = parse_input(input) {
            if let CommandObject::PopCommand { key } = object {
                f(key);
            } else {
                panic!("invalid object type for push command")
            }
        } else {
            panic!("invalid error response for push command")
        }
    }

    #[test]
    fn test_pop_command_with_key_is_valid() {
        test_valid_pop_command("pop foobar", |key| {
            assert_eq!(key, "foobar");
        });
    }

    #[test]
    fn test_pop_fails_if_tried_alone() {
        assert_failed_while_parsing("pop");
    }

    #[test]
    fn pop_fails_if_tried_with_tags_and_key() {
        assert_failed_while_parsing("pop [[fff]] drat");
    }

    #[test]
    fn pop_fails_if_tried_with_tags_and_no_keys() {
        assert_failed_while_parsing("pop [[foobar]]");
    }

    fn test_valid_list_command<T>(input: &str, f: T) where T: (Fn(Option<Vec<String>>) -> ()) {
        if let CommandParserResponse::Command(object) = parse_input(input) {
            if let CommandObject::ListCommand { tags } = object {
                f(tags);
            } else {
                panic!("invalid object type for push command")
            }
        } else {
            panic!("invalid error response for push command")
        }
    }

    #[test]
    fn test_list_all_by_itself_is_a_valid_command() {
        test_valid_list_command("ls", |tags| {
            assert!(tags.is_none());
        });
    }

    #[test]
    fn test_list_accepts_a_list_of_tags() {
        test_valid_list_command("ls [[foobar]] [[xxxx]]", |tags| {
            assert!(tags.is_some());
            let tags = tags.unwrap();

            assert_eq!(tags.get(0).unwrap(), "foobar");
            assert_eq!(tags.get(1).unwrap(), "xxxx");
        });
    }

    #[test]
    fn test_list_with_key_is_invalid() {
        assert_failed_while_parsing("ls fxxxx");
    }
}