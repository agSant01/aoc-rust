use std::fs;
use std::io;
use std::io::BufRead;

const TEST: bool = false;

struct Data {}

fn get_input() -> io::Result<io::Lines<io::BufReader<fs::File>>> {
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
    return Data {};
}

fn part1(data: &Vec<Data>) -> u32 {
    0
}

fn part2(data: &Vec<Data>) -> u32 {
    0
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
