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
    
}

pub fn find_difference(left: Vec<u32>, right: Vec<u32>) -> Vec<u32>{
    let vector: Vec<u32> = left.iter().zip(right.iter()).map(|x|{
        let mut y = 0;
        if x.0 > x.1{
            y = x.0 - x.1;
        }else if x.0 < x.1{
            y = x.1 - x.0;
        }else{
        }
        y
    })
    .collect();
    
    vector
}

pub fn total_distance(vector: Vec<u32>) -> u32{
    let mut number = 0;
    for x in vector{
        number+= x;
    }
    number
}

pub fn how_many_times(left: Vec<u32>, right: Vec<u32>) -> u32{
    let mut how_many: Vec<(u32, u32)> = vec![];
    for (position, l) in left.iter().enumerate(){
        how_many.push((*l, 0));
        for r in &right{ 
            if r == l{
                how_many[position].1 += 1;
            }
        }
    };

    let mut number = 0;
    for x in how_many.iter().map(|x| x.0 * x.1){
        number+= x;
    }
    number
}

