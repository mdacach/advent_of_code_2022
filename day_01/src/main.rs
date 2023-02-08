use itertools::Itertools;

fn main() {
    let lines = include_str!("../input.txt");
    let answer: u64 = lines
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|iter| {
            let mut block_sum = None;
            // While we have numbers
            // if the next value is an empty line, it will not contain Some(v)
            // so we break from the while
            while let Some(Some(v)) = iter.next() {
                block_sum = Some(block_sum.unwrap_or(0) + v);
            }
            block_sum
        })
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("Answer: {answer:?}");
}
