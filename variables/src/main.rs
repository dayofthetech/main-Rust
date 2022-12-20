// This section about mut and variables is under tutorial2 by TimCode
// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// Numeric operations worded in Rust
// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let floored = 2 / 3; // Results in 0

//     // remainder
//     let remainder = 43 % 5;
// }

// Char
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// decostructed tuples
// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");
// }


// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     //The issue with this line is that if user enters a number outside the array index,
//     //it will throw error.
//     //12/13 - and then documentation moves to another topic lol
//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }

// parameters in functions
//below - the parameter value has a singed value for int, while the unit has the char type
//as long as the char parameter is inside the quotes it can work
// fn main() {
//     print_labeled_measurement(5, '5');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// bool
// fn main() {
//     let number = 3;

//     if number {
//         // intersting, even tho 3 is a int, is not considered true and the program throws an error
//         // so unless you hard code the number = 3
//         //  or add if number != 0 {
//     //     println!("number was something other than zero");
//     // }
//         println!("number was three");
//     }
// }

// FizzBuzz on Rust
// fn main() {
//     let number = 56;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let condition = true;
//     // this will continue to print number because it always will evaluate to true
//     //  only if the value after else gets change to something else besides an int, it will give error

//     let number = if condition { 8 } else { 6 };

//     println!("The value of number is: {number}");
// }

//returning values from loop
// fn main() {
//     // mutable var to be reasigned
//     let mut counter = 0;

//     //this itaration holds the value of each loop
//     let result = loop {
//         counter += 1;

//         //once it reaches it goal it will break and perform a last action
//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

//loop labels
// fn main() {
//     //the count is increasing until it reaches the second loop
//     let mut count = 0;
//     //first loop - outer
//     'counting_up: loop {
//         println!("count = {count}");
//         //creates a second var
//         // and remaining resets back to 10 once it hits the first if
//         let mut remaining = 10;

//         //inner loop
//         loop {
//             //calling remaining from outside, and it does not exit until it matches one of the args
//             //the code still prints because remaining resets back to 10
//             println!("remaining = {remaining}");
//             if remaining == 5 {
//                 //once remaining matches 9, only this if breaks, not the whole loop
//                 break;
//             }
//             if count == 4 {
//                 //once it reaches 2, it breaks the outer loop
//                 break 'counting_up;
//             }
//             //outside any ifs
//             remaining -= 1;
//         }

//         //this is inside the outer loop
//         count += 1;
//     }
//     println!("End count = {count}");
// }

//conditionals with if
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         //while the number is not 0, print and keep reducing number
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

//conditionals foor lop
// fn main() {
//     //only works with int, if i try to add char it wont work
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }
//same code as above but using the rev method
// fn main() {
//     //which i guess it works from the element before the last int
//     //then it goes in reverse until it reaches the index
//     for number in (1..8).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }

//storage heap -  memory allocation
// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{}", s); // This will print `hello, world!`
// }

//ways to create a full copy of a var
// fn main() {
//     let s1 = String::from("hello");
//     //in this example the heap data also gets copied
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }
//unlike integers that don't req the clone method, because rust assgigs a
//auto method called copy
// fn main() {
//     let x = 5;
//     let y = x;

//     println!("x = {}, y = {}", x, y);
// }
//below - These are other types that implement copy
// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that also implement Copy.
// For example, (i32, i32) implements Copy,
// but (i32, String) does not.

// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1

//     let s2 = String::from("hello");     // s2 comes into scope

//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

//reference, this method allows to call a value and the pointer of data will be available
// fn main() {
//     let s1 = String::from("hello world!");

//     //the ampersands & represent reference and allow to refer or point to some value without taking owenrship
//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, it is not dropped.

//when the function has the reference & as parameter instead of the actual value
//there is no need to return the value since it was never owned
//   fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// //this will print error because references & are not allowed to modify
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

//a fixed version will include the mut for the variables
// is missing a print
// fn main() {
//     //change the s to be mut
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }

//The Slice Type
//the instruction and notes from this is located in the word splicing game

//Defining and Instanting Structs

//below - defining a struct

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
// }


// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     // --snip--

//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: String::from("another@example.com"),
            //the update syntax achieves the same effect with less
            //code by adding two dots, .. this means the remaining fields will stay the same
//         ..user1
//     };
// }
