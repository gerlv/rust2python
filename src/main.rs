use std::collections::HashMap;

#[derive(Debug)]
struct Matrix(f32, f32);

fn main() {
    println!("Hello, world!");

    let m = Matrix(101.2, 202.12);
    println!("{:?}", m);

    // arrays
    println!("### Arrays");

    let xs = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("First {:?} {:?}", xs[0], ys[0]);
    println!("Len: {:?} {:?}", xs.len(), ys.len());

    println!("Slice xs: {:?}", &xs[1..3]);
    println!("Slice ys: {:?}", &ys[1..3]);

    // println!("Out of bound {:?}", xs[5]); // - fails to compile

    // vectors (lists in python)
    println!("### Vectors");

    let mut xs = vec![1_i32, 2, 3];
    println!("Initial vector {:?}", xs);
    xs.push(4);
    println!("Added 4: {:?}", xs);
    // panics - println!("Access 6th: {:?}", xs[5]);
    println!("Slice: {:?}", &xs[1..3]);
    println!("Pop last element {:?}", xs.pop());
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("Enumerate pos: {} val: {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Multiplied vector: {:?}", xs);

    // 0..10 <- creates an iterator, collect collects it
    let collected_iter: Vec<i32> = (0..10).collect();
    println!("Collected iterator: {:?}", collected_iter);

    // Strings
    println!("### Strings");
    // .chars
    // crate: unicode_normalization
    // utf-8
    // Uses 1-4 bytes per character
    // 7  bits - from U+0000 to U+007F   - 0xxx_xxxx
    // 11 bits - from U+0080 to U+07FF   - 110x_xxxx 10xx_xxxx
    // 16 bits - from U+0800 to U+FFFF   - 1110_xxxx 10xx_xxxx 10xx_xxxx
    // 21 bits - from U+1000 to U+10FFFF - 1111_xxxx 10xx_xxxx 10xx_xxxx 10xx_xxxx
    // 110x - two bytes are used, 1110 - three bytes are used, 1111 - 4 bytes are used
    // A = hex(41), bin(0100_0001)
    // Ä = hex(C3 84), bin(1100_0011 1000_0100)
    // € = hex(E2 82 AC), bin(1110_0010 1000_0001 1010_1100)
    // 𝄞 = hex(F0 9D 84 9E), bin(1111_0000 1001_1101 1000_0100 1001_1110)

    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");
    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    let panagram: &'static str = "the quick brown fox jumps over the lazy";
    println!("Panagram: {}", panagram);

    println!("Words in reverse");
    for word in panagram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = panagram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used    chars: {}", string);
    println!("Trimmed chars: {}", trimmed_str);

    let names: Vec<String> = vec!["oleg".to_string(), "óleg".to_string(), "¶πø^ä".to_string()];
    for name in names.iter() {
        println!("{} -> {}", name, naive_capitalize(name));
        println!("{} -> {}", name, ascii_capitalize(name));
    }

    // UTF-8

    let ascii = String::from("Hello");
    println!(
        "'{}': length: {}, chars: {}, memsize: {}",
        ascii,
        ascii.len(),
        ascii.chars().count(),
        std::mem::size_of::<String>() + ascii.len()
    );
    println!("{:?}", ascii.as_bytes());
    println!("after e: {:?}", &ascii[2..]);

    let uni = String::from("Héllǒ");
    println!(
        "'{}': length: {}, chars: {}, memsize: {}",
        uni,
        uni.len(),
        uni.chars().count(),
        std::mem::size_of::<String>() + uni.len()
    );
    println!("{:?}", uni.as_bytes());
    // println!("after e: {:?}", &uni[2..]);  // throws error - byte index 2 is not a char boundary; it is inside 'é' (bytes 1..3) of `Héllǒ`

    // multiline strings
    let more = "many
    lines"
        .to_string();
    println!("{}", more);

    let escape = " blah \"\"\" asd '
newline \n
\n
test
"
    .to_string();
    println!("{}", escape);

    let single_char = 'a'; // 'ab' <- fails
    println!("{}", single_char);

    // functions

    println!("{}", do_something(3));

    // closures
    let example_closure = |x| x;
    let s = example_closure(String::from("Hello"));
    // let n = example_closure(5); <- fails as closures are strongly typed
    let n = example_closure(5.to_string()); // <- works

    println!("{} {}", s, n);

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    let items: Vec<i32> = vec![1, 2, 3, 4, 5];
    let plus_one: Vec<_> = items.iter().map(|x| x + 1).collect();
    let sum_all: i32 = items.iter().map(|x| x + 1).sum();
    println!("{:?} {}", plus_one, sum_all);

    let two_args = |x, y| x - y;
    println!("{}", two_args(5, 3));

    v05_fizzbuzz();

    v06_pattern_matching();

    v07_structs();

    v08_hashmaps();

    v09_iterators();

    v0a_error_handling();

    v0b_traits();

    v0c_operator_overloading();
}

