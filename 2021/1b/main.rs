use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("./input").unwrap();
    let mut lines = BufReader::new(f).lines();

    let mut num_increases: i32 = 0;

    let mut prev_by_3: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut prev_by_2: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut prev_by_1: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    let mut curr_window = prev_by_3 + prev_by_2 + prev_by_1;

    for line in lines {
        let curr_depth: i32 = line.unwrap().parse().unwrap();
        let new_window = curr_window + curr_depth - prev_by_3;

        if new_window > curr_window {
            num_increases += 1; 
        }

        curr_window = new_window;
        prev_by_3 = prev_by_2;
        prev_by_2 = prev_by_1;
        prev_by_1 = curr_depth;
    }

    println!("num_increases is {}", num_increases);
}
