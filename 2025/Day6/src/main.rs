use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");
    let mut swapped = Vec::<Vec::<i64>>::new();
    let mut count = 0;


    for (i, line) in contents.lines().enumerate() {
        if i < contents.lines().count() - 1 {
            let res = get_numbers(line);
            for (j, digits) in res.iter().enumerate() {
                if j >= swapped.len() {
                    swapped.push(Vec::new());
                }
                swapped[j].push(*digits);
            }
        } else {
            let res = get_operators(line);
            for (x, operator) in res.iter().enumerate() {
                let mut sum = 0;
                if operator == &'+' {
                    for num in swapped[x].iter() {
                        sum += num;
                    }
                } else if operator == &'*' {
                    for (y, num) in swapped[x].iter().enumerate() {
                        if y == 0 { sum = *num }
                        else { sum *= num };
                    }
                }
                count += sum;
            }
        }
    }

    println!("Problem one solution: {}", count);
}

fn get_numbers(s: &str) -> Vec<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    return RE.find_iter(s)
        .filter_map(|pattern_match| pattern_match.as_str().parse().ok())
        .collect();
}

fn get_operators(s: &str) -> Vec<char> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\*|\+").unwrap();
    }
    return RE.find_iter(s)
        .filter_map(|pattern_match| pattern_match.as_str().chars().next())
        .collect();
}