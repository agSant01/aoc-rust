use std::fs;
use std::io;
use std::io::BufRead;

const TEST: bool = false;

type Data = u32;

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
    line.parse().unwrap()
}

fn part1(data: &Vec<Data>) -> u32 {
    // Implementation 1
    let mut inc1: u32 = 0;
    let mut prev = &data[0];
    for point in data {
        if point > prev {
            inc1 += 1;
        }
        prev = point;
    }

    // Implementation 2
    let inc2: u32 = data
        .as_slice()
        .windows(2)
        .filter(|d| d[1] > d[0])
        .count()
        .try_into()
        .unwrap();

    // Implementation 3
    let mut inc3 = 0;
    for i in 0..data.len() - 1 {
        inc3 += (data[i + 1] > data[i]) as u32;
    }

    // println!("{}, {}, {}", inc1, inc2, inc3);
    assert_eq!(inc1, inc2);
    assert_eq!(inc1, inc3);

    return inc1;
}

fn part2(data: &Vec<Data>) -> u32 {
    // Implementation 1
    let inc1: u32 = data
        .as_slice()
        .windows(4)
        .filter(|d| d.last() > d.first())
        .count()
        .try_into()
        .unwrap();

    // Implementation 2
    let mut inc2 = 0;
    for i in 0..data.len() - 3 {
        inc2 += (data[i + 3] > data[i]) as u32;
    }

    // println!("{}, {}, {}", inc1, inc2, inc3);
    assert_eq!(inc2, inc1);

    return inc1;
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
