use std::io;

fn parse_range(range: &str) -> (usize, usize) {
    let (a, b) = range.split_once('-').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn main() -> Result<(), std::io::Error> {
    let mut num_overlap_fully = 0;
    let mut num_overlap_partially = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            break;
        }
        let input = input.trim();

        let mut tokens = input.split(',');
        let r1 = parse_range(tokens.next().unwrap());
        let r2 = parse_range(tokens.next().unwrap());

        if r1.0 >= r2.0 && r1.1 <= r2.1 || r2.0 >= r1.0 && r2.1 <= r1.1 {
            num_overlap_fully += 1;
        }

        if !(r1.1 < r2.0 || r1.0 > r2.1) {
            num_overlap_partially += 1;
        }
    }

    println!("Part 1: {}", num_overlap_fully);
    println!("Part 2: {}", num_overlap_partially);

    Ok(())
}
