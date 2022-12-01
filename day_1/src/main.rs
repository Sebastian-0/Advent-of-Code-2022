use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut calories = vec![];
    let mut sum = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            calories.push(sum);
            break;
        }

        match input.trim() {
            "" => {
                calories.push(sum);
                sum = 0;
            }
            num => sum += num.parse::<i32>().unwrap(),
        };
    }

    calories.sort();
    calories.reverse();

    println!("Max calories: {}", calories[0]);
    println!(
        "Top 3 calories: {}",
        calories[0] + calories[1] + calories[2]
    );

    Ok(())
}
