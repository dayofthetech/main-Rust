//The Slicin Type
//write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
// If the function doesn’t find a space in the string, the whole string must be one word,
//so the entire string should be returned.

//first this is how string slice looks
// fn main() {
//     let s = String::from("hello world");

//     //the [0..5] are [starting_index..ending_index]
//     //another way to start from index zero is to leave the first value empty
//     //[..5]

//     let _hello = &s[0..5];
//     let _world = &s[6..11];

//     println!("{}", _hello);
// }

//or drop both values and now it will take the entire string
//below are equal
// fn main() {
// let s = String::from("hello");

// let len = s.len();

// let _slice = &s[0..len];
// let _slice = &s[..];
// println!("{}", _slice);
// }
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

//12/15 -
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);

    //these two print hello world
    println!("{}", my_string);
    println!("{}", my_string_literal);
    //this one prints hello
    println!("{}", _word);
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


