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
    println!("can vote : {}", age >= 18);

    let mut adult: bool = false;
    match age {
        1..=18 => println!("Not adult"),
        19..=130 => println!("is Adult"),
        _ => {
            println!("age is older then 130, exit with error");
            std::process::exit(1);
        }
    }

    let arr: [i32; 14] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    println!("1st : {}", arr[0]);
    println!("length : {}", arr.len());

    let mut idx = 0;
    loop {
        if arr[idx] % 2 == 1 {
            idx += 1;
            continue;
        }
        if arr[idx] == 10 {
            // if 10 removed from array
            // index out of bounds: the len is 14 but the index is 14'
            // this "could" be found at compile time,
            // cago clippy is okay with this
            break;
        }
        println!("val : {}", arr[idx]);
        idx += 1;
    }

    idx = 0;
    while idx < arr.len() {
        println!("Arr[{}] = {}", idx, arr[idx]);
        idx += 1
    }

    for val in arr.iter() {
        println!("iter {}", val);
    }

    let food_tuple: (u8, String, f64) = (8, "pepperoni".to_string(), 100.0);

    println!(
        "The foods is called {} and it has {} calories, it is at {}%",
        food_tuple.1, food_tuple.0, food_tuple.2
    );

    let mut st1: String = String::new();
    st1.push('A');
    st1.push_str("BCD");
    println!("{}", st1);

    let st3 = String::from("a b c d e f g h i j k l m n x");
    println!("{}", st3);
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        print!("{}", char);
    }
    println!();

    let stack_string: &str = "data";
    let mut heap_string: String = stack_string.to_string();
    println!("{}, {}", stack_string, heap_string);

    let byte_arry = heap_string.as_bytes();

    let mut check_sum: u64 = 0;
    for i in byte_arry.iter() {
        // check_sum += u64::from(i);
        check_sum += *i as u64;
    }
    println!("check_sum {}", check_sum);

    std::process::exit(0);
}
