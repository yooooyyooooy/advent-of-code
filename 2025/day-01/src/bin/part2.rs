fn main() {
    let input = include_str!("input-2.txt");
    let result = part2(input);
    println!("Part 2 Result: {}", result);
}

fn part2(input: &str) -> i32 {
    let mut result: i32 = 0;
    let mut dial: i32 = 50;
    let ls: Vec<&str> = input.lines().collect();
    for item in &ls {
        let (action, times) = item.split_at(1);
        let mut times: i32 = times.parse().unwrap_or(0);
        result += times / 100;
        times = times % 100;

        match action {
            "L" => {
                if dial == 0 {
                    dial = 100;
                }

                dial -= times;

                if dial <= 0 {
                    result += 1;
                }
            }
            "R" => {
                if dial == 100 {
                    dial = 0;
                }

                dial += times;

                if dial >= 100 {
                    result += 1;
                }
            }
            _ => {}
        }

        dial = dial.rem_euclid(100);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let test_input = include_str!("input-test-2.txt");
        let result: i32 = part2(test_input);
        assert_eq!(result, 26);
    }
}
