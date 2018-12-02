use clap::{
    App,
    Arg,
};

pub struct Configuration {
    pub colors_disabled: bool,
}

impl Configuration {
    pub fn new() -> Self {
        let matches = App::new("ya2d2")
            .version("0.1")
            .author("Abhijat Malviya")
            .about("yet another to-do droid")
            .arg(Arg::with_name("no_colors").long("no-color").help("disable colors in the shell"))
            .get_matches();

        Configuration { colors_disabled: matches.is_present("no_colors") }
    }

    #[allow(dead_code)]
    pub fn default() -> Self {
        Configuration { colors_disabled: false }
    }
}
