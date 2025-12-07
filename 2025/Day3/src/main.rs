use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");
    let mut count = 0;

    for line in contents.lines() {
        let mut largest = '0';
        let mut second = '0';

        for (i, c) in line.chars().enumerate() {
            if i == line.len() - 1 {
                if c > second { second = c };
            } else if c > largest {
                largest = c;
                second = '0';
            } else if c > second {
                second = c;
            }
        }

        let largest_num = largest.to_digit(10).unwrap();
        let second_num = second.to_digit(10).unwrap();
        count += largest_num * 10 + second_num;
    }

    println!("Problem one solution: {}", count);
}