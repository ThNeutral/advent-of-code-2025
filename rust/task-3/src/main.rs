use std::fs;

fn get_maximum_power(battery_pack: &str) -> u64 {
    let len = battery_pack.len();
    let length_range: Vec<_> = (len - 11..len + 1).collect();
    let mut values: Vec<String> = Vec::with_capacity(12);

    let mut offset = 0;
    for length in length_range {
        println!("{}, {}, {}", length, len, offset);
        let (left_index, left_value) = battery_pack
            .split("")
            .enumerate()
            .filter(|(index, _)| *index <= length)
            .fold((0, 0), |prev, (index, item)| {
                if index < offset + 1 {
                    return prev;
                }

                if item.is_empty() {
                    return prev;
                }

                let (prev_index, prev_value) = prev;
                let next: u32 = item.parse().unwrap();
                if next > prev_value {
                    return (index, next);
                }
                return prev;
            });

        values.push(left_value.to_string());
        offset = left_index;
    }

    return values.join("").parse().unwrap();
}

fn main() {
    // let lines: Vec<_> = fs::read_to_string("./input.txt").unwrap().lines().collect();
    // let lines = vec![
    //     "987654321111111",
    //     "811111111111119",
    //     "234234234234278",
    //     "818181911112111",
    // ];

    let answer = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .fold(0, |prev, item| {
            let max_power = get_maximum_power(item);
            println!("{} {}", item, max_power);
            return prev + max_power;
        });

    println!("{}", answer);
}
