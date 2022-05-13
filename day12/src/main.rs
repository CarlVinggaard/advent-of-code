use std::fs;

fn main() {
    let filename = "./input.txt";

    let input = fs::read_to_string(filename).expect("There was a problem reading the file");

    let lines: Vec<(&str, &str)> = input.split("\n").map(|l| {
        let parts: Vec<&str> = l.split("-").collect();
        (parts[0], parts[1])
    }).collect();

    
}
