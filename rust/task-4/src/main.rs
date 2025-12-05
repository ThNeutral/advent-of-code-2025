use std::fs;

fn get_directions() -> Vec<(i32, i32)> {
    return vec![
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ];
}

fn is_reachable(grid: &Vec<Vec<u8>>, index: (i32, i32)) -> bool {
    let (row, column) = index;
    let item = grid[row as usize][column as usize];
    if item != '@' as u8 {
        return false;
    }

    let mut neighbour_rolls = 0;

    for direction in get_directions() {
        let (row_offset, column_offset) = direction;
        let next_row = row + row_offset;
        if next_row < 0 || next_row > grid.len() as i32 - 1 {
            continue;
        }

        let next_column = column + column_offset;
        if next_column < 0 || next_column > grid[0].len() as i32 - 1 {
            continue;
        }

        let next_item = grid[next_row as usize][next_column as usize];
        if next_item == '@' as u8 {
            neighbour_rolls += 1;
        }
    }

    return neighbour_rolls < 4;
}

fn main() {
    let mut grid: Vec<Vec<u8>> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| Vec::from(line.as_bytes()))
        .collect();

    let mut answer = 0;

    loop {
        let mut indexes_to_remove: Vec<(i32, i32)> = Vec::new();
        for (row_index, row) in grid.iter().enumerate() {
            let row_index = row_index as i32;
            for (column_index, _) in row.iter().enumerate() {
                let column_index = column_index as i32;
                if is_reachable(&grid.clone(), (row_index, column_index)) {
                    answer += 1;
                    indexes_to_remove.push((row_index, column_index));
                }
            }
        }

        for (row, column) in &indexes_to_remove {
            grid[*row as usize][*column as usize] = '.' as u8;
        }

        if indexes_to_remove.len() == 0 {
            break;
        }
    }

    println!("{}", answer);
}
