// This chapter is dedicated to the ownership, borrowing and slices

// OWNERSHIP
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `longest_owned(s1: String, s2: String) -> String` that returns the longer of
// two strings. Check that both original strings are moved into the function, and only the returned
// can still be used.
pub fn longest_owned(s1: String, s2: String) -> String {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// You can implement the function and use it right inside the `string_ownership` function.
#[allow(dead_code)]
pub fn string_ownership() {
    let s1 = String::from("abcd");
    let s2 = String::from("abc");
    let longest = longest_owned(s1, s2);

    println!("{longest}");
    // Compile-Error
    // println!("{s1}");
    // println!("{s2}");
}

// BORROWING
// ================================================================================================

pub fn print_length(s: &str) {
    println!("{}", s.len())
}

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.
//
// You can implement the function and use it right inside the `simple_borrowing` function.
#[allow(dead_code)]
pub fn simple_borrowing() {
    let s = String::from("abcd");
    print_length(&s);
    println!("{s}");
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.

pub fn append_and_return_length(string: &mut String, suffix: &str) -> usize {
    string.push_str(suffix);
    string.len()
}

// You can implement the function and use it right inside the `hard_borrowing` function.
#[allow(dead_code)]
pub fn hard_borrowing() {
    let mut s = String::from("abcd");
    let suffix = "_";
    println!("{} {}", append_and_return_length(&mut s, &suffix), s);
    println!("{} {}", append_and_return_length(&mut s, &suffix), s);
    println!("{} {}", append_and_return_length(&mut s, &suffix), s);
}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
    let bytes = slice.as_bytes();

    let mut end = bytes.len();
    while end > 0 && bytes[end - 1] == b' ' {
        end -= 1;
    }

    let mut start = end;
    while start > 0 && bytes[start - 1] != b' ' {
        start -= 1;
    }

    &slice[start..end]
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
    let bytes = sentence.as_bytes();

    let mut start = 0;

    let mut start_longest = start;
    let mut len_longest = 0;

    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' && i > 0 && bytes[i - 1] != b' ' { // first space, it means that word ended
            let len = i - start;
            if len >= len_longest {
                start_longest = start;
                len_longest = len;
            }
        } else if i > 0 && bytes[i - 1] == b' ' { // first non-space char, word started
            start = i;
        }
    }

    if start > start_longest && bytes.len() - start >= len_longest {
        start_longest = start;
        len_longest = bytes.len() - start;
    }

    &sentence[start_longest..start_longest+len_longest]
}
