use nom::types::CompleteStr;

pub type Tags<'a> = Option<Vec<CompleteStr<'a>>>;

pub enum CommandParserResponse {
    Command(CommandObject),
    Error(String),
}

pub enum CommandObject {
    PushCommand { payload: String, tags: Option<Vec<String>> },
    PopCommand { key: String },
    ListCommand { tags: Option<Vec<String>> },
}

impl CommandObject {
    fn build_tags(_tags: Tags) -> Option<Vec<String>> {
        _tags.map(|tags| tags.iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<String>>()
        )
    }

    pub fn push_command(_payload: CompleteStr, _tags: Tags) -> Self {
        CommandObject::PushCommand {
            payload: _payload.as_ref().to_owned(),
            tags: Self::build_tags(_tags),
        }
    }

    pub fn pop_command(_key: CompleteStr) -> Self {
        CommandObject::PopCommand { key: _key.as_ref().to_owned() }
    }

    pub fn ls_command(_tags: Tags) -> Self {
        CommandObject::ListCommand { tags: Self::build_tags(_tags) }
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
