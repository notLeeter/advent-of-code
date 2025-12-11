use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");
    let mut ranges = Vec::<(i64, i64)>::new();
    let mut count = 0;

    for line in contents.lines() {
        if line.contains("-") {
            let range = line.split("-").collect::<Vec<&str>>();
            let start = range[0].parse::<i64>().unwrap();
            let end = range[1].parse::<i64>().unwrap();

            ranges.push((start, end));
        } else if !line.is_empty(){
            let i = line.parse::<i64>().unwrap();

            for range in &ranges {
                if i >= range.0 && i <= range.1 {
                    count += 1;
                    break
                }
            }
        }
    }

    println!("Problem two solution: {}", count);
}