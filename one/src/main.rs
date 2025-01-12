use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    
    let file = File::open("locations.txt").unwrap();
    let reader = BufReader::new(file);
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in reader.lines(){
        match line{
            Ok(line) =>{
                for (count, words) in line
                .split_whitespace()
                .map(|sentence|{
                    sentence.parse::<u32>().unwrap()
                })
                .enumerate()
                {
                    if count == 0{
                        left.push(words);
                        left.sort();
                    }
                    else{
                        right.push(words);
                        right.sort();
                    }
                }
            }
            Err(e) => {
                println!("Trouble reading from line: {}", e);
            }
        }
    }

    println!{"Left: {:?}, Right{:?}", left, right};
}

