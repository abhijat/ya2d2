use std::sync::Arc;

use linefeed::{
    Completer,
    Completion,
    Prompter,
    Terminal,
};

pub struct CommandCompleter {
    commands: Vec<String>,
    pub db: Arc<sled::Tree>,
}

impl CommandCompleter {
    pub fn new(commands: Vec<String>, db: Arc<sled::Tree>) -> Self {
        CommandCompleter { commands, db }
    }
}

impl<Term> Completer<Term> for CommandCompleter where Term: Terminal {
    fn complete(&self, word: &str, prompter: &Prompter<Term>, start: usize, _end: usize)
                -> Option<Vec<Completion>> {
        let line = prompter.buffer();

        if line.starts_with("pop ") {
            let ids = self.db.iter()
                .map(|k| {
                    let (key, _) = k.unwrap();
                    String::from_utf8(key).unwrap()
                })
                .map(|s| Completion::simple(s))
                .collect::<Vec<Completion>>();

            if line == "pop " {
                Some(ids)
            } else {
                if start > 4 {
                    None
                } else {
                    let partial_id = &line[start..];

                    Some(ids.into_iter()
                        .filter(|s| s.completion.starts_with(partial_id))
                        .collect::<Vec<Completion>>())
                }
            }
        } else {
            let completions: Vec<Completion> = self.commands.iter()
                .filter(|&s| s.starts_with(word))
                .map(|s| Completion::simple(s.clone()))
                .collect();
            Some(completions)
        }
    }
}
