 use std::io;
fn main() {

    println!("Enter Number 1");
    let mut a = String::new();
    io::stdin().read_line(&mut a);
    let a:f32 = a.trim().parse().unwrap();

    println!("Enter NUmber 2");
    let mut b = String::new();
    io::stdin().read_line(&mut b);
    let b:f32 = b.trim().parse().unwrap();

    println!("press 1 for add \npress 2 for subtract \npress 3 for multiply \npress 4 for divide" );
    let mut c = String::new();
    io::stdin().read_line(&mut c);
    let c:i32 = c.trim().parse().unwrap();

    if c == 1 {
        println!("{} + {} = {} ", a , b , a+b );
    }

    else if c == 2 {
        println!("{} - {} = {}",a ,b , a-b );
    }

    else if c == 3 {
        println!("{} * {} = {}",a ,b , a*b );
    }

    else if c == 4 {
        println!("{} / {} = {}",a ,b , a/b );
    }

    else {
        println!("invalid option selected" );
    }



}
