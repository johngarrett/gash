use std::env;
use std::thread;
use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;

fn main() {
    let file_name = "test-name";
    let gash_dir = "/Users/garrepi/temp";

    gash_open(file_name, gash_dir);
}

fn gash_open(file: &str, dir: &str) {
    println!("{}: {}", file, dir);
    let path = Path::new(dir);
    assert!(
        env::set_current_dir(&path).is_ok()
    );

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(10));
        println!("its been 10 seconds!");
    });
}

