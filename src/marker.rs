use std::collections::HashSet;

pub struct Marker {
    content: Vec<char>,
}

impl Marker {
    pub fn new(s: String) -> Self {
        Self {
            content: s.chars().collect(),
        }
    }

    pub fn first_marker_index(&self, marker_len: usize) -> Option<usize> {
        for (idx, w) in self.content.windows(marker_len).enumerate() {
            let unique: HashSet<_> = w.iter().collect();

            if unique.len() == marker_len {
                return Some(idx + marker_len);
            }
        }

        None
    }
}
