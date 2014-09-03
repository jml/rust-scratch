
extern crate target;

// TODO: documentation!
// TODO: Better brace / completion handling in emacs (paredit? yasnippet?)
// TODO: Integration tests!



#[cfg(not(test))]
fn main() {
    println!("{}", target::is_anagram("Hello", "World!"))
}
