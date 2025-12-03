use std::collections::btree_map::Range;

use advent_of_code::utils::{utils::{get_digit_array_from_line, count_digits}, *};

pub fn get_answer()->String {
    let input_file = "src/advent2025/days/input/day3_input";

    let mut lines = utils::read_file_lines(&input_file.to_string());
    let mut counter = 0; 
    let digit_lines: Vec<Vec<i32>> = lines.iter().map(get_digit_array_from_line).collect();
    for digit_line in digit_lines{
        let value = get_max_sequence(&digit_line,12);
        counter += value;
        println!("{value}")
    }
    return format!("sum:{}\n", counter)
}

pub fn get_max_pair(line:Vec<u64>)->u64 {
    let max1_index = line
        .iter()
        .enumerate()
        .max_by(|(x, a), (y, b)| a.cmp(b).then(x.cmp(y).reverse()))
        .map(|(index, _)| index).unwrap();
    let max2_index;
    if max1_index == line.len()-1{
        max2_index = line
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != max1_index).map(|(_, v)| v)
        .enumerate()
        .max_by(|(x, a), (y, b)| a.cmp(b).then(x.cmp(y).reverse()))
        .map(|(index, _)| index).unwrap();
    } else{
        max2_index = line
        .iter()
        .skip(max1_index+1)
        .enumerate()
        .max_by(|(x, a), (y, b)| a.cmp(b).then(x.cmp(y).reverse()))
        .map(|(index, _)| index).unwrap();
    }
    let mut result = 0;
    if max1_index == line.len()-1{
        result = line[max2_index]*10 + line[max1_index];
    } else{
        result = line[max1_index]*10 + line[max2_index+max1_index+1];
    }
    println!("result {result}");
    return result;
}

pub fn get_max_sequence(line:&[i32], size:usize)->u64 {
    let max_index = line
        .iter()
        .rev()
        .skip(size as usize -1)
        .enumerate()
        .max_by(|(x, a), (y, b)| a.cmp(b).then(x.cmp(y)))
        .map(|(index, _)| index).unwrap();
    let true_index : usize = line.len() - max_index - (size);
    if size==1{
        return line[true_index] as u64;
    } else{
        let sequence = get_max_sequence(&line[true_index+1..line.len()], size-1);
        return sequence + (line[true_index] as u64 * u64::pow(10, count_digits(sequence)));
    }
}


pub fn process() {
    let answer = get_answer();
    println!("Day 3 answer: {}", answer);
}
