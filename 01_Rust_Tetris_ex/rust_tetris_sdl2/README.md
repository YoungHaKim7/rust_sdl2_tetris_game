# Result

```bash
cargo r --release
```


# Example(연구가 필요함)
- https://github.com/rust-lang/rust-clippy/issues/12895
  
```rs
lazy_static! {
    static ref FOO: String = "foo".to_uppercase();
}
static BAR: Lazy<String> = Lazy::new(|| "BAR".to_lowercase());
```

- Could be written as:

```rs
static FOO: LazyLock<String> = LazyLock::new(|| "foo".to_uppercase());
static BAR: LazyLock<String> = LazyLock::new(|| "BAR".to_lowercase());
```
