fn main() {
    // let o = Option::None;
    let i = Some(1);
    println!("{}", i.unwrap().to_string());

    let r1: Result<i32, &'static str> = Ok(1);
    r1.unwrap();
    let r2: Result<i32, &str> = Err("error");
    r2.unwrap_err();
    let _f = process_file();
    println!("{}", _f.unwrap());

    let maybe_some_string = Some(String::from("Hello, World!"));
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));
    let x_none: Option<&str> = None;
    assert_eq!(x_none.map(|s| s.len()), None);

    let x_clone = 12;
    let opt_x_clone = Some(&x_clone);
    assert_eq!(opt_x_clone.cloned(), Some(12));

    let x = Some(2);
    assert_eq!(x.is_some(), true);
    assert_eq!(x.is_none(), false);
    let x: Option<i32> = None;
    assert_eq!(x.is_some(), false);
    assert_eq!(x.is_none(), true);
}

fn process_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("Cargo.toml")
}