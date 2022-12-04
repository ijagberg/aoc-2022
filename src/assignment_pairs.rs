pub struct AssignmentPair {
    a: (usize, usize),
    b: (usize, usize),
}

impl AssignmentPair {
    pub fn new(a: (usize, usize), b: (usize, usize)) -> Self {
        if (a.0 > a.1) || (b.0 > b.1) {
            panic!()
        }
        Self { a, b }
    }

    pub fn overlaps(&self) -> bool {
        let (a, b) = (self.a, self.b);
        if (a.0 <= b.0) && (a.1 >= b.1) {
            true
        } else if (b.0 <= a.0) && (b.1 >= a.1) {
            true
        } else {
            false
        }
    }

    pub fn partially_overlaps(&self) -> bool {
        let (a, b) = (self.a, self.b);
        if ((a.0 < b.0) && (a.1 >= b.0)) || ((b.0 < a.0) && (b.1 >= a.0)) {
            true
        } else {
            self.overlaps()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlaps_test() {
        let pair = AssignmentPair::new((2, 8), (3, 7));

        assert!(pair.overlaps());
    }

    #[test]
    fn partially_overlaps_test() {
        let pair = AssignmentPair::new((2, 4), (6, 8));
        assert!(!pair.partially_overlaps());

        let pair = AssignmentPair::new((2, 3), (4, 5));
        assert!(!pair.partially_overlaps());

        let pair = AssignmentPair::new((5, 7), (7, 9));
        assert!(pair.partially_overlaps());

        let pair = AssignmentPair::new((2, 8), (3, 7));
        assert!(pair.partially_overlaps());

        let pair = AssignmentPair::new((6, 6), (4, 6));
        assert!(pair.partially_overlaps());

        let pair = AssignmentPair::new((2, 6), (4, 8));
        assert!(pair.partially_overlaps());
    }
}
