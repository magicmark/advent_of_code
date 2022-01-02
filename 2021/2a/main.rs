use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("./input").unwrap();
    let lines = BufReader::new(f).lines();

    let mut x = 0;
    let mut y = 0;

    for line in lines {
        let movement: String = line.unwrap();
        let parts: Vec<&str> = movement.split(" ").collect();

        match parts.as_slice() {
            ["forward", x_delta] => {
                x += x_delta.parse::<i32>().unwrap();
            }
            ["up", y_delta] => {
                y -= y_delta.parse::<i32>().unwrap();
            }
            ["down", y_delta] => {
                y += y_delta.parse::<i32>().unwrap();
            }
            _ => {
                panic!("yikes!")
            }
        }
    }

    println!("x: {}, y: {}, x*y: {}", x, y, x * y)
}
