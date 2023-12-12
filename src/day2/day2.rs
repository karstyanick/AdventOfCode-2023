
use std::fs;

struct Balls {
    color: String,
    count: u32
}

struct MaxColorValues {
    blue: Option<u32>,
    red: Option<u32>,
    green: Option<u32>
}

pub fn day2() {
    let file_contents = fs::read_to_string("./src/day2/input.txt").expect("Should have been able to read the file");
    part1(file_contents.clone());
    part2(file_contents);
}

fn part1(file_contents: String) {
    let lines = file_contents.lines().map(|line| {

        let parts = line.split(":").collect::<Vec<&str>>();
        let game = parts[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

        let max_color_values = extract_max_color_values(parts);

        if let (Some(max_blue), Some(max_red), Some(max_green)) = (max_color_values.blue, max_color_values.red, max_color_values.green) {
            if max_blue > 14 {
                return None
            }
            if max_green > 13 {
                return None
            }
            if max_red > 12 {
                return None
            }
        }else{
            println!("HEYO")
        }

        return Some(game);
    }).flatten().collect::<Vec<u32>>();

    println!("Part1: {}", lines.iter().sum::<u32>());
}

fn part2(file_contents: String) {
    let lines = file_contents.lines().map(|line| {
        
        let parts = line.split(":").collect::<Vec<&str>>();
        let max_color_values = extract_max_color_values(parts);

        if let (Some(max_blue), Some(max_red), Some(max_green)) = (max_color_values.blue, max_color_values.red, max_color_values.green) {
            return max_green*max_red*max_blue
        }else{
            return 0
        }

    }).collect::<Vec<u32>>();

    println!("Part2: {}", lines.iter().sum::<u32>());
}

fn extract_max_color_values(parts: Vec<&str>) -> MaxColorValues {
    let pulls = parts[1].split(";").map(|pull| pull.replace(" ", "")).collect::<Vec<String>>();

    let pulls_struct = pulls.into_iter().flat_map(|pull| pull.split(",").map(|balls| {

        let balls_struct = balls.split(",").map(|ball| {
            let number = ball.chars().filter(|char| char.is_numeric()).collect::<String>().parse::<u32>().unwrap();
            let color = ball.chars().filter(|char| char.is_alphabetic()).collect::<String>().parse::<String>().unwrap();

            return Balls {
                color: color.to_string(),
                count: number
            }
        }).collect::<Vec<Balls>>();

        return balls_struct

    }).collect::<Vec<Vec<Balls>>>()).flatten().collect::<Vec<Balls>>();

    let max_blue = pulls_struct.iter().filter(|balls| balls.color == "blue").map(|balls| balls.count).max();
    let max_red = pulls_struct.iter().filter(|balls| balls.color == "red").map(|balls| balls.count).max();
    let max_green = pulls_struct.iter().filter(|balls| balls.color == "green").map(|balls| balls.count).max();

    return MaxColorValues {
        blue: max_blue,
        red: max_red,
        green: max_green
    }
}