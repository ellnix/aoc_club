use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Copy, Clone)]
pub struct Loop<T> {
    pub start: usize,
    pub end_exclusive: usize,
    pub duplicate: T,
}

impl<T> Loop<T> {
    pub fn new(start: usize, end_exclusive: usize, duplicate: T) -> Self {
        Self { start, end_exclusive, duplicate }
    }

    pub fn len(&self) -> usize {
        self.end_exclusive - self.start
    }
}

impl<T> Loop<T>
where
    T: Eq + Hash,
{
    pub fn find<I>(seq: I) -> Option<Self>
    where
        I: Iterator<Item = T>,
    {
        let mut seen = HashMap::new();

        for (i, v) in seq.enumerate() {
            match seen.get(&v) {
                Some(&start) => return Some(Self::new(start, i, v)),
                None => {
                    seen.insert(v, i);
                },
            }
        }

        None
    }
}

impl<T> PartialEq for Loop<T> {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end_exclusive == other.end_exclusive
    }
}

impl<T> Eq for Loop<T> {}
