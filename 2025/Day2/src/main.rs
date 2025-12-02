use std::fs;
use std::vec::Vec;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");
    let mut ranges = Vec::<(i64, i64)>::new();

    for line in contents.lines() {
        for pair in line.trim().split(",") {
            let bounds: Vec<_> = pair.split("-").collect();
            let start: i64 = bounds[0].trim().parse().expect("Invalid number string Bound0");
            let end: i64 = bounds[1].trim().parse().expect("Invalid number string Bound1: ");
            ranges.push((start, end));
        }
    }

    // problem one
    let mut count = 0;
    for range in &ranges {
        for x in range.0..=range.1 {
            let str = x.to_string();
            let halves = str.split_at(str.len() / 2);
            if halves.0 == halves.1 { count += x }
        }
    }

    println!("Problem one solution: {}", count);


    // problem 2
    count = 0;

    for range in &ranges {
        for x in range.0..=range.1 {
            let str = x.to_string();
            for y in 0..=str.len()/2 {
                if y == 0 { continue };
                let chunks: Vec<&str> = sub_strings(&str, y);

                for z in 0..=chunks.len()-1 {
                    if z == 0 { continue };
                    if chunks[z] != chunks[z-1] { break };
                    if z == chunks.len() { count += x; break; };
                    break;
                }
            }
        }
    }

    println!("Problem two solution: {}", count);
}

fn sub_strings(string: &str, sub_len: usize) -> Vec<&str> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(&string[pos..pos + len]);
        pos += len;
    }
    subs
}