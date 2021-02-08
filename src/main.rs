#![allow(dead_code)]
use std::mem;

mod conditionals;
mod stack_heap;
mod data_structures;

const AUTHORS_NAME: &str = "Abbas Ogaji";
fn main() {
    // Variable Declaration

    // operators();
    // variable_decalartion();
    // scopes_and_shadowing();
    // stack_heap::stack_and_heap();
    // conditionals::for_loop_statement();
    // conditionals::match_statement();
    // data_structures::structures();
    // data_structures::optional_type();
    // data_structures::vectors();
    data_structures::slices();
    // conditionals::while_loop_statement();
    // conditionals::loop_statement();
}

fn scopes_and_shadowing() {
    let a = 345;
    {
        let b = 44;
        let a = 33;
        println!("b is = {}", b);
        println!("a variable is {}", a);
    }
    println!("a outside is = {}", a);
    println!("Authurs name is {}", AUTHORS_NAME)
}

// Operators
fn operators() {
    //aritemetic
    let mut a = 1 + 2 * 3;
    println!("a = {}", a);
    a = a + 1;
    println!("a is now = {}", a);
    a += 1;
    println!("a has again changed to = {}", a);
    a -= 2;
    println!("a has again for the third time changed to = {}", a);
    println!("a cubed is {}", i32::pow(a, 3));
    println!("pi constant is {}", std::f64::consts::PI)
}

fn variable_decalartion() {
    // immutable declaration
    let age: u8 = 200; // 8 bits
    println!("a = {}", age);

    // mutable declaration
    let mut salary: u16 = 65535;
    println!("salary is {}", salary);
    salary = 45000;
    println!(
        "salary is {} and size is {} bytes",
        salary,
        mem::size_of_val(&salary)
    );

    let var = 122323232;
    let str_var = "Abbas is a good guy leave him alone oooooo";
    println!(
        "default size of an interger is {} bytes",
        mem::size_of_val(&var)
    );
    println!(
        "default size of a string is {} bytes",
        mem::size_of_val(&str_var)
    );
}
