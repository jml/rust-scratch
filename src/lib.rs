use std::collections::TreeMap;

// TODO: documentation!
// TODO: Better brace / completion handling in emacs (paredit? yasnippet?)
// TODO: Integration tests!

// XXX: What's this &'a syntax I see everywhere?
// e.g. fn iter(&'a self) -> Entries<'a, K, V>

// XXX: How do I make this type signature not make promises about map
// implementation?

// XXX: I guess it's OK for the iterator to be mutable, since what else is
// .next() going to do? Is there a way to express this with immutable
// parameters?

// XXX: Cloning all over the shop here. Is there a more sensible way to do this?

// XXX: Untested (extracted from frequency)
pub fn accumulate<A: Ord + Clone, B: Clone, T: Iterator<A>>(sequence: &mut T, init: B, f: |B, A| -> B) -> TreeMap<A, B> {
    let mut map: TreeMap<A, B> = TreeMap::new();
    for x in *sequence {
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


pub fn is_anagram(first: &str, second: &str) -> bool {
    let mut map: TreeMap<char, int> = TreeMap::new();
    for c in first.chars() {
        let new_count =
            match map.pop(&c) {
                Some(i) => i + 1,
                None    => 1,
            };
        map.insert(c, new_count);
    }
    first == second
}

mod test {
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
}
