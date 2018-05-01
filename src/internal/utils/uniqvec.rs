use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

use smallvec::SmallVec;

pub struct UniqVec<V, S = RandomState> {
    values: Vec<V>,
    lookup: HashMap<u64, SmallVec<[usize; 4]>>,
    state: S
}

impl<V: Eq + Hash> UniqVec<V, RandomState> {
    pub fn new() -> UniqVec<V, RandomState> {
        UniqVec {
            values: Vec::new(),
            lookup: HashMap::new(),
            state: RandomState::new()
        }
    }
}

impl<V: Eq + Hash, S: BuildHasher> UniqVec<V, S> {
    pub fn push(&mut self, v: V) -> (usize, bool) {
        use std::collections::hash_map::Entry;

        let idx = self.values.len();

        let mut hasher = self.state.build_hasher();
        v.hash(&mut hasher);
        let hash = hasher.finish();

        match self.lookup.entry(hash) {
            Entry::Occupied(mut entry) => {
                let idxs = entry.into_mut();
                for idx in idxs.iter() {
                    if self.values[*idx] == v {
                        return (*idx, false);
                    }
                }
                let idx = self.values.len();
                self.values.push(v);
                idxs.push(idx);
                return (idx, true);
            },
            Entry::Vacant(entry) => {
                let idx = self.values.len();
                self.values.push(v);
                entry.insert(SmallVec::new()).push(idx);
                return (idx, true);
            }
        }
    }

    pub fn get(&self, idx: usize) -> Option<&V> {
        self.values.get(idx)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<T> AsRef<[T]> for UniqVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.values.as_ref()
    }
}
