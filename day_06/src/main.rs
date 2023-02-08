use itertools::Itertools;
use std::collections::HashSet;

fn all_different<'a>(iter: impl Iterator<Item = &'a char> + Clone) -> bool {
    let iter_clone = iter.clone();
    let set: HashSet<_> = iter_clone.collect();

    let size = iter.count();

    set.len() == size
}

fn main() {
    let input = include_str!("../input.txt");

    let bytes: Vec<char> = input.chars().collect();
    bytes
        .windows(14)
        .map(|x| all_different(x.iter()))
        .position(|x| x)
        .map(|x| dbg!(x + 14));
}
