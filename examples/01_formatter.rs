use std::fmt;

fn main() {
    // {} is a placeholder, will be stringified.
    println!("{} days", 31);

    // Positional arguments.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting after a `:`.
    println!("Base 10: {}", 424242);
    println!("Base 2(binary): {:b}", 424242);
    println!("Base 8(octal): {:o}", 424242);
    println!("Base 16(hexadecimal): {:x}", 424242);
    println!("Base 16(hexadecimal): {:X}", 424242);

    // Right-align text with a specified width.
    println!("{number:>5}", number=5);
    println!("{number:0>5}", number=5);
    println!("{number:0<5}", number=5);
    println!("{number:>width$}", number=1, width=6);

    // Only types that implement fmt::Display can be formatted with {}, user-defined
    // types do not implement fmt::Display by default.
    #[allow(dead_code)]
    struct NonDisplay(u32);
    // println!("{}", NonDisplay(100));

    println!("{}", User(1));

    let pi = 3.1415926;
    println!("{:.3}", pi);
    let prec = 4;
    println!("{pi:.prec$}", pi=pi);

    debug_print();
    container_print();
}

fn debug_print() {
    println!("{:?} months in a year.", 12);
    println!("{:?}", DebugPrintable(100));
    println!("{:?}", Deep(DebugPrintable(101)));
    println!("{:#?}", Deep(DebugPrintable(101)));

    let name = "Alice";
    let age: u8 = 30;
    let alice = Person { name, age };
    println!("{:#?}", alice);
}

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

#[derive(Debug)]
struct Person<'a> {
    #[allow(dead_code)]
    name: &'a str,
    #[allow(dead_code)]
    age: u8,
}

#[allow(dead_code)]
struct User(u32);

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User({})", self.0)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn container_print() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}