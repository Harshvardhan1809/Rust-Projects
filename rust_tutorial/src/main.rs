#![allow(unused)] // get rid of warnings for unused variables

// Running Rust code : cargo run main.rs

// use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
fn main() {
    println!("What is your name"); // println is a macro and not a function
    let mut name: String = String::new();
    let name = "Harsh";
    let greeting: &str = "Nice to meet you";
    //io::stdin().read_line(&mut name).expect("Didn't receive input");
    // marked that input is in name variable
    println!("Hello, {}! {} ", name.trim_end(), greeting);

    // Defining constant
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;

    // Rust allows variables with same name but different data-type (shadowing)
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned an number");
    // mut age is an int of size 32, string is a string
    // .trim() removes whitespace, .parsre() converts the form, .expect() is built-in error checking
    age = age + 1;

    println!("I'm {} and I want ${}", age, ONE_MIL);

    println!("Max usize: {}", usize::MAX);

    let num1: f32 = 1.111111111111111;
    println!("f32: {}", num1 + 0.000000000000001);

    let num1: f64 = 1.111111111111111;
    println!("f64: {}", num1 + 0.000000000000001);

    // let random_num: i32 = rand::thread_rng().gen_range(1..101);
    // println!("random number {}", random_num);

    // Ternary operators
    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can vote {}", can_vote);

    // match conditionals - just like switch
    match my_age {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an important birthday"),
    };

    let mut voting_age = 18;

    // match has more functionalities - cmp() provides order matching
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("Can Vote"),
    };

    // Arrays

    let arr1 = [1, 2, 3, 4, 5, 6];
    println!("First element : {}", arr1[0]);
    println!("Length of array  : {}", arr1.len());

    let mut loop_idx = 0; 

    loop {
        if(loop_idx % 2 == 0){
            loop_idx += 1;
            continue;
        }
        if(loop_idx == 5){
            println!("Val : {}", arr1[loop_idx]);
            break;
        }
        println!("Val : {}", arr1[loop_idx]);
        loop_idx += 1; 
    }

    let arr2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0; 
    while(loop_idx < arr2.len()){
        println!("Val : {}", arr2[loop_idx]); 
        loop_idx += 1; 
    }

    // Using iterator
    for val in arr2.iter(){
        println!("Val : {}", val); 
    }

    // Tuples

    // Strings 
    let mut st1 = String::new(); 
    st1.push('A'); 
    st1.push_str(" word"); 
    for word in st1.split_whitespace(){
        println!("{}", word); 
    }

    let st2 = st1.replace("A", "Another"); 
    println!("{}", st2); 

    // Vector 
    let st3 = String::from("z f w a g n r s"); 
    let mut v1: Vec<char> = st3.chars().collect(); 

    let int_u8: u8 = 5; 
    let int2_u8: u8 = 4; 
    let int_u32 : u32 = (int_u8 as u32) + (int2_u8 as u32); 
    println!("{}", int_u32); 


    // enums

}
