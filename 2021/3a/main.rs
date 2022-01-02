use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let f = File::open("./input")?;
    let lines = BufReader::new(f).lines();
    let mut num_lines = 0;

    // at each position, store a rolling count of the number of 1s seen
    let mut ones_count: Vec<i32> = Vec::new();

    for line in lines {
        num_lines += 1; // vomit, surely there has to be a better way to do this

        let diagnostic: &str = &line.unwrap();

        // one time initialization for the vector size
        if ones_count.is_empty() {
            ones_count.append(&mut vec![0; diagnostic.len()]);
        }

        for (idx, char) in diagnostic.chars().enumerate() {
            if char == '1' {
                ones_count[idx] += 1;
            }
        }
    }

    /* the following doesn't work cos i32 is fixed size, but was a good attempt...
    let gamma_binary_str = ones_count
        .iter()
        .map(|&num_ones| if num_ones > num_lines / 2 { 1 } else { 0 })
        .map(|val| val.to_string() )
        .collect::<Vec<_>>().join("");

    let gamma = i32::from_str_radix(&gamma_binary_str, 2).unwrap();
    let epsilon = !gamma;
    */

    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();

    for num_ones in ones_count.iter() {
        if num_ones > &(&num_lines / 2) {
            gamma_str.push('1');
            epsilon_str.push('0')
        } else {
            gamma_str.push('0');
            epsilon_str.push('1');
        }
    }

    let gamma = i32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_str, 2).unwrap();
    // println!("gamma: {:0>8b}, episilon: {:0>8b}", gamma, epsilon);
   
    println!("gamma: {}, episilon: {}, result: {}", gamma, epsilon, gamma * epsilon);
    Ok(())
}
