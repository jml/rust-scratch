
pub fn is_anagram(first: &str, second: &str) -> bool {
    first == second
}

mod test {

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

}
