// How to create a new rust folder and run code cmp
// - open cmp and go to the main rust where the folder will be branched off
// - type cargo new name and it will compile a new folder
//

fn main() {
    // below - print line
    println!("Hello, world! This is Rust");
}

// https://www.youtube.com/watch?v=xYgfW8cIbMA
//variables
//to make a variable you start with the coding block

// fn main() {
//     //when adding the let, the variable is inmutable
//     let x = 4;
//     println!("x is: {} ", x);

//     //to reassigned x to another int, you will need another let
//     let x = 5;
//     println!("x is: {} ", x);

//     // above - both lines will print and is working
//     // x is: 4
//     // x is: 5

// }

// fn main() {
//     //not, to use the same variable without the let, you will have to turn
//     //your first var into a mut
//     let mut x = 4;
//     println!("x is: {} ", x);

//     x = 5;
//     println!("x is: {} ", x);
//     //is working because both are int

// }

//name shadowing from different scopes
// fn main() {
//     let x = 4;
//     println!("x is: {} ", x);

//     //to make a scope you start with curly brackets
//     {
//         // let x = 2;
//         //but you can change the inner scope
//         //below - at this time x is 4 , and here in the inner code you are
//         //changing x to 7
//         let x = x + 3;
//         println!("x is: {} ", x);
//     }
//     let x = 5;
//     println!("x is: {} ", x);

//     //this will print normal,
//     // x is: 4
//     // x is: 2
//     // x is: 5
//     //the exterior scope wont change from what happen in the inner scope
//     //but the inner scope can change based from the outter

// }

//Constants
// fn main(){
//     //the way to wrtie a constant is snake UPPERCASE, and separte by underline
//     //you also have to add the type of the constast, in this case u32
//     const SECONDS_IN_MINUTE: u32 = 60;
//     println!("{}", SECONDS_IN_MINUTE);
// }

//Data Types
//u32 measn unsigned interegrs and you cannot use negative nu,ners
//i32 is integers numbers and you can use negatives

//float is for decimal

//bool is for boolenas, true or false - or 0 or 1

//char is for characters inside single quote ''

//Compaund Types
// fn main() {
//     //you have o add the type of the items in the tup
//     // let tup: (i32, bool, char) = (1, true, 'y');

//     //if you want to changes the values of the elements, you have to
//     //add the mut to the tupple
//     let mut tup: (i32, bool, char) = (1, true, 'y');
//     tup.0 = 10;

//     //to access the tuple elements you use tup.index
//     println!("{}", tup.0);
// }


// Arrays
// fn main(){
//     let arr = [1, 2, 3, 4, 5];
// }

//Collecting User Input
//to begin you new to load a crate or package that will be use in rust
//the one for input will be called io
//std means standard library
// use std::io;

// fn main(){
//     //your input variable must be a mut
//     let mut input = String::new();

//     //if an error were to occur, thats why you add the .expect
//     io::stdin().read_line(&mut input).expect("failed to read line");
//     println!("{}", input);
// }

//Arithmetic Type Casting
//An important part of Rust is that every int muts be of the same
//type, being u8, i8, f64 and so on.
//whichever numbers are being operated on

// fn main(){
//     //this will work - but if I choose to add a float, being f32 or so
//     //the numbers must have decimals such as
//     //20.0 or 15.0 in this case
//     let x: i64 = 20;
//     let y: i64 = 15;

//     let z = x + y;
//     println!("{}", z);
// }

//Add values of different types

//Converting string to integer
// use std::io;

// fn main(){
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("expected to read line");

//     let int_iput: i64 = input.trim().parse().unwrap();

//     println!("{}", int_iput + 2);
// }

//Conditons and Control Flow
//conditionals are those more than, less than type
// fn main(){
//     let cond = 2 < 5;

//     let cond2 = true && cond;
//     //this will print true
//     println!("{}", cond);

// }
//Now the other componding operator are
//&& for AND
// || for OR
// ! for NOT

//Control Flow if/else
// fn main(){
//     let food = "cookie";

//     if food == "cookie" {
//         println!("You like cookies");
//     } else if food == "fruit" {
//         println!("You like fruit");
//     } else if food == "bread" {
//         println!("You like bread");
//     } else {
//         println!("You dont like the menu");
//     }
// }

//Functions
//The naming convention for function is snakeCase name_name
// The reason this funciton is called main is because
//      the rust file under src is called main
//      for this example the functions will be inside the main function

//This section also introduce stamentd and expressions
//Statements are those thar don't return anything or something that is static
//  for example let x = 20; is a statement because it doesnt compute to anything else

//Expressions are the rest, so the value 20 itself can be considered a expression
//  below is an exmaple where the second let is considered an expression
//  and because the last line doesnt have a semicolon, it would be considered a return
//
// fn main(){
//     println!("Hello world!");
//     let number = {
//         let x = 3;
//         x + 1
//     };
// }
// fn main(){
//     println!("Hello world!");
//     add_numbers(20, 50);
// }

// //when passing parameters to a function, you need to specify the type, for example i32
// //
// fn add_numbers(x: i32, y: i32){
//     //the purpose of this f is to return the sum of x + y
//     //and the values are provided in the function above
//     println!("The sum is: {}", x + y)
// }

// A more complex return fn
// fn main(){
//     println!("Hello world!");
//     let result = add_numbers(9, 3);
//     println!("{}", result);
// }
// //to tell the fn you will return something, you add an arrow -> follow by the type of said function
// fn add_numbers(x: i32, y: i32) -> i32 {
//     // x + y

//     //or you can explicit called the return, which I dont know why not i not use this all the time
//     //and this can b with or without semicolon
//     //return x + y;

//     //or you can also create a statement as long as the expression is the last line inside the fn
//     //so exp result_fn will be return as the i32 from this fn
//     //naming convention does not apply for the main fn
//     // let result_fn = x + y;
//     // result_fn

//     //another method is if you want to return an early value from this fn, lets say there is an if statement
//     let result_fn = x + y;
//     if result_fn > 10 {
//         return result_fn - 5;
//     }
//     result_fn
// }

//