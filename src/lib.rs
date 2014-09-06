use std::collections::TreeMap;

// TODO: set up cargo documentation

// TODO: Better brace / completion handling in emacs (paredit? yasnippet?)
// TODO: Integration tests!

// XXX: How do I make this type signature not make promises about map
// implementation?
//
// I guess I could make it a method in a trait, and then implement it
// specifically for TreeMap, TrieMap, etc. But that's duplicating
// implementation.
//
// I could also have it take the mapping as a mutable out-parameter. But
// blergh.
//
// Also, different map structures have different constraints on key type.
// e.g. K: Ord for TreeMap, K: Hash for HashMap [XXX: does rust have this?]

// XXX: I guess it's OK for the iterator to be mutable, since what else is
// .next() going to do? Is there a way to express this with immutable
// parameters?

// XXX: Cloning all over the shop here. Is there a more sensible way to do this?

// XXX: Untested (extracted from frequency)

/// `accumulate` takes elements of a sequence and folds them into a map.
///
/// # Arguments
///
/// * `sequence` - A sequence of things that will become keys in the map.
/// * `init` - The value stored for all keys when they are first inserted in
///    the map.
/// * `f` - A fold function that is applied to the value every time we see that
///    key again.
///
/// # Returns
///
/// A map of elements in the sequences to results of the fold operation.
///
/// # Example
///
/// To count the number of times each element appears in a vector:
///
/// ```rust
/// let some_vec = vec![1i, 1i, 2i, 1i, 1i, 2i, 3i];
/// let counts = target::accumulate(&mut some_vec.iter(), 1i, |x, _| { x + 1i });
/// assert_eq!(counts.find(&&some_vec[0]), Some(&4i));
/// assert_eq!(counts.find(&&some_vec[2]), Some(&2i));
/// assert_eq!(counts.find(&&some_vec[6]), Some(&1i));
/// ```
///
/// Note that this is implemented as `frequency`.
pub fn accumulate<K: Ord + Clone, A: Clone, T: Iterator<K>>(sequence: &mut T, init: A, f: |A, K| -> A) -> TreeMap<K, A> {
    let mut map: TreeMap<K, A> = TreeMap::new();
    for x in *sequence {
        // XXX: Rather than pop & insert, just mutate the value in place
        let new_value =
            match map.pop(&x) {
                Some(i) => f(i, x.clone()),
                None    => init.clone(),
            };
        map.insert(x, new_value);
    }
    map
}

/// Count the number of times each element occurs in a sequence.
///
/// # Arguments
///
/// * `sequence` - A sequence of things to be counted.
///
/// # Returns
///
/// A map of elements in the sequence to the number of times they appear.
///
/// # Example
///
/// ```rust
/// let some_vec = vec![1i, 1i, 2i, 1i, 1i, 2i, 3i];
/// let counts = target::frequency(&mut some_vec.iter());
/// assert_eq!(counts.find(&&some_vec[0]), Some(&4i));
/// assert_eq!(counts.find(&&some_vec[2]), Some(&2i));
/// assert_eq!(counts.find(&&some_vec[6]), Some(&1i));
/// ```
pub fn frequency<A: Ord + Clone, T: Iterator<A>>(sequence: &mut T) -> TreeMap<A, int> {
    accumulate(sequence, 1, |x, _| { x + 1i })
}


/// Count the number of times each letter occurs in a string.
///
/// # Arguments
///
/// * `word` - A string of letters to be counted.
///
/// # Returns
///
/// A map of characters in the string to the number of times they appear.
///
/// # Example
///
/// ```rust
/// let some_str = "Hello";
/// let counts = target::letter_counts(some_str);
/// assert_eq!(counts.find(&'H'), Some(&1i));
/// assert_eq!(counts.find(&'e'), Some(&1i));
/// assert_eq!(counts.find(&'l'), Some(&2i));
/// assert_eq!(counts.find(&'o'), Some(&1i));
/// ```
pub fn letter_counts(word: &str) -> TreeMap<char, int> {
    frequency(&mut word.chars())
}


