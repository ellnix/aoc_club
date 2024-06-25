use crate::input::day_9::INPUT; // &str

pub fn part_1() -> usize {
    Group::root().total_score()
}

pub fn part_2() -> usize {
    Group::root().total_garbage()
}

#[derive(Debug, Default)]
struct Group {
    parent_score: usize,
    children: Vec<Group>,
    garbage: usize,
}

impl Group {
    fn new(parent_score: usize) -> Self {
        Self { parent_score, ..Self::default() }
    }

    fn root() -> Self {
        let mut chars = INPUT.chars();

        match chars.next() {
            Some('{') => (),
            Some(c) => panic!("invalid starting character in stream: {}", c),
            None => panic!("empty stream"),
        }

        let root = Self::parse(&mut chars, 0);

        if let Some(c) = chars.next() {
            panic!("invalid character after outermost group in stream: {}", c);
        }

        root
    }

    fn score(&self) -> usize {
        self.parent_score + 1
    }

    fn total_score(&self) -> usize {
        self.score() + self.children.iter().map(Self::total_score).sum::<usize>()
    }

    fn garbage(&self) -> usize {
        self.garbage
    }

    fn total_garbage(&self) -> usize {
        self.garbage() + self.children.iter().map(Self::total_garbage).sum::<usize>()
    }

    fn parse<I>(chars: &mut I, parent_score: usize) -> Self
    where
        I: Iterator<Item = char>,
    {
        let mut group = Self::new(parent_score);

        while let Some(c) = chars.next() {
            match c {
                '<' => group.garbage += Self::skip_garbage(chars),
                '{' => group.children.push(Self::parse(chars, group.score())),
                '}' => return group,
                ',' => (),
                _ => panic!("invalid non-garbage character found in stream: {}", c),
            }
        }

        panic!("end of stream reached without group being closed");
    }

    fn skip_garbage<I>(chars: &mut I) -> usize
    where
        I: Iterator<Item = char>,
    {
        let mut size = 0;

        while let Some(c) = chars.next() {
            match c {
                '!' => {
                    chars.next();
                },
                '>' => return size,
                _ => size += 1,
            }
        }

        panic!("end of stream reached without garbage being closed");
    }
}
