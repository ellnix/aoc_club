use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loop<T> {
    pub start: usize,
    pub elements: Vec<T>,
}

impl<T> Loop<T> {
    pub fn new(start: usize, elements: Vec<T>) -> Self {
        Self { start, elements }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn last_from_total(&self, total_count: usize) -> &T {
        &self.elements[total_count % self.len()]
    }
}

impl<T> Loop<T>
where
    T: Eq,
{
    pub fn find<I>(seq: I) -> Option<Self>
    where
        I: Iterator<Item = T>,
    {
        let mut elements = Vec::new();

        for v in seq {
            match elements.iter().find_position(|&lv| *lv == v) {
                Some((start, _)) => return Some(Self::new(start, elements)),
                None => elements.push(v),
            }
        }

        None
    }
}
