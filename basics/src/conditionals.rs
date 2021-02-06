pub fn for_loop_statement()
{
    for x in 0..11 
    {
        println!("x value is {}", x)
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

}


pub fn loop_statement(){
    let mut y =1;
    loop{
        y += 1;
        println!("x is {}", y);

        if y == 150 {break};
    }
}