use std::{collections::HashSet, io};

fn find_index_with_unique(line: &str, num_unique: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    for i in 0..bytes.len() - num_unique + 1 {
        let mut set: HashSet<u8> = HashSet::new();
        (i..i + num_unique).for_each(|j| {
            set.insert(bytes[j]);
        });
        if set.len() == num_unique {
            return Some(i);
        }
    }
    None
}

fn main() -> Result<(), std::io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    println!("Part 1: {}", find_index_with_unique(input, 4).unwrap() + 4);
    println!(
        "Part 2: {}",
        find_index_with_unique(input, 14).unwrap() + 14
    );

    Ok(())
}
