advent_of_code::solution!(4);

const DIRS: [(i32, i32); 8] = [
    (0, 1),  (1, 0),  (-1, 0),  (0, -1),
    (1, 1),  (1, -1), (-1, -1), (-1, 1),
];

const WORD: [char; 4] = [
    'X','M','A','S'
];

pub fn count(arr: &[Vec<char>], r: i32, c: i32) -> u32 {
    let mut count = 0;

    for (x, y) in DIRS {
        if (1..4).all(|i| {
            let tmp_r = r + x * i;
            let tmp_c = c + y * i;

            tmp_r >= 0
                && tmp_r < arr.len() as i32
                && tmp_c >= 0
                && tmp_c < arr[0].len() as i32
                && WORD[i as usize] == arr[tmp_r as usize][tmp_c as usize]
        }) {
            count += 1;
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut res = 0;

    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 'X' {
                res += count(&grid, i as i32, j as i32);
            }
        }
    }

    res.into()
}

pub fn count_two(arr: &Vec<Vec<char>>, r: i32, c: i32) -> Option<bool> {
    let mut counts: [i32; 2] = [0, 0];

    if r == 0
        || c == 0
        || r == arr.len() as i32 - 1
        || c == arr[0].len() as i32 - 1
    {
        return None;
    }

    for (x, y) in [(-1, -1), (1, 1), (-1, 1), (1, -1)] {
        let tmp_r: usize = (r + x).try_into().ok()?;
        let tmp_c: usize = (c + y).try_into().ok()?;
        let chr = arr[tmp_r][tmp_c];

        if chr == 'M' {
            counts[0] += 1;
        } else if chr == 'S' {
            counts[1] += 1;
        }
    }

    if counts[0] != 2
        || counts[1] != 2
        || arr[(r - 1) as usize][(c - 1) as usize] == arr[(r + 1) as usize][(c + 1) as usize] {
        return None;
    }

    Some(true)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let result = (1..grid.len() - 1)
        .flat_map(|i| (1..grid[0].len() - 1).map(move |j| (i, j)))
        .filter(|&(i, j)|
            grid[i][j] == 'A' && count_two(&grid, i as i32, j as i32).is_some())
        .count() as u32;

    Some(result)
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
