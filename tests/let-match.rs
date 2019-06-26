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
