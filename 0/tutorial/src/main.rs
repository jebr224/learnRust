#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello, {}! {}", name.trim_end(), greeting);

    const ONE_MIL: u32 = 1_000_000;
    println!("printing constant 1,000,000 {}", ONE_MIL);
    println!("printing constant PI {}", std::f32::consts::PI);

    // Unsigned integer : u8, u16, u32, u64, u128, usize
    // Signed integer   : i8, i16, i32, i64, i128, isize
    println!("Max u8   : {},", u8::MAX);
    println!("Max u16  : {},", u16::MAX);
    println!("Max u32  : {},", u32::MAX);
    println!("Max u64  : {},", u64::MAX);
    println!("Max u128 : {},", u128::MAX);

    println!("Min u8   : {},", u8::MIN);
    println!("Min u16  : {},", u16::MIN);
    println!("Min u32  : {},", u32::MIN);
    println!("Min u64  : {},", u64::MIN);
    println!("Min u128 : {},", u128::MIN);

    println!("Max i8   : {},", i8::MAX);
    println!("Max i16  : {},", i16::MAX);
    println!("Max i32  : {},", i32::MAX);
    println!("Max i64  : {},", i64::MAX);
    println!("Max i128 : {},", i128::MAX);

    println!("Min i8   : {},", i8::MIN);
    println!("Min i16  : {},", i16::MIN);
    println!("Min i32  : {},", i32::MIN);
    println!("Min i64  : {},", i64::MIN);
    println!("Min i128 : {},", i128::MIN);

    println!("usize");
    println!("Max usize: {}, Bits: {}", usize::MAX, usize::BITS);

    let is_good: bool = true;

    let num_1: i32 = 1;
    let num_2: i32 = 2;

    println!("1 + 2 = {}", num_1 + num_2);
    // no implementation for `u32 + i32 and `i32 + u32`

    println!("1 - 2 = {}", num_1 - num_2);
    // no implementation for `u32 - i32 and `i32 - u32`
    // with i32 => 1 -2 = -2
    // (runtime) wiht u32 => thread 'main' panicked at 'attempt to subtract with overflow',

    println!("1 * 2 = {}", num_1 * num_2);
    // no implementation for `u32 * i32 and `i32 * u32`

    println!("1 / 2 = {}", num_1 / num_2);
    // no implementation for `u32 / i32 and `i32 / u32`

    println!("1 % 2 = {}", num_1 % num_2);
    // no implementation for `i32 % u32`

    let fnum_1: f32 = 1. / 3.;
    let fnum_2: f64 = 1. / 3.;

    println!("f32 : {}", fnum_1);
    println!("f64 : {}", fnum_2);

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);

    println!("What is your age?");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Didn't Receive Input");

    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    println!("next year you will be {} years old", age + 1);

    let mut important_birthday: bool = false;
    let mut toys: bool = false;
    if (age > 0) && (age <= 18) {
        important_birthday = true;
        toys = true;
    } else if (age == 21) || (age == 50) || (age >= 65) {
        important_birthday = true;
    }

    println!("important birtday : {}", important_birthday);
    println!("toys : {}", toys);
    println!("can vote : {}", age >= 18)
}
