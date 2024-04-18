
fn main() {
    // println! is macro.
    println!("Hello, world!");

    // let s = "I am superman".to_string();
    let s = "I am superman";
    for _i in 1..10 {
        let tmp_s = s;
        println!("tmp_s: {:?}", tmp_s);
    }
}