use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

fn main() -> Result<(), Box<dyn Error>> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u128, Box<dyn Error>> {
        let mut seen_empty_line= false;
        let mut v = vec![];
        let mut ans = 0;
        for line in reader.lines() {
            let line = line?;
            if line == "" {
                seen_empty_line = true;
                continue;
            }
            if seen_empty_line {
                let id: u128 = line.parse()?;
                if is_fresh(&v, id) {
                    ans += 1;
                }
            } else {
                let chars: Vec<_> = line.trim().split("-").collect();
                let s = chars.get(0).unwrap().parse::<u128>()?;
                let e = chars.get(1).unwrap().parse::<u128>()?;
                v.push((s, e));
            }
        }
        Ok(ans)
    }
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<u128, Box<dyn Error>> {
        let mut v = vec![];
        for line in reader.lines() {
            let line = line?;
            if line == "" {
                break;
            }
            let chars: Vec<_> = line.trim().split("-").collect();
            let s = chars.get(0).unwrap().parse::<u128>()?;
            let e = chars.get(1).unwrap().parse::<u128>()?;
            v.push((s, e));
        }
        v.sort();
        let mut merged = vec![];
        for (s, e) in v.iter() {
            if merged.is_empty() {
                merged.push((*s, *e));
            } else {
                let last = merged.pop().unwrap();
                if last.0 <= *s && *s <= last.1 {
                    merged.push((last.0, std::cmp::max(last.1, *e)));
                } else {
                    merged.push(last);
                    merged.push((*s, *e));
                }
            }
        }
        let mut ans = 0;
        for (s, e) in merged {
            ans += e - s + 1;
        }
        Ok(ans)
    }
    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn is_fresh(ranges: &Vec<(u128, u128)>, id: u128) -> bool {
    for range in ranges {
        let (s, e) = range;
        if *s <= id && id <= *e {
            return true;
        }
    }
    false
}