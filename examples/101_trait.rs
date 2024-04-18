use std::fmt::Display;

fn main() {
    let p = Point { x: 1, y: 2 };
    print(p);

    let p = Point { x: 1.1, y: 2.1 };
    print(p);

    // Foo 没有实现 Display trait, 无法编译通过
    let p = Point { x: Foo, y: Foo};
    print(p);
}

struct Point<T> {
    x: T,
    y: T,
}

struct Foo;

// 范型函数, 限定范型类型必须实现 Display trait
fn print<T: Display>(p: Point<T>) {
    println!("Point {}, {}", p.x, p.y);
}
