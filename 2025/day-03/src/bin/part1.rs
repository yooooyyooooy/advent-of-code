use std::cmp::max;

fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Part 1 Result: {}", result);
}

fn part1(input: &str) -> i64 {
    let mut result = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let mut mx: i64 = 0;
        let batteries: Vec<&str> = line
            .char_indices()
            .map(|(i, _)| {
                return &line[i..i + 1];
            })
            .collect();

        for i in 0..batteries.len() - 1 {
            for j in i + 1..batteries.len() {
                let joltage: i64 = format!("{}{}", batteries[i], batteries[j])
                    .parse()
                    .unwrap_or(0);

                mx = max(mx, joltage);
            }
        }

        result += mx
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = include_str!("input-test.txt");
        let result: i64 = part1(test_input);
        assert_eq!(result, 357);
    }
}
