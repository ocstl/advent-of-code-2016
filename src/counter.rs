use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Clone, Debug, Default)]
pub struct Counter<K: Eq + Hash>(HashMap<K, usize>);

impl<K: Eq + Hash> Counter<K> {
    /// Create a new Counter.
    pub fn new() -> Self {
        Counter(HashMap::new())
    }

    /// Increment the counter for `element`.
    pub fn add(&mut self, element: K) {
        self.0.entry(element).and_modify(|e| *e += 1).or_insert(1);
    }

    /// Return a reference to the most common element. In case of a tie of two
    /// (or more) elements, an arbitrary element is returned.
    ///
    /// Returns `None` if the counter is empty.
    pub fn most_common(&self) -> Option<&K> {
        self.0.iter().max_by_key(|p| p.1).map(|(k, _)| k)
    }

    /// Return a reference to the least common element. In case of a tie of two
    /// (or more) elements, an arbitrary element is returned.
    ///
    /// Returns `None` if the counter is empty.
    pub fn least_common(&self) -> Option<&K> {
        self.0.iter().min_by_key(|p| p.1).map(|(k, _)| k)
    }
}

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
