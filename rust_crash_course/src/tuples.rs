//tuples group together values of different types
//max 12 elements
pub fn run() {
    let person: (&str, &str, i8) = ("bard", "georgia", 32);

    println!("{} is from {} and age", person.0, person.1);

}