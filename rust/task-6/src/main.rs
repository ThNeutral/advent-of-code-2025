use std::fs;

fn calc_expression(operands: &Vec<u64>, operator: &str) -> u64 {
    match operator {
        "+" => operands.iter().fold(0, |prev, next| prev + *next),
        "*" => operands.iter().fold(1, |prev, next| prev * *next),
        _ => panic!("Invalid operator {}", operator),
    }
}

fn main() {
    task1();
    task2();
}

fn task2() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let lines: Vec<&str> = file.lines().collect();

    let row_count = lines.len();
    let row_length = lines[0].len();

    let mut all_operand_sets: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    let mut current_operand_set: Vec<u64> = Vec::new();
    for row_index in 0..row_length {
        let mut column = Vec::with_capacity(row_count - 1);
        for column_index in 0..row_count - 1 {
            let char = lines[column_index].as_bytes()[row_index] as char;
            column.push(char);
        }

        let operator = lines[row_count - 1].as_bytes()[row_index] as char;
        if operator != ' ' {
            operators.push(operator);
        }

        let mut empty = 0;
        let mut column_str: String = String::from("");
        for char in column {
            if char == ' ' {
                empty += 1;
            }

            column_str += String::from(char).as_str();
        }

        if empty == column_str.len() {
            all_operand_sets.push(current_operand_set);
            current_operand_set = Vec::new();
            continue;
        }

        current_operand_set.push(column_str.trim().parse().unwrap());
    }

    all_operand_sets.push(current_operand_set);

    let mut answer = 0;
    for i in 0..all_operand_sets.len() {
        let operands = &all_operand_sets[i];

        let s = String::from(operators[i]);
        let operator = s.as_str();
        answer += calc_expression(operands, operator)
    }

    println!("{}", answer);
}

fn task1() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let lines: Vec<Vec<&str>> = file
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let row_count = lines.len();
    let row_length = lines[0].len();

    let mut answer: u64 = 0;
    for row_index in 0..row_length {
        let mut operands: Vec<u64> = Vec::new();
        for column_index in 0..row_count - 1 {
            operands.push(lines[column_index][row_index].parse().unwrap());
        }
        let operator = lines[row_count - 1][row_index];
        answer += calc_expression(&operands, operator);
    }

    println!("{}", answer);
}
