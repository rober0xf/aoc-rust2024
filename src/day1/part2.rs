use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let file_path = "./src/day1/input.txt";
    let numbers = fs::read_to_string(file_path).expect("could not read the file");

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    // returns an iterator over the rows of the string
    for row in numbers.lines() {
        // parts will an iterator over the numbers of the line. separated with spaces. and cast the to i64
        let mut parts = row
            .split_whitespace()
            .map(|s| s.parse::<i64>().expect("invalid number"));

        // if there are two values, we assign them to left and right
        if let (Some(l), Some(r)) = (parts.next(), parts.next()) {
            left.push(l);
            right.push(r);
        }
    }

    // an unordered map to count the frequency of each number
    let mut freq: HashMap<i64, i64> = HashMap::new();

    // we can iterate over the vector like this. its more safe and easier
    for &n in &right {
        *freq.entry(n).or_insert(0) += 1;
    }

    let mut sum: i64 = 0;

    // if the actual value in left is in the map then multiply it for the value
    for &n in &left {
        if let Some(&count) = freq.get(&n) {
            sum += n * count;
        }
    }

    println!("result: {}", sum);
}
