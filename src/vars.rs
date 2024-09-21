use std::convert::TryFrom;
use std::convert::TryInto;

use std::fmt;

struct Circle {
    radius: i32,
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

#[derive(Debug, PartialEq)]

struct EventNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 2 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

pub fn add_one(x: u32) -> u32 {
    println!("its added one {:?}", x + 1);
    x + 1
}

pub fn pluser() {
    let mut sum = 0;

    for i in 1..=8 {
        sum += i
    }

    println!("sum {:?}", sum);
}
