use std::fs;
use regex::Regex;
use std::collections::HashSet;

pub fn day4() {
    let file_contents = fs::read_to_string("./src/day4/input.txt").expect("Should have been able to read the file");
    part1(file_contents.clone());
    part2(file_contents.clone());
}

fn part1(file_contents: String){

    let number_of_matches_per_line = file_contents.lines().map(|line| {get_number_of_matches_per_line(line)}).collect::<Vec<u32>>();

    let card_score = number_of_matches_per_line.iter().map(|matches| {
        match matches {
            0 => 0,
            _ => u32::pow(2, matches - 1)
        }
    }).collect::<Vec<u32>>();    

    println!("Part1: {}", card_score.iter().sum::<u32>());
}

fn part2(file_contents: String){
    let number_of_matches_per_line = file_contents.lines().map(|line| {get_number_of_matches_per_line(line)}).collect::<Vec<u32>>();
    let mut result_vector = vec![1 as u32; number_of_matches_per_line.len()];

    number_of_matches_per_line.iter().enumerate().for_each(|(index, matches)| {

        for i in (index+1)..(index+1 + *matches as usize) {
            result_vector[i] += 1 * result_vector[index];
        }
    });

    println!("Part2: {}", result_vector.iter().sum::<u32>());
}

fn get_number_of_matches_per_line(line: &str) -> u32{
    let regex = Regex::new(r"Card [\d]: ").unwrap();
    let stripped_line = regex.replace(line, "");

    let number_sets = stripped_line.split(" | ").collect::<Vec<&str>>();
    let chosen_numbers = HashSet::<u32>::from_iter(number_sets[0].split(" ").map(|number| number.parse()).flatten());
    let winning_numbers = HashSet::<u32>::from_iter(number_sets[1].split(" ").map(|number| number.parse()).flatten());
    
    let intersect_length = chosen_numbers.intersection(&winning_numbers).collect::<Vec<&u32>>().len() as u32;

    return intersect_length
}