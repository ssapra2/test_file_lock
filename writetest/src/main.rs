use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Specify the file path where you want to write the text
    let file_path = "../test.md";

    // Open the file for writing
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create file: {:?}", err);
            return;
        }
    };

    // Define the text you want to write
    let text_to_write = "Hello, world";

    // Write the text to the file
    match file.write_all(text_to_write.as_bytes()) {
        Ok(_) => println!("Successfully wrote to the file."),
        Err(err) => eprintln!("Failed to write to the file: {:?}", err),
    }
}
