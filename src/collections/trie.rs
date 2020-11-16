use std::collections::HashMap;
use std::hash::Hash;
pub struct Trie<K: Copy + Eq + Hash> {
    is_leaf: bool,
    nodes: HashMap<K, Trie<K>>,
}

impl<K: Copy + Eq + Hash> Trie<K> {
    pub fn new() -> Trie<K> {
        Trie {
            is_leaf: false,
            nodes: HashMap::new(),
        }
    }
    pub fn add_value(&mut self, value: &[K]) {
        let mut current_tree = self;
        for c in value {
            current_tree = current_tree.nodes.entry(*c).or_insert(Trie {
                nodes: HashMap::new(),
                is_leaf: false,
            });
        }
        current_tree.is_leaf = true;
    }
    pub fn get_value(&self, key: &[K]) -> Option<&Trie<K>> {
        let mut current_tree = self;
        for b in key {
            match current_tree.nodes.get(&b) {
                Some(x) => {
                    current_tree = x;
                }
                None => return None,
            }
        }
        Some(current_tree)
    }
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_trie() {
        let test_values: Vec<&[u8]> =
            vec![b"test", b"test2", b"testable", b"cart", b"cartel", b"carp"];
        let mut trie: Trie<u8> = Trie::new();
        for test in test_values {
            trie.add_value(test);
        }
        let test_cases: Vec<(&[u8], usize, bool)> = vec![
            (b"test", 2, true),
            (b"car", 2, false),
            (b"cart", 1, true),
            (b"carte", 1, false),
        ];
        for case in test_cases {
            let result = trie.get_value(case.0).unwrap();
            assert_eq!(result.nodes.len(), case.1);
            assert_eq!(result.is_leaf(), case.2);
        }
    }

    #[test]
    fn test_no_value() {
        let test_values: Vec<&[u8]> =
            vec![b"test", b"test2", b"testable", b"cart", b"cartel", b"carp"];
        let mut trie: Trie<u8> = Trie::new();
        for test in test_values {
            trie.add_value(test);
        }
        match trie.get_value(b"cartels") {
            Some(_) => assert!(false, "Trie shouldn't return a value"),
            _ => {}
        }
    }
}
