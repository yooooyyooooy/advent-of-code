fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Part 1 Result: {}", result);
}

fn part1(input: &str) -> i32 {
    let mut result: i32 = 0;
    let mut dial: i32 = 50;
    let ls: Vec<&str> = input.lines().collect();
    for item in &ls {
        let (action, times) = item.split_at(1);
        let times: i32 = times.parse().unwrap_or(0);

        match action {
            "L" => {
                dial = (dial - times) % 100;
            }
            "R" => {
                dial = (dial + times) % 100;
            }
            _ => {}
        }

        if dial == 0 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = include_str!("input-test.txt");
        let result: i32 = part1(test_input);
        assert_eq!(result, 3);
    }
}
