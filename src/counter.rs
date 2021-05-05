use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct Counter<K: Eq + Hash>(HashMap<K, usize>);

impl<K: Eq + Hash> FromIterator<K> for Counter<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut h = HashMap::new();

        for item in iter {
            h.entry(item).and_modify(|e| *e += 1).or_insert(1);
        }

        Counter(h)
    }
}

impl<K: Eq + Hash> IntoIterator for Counter<K> {
    type Item = (K, usize);
    type IntoIter = std::collections::hash_map::IntoIter<K, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
