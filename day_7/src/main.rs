use std::{collections::HashMap, io};

enum Entry {
    File((String, usize)),
    Dir((String, HashMap<String, Entry>)),
}

struct Reader;
impl Reader {
    fn next(&self) -> Option<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.is_empty() {
            None
        } else {
            Some(input.trim().to_owned())
        }
    }
}

fn construct_tree(reader: &Reader, entries: &mut HashMap<String, Entry>) {
    loop {
        let stmt = match reader.next() {
            Some(stmt) => stmt,
            None => return,
        };

        match stmt.as_str() {
            "$ cd .." => return,
            "$ ls" => (),
            _ if stmt.starts_with("$ cd ") => {
                let name: String = stmt.chars().skip(5).collect();
                let entry = match entries.get_mut(&name) {
                    Some(e) => e,
                    None => {
                        entries.insert(name.clone(), Entry::Dir((name.clone(), HashMap::new())));
                        entries.get_mut(&name).unwrap()
                    }
                };

                match entry {
                    Entry::File(_) => panic!("File and dir with same name!"),
                    Entry::Dir((_, ref mut entries)) => construct_tree(reader, entries),
                }
            }
            _ if stmt.starts_with("dir ") => {
                let name: String = stmt.chars().skip(4).collect();
                if entries.get(&name).is_none() {
                    entries.insert(name.clone(), Entry::Dir((name, HashMap::new())));
                }
            }
            _ => {
                let (size, name) = stmt.split_once(' ').unwrap();
                if entries.get(name).is_none() {
                    entries.insert(
                        name.into(),
                        Entry::File((name.into(), size.parse().unwrap())),
                    );
                }
            }
        }
    }
}

fn dir_sum(entry: &Entry) -> (usize, usize) {
    match entry {
        Entry::File((_, size)) => (*size, 0),
        Entry::Dir((_, entries)) => {
            let (mut total_size, mut total_sum) = (0, 0);
            for e in entries.values() {
                let (size, sum) = dir_sum(e);
                total_size += size;
                total_sum += sum;
            }
            if total_size < 100_000 {
                total_sum += total_size;
            }
            (total_size, total_sum)
        }
    }
}

fn find_closest_size(entry: &Entry, target: usize) -> (usize, Option<usize>) {
    match entry {
        Entry::File((_, size)) => (*size, None),
        Entry::Dir((_, entries)) => {
            let mut total_size = 0;
            let mut total_closest: Option<usize> = None;
            for e in entries.values() {
                let (size, closest) = find_closest_size(e, target);
                total_size += size;
                if let Some(closest) = closest {
                    total_closest = total_closest.map(|v| closest.min(v)).or(Some(closest));
                }
            }
            if total_size >= target {
                total_closest = total_closest
                    .map(|v| total_size.min(v))
                    .or(Some(total_size));
            }
            (total_size, total_closest)
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let reader = Reader;
    reader.next(); // Skip 'cd /'

    let mut entries = HashMap::new();
    construct_tree(&reader, &mut entries);
    let root = Entry::Dir(("/".into(), entries));

    let (size, sum) = dir_sum(&root);
    println!("Part 1: {}", sum);

    let (_, closest) = find_closest_size(&root, 30_000_000 - (70_000_000 - size));
    println!("Part 2: {}", closest.unwrap());

    Ok(())
}
