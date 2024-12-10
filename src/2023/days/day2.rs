use std::collections::{HashMap, btree_map::Range};

use crate::utils;

const INPUT:&str = "res/input2";

struct Game {
    id: u32,
    rounds: Vec<HashMap<char,u32>>,
}

pub(crate) fn task1(){
    let file_content = utils::read_file_lines(INPUT.to_owned());
    let lines = file_content.lines();

    let mut sum = 0;

    for line in lines {
        let game_result = parse_line_to_game(line);
        //println!("{}", game_result.id);
    }

    println!("Task1 executed")
}

pub(crate) fn task2(){
    println!("Task2 executed")
}


fn parse_line_to_game(line: &str) -> Game{
    let game_res_split: Vec<&str> = line.split(":").collect();
    let game_info_split = game_res_split[0].split_whitespace().collect::<Vec<&str>>();
    let game_id = game_info_split[1].parse().unwrap();
    let game_results_split = game_res_split[1].split_whitespace().collect::<Vec<&str>>();
    let split_size = game_results_split.len();
    for i in 0..split_size/2 {
        let amount = game_results_split[i*2];
        let color = game_results_split[i*2+1];
        println!("{}: {}", amount, color);
    }
    
    let round1 = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
    ]);
    let rounds = vec![round1];
    let parsed_game = Game {id:game_id, rounds:rounds}; 
    return parsed_game;
}