use advent_of_code::utils::*;

pub fn get_answer()->String {
    let input_file = "src/advent2025/days/input/day1_input";

    let mut str_lines = utils::read_file_lines(&input_file.to_string());
    let mut counter = 0;
    let mut dist_sum = 50;
    for x in str_lines.into_iter() {
        let mut rotations;
        (dist_sum, rotations)=sum_RL(dist_sum, &x);
        counter += rotations
    }
    return format!("sum:{}\n", counter)
}

pub fn sum_RL(sum:i32, to_add:&str, )->(i32, i32) {
    let mut result = sum;
    let ( direction,number) = to_add.split_at(1);
    let mut full_rotations = (number.parse::<i32>().unwrap()/100);
    println!("Cut rotations: {full_rotations}");
    let number = number.parse::<i32>().unwrap() % 100;
    if number != 0{
    if direction == "R"{
        result += number;
        if result >99{
            result = result%100;
            full_rotations +=1;
        }
        
    } else {
        result -= number;
        if result < 0 {
            result = 100 + result;
            if (sum != 0){
                full_rotations += 1;
            }
        } else if result == 0 {
            full_rotations +=1;
        }
    }
}
    println!("input: {sum} to add: {to_add} number: {number} output: {result}, final rotations: {full_rotations}");
    return (result, full_rotations);
}




pub fn process() {
    let answer = get_answer();
    println!("Day 1 answer: {}", answer);
}
