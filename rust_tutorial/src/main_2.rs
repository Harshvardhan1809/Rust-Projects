#![allow(unused)] // get rid of warnings for unused variables

// use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};

use std::ops::Add;
use std::collections::HashMap; 

// Functions 
fn get_sum2(x: i32, y: i32) -> i32 {
    // Returns x+y
    x+y
}

fn get_sum_list(list: &[i32]) -> i32 {
    let mut sum: i32 = 0; 
    for val in list.iter(){
        sum += val; 
    }
    return sum; 
}

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T{
    // Add is the trait of the T generic type
    return x+y;
}

fn main(){

    // Enums 
    enum Day {
        Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday 
    }

    impl Day{
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true, 
                _ => false, 
            }
        }
    }

    let today:Day = Day::Monday; 
    match today {
        Day::Monday => println!("Everyday hates Monday"), 
        _ => println!("Not a Monday"), 
    }

    println!("Is today a weekend {}", today.is_weekend()); 


    // Vectors
    let vec1 : Vec<i32> = Vec::new(); 
    let mut vec2 = vec![1,2,3,4]; 
    vec2.push(5); 
    println!("1st value : {}", vec2[0]);
    let second: &i32 = &vec2[1]; 
    match vec2.get(1){
        Some(second) => println!("2nd value : {}", second), 
        None => println!("Not found"), 
    }

    // Using reference, use star to manipulate values 
    for i in &mut vec2 {
        *i *= 2; 
    }

    for i in &vec2{
        println!("{}", i);
    }

    // Popping and getting the specific value
    println!("Pop : {:?}", vec2.pop());


    // Functions
    println!("Sum : {}", get_sum2(5,4));

    let num_list : Vec<i32> = vec![1,2,3,4]; 
    println!("Sum : {}", get_sum_list(&num_list));


    // Generics
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("3.4 + 2.2 = {}", get_sum_gen(3.4, 2.2));


    // Ownership
    let str1: String = String::from("World"); 
    let str2: String = str1.clone(); 
    // ownership is transferred to str2
    println!("Hello {}", str1);


    // Hash Maps
    let mut heros = HashMap::new(); 
    heros.insert("Superman", "Clark Kent");
    heros.insert("Batman", "Bruce Wayne");
    heros.insert("The Flash", "Barry Allen");

    for(k,v) in heros.iter(){
        println!("{}, {}", k, v); 
    } 
    if(heros.contains_key(&"Batman")){
        let the_batman = heros.get(&"Batman"); 
        match (the_batman){
            Some(x) => println!("Batman is a hero"), 
            None => println!("Batman is not a hero"), 
        }
    }

    println!("Hello World"); 
}

// what to do -> mer