extern crate rand;
use rand::Rng;

pub fn generate_rand_number(){
    let mut rng = rand::thread_rng();
    let b : u32 = rng.gen();
    println!("{}", b);
}