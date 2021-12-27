use std::fs;
use std::io;
use std::io::BufRead;

const TEST: bool = false;

#[derive(Debug)]
struct Data {
    command: String,
    units: u32,
}

fn get_input() -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    // let vec = vec![];
    let file_path;

    if TEST {
        file_path = String::from("input/input_test.txt");
    } else {
        file_path = String::from("input/input.txt");
    }

    let file = fs::File::open(file_path)?;

    return Ok(io::BufReader::new(file).lines());
}

fn parse_line(line: &String) -> Data {
    let mut parsed = line.split(' ');

    Data {
        command: parsed.next().unwrap().to_string(),
        units: parsed.next().unwrap().parse::<u32>().unwrap(),
    }
}

fn part1(data: &Vec<Data>) -> u32 {
    let mut depth = 0;
    let mut horizontal = 0;

    for command in data {
        match command.command.as_str() {
            "forward" => horizontal += command.units,
            "down" => depth += command.units,
            "up" => depth -= command.units,
            _ => panic!("Unexpected instruction: {}", command.command),
        }
    }

    horizontal * depth
}

fn part2(data: &Vec<Data>) -> u32 {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for command in data {
        match command.command.as_str() {
            "forward" => {
                horizontal += command.units;
                depth += aim * command.units;
            }
            "down" => aim += command.units,
            "up" => aim -= command.units,
            _ => panic!("Unexpected instruction: {}", command.command),
        }
    }

    horizontal * depth
}

fn main() {
    if let Ok(lines) = get_input() {
        let mut shaped_data: Vec<Data> = vec![];

        for line in lines {
            if let Ok(line_res) = line {
                shaped_data.push(parse_line(&line_res))
            }
        }

        let mut result = part1(&shaped_data);
        println!("Result part 1: {}", result);

        result = part2(&shaped_data);
        println!("Result part 2: {}", result);
    }
}
