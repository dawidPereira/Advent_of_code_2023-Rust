use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "../src/files/day_01.txt";
    let map = get_digits();

    let result: i32 = fs::read_to_string(file_path)
        .unwrap_or_else(|err| panic!("Failed to read the file. Path {} | Error {}", file_path, err))
        .lines()
        .filter_map(|s| {
            let digits = get_digit_chars(s.chars().collect::<Vec<char>>(), &map);
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

fn get_digit_chars(mut line_chars: Vec<char>, map: &HashMap<&'static str, &'static str>) -> Vec<char>{
    let mut digits = Vec::new();
    for _ in 0..line_chars.len() {
        let char = line_chars.get(0).unwrap();
        if char.is_digit(10) {
            digits.push(*char);
            line_chars.remove(0);
            continue;
        }
        for (key, value) in map {
            if line_chars.iter().take(key.len()).collect::<String>() == *key{
                digits.push(value.chars().nth(0).unwrap());
            }
        }
        line_chars.remove(0);

    }
    digits
}
fn get_digits() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("one", "1");
    map.insert("two", "2");
    map.insert("three", "3");
    map.insert("four", "4");
    map.insert("five", "5");
    map.insert("six", "6");
    map.insert("seven", "7");
    map.insert("eight", "8");
    map.insert("nine", "9");
    map
}