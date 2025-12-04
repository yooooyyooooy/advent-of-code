use std::{cmp::max, result};

fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);
    println!("Part 2 Result: {}", result);
}

fn part2(input: &str) -> i64 {
    let mut result = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        println!("current line: {}", line);
        let mut mx: i64 = 0;

        let batteries = gen_battery_combinations("".to_string(), line);
        for battery in batteries {
            let joltage: i64 = battery.parse().unwrap_or(0);
            mx = max(mx, joltage);
        }

        result += mx
    }

    result
}

fn gen_battery_combinations(curr_str: String, s_chunk: &str) -> Vec<String> {
    // base case
    if curr_str.len() + s_chunk.len() < 12 {
        return vec![];
    }

    if curr_str.len() == 12 && s_chunk.is_empty() {
        return vec![curr_str];
    }

    let mut result: Vec<String> = vec![];

    // recursion
    for (i, ch) in s_chunk.char_indices() {
        let new_str = format!("{}{}", curr_str, ch);
        let new_chunk = &s_chunk[i + 1..];
        result.extend(gen_battery_combinations(new_str, new_chunk));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let test_input = include_str!("input-test.txt");
        let result: i64 = part2(test_input);
        assert_eq!(result, 3121910778619);
    }
}
