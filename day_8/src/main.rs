use std::io;

fn mark_visible(
    visible: &mut Vec<Vec<bool>>,
    grid: &Vec<Vec<u8>>,
    xs: usize,
    ys: usize,
    dx: i32,
    dy: i32,
) {
    let inside = |x: i32, y: i32| -> bool {
        x >= 0 && y >= 0 && x < grid[0].len() as i32 && y < grid.len() as i32
    };

    let mut highest = -1;
    let mut x = xs as i32;
    let mut y = ys as i32;
    while inside(x, y) {
        let height = grid[y as usize][x as usize] as i32;
        if height > highest {
            highest = height;
            visible[y as usize][x as usize] = true;
        }

        x += dx;
        y += dy;
    }
}

fn max_visible_score(grid: &Vec<Vec<u8>>) -> usize {
    let mut visible = vec![vec![false; grid[0].len()]; grid.len()];

    for r in 0..grid.len() {
        mark_visible(&mut visible, grid, 0, r, 1, 0);
        mark_visible(&mut visible, grid, grid[0].len() - 1, r, -1, 0);
    }

    for c in 0..grid[0].len() {
        mark_visible(&mut visible, grid, c, 0, 0, 1);
        mark_visible(&mut visible, grid, c, grid.len() - 1, 0, -1);
    }

    visible.iter().flatten().filter(|v| **v).count()
}

fn view_distance(grid: &Vec<Vec<u8>>, xs: usize, ys: usize, dx: i32, dy: i32) -> i32 {
    let inside = |x: i32, y: i32| -> bool {
        x >= 0 && y >= 0 && x < grid[0].len() as i32 && y < grid.len() as i32
    };

    let height = grid[ys][xs];
    let mut score = 0;
    let mut x = xs as i32 + dx;
    let mut y = ys as i32 + dy;
    while inside(x, y) {
        score += 1;
        let h = grid[y as usize][x as usize];
        if h >= height {
            break;
        }

        x += dx;
        y += dy;
    }
    score
}

fn max_scenic_score(grid: &Vec<Vec<u8>>) -> i32 {
    let mut score = 0;
    for r in 1..grid.len() - 1 {
        for c in 1..grid[0].len() - 1 {
            score = score.max(
                view_distance(grid, c, r, 1, 0)
                    * view_distance(grid, c, r, -1, 0)
                    * view_distance(grid, c, r, 0, 1)
                    * view_distance(grid, c, r, 0, -1),
            );
        }
    }
    score
}

fn main() -> Result<(), std::io::Error> {
    let mut grid: Vec<Vec<u8>> = vec![];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            break;
        }
        let input = input.trim();

        grid.push(input.as_bytes().iter().map(|c| c - b'0').collect());
    }

    let visible_count = max_visible_score(&grid);
    println!("Part 1: {}", visible_count);

    let score = max_scenic_score(&grid);
    println!("Part 2: {}", score);

    Ok(())
}
