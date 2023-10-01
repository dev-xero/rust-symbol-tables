pub struct Entry<K, V> {
    pub key: K,
    pub value: V
}

pub struct SymbolTable<K, V> {
    pub contents: Vec<Entry<K, V>>
}

impl<K: PartialEq + Clone, V: Clone> SymbolTable<K, V> {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }

    pub fn from(list: &[(K, V)]) -> Self {
        let mut sym_table = Self::new();
        for (key, value) in list.to_vec() {
            sym_table.insert(key, value);
        }

        sym_table
    }

    pub fn is_empty(&self) -> bool {
        self.contents.len() == 0
    }

    pub fn size(&self) -> usize {
        self.contents.len()
    }

    pub fn contains(&self, key: &K) -> (bool, usize) {
        let mut idx = 0;
        for sym_item in &self.contents {
            if &sym_item.key == key {
                return (true, idx)
            }
            idx += 1;
        }

        (false, idx)
    }

    pub fn insert(&mut self, key: K, value: V) -> &mut Self {
        let (contains_key, idx) = self.contains(&key);
        if contains_key {
            self.contents.remove(idx);
            self.contents.insert(idx, Entry { key, value});
        } else {
            self.contents.push(Entry { key, value });
        }

        return self;
    }

    pub fn get(&mut self, key: &K) -> Option<&Entry<K, V>> {
        for sym_item in &self.contents {
            if &sym_item.key == key {
                return Some(sym_item)
            }
        }

        None
    }
}