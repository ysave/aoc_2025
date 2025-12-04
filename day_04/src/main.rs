use std::fs;

fn main() {
    // --snip--

    let file = fs::read_to_string("rolls.txt")
        .expect("Should have been able to read the file");

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in file.lines() {
        let chars: Vec<char> = line.chars().collect(); 
        grid.push(chars);
    }
    part1(&grid);
    part2(&mut grid);
}

fn part1(grid: &Vec<Vec<char>>) {
    let mut accessible_rolls = 0;
    let dirs = [
        (-1,-1), (-1,0), (-1,1),
        (0,-1),          (0,1),
        (1,-1),  (1,0),  (1,1),
    ];

    for (r, row) in grid.iter().enumerate() {
        for (c, _col) in row.iter().enumerate() {
            let current = grid[r][c];
            if current != '@' {
                continue;
            }
            let mut neighbors = 0;

            for (dr, dc) in dirs.iter() {
                let new_r = r as isize + dr;
                let new_c = c as isize + dc;

                if new_r < 0 || new_c < 0 { continue; } //check if in bounds
                let new_r = new_r as usize;
                let new_c = new_c as usize;

                if new_r >= grid.len() || new_c >= grid[0].len() { continue; } //check if in bounds

                if grid[new_r][new_c] == '@' {
                    neighbors += 1;
                }
            }

            if neighbors < 4 {
               accessible_rolls += 1;
            }
        }   
    }
    println!("Solution Part One: {}", accessible_rolls);
}


fn part2(grid: &mut Vec<Vec<char>>) {
    let mut accessible_rolls = 0;
    let mut accessible_roll_changes = true;
    let dirs = [
        (-1,-1), (-1,0), (-1,1),
        (0,-1),          (0,1),
        (1,-1),  (1,0),  (1,1),
    ];

    while accessible_roll_changes {
        let rolls_before = accessible_rolls;
        let mut to_remove = Vec::new();

        for (r, row) in grid.iter().enumerate() {
            for (c, _col) in row.iter().enumerate() {
                if grid[r][c] != '@' { continue; }

                let mut neighbors = 0;

                for (dr, dc) in dirs {
                    let new_r = r as isize + dr;
                    let new_c = c as isize + dc;

                    if new_r < 0 || new_c < 0 { continue; } //check if in bounds
                    let (new_r,new_c) = (new_r as usize, new_c as usize);

                    if new_r >= grid.len() || new_c >= grid[0].len() { continue; } //check if in bounds

                    if grid[new_r][new_c] == '@' {
                        neighbors += 1;
                    }
                }

                if neighbors < 4 {
                    accessible_rolls += 1;
                    to_remove.push((r,c));
                }
            }
        }

        for (r,c) in to_remove {
            grid[r][c] = '.';
        }

        if rolls_before == accessible_rolls {
            accessible_roll_changes = false;
        }
    }

    println!("Solution Part Two: {}", accessible_rolls);
}