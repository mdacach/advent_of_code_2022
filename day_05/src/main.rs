use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::{all_consuming, map, opt};
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

#[derive(Debug)]
struct Crate(char);

fn parse_crate(input: &str) -> IResult<&str, Crate> {
    let mut parser = delimited(tag("["), take(1_usize), tag("]"));
    map(parser, |single_char_string: &str| {
        Crate(single_char_string.chars().next().unwrap())
    })(input)
}

fn parse_hole(input: &str) -> IResult<&str, ()> {
    // `drop` returns nothing
    // so we parse with tag (which advances on the input string) and then drop the value
    // essentially we discard the holes
    map(tag("   "), drop)(input)
}

fn parse_crate_or_hole(input: &str) -> IResult<&str, Option<Crate>> {
    let wrap_into_some = Some;
    let just_output_none = |_v| None;
    alt((
        map(parse_crate, wrap_into_some),
        map(parse_hole, just_output_none),
    ))(input)
}

fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    // Parse the first token
    let (mut tail, maybe_c) = parse_crate_or_hole(input)?;
    let mut maybe_crates = vec![maybe_c];

    loop {
        // After that, the tokens are preceded with (" ")
        // and they may end, so it's `opt`ional.
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(tail)?;
        if let Some(c) = maybe_c {
            maybe_crates.push(c);
            // Our part was already consumed, so we continue with the tail
            tail = next_i;
        } else {
            break; // We are done
        }
    }

    Ok((tail, maybe_crates))
}

fn main() {
    let input = include_str!("../example.txt");
    let mut crate_lines = Vec::new();

    for line in input.lines() {
        // `all_consuming` fails if there's any leftover input
        // (our `parse_crate_line` should indeed consume all the line
        if let Ok((_empty_tail, parsed_crates)) = all_consuming(parse_crate_line)(line) {
            crate_lines.push(parsed_crates);
        }
    }

    for line in &crate_lines {
        println!("{line:?}");
    }
}

mod my_solution {
    use std::collections::VecDeque;
    use std::str::FromStr;

    #[derive(Debug, Copy, Clone)]
    struct Move {
        quantity: u64,
        source: u64,
        destination: u64,
    }

    impl FromStr for Move {
        type Err = color_eyre::Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split_whitespace();
            parts.next(); // 'move'
            let quantity = parts.next().unwrap().parse::<u64>()?;
            parts.next(); // 'from'
            let source = parts.next().unwrap().parse::<u64>()?;
            parts.next(); // 'to'
            let destination = parts.next().unwrap().parse::<u64>()?;

            Ok(Self {
                quantity,
                source,
                destination,
            })
        }
    }

    #[derive(Debug, Clone)]
    struct State {
        stacks: Vec<VecDeque<char>>,
    }

    impl State {
        fn new(number_stacks: usize) -> Self {
            let mut stacks: Vec<_> = Vec::new();
            for _ in 0..number_stacks {
                stacks.push(VecDeque::new());
            }

            Self { stacks }
        }

        fn add_to_front_of_stack(&mut self, stack_number: usize, to_add: char) {
            self.stacks[stack_number].push_front(to_add);
        }

        // Assumes move is valid given state
        fn perform_move(&mut self, m: Move) {
            let Move {
                quantity,
                source,
                destination,
            } = m;

            for _ in 0..quantity {
                let char = self.stacks[(source - 1) as usize].pop_back().unwrap();
                self.stacks[(destination - 1) as usize].push_back(char);
            }
        }

        // Assumes move is valid given state
        fn perform_batch_move(&mut self, m: Move) {
            let Move {
                quantity,
                source,
                destination,
            } = m;

            let mut crates = Vec::new();
            for _ in 0..quantity {
                let char = self.stacks[(source - 1) as usize].pop_back().unwrap();
                crates.push(char);
            }
            crates.reverse();
            for c in crates {
                self.stacks[(destination - 1) as usize].push_back(c);
            }
        }

        fn get_topmost_crates(&self) -> Vec<char> {
            let mut res = Vec::new();
            for s in &self.stacks {
                res.push(*s.back().unwrap());
            }

            res
        }
    }

    fn main() -> color_eyre::Result<()> {
        let input = include_str!("../input.txt");

        let mut input = input.split("\n\n");
        let input_crates = input.next().unwrap();
        let input_moves = input.next().unwrap();

        // TODO: hard-coded
        let number_stacks = 9;
        let mut state = State::new(number_stacks);

        for line in input_crates.lines() {
            dbg!(line);
            let mut current_stack = 0;
            let mut char_position = 1;
            let mut skip_position = 3;
            for (i, c) in line.chars().enumerate() {
                if i == char_position {
                    if c.is_alphabetic() {
                        state.add_to_front_of_stack(current_stack, c);
                    }
                    current_stack += 1;
                    char_position += 4;
                } else if i == skip_position {
                    skip_position += 4
                }
            }
        }

        input_moves
            .lines()
            .map(Move::from_str)
            .map(|x| dbg!(x.unwrap()))
            .for_each(|m| state.perform_batch_move(m));

        dbg!(state.get_topmost_crates());

        Ok(())
    }
}
