use std::collections::TreeMap;

// TODO: documentation!
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

pub fn frequency<A: Ord + Clone, T: Iterator<A>>(sequence: &mut T) -> TreeMap<A, int> {
    accumulate(sequence, 1, |x, _| { x + 1i })
}


pub fn letter_counts(word: &str) -> TreeMap<char, int> {
    frequency(&mut word.chars())
}


pub fn is_anagram(first: &str, second: &str) -> bool {
    // XXX: This can be made faster (e.g. by escaping early when we detect
    // different lengths), but finish the program first.
    letter_counts(first) == letter_counts(second)
}


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
}
