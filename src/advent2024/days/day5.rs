use std::ops::Index;

use adventofcode_rust::utils::*;
use regex::Regex;
use itertools::{all, Itertools};


pub fn get_answer_1()->String {

    let input_file = "src/advent2024/days/input/day5_input";

    let mut str_lines = utils::read_file_lines(&input_file.to_string());

    let mut sort: Vec<i32> = Vec::new();
    let mut not_sort: Vec<(i32, i32)> =  Vec::new();

    let mut splitter = 0;
    for i in 0..str_lines.len(){
        if str_lines[i].is_empty() || str_lines[i] == "\r"{
            splitter = i;
            break;
        }
    }
    loop{
        not_sort = Vec::new();
        for i in 0..splitter{
            let line = str_lines.get(i).unwrap();
            let (left, right) = line.split("|")
            .map(|number| number.trim().parse::<i32>().unwrap())
            .collect_tuple().unwrap();
            let added = fill_sorts(&mut sort, left, right);
            if !added {
                not_sort.push((left, right));
            } 
            println!("{:?}", sort);
        }
        if not_sort.is_empty(){
            break;
        }
    }
    loop{
        let mut added = false;
        for i in 0..splitter{
            let line = str_lines.get(i).unwrap();
            let (left, right) = line.split("|")
            .map(|number| number.trim().parse::<i32>().unwrap())
            .collect_tuple().unwrap();
            added = added | fill_sorts(&mut sort, left, right);
        }
        println!("{:?}", sort);
        if !added{
            break;
        }
    }
    println!("{:?}", sort);
    while !not_sort.is_empty(){
        let mut to_remove:Vec<usize> = Vec:: new();
        for i in 0..not_sort.len(){
            let (left, right) = not_sort.get(i).unwrap();
            let added = fill_sorts(&mut sort, *left, *right);
            if added{
                to_remove.push(i);
            }
        }
        for i in (0..to_remove.len()).rev(){
            not_sort.remove(to_remove[i]);
        }
        
    }
    let mut sum = 0;
    for i in splitter+1..str_lines.len(){
        print!("Array: {};", str_lines.get(i).unwrap());
        let int_line = utils::get_int_array_from_line_with_comas(str_lines.get(i).unwrap());
        let s = sort.is_sorted_by(|a, b| sort.iter().position(|&x| x == *a) < sort.iter().position(|&x| x == *b));
        let mut sorted = true;
        for i in 0..(int_line.len()-1){
            let first = int_line.get(i).unwrap();
            let second = int_line.get(i+1).unwrap();
            let first_position = sort.iter().position(|&x| x == *first).unwrap();
            let second_position = sort.iter().position(|&x| x == *second).unwrap();
            print!(" {}<{} ", first_position, second_position);
            if first_position > second_position{
                sorted = sorted && false;
            }
        }
        println!("sorted: {}", sorted);
        if sorted{
            sum = sum + (int_line.get(int_line.len()/2).unwrap());
        }
    }

    return format!("Sum: {}", sum)
}

fn fill_sorts(sort: &mut Vec<i32>, left: i32, right: i32) -> bool{
    let mut added = false;

    let mut left_index = usize::MIN;
    let mut left_found = false;
    let mut right_index = usize::MAX;
    let mut right_found = false;
    if sort.is_empty(){
        added = true;
        sort.push(left);
        sort.push(right);
        return true;
    }
    for index in 0..sort.len(){
        if sort[index] == left{
            left_index = index;
            left_found = true;
        }
        if sort[index] == right{
            right_index = index;
            right_found = true;
        }
    }
    if !left_found && !right_found{
        return false;
    }
    if left_found && right_found {
        if left_index > right_index{
            sort[left_index] = right;
            sort[right_index] = left;
            return true;
        }
        return true;
    }

    if left_found & !right_found{
        sort.insert(left_index+1, right);
        return true;
    }
    if right_found & !left_found{
        sort.insert(right_index, left);
        return true;
    }
    return false;
}

pub fn get_answer_2()->String {

    let input_file = "src/advent2024/days/input/test_input";

    let str_lines = utils::read_file_lines(&input_file.to_string());

    return format!("Sum:")
}

pub fn process() {
    let answer_1 = get_answer_1();
    let answer_2 = get_answer_2();
    println!("Day 4 answers: {}, {}", answer_1, answer_2);
}
