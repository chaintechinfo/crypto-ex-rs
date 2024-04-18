# Rust

> A language empowering everyone to build reliable and efficient software.

Rust 是一个现代化的编程语言，聚焦在安全性、速度和并发性上。它的语法类似于 C++，但是它的设计更加注重安全性。 Rust 通过使用所有权系统来实现内存安全，
这使得它能够在编译时防止数据竞争。

Features

1. 支持类型推导，但声明常量 (const) 和静态变量 (static) 时需要声明类型
2. 默认变量是不可变的，符合最小权限原则，需要使用 mut 关键字来声明可变变量；如果使用 mut 修饰但没有改变变量的值，编译器会给出警告
3. Rust 的世界中，函数是一等公民，所以可以作为参数、返回值等，所以函数签名非常重要
4. 如果一个函数没有返回值，那么它的返回值类型 unit 类型，它只有一个值，就是 ()
5. 在 rust 中，函数的最后一个表达式没有分号，那么它的值就是函数的返回值；如果以分号结尾，那么它的返回值就是 unit 类型
6. rust 支持自定义类型，可以使用 struct 关键字来定义结构体，使用 enum 关键字来定义枚举
7. Rust 提供了派生宏，大大简化了一些标准接口的定义，形式如：#[derive(Debug)]
8. Rust 的错误处理比较的不一样，它使用 Result<T, E> 类型来处理错误，同时使用 ? 操作符来简化错误处理
9. Rust 支持宏，宏是一种元编程的方式，它可以在编译时生成代码，宏的工作方式和 C++ 中的模板类似，但是它的语法更加的灵活；声明宏和过程宏，过程宏有函数宏、派生宏和属性宏

## Rust 语言规则

--

## Toolchain

可以看 [Rust lang get started](https://www.rust-lang.org/zh-CN/learn/get-started).

- Install

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装 Rust 非常方便, 安装成功后，包含很多命令，比如 rustc, cargo, rustup 等。

- rustc

rust 编译器，用于编译 rust 代码。

```shell
# Rust version
rustc --version

# Compile and run
rustc hello_world.rs
./hello_world

# Compile and run in a target directory
rustc --out-dir=target hello_world.rs
./target/hello_world

```

- [cargo](https://doc.rust-lang.org/cargo/index.html)

cargo 是 rust 的构建工具和包管理器，它可以用于创建新项目，构建项目，运行测试，生成文档等。

```shell
# create a new project
cargo new hello_cargo --bin

# build documentation
cargo doc

# test doc
cargo test --doc

# build and run
cargo run
cargo build

# publish
cargo publish
```

- 升级 rust

升级 rust 的版本，使用 `rustup update` 命令，如果要升级到 nightly 版本，使用 `rustup update nightly` 命令。
如果要删除 rust，使用 `rustup self uninstall` 命令。

## Rust Core

Rust 是一个静态类型的编程语言，融合了非常多语言的特性，以及自己解决特定问题的思路，所以 Rust 有自己的语言特性和语言规则。

### Comments

Rust 有多种注释方式，其中 

1. `///` 和 `//!` 用于文档注释，在生成文档时非常有用
2. `//` 和 `/* */` 用于普通注释，普通的行注释和块注释

### 操作符 operator

#### ?

? 操作符用于简化错误处理，它只能用于返回 Result 类型的函数，如果函数返回 Ok(T) 类型的值，那么 ? 操作符会返回 Ok(T) 中的值；

### Data Types

- 原始类型
  - 类型字面量以及操作符

```rust
fn test() {
  println!("1 + 2 = {}", 1u32 + 2);
  println!("1 - 2 = {}", 1i32 - 2);
  println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
  println!("true and false is {}", true && false);
  println!("true or false is {}", true || false);
  println!("not true is {}", !true);

  println!("0011 & 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 | 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 ^ 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
  println!("One million is written as {}", 1_000_000u32);
}
```

- 复合类型
  - Tuple
  - Array
- 自定义类型
  - struct: tuple struct, the classic C struct, Unit struct (field-less, useful for generics)
  - enum
  - const or static

Rust 提供了几种改变和定义类型的方式：

1. 原始类型间的转换：Rust 不支持隐式类型转换，可以使用 as 执行显示类型转换
2. 指定所需的字面量类型
3. 使用类型推断
4. 使用别名

### Variables

变量绑定以及变量类型推导，let 关键字用于声明变量，let 关键字后面跟一个模式，模式用于匹配变量的类型，
如果没有指定变量类型，Rust 会根据变量的使用上下文来推导变量的类型。

变量绑定都有一个 scope，只有在 scope 内才能使用，scope 以 {} 为界限。

### Pointer and Reference

Rust 中的指针和引用有什么区别呢？指针和引用都是持有一些内存地址的变量，主要区别在于语义上的不同。
引用需要受借用检查器检查，而指针不需要；从这里可以看出，指针是一种不安全的操作，而引用是一种安全的操作。
一个指针指向的内存地址可以被解构为任何类型，并且指针可能是不安全的，因为它可能指向一个无效的内存地址；
而引用受到了一些约束，它只能指向有效的内存地址，并且它的类型是明确的。(这些是受到编译器强制检查的)

[What's the difference between references and pointers in Rust?](https://ntietz.com/blog/rust-references-vs-pointers/)

### Ownership

### macro

几个问题：

1. 宏的工作原理和作用
2. 如何自定义宏

#### format 和 print 系列宏

出自：[std::fmt](https://doc.rust-lang.org/std/fmt/)

- println!()：打印文本到控制台 (io::stdout), 并在最后一个参数后面加上换行符
- format!(): 格式化文本，并使用 String 类型的变量接收
- print!()：打印文本到控制台 (io::stdout)，不换行
- eprint!()：打印文本到标准错误输出 (io::stderr)，不换行
- eprintln!()：打印文本到标准错误输出 (io::stderr)，并在最后一个参数后面加上换行符

**一个重要结论**：如果要使用 format 或这 print 等宏格式化文本，要用到 {} 占位符，而 {} 占位符的作用是什么呢？它是一个格式化参数，
可以接收任何实现了 Display trait 的类型。类型如果没有实现 Display trait，就不能使用 {} 占位符。

{} 标记 用于 fmt::Display
{:?} 标记 用于 fmt:Debug, 所有的 std 库类型都实现了 Debug trait
{:#?} 标记用于 fmt:Debug, 但是会更好看一些

#### #[allow(dead_code)]

这个宏用于禁止编译器对未使用的代码的警告，这个宏可以用于函数、结构体、枚举、impl 块等。

```rust
// 加了这个宏，就不会有警告了
#[allow(dead_code)]
fn unused_function() {
    println!("This function is unused.");
}
```

#### vec![]

vec 宏用于创建一个 Vec<T> 类型的变量，其中 T 是任意类型。

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);
}
```

#### panic!()

panic!() 宏用于在程序中触发一个 panic，它会打印一个错误信息，然后退出程序。

```rust
fn main() {
    panic!("crash and burn");
}
```

## 模块和 crate

