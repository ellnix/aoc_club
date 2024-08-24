use crate::input::day_17::INPUT;

pub fn part_1() -> usize {
    let mut spinlock = Spinlock::default();
    spinlock.spin_a_lot(2017);
    spinlock.after_last_value_written()
}

pub fn part_2() -> usize {
    let mut spinlock = GoodEnoughSpinlock::default();
    spinlock.spin_a_lot(50_000_000);
    spinlock.after_0()
}

#[derive(Debug)]
struct Spinlock {
    buffer: Vec<usize>,
    current_pos: usize,
    next_value: usize,
}

impl Spinlock {
    pub fn spin(&mut self) {
        self.current_pos = (self.current_pos + INPUT) % self.buffer.len() + 1;
        self.buffer.insert(self.current_pos, self.next_value);
        self.next_value += 1;
    }

    pub fn spin_a_lot(&mut self, times: usize) {
        for _ in 0..times {
            self.spin();
        }
    }

    pub fn after_last_value_written(&self) -> usize {
        self.buffer[(self.current_pos + 1) % self.buffer.len()]
    }
}

impl Default for Spinlock {
    fn default() -> Self {
        Self { buffer: vec![0], current_pos: 0, next_value: 1 }
    }
}

#[derive(Debug)]
struct GoodEnoughSpinlock {
    after_0: Option<usize>,
    size: usize,
    current_pos: usize,
    next_value: usize,
}

impl GoodEnoughSpinlock {
    pub fn spin(&mut self) {
        self.current_pos = (self.current_pos + INPUT) % self.size + 1;
        self.size += 1;
        if self.current_pos == 1 {
            self.after_0 = Some(self.next_value);
        }
        self.next_value += 1;
    }

    pub fn spin_a_lot(&mut self, times: usize) {
        for _ in 0..times {
            self.spin();
        }
    }

    pub fn after_0(&self) -> usize {
        self.after_0.expect("spin first")
    }
}

impl Default for GoodEnoughSpinlock {
    fn default() -> Self {
        Self { after_0: None, size: 1, current_pos: 0, next_value: 1 }
    }
}
