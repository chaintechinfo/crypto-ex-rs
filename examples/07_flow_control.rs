fn main() {
    // if/else
    let n = 5;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };
    println!("{} -> {}", n, big_n);

    // loop
    'outer: loop {
        println!("Entered the outer loop");

        #[allow(unreachable_code, unused_labels)]
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
    }
    println!("Exited the outer loop");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    // for..in
    // for n in 1..=100 { ... }
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz, ");
        } else if n % 3 == 0 {
            println!("fizz, ");
        } else if n % 5 == 0 {
            println!("buzz, ");
        } else {
            println!("{}, ", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names);

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    // match
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime!"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    match_tuples();
    match_arrays_slices();
    match_enums();
    match_pointer();
}

fn match_tuples() {
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("Last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("Some middle number, `3`, and `4`"),
        // (x, y, z) => println!("It doesn't matter what they are"),
        // (..) => println!("It doesn't matter what they are"),
        _ => println!("It doesn't matter what they are"),
    }
}

fn match_arrays_slices() {
    let array = [1, -2, 6];
    match array {
        [0, s, t] => println!("array[0] = `0`, array[1] = {}, array[2] = {}", s, t),
        [1, _, t] => println!("array[0] = `1`, array[2] = {}", t),
        [-1, s, ..] => println!("array[0] = `-1`, array[1] = {}", s),
        [3, s, t @ ..] => println!("array[0] = `3`, array[1] = {}, the other elements were {:?}", s, t),
        [f, m @ .., l] =>
            println!("array[0] = {}, its middle elements were {:?} and array[3] = {}", f, m, l),
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn match_enums() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");

    match color {
        Color::Red => { println!("The color is Red!") }
        Color::Blue => { println!("The color is Blue!") }
        Color::Green => { println!("The color is Red!") }
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
    }
}

fn match_pointer() {
    let reference = &4;
    let reference2 = &4;
    let reference4 = &reference;
    let reference5 = *reference4;

    {
        let reference3 = &4;
        println!("reference3 is {:p}", reference3);
    }

    println!("reference is {:?}", reference);
    println!("reference is {:p}", reference);
    println!("reference2 is {:p}", reference2);
    println!("reference4 is {:p}", reference4);
    println!("reference5 is {:p}", reference5);
    println!("&reference5 is {:p}", &reference5);
    println!("&&reference5 is {:p}", &&reference5);

    let x = 8;
    let y = &x;
    println!("y is {}", *y);
    let z = *&y;
}
