advent_of_code::solution!(2);

fn solve_one_line(input: &str) -> bool {
    let vec: Vec<u32> = input
        .split_whitespace()
        .filter_map(|item| item.parse().ok())
        .collect();

    vec.windows(2).all(|w| {
        w[0] > w[1] &&
        w[0] - w[1] > 0 &&
        w[0] - w[1] <= 3
    }) ||
    vec.windows(2).all(|w| {
        w[1] > w[0] &&
        w[1] - w[0] > 0 &&
        w[1] - w[0] <= 3
    })
}

fn solve_two(line: &Vec<u32>) -> bool {
    for i in 0..line.len() {
        let curr: Vec<u32> = line[..i].iter()
            .chain(&line[i+1..])
            .cloned()
            .collect();

        let satisfied =
            curr.windows(2).all(|w| {
                w[0] > w[1] &&
                w[0] - w[1] > 0 &&
                w[0] - w[1] <= 3
            }) ||
            curr.windows(2).all(|w| {
                w[1] > w[0] &&
                w[1] - w[0] > 0 &&
                w[1] - w[0] <= 3
            });
        if satisfied { return true; }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .fold(0, |acc, line| {
            if solve_one_line(line) {
                acc + 1
            } else {
                acc
            }
        })
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines()
        .fold(0, |acc, line| {
            let vec: Vec<u32> = line.split_whitespace()
                .filter_map(|item| item.parse().ok())
                .collect();
            if solve_two(&vec) || solve_one_line(line) {
                acc + 1
            } else {
                acc
            }
        })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
