use std::collections::hash_map::RandomState;
use std::collections::BTreeMap;
use std::hash::{BuildHasher, Hash, Hasher};

use smallvec::SmallVec;

pub struct UniqMap<K, V> {
    values: BTreeMap<K, V>,
    lookup: BTreeMap<u64, SmallVec<[K; 4]>>,
    state: RandomState,
}

impl<K: Ord + Clone, V: Eq + Hash> UniqMap<K, V> {
    pub fn new() -> UniqMap<K, V> {
        UniqMap {
            values: BTreeMap::new(),
            lookup: BTreeMap::new(),
            state: RandomState::new(),
        }
    }
}

impl<K: Ord + Clone, V: Eq + Hash> UniqMap<K, V> {
    pub fn insert(&mut self, k: K, v: V) -> Option<K> {
        use std::collections::btree_map::Entry;

        let mut hasher = self.state.build_hasher();
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
