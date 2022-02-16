use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::combinatorics::{Combination, Permutation};

pub fn dictionary() -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    // Read our word list.
    let mut words = HashSet::new();
    let file = File::open("words.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        words.insert(line?);
    }
    Ok(words)
}

pub fn words(words: &HashSet<String>, letters: &str, min: usize) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut results = vec![];
    let letters = letters.chars().collect::<Vec<char>>();

    for x in min..letters.len() + 1 {
        for c in Combination::new(letters.len(), x) {
            for p in Permutation::new(c.len()) {
                let mut s = String::new();
                for i in p.iter() {
                    s.push(letters[c[*i]]);
                }

                if words.contains(&s) && !seen.contains(&s) {
                    seen.insert(s.clone());
                    results.push(s);
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::words::*;
    use std::collections::HashSet;

    macro_rules! assert_eq_sorted {
        ($l:expr, $r:expr) => {
            let l = $l.sort();
            let r = $r.sort();
            assert_eq!(l, r);
        };
    }

    #[test]
    fn test_words() {
        let mut set = HashSet::new();
        set.insert("cat".to_string());
        set.insert("at".to_string());
        assert_eq_sorted!(words(&set, "cat", 2), ["cat".to_string(), "at".to_string()]);
    }
}
