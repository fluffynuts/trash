use std::env;
use std::fs;
use glob::{glob, Paths};
use std::error::Error;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        print!("Nothing specified on the commandline to trash! Bailing out.");
        std::process::exit(1);
    }
    let started = SystemTime::now();
    let mut removed = 0;

    for path in &args {
        match glob(path) {
            Ok(globs) => {
                removed += try_trash_by_globs(globs);
            }
            Err(e) => print_error(&e)
        }
    }

    let elapsed = started.elapsed().expect("should be able to read elapsed time");
    let ms: f64 = elapsed.as_millis() as f64;
    let seconds= ms / 1000.0;
    println!(
        "removed {} item{} in {:.2}s",
        removed,
        if removed == 1 { "" } else { "s" },
        seconds
    );
}

fn try_trash_by_globs(globs: Paths) -> i32 {
    let mut trashed = 0;
    for g in globs {
        match g {
            Ok(p) => match try_trash(&p.to_str().unwrap().to_owned()) {
                Ok(_) => trashed += 1,
                Err(_) => {}
            }
            Err(e) => print_error(&e)
        }
    }
    return trashed;
}

fn print_error(err: &dyn Error) {
    println!("{:?}", err);
}

fn try_trash(path: &String) -> Result<(), String> {
    let meta = fs::metadata(path);
    if !meta.is_ok() {
        println!("Unable to find {}", path);
        return Ok(());
    }
    let metadata = meta.unwrap();
    return if metadata.is_file() {
        println!("remove file: {}", path);
        try_remove_file(path)
    } else if metadata.is_dir() {
        println!("remove dir:  {}", path);
        try_remove_dir(path)
    } else {
        Err(format!("Can't remove {}: ", path))
    };
}

fn try_remove_file(path: &String) -> Result<(), String> {
    let result = fs::remove_file(path);
    return match result {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Unable to remove file {}: {:?}", path, e))
    };
}

fn try_remove_dir(path: &String) -> Result<(), String> {
    let result = fs::remove_dir_all(path);
    return match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e))
    };
}

