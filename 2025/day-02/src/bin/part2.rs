fn main() {
    let input = include_str!("input-2.txt");
    let result = part2(input);
    println!("Part 2 Result: {}", result);
}

fn part2(input: &str) -> i32 {
    let mut result: i32 = 0;

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
