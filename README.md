# off-side.rs
Implements a Rust syntax variant with off-side rule, just like in Python.

Disclaimer: implementing a different standard of the language is apparently not a good idea.
This project is just a proof of concept for fun.

## Off-side rule
[Off-side rule][off-side wiki] means that a computer language uses indents instead of using `{}` braces to distinguish
code hierarchy. Well-known examples include YAML (a data serialization language), Python (a programming language) and
PugJS (a JavaScript markup preprocessor language).

How to use:
```rust
off_side!{
fn world(hello: u8):
    if hello >= 128:
        println!("Hello world!");
    else:
        println!("Bye world!");

fn main():
    world(150);
    world(50);
}
```

  [off-side wiki]: https://en.wikipedia.org/wiki/Off-side_rule
