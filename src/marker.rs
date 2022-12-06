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

    pub fn marker_indices(&self, marker_len: usize) -> Vec<usize> {
        let mut markers = Vec::new();
        for (idx, w) in self.content.windows(marker_len).enumerate() {
            let unique: HashSet<_> = w.iter().collect();

            if unique.len() == marker_len {
                markers.push(idx + marker_len);
            }
        }

        markers
    }
}
