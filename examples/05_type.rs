fn main() {
    let decimal = 65.4321_f32;
    // let integer: u8 = decimal;
    let integer = decimal as u8;
    let character = integer as char;
    // let character = decimal as char;  // Error

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);
    // println!("1000 as a u8 is: {}", 1000 as u8);
    println!("  -1 as a u8 is: {}", (-1i8) as u8);
    println!("1000 mod 256 is: {}", 1000 % 256);
    println!(" 128 as a i16 is: {}", 128 as i16);
    // println!(" 128 as a i8 is: {}", 128 as i8);

    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }

    // Type inference
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);

    // Type alias
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;
    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}
