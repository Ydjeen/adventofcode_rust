use adventofcode_rust::utils::*;
use regex::Regex;
use itertools::Itertools;

pub fn get_answer_1()->String {

    let input_file = "src/advent2024/days/input/day3_input";

    let str_lines = utils::read_file_lines(&input_file.to_string());


    return format!("Sum:{}", sum)
}

pub fn get_answer_2()->String {

    let input_file = "src/advent2024/days/input/day3_input";

    let str_lines = utils::read_file_lines(&input_file.to_string());


    return format!("Sum:{}", sum)
}



pub fn process() {
    let answer_1 = get_answer_1();
    let answer_2 = get_answer_2();
    println!("Day 3 answers: {}, {}", answer_1, answer_2);
}
