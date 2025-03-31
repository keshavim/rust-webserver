use database::Database;
use std::env;

//add arguments
fn main() {
    let args: Vec<_> = env::args().collect();

    let mut database = Database::load();
    if args[1] == "add" {
        for path in args.iter().skip(2) {
            let _ = database
                .add(path)
                .inspect_err(|e| eprintln!("failed to add {path}: {e} "));
        }
    } else if args[1] == "remove" {
        for path in args.iter().skip(2) {
            let _ = database
                .remove(path)
                .inspect_err(|e| eprintln!("failed to add {path}: {e} "));
        }
    } else if args[1] == "clear" {
        let _ = database
            .clear()
            .inspect_err(|e| eprintln!("failed to clear: {e} "));
        println!("database cleared")
    } else if args[1] == "help" {
        println!();
        Database::help();
    } else {
        eprintln!("invalid args\n");
        Database::help();
    }
}
