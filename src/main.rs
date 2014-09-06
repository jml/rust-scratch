
extern crate target;

// TODO: documentation!
// TODO: Better brace / completion handling in emacs (paredit? yasnippet?)
// TODO: Integration tests!


use std::io::BufferedReader;
use std::io::File;

use std::ascii::AsciiExt;


#[cfg(not(test))]
fn main() {
    let path = Path::new("/usr/share/dict/words");
    let mut file = BufferedReader::new(File::open(&path));
    let target_word = "CANDIDATE";
    let target_char = 'C';
    for line in file.lines().map(|x| { x.unwrap().as_slice().trim().to_ascii_upper() }) {
        if target::matches_target(target_word, target_char, line.as_slice()) {
            println!("{}", line);
        }
    }
}
