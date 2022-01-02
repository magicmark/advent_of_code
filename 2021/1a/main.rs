use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("./input").unwrap();
    let lines = BufReader::new(f).lines();

    let mut num_increases: i32 = 0;
    let mut previous_depth: Option<i32> = None;

    for line in lines {
        let curr_depth: i32 = line.unwrap().parse().unwrap();
        
        if previous_depth != None {
            if curr_depth > previous_depth.unwrap() {
                num_increases += 1;
            }
        }

        previous_depth = Some(curr_depth);    
    }
   
    println!("num_increases is {}", num_increases);
}
