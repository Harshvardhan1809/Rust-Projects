#![allow(unused)] // get rid of warnings for unused variables
#![warn(bare_trait_objects)]

// Running Rust code : cargo run main_4.rs

// use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};

use std::ops::Add;
use std::collections::HashMap; 

use std::thread; 
use std::time::Duration; 

use std::rc::Rc; 
use std::cell::RefCell; 
use std::sync::{Arc, Mutex}; 

fn main(){

    // Smart Pointers
    // Create binary tree using box 
    // BOX : you have data on heap but create pointer on stack 

    let b_int_1 = Box::new(10);
    println!("b_int_1 : {}", b_int_1); 

    struct TreeNode<T>{
        pub left : Option<Box<TreeNode<T>>>, 
        pub right : Option<Box<TreeNode<T>>>, 
        pub key: T, 
    }

    impl <T> TreeNode<T>{
        pub fn new(key: T) -> Self{
            TreeNode{
                left: None, 
                right: None, 
                key, 
            }

        }
        pub fn left(mut self, node: TreeNode<T>) -> Self{
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self{
            self.right = Some(Box::new(node));
            self
        }
    }
    let node1 = TreeNode::new(1).left(TreeNode::new(2)).right(TreeNode::new(3)); 


    // Concurrency 
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i); 
            thread::sleep(Duration::from_millis(1)); 
        }
    }); 

    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1)); 
    }

    thread1.join().unwrap(); 

    pub struct Bank{
        balance: f32,
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32){
        
        let mut bank_ref = the_bank.lock().unwrap();
        if (bank_ref.balance < 5.00) {
            println!("Current Balance : {} Withdraw a smaller amount", bank_ref.balance); 
        }
        else {
            bank_ref.balance -= amt; 
            println!("Customer withdrew {} Current Balance {}", amt, bank_ref.balance); 
        }
    }
    fn customer(the_bank: Arc<Mutex<Bank>>){
        withdraw(&the_bank, 5.00); 
    }
    let bank : Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance: 20.00})); 

    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        } )
    }); 

    for handle in handles{
        handle.join().unwrap(); 
    }

    println!("Total {}", bank.lock().unwrap().balance); 

}   