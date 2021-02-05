use std::mem;

fn main() {
    // Variable Declaration
    // immutable declaration
    // let age : u8 = 200; // 8 bits
    // println!("a = {}", age);

    // // mutable declaration
    // let mut salary : u16 = 65535;
    // println!("salary is {}", salary);
    // salary = 45000;
    // println!("salary is {} and size is {} bytes", salary, mem::size_of_val(&salary));
    operators()
}

// Operators
fn operators(){
    let mut a = 1+2*3;
    println!("a = {}", a);
    a = a+1;
    println!("a is now = {}", a);
    a += 1;
    println!("a has again changed to = {}", a);
    a -= 2;
    println!("a has again for the third time changed to = {}", a)

}