use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

const TEST: bool = false;

type Data = Vec<bool>;

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
    return line.chars().map(|c| c == '1').collect();
}

fn part1_a(data: &Vec<Data>) -> u32 {
    let mut hash_map = HashMap::new();

    let total = data.len() as u32;
    let mut gamma = 0;
    let mut epsilon = 0;

    for line in data {
        for (i, &c) in line.iter().enumerate() {
            *hash_map.entry(i as u32).or_insert(0) += c as u32;
        }
    }

    let mut sorted_entries: Vec<_> = hash_map.into_iter().collect();
    sorted_entries.sort_by_key(|d| d.0);

    for (_, ones) in sorted_entries {
        gamma += (2 * ones > total) as u32;
        gamma <<= 1;

        epsilon += (2 * ones < total) as u32;
        epsilon <<= 1;
    }

    gamma >>= 1;
    epsilon >>= 1;

    (epsilon * gamma) as u32
}

fn part1_b(data: &Vec<Data>) -> u32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    let mut ones;

    let total_words = data.len();

    for i in 0..data[0].len() {
        ones = 0;
        for j in 0..total_words {
            ones += data[j][i] as usize;
        }

        gamma += (2 * ones > total_words) as u32;
        gamma <<= 1;

        epsilon += (2 * ones < total_words) as u32;
        epsilon <<= 1;
    }
    gamma >>= 1;
    epsilon >>= 1;

    assert_eq!(epsilon, !gamma & (1 << data[0].len()) - 1);

    gamma * epsilon
}

fn part1(data: &Vec<Data>) -> u32 {
    let res1 = part1_a(data);
    let res2 = part1_b(data);

    assert_eq!(res1, res2);

    res1
}

fn bit_arr_to_dec(bit_arr: &Vec<bool>) -> u32 {
    let mut result = 0;
    for &bit in bit_arr {
        result <<= 1;
        result += bit as u32;
    }
    result
}

fn find_rating(words: &Vec<Data>, criteria: bool) -> &Data {
    let mut tmp_words: Vec<&Data> = words.iter().collect();

    let mut pos = 0;
    let mut ones: usize;

    while tmp_words.len() > 1 {
        let rem_wrd = tmp_words.len();
        ones = tmp_words.iter().filter(|word| word[pos]).count();
        tmp_words = tmp_words
            .into_iter()
            .filter(|word| (word[pos] == criteria) == (2 * ones >= rem_wrd))
            .collect();
        pos += 1;
    }

    tmp_words[0]
}

fn part2(data: &Vec<Data>) -> u32 {
    let ox_rating = find_rating(data, true);
    // println!("{:?}", ox_rating);

    let co_rating = find_rating(data, false);
    // println!("{:?}", co_rating);

    bit_arr_to_dec(ox_rating) * bit_arr_to_dec(co_rating)
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
