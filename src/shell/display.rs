use ansi_term::Color;

use config::Configuration;

pub(super) struct Display {
    colors_disabled: bool,
}

impl Display {
    pub fn new(config: Option<&Configuration>) -> Self {
        match config {
            None => Display { colors_disabled: false },
            Some(c) => Display { colors_disabled: c.colors_disabled },
        }
    }

    pub fn show(&self, s: &str) {
        if self.colors_disabled {
            println!("{}", s);
        } else {
            let style = Color::Red.bold();
            println!("{}", style.paint(s));
        }
    }
}

