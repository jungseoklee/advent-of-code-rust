use std::collections::HashMap;
use std::error::Error;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;
use good_lp::*;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";


fn main() -> Result<(), Box<dyn Error>> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, r: i32) -> Result<u128, Box<dyn Error>> {
        let mut light_diagrams: Vec<Vec<usize>> = Vec::new();
        let mut buttons = HashMap::new();
        let mut joltages = Vec::new();
        let mut id: usize = 0;
        for line in reader.lines() {
            let line = line?;
            let line = line.trim()
                .split(' ')
                .collect::<Vec<&str>>();
            println!("{:?}", line);
            let light_digram: Vec<usize> = line[0].chars()
                .filter(|&c| c == '#' || c == '.')
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<usize>>();
            println!("{:?}", light_digram);
            light_diagrams.push(light_digram);
            let n = line.len();
            for i in 1..(n - 1) {
                let button: Vec<usize> = line[i].chars()
                    .filter(|&c| c.is_digit(10) || c == ',')
                    .collect::<String>()
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                println!("{:?}", button);
                buttons.entry(id).or_insert(Vec::new()).push(button);
            }
            let joltage: Vec<usize> = line[n - 1].chars()
                .filter(|&c| c.is_digit(10) || c == ',')
                .collect::<String>()
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            joltages.push(joltage);
            id += 1;
        }
        let mut ans = 0;
        for i in 0..id {
            ans += find(&light_diagrams[i], buttons.get(&i).unwrap());
        }
        Ok(ans)
    }
    assert_eq!(7, part1(BufReader::new(TEST.as_bytes()), 10)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1000)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<i32, Box<dyn Error>> {
        let mut light_diagrams: Vec<Vec<usize>> = Vec::new();
        let mut buttons = HashMap::new();
        let mut joltages = Vec::new();
        let mut id: usize = 0;
        for line in reader.lines() {
            let line = line?;
            let line = line.trim()
                .split(' ')
                .collect::<Vec<&str>>();
            let light_digram: Vec<usize> = line[0].chars()
                .filter(|&c| c == '#' || c == '.')
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<usize>>();
            light_diagrams.push(light_digram);
            let n = line.len();
            for i in 1..(n - 1) {
                let button: Vec<usize> = line[i].chars()
                    .filter(|&c| c.is_digit(10) || c == ',')
                    .collect::<String>()
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                buttons.entry(id).or_insert(Vec::new()).push(button);
            }
            let joltage: Vec<usize> = line[n - 1].chars()
                .filter(|&c| c.is_digit(10) || c == ',')
                .collect::<String>()
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            joltages.push(joltage);
            id += 1;
        }
        let mut ans = 0;
        for i in 0..id {
            ans += ilp(&joltages[i], buttons.get(&i).unwrap())?;
        }
        Ok(ans)
    }
    assert_eq!(1, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn find(target: &Vec<usize>, buttons: &Vec<Vec<usize>>) -> u128 {
    if target.iter().all(|&v| v == 0) {
        return 0;
    }

    let mut ans = 1;
    loop {
        if rec(target, &vec![0; target.len()], buttons, ans) {
            return ans;
        }
        ans += 1;
    }
}

fn rec(target: &Vec<usize>, current: &Vec<usize>, buttons: &Vec<Vec<usize>>, n: u128) -> bool {
    if target == current {
        return true;
    }
    if n == 0 {
        return false;
    }
    for button in buttons.iter() {
        let mut cur = current.clone();
        for &index in button.iter() {
            let v = cur[index];
            cur[index] = if v == 1 { 0 } else { 1 };
        }
        let res = rec(target, &cur, buttons, n - 1);
        if res {
            return true;
        }
    }

    false
}

fn ilp(target: &Vec<usize>, buttons: &Vec<Vec<usize>>) -> Result<i32, Box<dyn Error>> {
    let n_buttons = buttons.len();
    let n = target.len();

    let mut vars = variables!();
    let x: Vec<Variable> = (0..n_buttons)
        .map(|_| vars.add(variable().integer().min(0).max(1000)))
        .collect();

    let expression: Expression = x.iter().map(|&v| v).sum();

    let mut problem = vars.minimise(expression).using(default_solver);

    for counter_idx in 0..n {
        let mut expr = Expression::from(0);

        for (button_idx, button) in buttons.iter().enumerate() {
            if button.contains(&counter_idx) {
                expr = expr + x[button_idx];
            }
        }

        problem = problem.with(constraint!(expr == target[counter_idx] as f64));
    }

    let solution = problem.solve()?;

    let res: f64 = x.iter()
        .map(|&var| solution.value(var))
        .sum();

    Ok(res.round() as i32)
}