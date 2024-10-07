use std::{collections::HashMap, hash::Hash, marker::PhantomData};

pub trait DenseInternedInfoMapKey
where
    Self: Sized,
{
    fn from(index: usize) -> Self;

    fn index(&self) -> usize;

    fn copy(&self) -> Self {
        Self::from(self.index())
    }
}

struct Entry<V, I> {
    value: V,
    info: Option<I>,
}

pub struct DenseInternedInfoMap<K: DenseInternedInfoMapKey, V: Eq + Hash + Clone, I> {
    store: Vec<Entry<V, I>>,
    unique: HashMap<V, K>,
    generic: PhantomData<K>,
}

impl<K: DenseInternedInfoMapKey, V: Eq + Hash + Clone, I> Default
    for DenseInternedInfoMap<K, V, I>
{
    fn default() -> Self {
        Self {
            store: Vec::default(),
            unique: HashMap::default(),
            generic: PhantomData,
        }
    }
}

impl<K: DenseInternedInfoMapKey, V: Eq + Hash + Clone, I> DenseInternedInfoMap<K, V, I> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, value: V) -> K {
        self.add_entry(Entry { value, info: None })
    }

    pub fn add_with_info(&mut self, value: V, info: I) -> K {
        self.add_entry(Entry {
            value,
            info: Some(info),
        })
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.contains_key(key) {
            Some(&self.store[key.index()].value)
        } else {
            None
        }
    }

    pub fn info(&mut self, key: &K) -> Option<&I> {
        if self.contains_key(key) {
            self.store[key.index()].info.as_ref()
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        key.index() < self.store.len()
    }

    pub fn contains(&self, value: &V) -> bool {
        self.unique.contains_key(value)
    }

    fn add_entry(&mut self, entry: Entry<V, I>) -> K {
        if let Some(key) = self.unique.get(&entry.value) {
            key.copy()
        } else {
            let index = self.store.len();
            let key = K::from(index);
            self.unique.insert(entry.value.clone(), key.copy());
            self.store.push(entry);
            key
        }
    }
}
