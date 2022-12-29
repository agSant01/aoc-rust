use std::cmp::{max, min};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::ops::Add;

const TEST: bool = false;

#[derive(Debug)]
pub struct Data {
    value: String,
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

fn parse_line(line: String) -> Data {
    return Data { value: line };
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Add<&(i64, i64)> for &Position {
    type Output = Position;
    fn add(self, other: &(i64, i64)) -> Self::Output {
        Position {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl PartialEq<(i64, i64)> for &Position {
    fn eq(&self, other: &(i64, i64)) -> bool {
        return self.x == other.0 && self.y == other.1;
    }
}

#[allow(dead_code)]
fn print_grid(elves: &HashSet<Position>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    elves.iter().for_each(|elf| {
        min_x = min(min_x, elf.x);
        max_x = max(max_x, elf.x);
        min_y = min(min_y, elf.y);
        max_y = max(max_y, elf.y);
    });
    min_x -= 3;
    max_x += 3;
    min_y -= 3;
    max_y += 3;

    for y in min_y..=max_y {
        print!("{:>3}", y);
        for x in min_x..=max_x {
            if elves.contains(&Position { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("======================");
}

fn parse_elves_positions(lines: &Vec<Data>) -> HashSet<Position> {
    let mut positions = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.value.chars().enumerate() {
            if c == '#' {
                positions.insert(Position {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }
    positions
}

fn run_simulation(elves: &mut HashSet<Position>, part: u8) -> usize {
    let mut rounds: usize = 0;
    let mut moved;
    let mut must_move;
    let deltas = [
        [(-1, -1), (0, -1), (1, -1)], // N
        [(-1, 1), (0, 1), (1, 1)],    // S
        [(-1, 1), (-1, 0), (-1, -1)], // W
        [(1, 1), (1, 0), (1, -1)],    // E
    ];
    let compass = [
        (0, -1), // N
        (0, 1),  // S
        (-1, 0), // W
        (1, 0),  // E
    ];
    let mut to_move_dict: HashMap<Position, Vec<Position>> = HashMap::new();
    loop {
        moved = false;
        for elf in elves.iter() {
            must_move = false;
            'col: for x in elf.x - 1..=elf.x + 1 {
                for y in elf.y - 1..=elf.y + 1 {
                    if elf == (x, y) {
                        continue;
                    }
                    if elves.contains(&Position { x, y }) {
                        must_move = true;
                        break 'col;
                    }
                }
            }

            if must_move == false {
                continue;
            }

            let mut to_move_pos = None;
            for k in 0..4 {
                let nk = (rounds + k) % 4;
                if deltas[nk]
                    .iter()
                    .all(|delta| !elves.contains(&(elf + delta)))
                {
                    to_move_pos = Some(elf + &compass[nk]);
                    break;
                }
            }
            if let Some(valid_pos) = to_move_pos {
                moved = true;
                if to_move_dict.contains_key(&valid_pos) {
                    if let Some(list) = to_move_dict.get_mut(&valid_pos) {
                        list.push(*elf);
                    }
                } else {
                    to_move_dict.insert(valid_pos, vec![*elf]);
                }
            }
        }

        for (new, old) in &to_move_dict {
            if old.len() == 1 {
                elves.insert(*new);
                elves.remove(&old[0]);
            }
        }

        rounds += 1;

        if part == 1 && rounds >= 10 {
            return rounds;
        }

        if part == 2 && !moved {
            return rounds;
        }

        to_move_dict.clear();
    }
}

fn part1(data: &Vec<Data>) -> i64 {
    let mut elves_positions = parse_elves_positions(&data);

    run_simulation(&mut elves_positions, 1);

    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for elf_pos in elves_positions.iter() {
        min_x = min(min_x, elf_pos.x);
        max_x = max(max_x, elf_pos.x);
        min_y = min(min_y, elf_pos.y);
        max_y = max(max_y, elf_pos.y);
    }

    ((max_x - min_x + 1) * (max_y - min_y + 1)) - elves_positions.len() as i64
}

pub fn part2(data: &Vec<Data>) -> i64 {
    let mut elves_positions = parse_elves_positions(&data);
    run_simulation(&mut elves_positions, 2) as i64
}

fn main() {
    if let Ok(lines) = get_input() {
        let mut shaped_data: Vec<Data> = vec![];
        for line in lines {
            if let Ok(line_res) = line {
                shaped_data.push(parse_line(line_res))
            }
        }
        let mut result = part1(&shaped_data);
        println!("Result part 1: {}", result);

        result = part2(&shaped_data);
        println!("Result part 2: {}", result);
    };
}
