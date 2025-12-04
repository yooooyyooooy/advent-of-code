fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);
    println!("Part 1 Result: {}", result);
}

fn part1(input: &str) -> i64 {
    let mut result_list: Vec<i64> = vec![];
    let line: &str = input.lines().next().unwrap_or("");
    let id_ranges: Vec<&str> = line.split(",").collect();
    for id_range in id_ranges {
        let parts: Vec<&str> = id_range.split("-").collect();
        let start: i64 = parts[0].parse().unwrap_or(0);
        let end: i64 = parts[1].parse().unwrap_or(0);

        for id in start..end + 1 {
            let id_str = id.to_string();

            // odd length id -> always valid
            if id_str.len() % 2 != 0 {
                continue;
            }

            let mid = id_str.len() / 2;
            let (left, right) = id_str.split_at(mid);
            if left == right {
                result_list.push(id);
            }
        }
    }

    result_list.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = include_str!("input-test.txt");
        let result: i64 = part1(test_input);
        assert_eq!(result, 1227775554);
    }
}
