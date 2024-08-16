use std::cmp::min;
use std::iter::FusedIterator;
use std::vec;

use itertools::Itertools;

pub trait LoopingItertools<T> {
    fn looping(self, size: usize) -> Looping<T>;
}

impl<T, I> LoopingItertools<T> for I
where
    T: Eq + Clone,
    I: Iterator<Item = T>,
{
    fn looping(self, size: usize) -> Looping<T> {
        let mut prefix = Vec::new();

        for e in self {
            match prefix.iter().find_position(|&ve| *ve == e) {
                Some((start, _)) => {
                    let cycle = prefix.split_off(start);
                    return Looping::new(prefix, cycle, size);
                },
                None => prefix.push(e),
            }
        }

        panic!("no loop detected");
    }
}

#[derive(Debug, Clone)]
pub struct Looping<T> {
    prefix: vec::IntoIter<T>,
    cycle: Vec<T>,
    cycle_pos: usize,
    cycle_size: usize,
}

impl<T> Looping<T> {
    pub fn new(prefix: Vec<T>, cycle: Vec<T>, size: usize) -> Self {
        let prefix_len = prefix.len();
        Self {
            prefix: prefix.into_iter(),
            cycle,
            cycle_pos: 0,
            cycle_size: size.saturating_sub(prefix_len),
        }
    }

    fn cycle_len(&self) -> usize {
        self.cycle_size - self.cycle_pos
    }
}

impl<T> Iterator for Looping<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.prefix.next() {
            Some(e) => Some(e),
            None if self.cycle_len() == 0 => None,
            None => {
                let e = self.cycle[self.cycle_pos % self.cycle.len()].clone();
                self.cycle_pos += 1;
                Some(e)
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.prefix.len() + self.cycle_len();
        (exact, Some(exact))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.len() == 0 {
            return None;
        }

        let last_pos_in_cycle = (self.cycle_size - 1) % self.cycle.len();
        self.cycle
            .into_iter()
            .nth(last_pos_in_cycle)
            .or_else(|| self.prefix.last())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let prefix_len = self.prefix.len();
        self.prefix.nth(n).or_else(|| {
            self.cycle_pos = min(self.cycle_pos + (n - prefix_len), self.cycle_size);
            (self.cycle_len() != 0)
                .then(|| self.next())
                .unwrap_or_default()
        })
    }
}

impl<T> ExactSizeIterator for Looping<T> where T: Clone {}
impl<T> FusedIterator for Looping<T> where T: Clone {}

#[cfg(test)]
#[cfg(feature = "utils")]
mod tests {
    use super::*;

    const DATA: &[usize] = &[1, 2, 3, 4, 5, 6, 3];

    #[test]
    fn test_iterator() {
        let v = DATA.iter().looping(11).copied().collect::<Vec<_>>();
        assert_eq!([1, 2, 3, 4, 5, 6, 3, 4, 5, 6, 3], *v.as_slice());
    }

    #[test]
    fn test_exact_size() {
        let mut i = DATA.iter().looping(11);
        assert_eq!(11, i.len());
        assert_eq!((11, Some(11)), i.size_hint());

        let _ = i.next();
        assert_eq!(10, i.len());
        assert_eq!((10, Some(10)), i.size_hint());

        let _ = i.next();
        let _ = i.next();
        assert_eq!(8, i.len());
        assert_eq!((8, Some(8)), i.size_hint());

        while i.next().is_some() {}
        assert_eq!(0, i.len());
        assert_eq!((0, Some(0)), i.size_hint());
    }

    #[test]
    fn test_count() {
        let i = DATA.iter().looping(11);
        assert_eq!(11, i.count());

        let mut i = DATA.iter().looping(11);
        let _ = i.next();
        let _ = i.next();
        let _ = i.next();
        assert_eq!(8, i.count());
    }

    #[test]
    fn test_last() {
        let i = DATA.iter().copied().looping(11);
        assert_eq!(Some(3), i.last());

        let mut i = DATA.iter().copied().looping(11);
        let _ = i.next();
        let _ = i.next();
        let _ = i.next();
        assert_eq!(Some(3), i.last());

        let mut i = DATA.iter().copied().looping(11);
        while i.next().is_some() {}
        assert_eq!(None, i.last());
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn test_nth() {
        let expected = [1, 2, 3, 4, 5, 6, 3, 4, 5, 6, 3];

        let mut i = DATA.iter().copied().looping(11);
        let mut ei = expected.iter().copied();
        while let Some(e) = i.nth(0) {
            println!("{}", e);
            assert_eq!(ei.next(), Some(e));
        }
        assert!(ei.next().is_none());

        let mut i = DATA.iter().copied().looping(11);
        assert_eq!(Some(2), i.nth(1));
        assert_eq!(Some(4), i.nth(1));
        assert_eq!(Some(3), i.nth(2));
        assert!(i.nth(7).is_none());
        assert!(i.next().is_none());

        let mut i = DATA.iter().copied().looping(11);
        assert_eq!(Some(3), i.nth(10));
        assert!(i.next().is_none());
    }
}
