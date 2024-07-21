use crate::input::day_13::INPUT; // &[Layer]

pub fn part_1() -> usize {
    INPUT
        .iter()
        .filter(|layer| layer.catches(0))
        .map(Layer::severity)
        .sum()
}

pub fn part_2() -> usize {
    (1usize..)
        .find(|&delay| !INPUT.iter().any(|layer| layer.catches(delay)))
        .unwrap()
}

#[derive(Debug)]
pub struct Layer {
    pub depth: usize,
    pub range: usize,
}

impl Layer {
    pub fn catches(&self, delay: usize) -> bool {
        (self.depth + delay) % ((self.range - 1) * 2) == 0
    }

    pub fn severity(&self) -> usize {
        self.depth * self.range
    }
}
