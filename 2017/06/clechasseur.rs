use std::collections::HashMap;

use crate::input::day_6::INPUT;

pub fn part_1() -> usize {
    realloc_loop().cycles()
}

pub fn part_2() -> usize {
    realloc_loop().loop_len()
}

#[derive(Debug)]
struct LoopStatus {
    first_seen_at: usize,
    next_seen_at: usize,
}

impl LoopStatus {
    fn cycles(&self) -> usize {
        self.next_seen_at
    }

    fn loop_len(&self) -> usize {
        self.next_seen_at - self.first_seen_at
    }
}

fn realloc_loop() -> LoopStatus {
    let mut banks: Vec<_> = INPUT.into();
    let mut seen = HashMap::new();
    let mut cycles = 0;

    seen.insert(banks.clone(), 0);
    loop {
        let (mut i, max_blocks) = banks
            .iter_mut()
            .enumerate()
            .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
            .unwrap();

        let mut redist = *max_blocks;
        *max_blocks = 0;

        while redist > 0 {
            i = (i + 1) % banks.len();
            banks[i] += 1;
            redist -= 1;
        }

        cycles += 1;
        if let Some(first_seen_at) = seen.insert(banks.clone(), cycles) {
            break LoopStatus { first_seen_at, next_seen_at: cycles };
        }
    }
}
