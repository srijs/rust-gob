use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

use smallvec::SmallVec;

pub struct UniqMap<K, V, S = RandomState> {
    values: HashMap<K, V, S>,
    lookup: HashMap<u64, SmallVec<[K; 4]>>,
}

impl<K: Eq + Hash + Clone, V: Eq + Hash> UniqMap<K, V, RandomState> {
    pub fn new() -> UniqMap<K, V, RandomState> {
        UniqMap {
            values: HashMap::new(),
            lookup: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash + Clone, V: Eq + Hash, S: BuildHasher> UniqMap<K, V, S> {
    pub fn insert(&mut self, k: K, v: V) -> Option<K> {
        use std::collections::hash_map::Entry;

        let mut hasher = self.values.hasher().build_hasher();
        v.hash(&mut hasher);
        let hash = hasher.finish();

        match self.lookup.entry(hash) {
            Entry::Occupied(mut entry) => {
                for key in entry.get() {
                    if self.values[key] == v {
                        return Some(key.clone());
                    }
                }
                entry.get_mut().push(k.clone());
                self.values.insert(k, v);
                return None;
            }
            Entry::Vacant(entry) => {
                entry.insert(SmallVec::new()).push(k.clone());
                self.values.insert(k, v);
                return None;
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.values.get(key)
    }
}
