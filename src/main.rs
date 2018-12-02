extern crate linefeed;
extern crate md5;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


mod cmd;
mod task;
mod shell;
mod initialize;


fn main() {
    match initialize::database() {
        Ok(tree) => {

            let reader = initialize::reader()
                .expect("Failed to build shell reader");

            let result = shell::start_shell(&tree, &reader);
            if let Err(err) = result {
                eprintln!("failed with error = {:?}", err);
            }
        }
        Err(err) => {
            eprintln!("failed with error = {:?}", err);
        }
    }
}
