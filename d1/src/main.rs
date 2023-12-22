use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn part1(contents: &String) -> i32 {
    let mut left = -1;
    let mut right = -1;
    let mut sum = 0;

    for line in contents.lines() {
        for ch in line.chars() {
            if ch.is_numeric() {
                if left == -1 {
                    left = ch as i32 - '0' as i32;
                }
                right = ch as i32 - '0' as i32;
            }
        }

        sum += (left * 10) + right;
        left = -1;
    }
    sum
}

fn part2(contents: &String) -> i32 {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <filename>", args[0]);
    }

    let path = Path::new(&args[1]);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(e) => panic!("Error: {}", e),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(e) => panic!("Couldn't read {}: {}", display, e),
        Ok(_) => (),
    }

    let sum = part1(&contents);
    

    println!("{}", sum);
}
