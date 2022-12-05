use std::io;

use regex::Regex;

fn main() -> Result<(), std::io::Error> {
    let mut crates: Vec<Vec<char>> = vec![];

    // Read crates
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input == "\n" {
            break;
        }

        let mut line_idx = 1;
        while line_idx < input.len() {
            if crates.len() < line_idx / 4 + 1 {
                crates.push(Vec::new());
            }

            let char = input.as_bytes()[line_idx] as char;
            if char != ' ' && !('0'..='9').contains(&char) {
                crates[line_idx / 4].insert(0, char);
            }

            line_idx += 4;
        }
    }

    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut reverse_crates = crates.clone();

    // Read moves
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            break;
        }
        let input = input.trim();

        let captures = regex.captures(input).unwrap();

        let count = captures[1].parse::<usize>().unwrap();
        let from = captures[2].parse::<usize>().unwrap() - 1;
        let to = captures[3].parse::<usize>().unwrap() - 1;

        let vals: Vec<char> = (0..count).map(|_| crates[from].pop().unwrap()).collect();
        vals.into_iter().for_each(|v| crates[to].push(v));

        let vals: Vec<char> = (0..count)
            .map(|_| reverse_crates[from].pop().unwrap())
            .collect();
        vals.into_iter()
            .rev()
            .for_each(|v| reverse_crates[to].push(v));
    }

    let top_names: String = crates.iter().map(|c| c.last().unwrap()).collect();
    let top_names_reverse: String = reverse_crates.iter().map(|c| c.last().unwrap()).collect();
    println!("Part 1: {}", top_names);
    println!("Part 2: {}", top_names_reverse);

    Ok(())
}
