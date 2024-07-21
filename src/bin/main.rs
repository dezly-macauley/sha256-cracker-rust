use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use sha2::{Sha256, Digest};

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please enter one arguement");
        println!("E.g. Cargo run <sha256 hash>");
        exit(1);
    }

    let wanted_hash: &String = &args[1];

    let password_file: &str = "src/rockyou.txt";

    let mut attempts: i32 = 1;

    println!("Attempting to hack {}", wanted_hash);

    let password_list: File = File::open(password_file).unwrap();
    
    let reader = BufReader::new(password_list);

    for line in reader.lines() {

        let line: String = line.unwrap();
        let password = line.trim().to_owned().into_bytes();

        let password_hash: String = format!("{:x}", Sha256::digest(&password));

        println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);

        if & password_hash == wanted_hash {
            println!("Password hash found after {} attempts {} hashes to {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
            exit(0);
        }
        attempts += 1;

    } 
    
    println!("Password hash not found.");

}
