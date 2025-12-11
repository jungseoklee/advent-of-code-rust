use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{Itertools};
use adv_code_2025::*;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
/*const TEST: &str = "\
1,1
1,2
2,2
2,3
1,3
1,4
4,4
4,1
";*/

#[derive(Debug, Clone, Copy)]
struct Tile {
    row: i128,
    col: i128,
}

impl Tile {
    fn new(row: i128, col: i128) -> Self {
        Self { row, col }
    }

    fn area(&self, other: &Tile) -> i128 {
        let dr = self.row.abs_diff(other.row) + 1;
        let dc = self.col.abs_diff(other.col) + 1;
        (dr * dc) as i128
    }
}


impl From<&str> for Tile {
    fn from(s: &str) -> Self {
        let (col, row) = s
            .split_once(',')
            .map(|(col, row)| (col.parse().unwrap(), row.parse().unwrap()))
            .unwrap();

        Self::new(row, col)
    }
}

struct Grid {
    rows: Vec<i128>,
    cols: Vec<i128>,
    tiles: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    r1: i128,
    c1: i128,
    r2: i128,
    c2: i128,
}

impl Rect {
    fn new(r1: i128, c1: i128, r2: i128, c2: i128) -> Self {
        Self {
            r1: r1.min(r2),
            c1: c1.min(c2),
            r2: r1.max(r2),
            c2: c1.max(c2),
        }
    }
}

impl Grid {

    fn get_axis(mut v: Vec<i128>) -> Vec<i128> {
        let min = *v.iter().min().unwrap();
        let max = *v.iter().max().unwrap();
        v.push(min - 1);
        v.push(max + 1);
        v.sort();
        v.dedup();
        v
    }

    fn new(input_tiles: &Vec<Tile>) -> Self {
        let rows = Self::get_axis(input_tiles.iter().map(|t| t.row).collect());
        let cols = Self::get_axis(input_tiles.iter().map(|t| t.col).collect());
        let tiles = vec![vec!['i'; cols.len()]; rows.len()];
        let mut grid = Self {
            rows,
            cols,
            tiles,
        };
        grid.mark_boundary(input_tiles);
        grid.mark_outside();
        grid
    }

    fn mark_boundary(&mut self, input_tiles: &[Tile]) {
        for (t1, t2) in input_tiles.iter().circular_tuple_windows() {
            let rect = self.get_rect(t1, t2);
            if rect.r1 == rect.r2 {
                for c in rect.c1..=rect.c2 {
                    self.tiles[rect.r1 as usize][c as usize] = 'b';
                }
            } else {
                for r in rect.r1..=rect.r2 {
                    self.tiles[r as usize][rect.c1 as usize] = 'b';
                }
            }
        }
    }

    fn mark_outside(&mut self) {
        let dr = [1, -1, 0, 0];
        let dc = [0, 0, 1, -1];
        let mut q = VecDeque::new();

        let s = Tile { row: 0, col: 0 };
        self.tiles[s.row as usize][s.col as usize] = 'o';
        q.push_back(s);
        while let Some(t) = q.pop_front() {
            for i in 0..4 {
                let nr = t.row + dr[i];
                let nc = t.col + dc[i];
                if nr < 0 || nc < 0 || nr >= self.rows.len() as i128 || nc >= self.cols.len() as i128 {
                    continue;
                }
                if self.tiles[nr as usize][nc as usize] == 'i' {
                    self.tiles[nr as usize][nc as usize] = 'o';
                    q.push_back(Tile { row: nr, col: nc });
                }
            }
        }
    }

    fn get_rect(&self, t1: &Tile, t2: &Tile) -> Rect {
        let (r1, c1) = (
            self.rows.binary_search(&t1.row).unwrap() as i128,
            self.cols.binary_search(&t1.col).unwrap() as i128,
        );
        let (r2, c2) = (
            self.rows.binary_search(&t2.row).unwrap() as i128,
            self.cols.binary_search(&t2.col).unwrap() as i128,
        );
        Rect::new(r1, c1, r2, c2)
    }

    fn valid(&self, rect: Rect) -> bool {
        (rect.r1..=rect.r2)
            .cartesian_product(rect.c1..=rect.c2)
            .all(|(r, c)| self.tiles[r as usize][c as usize] != 'o')
    }
}

fn parse<R: BufRead>(reader: R) -> Result<Vec<Tile>, Box<dyn Error>> {
    let mut tiles: Vec<Tile> = Vec::new();
    for line in reader.lines() {
        tiles.push(Tile::from(line?.trim()));
    }
    Ok(tiles)
}

fn main() -> Result<(), Box<dyn Error>> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, r: i32) -> Result<i128, Box<dyn Error>> {
        let tiles = parse(reader)?;
        let mut ans = 0;
        let n = tiles.len();
        (0..n)
            .cartesian_product(0..n)
            .for_each(|(i, j)| {
                ans = std::cmp::max(ans, tiles[i].area(&tiles[j]));
            });
        Ok(ans)
    }
    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()), 10)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 1000)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<i128, Box<dyn Error>> {
        // Referred to https://gist.github.com/icub3d/6282ddab0b1d012ef054a9f212b12973 for a core idea, grid compaction, and idiomatic expressions.
        let tiles = parse(reader)?;
        let grid = Grid::new(&tiles);
        let ans = tiles
            .iter()
            .enumerate()
            .flat_map(|(i, t1)| {
                tiles[i + 1..]
                    .iter()
                    .map(|t2| (t1.area(t2), grid.get_rect(t1, t2)))
            })
            .filter(|(_, rect)| grid.valid(*rect))
            .map(|(area, _)| area)
            .max()
            .unwrap();
        Ok(ans)
    }
    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

