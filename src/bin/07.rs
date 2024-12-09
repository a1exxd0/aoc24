use advent_of_code::template::commands::solve;

advent_of_code::solution!(7);

fn solve_one(target: u64, items: &Vec<u64>, i: usize, curr: u64) -> bool {
    if curr > target {
        return false;
    } else if curr == target && i == items.len() {
        return true;
    } else if i == items.len() {
        return false;
    } else {
        return solve_one(target, items, i + 1, curr + items[i])
            || solve_one(target, items, i + 1, curr * items[i]);
    }
}

fn solve_two(target: u64, items: &Vec<u64>, i: usize, curr: u64) -> bool {
    if curr > target {
        return false;
    } else if curr == target && i == items.len() {
        return true;
    } else if i == items.len() {
        return false;
    } else {
        let concatenated: u64 = format!("{}{}", curr, items[i]).parse().unwrap();
        return solve_two(target, items, i + 1, curr + items[i])
            || solve_two(target, items, i + 1, curr * items[i])
            || solve_two(target, items, i + 1, concatenated);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.split_once(':'));

    let mut res: u64 = 0;

    for line in lines {
        if let Some((_t, _elems)) = line {
            let (target, items): (u64, Vec<u64>) = (
                _t.parse().unwrap(),
                _elems
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            );

            if solve_one(target, &items, 0, 0) {
                res += target;
            }
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.split_once(':'));

    lines
        .filter_map(|opt| {
            if let Some((_t, _elems)) = opt {
                let target: u64 = _t.parse().ok()?;
                let items = _elems
                    .split_whitespace()
                    .map(|n| n.parse().ok())
                    .collect::<Option<Vec<u64>>>()?;

                if solve_two(target, &items, 0, 0) {
                    Some(target)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum::<u64>()
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
