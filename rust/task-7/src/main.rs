use std::{collections::HashSet, fs};

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    let manifold_height = grid.len();
    let manifold_length = grid[0].len();

    let mut paths = Vec::new();
    for i in 0..manifold_height {
        paths.push(vec![0_u64; manifold_length]);
    }

    let mut splits = 0;
    for i in 1..manifold_height {
        for j in 0..manifold_length {
            match grid[i - 1][j] {
                'S' => {
                    paths[i][j] = 1;
                }
                '^' => {
                    if paths[i - 1][j] != 0 {
                        splits += 1;
                    }
                    paths[i][j + 1] += paths[i - 1][j];
                    paths[i][j - 1] += paths[i - 1][j];
                    paths[i - 1][j] = 0;
                }
                '.' => {
                    paths[i][j] += paths[i - 1][j];
                    paths[i - 1][j] = 0;
                }
                _ => {
                    panic!("Unexpected symbol");
                }
            }
        }
    }

    let mut timelines = 0;
    for &path in paths.last().unwrap() {
        timelines += path;
    }

    println!("SPLITS {}", splits);
    println!("TIMELINES {}", timelines);
}
