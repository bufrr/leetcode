use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current.children.entry(c).or_insert(TrieNode::new());
        }
        current.is_end_of_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }
        current.is_end_of_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;
        for c in prefix.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }
        true
    }
}
