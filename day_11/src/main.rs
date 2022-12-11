#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: fn(u64) -> u64,
    test: fn(u64) -> usize,
    inspections: usize,
}

fn round(monkies: &mut Vec<Monkey>, modulus: Option<u64>) {
    let mut new_items = vec![vec![]; monkies.len()];

    for i in 0..monkies.len() {
        let monkey = &mut monkies[i];

        for item in monkey.items.iter() {
            monkey.inspections += 1;
            let worry = if let Some(md) = modulus {
                (monkey.operation)(*item) % md
            } else {
                (monkey.operation)(*item) / 3
            };
            let idx = (monkey.test)(worry);
            new_items[idx].push(worry);
        }
        monkey.items.clear();

        for j in 0..monkies.len() {
            monkies[j].items.append(&mut new_items[j]);
        }
    }
}

fn solve(mut monkies: Vec<Monkey>, rounds: usize, modulus: Option<u64>) -> usize {
    for _ in 0..rounds {
        round(&mut monkies, modulus);
    }

    let mut inspections: Vec<usize> = monkies.iter().map(|m| m.inspections).collect();
    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

fn monkeys_example() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![79, 98],
            operation: |v| v * 19,
            test: |v| if v % 23 == 0 { 2 } else { 3 },
            inspections: 0,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: |v| v + 6,
            test: |v| if v % 19 == 0 { 2 } else { 0 },
            inspections: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: |v| v * v,
            test: |v| if v % 13 == 0 { 1 } else { 3 },
            inspections: 0,
        },
        Monkey {
            items: vec![74],
            operation: |v| v + 3,
            test: |v| if v % 17 == 0 { 0 } else { 1 },
            inspections: 0,
        },
    ]
}

fn monkeys_full() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![72, 97],
            operation: |v| v * 13,
            test: |v| if v % 19 == 0 { 5 } else { 6 },
            inspections: 0,
        },
        Monkey {
            items: vec![55, 70, 90, 74, 95],
            operation: |v| v * v,
            test: |v| if v % 7 == 0 { 5 } else { 0 },
            inspections: 0,
        },
        Monkey {
            items: vec![74, 97, 66, 57],
            operation: |v| v + 6,
            test: |v| if v % 17 == 0 { 1 } else { 0 },
            inspections: 0,
        },
        Monkey {
            items: vec![86, 54, 53],
            operation: |v| v + 2,
            test: |v| if v % 13 == 0 { 1 } else { 2 },
            inspections: 0,
        },
        Monkey {
            items: vec![50, 65, 78, 50, 62, 99],
            operation: |v| v + 3,
            test: |v| if v % 11 == 0 { 3 } else { 7 },
            inspections: 0,
        },
        Monkey {
            items: vec![90],
            operation: |v| v + 4,
            test: |v| if v % 2 == 0 { 4 } else { 6 },
            inspections: 0,
        },
        Monkey {
            items: vec![88, 92, 63, 94, 96, 82, 53, 53],
            operation: |v| v + 8,
            test: |v| if v % 5 == 0 { 4 } else { 7 },
            inspections: 0,
        },
        Monkey {
            items: vec![70, 60, 71, 69, 77, 70, 98],
            operation: |v| v * 7,
            test: |v| if v % 3 == 0 { 2 } else { 3 },
            inspections: 0,
        },
    ]
}

fn main() -> Result<(), std::io::Error> {
    let (monkies, md) = (monkeys_example(), 96577);
    // let (monkies, md) = (monkeys_full(), 9699690);

    println!("Part 1: {}", solve(monkies.clone(), 20, None));
    println!("Part 2: {}", solve(monkies, 10_000, Some(md)));
    Ok(())
}
