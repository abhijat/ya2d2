extern crate ansi_term;
extern crate clap;
extern crate linefeed;
extern crate md5;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use config::Configuration;

mod cmd;
mod task;
mod shell;
mod config;
mod initialize;


fn main() {
    let config = Configuration::new();

    match initialize::database() {
        Ok(tree) => {
            let reader = initialize::reader()
                .expect("Failed to build shell reader");

            let result = shell::start_shell(&tree, &reader, Some(&config));
            if let Err(err) = result {
                eprintln!("failed with error = {:?}", err);
            }
        }
        Err(err) => {
            eprintln!("failed with error = {:?}", err);
        }
    }
}
