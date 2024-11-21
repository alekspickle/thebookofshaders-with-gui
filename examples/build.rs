use std::fs::create_dir;

fn main() {
    //create 'output' directory
    match create_dir("outputs") {
        Ok(result) => println!("created output folder {:?}", result),
        Err(result) => println!("failed to create outputs folder {}", result),
    }
}