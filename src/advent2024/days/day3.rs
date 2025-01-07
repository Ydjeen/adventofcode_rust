use adventofcode_rust::utils::*;
use regex::Regex;
use itertools::Itertools;

pub fn get_answer_1()->String {

    let input_file = "src/advent2024/days/input/day3_input";

    let str_lines = utils::read_file_lines(&input_file.to_string());

    let re = Regex::new(r"mul\((\d)+\,(\d)+\)").unwrap();
    let mut sum = 0;
    for line in str_lines{
        let findings: Vec<_> = re.find_iter(line.as_str()).map(|m| m.as_str()).collect();
        for finding in findings{
            let (r, b) = finding.get(4..finding.len()-1).unwrap().split(",").map(|number|number.parse::<i32>().unwrap()).collect_tuple().unwrap();
            sum += r * b;
        }
    }

    return format!("Sum:{}", sum)
}

pub fn get_answer_2()->String {

    let input_file = "src/advent2024/days/input/day3_input";

    let str_lines = utils::read_file_lines(&input_file.to_string());
    let mut enabled = true;

    let re = Regex::new(r"(do\(\))|(don\'t\(\))|(mul\((\d)+\,(\d)+\))").unwrap();
    let enable = "do()";
    let dont = "don't()";
    let mut sum = 0;
    for line in str_lines{
        let findings: Vec<_> = re.find_iter(line.as_str()).map(|m| m.as_str()).collect();
        for finding in findings{
            if finding.to_lowercase().eq(enable){
                enabled = true;
            } else if finding.to_lowercase().eq(dont) {
                enabled = false;
            } else{
                println!("{}",finding);
            let (r, b) = finding.get(4..finding.len()-1).unwrap().split(",").map(|number|number.parse::<i32>().unwrap()).collect_tuple().unwrap();
            if enabled {
                sum += r * b;
            }
            }
        }
    }

    return format!("Sum:{}", sum)
}



pub fn process() {
    let answer_1 = get_answer_1();
    let answer_2 = get_answer_2();
    println!("Day 3 answers: {}, {}", answer_1, answer_2);
}
