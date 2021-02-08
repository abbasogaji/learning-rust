fn how_many(x:i32) -> &'static str
{
    match x {
        0 => "no oranges",
        1 | 2 => "one or two oranges",
        8..=11 => "a few",
        12 => "a dozen",
        _ if (x%2 == 0) => "oranges of even numbers",
        _ => "so many oranges"
    }
}

pub fn matching(){  

    for x in 0..15
    {
        println!("{}: has {}",x, how_many(x));
    }

    let point = (3,5);
    println!("point is {:?}", point);
    match point
    {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis,  y axis = {}", y),
        (x,y) => println!("x axis = {}, y axis = {}", x,y)
    }
} 