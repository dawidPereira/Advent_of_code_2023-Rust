use std::fs;

fn main() {
    let file_path = "../src/files/day_01.txt";
    let result: i32 = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Failed to read the file. Path {} | Error {}", file_path, err))
        .lines()
        .filter_map(|s| {
            let digits: Vec<char> = s.chars()
                .filter(|c|
                    c.is_digit(10))
                .collect();
            if digits.is_empty() {
                None
            } else {
                Some(format!("{}{}", digits[0], digits[digits.len() - 1])
                    .parse::<i32>()
                    .unwrap_or(0))
            }
        })
        .sum();

    println!("Result: {:?}", result);
}