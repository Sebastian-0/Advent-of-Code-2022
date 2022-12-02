use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut score_pt_1 = 0;
    let mut score_pt_2 = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            break;
        }

        score_pt_1 += match input.trim() {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("Invalid input!"),
        };

        score_pt_2 += match input.trim() {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("Invalid input!"),
        };
    }

    println!("Score part 1: {}", score_pt_1);
    println!("Score part 2: {}", score_pt_2);

    Ok(())
}
