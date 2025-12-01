use std::fs;

fn main() {
    // problem one
    let contents = fs::read_to_string("input").expect("Failed to read input file");

    let mut val = 50;
    let mut count = 0;

    for line in contents.lines() {
        let vals = line.split_at(1);
        let direction = if vals.0 == "L" { -1 } else { 1 };
        val += direction * vals.1.parse::<i32>().expect("Failed to parse number");
        while val < 0 {
            val += 100;
        }
        val %= 100;

        if val == 0 { count += 1 };
    }

    println!("Problem one solution: {}", count);


    // problem two

    val = 50;
    count = 0;
    let mut cache = 50;

    for line in contents.lines() {
        let vals = line.split_at(1);
        let direction = if vals.0 == "L" { -1 } else { 1 };
        let movement = direction * vals.1.parse::<i32>().expect("Failed to parse number");

        if movement >= 100 {
            count += movement / 100;
            val += movement % 100;
        } else if movement <= -100 {
            count += (-movement) / 100;
            val -= (-movement) % 100;
        } else {
            val += movement
        }


        if val < 0 {
            val += 100;
            if cache != 0 { count += 1; }
        } else if val >= 100 {
            val = val % 100;
            count += 1;
        } else if val == 0 { count += 1 };
        cache = val;
    }

    println!("Problem two solution: {}", count);
}