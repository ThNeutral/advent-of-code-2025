use std::{cmp, fs};

fn main() {
    part1();
    part2();
}

fn part2() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();

    let input: Vec<&str> = file_contents.split("\n\n").collect();

    let ranges = input[0];

    let mut limits: Vec<(_, _)> = ranges
        .lines()
        .map(|line| {
            let parts: Vec<u64> = line
                .trim()
                .split("-")
                .map(|part| part.parse().unwrap())
                .collect();

            return (parts[0], parts[1]);
        })
        .collect();

    limits.sort_by(|(left_lower_bound, _), (right_lower_bound, _)| {
        left_lower_bound.partial_cmp(right_lower_bound).unwrap()
    });

    let (answer, _) = limits
        .iter()
        .fold((0, 0), |prev, (lower_bound, higher_bound)| {
            let (mut total_count, mut highest_reached_bound) = prev;

            if *lower_bound > highest_reached_bound + 1 {
                total_count += (higher_bound - lower_bound) + 1;
                highest_reached_bound = *higher_bound;
            } else {
                let added_length = higher_bound.saturating_sub(highest_reached_bound);
                total_count += added_length;

                highest_reached_bound = cmp::max(*higher_bound, highest_reached_bound);
            }

            return (total_count, highest_reached_bound);
        });

    println!("{}", answer);
}

fn part1() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();

    let input: Vec<&str> = file_contents.split("\n\n").collect();

    let ranges = input[0];
    let ids = input[1];

    let limits: Vec<(u64, u64)> = ranges
        .lines()
        .map(|line| {
            let parts: Vec<u64> = line
                .trim()
                .split("-")
                .map(|part| part.parse().unwrap())
                .collect();

            return (parts[0], parts[1]);
        })
        .collect();

    let answer: u64 = ids
        .lines()
        .map(|id| id.parse::<u64>().unwrap())
        .fold(0, |answer, id| {
            for (lower_limit, upper_limit) in &limits {
                if id >= *lower_limit && id <= *upper_limit {
                    return answer + 1;
                }
            }
            return answer;
        });

    println!("{}", answer);
}
