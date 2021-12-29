use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

const TEST: bool = false;

#[derive(Debug)]
struct Data {
    start: (i32, i32),
    end: (i32, i32),
}

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
    let tuples = line
        .split(" -> ")
        .map(|point| {
            point
                .split(',')
                .map(|i| i.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect::<Vec<(i32, i32)>>();

    return Data {
        start: tuples[0],
        end: tuples[1],
    };
}

fn valid_range(a: i32, b: i32) -> Box<dyn Iterator<Item = i32>> {
    if a < b {
        return Box::new(a..=b);
    }

    return Box::new((b..=a).rev());
}

fn part1(data: &Vec<Data>) -> u32 {
    let mut lines_count: HashMap<(i32, i32), u32> = HashMap::new();

    for data_point in data {
        if data_point.start.1 == data_point.end.1 {
            for x in valid_range(data_point.start.0, data_point.end.0) {
                *lines_count.entry((x, data_point.start.1)).or_insert(0) += 1;
            }
        } else if data_point.start.0 == data_point.end.0 {
            for y in valid_range(data_point.start.1, data_point.end.1) {
                *lines_count.entry((data_point.start.0, y)).or_insert(0) += 1;
            }
        }
    }

    lines_count.values().filter(|&&v| v >= 2).count() as u32
}

#[allow(dead_code)]
fn print_grid(coords: &HashMap<(i32, i32), u32>, x: u16, y: u16) {
    for yi in 0..x as i32 {
        for xi in 0..y as i32 {
            if let Some(coord) = coords.get(&(xi, yi)) {
                print!("{}", coord);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part2(data: &Vec<Data>) -> u32 {
    let mut lines_count: HashMap<(i32, i32), u32> = HashMap::new();

    for data_point in data {
        if data_point.start.1 == data_point.end.1 {
            for x in valid_range(data_point.start.0, data_point.end.0) {
                *lines_count.entry((x, data_point.start.1)).or_insert(0) += 1;
            }
        } else if data_point.start.0 == data_point.end.0 {
            for y in valid_range(data_point.start.1, data_point.end.1) {
                *lines_count.entry((data_point.start.0, y)).or_insert(0) += 1;
            }
        } else if data_point.start.0 != data_point.end.0 && data_point.start.1 != data_point.end.1 {
            // println!("{:?}", data_point);
            let mut range_x = valid_range(data_point.start.0, data_point.end.0);
            for y in valid_range(data_point.start.1, data_point.end.1) {
                *lines_count.entry((range_x.next().unwrap(), y)).or_insert(0) += 1;
            }
        }
    }

    lines_count.values().filter(|&&v| v >= 2).count() as u32
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
