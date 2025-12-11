use std::error::Error;
use std::fs::{File, ReadDir};
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn parse<R: BufRead>(reader: R) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        grid.push(line?.chars().collect());
    }
    Ok(grid)
}

fn main() -> Result<(), Box<dyn Error>> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32, Box<dyn Error>> {
        Ok(simulate(&mut parse(reader)?))
    }
    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<i32, Box<dyn Error>> {
        let mut grid = parse(reader)?;
        let mut ans = 0;
        loop {
            let res = simulate(&mut grid);
            if res == 0 {
                break;
            }
            ans += res;
        }
        Ok(ans)
    }
    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn simulate(grid: &mut Vec<Vec<char>>) -> i32 {
    let row_size = grid.len();
    let col_size = grid[0].len();

    let dr = [1, -1, 0, 0, 1, 1, -1, -1];
    let dc = [0, 0, 1, -1, 1, -1, 1, -1];

    let mut res = 0;
    let mut updates: Vec<(usize, usize)> = Vec::new();
    for row in 0..row_size {
        for col in 0..col_size {
            if grid[row][col] == '.' {
                continue;
            }
            let mut cnt = 0;
            for i in 0..8 {
                let nr = row as i32 + dr[i];
                let nc = col as i32 + dc[i];

                if nr < 0 || nc < 0 || nr >= row_size as i32 || nc >= col_size as i32 {
                    continue;
                }
                if grid[nr as usize][nc as usize] == '@' {
                    cnt += 1;
                }
            }

            if cnt < 4 {
                updates.push((row, col));
                res += 1;
            }
        }
    }
    updates.iter().for_each(|&(row, col)| {grid[row][col] = '.'});
    res
}