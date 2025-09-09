use std::cmp::min;
use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    let rectangle_dimensions: Vec<i32> = lines[0]
        .split_whitespace()
        .map(|number_string|number_string.parse::<i32>().unwrap())
        .collect();

    let height = rectangle_dimensions[0];
    let width = rectangle_dimensions[1];

    for row in 0 .. height{
        for column in 0 .. width {
            let distance_to_edge = min(min(row, height - row - 1), min(column, width - column - 1)) + 1;
            if distance_to_edge < 10 {
                print!("{}", distance_to_edge);
            } else {
                print!(".")
            }
        }
        println!();
    }
}