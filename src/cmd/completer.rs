use linefeed::{
    Completer,
    Completion,
    Prompter,
    Terminal,
};

pub struct CommandCompleter {
    commands: Vec<String>
}

impl CommandCompleter {
    pub fn new(commands: Vec<String>) -> Self {
        CommandCompleter { commands }
    }
}

impl<Term> Completer<Term> for CommandCompleter where Term: Terminal {
    fn complete(
        &self,
        word: &str,
        _prompter: &Prompter<Term>,
        _start: usize,
        _end: usize,
    ) -> Option<Vec<Completion>> {
        let completions: Vec<Completion> = self.commands.iter()
            .filter(|&s| s.starts_with(word))
            .map(|s| Completion::simple(s.clone()))
            .collect();
        Some(completions)
    }
}