// XXX: Could be made more generic: are two sequences 'anagrams' of each
// other?
pub fn is_anagram(first: &str, second: &str) -> bool {
    // XXX: This can be made faster (e.g. by escaping early when we detect
    // different lengths), but finish the program first.
    letter_counts(first) == letter_counts(second)
}


// XXX: Could be made more generic: can the second sequence be found
// rearranged in the first?
pub fn is_sub_anagram(larger: &str, smaller: &str) -> bool {
    let mut frequencies = letter_counts(larger);
    for c in smaller.chars() {
        match frequencies.find_mut(&c) {
            Some(x) => if *x >= 1i { *x -= 1i } else { return false },
            None    => return false
        }
    }
    true
}

/// A word matches the target word if it has the target character in it, is four or
/// more letters long, and is a sub-anagram of the target word.
pub fn matches_target(target_word: &str, target_char: char, candidate: &str) -> bool {
    candidate.char_len() >= 4
        && candidate.contains_char(target_char)
        // XXX: we're going to be calling this a lot with the same
        // target_word, which means that it will recalculate the frequency a
        // lot. Instead, we probably want to calculate it just once, and
        // either memcpy it (with current mutate-y algorithm) or build
        // frequency for candidate and implement less-than.
        && is_sub_anagram(target_word, candidate)
}


mod test {
    // XXX: This gives a warning in 'cargo test', but if I delete it, my
    // flycheck complains.
    use std::collections::TreeMap;

    #[test]
    fn empty_anagram_of_empty() {
        assert!(super::is_anagram("", ""));
    }

    #[test]
    fn equal_things_are_anagrams() {
        assert!(super::is_anagram("foo", "foo"));
    }

    #[test]
    fn obvious_non_anagrams() {
        assert!(!(super::is_anagram("foo", "bar")));
    }

    #[test]
    fn anagrams() {
        assert!(super::is_anagram("foo", "ofo"));
        assert!(super::is_anagram("foo", "oof"));
    }

    #[test]
    fn non_anagrams() {
        assert!(!super::is_anagram("foo", "ffoo"));
    }

    #[test]
    fn empty_frequency() {
        let v:Vec<int> = vec![];
        let m: TreeMap<&int, int> = TreeMap::new();
        let f: TreeMap<&int, int> = super::frequency(&mut v.iter());
        assert_eq!(m, f);
    }

    #[test]
    fn single_frequency() {
        let v:Vec<&str> = vec!["hello"];
        let mut m: TreeMap<&&str, int> = TreeMap::new();
        m.insert(&v[0], 1);
        let f: TreeMap<&&str, int> = super::frequency(&mut v.iter());
        assert_eq!(m, f);
    }

    #[test]
    fn multiple_frequency() {
        let v = "hello world";
        let mut m: TreeMap<char, int> = TreeMap::new();
        m.insert('h', 1);
        m.insert('e', 1);
        m.insert('l', 3);
        m.insert('o', 2);
        m.insert('w', 1);
        m.insert('r', 1);
        m.insert('d', 1);
        m.insert(' ', 1);
        let f: TreeMap<char, int> = super::frequency(&mut v.chars());
        assert_eq!(m, f);
    }

    #[test]
    fn anagrams_are_subanagrams() {
        assert!(super::is_sub_anagram("foo", "foo"));
        assert!(super::is_sub_anagram("foo", "ofo"));
        assert!(super::is_sub_anagram("foo", "oof"));
    }

    #[test]
    fn actual_subanagrams() {
        assert!(super::is_sub_anagram("target", "rag"));
        assert!(super::is_sub_anagram("target", "rage"));
        assert!(super::is_sub_anagram("target", "great"));
    }

    #[test]
    fn non_subanagrams() {
        assert!(!super::is_sub_anagram("target", "kennel"));
        assert!(!super::is_sub_anagram("target", "targeter"));
    }

    #[test]
    fn matches_target_word() {
        assert!(super::matches_target("target", 't', "great"));
    }

    #[test]
    fn too_short_for_target() {
        assert!(!super::matches_target("target", 't', "get"));
    }

    #[test]
    fn target_char_not_present() {
        assert!(!super::matches_target("target", 't', "rage"));
    }

    #[test]
    fn not_subanagram_of_target() {
        assert!(!super::matches_target("target", 't', "goat"));
    }
}
