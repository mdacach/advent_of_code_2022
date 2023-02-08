use itertools::Itertools;
use std::collections::HashSet;

// So that we can't bypass the TryFrom and populate item with an invalid u8
mod item {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Item(u8);

    impl From<u8> for Item {
        fn from(value: u8) -> Self {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Item(value),
                _ => panic!(),
            }
        }
    }

    impl Item {
        pub fn priority(self) -> u64 {
            match self.0 {
                b'a'..=b'z' => 1 + (self.0 - b'a') as u64,
                b'A'..=b'Z' => 27 + (self.0 - b'A') as u64,
                _ => panic!(),
            }
        }
    }
}

fn solve_for_group(group: &[&str]) -> char {
    // Split into two equal length strings
    let (first, second, third) = (group[0], group[1], group[2]);

    let mut first_set = HashSet::new();
    for item in first.chars() {
        first_set.insert(item);
    }

    let mut second_set = HashSet::new();
    for item in second.chars() {
        second_set.insert(item);
    }

    for item in third.chars() {
        if first_set.contains(&item) && second_set.contains(&item) {
            return item;
        }
    }

    panic!();
}

use item::Item;

fn main() -> color_eyre::Result<()> {
    let input = include_str!("../input.txt");
    let sacks = input.lines().map(|line| {
        line.bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()
    });

    let sum = itertools::process_results(sacks, |rs| {
        rs.tuples()
            .map(|(a, b, c)| {
                a.iter()
                    .find(|i| b.contains(i) && c.contains(i))
                    .map(|i| dbg!(i.priority()))
                    .unwrap_or_default()
            })
            .sum::<u64>()
    })?;
    dbg!(sum);

    Ok(())
}
