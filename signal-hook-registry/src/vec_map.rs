use std::mem;

/// A small map backed by a sorted vector.
#[derive(Clone, Default)]
pub struct VecMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K: Ord, V> VecMap<K, V> {
    pub fn new() -> Self {
        VecMap {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
    }

    pub fn contains(&self, key: &K) -> bool {
        self.keys.binary_search(key).is_ok()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.keys.binary_search(key) {
            Ok(i) => Some(&self.values[i]),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.keys.binary_search(key) {
            Ok(i) => Some(&mut self.values[i]),
            Err(_) => None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.keys.binary_search(&key) {
            Ok(i) => {
                let old = mem::replace(&mut self.values[i], value);
                Some(old)
            }
            Err(i) => {
                self.keys.insert(i, key);
                self.values.insert(i, value);
                None
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.keys.binary_search(key) {
            Ok(i) => {
                self.keys.remove(i);
                Some(self.values.remove(i))
            }
            Err(_) => None,
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.values.iter()
    }
}
