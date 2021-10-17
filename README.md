# Hoax

Simple procedural macro crate that *"hides"* string literals in the binary in **plain sight**.


## Usage
```toml
[dependencies]
hoax = "1.0.0"
```

### *hoax!*
Panics only when the first token tree is not a string literal, any followings are ignored. <br>
Returns a `String` collected from a vector of chars.

### *Example*
```rust
#[macro_use] extern crate hoax;

println!("{}", "I am not hidden :c");
println!("{}", hoax!("I guess I am hidden c:"));
```

### *Expansion*
```rust
hoax!("cat\n")
```
is the same as
```rust
{vec!['c', 'a', 't', '\n',].iter().collect::<String>()}
```
