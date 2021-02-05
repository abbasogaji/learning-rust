use std::mem;

fn main() {
    // Variable Declaration
    // immutable declaration
    let age : u8 = 200; // 8 bits
    println!("a = {}", age);

    // mutable declaration
    let mut salary : u16 = 65535;
    println!("salary is {}", salary);
    salary = 45000;
    println!("salary is {} and size is {} bytes", salary, mem::size_of_val(&salary));
}
