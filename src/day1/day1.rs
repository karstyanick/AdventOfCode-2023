use std::fs;

const PATTERNS: [&str; 18] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

pub fn day1() {
    let file_contents = fs::read_to_string("./src/day1/input.txt").expect("Should have been able to read the file");
    part1(file_contents.clone());
    part2(file_contents);
}

pub fn part1(file_contents: String){
    let numbers = file_contents.lines().map(|line| {
        let numerics = line.chars().filter(|char| char.is_numeric()).collect::<Vec<char>>();

        let mut string = String::from("");
        string.push(*numerics.first().unwrap());
        string.push(*numerics.last().unwrap());

        return string.parse::<u32>().unwrap();
    }).collect::<Vec<u32>>();

    let result = numbers.iter().sum::<u32>();

    println!("Part 1: {}", result);
}

pub fn part2(file_contents: String){
    let numbers = file_contents.lines().map(|line| {

        let mut contained_patterns = PATTERNS.iter().map(|pattern| search_for_pattern(line, pattern)).flatten().collect::<Vec<(u32, String)>>();

        contained_patterns.sort_by(|a, b| a.0.cmp(&b.0));

        if let Some(first) = contained_patterns.first() {
            let last = contained_patterns.last().unwrap();

            let first_number = translate_string_to_number(first.1.clone());
            let last_number = translate_string_to_number(last.1.clone());

            match (first_number + &last_number).parse::<u32>() {
                Ok(number) => return Some(number),
                Err(_) => return None
            }
        } else {
            None
        }

    }).flatten().collect::<Vec<u32>>();

    let result = numbers.iter().sum::<u32>();

    println!("Part 2: {}", result);
}

fn search_for_pattern(line: &str, pattern: &str) -> Vec<(u32, String)> {
    let matches = line.match_indices(pattern).map(|result| (result.0 as u32, result.1.to_string())).collect::<Vec<(u32, String)>>();
    return matches;
}

fn translate_string_to_number(text: String) -> String {
    match text.parse::<u32>() {
        Ok(_number) => return text,
        Err(_) => {
            let index = PATTERNS.iter().position(|pattern| *pattern == text).unwrap();
            return (index as u32 + 1).to_string();
        }
    }
}