use std::fs::read_to_string;
use regex::Regex;

const INPUT: &'static str = "day_03_input.txt";

pub fn calc1() -> i32 {
    let mut result = 0;
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let input = read_to_string(INPUT).unwrap();
    re.find_iter(input.as_str())
        .map(|x| x.as_str())
        .map(do_mult)
        .for_each(|x| result += x);
    result
}

fn do_mult(x: &str) -> i32 {
    let a: i32 = x[4..x.find(",").unwrap()].parse().unwrap();
    let b: i32 = x[x.find(",").unwrap() + 1..x.find(")").unwrap()].parse().unwrap();
    a * b
}

pub fn calc2() -> i32 {
    let mut result = 0;
    let muls = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let dos = Regex::new(r"do(n't)?\(\)").unwrap();
    let input = read_to_string(INPUT).unwrap();
    let mut all_tokens = Vec::new();
    all_tokens.append(&mut extract_tokens(muls, &input));
    all_tokens.append(&mut extract_tokens(dos, &input));
    all_tokens.sort_by(|a, b| a.0.cmp(&b.0));

    let mut enabled = true;
    for token in all_tokens {
        match token.1 {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => if enabled { result += do_mult(token.1) },
        }
    }

    result
}

fn extract_tokens(re: Regex, input: &String) -> Vec<(usize, &str)> {
    re.find_iter(input.as_str())
        .map(|m| {
            let start_pos = m.start();
            let token = m.as_str();
            (start_pos, token)
        })
        .collect()
}