fn naive_capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

fn ascii_capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_ascii_uppercase().to_string() + c.as_str(),
    }
}

fn do_something(x: u32) -> u32 {
    fn do_something_else(x: u32) -> u32 {
        x * 3
    }
    do_something_else(x)
}

fn v05_fizzbuzz() {
    let fizz_buzz = |x| {
        if x % 15 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", x)
        }
    };

    println!("---- FizzBuzz Start ----");
    for i in 1..16 {
        fizz_buzz(i);
    }
    println!("----");
    (1..16).into_iter().for_each(fizz_buzz);
    println!("---- FizzBuzz End ----");
}

fn v06_pattern_matching() {
    println!("---- Pattern Matching Start ----");

    struct Dog {
        name: String,
    }

    struct Cat {
        age: u8,
    }

    enum Animals {
        Dog(Dog),
        Cat(Cat),
    }

    fn classify(animal: Animals) {
        match animal {
            Animals::Dog(d) => println!("A dog named: {}", d.name),
            Animals::Cat(Cat { age }) => println!("A cat aged: {}", age),
            // _ => println!("any animal") // <- any case, here will show a warning
        }
    }

    fn number(x: i32) {
        match x {
            1 => println!("one"),
            2 | 3 => println!("two or three"),
            4..=i32::MAX => println!("four or bigger"),
            _ => println!("anything"),
        }
    }

    for x in 0..6 {
        number(x)
    }
    classify(Animals::Dog(Dog {
        name: "Fido".to_string(),
    }));
    classify(Animals::Cat(Cat { age: 3 }));

    let fizz_buzz = |x| match (x % 3, x % 5) {
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        (_, _) => println!("{}", x),
    };

    (1..24).into_iter().for_each(fizz_buzz);

    println!("---- Pattern Matching End ----");
}

fn v07_structs() {
    println!("---- Structs Start ----");

    struct A {
        x: i32,
        y: i32,
    }

    // struct B {
    //     members: static Vec<i32>, <- doesn't exist in rust
    // }

    let mut a = A { x: 1, y: 2 };
    a.x += 2;

    let a_updated = A { y: 4, ..a };
    println!("a.x {} a_updated.x {}", a.x, a_updated.x);
    println!("a.y {} a_updated.y {}", a.y, a_updated.y);

    #[derive(Debug)]
    struct D {
        x: i32,
        y: i32,
        z: i32,
    }

    impl D {
        fn new(x: i32, y: i32, z: Option<i32>) -> Self {
            match z {
                Some(z) => Self { x, y, z },
                None => Self { x, y, z: 0 },
            }
        }
    }

    use std::default::Default; // import trait

    impl Default for D {
        fn default() -> Self {
            Self { x: 0, y: 0, z: 0 }
        }
    }

    let d1 = D::new(1, 2, None);

    // provides default option for all values, no way to enforce just 1 value
    // if users have to provide some value, use `new` convention
    let d2 = D {
        x: 1,
        y: 2,
        ..D::default()
    };

    println!("{:?}", d1);
    println!("{:?}", d2);

    println!("---- Structs End ----");
}

