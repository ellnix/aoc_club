use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::iter::successors;
use std::str::FromStr;

use itertools::Itertools;

use crate::input::day_21::INPUT;

pub fn part_1() -> usize {
    on_count_after(5)
}

pub fn part_2() -> usize {
    on_count_after(18)
}

fn on_count_after(iterations: usize) -> usize {
    iterate(Rules::default())
        .nth(iterations)
        .unwrap()
        .on_count()
}

const INITIAL_PATTERN: &str = ".#./..#/###";

fn iterate(rules: Rules) -> impl Iterator<Item = Pattern> {
    let pattern: Pattern = INITIAL_PATTERN.parse().unwrap();

    successors(Some(pattern), move |pattern| Some(pattern.enhance(&rules)))
}

/// Represents a pattern that we can match and enhance.
///
/// Also used for the full image, since it's pretty much the same idea. (I think.)
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pattern(Vec<String>);

impl Pattern {
    pub fn size(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of pixels in the pattern that are `on` (e.g. `#`).
    pub fn on_count(&self) -> usize {
        self.0
            .iter()
            .map(|line| line.bytes().filter(|&b| b == b'#').count())
            .sum()
    }

    /// Splits this pattern into 2x2 or 3x3 sub-patterns (depending on the pattern's size),
    /// enhances each pattern by applying the corresponding rule, then reconstructs the larger
    /// pattern using all enhanced sub-patterns.
    pub fn enhance(&self, rules: &Rules) -> Self {
        let chunk_size = if self.size() % 2 == 0 { 2 } else { 3 };

        Self(
            self.0
                .chunks(chunk_size)
                .flat_map(|lines_chunk| {
                    // `lines_chunk` contains all lines required to build a line of sub-patterns.

                    // Split the lines into sub-patterns and transform each one using the rules.
                    // This produces a vector of enhanced sub-patterns.
                    let enhanced_line = (0..self.size() / chunk_size)
                        .map(move |chunk_idx| {
                            let start = chunk_idx * chunk_size;

                            Self(
                                lines_chunk
                                    .iter()
                                    .map(|line| line[start..start + chunk_size].to_string())
                                    .collect_vec(),
                            )
                        })
                        .map(|pattern| rules.enhance(&pattern))
                        .collect_vec();

                    // Turn the line of sub-patterns into joined lines of the larger pattern.
                    (0..=chunk_size).map(move |line_idx| {
                        enhanced_line
                            .iter()
                            .map(|pattern| &pattern.0[line_idx])
                            .join("")
                    })

                    // At this point, `flat_map` will flatten all the joined lines,
                    // which will form the large enhanced pattern. We can gather those
                    // into a vector and create a new Pattern with it.
                })
                .collect_vec(),
        )
    }

    /// Converts a pattern into all possible combinations that could match it,
    /// by flipping it horizontally and vertically, then generating all rotations.
    ///
    /// In theory, this generates 16 combinations per pattern, but in practice there
    /// are usually collisions so the rules will contain less than that.
    pub fn into_combinations(self) -> impl Iterator<Item = Self> {
        vec![self.flip_horizontally(), self]
            .into_iter()
            .flat_map(|pattern| vec![pattern.flip_vertically(), pattern])
            .flat_map(Self::rotations)
    }

    fn flip_horizontally(&self) -> Self {
        Self(self.0.iter().rev().cloned().collect_vec())
    }

    fn flip_vertically(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(|line| String::from_utf8(line.bytes().rev().collect_vec()).unwrap())
                .collect_vec(),
        )
    }

    /// Returns an iterator for the columns in the pattern, from left to right
    /// (e.g., it transposes the matrix).
    ///
    /// (Mental note: steal this code for the `matrix` exercise on Exercism.org)
    fn columns(&self) -> impl DoubleEndedIterator<Item = String> + '_ {
        (0..self.size()).map(|col_idx| {
            String::from_utf8(
                self.0
                    .iter()
                    .map(|line| line.as_bytes()[col_idx])
                    .collect_vec(),
            )
            .unwrap()
        })
    }

    fn rotate_left(&self) -> Self {
        Self(self.columns().rev().collect_vec())
    }

    /// Returns an iterator of all 4 possible rotations of this pattern.
    fn rotations(self) -> impl Iterator<Item = Self> {
        successors(Some(self), |pattern| Some(pattern.rotate_left())).take(4)
    }
}

impl FromStr for Pattern {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split("/").map(Into::into).collect_vec()))
    }
}

impl Display for Pattern {
    /// Displays this pattern in a user-friendly manner.
    ///
    /// - `{}` will give you a compact version (e.g. `../.#`).
    /// - `{:#}` will give you a multi-line version.
    ///
    /// (This method isn't used to solve the puzzle, I only used it for debugging.)
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sep = if f.alternate() { "\n" } else { "/" };
        write!(f, "{}", self.0.join(sep))
    }
}

/// Stores all enhancement rules for the puzzle. Essentially just a wrapper over a `HashMap`.
#[derive(Debug, Clone)]
struct Rules(HashMap<Pattern, Pattern>);

impl Rules {
    /// Enhances the given pattern by applying the corresponding rule.
    pub fn enhance(&self, pattern: &Pattern) -> Pattern {
        self.0
            .get(pattern)
            .unwrap_or_else(|| panic!("no rule found for '{pattern}'"))
            .clone()
    }
}

impl FromStr for Rules {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .flat_map(|line| {
                    let (from, to) = line
                        .split(" => ")
                        .map(|pat| pat.parse::<Pattern>().unwrap())
                        .collect_tuple()
                        .unwrap();

                    from.into_combinations()
                        .map(move |pattern| (pattern, to.clone()))
                })
                .collect(),
        ))
    }
}

impl Default for Rules {
    fn default() -> Self {
        INPUT.parse().unwrap()
    }
}
