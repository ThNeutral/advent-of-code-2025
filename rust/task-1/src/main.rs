use std::fs;

fn count_passes(prev: i16, next: i16) -> i16 {
    let mut passes = 0;

    if prev / 100 != next / 100 {
        let diff = (prev.abs() / 100 - next.abs() / 100).abs();
        passes = diff;
    }

    if prev.signum() != next.signum() && prev != 0 {
        passes += 1;
    }

    return passes;
}

fn main() {
    let file = fs::read_to_string("./input_georgi.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();
    // let lines = vec![
    //     "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    // ];

    let mut answer = 0;
    let mut initial_rotation = 50;
    lines
        .iter()
        .map(|l| l.split_at(1))
        .map(|(dir, step)| {
            let mut sign = -1;
            if dir == "R" {
                sign = 1
            }
            let step_number = step.parse::<i16>().unwrap();
            return sign * step_number;
        })
        .for_each(|step| {
            let previous_rotation = initial_rotation;
            let next_rotation = previous_rotation + step;

            answer += count_passes(previous_rotation, next_rotation);

            initial_rotation = next_rotation % 100;
        });

    println!("{}", answer);
}
