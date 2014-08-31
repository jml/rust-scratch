
extern crate target;

#[cfg(not(test))]
fn main() {
    println!("{}", target::is_anagram("Hello", "World!"))
}
