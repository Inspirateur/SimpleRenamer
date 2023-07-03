use std::collections::HashMap;

pub fn text_histo(text: &str) -> HashMap<char, usize> {
    let mut res = HashMap::new();
    for c in text.chars() {
        *res.entry(c).or_default() += 1;
    }
    res
}

pub trait TextHisto {
    fn count(&self, c: &char) -> usize;

    fn dist(&self, other: &Self) -> usize;
}

impl TextHisto for HashMap<char, usize> {
    fn count(&self, c: &char) -> usize {
        *self.get(c).unwrap_or(&0)
    }

    fn dist(&self, other: &Self) -> usize {
        let mut res: usize = 0;
        for (c, count) in other.iter() {
            res += self.count(c).abs_diff(*count);
        }
        // also count up char in self that are not in other
        for (c, count) in self.iter() {
            if !other.contains_key(c) {
                res += count;
            }
        }
        res
    }
}