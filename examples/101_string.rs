fn main() {
    // char
    let s = String::from("中国你好");
    let char_vec: Vec<char> = s.chars().collect();
    println!("{:?}", char_vec);

    for ch in s.chars() {
        println!("{:?}", ch);
    }

    // parse
    let a = "10".parse::<u32>();
    let aa: u32 = "10".parse().unwrap();
    println!("{:?}, {:?}", a, aa);
}