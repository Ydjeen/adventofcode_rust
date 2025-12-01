use adventofcode_rust::utils::*;
use regex::Regex;
use itertools::Itertools;

pub fn get_answer_1()->String {

    let input_file = "src/advent2024/days/input/day4_input";

    let str_lines = utils::read_file_lines(&input_file.to_string());
    let mut sum = 0;
    for line_count in 0..str_lines.len()-3{
        for symbol_count in 0..str_lines[line_count].len()-3{
            let counted = count_xmases(line_count, symbol_count, &str_lines);
            sum += counted;
        }
    }
    return format!("Sum:{}", sum)
}

fn count_xmases(y:usize, x:usize, lines: &Vec<String>) -> i32{
    let mut sum = 0;
    let mut word_vertical: [char; 4] = ['a';4];
    let mut word_horizontal: [char; 4] = ['a';4];
    let mut word_top_left_to_bottom_right: [char; 4] = ['a';4];
    let mut word_top_right_to_bottom_left: [char; 4] = ['a';4];
    for i in 0..4{
        for j in 0..4{
            word_vertical[j] = lines[y+j].chars().nth(x+i).unwrap();
            word_horizontal[j] = lines[y+i].chars().nth(x+j).unwrap();  
        }
        word_top_left_to_bottom_right[i] = lines[y+i].chars().nth(x+i).unwrap();
        word_top_right_to_bottom_left[i] = lines[y+i].chars().nth(x+3-i).unwrap();
        if x == 0 {
            sum += found_xmax(word_vertical);
        } else {
            if i == 3{
                sum += found_xmax(word_vertical);
            }
        }
        if y == 0 {
            sum += found_xmax(word_horizontal);
        } else {
            if i == 3{
                sum += found_xmax(word_horizontal);
            }
        }
    }
    for w in [word_top_left_to_bottom_right, word_top_right_to_bottom_left]{
        sum += found_xmax(w);
    }
    return sum;
}

fn found_xmax(word:[char; 4]) -> i32{
    let chars =  ['X','M','A','S'];
    let chars_reversed =  ['S','A','M','X'];
    let mut sum = 0;
    if chars == word{
        sum += 1;
    }
    if chars_reversed == word{
        sum += 1;
    }
    return sum;
}

pub fn get_answer_2()->String {

    let input_file = "src/advent2024/days/input/day4_input";

    let str_lines = utils::read_file_lines(&input_file.to_string());
    let mut sum = 0;
    for line_count in 0..str_lines.len()-2{
        for symbol_count in 0..str_lines[line_count].len()-2{
            let counted = count_diag_xmases(line_count, symbol_count, &str_lines);
            sum += counted;
        }
    }

    return format!("Sum:{}", sum)
}

fn count_diag_xmases(y:usize, x:usize, lines: &Vec<String>) -> i32{
    let mut sum = 0;
    const word_len: usize = 3;
    let mut word_top_left_to_bottom_right: [char; word_len] = ['a';word_len];
    let mut word_top_right_to_bottom_left: [char; word_len] = ['a';word_len];
    for i in 0..word_len{
        word_top_left_to_bottom_right[i] = lines[y+i].chars().nth(x+i).unwrap();
        word_top_right_to_bottom_left[i] = lines[y+i].chars().nth(x+word_len-1-i).unwrap();
    }
    for w in [word_top_left_to_bottom_right, word_top_right_to_bottom_left]{
        sum += found_max(w);
    }
    if sum == 1{
        return 0;
    }
    if sum == 2{
        return 1;
    }
    return 0;
}

fn found_max(word:[char; 3]) -> i32{
    let chars =  ['M','A','S'];
    let chars_reversed =  ['S','A','M'];
    let mut sum = 0;
    if chars == word{
        sum += 1;
    }
    if chars_reversed == word{
        sum += 1;
    }
    return sum;
}



pub fn process() {
    let answer_1 = get_answer_1();
    let answer_2 = get_answer_2();
    println!("Day 4 answers: {}, {}", answer_1, answer_2);
}
