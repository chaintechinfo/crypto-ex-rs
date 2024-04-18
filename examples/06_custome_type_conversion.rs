#![allow(dead_code)]

extern crate core;

use core::fmt;
use std::fmt::Formatter;

fn main() {
    let my_str = "hello";
    let mut my_string = String::from(my_str);
    my_string.push_str(", world");
    assert_eq!(my_string, "hello, world");

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number2 = int.into();
    println!("My number is {:?}", num);

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Parsing a string into a number
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug)]
struct Number2 {
    value: i32,
}
impl Into<Number2> for i32 {
    fn into(self) -> Number2 {
        Number2 { value: self }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 { Ok(EvenNumber(value)) } else { Err(()) }
    }
}

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}
