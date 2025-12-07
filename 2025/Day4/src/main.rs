use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");
    let lines: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            let mut counter = 0;

            for dj in -1isize..=1 {
                for di in -1isize..=1 {
                    if di != 0 || dj != 0 {
                        let nr = row as isize + dj;
                        let nc = col as isize + di;

                        if nr >= 0 && nc >= 0
                            && (nr as usize) < lines.len()
                            && (nc as usize) < lines[nr as usize].len()
                        {
                            if lines[nr as usize][nc as usize] == '@' {
                                counter += 1;
                            }
                        }
                    }
                }
            }

            if counter < 4 && *ch == '@' {
                count += 1;
            }
        }
    }

    println!("Problem one solution: {}", count);
}