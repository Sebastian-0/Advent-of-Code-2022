use std::{collections::HashSet, io};

fn score(c: &char) -> u32 {
    if c.is_lowercase() {
        1 + *c as u32 - 'a' as u32
    } else {
        27 + *c as u32 - 'A' as u32
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut score_wrong_pocket = 0;
    let mut score_shared_item = 0;

    let mut intersection = HashSet::<char>::new();
    let mut idx = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            break;
        }
        let input = input.trim();

        // Part one, score for item in wrong pocket
        let left = input[..input.len() / 2].chars().collect::<HashSet<char>>();
        let right = input[input.len() / 2..].chars().collect::<HashSet<char>>();

        for c in left.intersection(&right) {
            score_wrong_pocket += score(c);
        }

        // Part two, score for shared item in group of three lines
        if idx % 3 == 0 {
            intersection = input.chars().collect();
        } else {
            intersection = intersection
                .intersection(&input.chars().collect())
                .cloned()
                .collect();
        }

        if idx % 3 == 2 {
            for c in intersection.iter() {
                score_shared_item += score(c)
            }
        }

        idx += 1;
    }

    println!("Score part 1: {}", score_wrong_pocket);
    println!("Score part 2: {}", score_shared_item);

    Ok(())
}
