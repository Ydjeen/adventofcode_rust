use adventofcode_rust::utils::*;

pub fn get_answer()->String {
    let input_file = "src/advent2024/days/day1_input";

    let str_lines = utils::read_file_lines(&input_file.to_string());
    let int_lines: Vec<(i32, i32)> = str_lines.into_iter().map(|line| utils::get_int_tuple_from_line(&line)).collect();
    let mut dist_sum = 0;
    let mut left:Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();
    for line in int_lines{
        left.push(line.0);
        right.push(line.1);
    }
    left.sort();
    right.sort();
    
    for i in 0..left.len() {
        let dist = (left[i] - right[i]).abs();
        println!("{} {} {}",left[i] , right[i], dist);
        dist_sum += dist;
    }

    let mut similarity = 0;
    let mut left_pointer = 0;
    let mut right_pointer = 0;
    while left_pointer < left.len(){
        let current_number = left[left_pointer];
        let mut current_number_occurencies: i32 = 0;
        while left_pointer < left.len() && left[left_pointer] == current_number{
            current_number_occurencies += 1;
            left_pointer += 1;
        }

        let mut right_number_occurencies = 0;
        while right_pointer < right.len() && right[right_pointer] <= current_number{
            if right[right_pointer] == current_number{
                right_number_occurencies += 1;
            }
            right_pointer += 1;
        }
        println!("left_num {} left occ {} right occ {}0", current_number, current_number_occurencies, right_number_occurencies);
        similarity += current_number * right_number_occurencies * current_number_occurencies;
    }

    return format!("Dist_sum:{}\nSimilarity: {}", dist_sum, similarity)
}



pub fn process() {
    let answer = get_answer();
    println!("Day 1 answer: {}", answer);
}
