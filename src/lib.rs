#![deny(rust_2018_idioms)]
use crate::oom::OneOrMany;
use std::borrow::Borrow;
use std::collections::hash_map::{IntoIter, Iter};
use std::collections::HashMap;
use std::hash::Hash;

pub mod oom;

pub struct Hasheimer<K, V>(pub HashMap<K, OneOrMany<V>>)
where
    K: PartialEq + Eq + Hash;

impl<K, V> Hasheimer<K, V>
where
    K: PartialEq + Eq + Hash,
{
    #[must_use]
    pub fn new() -> Self {
        Hasheimer(HashMap::new())
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&OneOrMany<V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.0.get(k)
    }

    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut OneOrMany<V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.0.get_mut(k)
    }

    pub fn get_if_single<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if let Some(OneOrMany::Single(v)) = self.get(k) {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_if_multiple<Q>(&self, k: &Q) -> Option<&Vec<V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        if let Some(OneOrMany::Many(v)) = self.get(k) {
            Some(v)
        } else {
            None
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<OneOrMany<V>> {
        let replaced = self.remove(&k);
        if let Some(rem) = replaced {
            match rem {
                OneOrMany::Single(ele) => self.raw_insert(k, OneOrMany::Many(vec![ele, v])),
                OneOrMany::Many(mut vec) => {
                    vec.push(v);
                    todo!()
                }
            }
        } else {
            None
        }
    }

    pub fn raw_insert<Q>(&mut self, k: K, v: OneOrMany<V>) -> Option<OneOrMany<V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.0.insert(k, v)
    }

    pub fn remove(&mut self, k: &K) -> Option<OneOrMany<V>> {
        self.0.remove(k)
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, K, OneOrMany<V>> {
        self.0.iter()
    }
}

impl<K, V> Default for Hasheimer<K, V>
where
    K: PartialEq + Eq + Hash,
{
    fn default() -> Self {
        Hasheimer::new()
    }
}

impl<K, V> IntoIterator for Hasheimer<K, V>
where
    K: PartialEq + Eq + Hash,
{
    type Item = (K, OneOrMany<V>);
    type IntoIter = IntoIter<K, OneOrMany<V>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K, V> Extend<(K, OneOrMany<V>)> for Hasheimer<K, V>
where
    K: PartialEq + Eq + Hash,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, OneOrMany<V>)>,
    {
        self.0.extend(iter);
    }
}
