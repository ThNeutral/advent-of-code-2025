use std::{collections::linked_list::ExtractIf, fs};

fn is_invalid_id(id: u64) -> bool {
    let id_string = id.to_string();

    let allowed_parts: Vec<&str> = id_string
        .as_bytes()
        .iter()
        .take(id_string.len() / 2 + 1)
        .enumerate()
        .filter(|&(index, _)| index != 0)
        .map(|(index, _)| {
            let slice = &id_string.as_bytes()[0..index];
            return str::from_utf8(slice).unwrap();
        })
        .collect();

    for allowed_part in allowed_parts {
        if id_string.len() % allowed_part.len() != 0 {
            continue;
        }

        let invalid = id_string
            .as_bytes()
            .chunks(allowed_part.len())
            .fold(true, |prev, next| {
                let str = str::from_utf8(next).unwrap();
                let is_equal = str == allowed_part;
                return prev & is_equal;
            });

        if invalid {
            return true;
        }
    }

    return false;
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    // let input = String::from(
    //     "11-22,95-115,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    // );

    let mut answer = 0;
    input
        .split(',')
        .map(|part| {
            let parts: Vec<_> = part.split('-').collect();
            return (parts[0], parts[1]);
        })
        .map(|(left, right)| return (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()))
        .for_each(|(start, end)| {
            for id in start..end + 1 {
                if is_invalid_id(id) {
                    answer += id;
                }
            }
        });

    println!("{}", answer);
}
