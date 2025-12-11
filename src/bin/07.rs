use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

/*
const TEST: &str = "\
.......S.......
...............
......1^1......
...............
.....1^2^1.....
...............
....1^3^3^1....
...............
...1^4^331^1...
...............
..1^5^434^2^1..
...............
.1^154^74021^1.
...............
1^2^0^1^1^211^1
...............
";
*/

fn main() -> Result<(), Box<dyn Error>> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32, Box<dyn Error>> {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            grid.push(line.trim().chars().collect());
        }

        Ok(count(&grid))
    }
    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<u128, Box<dyn Error>> {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            grid.push(line.trim().chars().collect());
        }
        let row_size = grid.len();
        let col_size = grid[0].len();
        let mut v = vec![vec![0u128; col_size]; row_size];
        for c in 0..col_size {
            v[0][c] = if grid[0][c] == 'S' { 1 } else { 0 };
        }
        for r in 1..row_size {
            for c in 0..col_size {
                if grid[r][c] == '^' {
                    v[r][c] = 0;
                    if c as i32 - 1 >= 0 {
                        v[r][c - 1] += v[r - 1][c];
                    }
                    if c + 1 < col_size {
                        v[r][c + 1] += v[r - 1][c];
                    }
                } else {
                    v[r][c] += v[r - 1][c];
                }
            }
        }
        let mut ans = 0;
        for c in 0..col_size {
            ans += v[row_size - 1][c];
        }
        Ok(ans)
    }
    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn count(grid: &Vec<Vec<char>>) -> u32 {
    let row_size = grid.len();
    let col_size = grid[0].len();
    let mut start = (row_size + 1, col_size + 1);
    for c in 0..col_size {
        if grid[0][c] == 'S' {
            start = (0, c);
            break;
        }
    }
    let mut res = 0;
    let mut visited = vec![vec![false; col_size]; row_size];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((start.0 + 1, start.1));
    while !q.is_empty() {
        let (r, c) = q.pop_front().unwrap();
        if visited[r][c] {
            continue;
        }
        visited[r][c] = true;
        if grid[r][c] == '.' && r + 1 < grid.len() {
            q.push_back((r + 1, c));
        }
        if grid[r][c] == '^' {
            res += 1;
            if c as i32 - 1 >= 0 {
                q.push_back((r, c - 1));
            }
            if c + 1 < grid[0].len() {
                q.push_back((r, c + 1));
            }
        }
    }
    res
}