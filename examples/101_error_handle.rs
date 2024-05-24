// Error Handling in Rust.
// 1. 不可恢复错误 (Unrecoverable Errors)
//  - panic! 宏
//  - todo! 宏
//  - unimplemented! 宏
//  - unreachable! 宏
// 2. 可恢复错误 (Recoverable Errors)
//  - Result<T, E>: T 代表成功返回的值, E 代表错误类型
//    - Ok(T): 成功返回的值
//    - Err(E): 错误返回的值

use std::net::IpAddr;
use std::error::Error;

// 业务来决定使用那一种错误处理方式
fn main() {
    // Case1: call foo
    // foo();  // 这里会提示 warning, 因为没有处理返回的 Result
    let _ = foo();  // 这里忽略了返回的 Result

    // Case2: parse error
    parse_error();
}

// Case1
fn foo() -> Result<String, String> {
    Ok("foo".to_string())
}

// Case2
fn parse_error() {
    let s = "100eee";
    if let Err(e) = s.parse::<i32>() {
        // here e is ParseIntError
        println!("Failed conversion to i32: {e}");
    }

    let addr = "127.0.0.1:8000".parse::<IpAddr>();
    if let Err(e) = addr {
        // here e is AddrParseError
        println!("Failed conversion to IpAddr: {e}");
    }
}

// Case3: how to handle Result<T,E> in the upper logic.
fn handle_result() {

}
