use std::{io};
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    let vector_length = (lines[0].parse::<usize>().unwrap() + 1) / 2;

    let mut vector_numbers: Vec<i32> = lines[1]
        .split_whitespace()
        .map(|number_string|number_string.parse::<i32>().unwrap())
        .collect();
    
    vector_numbers.sort();
    
    let amount: i32 = vector_numbers.iter()
        .rev()
        .take(vector_length)
        .sum();

    eprintln!("Kattis print this comment to an irrelevant stream!");
    println!("{amount}");
}