advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .fold(0, |acc, cap| {
            let x: u32 = cap[1].parse().unwrap();
            let y: u32 = cap[2].parse().unwrap();
            acc + (x * y)
        }).into()
}



pub fn part_two(input: &str) -> Option<u32> {
    let re_mul = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_do = Regex::new(r"^do\(\)").unwrap();
    let re_dont = Regex::new(r"^don't\(\)").unwrap();

    let mut enabled = true;
    let mut total: u32 = 0;

    let mut position = 0;
    while position < input.len() {
        if let Some(_) = re_do.find(&input[position..]) {
            enabled = true;
            position += 4;
        } else if let Some(_) = re_dont.find(&input[position..]) {
            enabled = false;
            position += 7;
        } else if let Some(cap) = re_mul.captures(&input[position..]) {
            let x: u32 = cap[1].parse().unwrap();
            let y: u32 = cap[2].parse().unwrap();

            if enabled {
                total += x * y;
            }

            position += cap[0].len();
        } else {
            position += 1;
        }
    }

    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, result);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, result);
    }
}
