use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Range {
    lower: u64,
    upper: u64,
}

impl FromStr for Range {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let lower = parts.next().unwrap().parse()?;
        let upper = parts.next().unwrap().parse()?;

        Ok(Self { lower, upper })
    }
}

impl Range {
    fn is_contained(first: Range, second: Range) -> bool {
        first.lower >= second.lower && first.upper <= second.upper
    }

    fn overlap(first: Range, second: Range) -> bool {
        first.lower <= second.lower && first.upper >= second.lower
            || second.lower <= first.lower && second.upper >= first.lower
    }
}

#[derive(Debug, Clone, Copy)]
struct Assignment {
    first: Range,
    second: Range,
}

impl Assignment {
    fn has_contained_ranges(self) -> bool {
        Range::is_contained(self.first, self.second) || Range::is_contained(self.second, self.first)
    }

    fn has_overlapping_ranges(self) -> bool {
        Range::overlap(self.first, self.second)
    }
}

impl FromStr for Assignment {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let first = parts.next().unwrap().parse()?;
        let second = parts.next().unwrap().parse()?;

        Ok(Self { first, second })
    }
}

trait RangeInclusiveExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T> RangeInclusiveExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.contains(self.start()) || other.contains(self.end())
    }
}

fn mine() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let input = include_str!("../input.txt");

    let result_iter = input.lines().map(Assignment::from_str);
    let answer = itertools::process_results(result_iter, |iter| {
        iter.map(|x| dbg!(x))
            .map(Assignment::has_overlapping_ranges)
            .filter(|&x| x)
            .count()
    });

    dbg!(answer);

    Ok(())
}

use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let input = include_str!("../input.txt");

    let answer = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|part| {
                    part.split('-')
                        .map(|value| value.parse::<u32>().unwrap())
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .unwrap()
                })
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .filter(|(first, second)| first.overlaps_or_is_overlapped(second))
        .count();
    dbg!(answer);

    Ok(())
}
