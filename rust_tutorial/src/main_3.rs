#![allow(unused)] // get rid of warnings for unused variables
#[warn(bare_trait_objects)]

// Running Rust code : cargo run main_3.rs

// use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};

use std::ops::Add;
use std::collections::HashMap; 

mod restaurant; 
use crate::restaurant::order_food; 

fn main(){

    // Structs
    struct Customer{
        name: String,
        address : String, 
        balance : f32, 
    }

    let mut bob: Customer = Customer{
        name: String::from("Bob Smith"), 
        address: String::from("555 Main St"), 
        balance: 234.50
    };

    bob.address = String::from("505 Main St"); 

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32; 
    }

    struct Rectangle {length: f32, width: f32}; 
    struct Circle {length: f32, width: f32}; 

    impl Shape for Rectangle{
        // new() is the constructor
        fn new(length: f32, width: f32) -> Rectangle{
            return Rectangle{length, width}; 
        }

        fn area(&self) -> f32 {
            return self.length * self.width; 
        }
    }
   // let rec: Rectangle = Shape::new{length: 10.0, width: 10.0}; 


   // Modules 
   order_food(); 

   // Exceptions and panic 
   // panic!("Terrible Error");
   let path: &str = "lines.txt"; 
   let output = File::create(path);
   let mut output: File = match output{
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}", error); 
        }
   };
   // Writing to the file 
   write!(output, "Just some\nrandom words").expect("Failed to write to file");

   let input = File::open(path).unwrap(); 
   // unwrap extracts the output we want from the Result returned 
   let buffered = BufReader::new(input); 
   for line in buffered.lines(){
    println!("{}", line.unwrap()); 
   }
   
   let output2 = File::create("rand.txt"); 
   let output2 = match output2{
        Ok(file) => file,
        Err(error) => match(error.kind()){
            ErrorKind::NotFound => match File::create("rand.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file {:?}", e), 
            }
            _other_error => panic!("Problem opening file {:?}", error), 
        },
   };


   // Iterators
   let mut arr_it = [1,2,3,4]; 
   for val in arr_it.iter(){
        println!("Val : {}", val); 
   }
   let mut iter1 = arr_it.iter(); 
   println!("1st : {:?}", iter1.next().unwrap());


   // Closures 
   let can_vote = |age: i32| -> bool {
        age >= 18
   }; 
   println!("Can vote? {}", can_vote(8)); 

}

