use std::io;

fn main() {
   
    println!("Enter 1st Number = ");
    let mut a = String::new();
    io::stdin().read_line(&mut a);
    let a:f32 = a.trim().parse().unwrap();

    println!("Enter 2nd Number = " );
    let mut b = String::new();
    io::stdin().read_line(&mut b);
    let b:f32 = b.trim().parse().unwrap();

   println!("Enter Operation " );
    let mut c = String::new();
    io::stdin().read_line(&mut c);
    let c:&str = c.trim();

    if c == "+" {
        println!("{} + {} = {}", a,b, a + b );
    }
    else if c == "-" {
         println!("{} - {} = {}", a,b, a - b );
    }

    else if c == "/" {
         println!("{} / {} = {}", a,b, a / b );
    }

    else if c == "*" {
         println!("{} * {} = {}", a,b, a * b );
    }

    else {
        println!("invalid operation" );
    }

}
