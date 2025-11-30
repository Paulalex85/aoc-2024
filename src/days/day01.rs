use crate::days::Day;
use itertools::Itertools;
use winnow::ascii::{digit1, newline, space1};
use winnow::combinator::{opt, separated, separated_pair, terminated};
use winnow::{Parser, Result};

pub struct Day01;

pub struct Side {
    left: Vec<usize>,
    right: Vec<usize>,
}

fn parse_line(input: &mut &str) -> Result<(usize, usize)> {
    separated_pair(digit1.parse_to(), space1, digit1.parse_to()).parse_next(input)
}

impl Day for Day01 {
    type Input = Side;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let lines: Vec<_> =
            terminated(separated(1.., parse_line, newline), opt(newline)).parse_next(input)?;
        let (left, right) = lines.into_iter().unzip();
        Ok(Side { left, right })
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.left.clone().sort();
        input.right.clone().sort();
        input
            .left
            .iter()
            .sorted()
            .zip(input.right.iter().sorted())
            .map(|(a, b)| a.abs_diff(*b))
            .sum()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
