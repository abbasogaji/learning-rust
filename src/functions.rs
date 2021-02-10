struct Point{
    x : i32,
    y : i32
}

struct Line{
    start : Point,
    end : Point
}

impl Line{
    fn len(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        ((dx*dx + dy*dy) as f64).sqrt()
    }
}

fn increase(x : &mut i32){
    //dereferencing a variable x and updating its value
    *x += 1;
}

//funtions with return type
fn product(x:i32, y:i32) -> i32
{
    return x*y;
}

pub fn print_value(){
    let mut x = 2;

    // passing a reference of the variable x
    increase(&mut x);
    println!("{}", x)
}

pub fn get_length_of_line(){
    let p1 = Point {x : 22, y : 45};
    let p2 = Point {x : 56, y : 77};
    let line = Line {start : p1, end : p2};
    println!("Length of a line is {}", line.len());

}
// Assigning a function to the a variable
pub fn closure(){
    let pv = print_value;
    pv();
    //closure - function only exists in this function
    let plus_one = |x:i32| -> i32 { x+1 };
    let a = 6;
    println!("{}", plus_one(a));
    let mut y = 44;
    let print_value_two = |x:i32| 
    {
        let z = x+y;
        return z;
    };
    println!("executing function witthin closure and value is {}", print_value_two(22));
    let borrow_value = &mut y;
    println!(" i can prove it cause borrowed value is {}", borrow_value);

    
}

trait Animal{
    fn create(name : &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human{
    name : &'static str
}

struct Cat{
    name : &'static str
}

impl Animal for Human{
    fn create(name: &'static str) -> Human {
        Human{ name : name}
    }

    fn name(&self)  -> &'static str {
        return  self.name;
    }

    fn talk(&self){
        println!("{} says `hallooooo`", self.name());
    }
}

impl Animal for Cat{
    fn create(name: &'static str) -> Cat {
        Cat{ name : name}
    }

    fn name(&self)  -> &'static str {
        return  self.name;
    }

    fn talk(&self){
        println!("{} says `meowwwww`", self.name());
    }
}

pub fn traits(){
   let person1 = Human { name : "Abbas" };
   person1.talk();

   let feline = Cat { name : "milky"};
   feline.talk();

   let feline2 = Cat::create("sami");
   feline2.talk();

   let feline3 : Cat = Animal::create("wipo");
   feline3.talk(); 
}