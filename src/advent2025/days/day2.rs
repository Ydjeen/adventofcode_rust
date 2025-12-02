use std::collections::btree_map::Range;

use advent_of_code::utils::{utils::get_int_tuple_from_line, *};

pub fn get_answer()->String {
    let input_file = "src/advent2025/days/input/day2_input";

    let mut lines = utils::read_file_comma_separated_lines(&input_file.to_string());
    let mut counter = 0;
    for line in lines{
        println!("parsing {line}");
        let (bottom, top) = get_int_tuple_from_line(&line, "-");
        counter += get_illegals_in_range_sequence(bottom, top);
    }
    return format!("sum:{}\n", counter)
}

pub fn get_illegals_in_range_tuple(bottom:i64, top:i64)->i64 {
    let mut result = 0;
    for i in bottom..top{
        let mut legal = true;
        let digits_amount = i.checked_ilog10().unwrap_or(0) + 1;
        if digits_amount & 1 == 1 {
            continue;
        }
        let mut left = i / i64::pow(10,digits_amount / 2);
        let mut right = i % i64::pow(10,digits_amount / 2);
        if left == right {
            legal == false;
            result += i;
            println!("illegal: {i}");
        }
    }
    return (result);
}

pub fn get_illegals_in_range_sequence(bottom:i64, top:i64)->i64 {
    let mut result = 0;
    for i in bottom..top{
        if (i==20202){
            print!("wow");
        }
        let mut legal = false;
        let digits_amount = i.checked_ilog10().unwrap_or(0) + 1;
        for order in 1..((digits_amount/2)+1){
            let mut legal = false;
            let rest = i % i64::pow(10,order);
            if rest.checked_ilog10().unwrap_or(0) + 1 < order{
                continue;
            }
            let mut investigated = i;
            while investigated > 0{
                let investigated_rest = investigated % i64::pow(10,order);
                if rest != investigated_rest{
                    legal = true;
                    break;
                }
                investigated = investigated / i64::pow(10,order);
            }
            if investigated == 0 && legal == false{
                result += i;
                println!("illegal: {i}");
                break;
            }

        }
    }
    return (result);
}




pub fn process() {
    let answer = get_answer();
    println!("Day 1 answer: {}", answer);
}