#[macro_use]
extern crate maplit;
fn v08_hashmaps() {
    println!("---- HashMaps Start ----");

    use std::collections::HashMap;

    let literal: HashMap<_, _> = vec![("key", "value"), ("blah", "blubb")]
        .into_iter()
        .collect();
    println!("{:?}", literal);

    let mut mutable = HashMap::new();
    mutable.insert("one", 1);
    mutable.insert("two", 2);
    mutable.remove("one");
    println!("{:?}", mutable.get("one"));
    println!("{:?}", mutable.get("two"));

    mutable.insert("three", 3);

    for (k, v) in &mutable {
        println!("{}: {}", k, v);
    }

    let map = hashmap! {
        "a" => 1,
        "b" => 2,
    };

    println!("{:?}", map);

    println!("---- HashMaps End ----");
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn v09_iterators() {
    println!("---- Iterators Start ----");
    println!("Code in tests run using `cargo test`");
    println!("---- Iterators End ----");
}

#[cfg(test)]
mod tests {
    use super::*; // test has to import what we define in the library

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}

// Start error handling

fn v0a_error_handling() {
    println!("---- Error handling Start ----");
    // rust has no exceptions
    // use Result<T, E> to return value or error
    // or Option<T> for optional values
    // For libraries - https://crates.io/crates/thiserror
    // For apps - `anyhow` or `eyre`

    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    // let nothing: Option<std::option::Option<None>> = None;

    give_royal(bird);
    // give_royal(nothing); - throws an error on unwrap

    // unpacking options with ? <- this can be nested in a line, i.e. op1?.op2?.value;

    let age: Option<u8> = Some(32);
    next_birthday(age);

    let twenty = multiply("10", "2");
    println!("Twenty: {}", twenty);
    // let tt = multiply("t", "2"); // thorws an error
    // println!("double is {}", tt);

    // error handling boilerplate
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));

    println!("---- Error handling End ----");
}

// error handling boilerplate for Rust
use std::error;
use std::error::Error;
use std::fmt;
use std::fmt::{format, Formatter};
use std::num::ParseIntError;

// Initial implementation, without custom errors
// standard type result, error boxed
// type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
//
// #[derive(Debug)]
// struct EmptyVec;  // error struct + display for this
// impl fmt::Display for EmptyVec {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "invalid first item to double")
//     }
// }
// impl error::Error for EmptyVec {}
//
// fn double_first(vec: Vec<&str>) -> Result<i32> {
//     let first = vec.first().ok_or(EmptyVec)?;
//     let parsed = first.parse::<i32>()?;
//     Ok(2 * parsed)
// }
//

// Second implementation, with custom errors
// type Result<T> = std::result::Result<T, DoubleError>;
//
// impl fmt::Display for DoubleError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match *self {
//             DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
//             DoubleError::Parse(ref e) => e.fmt(f),
//         }
//     }
// }
//
// impl error::Error for DoubleError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match *self {
//             DoubleError::EmptyVec => None,
//             DoubleError::Parse(ref e) => Some(e),
//         }
//     }
// }
//
// #[derive(Debug)]
// enum DoubleError {
//     EmptyVec,
//     Parse(ParseIntError)
// }
//
// // conversion from ParseIntError to DoubleError
// // This will be automatically called by `?` if a `ParseIntError`
// // needs to be converted into `DoubleError`
// impl From<ParseIntError> for DoubleError {
//     fn from(err: ParseIntError) -> Self {
//         DoubleError::Parse(err)
//     }
// }

use eyre::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum DoubleError {
    #[error("no first item")]
    EmptyVec,
    #[error("invalid first item, error: '{0}'")]
    Parse(#[from] std::num::ParseIntError),
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => {
            println!("The first double is {}", n)
        }
        Err(error) => {
            println!("Error: {}", error)
        }
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>().map_err(|e| DoubleError::Parse(e))?;
    Ok(2 * parsed)
}

fn multiply(first_num_str: &str, second_num_str: &str) -> i32 {
    let first_num = first_num_str.parse::<i32>().unwrap();
    let second_num = second_num_str.parse::<i32>().unwrap();
    first_num * second_num
}

fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // if current age is None - returns None;
    // if current age is Some - inner u8 gets assigned to a value
    let next_age: u8 = current_age?;
    Some(format!("Next year I will be {}", next_age))
}

fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck, I'm putting this snake back"),
        Some(inner) => println!("{}? How nice", inner),
        None => println!("No gift? Oh well"),
    }
}

fn give_royal(gift: Option<&str>) {
    let inside = gift.unwrap(); // throws an error if gift is None
    if inside == "snake" {
        panic!("AaaaAaaAA!1");
    }
    println!("I love {}s", inside);
}

// End error handling
fn v0b_traits() {
    // informs about functionality a type can share
    // similar to interfaces or mixins in Python

    println!("---- Traits Start ----");
    // derive macro automatically implement traits to your structs

    let d1 = D {
        x: 3,
        ..D::default()
    };
    let d2 = D { x: 3, y: 5 };
    println!("D sums: {:?}", d1 + d2);

    // useful traits:
    // From or TyFrom to convert from strings to some val
    // Display - to display and format values

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    let p = make_person(8);
    println!("Person: {}", p.name());

    let cs = CollegeStudent("Bert".to_string());
    println!("{}", comp_sci_student_greeting(&cs));
    println!("{}", comp_sci(&cs));
    let prog = RustProgrammer("Bob".to_string());
    println!("{}", comp_sci_vs_programmer(&cs, &prog));

    println!("---- Traits End ----");
}

