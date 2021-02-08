struct Point{
    x : i32,
    y : i32
}
enum Color{
    RED = 2,
    BLUE = 3,
    GREEN = 4
}
pub fn structures(){
        let location = Point{ x : 22, y : 56};
        println!("Your current location is {} x axis and {} y axis", location.x, location.y);
}

pub fn enums(){
    let c : Color = Color::RED;
    match c {
        Color::RED => println!(" color is red"),
        Color::BLUE => println!(" color is BLUE"),
        Color::GREEN => println!(" color is GREEN")
    }
}

pub fn arrays(){
    let mut a:[i32;5] = [1,2,3,4,5];
    a[1] = 345;

    println!("a has {} elements, first element is {}", a.len(), a[0] );
    println!("a array contains {:?}", a);
}

pub fn vectors(){
    let mut a  = Vec::new();
    a.push(1);
    a.push(4);
    a.push(6);
    println!("a vector contains {:?}", a);
    let idx:usize = 2;
    println!("a[idx] is {}", a[idx]);

    match a.get(2) {
        Some(x) => println!("x is {}",x),
        None => println!("index not found")
    }

    for x in &a{
        println!("iterate as {}",x);
    }
    let last_elm = a.pop();
    println!("the last removed element is {:?}", last_elm);

}

fn use_slices(slice : &mut[usize]){
    println!("slice value is {} and length is {}", slice[0], slice.len());
    slice[1] = 554; 
}
pub fn slices(){
    let mut arrs = [34,45,643,23];
    use_slices(&mut arrs[1..3]);
    println!("a is {:?}", arrs);
}


pub fn strings(){

    //utf-8
    // string slice
    // string slice are statically allocated with the program and can be referenced
    let s : &str = "Hello world";
    println!("{}",s);

    for c in s.chars()
    {
        println!("{}",c);
    }

    //utf-8
    //String (Heap memory)
    let mut st = String::new();
    st.push_str("Abbas");
    st.push_str(" Ogaji");
    println!("{}", st);

    let mut letters = String::new();

    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        println!("{}", a);
        letters.push(a as char);
        if a < ('z' as u8){
            letters.push_str(", ");
        }
        a+=1;
    }
    println!("{}", letters);
    
    //conversion &str <> String
    let u:&str = &letters;
    let ustring = u.to_string();
    println!("conversion works {}", ustring);

    // concatenation
    // String + &str
    let mut z = ustring + &u;
    println!("{}", z);
    z.remove(0);

}

fn sum_and_product(x : i32, y : i32) -> (i32, i32)
{
       return (x+y, x*y);
}

pub fn tuple(){
    let x = 32;
    let y = 55;
    let sp = sum_and_product(x, y);
    println!("sum and product value is {:?}", sp);
    println!("{0} + {1} = {2}, {0} x {1} = {3}", x,y, sp.0, sp.1);
    
    //destructing 
    let (a,b) = sp;
    println!("sum is {}, product is {}", a,b);
}

pub fn optional_type(){
    let optional_val : Option<f64> = Some(3.0/2.0);
    println!("{:?}", optional_val);
}