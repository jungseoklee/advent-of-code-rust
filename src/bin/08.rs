use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

fn main() -> Result<(), Box<dyn Error>> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, r: i32) -> Result<i128, Box<dyn Error>> {
        let mut positions: Vec<(i128, i128, i128)> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let line = line.trim()
                .split(',')
                .map(|v| v.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            positions.push((line[0], line[1], line[2]));
        }
        let mut positions_by_dist: Vec<(i128, (usize, usize))> = Vec::new();
        let n = positions.len();
        for i in 0..n {
            for j in i + 1..n {
                let d = dist(positions[i], positions[j]);
                positions_by_dist.push((d, (i, j)));
            }
        }
        positions_by_dist.sort();

        let mut ids = vec![-1; positions.len()];
        let mut merged_id = 1;
        let mut iter = 0;
        for p in positions_by_dist {
            if ids[p.1.0] == -1 && ids[p.1.1] == -1 {
                ids[p.1.0] = merged_id;
                ids[p.1.1] = merged_id;
                merged_id += 1;
            } else if ids[p.1.0] == -1 && ids[p.1.1] != -1 {
                ids[p.1.0] = ids[p.1.1];
            } else if ids[p.1.0] != -1 && ids[p.1.1] == -1 {
                ids[p.1.1] = ids[p.1.0];
            } else {
                let id_to_override = ids[p.1.1];
                for i in 0..ids.len() {
                    if ids[i] == id_to_override {
                        ids[i] = ids[p.1.0];
                    }
                }
            }
            //println!("{:?}, {:?}, {}", ids, p, iter);
            iter += 1;
            if iter == r {
                break;
            }
        }
        for i in 0..ids.len() {
            if ids[i] == -1 {
                ids[i] = merged_id;
                merged_id += 1;
            }
        }
        let mut m = HashMap::new();
        for i in 0..ids.len() {
            let cnt = m.entry(ids[i]).or_insert(0);
            *cnt += 1;
        }
        let mut values = m.values().collect::<Vec<_>>();
        values.sort();
        Ok(values.into_iter().rev().take(3).product::<i128>())
    }
    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()), 10)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1000)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<i128, Box<dyn Error>> {
        let mut positions: Vec<(i128, i128, i128)> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let line = line.trim()
                .split(',')
                .map(|v| v.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            positions.push((line[0], line[1], line[2]));
        }
        let mut positions_by_dist: Vec<(i128, (usize, usize))> = Vec::new();
        let n = positions.len();
        for i in 0..n {
            for j in i + 1..n {
                let d = dist(positions[i], positions[j]);
                positions_by_dist.push((d, (i, j)));
            }
        }
        positions_by_dist.sort();

        let mut ids = vec![-1; positions.len()];
        let mut merged_id = 1;
        for p in positions_by_dist {
            if ids[p.1.0] == -1 && ids[p.1.1] == -1 {
                ids[p.1.0] = merged_id;
                ids[p.1.1] = merged_id;
                merged_id += 1;
            } else if ids[p.1.0] == -1 && ids[p.1.1] != -1 {
                ids[p.1.0] = ids[p.1.1];
            } else if ids[p.1.0] != -1 && ids[p.1.1] == -1 {
                ids[p.1.1] = ids[p.1.0];
            } else {
                let id_to_override = ids[p.1.1];
                for i in 0..ids.len() {
                    if ids[i] == id_to_override {
                        ids[i] = ids[p.1.0];
                    }
                }
            }
            let mut connected = true;
            let mut merged_id = -1;
            for i in 0..ids.len() {
                if ids[i] == -1 {
                    connected = false;
                    break;
                }
                if merged_id == -1 {
                    merged_id = ids[i];
                    continue;
                }
                if merged_id != ids[i] {
                    connected = false;
                    break;
                }
            }
            if connected {
                println!("{:?}, {:?}", positions[p.1.0], positions[p.1.1]);
                return Ok(positions[p.1.0].0 * positions[p.1.1].0);
            }
            //println!("{:?}, {:?}, {}", ids, p, iter);
        }
        Ok(0)
    }
    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn dist(a: (i128, i128, i128), b: (i128, i128, i128)) -> i128 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}