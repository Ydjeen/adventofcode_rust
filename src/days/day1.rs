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