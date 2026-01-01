use std::collections::HashMap;

pub struct StringInterner {
    strings: Vec<String>,
    string_to_id: HashMap<String, usize>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            string_to_id: HashMap::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(&id) = self.string_to_id.get(s) {
            id
        } else {
            let id = self.strings.len();
            self.strings.push(s.to_string());
            self.string_to_id.insert(s.to_string(), id);
            id
        }
    }

    pub fn resolve(&self, id: usize) -> Option<&str> {
        self.strings.get(id).map(|s| s.as_str())
    }
}