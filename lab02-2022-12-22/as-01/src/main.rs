use std::io;

fn main() {
    println!("Enter a number:");
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    
    let x: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
        return;
        },
    };
    
    for i in 0 .. x {
        for j in 0 .. x {
            if i >= j {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
