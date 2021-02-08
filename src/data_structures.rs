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

fn use_slices(slice : &[usize]){
    println!("slice value is {} and length is {}", slice[0], slice.len());
}

pub fn slices(){
        let arrs = [34,45,643,23];
        use_slices(&arrs[1..3]);
}

pub fn optional_type(){
    let optional_val : Option<f64> = Some(3.0/2.0);
    println!("{:?}", optional_val);
}