extern crate file_lock;

use file_lock::{FileLock, FileOptions};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Duration;

fn main() {
    let should_we_block = true;
    let lock_for_writing = FileOptions::new().write(true).create(true);

    let mut filelock = match FileLock::lock("../test.md", should_we_block, lock_for_writing) {
        Ok(lock) => lock,
        Err(err) => panic!("Error getting write lock: {}", err),
    };

    //filelock.file.write_all(b"Hello, World!").unwrap();
    std::thread::sleep(Duration::from_secs(30));

    match filelock.unlock() {
        Ok(_) => println!("Successfully unlocked the file"),
        Err(err) => panic!("Error unlocking the file: {}", err),
    };
}
