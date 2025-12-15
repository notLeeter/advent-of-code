use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input").expect("Error reading example");
    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();

    let mut beams: HashSet<usize> = HashSet::new();

    let start_col = grid[0].iter().position(|&c| c == 'S').unwrap();
    beams.insert(start_col);

    let mut count = 0;

    for row in 1..height {
        let mut next_beams = HashSet::new();
        for &col in &beams {
            match grid[row][col] {
                '.' => {
                    next_beams.insert(col);
                }
                '^' => {
                    count += 1;
                    if col > 0 {
                        next_beams.insert(col - 1);
                    }
                    if col + 1 < width {
                        next_beams.insert(col + 1);
                    }
                }
                _ => {}
            }
        }

        beams = next_beams;
        if beams.is_empty() {
            break;
        }
    }

    println!("{}", count);
}
