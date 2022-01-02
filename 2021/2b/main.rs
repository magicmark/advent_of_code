use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("./input").unwrap();
    let lines = BufReader::new(f).lines();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for line in lines {
        let movement: String = line.unwrap();
        let parts: Vec<&str> = movement.split(" ").collect();

        match parts.as_slice() {
            ["forward", val] => {
                x += val.parse::<i32>().unwrap();
                y += aim * val.parse::<i32>().unwrap();
            }
            ["up", aim_delta] => {
                aim -= aim_delta.parse::<i32>().unwrap();
            }
            ["down", aim_delta] => {
                aim += aim_delta.parse::<i32>().unwrap();
            }
            _ => {
                panic!("yikes!")
            }
        }
    }

    println!("x: {}, y: {}, x*y: {}", x, y, x * y)
}
