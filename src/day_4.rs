use std::fs::read_to_string;

pub fn calc1() -> i32 {
    let matrix: Vec<Vec<char>> = read_matrix();
    word_count("XMAS", &matrix)
}

fn word_count(word: &str, matrix: &Vec<Vec<char>>) -> i32 {
    let chars = word.chars().collect::<Vec<char>>();
    let mut result = 0;
    for i in 0 .. matrix.len() {
        for j in 0 .. matrix[i].len() {
            if matrix[i][j] == chars[0] {
                if check(&chars, matrix, i, j, 1, 0) {
                    result += 1;
                }
                if check(&chars, matrix, i, j, -1, 0) {
                    result += 1;
                }
                if check(&chars, matrix, i, j, 0, 1) {
                    result += 1;
                }
                if check(&chars, matrix, i, j, 0, -1) {
                    result += 1;
                }
                if check(&chars, matrix, i, j, 1, 1) {
                    result += 1;
                }
                if check(&chars, matrix, i, j, -1, -1) {
                    result += 1;
                }
                if check(&chars, matrix, i, j, -1, 1) {
                    result += 1;
                }
                if check(&chars, matrix, i, j, 1, -1) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn check(chars: &Vec<char>, matrix: &Vec<Vec<char>>, i: usize, j: usize, dir_horizontal: i32, dir_vertical: i32) -> bool {
    if chars.len() > matrix[0].len() - j && dir_horizontal == 1 {
        return false;
    }
    if chars.len() > j + 1 && dir_horizontal == -1 {
        return false;
    }
    if chars.len() > matrix.len() - i && dir_vertical == 1 {
        return false;
    }
    if chars.len() > i + 1 && dir_vertical == -1 {
        return false;
    }
    
    for x in 0..chars.len() {
        let x_pos = j as i32 + (x as i32 * dir_horizontal);
        let y_pos = i as i32 + (x as i32 * dir_vertical);
        if matrix[y_pos as usize][x_pos as usize] != chars[x] {
            return false;
        }
    }
    true
}

fn read_matrix() -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in read_to_string("day_04_input.txt").unwrap().lines() {
        matrix.push(line.chars().collect());
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_zero_for_empty_matrix() {
        let matrix: Vec<Vec<char>> = Vec::new();
        assert_eq!(word_count("XMAS", &matrix), 0);
    }
    
    #[test]
    fn returns_one_for_horizontal() {
        let matrix: Vec<Vec<char>> = vec![vec!['X', 'M', 'A', 'S']];
        assert_eq!(word_count("XMAS", &matrix), 1);
    }

    #[test]
    fn returns_zero_for_horizontal_wrong() {
        let matrix: Vec<Vec<char>> = vec![vec!['X', 'M', 'A', 'G']];
        assert_eq!(word_count("XMAS", &matrix), 0);
    }
    
    #[test]
    fn returns_zero_for_horizontal_beyond_right_edge() {
        let matrix: Vec<Vec<char>> = vec![vec!['X', 'M', 'A']];
        assert_eq!(word_count("XMAS", &matrix), 0);
    }
    
    #[test]
    fn returns_one_for_horizontal_reversed() {
        let matrix: Vec<Vec<char>> = vec![vec!['S', 'A', 'M', 'X']];
        assert_eq!(word_count("XMAS", &matrix), 1);
    }

    #[test]
    fn returns_one_for_vertical() {
        let matrix: Vec<Vec<char>> = vec![
            vec!['X'],
            vec!['M'],
            vec!['A'],
            vec!['S'],
        ];
        assert_eq!(word_count("XMAS", &matrix), 1);
    }

    #[test]
    fn returns_one_for_vertical_reversed() {
        let matrix: Vec<Vec<char>> = vec![
            vec!['S'],
            vec!['A'],
            vec!['M'],
            vec!['X'],
        ];
        assert_eq!(word_count("XMAS", &matrix), 1);
    }
    #[test]
    fn returns_one_for_diagonal_bottom_right() {
        let matrix: Vec<Vec<char>> = vec![
            vec!['X', '.', '.', '.'],
            vec!['.', 'M', '.', '.'],
            vec!['.', '.', 'A', '.'],
            vec!['.', '.', '.', 'S'],
        ];
        assert_eq!(word_count("XMAS", &matrix), 1);
    }

    #[test]
    fn returns_one_for_diagonal_top_left() {
        let matrix: Vec<Vec<char>> = vec![
            vec!['S', '.', '.', '.'],
            vec!['.', 'A', '.', '.'],
            vec!['.', '.', 'M', '.'],
            vec!['.', '.', '.', 'X'],
        ];
        assert_eq!(word_count("XMAS", &matrix), 1);
    }

    #[test]
    fn returns_one_for_diagonal_bottom_left() {
        let matrix: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', 'X'],
            vec!['.', '.', 'M', '.'],
            vec!['.', 'A', '.', '.'],
            vec!['S', '.', '.', '.'],
        ];
        assert_eq!(word_count("XMAS", &matrix), 1);
    }
    #[test]
    fn returns_one_for_diagonal_top_right() {
        let matrix: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', 'S'],
            vec!['.', '.', 'A', '.'],
            vec!['.', 'M', '.', '.'],
            vec!['X', '.', '.', '.'],
        ];
        assert_eq!(word_count("XMAS", &matrix), 1);
    }
}