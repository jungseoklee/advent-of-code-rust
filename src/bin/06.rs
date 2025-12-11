use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

fn main() -> Result<(), Box<dyn Error>> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i128, Box<dyn Error>> {
        let mut ans = 0;
        let mut grid: Vec<Vec<i128>> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.starts_with("+") || line.starts_with("*") {
                let ops = line.split_whitespace().collect::<Vec<_>>();
                for i in 0..ops.len() {
                    let mut v = 0;
                    match ops[i].as_bytes()[0] as char {
                        '+' => {
                            for r in 0..grid.len() {
                                v += grid[r][i];
                            }
                        },
                        '*' => {
                            v = 1;
                            for r in 0..grid.len() {
                                v *= grid[r][i];
                            }
                        },
                        _ => unreachable!(),
                    }
                    ans += v;
                }
                break;
            }
            grid.push(line.split_whitespace()
                .map(|s| s.parse::<i128>())
                .collect::<Result<Vec<_>, _>>()?);
        }
        Ok(ans)
    }
    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<i128, Box<dyn Error>> {
        let mut lines: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            lines.push(line.chars().collect());
        }
        let mut start_indices = vec![];
        let last_line = lines.last().unwrap();
        for i in 0..last_line.len() {
            if last_line[i] == '+' || last_line[i] == '*' {
                start_indices.push(i);
            }
        }
        let max_len = lines.iter().map(|l| l.len()).max().unwrap();
        start_indices.push(max_len + 1);

        let mut ans = 0;
        for i in 0..(start_indices.len() - 1) {
            let op = last_line[start_indices[i]];
            let mut v = if op == '+' { 0 } else { 1 };
            let start_index = start_indices[i];
            let len = start_indices[i + 1] - start_indices[i] - 1;
            let end_index = if i == start_indices.len() - 1 { max_len } else { start_index + len };
            for j in start_index..end_index {
                let mut num: Vec<char> = vec![];
                for k in 0..(lines.len() - 1) {
                    if  j <= lines[k].len() - 1 && lines[k][j] != ' ' {
                        num.push(lines[k][j]);
                    }
                }
                let num = num.iter().collect::<String>().parse::<i128>()?;
                match op {
                    '+' => v += num,
                    '*' => v *= num,
                    _ => unreachable!(),
                }
            }
            ans += v;
        }
        Ok(ans)
    }
    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
