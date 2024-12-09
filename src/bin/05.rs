advent_of_code::solution!(5);


pub fn part_one(input: &str) -> Option<u32> {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();

    let mut map = [[false; 100]; 100];

    sections[0]
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .for_each(|(a, b)| { map[a][b] = true; } );

    let groups: Vec<Vec<u32>> = sections[1]
        .lines()
        .map(|line| {
            line.split(",")
                .filter_map(|n| {
                    n.trim().parse::<u32>().ok()
                })
                .collect()
        })
        .collect();

    let mut res: u32 = 0;

    for group in groups {
        let mut valid = true;
        for i in 0..(group.len()-1) {
            for j in (i+1)..group.len() {
                if !map[group[i] as usize][group[j] as usize] {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        }

        if valid {
            res += group[(group.len()) / 2];
        }
    }

    res.into()
}

use  std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;

pub fn part_two(input: &str) -> Option<u32> {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();

    let mut map = [[Less; 100]; 100];

    sections[0]
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .for_each(|(a, b)| { map[a][b] = Greater; } );

    let groups: Vec<Vec<u32>> = sections[1]
        .lines()
        .map(|line| {
            line.split(",")
                .filter_map(|n| {
                    n.trim().parse::<u32>().ok()
                })
                .collect()
        })
        .collect();

    let mut res: u32 = 0;
    for mut group in groups {
        let mut valid = true;
        for i in 0..(group.len()-1) {
            for j in (i+1)..group.len() {
                if map[group[i] as usize][group[j] as usize] == Less {
                    valid = false;
                    break;
                }
            }
            if !valid {
                break;
            }
        }

        if !valid {
            let mid = group.len() / 2;
            group.select_nth_unstable_by(mid, |a, b| {
                map[*a as usize][*b as usize]
            });
            res += group[mid];
        }
    }

    res.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = part_one(input);
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