#[derive(Debug, PartialEq, Default)]
struct D {
    x: i32,
    y: i32,
}

impl std::ops::Add for D {
    type Output = D;

    fn add(self, rhs: Self) -> Self::Output {
        D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        // similar to defer in go, this will execute when iterator stops
        // so you can clean up resources or close connection to db, etc

        println!("Dropping at {}", self.count);
    }
}

// can also define trait for a return type

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

// super traits - interface overloading

trait Person {
    fn name(&self) -> String {
        String::from("Unnamed")
    }
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct StreetPerson(String);
struct RustProgrammer(String);
struct CollegeStudent(String);
impl Person for StreetPerson {}
impl Person for RustProgrammer {}

impl Programmer for RustProgrammer {
    fn fav_language(&self) -> String {
        return String::from("Rust");
    }
}

impl Person for CollegeStudent {
    fn name(&self) -> String {
        self.0.clone()
    }
}

impl Programmer for CollegeStudent {
    fn fav_language(&self) -> String {
        return String::from("From Python to Rust");
    }
}

impl Student for CollegeStudent {
    fn university(&self) -> String {
        String::from("Community college")
    }
}

impl CompSciStudent for CollegeStudent {
    fn git_username(&self) -> String {
        self.0.to_lowercase()
    }
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "Name is {} I attend {} Fav lang is {} git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn comp_sci<T: CompSciStudent>(student: &T) -> String {
    // Same function as above but instead of &dyn stuff it uses generic type
    comp_sci_student_greeting(student)
}

fn comp_sci_vs_programmer<T, U>(student: &T, programmer: &U) -> String
where
    T: CompSciStudent,
    U: Programmer + Person,
{
    println!(
        "Programmer named {} loves {}",
        programmer.name(),
        programmer.fav_language()
    );
    comp_sci(student)
}

fn make_person(rnd: u8) -> Box<dyn Person> {
    // return generic value
    // dyn used for heap allocations which aren't known at the compile time
    // Box is known at the compile type as it's a pointer to the heap
    match rnd {
        0..=3 => Box::new(StreetPerson("Bob".to_string())),
        4..=6 => Box::new(CollegeStudent("Jake".to_string())),
        7..=9 => Box::new(RustProgrammer("Don".to_string())),
        _ => Box::new(StreetPerson("Someone".to_string())),
    }
}

// End traits

// Start 0C. Operator Overloading
fn v0c_operator_overloading() {
    println!("---- Operator Overloading Start ----");

    let ms = MyString("Foo".to_string());
    println!("Foo + Bar = {}", ms.clone() + "Bar".to_string());
    println!("Foo2 + -12 = {}", ms.clone() + -12_i32);
    println!("100 + Foo3 = {}", 100_i32 + ms.clone());
    println!("Foo4 * -12 = {}", ms.clone() * -12_i32);
    println!("Foo4 * 12 = {}", ms.clone() * 12_i32);

    println!("---- Operator Overloading End ----");
}

use std::fmt::Write;
use std::ops;

#[derive(Debug, Clone)]
struct MyString(String);

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Add<String> for MyString {
    type Output = MyString;

    fn add(self, rhs: String) -> Self::Output {
        println!("> MyString.add<String>({}) was called", &rhs);
        MyString(format!("{}{}", self.0, rhs))
    }
}

impl ops::Add<i32> for MyString {
    type Output = MyString;

    fn add(self, rhs: i32) -> Self::Output {
        println!("> MyString.add<i32>({}) was called", &rhs);
        MyString(format!("{}{}", self.0, rhs))
    }
}

impl ops::Add<MyString> for i32 {
    type Output = MyString;

    fn add(self, rhs: MyString) -> Self::Output {
        println!("> i32.add<MyString>({}) was called", &rhs);
        MyString(format!("{}{}", self, &rhs))
    }
}

impl ops::Mul<i32> for MyString {
    type Output = MyString;

    fn mul(self, rhs: i32) -> Self::Output {
        println!("> MyString.mul<i32>({}) was called", &rhs);
        let caplen: usize = if rhs < 0 { 0 } else { rhs as usize };
        let mut temp = String::with_capacity(&self.0.len() * caplen);
        for _ in 0..rhs {
            temp.write_str(&self.0).expect("writing to string failed");
        }
        MyString(temp)
    }
}

// End 0C. Operator Overloading
