use std::fs;

pub fn day3() {
    let file_contents = fs::read_to_string("./src/day3/input.txt").expect("Should have been able to read the file");
    part1(file_contents.clone());
}

fn part1(file_contents: String) {
    let lines = file_contents.lines().enumerate().map(|(line_index, line)| line).collect::<Vec<&str>>();

    let valid_items = lines.iter().enumerate().map(|(line_index, line)| {

        let mut last_was_number = false;
        let mut current_number = String::from("");

        let mut validate_next_number = false;

        let numbers_with_index = line.char_indices().map(|(index, character)| {

            let is_end_of_line = index == line.len() - 1;

            let mut usedIndex = index;

            if(is_end_of_line && character.is_numeric()){
                usedIndex += 1;
                current_number.push(character);
            }
            
            if (index == 136){
                println!("HEYO")
            }

            if character == '.' || is_end_of_line{
                if last_was_number {
                    if validate_next_number || filter_other_line_adjacent(lines.clone(), line_index, usedIndex - current_number.len(), current_number.len()){

                        let return_value = Some(current_number.parse().unwrap());
                        last_was_number = false;
                        current_number.clear();
                        validate_next_number = false;
                        return return_value;
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
                    if last_was_number || filter_other_line_adjacent(lines.clone(), line_index, usedIndex - current_number.len(), current_number.len()){
                        let return_value = Some(current_number.parse().unwrap());

                        last_was_number = false;
                        current_number.clear();
                        validate_next_number = true;

                        return return_value
                    }else{
                        validate_next_number = true;
                        return None
                    }
                }
            }
        }).flatten().collect::<Vec<u32>>();

        return numbers_with_index
    }).flatten().collect::<Vec<u32>>();

    println!("{:?}", valid_items);
    println!("{:?}", valid_items.iter().sum::<u32>())
}




fn filter_other_line_adjacent(lines: Vec<&str>, current_line_index: usize, index: usize, number_length: usize) -> bool{
        
    let starting_index = if index > 0 { index - 1} else { index };
    let ending_index = if index + number_length < (lines[0].len() - 1) { index + number_length + 1 } else { index + number_length };

    let mut contains_symbol = false;

    if(current_line_index > 0){
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