use std::{collections::HashSet, io};

struct Node {
    x: i32,
    y: i32,
    tail: Option<Box<Node>>,
}

impl Node {
    fn new(tail: Option<Box<Node>>) -> Self {
        Self { x: 0, y: 0, tail }
    }
}

struct State {
    head: Node,
    seen: HashSet<(i32, i32)>,
}

fn signum(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn update_tail(seen: &mut HashSet<(i32, i32)>, head: &mut Node) {
    let tail = match &mut head.tail {
        Some(t) => t.as_mut(),
        None => {
            seen.insert((head.x, head.y));
            return;
        }
    };

    let dx = (tail.x - head.x).abs();
    let dy = (tail.y - head.y).abs();
    let max_dist = dx.max(dy);

    if max_dist == 2 {
        tail.x += signum(head.x, tail.x);
        tail.y += signum(head.y, tail.y);
    }

    update_tail(seen, tail);
}

fn do_move(state: &mut State, dx: i32, dy: i32, steps: usize) {
    for _ in 0..steps {
        state.head.x += dx;
        state.head.y += dy;
        update_tail(&mut state.seen, &mut state.head);
    }
}

fn solve(instructions: &[(String, usize)], tail_length: usize) -> usize {
    let mut head = Node::new(None);
    for _ in 0..tail_length {
        head = Node::new(Some(Box::new(head)));
    }
    let mut state = State {
        head,
        seen: HashSet::new(),
    };

    for (dir, count) in instructions.iter() {
        match dir.as_str() {
            "U" => do_move(&mut state, 0, 1, *count),
            "D" => do_move(&mut state, 0, -1, *count),
            "R" => do_move(&mut state, 1, 0, *count),
            "L" => do_move(&mut state, -1, 0, *count),
            _ => (),
        };
    }
    state.seen.len()
}

fn main() -> Result<(), std::io::Error> {
    let mut instructions: Vec<(String, usize)> = vec![];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.is_empty() {
            break;
        }
        let input = input.trim();

        let (dir, count) = input.split_once(' ').unwrap();
        let count = count.parse().unwrap();
        instructions.push((dir.into(), count));
    }

    println!("Part 1: {}", solve(&instructions, 1));
    println!("Part 2: {}", solve(&instructions, 9));
    Ok(())
}
