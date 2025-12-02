use std::fs;

pub fn read_file_string(file_path:&String)->String{
    let contents = fs::read_to_string(file_path).expect(&format!("File {} not found.", file_path));
    return contents;
}

pub fn read_file_lines(file_path:&String)->Vec<String>{
    let lines:Vec<String> = fs::read_to_string(file_path).expect(&format!("File {} not found.", file_path))
    .split("\n")
    .map(|line| line.trim().to_string())
    .collect();
    return lines;
}

pub fn read_file_comma_separated_lines(file_path:&String)->Vec<String>{
    let lines:Vec<String> = fs::read_to_string(file_path).expect(&format!("File {} not found.", file_path))
    .split(",")
    .map(|line| line.to_string())
    .collect();
    return lines;
}

pub fn get_int_tuple_from_line(line: &str, splitter: &str) -> (i64, i64){
    let line_iterator:Vec<&str> = line.split(splitter).collect();
    let first = line_iterator[0].parse::<i64>().unwrap();
    let second = line_iterator[1].parse::<i64>().unwrap();
    return (first, second)
}

pub fn get_int_array_from_line(line: &str) -> Vec<i32>{
    let line_iterator = line.split_whitespace().map(|x|x.parse::<i32>().unwrap()).collect();
    return line_iterator
}

pub fn get_int_array_from_line_with_comas(line: &str) -> Vec<i32>{
    let line_iterator = line.split(',').map(|x|x.parse::<i32>().unwrap()).collect();
    return line_iterator
}

pub fn get_str_array_from_line_with_comas(line: &str) -> Vec<&str>{
    let line_iterator = line.split(',').collect();
    return line_iterator
}


pub fn find_smallest_pair(input_vec: &Vec<(i32, i32)>, min_value_tuple: (i32, i32))->(i32, i32){
    //let mut left_found = std::i32::MAX;
    //let mut right_found: i32 = std::i32::MAX;
    let mut left_found = min_value_tuple.0;
    let mut right_found = min_value_tuple.1;
    let mut found = false;
    for pair in input_vec{
        if left_found == min_value_tuple.0{
            if pair.0 > min_value_tuple.0{
                left_found = pair.0;
            }
        } else {
            if pair.0 < left_found && pair.0 > min_value_tuple.0{
                left_found = pair.0;
            }
        }
        if right_found == min_value_tuple.1{
            if pair.1 > min_value_tuple.1{
                right_found = pair.1;
            }
        } else {
            if pair.1 < right_found && pair.1 > min_value_tuple.1{
                right_found = pair.1;
            }
        }
    }
    if left_found != std::i32::MAX && right_found != std::i32::MAX {
        return (left_found, right_found)
    }
    else {
        return min_value_tuple;
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}