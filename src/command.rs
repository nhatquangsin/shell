use std::env;
use std::path::Path;

pub fn process_cd(mut args: Vec<String>) {
    let new_dir = args.pop().map_or("/".to_string(), |x| x);
    let root = Path::new(&new_dir);

    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}
