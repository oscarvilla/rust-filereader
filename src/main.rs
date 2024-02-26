use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    // you can run with cargo run -- "sample.txt", 
    // or cargo run -- sample.txt to read and show the text in sampl.txt file
    let args: Vec<String> = env::args().collect();
    let my_file_path = &args[1];
    println!("The passed in file path: {}", my_file_path);
    let file = File::open(my_file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}