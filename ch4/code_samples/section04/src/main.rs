fn main() {
    println!("Hello, world!");

    let my_string = String::from("hello world");

    // works on slices of Strings, whether partial or whole
    let word = first_word_slice(&my_string[0..6]);
    let word = first_word_slice(&my_string[..]);
    // also works on references to Strings which are equivalent to whole slices of Strings
    let word = first_word_slice(&my_string);

    let my_string_literal = "hello world";

    // works on slices of string literals, whether partial or whole
    let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);
    // because string literals are string slices already, this works too, without slice syntax
    let word = first_word_slice(my_string_literal);

    // slices also work on arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// returning an index of the end of the first word, or return len of string if no spaces. Rust has slices so this is not needed
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

// instead of returning an index, we can return a slice of the string.
// This is better because we don't have to worry about juggling indexes or an index being out of bounds
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

// let's try to make a second_word function that returns the second word of a string
fn second_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    let start_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if start_index > 0 && item == b' ' {
            return &s[start_index..i];
        } else if item == b' ' {
            let start_index = i+1;
        }
    }

    return &s[..];
}