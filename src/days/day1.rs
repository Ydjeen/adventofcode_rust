use crate::utils;

const INPUT:&str = "res/input1";


pub(crate) fn task1(){
    let file_content = utils::read_file_lines(INPUT.to_owned());
    let lines = file_content.split("\n");
    let mut sum = 0;
    for line in lines {
        let number = get_number_from_line(line);
        sum = sum + number;
    }
    println!("Sum is equal to:{}", sum);

    println!("Task1 executed")
}

pub(crate) fn task2(){
    let file_content = utils::read_file_lines(INPUT.to_owned());
    let lines = file_content.split("\n");
    let mut sum = 0;
    for line in lines {
        let number = get_dig_and_str_number_from_line(line);
        sum = sum + number;
    }
    println!("Sum is equal to:{}", sum);

    println!("Task2 executed")
}


fn get_number_from_line(line: &str) -> u32{
    let mut first_char = 0;
    let mut init = false;
    let mut last_char = 0;
    for c in line.chars() { 
        if c.is_numeric(){
            if !init{
                init = true;
                first_char = 10 * c.to_digit(10).unwrap();
            }
            last_char = c.to_digit(10).unwrap();
        }
    }
    return first_char + last_char;
}

const DIGIT_WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_dig_and_str_number_from_line(line: &str) -> u32{
    let mut first_initialized = false;
    let mut found_digit_check: bool = false;
    let mut first_char = 0;
    let mut last_char = 0;
    let mut found_digit = 0;
    for (i, c) in line.chars().enumerate() { 
        found_digit_check = false;
        if c.is_numeric(){
            found_digit = c.to_digit(10).unwrap();
            found_digit_check = true;
        } else{
            let line_slice = &line[i..];
            for (i, word) in DIGIT_WORDS.iter().enumerate(){
                if line_slice.starts_with(word){
                    found_digit = (i as u32)+1;
                    found_digit_check = true;
                }
            }
        }
        if found_digit_check{
            if !first_initialized{
                first_char = 10 * found_digit;
                first_initialized = true;
            }
            last_char = found_digit;
        }
    }
    println!("{line}, {first_char}, {last_char}");
    return first_char + last_char;
}