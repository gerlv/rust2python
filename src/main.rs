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

    let names: Vec<String> = vec![
        "oleg".to_string(),
        "óleg".to_string(),
        "¶πø^ä".to_string(),
    ];
    for name in names.iter() {
        println!("{} -> {}", name, naive_capitalize(name));
        println!("{} -> {}", name, ascii_capitalize(name));
    }

    // UTF-8

    let ascii = String::from("Hello");
    println!("'{}': length: {}, chars: {}, memsize: {}", ascii, ascii.len(), ascii.chars().count(),
        std::mem::size_of::<String>() + ascii.len()
    );
    println!("{:?}", ascii.as_bytes());
    println!("after e: {:?}", &ascii[2..]);

    let uni = String::from("Héllǒ");
    println!("'{}': length: {}, chars: {}, memsize: {}", uni, uni.len(), uni.chars().count(),
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

    let x = vec![1,2,3];
    let equal_to_x = move |z| z == x;
    let y = vec![1,2,3];
    assert!(equal_to_x(y));

    let items : Vec<i32> = vec![1,2,3,4,5];
    let plus_one: Vec<_> = items.iter().map(|x| x + 1).collect();
    let sum_all: i32 = items.iter().map(|x| x + 1).sum();
    println!("{:?} {}", plus_one, sum_all);

    let two_args = |x,y| x - y;
    println!("{}", two_args(5, 3));

    v05_fizzbuzz();

    v06_pattern_matching();
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
            Animals::Cat(c) => println!("A cat aged: {}", c.age),
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

    for x in 0..6 { number(x) }
    classify(Animals::Dog(Dog {
        name: "Fido".to_string(),
    }));
    classify(Animals::Cat(Cat { age: 3 }));

    println!("---- Pattern Matching End ----");
}
