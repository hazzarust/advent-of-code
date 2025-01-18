use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("reports.txt").unwrap();
    let buffer = BufReader::new(file);

    let mut count = 0;

    for line in buffer.lines(){
        match line{
            Ok(line) => {
                let mut vector: Vec<u32> = vec![];
                let mut score = 0;
                for number in line.split_whitespace().map(|n| n.to_string().parse::<u32>().unwrap()){
                    vector.push(number);
                }
                if (vector.iter().is_sorted() || vector.iter().rev().is_sorted()){
                    vector.sort();
                    for (pos, num) in vector.iter().enumerate(){
                        if pos == 0{
                            if (vector[pos + 1] - vector[pos] != 1) && (vector[pos + 1] - vector[pos] != 2) && (vector[pos + 1] - vector[pos] != 3){
                                continue;
                            }
                            else{
                                score+=1;
                            }
                        }
                        else if pos == (vector.len() - 1).try_into().unwrap(){
                            if vector[pos] - vector[pos - 1] == 1 && vector[pos] - vector[pos - 1] == 2 && vector[pos] - vector[pos - 1] == 3{
                                continue;
                            }else{
                                score+=1;
                            }
                        }
                        else{
                            if vector[pos + 1] - vector[pos] == 1 || vector[pos + 1] - vector[pos] == 2 || vector[pos + 1] - vector[pos] == 3{
                                score+=1;
                            }
                        }
                    }
                    if score == vector.len(){
                        count+=1;
                        println!("LENGTH ACHIEVED: {:?}, SCORE {:?}", vector.len(), score);
                    }

                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }


    println!("{}", count);
    

    Ok(())
}


