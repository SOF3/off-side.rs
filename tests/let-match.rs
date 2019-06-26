// Copyright (C) 2019 chankyin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
