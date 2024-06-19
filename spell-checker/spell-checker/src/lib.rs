///////////////////////////////////////////////////////////////////////////////

use std::{borrow::Borrow, collections::BinaryHeap, fs, path::PathBuf};

use anyhow::Result;

use cs_240_library::data_structures::hashset::HashSet;

///////////////////////////////////////////////////////////////////////////////

/*

Video notes

- explain data structure
    - hash set
        - O(1) lookup
        - O(1) add
    - binary heap in suggest function
- each function in here
- each function in main.rs
    - the match statements
- demo cli
- demo test

*/

///////////////////////////////////////////////////////////////////////////////

/// Ternary operator
///
/// Returns a if c is true, otherwise b
#[inline]
fn t<T>(c: bool, a: T, b: T) -> T {
    if c {
        a
    } else {
        b
    }
}

/// 3 way min
#[inline]
fn min<T>(a: T, b: T, c: T) -> T
where
    T: Ord,
{
    a.min(b).min(c)
}

///////////////////////////////////////////////////////////////////////////////

/// Edit distance between two words
fn levenshtein_distance(first: &str, second: &str) -> usize {
    // iterative single row based on [30], [29], [27], and [31]

    // just changing the types to be easier to work with
    let first: Vec<char> = first.chars().collect();
    let second: Vec<char> = second.chars().collect();

    // create the row [0..|s|+1]
    let mut row: Vec<usize> = (0..second.len() + 1).into_iter().collect();

    // variables to track what would have been previous rows
    let mut previous_diagonal;
    let mut previous_above = 0;

    // for each character in `first`
    for i in 0..first.len() {
        row[0] = i + 1;

        // for each character in `second`
        for j in 0..second.len() {
            // check if the characters are equal
            let indicator = t(first[i] != second[j], 1, 0);
            // update tracking variables
            previous_diagonal = previous_above;
            previous_above = row[j + 1];
            // update current row entry with 3 way minimum from deleting,
            // inserting and substituting
            row[j + 1] = min(
                previous_above + 1,            // deleting
                row[j] + 1,                    // inserting
                previous_diagonal + indicator, // substituting
            );
        }
    }

    // return the last entry in the row
    row[second.len()]
}
///////////////////////////////////////////////////////////////////////////////

/// Struct for priority queue entries
#[derive(Debug, Eq, PartialOrd)]
struct RankedWord {
    ranking: usize,
    word: String,
}

//---------------------------------------------------------------------------//

impl PartialEq for RankedWord {
    fn eq(&self, other: &Self) -> bool {
        self.ranking == other.ranking && self.word == other.word
    }
}

impl Ord for RankedWord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ranking.cmp(&other.ranking)
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct SpellChecker {
    // HashSet with String entries, implementation found in CodeLibrary with
    // separate chaining.
    dictionary: HashSet<String>,
}

//---------------------------------------------------------------------------//

impl SpellChecker {
    /// Returns a spell checker with no known words
    pub fn new() -> SpellChecker {
        // capacity picked mostly at random
        SpellChecker {
            dictionary: HashSet::new().with_capacity(50000),
        }
    }

    //.......................................................................//

    /// Returns true if the word is in the dictionary, otherwise false
    pub fn check(&self, word: &str) -> bool {
        // just check if the HashSet contains the normalized (lower cased) word
        self.dictionary.contains(word.to_lowercase())
    }

    /// Returns a sorted list of similar words
    pub fn suggest(&self, word: &str) -> Vec<String> {
        // create a new binary heap to keep track of closest matches
        // we're checking every word in the dictionary so we'll initialize the
        // heap with the number of words as capacity
        let mut suggestions = BinaryHeap::with_capacity(self.dictionary.len());

        // check each word in our dictionary
        for other in &self.dictionary.items() {
            // add each in sorted order to our binary heap based on it's edit
            // to the provided word
            suggestions.push(RankedWord {
                ranking: levenshtein_distance(word, &other),
                word: other.to_string(),
            })
        }

        // convert to a sorted vector to maintain ordering
        let suggestions = suggestions.into_sorted_vec();

        // we need to iterate over each entry and just return the word,
        // without the ranking
        suggestions.iter().map(|c| c.word.to_owned()).collect()
    }

    //.......................................................................//

    /// Adds the given word to the dictionary
    pub fn add_word(&mut self, word: &str) {
        // just add the normalized word to the hash set
        self.dictionary.insert(word.to_lowercase());
    }

    /// Returns all known words
    pub fn words(&self) -> Vec<String> {
        self.dictionary.borrow().items()
    }

    /// Returns all known words in sorted order
    pub fn sorted_words(&self) -> Vec<String> {
        let mut words = self.words();
        words.sort();
        words
    }

    //.......................................................................//

    /// Returns a formatted string of all known words in sorted order
    pub fn to_string(&self) -> String {
        self.sorted_words()
            .into_iter()
            .fold("".to_owned(), |acc, elem| acc + &elem + "\n")
    }

    //.......................................................................//

    /// Returns a spell checker with words from the provided dictionary file
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let mut context = Self::new();

        // quick way to get the contents of a file as a string
        let contents = fs::read_to_string(path)?;

        // my dictionary format seems to have a word per line and some words
        // have /... chunks for conjugation.
        // we'll just ignore conjugation here.
        for line in contents.lines() {
            let word = line.split("/").next().unwrap_or_default();
            context.add_word(&word);
        }

        Ok(context)
    }

    /// Dumps all known words to the given file.
    /// Any existing file contents are overwritten.
    pub fn to_file(&self, path: &str) -> Result<()> {
        // quick way to write a string to a given file
        Ok(fs::write(path, self.to_string())?)
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn levenshtein_distance_tests() {
        assert_eq!(levenshtein_distance("INTENTION", "EXECUTION"), 5);
    }

    #[test]
    fn in_memory_tests() {
        let mut spell_checker = SpellChecker::new();

        assert!(!spell_checker.check("test"));
        assert_eq!(spell_checker.words(), Vec::<String>::new());
        assert_eq!(spell_checker.to_string(), "");

        spell_checker.add_word("a");

        assert!(!spell_checker.check("test"));
        assert!(spell_checker.check("a"));
        assert_eq!(spell_checker.words(), vec!["a"]);
        assert_eq!(spell_checker.to_string(), "a\n");

        spell_checker.add_word("test");

        assert!(spell_checker.check("test"));
        assert!(spell_checker.check("a"));
        assert_eq!(spell_checker.sorted_words(), vec!["a", "test"]);
        assert_eq!(spell_checker.to_string(), "a\ntest\n");

        assert_eq!(spell_checker.suggest("tes"), vec!["test", "a"]);

        spell_checker.add_word("the");
        spell_checker.add_word("was");
        spell_checker.add_word("it");
        spell_checker.add_word("programming");
        spell_checker.add_word("cat");

        assert!(spell_checker.check("test"));
        assert!(spell_checker.check("a"));
        assert_eq!(
            spell_checker.sorted_words(),
            vec!["a", "cat", "it", "programming", "test", "the", "was"]
        );
        assert_eq!(
            spell_checker.to_string(),
            "a\ncat\nit\nprogramming\ntest\nthe\nwas\n"
        );
    }
}

///////////////////////////////////////////////////////////////////////////////
