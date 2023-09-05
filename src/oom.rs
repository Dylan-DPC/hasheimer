use core::ops::{Index, IndexMut};

pub enum OneOrMany<V> {
    Single(V),
    Many(Vec<V>),
}

impl<V> Index<usize> for OneOrMany<V> {
    type Output = V;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            OneOrMany::Single(v) if index == 0 => v,
            OneOrMany::Single(_) => {
                panic!("this has only a single value")
            }
            OneOrMany::Many(v) => v.index(0),
        }
    }
}

impl<V> IndexMut<usize> for OneOrMany<V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            OneOrMany::Single(v) if index == 0 => v,
            OneOrMany::Single(_) => {
                panic!("this has only a single value")
            }
            OneOrMany::Many(v) => v.index_mut(0),
        }
    }
}
