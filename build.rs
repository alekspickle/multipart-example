use std::process::Command;

fn main() {
    //create 'downloads' directory
    Command::new("mkdir").arg("downloads")
                        .status().unwrap();
}