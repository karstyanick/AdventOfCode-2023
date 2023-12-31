use std::fs;
#[derive(Clone)]
struct PartNumber {
    number: u32,
    index: usize,
    length: usize,
    line: usize
}

pub fn day3() {
    let file_contents = fs::read_to_string("./src/day3/input.txt").expect("Should have been able to read the file");
    part1(file_contents.clone());
    part2(file_contents.clone());
}

fn part1(file_contents: String) {
    let lines = file_contents.lines().map(|line| line).collect::<Vec<&str>>();
    let part_numbers = get_part_numbers(lines.clone());
    println!("Part1: {}", part_numbers.iter().map(|part_number| part_number.number).sum::<u32>())
}

fn part2(file_contents: String) {
    let lines = file_contents.lines().map(|line| line).collect::<Vec<&str>>();
    let part_numbers = get_part_numbers(lines.clone());
    
    let gear_ratios = lines.iter().enumerate().map(|(line_index, line)| {
        
        let line_gear_ratios = line.char_indices().map(|(index, character)| {
            if character == '*' {
                let above_adjacent = filter_adjacent_part_numbers(part_numbers.clone(), line_index-1, index);
                let below_adjacent = filter_adjacent_part_numbers(part_numbers.clone(), line_index+1, index);
                let same_line_adjacent = filter_adjacent_part_numbers(part_numbers.clone(), line_index, index);

                let all_adjacent = [above_adjacent, below_adjacent, same_line_adjacent].concat();

                if all_adjacent.len() == 2 {
                    return Some(all_adjacent[0] * all_adjacent[1])
                }
            }
            return None
        }).flatten().collect::<Vec<u32>>();

        return line_gear_ratios

    }).flatten().collect::<Vec<u32>>();

    println!("Part2: {}", gear_ratios.iter().sum::<u32>());
}

fn filter_adjacent_part_numbers(part_numbers: Vec<PartNumber>, line_index: usize, index: usize) -> Vec<u32>{
    return part_numbers.iter().filter(|part_number| {
        part_number.line == line_index && 
        ((part_number.index..part_number.index + part_number.length).contains(&index) || 
        (part_number.index..part_number.index + part_number.length).contains(&(index + 1)) || 
        (part_number.index..part_number.index + part_number.length).contains(&(index - 1)))
    }).map(|part_numer| part_numer.number).collect::<Vec<u32>>();
}

fn get_part_numbers(lines: Vec<&str>) -> Vec<PartNumber>{
    return lines.iter().enumerate().map(|(line_index, line)| {

        let mut last_was_number = false;
        let mut current_number = String::from("");

        let mut validate_next_number = false;

        let numbers_with_index = line.char_indices().map(|(index, character)| {

            let is_end_of_line = index == line.len() - 1;
            let mut used_index = index;

            if is_end_of_line && character.is_numeric() {
                used_index += 1;
                current_number.push(character);
            }

            if character == '.' || is_end_of_line{
                if last_was_number {
                    if validate_next_number || filter_other_line_adjacent(lines.clone(), line_index, used_index - current_number.len(), current_number.len()){
                        let return_value = current_number.parse::<u32>().unwrap();
                        let length = current_number.len();
                        let index = used_index - current_number.len();
                        last_was_number = false;
                        current_number.clear();
                        validate_next_number = false;

                        return Some(PartNumber {
                            number: return_value,
                            index,
                            length,
                            line: line_index
                        });
                    }else{
                        last_was_number = false;
                        current_number.clear();
                        return None
                    }
                }else{
                    validate_next_number = false;
                    return None
                }
            }else{
                if character.is_numeric() {
                    last_was_number = true;
                    current_number.push(character);
                    return None;
                }else{                    
                    if last_was_number || filter_other_line_adjacent(lines.clone(), line_index, used_index - current_number.len(), current_number.len()){
                        let return_value = current_number.parse::<u32>().unwrap();
                        let length = current_number.len();
                        let index = used_index - current_number.len();
                        last_was_number = false;
                        current_number.clear();
                        validate_next_number = true;

                        return Some(PartNumber {
                            number: return_value,
                            index,
                            length,
                            line: line_index
                        
                        });
                    }else{
                        validate_next_number = true;
                        return None
                    }
                }
            }
        }).flatten().collect::<Vec<PartNumber>>();

        return numbers_with_index
    }).flatten().collect::<Vec<PartNumber>>();
}

fn filter_other_line_adjacent(lines: Vec<&str>, current_line_index: usize, index: usize, number_length: usize) -> bool{
        
    let starting_index = if index > 0 { index - 1} else { index };
    let ending_index = if index + number_length < (lines[0].len() - 1) { index + number_length + 1 } else { index + number_length };

    let mut contains_symbol = false;

    if current_line_index > 0 {
        lines[current_line_index-1].get(starting_index..ending_index).unwrap().chars().for_each(|character| {
            if character != '.' && !character.is_numeric() {
                contains_symbol = true;
            }
        });
    }

    if current_line_index < lines.len() - 1 {
        lines[current_line_index + 1].get(starting_index..ending_index).unwrap().chars().for_each(|character| {
            if character != '.' && !character.is_numeric() {
                contains_symbol = true;
            }
        });
    }

    return contains_symbol
}