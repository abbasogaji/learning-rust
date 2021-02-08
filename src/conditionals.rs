pub fn for_loop_statement()
{
    for x in 0..11 
    {
        println!("x value is {}", x)
    }
    for (pos, y) in (30..40).enumerate()
    {
        println!("pos {} has value {}", pos, y);
    }
}

pub fn while_loop_statement(){
    let mut x = 1;
    while x < 10
    {
        x += 1;
        println!("x is {}", x);
    }

}

pub fn match_statement(){
    let country_code = 234;
    let country = match country_code{
        44 => "United Kingdom",
        234 => "Nigeria",
        7 => "Russia",
        1 => "USA",
        2..=999 => "Unknown",
        _ => "Invalid"
    };
    println!("the country with code {} is {}", country_code, country)
}


pub fn loop_statement(){
    let mut y =1;
    loop{
        y += 1;
        println!("x is {}", y);

        if y == 150 {break};
    }
}