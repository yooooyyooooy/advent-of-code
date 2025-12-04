use std::cmp::min;

fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);
    println!("Part 2 Result: {}", result);
}

fn part2(input: &str) -> i64 {
    let mut result_list: Vec<i64> = vec![];
    let line: &str = input.lines().next().unwrap_or("");
    let id_ranges: Vec<&str> = line.split(",").collect();
    for id_range in id_ranges {
        let parts: Vec<&str> = id_range.split("-").collect();
        let start: i64 = parts[0].parse().unwrap_or(0);
        let end: i64 = parts[1].parse().unwrap_or(0);

        for id in start..end + 1 {
            let id_str = id.to_string();
            let mid = id_str.len() / 2;

            let mut invalid_id = false;
            for i in 1..mid + 1 {
                let chunks = create_chunks(&id_str, i);
                if !check_valid_chunks(chunks) {
                    invalid_id = true;
                }
            }

            if invalid_id {
                result_list.push(id);
            }
        }
    }

    result_list.iter().sum()
}

fn create_chunks(s: &str, chunk_size: usize) -> Vec<&str> {
    let mut chunks: Vec<&str> = vec![];
    let mut i = 0;
    while i < s.len() {
        let end = min(i + chunk_size, s.len());
        chunks.push(&s[i..end]);
        i += chunk_size
    }

    chunks
}

fn check_valid_chunks(c: Vec<&str>) -> bool {
    if c.len() == 1 {
        return true;
    }

    let mut valid_chunks = false;
    let first_chunk = c[0];

    for i in 1..c.len() {
        if first_chunk != c[i] {
            valid_chunks = true;
            break;
        }
    }

    valid_chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let test_input = include_str!("input-test.txt");
        let result: i64 = part2(test_input);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_create_chunks() {
        let test_input = "123412341";
        let result_1 = create_chunks(test_input, 2);
        let result_2 = create_chunks(test_input, 5);
        assert_eq!(result_1, vec!["12", "34", "12", "34", "1"]);
        assert_eq!(result_2, vec!["12341", "2341"]);
    }

    #[test]
    fn test_check_valid_chunks() {
        assert_eq!(check_valid_chunks(vec!["12", "12", "12", "1"]), true);
        assert_eq!(check_valid_chunks(vec!["12", "1", "12", "12"]), true);
        assert_eq!(check_valid_chunks(vec!["12"]), true);
        assert_eq!(check_valid_chunks(vec!["12", "12", "12", "12"]), false);
    }
}
