use std::fs::read_to_string;

const INPUT: &'static str = "day_02_input.txt";

pub fn calc1() -> i32 {
    let mut safe_reports_num = 0;
    for line in read_to_string(INPUT).unwrap().lines() {
        if is_safe_report(line) {
            safe_reports_num += 1;
        }
    }
    safe_reports_num
}

pub fn calc2() -> i32 {
    let mut safe_reports_num = 0;
    for line in read_to_string(INPUT).unwrap().lines() {
        if is_safe_report_perms(line) {
            safe_reports_num += 1;
        }
    }
    safe_reports_num
}

fn is_safe_report(line: &str) -> bool {
    let split = line.split_whitespace();
    if line.is_empty() {
       return false
    }
    if split.clone().count() == 1 {
        return true
    }
    let numbers: Vec<i32> = split.map(|x| x.parse::<i32>().unwrap()).into_iter().collect();
    is_valid(&numbers)
}

fn is_valid(numbers: &Vec<i32>) -> bool {
    let is_rising = numbers[0] < numbers[1];
    for i in 1..numbers.len() {
        if is_rising && numbers[i] < numbers[i - 1] || !is_rising && numbers[i] > numbers[i - 1] {
            return false
        }
        if numbers[i] == numbers[i - 1] {
            return false
        }
        if (numbers[i] - numbers[i - 1]).abs() > 3 {
            return false
        }
    }
    true
}

fn is_safe_report_perms(line: &str) -> bool {
    let split = line.split_whitespace();
    if line.is_empty() {
        return false
    }
    if split.clone().count() == 1 {
        return true
    }
    let numbers: Vec<i32> = split.map(|x| x.parse::<i32>().unwrap()).into_iter().collect();
    let mut permutations: Vec<Vec<i32>> = Vec::new();
    permutations.push(numbers.clone());
    for i in 0..numbers.len() {
        let mut permutation = numbers.clone();
        permutation.remove(i);
        permutations.push(permutation);
    }
    permutations.iter().any(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_true_when_single_number() {
        assert_eq!(true, is_safe_report("1"));
    }

    #[test]
    fn returns_false_when_empty_line() {
        assert_eq!(false, is_safe_report(""));
    }

    #[test]
    fn returns_true_when_monotonically_increasing() {
        assert_eq!(true, is_safe_report("1 2 3 4 5"));
    }

    #[test]
    fn returns_true_when_monotonically_decreasing() {
        assert_eq!(true, is_safe_report("5 4 3 2 1"));
    }

    #[test]
    fn returns_false_when_not_monotonically_increasing() {
        assert_eq!(false, is_safe_report("1 2 3 7 5"));
    }

    #[test]
    fn returns_false_when_not_monotonically_decreasing() {
        assert_eq!(false, is_safe_report("9 5 3 16 18"));
    }

    #[test]
    fn returns_false_when_distance_is_zero() {
        assert_eq!(false, is_safe_report("1 2 3 4 4"));
    }

    #[test]
    fn returns_false_when_distance_is_greater_than_3() {
        assert_eq!(false, is_safe_report("1 2 3 12 13"));
        assert_eq!(false, is_safe_report("31 30 20 19"));
    }

    #[test]
    fn returns_true_if_bad_value_removed() {
        assert_eq!(true, is_safe_report_perms("1 2 3 7 5"));
    }
}