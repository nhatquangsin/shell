use std::env;
use std::path::Path;

pub fn process_cd(args: std::str::SplitWhitespace) {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);

    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}
