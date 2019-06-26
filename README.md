# off-side.rs

| Build | Status |
| :---: | :----: |
| Travis-CI | [![](https://travis-ci.com/chankyin/off-side.rs.svg?branch=master)](https://travis-ci.com/chankyin/off-side.rs) |

Use off-side syntax (indent instead of braces, like in Python) to write Rust!

Disclaimer: implementing a different standard of the language is apparently not a good idea.
This project is just a proof of concept for fun.

## Off-side rule
[Off-side rule][off-side wiki] means that a computer language uses indents instead of using `{}` braces to distinguish
code hierarchy. Well-known examples include YAML (a data serialization language), Python (a programming language) and
PugJS (a JavaScript markup preprocessor language).

## Example
```rust
#[macro_use]
extern crate off_side;

use std::io::{Error, ErrorKind};

off_side! {

fn try_int(i: i32) -> Result<i32, Error>:
    match i:
        0 => Ok(1),
        1 => Err(Error::new(ErrorKind::NotFound, "Hello")),
        _ => unreachable!(),

fn main():
    for i in 0..2:
        let result = try_int(i);
        let num = match result:;
            Ok(i) => i,
            Err(err) => err.raw_os_error().unwrap_or_else(|| 3),
        println!("num: {}", num);

}
```

## Syntax specification
- Indent rules follow the specification in the [`indent-stack`][indent-stack] crate.
- Due to `proc_macro` limitations, all indent characters are considered the same.
  In other words, if you mix tabs and spaces, tabs will be considered as one space.
- Do not place block comments `/* */` before line start. They *might* be considered as spaces as well.
- On the parent line of indent blocks that should have been wrapped by braces, end the parent line with an extra `:`.
- If the braced indent block should be followed by a semicolon (e.g. `let` statements with `match` or `if`/`else`),
  end the parent line with `:;`.

# Final disclaimer
This library is a **proof of concept** and **just for fun**.
Not recommended for use in production or in publicly published crates.

  [off-side wiki]: https://en.wikipedia.org/wiki/Off-side_rule
  [indent-stack]: https://docs.rs/indent-stack
