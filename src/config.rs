use clap::{
    App,
    Arg,
};

pub struct Configuration {
    no_colors: bool,
}

impl Configuration {
    pub fn new() -> Self {
        let matches = App::new("ya2d2")
            .version("0.1")
            .author("Abhijat Malviya")
            .about("yet another to-do droid")
            .arg(Arg::with_name("no_colors").long("no-color").help("disable colors in the shell"))
            .get_matches();

        Configuration { no_colors: matches.is_present("no_colors") }
    }
}
