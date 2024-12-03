use std::collections::HashMap;
use std::fs::read_to_string;

const INPUT: &'static str = "day_01_input.txt";

pub fn calc1() -> i32 {
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in read_to_string(INPUT).unwrap().lines() {
        let mut split = line.split_ascii_whitespace();
        col1.push(split.next().unwrap().parse::<i32>().unwrap());
        col2.push(split.next().unwrap().parse::<i32>().unwrap());
    }
    col1.sort();
    col2.sort();
    let mut sum = 0;
    for i in 0..col1.len() {
        sum += (col1[i] - col2[i]).abs();
    }

    sum
}

pub fn calc2() -> i32 {
    let mut col1: Vec<i32> = Vec::new();
    let mut appearances: HashMap<i32, i32> = HashMap::new();
    for line in read_to_string(INPUT).unwrap().lines() {
        let mut split = line.split_ascii_whitespace();
        col1.push(split.next().unwrap().parse::<i32>().unwrap());
        let col2_number = split.next().unwrap().parse::<i32>().unwrap();
        add_or_increment(&mut appearances, &col2_number);
    }
    let mut sum = 0;
    for i in 0..col1.len() {
        sum += col1[i] * appearances.get(&col1[i]).unwrap_or(&0);
    }
    sum
}

fn add_or_increment(appearances: &mut HashMap<i32, i32>, col2_number: &i32) {
    if appearances.contains_key(&col2_number) {
        appearances.insert(*col2_number, appearances.get(&col2_number).unwrap() + 1);
    } else {
        appearances.insert(*col2_number, 1);
    }
}
