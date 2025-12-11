use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const TEST_PART_2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

struct Graph {
    g: HashMap<u32, Vec<u32>>,
    m: HashMap<String, u32>,
    id: u32,
}

impl Graph {
    fn new() -> Graph {
        Self {
            g: HashMap::new(),
            m: HashMap::new(),
            id: 0,
        }
    }

    fn insert(&mut self, u: String, v: String) {
        let u_id = self.get_id(u);
        let v_id = self.get_id(v);
        self.g.entry(u_id).or_insert(Vec::new()).push(v_id);
    }

    fn get_id(&mut self, u: String) -> u32 {
        if self.m.contains_key(&u) {
            self.m[&u]
        } else {
            let new_id = self.id;
            self.m.insert(u, new_id);
            self.id += 1;
            new_id
        }
    }
    fn solve_part1(&mut self) -> u32 {
        let you_id = self.get_id(String::from("you"));
        let out_id = self.get_id(String::from("out"));
        let mut visited = vec![0; self.id as usize];
        self.dfs(you_id, &mut visited);
        visited[out_id as usize]
    }

    fn dfs(&self, u: u32, visited: &mut Vec<u32>) {
        visited[u as usize] += 1;
        if self.g.contains_key(&u) {
            for v in self.g.get(&u).unwrap().iter() {
                self.dfs(*v, visited);
            }
        }
    }

    fn solve_part2(&mut self) -> u128 {
        let svr_id = self.get_id(String::from("svr"));
        let out_id = self.get_id(String::from("out"));
        let dac_id = self.get_id(String::from("dac"));
        let fft_id = self.get_id(String::from("fft"));

        let svr_to_fft = self.dfs2(svr_id, fft_id, &vec![dac_id, out_id], &mut HashMap::new());
        let fft_to_dac = self.dfs2(fft_id, dac_id, &vec![svr_id, out_id], &mut HashMap::new());
        let dac_to_out = self.dfs2(dac_id, out_id, &vec![svr_id, fft_id], &mut HashMap::new());

        let svr_to_dac = self.dfs2(svr_id, dac_id, &vec![fft_id, out_id], &mut HashMap::new());
        let dac_to_fft = self.dfs2(dac_id, fft_id, &vec![svr_id, out_id], &mut HashMap::new());
        let fft_to_out = self.dfs2(fft_id, out_id, &vec![svr_id, dac_id], &mut HashMap::new());

        svr_to_fft * fft_to_dac * dac_to_out + svr_to_dac * dac_to_fft * fft_to_out
    }

    fn dfs2(&self, src: u32, dst: u32, nodes_to_exclude: &Vec<u32>, memo: &mut HashMap<u32, u128>) -> u128 {
        if nodes_to_exclude.contains(&src) {
            return 0;
        }

        if src == dst {
            return 1;
        }

        if let Some(&res) = memo.get(&src) {
            return res;
        }

        let mut res = 0;
        if let Some(neighbors) = self.g.get(&src) {
            for &neighbor in neighbors {
                res += self.dfs2(neighbor, dst, nodes_to_exclude, memo);
            }
        }
        memo.insert(src, res);
        res
    }
}

fn parse<R: BufRead>(reader: R) -> Result<Graph, Box<dyn Error>> {
    let mut graph = Graph::new();
    for line in reader.lines() {
        let line = line?;
        let line = line
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();
        let u = &line[0][0..line[0].len() -1];
        for i in 1..line.len() {
            let v = line[i];
            graph.insert(u.to_string(), v.to_string());
        }
    }
    Ok(graph)
}

fn main() -> Result<(), Box<dyn Error>> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, _r: i32) -> Result<u32, Box<dyn Error>> {
        Ok(parse(reader)?.solve_part1())
    }
    assert_eq!(5, part1(BufReader::new(TEST.as_bytes()), 10)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1000)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<u128, Box<dyn Error>> {
        Ok(parse(reader)?.solve_part2())
    }
    assert_eq!(2, part2(BufReader::new(TEST_PART_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
