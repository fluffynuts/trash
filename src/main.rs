use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        print!("Nothing specified on the commandline to trash! Bailing out.");
        std::process::exit(1);
    }

    if !ensure_trash_dir_exists() {
        print!("Unable to ensure trash dir exists. Bailing out.");
    }

    for path in &args {
        let meta = fs::metadata(path);
        if !meta.is_ok() {
            print!("Unable to find {}", path);
            continue;
        }
        let metadata = meta.unwrap();
        if metadata.is_file() {
            try_remove_file(path)
        } else if metadata.is_dir() {
            try_remove_dir(path)
        } else {
            print!("Can't remove {}: not a file or dir", path);
        }
    }
}

fn try_remove_file(path: &String) {
    let result = fs::remove_file(path);
    if !result.is_ok() {
        print!("Unable to remove file {}: {:?}", path, result.err());
    }
}

fn try_remove_dir(path: &String) {
    let result = fs::remove_dir_all(path);
    if !result.is_ok() {
        print!("Unable to remove dir {}: {:?}", path, result.err());
    }
}

fn ensure_trash_dir_exists() -> bool {
    let target = "C:\\.trash";
    let meta = fs::metadata(target);
    if meta.is_ok() {
        return true;
    }
    let result = fs::create_dir(target);
    return result.is_ok();
}
