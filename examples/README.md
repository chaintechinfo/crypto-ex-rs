# Rust Examples

Run the examples with `cargo run --example <name>`.

```shell
cargo run --example borrow
# or with parameters
cargo run --example borrow -- 1 2 3
```

Another way is to compile the examples with `rustc` and run the binary.

```shell
cd borrow
rustc --out-dir=target main.rs
./target/main
```
