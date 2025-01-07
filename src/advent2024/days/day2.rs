use adventofcode_rust::utils::*;

pub fn get_answer()->String {
    let input_file = "src/advent2024/days/input/day2_input";

    let str_lines = utils::read_file_lines(&input_file.to_string());
    let int_lines: Vec<Vec<i32>> = str_lines.into_iter().map(|line| utils::get_int_array_from_line(&line)).collect();
    let mut safe_inputs_1 = 0;

    for line in &int_lines{
        let safe;
        if line[1] > line[0]{
            safe = line.windows(2).all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);
        } else {
            safe = line.windows(2).all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);
        }
        if safe{
            safe_inputs_1 += 1;
        }
    }

    let mut safe_inputs_2 = 0;

    for line in &int_lines{
        
        if check_line(&line){
            safe_inputs_2 += 1;
        }
    }


    return format!("Safe inputs :{} {}", safe_inputs_1, safe_inputs_2);
}

pub fn check_line_simple(line: &Vec<i32>) -> bool{
    let safe;
    if line[1] > line[0]{
        safe = line.windows(2).all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3);
    } else {
        safe = line.windows(2).all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3);
    }
    return safe;
}

pub fn check_line(line: &Vec<i32>) -> bool{
    let mut safe = true;
    let mut tolerated = false;
    let mut direction = 0;
    let mut prev = line[0];
    let mut next;
    let mut counter = 1;

    if line[line.len()-1] > line[0]{
        direction = 1;
    } else if line[line.len()-1] < line[0] {
        direction = -1;
    } else {
        safe = false;
    }

    while counter < line.len() && safe{
        let local_safe;
        next = line[counter];
        if direction == 1 {
            local_safe = next - prev >= 1 && next - prev <= 3;
        } else {
            local_safe = prev - next >= 1 && prev - next <= 3;
        }
        
        if !local_safe{
            if tolerated{
                safe = false;
            } else{
                tolerated = true;
            }
        } else{
            prev = next;
        }
        counter +=1;
    }
    if !safe {
        
        for i in 0..line.len()-1{
            let mut new_line = line.clone();
            new_line.remove(i);
            if check_line_simple(&new_line){
                println!("{:?} {:?}" , &line[..], &new_line[..]);
                safe = true;
            }
        }
    }
    return safe;
}



pub fn process() {
    let answer = get_answer();
    println!("Day 2 answer: {}", answer);
}