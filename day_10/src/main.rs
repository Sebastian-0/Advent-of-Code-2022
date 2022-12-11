use std::io;

struct Cpu {
    x: i32,
    cycle: i32,
    signal_strength_sum: i32,
    screen: Vec<Vec<char>>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            cycle: 0,
            signal_strength_sum: 0,
            screen: vec![vec![' '; 40]; 6],
        }
    }

    fn cycle(&mut self) {
        self.cycle += 1;
        if (self.cycle - 20) % 40 == 0 {
            self.signal_strength_sum += self.cycle * self.x;
        }
        let row = (self.cycle - 1) / self.screen[0].len() as i32;
        let pixel = (self.cycle - 1) % self.screen[0].len() as i32;
        self.screen[row as usize][pixel as usize] = if self.x - 1 <= pixel && self.x + 1 >= pixel {
            '#'
        } else {
            '.'
        }
    }

    fn print_screen(&self) {
        for row in self.screen.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut cpu = Cpu::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            break;
        }
        let input = input.trim();

        match input {
            "noop" => {
                cpu.cycle();
            }
            addx => {
                cpu.cycle();
                cpu.cycle();
                cpu.x += addx.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            }
        };
    }

    println!("Part 1: {}", cpu.signal_strength_sum);
    println!("Part 2:");
    cpu.print_screen();
    Ok(())
}
