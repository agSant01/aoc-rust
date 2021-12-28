use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::io::BufRead;

const TEST: bool = false;

type Data = String;

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
    return line.clone();
}

fn lines_to_map(lines: &Vec<String>, grid_list: &mut Vec<HashMap<u32, (u32, u32)>>) {
    let mut grid: HashMap<u32, (u32, u32)> = HashMap::new();

    let mut y = 0;
    let mut x: u32;

    for line in lines {
        if line.is_empty() {
            grid_list.push(grid);
            grid = HashMap::new();
            y = 0;
        } else {
            x = 0;
            for item in line.trim().split_whitespace() {
                // println!("Item; \"{}\"", item);
                grid.insert(item.parse().unwrap(), (x, y));
                x += 1;
            }
            y += 1;
        }
    }
    grid_list.push(grid);
}

fn has_bingo(board_plays: &HashSet<&(u32, u32)>) -> bool {
    for x in 0..5 {
        let mut horizontal = true;
        for y in 0..5 {
            if !board_plays.contains(&(x, y)) {
                horizontal = false;
                break;
            }
        }
        if horizontal {
            return true;
        }
    }

    for y in 0..5 {
        let mut vertical = true;
        for x in 0..5 {
            if !board_plays.contains(&(x, y)) {
                vertical = false;
                break;
            }
        }
        if vertical {
            return true;
        }
    }

    false
}

fn part1(plays: &Vec<u32>, data: &Vec<Data>) -> u32 {
    let mut grid_list = Vec::with_capacity((data.len() - 2) / 5);
    lines_to_map(&data[2..].to_vec(), &mut grid_list);

    let mut board_plays: Vec<HashSet<&(u32, u32)>> =
        (0..grid_list.len()).map(|_| HashSet::new()).collect();

    for play in plays {
        for (idx, board) in grid_list.iter().enumerate() {
            if !board.contains_key(play) {
                continue;
            }

            board_plays[idx].insert(board.get(play).unwrap());

            if has_bingo(&board_plays[idx]) {
                return play
                    * board
                        .iter()
                        .filter(|entry| !board_plays[idx].contains(entry.1))
                        .map(|entry| entry.0)
                        .sum::<u32>();
            }
        }
    }

    0
}

fn part2(plays: &Vec<u32>, data: &Vec<Data>) -> u32 {
    let mut grid_list = Vec::with_capacity((data.len() - 2) / 5);
    lines_to_map(&data[2..].to_vec(), &mut grid_list);

    let mut board_plays: Vec<HashSet<&(u32, u32)>> =
        (0..grid_list.len()).map(|_| HashSet::new()).collect();

    let mut winners: HashSet<usize> = HashSet::new();
    for play in plays {
        for (idx, board) in grid_list.iter().enumerate() {
            if !board.contains_key(play) {
                continue;
            }

            board_plays[idx].insert(board.get(play).unwrap());

            if has_bingo(&board_plays[idx]) {
                winners.insert(idx);
            }

            if winners.len() == board_plays.len() {
                return play
                    * board
                        .iter()
                        .filter(|entry| !board_plays[idx].contains(entry.1))
                        .map(|entry| entry.0)
                        .sum::<u32>();
            }
        }
    }

    0
}

fn main() {
    if let Ok(lines) = get_input() {
        let mut shaped_data: Vec<Data> = vec![];

        let mut plays: Vec<u32> = vec![];

        for (i, line) in lines.enumerate() {
            if let Ok(line_res) = line {
                if i == 0 {
                    plays = line_res
                        .trim()
                        .split(',')
                        .map(|d| d.parse().unwrap())
                        .collect();
                }
                shaped_data.push(parse_line(&line_res))
            }
        }

        let mut result = part1(&plays, &shaped_data);
        println!("Result part 1: {}", result);

        result = part2(&plays, &shaped_data);
        println!("Result part 2: {}", result);
    }
}
